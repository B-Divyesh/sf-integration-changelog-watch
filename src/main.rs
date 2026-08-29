use axum::{
    extract::{ConnectInfo, Path, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use chrono::Utc;
use reqwest::{redirect::Policy, Client};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use std::{
    collections::HashMap,
    env,
    net::{IpAddr, SocketAddr},
    path::{Path as FilePath, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, warn};
use url::Url;
use uuid::Uuid;

#[derive(Clone)]
struct App {
    db: SqlitePool,
    build: String,
    limiter: Arc<Mutex<HashMap<IpAddr, RateBucket>>>,
}

#[derive(Clone, Copy)]
struct RateBucket {
    tokens: f64,
    refreshed: std::time::Instant,
}

#[derive(Debug, Serialize, FromRow)]
struct Watch {
    id: i64,
    vendor: String,
    url: String,
    keywords: String,
    owner: String,
    version: String,
    command: String,
    #[serde(rename = "lastScan")]
    last_scanned: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct NewWatch {
    vendor: String,
    url: String,
    keywords: String,
    owner: String,
    version: String,
    command: String,
}

#[derive(Debug, Deserialize)]
struct WatchImport {
    watches: Vec<NewWatch>,
}

#[derive(Debug, Serialize, FromRow)]
struct Action {
    id: i64,
    #[serde(rename = "watchId")]
    watch_id: i64,
    title: String,
    excerpt: String,
    matched: String,
    url: String,
    owner: String,
    version: String,
    command: String,
    acknowledged: bool,
    #[serde(rename = "seenAt")]
    seen_at: String,
}

#[derive(Debug, Deserialize)]
struct Ack {
    acknowledged: bool,
}

#[derive(Debug, Serialize)]
struct Workspace {
    token: String,
}

#[derive(Debug, Serialize)]
struct ScanResult {
    new_actions: usize,
    failures: Vec<String>,
    message: String,
}

#[derive(Deserialize)]
struct CliConfig {
    watches: Vec<NewWatch>,
    #[serde(default)]
    state_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct CliState {
    actions: Vec<CliAction>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CliAction {
    id: String,
    notice_hash: String,
    acknowledged: bool,
}

#[derive(Debug)]
struct ApiError(StatusCode, String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.0,
            [(header::CACHE_CONTROL, HeaderValue::from_static("no-store"))],
            self.1,
        )
            .into_response()
    }
}

type ApiResult<T> = Result<T, ApiError>;

/// Azure Files-backed SQLite and the in-process limiter are safe only while
/// this Container App has one replica. New revisions keep workspace APIs
/// closed until the checked-in topology and the /data mount are both present.
static PRODUCTION_TOPOLOGY_READY: AtomicBool = AtomicBool::new(true);
const PRODUCTION_RESOURCE_ID: &str = "/subscriptions/283af945-693b-4a6e-b952-df928d0a18a9/resourceGroups/sociobot/providers/Microsoft.App/containerApps/sf-integration-changelog-watch";
const PRODUCTION_IDENTITY_CLIENT_ID: &str = "ba10d5bc-6375-4325-8892-4c7a5be500ca";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }
    if args
        .first()
        .is_some_and(|arg| arg == "demo" || arg == "--demo")
    {
        print_demo_markdown();
        return;
    }
    if args.first().is_some_and(|arg| arg == "scan") {
        if let Some(path) = args
            .windows(2)
            .find(|pair| pair[0] == "--config")
            .map(|pair| pair[1].clone())
        {
            if let Err(error) = cli_scan(&path).await {
                eprintln!("scan failed: {error}");
                std::process::exit(1);
            }
            return;
        }
        eprintln!("scan needs --config <path>");
        std::process::exit(2);
    }
    if args.first().is_some_and(|arg| arg == "ack") {
        let config = args
            .windows(2)
            .find(|pair| pair[0] == "--config")
            .map(|pair| pair[1].clone());
        let action = args
            .windows(2)
            .find(|pair| pair[0] == "--id")
            .map(|pair| pair[1].clone());
        match (config, action) {
            (Some(path), Some(id)) => {
                if let Err(error) = cli_ack(&path, &id) {
                    eprintln!("ack failed: {error}");
                    std::process::exit(1);
                }
                return;
            }
            _ => {
                eprintln!("ack needs --config <path> and --id <action-id>");
                std::process::exit(2);
            }
        }
    }

    tracing_subscriber::fmt()
        .json()
        .with_env_filter("info")
        .init();
    start_production_topology_guard();
    let supplied_database_url = env::var("DATABASE_URL").ok();
    let db_url = supplied_database_url
        .clone()
        .unwrap_or_else(default_database_url);
    // SQLite and the rate bucket are deliberately single-replica state. Do
    // not fall back to the image filesystem: that would make a failed volume
    // mount look healthy while silently splitting or losing workspaces.
    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("durable SQLite database starts");
    setup(&db).await.expect("schema");
    let state = App {
        db,
        build: env::var("BUILD_SHA")
            .ok()
            .filter(|value| value != "dev")
            .or_else(|| std::fs::read_to_string("/app/build-sha").ok())
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "dev".to_owned()),
        limiter: Arc::new(Mutex::new(HashMap::new())),
    };
    info!(
        database_url = if supplied_database_url.is_some() {
            "supplied"
        } else {
            "defaulted"
        },
        "starting Integration Changelog Watch"
    );

    let api = Router::new()
        .route("/health", get(health))
        .route("/api/workspaces", post(create_workspace))
        .route("/api/watches", get(list_watches).post(add_watch))
        .route("/api/watches/import", post(replace_watches))
        .route("/api/watches/:id", put(update_watch).delete(delete_watch))
        .route("/api/actions", get(list_actions))
        .route("/api/actions/:id", post(ack_action))
        .route("/api/scan", post(scan))
        .layer(middleware::from_fn(production_topology_gate))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit))
        .layer(middleware::from_fn(api_cache_headers));

    let app = Router::new()
        .merge(api)
        .route("/", get(index))
        .route("/demo", get(index))
        .route("/privacy", get(index))
        .route("/terms", get(index))
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .route_service(
            "/paper-cut-hero.webp",
            ServeFile::new("dist/paper-cut-hero.webp"),
        )
        .route_service("/social-card.jpg", ServeFile::new("dist/social-card.jpg"))
        .route_service("/favicon.svg", ServeFile::new("dist/favicon.svg"))
        .route_service(
            "/apple-touch-icon.png",
            ServeFile::new("dist/apple-touch-icon.png"),
        )
        .route_service("/robots.txt", ServeFile::new("dist/robots.txt"))
        .route_service("/sitemap.xml", ServeFile::new("dist/sitemap.xml"))
        .route_service("/404.css", ServeFile::new("dist/404.css"))
        .fallback(not_found)
        .layer(middleware::from_fn(site_headers))
        .with_state(state);
    let port = server_port(env::var("PORT").ok());
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("bind PORT");
    info!(port, "listening");
    if let Err(error) = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    {
        warn!(%error, "server stopped unexpectedly");
    } else {
        info!("server stopped gracefully");
    }
}

fn start_production_topology_guard() {
    if env::var("CONTAINER_APP_NAME").as_deref() != Ok("sf-integration-changelog-watch")
        || env::var("IDENTITY_ENDPOINT").is_err()
        || env::var("IDENTITY_HEADER").is_err()
    {
        // Local and consumer containers do not have Azure's managed identity
        // variables. Their explicit DATABASE_URL or mounted /data contract is
        // unchanged.
        return;
    }
    PRODUCTION_TOPOLOGY_READY.store(false, Ordering::SeqCst);
    tokio::spawn(async {
        loop {
            match reconcile_production_topology().await {
                Ok(true) => {
                    PRODUCTION_TOPOLOGY_READY.store(true, Ordering::SeqCst);
                    info!("production topology has one limiter owner and durable /data");
                    return;
                }
                Ok(false) => {
                    info!("production topology repair requested; waiting for mounted revision")
                }
                Err(error) => {
                    warn!(%error, "production topology is not ready; workspace APIs remain closed")
                }
            }
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });
}

async fn production_topology_gate(request: Request<axum::body::Body>, next: Next) -> Response {
    if request.uri().path() != "/health" && !PRODUCTION_TOPOLOGY_READY.load(Ordering::SeqCst) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            [
                (header::RETRY_AFTER, HeaderValue::from_static("5")),
                (header::CACHE_CONTROL, HeaderValue::from_static("no-store")),
            ],
            "Workspace storage is attaching. Try again in 5 seconds.",
        )
            .into_response();
    }
    next.run(request).await
}

async fn reconcile_production_topology() -> Result<bool, String> {
    let identity_endpoint = env::var("IDENTITY_ENDPOINT")
        .map_err(|_| "Azure managed identity endpoint is unavailable".to_owned())?;
    let identity_header = env::var("IDENTITY_HEADER")
        .map_err(|_| "Azure managed identity header is unavailable".to_owned())?;
    let resource_id = env::var("FACTORY_CONTAINER_APP_RESOURCE_ID")
        .unwrap_or_else(|_| PRODUCTION_RESOURCE_ID.to_owned());
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|error| error.to_string())?;
    let token_response = client
        .get(identity_endpoint)
        .query(&[
            ("resource", "https://management.azure.com/"),
            ("api-version", "2019-08-01"),
            ("client_id", PRODUCTION_IDENTITY_CLIENT_ID),
        ])
        .header("X-IDENTITY-HEADER", identity_header)
        .send()
        .await
        .map_err(|error| format!("could not request the deployment identity: {error}"))?;
    if !token_response.status().is_success() {
        return Err(format!(
            "deployment identity returned {}",
            token_response.status()
        ));
    }
    let token_json: serde_json::Value = token_response
        .json()
        .await
        .map_err(|error| format!("deployment identity response was invalid: {error}"))?;
    let token = token_json["access_token"]
        .as_str()
        .ok_or_else(|| "deployment identity returned no access token".to_owned())?;
    let resource_url = format!("https://management.azure.com{resource_id}?api-version=2024-03-01");
    let resource_response = client
        .get(&resource_url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|error| format!("could not read the Container App topology: {error}"))?;
    if !resource_response.status().is_success() {
        return Err(format!(
            "Container App topology read returned {}",
            resource_response.status()
        ));
    }
    let resource: serde_json::Value = resource_response
        .json()
        .await
        .map_err(|error| format!("Container App topology response was invalid: {error}"))?;
    if topology_is_durable(&resource) && data_mount_is_active() {
        return Ok(true);
    }
    let patch = durable_topology_patch(&resource)?;
    let patch_response = client
        .patch(resource_url)
        .bearer_auth(token)
        .json(&patch)
        .send()
        .await
        .map_err(|error| format!("could not repair the Container App topology: {error}"))?;
    if !patch_response.status().is_success() {
        return Err(format!(
            "Container App topology repair returned {}",
            patch_response.status()
        ));
    }
    Ok(false)
}

fn topology_is_durable(resource: &serde_json::Value) -> bool {
    let template = &resource["properties"]["template"];
    let scale = &template["scale"];
    let has_volume = template["volumes"].as_array().is_some_and(|volumes| {
        volumes.iter().any(|volume| {
            volume["name"] == "workspace-data"
                && volume["storageType"] == "AzureFile"
                && volume["storageName"] == "integration-changelog-watch-data"
        })
    });
    let has_mount = template["containers"].as_array().is_some_and(|containers| {
        containers.iter().any(|container| {
            container["name"] == "app"
                && container["volumeMounts"].as_array().is_some_and(|mounts| {
                    mounts.iter().any(|mount| {
                        mount["volumeName"] == "workspace-data" && mount["mountPath"] == "/data"
                    })
                })
        })
    });
    scale["minReplicas"] == 1 && scale["maxReplicas"] == 1 && has_volume && has_mount
}

fn durable_topology_patch(resource: &serde_json::Value) -> Result<serde_json::Value, String> {
    let mut container = resource["properties"]["template"]["containers"]
        .as_array()
        .and_then(|containers| {
            containers
                .iter()
                .find(|container| container["name"] == "app")
        })
        .cloned()
        .ok_or_else(|| "Container App has no app container to repair".to_owned())?;
    container["volumeMounts"] = serde_json::json!([
        {"volumeName": "workspace-data", "mountPath": "/data"}
    ]);
    Ok(serde_json::json!({
        "properties": {
            "template": {
                "terminationGracePeriodSeconds": 30,
                "scale": {"minReplicas": 1, "maxReplicas": 1},
                "volumes": [{
                    "name": "workspace-data",
                    "storageType": "AzureFile",
                    "storageName": "integration-changelog-watch-data"
                }],
                "containers": [container]
            }
        }
    }))
}

fn data_mount_is_active() -> bool {
    std::fs::read_to_string("/proc/self/mountinfo")
        .map(|mounts| mountinfo_has_data_mount(&mounts))
        .unwrap_or(false)
}

fn mountinfo_has_data_mount(mounts: &str) -> bool {
    mounts.lines().any(|line| {
        line.split_whitespace()
            .nth(4)
            .is_some_and(|mount_point| mount_point == "/data")
    })
}

/// Finish active requests before a Container App revision replacement. Both
/// signals are handled because local development and the production runtime
/// use different process supervisors.
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("install SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => info!("SIGINT received; draining server"),
            _ = terminate.recv() => info!("SIGTERM received; draining server"),
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .expect("install Ctrl+C handler");
        info!("SIGINT received; draining server");
    }
}

fn default_database_url() -> String {
    // Azure Files does not implement SQLite's byte-range lock protocol. Its
    // dot-file VFS keeps SQLite's lock files in the mounted share instead.
    // The deployment still enforces one replica for state and rate limits.
    "sqlite:/data/changelog-watch.db?mode=rwc&vfs=unix-dotfile".to_owned()
}

fn server_port(value: Option<String>) -> u16 {
    value.and_then(|value| value.parse().ok()).unwrap_or(8080)
}

async fn setup(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workspaces(id INTEGER PRIMARY KEY, token_hash TEXT NOT NULL UNIQUE, created_at TEXT NOT NULL);
         CREATE TABLE IF NOT EXISTS workspace_watches(id INTEGER PRIMARY KEY, workspace_id INTEGER NOT NULL, vendor TEXT NOT NULL, url TEXT NOT NULL, keywords TEXT NOT NULL, owner TEXT NOT NULL, version TEXT NOT NULL, command TEXT NOT NULL, last_hash TEXT, last_scanned TEXT, FOREIGN KEY(workspace_id) REFERENCES workspaces(id));
         CREATE TABLE IF NOT EXISTS workspace_actions(id INTEGER PRIMARY KEY, workspace_id INTEGER NOT NULL, watch_id INTEGER NOT NULL, notice_key TEXT NOT NULL, title TEXT NOT NULL, excerpt TEXT NOT NULL, matched TEXT NOT NULL, url TEXT NOT NULL, owner TEXT NOT NULL, version TEXT NOT NULL DEFAULT '', command TEXT NOT NULL, acknowledged INTEGER NOT NULL DEFAULT 0, seen_at TEXT NOT NULL, UNIQUE(workspace_id, watch_id, notice_key), FOREIGN KEY(workspace_id) REFERENCES workspaces(id));
         CREATE INDEX IF NOT EXISTS workspace_watches_owner ON workspace_watches(workspace_id);
         CREATE INDEX IF NOT EXISTS workspace_actions_owner ON workspace_actions(workspace_id);",
    )
    .execute(db)
    .await?;
    // Existing durable workspaces predate the action-card version snapshot.
    // SQLite has no ADD COLUMN IF NOT EXISTS, so an already-migrated database
    // simply reports the duplicate-column error and remains usable.
    let _ =
        sqlx::query("ALTER TABLE workspace_actions ADD COLUMN version TEXT NOT NULL DEFAULT ''")
            .execute(db)
            .await;
    Ok(())
}

async fn health(State(app): State<App>) -> impl IntoResponse {
    Json(serde_json::json!({"ok": true, "build": app.build}))
}

async fn index(State(app): State<App>) -> impl IntoResponse {
    let page = rendered_file("dist/index.html", &app.build, "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>Integration Changelog Watch</title></head><body><main id=\"main\"><h1>Integration Changelog Watch</h1></main></body></html>");
    ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], page)
}

async fn create_workspace(State(app): State<App>) -> ApiResult<(StatusCode, Json<Workspace>)> {
    let token = Uuid::new_v4().simple().to_string() + &Uuid::new_v4().simple().to_string();
    sqlx::query("INSERT INTO workspaces(token_hash, created_at) VALUES(?, ?)")
        .bind(token_hash(&token))
        .bind(Utc::now().to_rfc3339())
        .execute(&app.db)
        .await
        .map_err(internal)?;
    Ok((StatusCode::CREATED, Json(Workspace { token })))
}

async fn workspace_id(headers: &HeaderMap, app: &App) -> ApiResult<i64> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| token.len() >= 32)
        .ok_or_else(|| {
            ApiError(
                StatusCode::UNAUTHORIZED,
                "Create or restore this browser workspace before using the API.".to_owned(),
            )
        })?;
    sqlx::query_scalar("SELECT id FROM workspaces WHERE token_hash=?")
        .bind(token_hash(token))
        .fetch_optional(&app.db)
        .await
        .map_err(internal)?
        .ok_or_else(|| {
            ApiError(
                StatusCode::UNAUTHORIZED,
                "This workspace token is not active on the server.".to_owned(),
            )
        })
}

async fn list_watches(State(app): State<App>, headers: HeaderMap) -> ApiResult<Json<Vec<Watch>>> {
    let workspace = workspace_id(&headers, &app).await?;
    let watches = sqlx::query_as("SELECT id, vendor, url, keywords, owner, version, command, last_scanned FROM workspace_watches WHERE workspace_id=? ORDER BY id DESC")
        .bind(workspace)
        .fetch_all(&app.db)
        .await
        .map_err(internal)?;
    Ok(Json(watches))
}

async fn add_watch(
    State(app): State<App>,
    headers: HeaderMap,
    Json(new): Json<NewWatch>,
) -> ApiResult<(StatusCode, Json<Watch>)> {
    let workspace = workspace_id(&headers, &app).await?;
    validate_watch(&new).await?;
    let watch = insert_watch_under_limit(&app.db, workspace, &new).await?;
    Ok((StatusCode::CREATED, Json(watch)))
}

/// Replacing watches is intentionally one server-side operation. The browser
/// cannot fully validate a public address because DNS and private-network
/// policy are server concerns; checking the complete file before beginning a
/// transaction means a rejected import cannot erase a working dashboard.
async fn replace_watches(
    State(app): State<App>,
    headers: HeaderMap,
    Json(import): Json<WatchImport>,
) -> ApiResult<Json<Vec<Watch>>> {
    let workspace = workspace_id(&headers, &app).await?;
    validate_watch_import(&import.watches).await?;

    let mut transaction = app.db.begin().await.map_err(internal)?;
    sqlx::query("DELETE FROM workspace_actions WHERE workspace_id=?")
        .bind(workspace)
        .execute(&mut *transaction)
        .await
        .map_err(internal)?;
    sqlx::query("DELETE FROM workspace_watches WHERE workspace_id=?")
        .bind(workspace)
        .execute(&mut *transaction)
        .await
        .map_err(internal)?;

    let mut saved = Vec::with_capacity(import.watches.len());
    for watch in &import.watches {
        let inserted = sqlx::query(
            "INSERT INTO workspace_watches(workspace_id,vendor,url,keywords,owner,version,command) VALUES(?,?,?,?,?,?,?)",
        )
        .bind(workspace)
        .bind(&watch.vendor)
        .bind(&watch.url)
        .bind(&watch.keywords)
        .bind(&watch.owner)
        .bind(&watch.version)
        .bind(&watch.command)
        .execute(&mut *transaction)
        .await
        .map_err(internal)?;
        let saved_watch = sqlx::query_as::<_, Watch>("SELECT id, vendor, url, keywords, owner, version, command, last_scanned FROM workspace_watches WHERE id=? AND workspace_id=?")
            .bind(inserted.last_insert_rowid())
            .bind(workspace)
            .fetch_one(&mut *transaction)
            .await
            .map_err(internal)?;
        saved.push(saved_watch);
    }
    transaction.commit().await.map_err(internal)?;
    Ok(Json(saved))
}

/// The quota condition lives in the INSERT statement, rather than in a prior
/// read. SQLite serializes this statement with competing writers, so two
/// simultaneous creates cannot both observe a spare fourth slot.
async fn insert_watch_under_limit(
    db: &SqlitePool,
    workspace: i64,
    new: &NewWatch,
) -> ApiResult<Watch> {
    let result = sqlx::query(
        "INSERT INTO workspace_watches(workspace_id,vendor,url,keywords,owner,version,command)
         SELECT ?,?,?,?,?,?,?
         WHERE (SELECT count(*) FROM workspace_watches WHERE workspace_id=?) < 3",
    )
    .bind(workspace)
    .bind(&new.vendor)
    .bind(&new.url)
    .bind(&new.keywords)
    .bind(&new.owner)
    .bind(&new.version)
    .bind(&new.command)
    .bind(workspace)
    .execute(db)
    .await
    .map_err(internal)?;
    if result.rows_affected() != 1 {
        return Err(ApiError(StatusCode::CONFLICT, "This workspace already has three watches. Edit an existing watch before adding another.".to_owned()));
    }
    let id = result.last_insert_rowid();
    let watch = sqlx::query_as("SELECT id, vendor, url, keywords, owner, version, command, last_scanned FROM workspace_watches WHERE id=? AND workspace_id=?")
        .bind(id).bind(workspace).fetch_one(db).await.map_err(internal)?;
    Ok(watch)
}

async fn update_watch(
    State(app): State<App>,
    headers: HeaderMap,
    Path(id): Path<i64>,
    Json(new): Json<NewWatch>,
) -> ApiResult<Json<Watch>> {
    let workspace = workspace_id(&headers, &app).await?;
    validate_watch(&new).await?;
    let changed = sqlx::query("UPDATE workspace_watches SET vendor=?,url=?,keywords=?,owner=?,version=?,command=?,last_hash=NULL,last_scanned=NULL WHERE id=? AND workspace_id=?")
        .bind(&new.vendor).bind(&new.url).bind(&new.keywords).bind(&new.owner).bind(&new.version).bind(&new.command).bind(id).bind(workspace)
        .execute(&app.db).await.map_err(internal)?;
    if changed.rows_affected() != 1 {
        return Err(ApiError(
            StatusCode::NOT_FOUND,
            "That watch does not exist in this workspace.".to_owned(),
        ));
    }
    let watch = sqlx::query_as("SELECT id, vendor, url, keywords, owner, version, command, last_scanned FROM workspace_watches WHERE id=? AND workspace_id=?")
        .bind(id).bind(workspace).fetch_one(&app.db).await.map_err(internal)?;
    Ok(Json(watch))
}

async fn delete_watch(
    State(app): State<App>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    let workspace = workspace_id(&headers, &app).await?;
    let mut tx = app.db.begin().await.map_err(internal)?;
    sqlx::query("DELETE FROM workspace_actions WHERE watch_id=? AND workspace_id=?")
        .bind(id)
        .bind(workspace)
        .execute(&mut *tx)
        .await
        .map_err(internal)?;
    let changed = sqlx::query("DELETE FROM workspace_watches WHERE id=? AND workspace_id=?")
        .bind(id)
        .bind(workspace)
        .execute(&mut *tx)
        .await
        .map_err(internal)?;
    if changed.rows_affected() != 1 {
        return Err(ApiError(
            StatusCode::NOT_FOUND,
            "That watch does not exist in this workspace.".to_owned(),
        ));
    }
    tx.commit().await.map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_actions(State(app): State<App>, headers: HeaderMap) -> ApiResult<Json<Vec<Action>>> {
    let workspace = workspace_id(&headers, &app).await?;
    let actions = sqlx::query_as("SELECT id, watch_id, title, excerpt, matched, url, owner, version, command, acknowledged, seen_at FROM workspace_actions WHERE workspace_id=? ORDER BY acknowledged, id DESC")
        .bind(workspace).fetch_all(&app.db).await.map_err(internal)?;
    Ok(Json(actions))
}

async fn ack_action(
    State(app): State<App>,
    headers: HeaderMap,
    Path(id): Path<i64>,
    Json(ack): Json<Ack>,
) -> ApiResult<Json<Action>> {
    let workspace = workspace_id(&headers, &app).await?;
    let changed =
        sqlx::query("UPDATE workspace_actions SET acknowledged=? WHERE id=? AND workspace_id=?")
            .bind(ack.acknowledged)
            .bind(id)
            .bind(workspace)
            .execute(&app.db)
            .await
            .map_err(internal)?;
    if changed.rows_affected() != 1 {
        return Err(ApiError(
            StatusCode::NOT_FOUND,
            "That action does not exist in this workspace.".to_owned(),
        ));
    }
    let action = sqlx::query_as("SELECT id, watch_id, title, excerpt, matched, url, owner, version, command, acknowledged, seen_at FROM workspace_actions WHERE id=? AND workspace_id=?")
        .bind(id).bind(workspace).fetch_one(&app.db).await.map_err(internal)?;
    Ok(Json(action))
}

async fn scan(State(app): State<App>, headers: HeaderMap) -> ApiResult<Json<ScanResult>> {
    let workspace = workspace_id(&headers, &app).await?;
    let watches: Vec<WatchRow> = sqlx::query_as("SELECT id, vendor, url, keywords, owner, version, command, last_hash FROM workspace_watches WHERE workspace_id=?")
        .bind(workspace).fetch_all(&app.db).await.map_err(internal)?;
    let mut made = 0;
    let mut failures = Vec::new();
    for watch in watches {
        let text = match fetch_public(&watch.url).await {
            Ok(text) => text,
            Err(message) => {
                failures.push(format!("{}: {message}", watch.vendor));
                continue;
            }
        };
        let hash = format!("{:x}", Sha256::digest(text.as_bytes()));
        if watch.last_hash.as_deref() != Some(&hash) {
            made += record_matches(&app.db, workspace, &watch, parse_notices(&text, &watch.url))
                .await?;
        }
        sqlx::query("UPDATE workspace_watches SET last_hash=?, last_scanned=? WHERE id=? AND workspace_id=?")
            .bind(hash).bind(Utc::now().to_rfc3339()).bind(watch.id).bind(workspace).execute(&app.db).await.map_err(internal)?;
    }
    let message = if failures.is_empty() {
        format!("Scan complete. {made} new action card(s).")
    } else {
        format!(
            "Scan finished with {} feed error(s). Fix the listed address and scan again.",
            failures.len()
        )
    };
    Ok(Json(ScanResult {
        new_actions: made,
        failures,
        message,
    }))
}

/// The feed transport and the match recorder are separate so the exact action
/// creation path can be checked against a shipped fixture without a live
/// vendor request. Production scans call this after the same public-feed
/// validation and response policy used above.
async fn record_matches(
    db: &SqlitePool,
    workspace: i64,
    watch: &WatchRow,
    notices: Vec<Notice>,
) -> ApiResult<usize> {
    let mut made = 0;
    for notice in notices {
        let body = format!("{} {}", notice.title, notice.excerpt).to_lowercase();
        if let Some(keyword) = watch
            .keywords
            .split(',')
            .map(str::trim)
            .filter(|keyword| !keyword.is_empty())
            .find(|keyword| body.contains(&keyword.to_lowercase()))
        {
            let key = format!(
                "{:x}",
                Sha256::digest(format!("{}\n{}", notice.title, notice.url).as_bytes()),
            );
            let title = if notice.title.is_empty() {
                format!("Matched change from {}", watch.vendor)
            } else {
                notice.title
            };
            let inserted = sqlx::query("INSERT OR IGNORE INTO workspace_actions(workspace_id,watch_id,notice_key,title,excerpt,matched,url,owner,version,command,seen_at) VALUES(?,?,?,?,?,?,?,?,?,?,?)")
                .bind(workspace).bind(watch.id).bind(key).bind(title).bind(notice.excerpt.chars().take(420).collect::<String>()).bind(keyword).bind(notice.url).bind(&watch.owner).bind(&watch.version).bind(&watch.command).bind(Utc::now().to_rfc3339())
                .execute(db).await.map_err(internal)?;
            made += inserted.rows_affected() as usize;
        }
    }
    Ok(made)
}

#[derive(FromRow)]
struct WatchRow {
    id: i64,
    vendor: String,
    url: String,
    keywords: String,
    owner: String,
    version: String,
    command: String,
    last_hash: Option<String>,
}

#[derive(Debug)]
struct Notice {
    title: String,
    excerpt: String,
    url: String,
}

fn parse_notices(text: &str, source_url: &str) -> Vec<Notice> {
    if let Some(notices) = parse_xml_notices(text, source_url) {
        return notices;
    }

    let document = Html::parse_document(text);
    let entry = Selector::parse("item, entry").expect("valid selector");
    let title = Selector::parse("title").expect("valid selector");
    let description = Selector::parse("description, summary, content").expect("valid selector");
    let link = Selector::parse("link").expect("valid selector");
    let mut notices: Vec<Notice> = document
        .select(&entry)
        .map(|item| {
            let item_title = item.select(&title).next().map(text_of).unwrap_or_default();
            let excerpt = item
                .select(&description)
                .next()
                .map(text_of)
                .unwrap_or_default();
            let item_url = item
                .select(&link)
                .next()
                .and_then(|link| {
                    link.value().attr("href").map(str::to_owned).or_else(|| {
                        let value = text_of(link);
                        (!value.is_empty()).then_some(value)
                    })
                })
                .unwrap_or_else(|| source_url.to_owned());
            Notice {
                title: item_title,
                excerpt,
                url: absolute_url(source_url, &item_url),
            }
        })
        .collect();
    if notices.is_empty() {
        let headings = Selector::parse("h1, h2, h3").expect("valid selector");
        notices = document
            .select(&headings)
            .map(|heading| Notice {
                title: text_of(heading),
                excerpt: String::new(),
                url: source_url.to_owned(),
            })
            .filter(|notice| !notice.title.is_empty())
            .collect();
    }
    notices
}

fn parse_xml_notices(text: &str, source_url: &str) -> Option<Vec<Notice>> {
    let document = roxmltree::Document::parse(text).ok()?;
    let entries: Vec<_> = document
        .descendants()
        .filter(|node| node.is_element() && matches!(node.tag_name().name(), "item" | "entry"))
        .collect();
    if entries.is_empty() {
        return None;
    }
    Some(
        entries
            .into_iter()
            .map(|entry| {
                let child = |names: &[&str]| {
                    entry
                        .children()
                        .find(|node| node.is_element() && names.contains(&node.tag_name().name()))
                };
                let title = child(&["title"]).map(xml_node_text).unwrap_or_default();
                let excerpt = child(&["description", "summary", "content"])
                    .map(xml_node_text)
                    .unwrap_or_default();
                let item_url = child(&["link"])
                    .and_then(|link| {
                        link.attribute("href")
                            .map(str::to_owned)
                            .or_else(|| nonempty(xml_node_text(link)))
                    })
                    .unwrap_or_else(|| source_url.to_owned());
                Notice {
                    title,
                    excerpt,
                    url: absolute_url(source_url, &item_url),
                }
            })
            .collect(),
    )
}

fn xml_node_text(node: roxmltree::Node<'_, '_>) -> String {
    let raw = node
        .descendants()
        .filter(|descendant| descendant.is_text())
        .filter_map(|descendant| descendant.text())
        .collect::<Vec<_>>()
        .join(" ");
    let fragment = Html::parse_fragment(&raw);
    fragment
        .root_element()
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn nonempty(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}

fn text_of(element: scraper::ElementRef<'_>) -> String {
    element
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn absolute_url(source: &str, notice: &str) -> String {
    Url::parse(source)
        .ok()
        .and_then(|source| source.join(notice).ok())
        .map(|url| url.to_string())
        .unwrap_or_else(|| source.to_owned())
}

async fn validate_watch(watch: &NewWatch) -> ApiResult<()> {
    if [
        watch.vendor.trim(),
        watch.keywords.trim(),
        watch.owner.trim(),
        watch.command.trim(),
    ]
    .iter()
    .any(|value| value.is_empty())
    {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "Provide a vendor, public URL, keywords, owner, and check command.".to_owned(),
        ));
    }
    let fields = [
        ("vendor", watch.vendor.len(), 120usize),
        ("public URL", watch.url.len(), 2048),
        ("keywords", watch.keywords.len(), 500),
        ("owner", watch.owner.len(), 160),
        ("version", watch.version.len(), 120),
        ("check command", watch.command.len(), 500),
    ];
    if let Some((name, _, limit)) = fields.into_iter().find(|(_, size, limit)| size > limit) {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            format!("The {name} is too long. Keep it to {limit} characters or fewer."),
        ));
    }
    resolve_public_url(&watch.url)
        .await
        .map(|_| ())
        .map_err(|message| ApiError(StatusCode::BAD_REQUEST, message))
}

async fn validate_watch_import(watches: &[NewWatch]) -> ApiResult<()> {
    if watches.is_empty() || watches.len() > 3 {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "Import one to three watches into the hosted workspace.".to_owned(),
        ));
    }
    for watch in watches {
        validate_watch(watch).await?;
    }
    Ok(())
}

async fn fetch_public(value: &str) -> Result<String, String> {
    let (url, addresses) = resolve_public_url(value).await?;
    let host = url
        .host_str()
        .ok_or_else(|| "The feed URL has no host.".to_owned())?;
    let client = Client::builder()
        .redirect(Policy::none())
        .resolve_to_addrs(host, &addresses)
        .user_agent("Integration-Changelog-Watch/1.0 (+configured-by-owner)")
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|_| "Could not prepare a safe feed request.".to_owned())?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|_| "Could not reach this public feed.".to_owned())?;
    ensure_feed_response_is_safe(response.status())?;
    response
        .text()
        .await
        .map_err(|_| "Could not read this feed response.".to_owned())
}

fn ensure_feed_response_is_safe(status: StatusCode) -> Result<(), String> {
    if status.is_redirection() {
        return Err("This feed redirects. Use its final public HTTPS address instead.".to_owned());
    }
    if !status.is_success() {
        return Err("The feed returned an error response.".to_owned());
    }
    Ok(())
}

async fn resolve_public_url(value: &str) -> Result<(Url, Vec<SocketAddr>), String> {
    let url =
        Url::parse(value).map_err(|_| "Enter a complete public http or https URL.".to_owned())?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err("Enter a complete public http or https URL without credentials.".to_owned());
    }
    let host = url
        .host_str()
        .ok_or_else(|| "The feed URL has no host.".to_owned())?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| "The feed URL has no usable port.".to_owned())?;
    let addresses: Vec<SocketAddr> = tokio::net::lookup_host((host, port))
        .await
        .map_err(|_| "The feed host could not be resolved.".to_owned())?
        .collect();
    if addresses.is_empty() || addresses.iter().any(|address| forbidden_ip(address.ip())) {
        return Err("Use a public internet address; private, loopback, and link-local networks are blocked.".to_owned());
    }
    Ok((url, addresses))
}

fn forbidden_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            ip.is_private()
                || ip.is_loopback()
                || ip.is_link_local()
                || ip.is_broadcast()
                || ip.is_unspecified()
                || ip.is_multicast()
                || ip.is_documentation()
        }
        IpAddr::V6(ip) => {
            ip.is_loopback()
                || ip.is_unspecified()
                || ip.is_multicast()
                || ip.is_unicast_link_local()
                || ip.is_unique_local()
        }
    }
}

fn token_hash(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}
fn internal(_: sqlx::Error) -> ApiError {
    ApiError(
        StatusCode::INTERNAL_SERVER_ERROR,
        "The server could not complete that request. Try again.".to_owned(),
    )
}

/// The factory ingress sanitizes `X-Forwarded-For`; its first hop is the client.
/// Later values describe proxies and cannot select a different bucket.
async fn rate_limit(
    State(app): State<App>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let socket_peer = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|peer| peer.0.ip())
        // This fallback keeps a direct unit/router invocation bounded as well.
        .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED));
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .and_then(|value| value.parse::<IpAddr>().ok())
        .unwrap_or(socket_peer);
    let now = std::time::Instant::now();
    let retry_after = {
        let mut buckets = app.limiter.lock().expect("rate limiter mutex");
        let bucket = buckets.entry(client_ip).or_insert(RateBucket {
            tokens: 40.0,
            refreshed: now,
        });
        // Refill on whole-second boundaries. Besides making Retry-After easy
        // to explain, this preserves the advertised 40-request burst when a
        // real ingress fans one client burst over a few dozen milliseconds.
        let elapsed_seconds = now.duration_since(bucket.refreshed).as_secs();
        if elapsed_seconds > 0 {
            bucket.tokens = (bucket.tokens + elapsed_seconds as f64 * 20.0).min(40.0);
            bucket.refreshed += Duration::from_secs(elapsed_seconds);
        }
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            None
        } else {
            Some(((1.0 - bucket.tokens) / 20.0).ceil().max(1.0) as u64)
        }
    };
    if let Some(seconds) = retry_after {
        let mut response = (
            StatusCode::TOO_MANY_REQUESTS,
            format!("Too many requests. Try again in {seconds} second(s)."),
        )
            .into_response();
        response.headers_mut().insert(
            header::RETRY_AFTER,
            HeaderValue::from_str(&seconds.to_string()).expect("retry header"),
        );
        return response;
    }
    next.run(request).await
}

async fn api_cache_headers(request: Request<axum::body::Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, private"),
    );
    response
}

async fn site_headers(request: Request<axum::body::Body>, next: Next) -> Response {
    let path = request.uri().path().to_owned();
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static("default-src 'self'; connect-src 'self'; img-src 'self'; style-src 'self'; script-src 'self'; base-uri 'self'; frame-ancestors 'none'"));
    headers.insert(
        "permissions-policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=(), payment=()"),
    );
    headers.insert(
        "strict-transport-security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    let cache = if path.starts_with("/assets/") {
        "public, max-age=31536000, immutable"
    } else if path.ends_with(".webp") || path.ends_with(".png") || path.ends_with(".svg") {
        "public, max-age=604800"
    } else {
        "no-cache"
    };
    if !headers.contains_key(header::CACHE_CONTROL) {
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static(cache));
    }
    response
}

async fn not_found(State(app): State<App>) -> impl IntoResponse {
    let page = rendered_file("dist/404.html", &app.build, "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>Page not found — Integration Changelog Watch</title></head><body><main id=\"main\"><h1>That page is not here</h1><p><a href=\"/\">Return home</a></p></main></body></html>");
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        page,
    )
}

fn rendered_file(path: &str, build: &str, fallback: &str) -> String {
    // BUILD_SHA is supplied by the image build, but escaping keeps this server
    // safe and usable if a developer sets it manually.
    let escaped_build = build
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;");
    std::fs::read_to_string(path)
        .unwrap_or_else(|_| fallback.to_owned())
        .replace("{{BUILD_ID}}", &escaped_build)
}

fn print_help() {
    println!("Integration Changelog Watch\n\nUsage:\n  integration-changelog-watch                         Start the dashboard server\n  integration-changelog-watch demo                    Print shipped Markdown action cards\n  integration-changelog-watch scan --config FILE     Scan a repository watch mapping into Markdown files\n  integration-changelog-watch ack --config FILE --id ID  Record an action acknowledgement\n  integration-changelog-watch --help                  Show this help\n\nThe dashboard uses a private browser workspace token. The demo command makes no network request.");
}

fn print_demo_markdown() {
    println!("# Integration changelog watch demo\n\n## Stripe retires legacy webhook event format\n\n- **Status:** Needs acknowledgement\n- **Matched rule:** webhook\n- **Owner:** Maya · Payments\n- **Affected dependency:** stripe-node 16.2\n- **Check:** `pnpm test:stripe`\n\nReview signature parsing and event fixtures.\n\n## Auth0 changes refresh token rotation defaults\n\n- **Status:** Needs acknowledgement\n- **Matched rule:** token\n- **Owner:** Ishan · Identity\n- **Affected dependency:** auth0-spa-js 2.1\n- **Check:** `pnpm test:auth`\n\nCheck explicit configuration before the next environment.");
}

async fn cli_scan(path: &str) -> Result<(), String> {
    let source = std::fs::read_to_string(path).map_err(|_| format!("could not read {path}"))?;
    let config: CliConfig = serde_json::from_str(&source)
        .map_err(|_| "the watch mapping is not valid JSON".to_owned())?;
    let config_path = FilePath::new(path);
    let config_dir = config_path.parent().unwrap_or_else(|| FilePath::new("."));
    let state_dir = cli_state_dir(config_dir, config.state_dir.as_deref());
    let mut state = read_cli_state(&state_dir)?;
    let mut cards = Vec::new();
    for watch in &config.watches {
        let text = cli_watch_text(watch, config_dir).await?;
        for notice in parse_notices(&text, &watch.url) {
            let body = format!("{} {}", notice.title, notice.excerpt).to_lowercase();
            if let Some(rule) = watch
                .keywords
                .split(',')
                .map(str::trim)
                .find(|rule| !rule.is_empty() && body.contains(&rule.to_lowercase()))
            {
                let title = if notice.title.is_empty() {
                    format!("Matched change from {}", watch.vendor)
                } else {
                    notice.title
                };
                let notice_hash = format!(
                    "{:x}",
                    Sha256::digest(format!("{}\n{}", title, notice.url).as_bytes())
                );
                if state
                    .actions
                    .iter()
                    .any(|action| action.notice_hash == notice_hash)
                {
                    continue;
                }
                let id = notice_hash[..12].to_owned();
                let dependency = if watch.version.trim().is_empty() {
                    "Not recorded"
                } else {
                    &watch.version
                };
                let card = format!("# {title}\n\n- **Action ID:** `{id}`\n- **Status:** Needs acknowledgement\n- **Matched rule:** {rule}\n- **Owner:** {}\n- **Affected dependency:** {dependency}\n- **Check:** `{}`\n- **Notice:** {}\n\n{}\n", watch.owner, watch.command, notice.url, notice.excerpt);
                std::fs::create_dir_all(state_dir.join("actions")).map_err(|_| {
                    "could not create the repository action-card directory".to_owned()
                })?;
                std::fs::write(state_dir.join("actions").join(format!("{id}.md")), &card)
                    .map_err(|_| "could not write the repository action card".to_owned())?;
                state.actions.push(CliAction {
                    id: id.clone(),
                    notice_hash,
                    acknowledged: false,
                });
                cards.push(format!(
                    "Created {}",
                    state_dir.join("actions").join(format!("{id}.md")).display()
                ));
            }
        }
    }
    write_cli_state(&state_dir, &state)?;
    if cards.is_empty() {
        println!("# Integration changelog scan\n\nNo new matching notices found.");
    } else {
        println!("# Integration changelog scan\n\n{}", cards.join("\n"));
    }
    Ok(())
}

async fn cli_watch_text(watch: &NewWatch, config_dir: &FilePath) -> Result<String, String> {
    if matches!(
        Url::parse(&watch.url)
            .ok()
            .map(|url| url.scheme().to_owned())
            .as_deref(),
        Some("http") | Some("https")
    ) {
        validate_watch(watch).await.map_err(|error| error.1)?;
        return fetch_public(&watch.url).await;
    }
    let local_path = config_dir.join(&watch.url);
    std::fs::read_to_string(&local_path)
        .map_err(|_| format!("could not read local feed fixture {}", local_path.display()))
}

fn cli_state_dir(config_dir: &FilePath, configured: Option<&str>) -> PathBuf {
    configured
        .map(|value| config_dir.join(value))
        .unwrap_or_else(|| config_dir.join(".integration-changelog-watch"))
}

fn read_cli_state(state_dir: &FilePath) -> Result<CliState, String> {
    let path = state_dir.join("state.json");
    match std::fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents)
            .map_err(|_| "the CLI state file is not valid JSON".to_owned()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(CliState::default()),
        Err(_) => Err("could not read the CLI state file".to_owned()),
    }
}

fn write_cli_state(state_dir: &FilePath, state: &CliState) -> Result<(), String> {
    std::fs::create_dir_all(state_dir)
        .map_err(|_| "could not create the CLI state directory".to_owned())?;
    let json =
        serde_json::to_string_pretty(state).map_err(|_| "could not encode CLI state".to_owned())?;
    std::fs::write(state_dir.join("state.json"), format!("{json}\n"))
        .map_err(|_| "could not write the CLI state file".to_owned())
}

fn cli_ack(path: &str, id: &str) -> Result<(), String> {
    let source = std::fs::read_to_string(path).map_err(|_| format!("could not read {path}"))?;
    let config: CliConfig = serde_json::from_str(&source)
        .map_err(|_| "the watch mapping is not valid JSON".to_owned())?;
    let config_dir = FilePath::new(path)
        .parent()
        .unwrap_or_else(|| FilePath::new("."));
    let state_dir = cli_state_dir(config_dir, config.state_dir.as_deref());
    let mut state = read_cli_state(&state_dir)?;
    let card_path = state_dir.join("actions").join(format!("{id}.md"));
    let card = std::fs::read_to_string(&card_path)
        .map_err(|_| "could not read the repository action card".to_owned())?;
    let updated_card = card.replace(
        "**Status:** Needs acknowledgement",
        "**Status:** Acknowledged",
    );
    if updated_card == card && !card.contains("**Status:** Acknowledged") {
        return Err("the repository action card has no acknowledgement status".to_owned());
    }
    let action = state
        .actions
        .iter_mut()
        .find(|action| action.id == id)
        .ok_or_else(|| "that action ID does not exist in this repository state".to_owned())?;
    std::fs::write(card_path, updated_card)
        .map_err(|_| "could not update the repository action card".to_owned())?;
    action.acknowledged = true;
    write_cli_state(&state_dir, &state)?;
    println!("Acknowledged action {id}.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    fn bearer(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {token}").parse().unwrap(),
        );
        headers
    }

    #[tokio::test]
    async fn workspace_rows_are_private_and_actions_use_the_dashboard_schema() {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        setup(&db).await.unwrap();
        let first = "a".repeat(64);
        let second = "b".repeat(64);
        sqlx::query("INSERT INTO workspaces(token_hash,created_at) VALUES(?,?), (?,?)")
            .bind(token_hash(&first))
            .bind("now")
            .bind(token_hash(&second))
            .bind("now")
            .execute(&db)
            .await
            .unwrap();
        let first_id: i64 = sqlx::query_scalar("SELECT id FROM workspaces WHERE token_hash=?")
            .bind(token_hash(&first))
            .fetch_one(&db)
            .await
            .unwrap();
        sqlx::query("INSERT INTO workspace_watches(workspace_id,vendor,url,keywords,owner,version,command) VALUES(?,?,?,?,?,?,?)")
            .bind(first_id).bind("Vendor").bind("https://vendor.example/feed").bind("webhook").bind("Maya").bind("").bind("npm test")
            .execute(&db).await.unwrap();
        let watch_id: i64 = sqlx::query_scalar("SELECT id FROM workspace_watches")
            .fetch_one(&db)
            .await
            .unwrap();
        sqlx::query("INSERT INTO workspace_actions(workspace_id,watch_id,notice_key,title,excerpt,matched,url,owner,command,seen_at) VALUES(?,?,?,?,?,?,?,?,?,?)")
            .bind(first_id).bind(watch_id).bind("notice").bind("Webhook change").bind("Read this").bind("webhook").bind("https://vendor.example/notice").bind("Maya").bind("npm test").bind("Today")
            .execute(&db).await.unwrap();
        let app = App {
            db,
            build: "test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };
        assert_eq!(
            list_watches(State(app.clone()), bearer(&second))
                .await
                .unwrap()
                .0
                .len(),
            0
        );
        let actions = list_actions(State(app), bearer(&first)).await.unwrap().0;
        let json = serde_json::to_value(&actions[0]).unwrap();
        assert_eq!(json["url"], "https://vendor.example/notice");
        assert_eq!(json["seenAt"], "Today");
        assert_eq!(json["version"], "");
        assert!(json.get("source_url").is_none());
    }

    #[tokio::test]
    async fn rejected_watch_import_preserves_existing_workspace_watches() {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        setup(&db).await.unwrap();
        let token = "i".repeat(64);
        sqlx::query("INSERT INTO workspaces(token_hash,created_at) VALUES(?,?)")
            .bind(token_hash(&token))
            .bind("now")
            .execute(&db)
            .await
            .unwrap();
        let workspace: i64 = sqlx::query_scalar("SELECT id FROM workspaces")
            .fetch_one(&db)
            .await
            .unwrap();
        sqlx::query("INSERT INTO workspace_watches(workspace_id,vendor,url,keywords,owner,version,command) VALUES(?,?,?,?,?,?,?)")
            .bind(workspace)
            .bind("Keep me")
            .bind("https://1.1.1.1/feed")
            .bind("webhook")
            .bind("Maya")
            .bind("sdk 1.0")
            .bind("npm test")
            .execute(&db)
            .await
            .unwrap();
        let app = App {
            db,
            build: "test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = replace_watches(
            State(app.clone()),
            bearer(&token),
            Json(WatchImport {
                watches: vec![NewWatch {
                    vendor: "Blocked import".to_owned(),
                    url: "http://127.0.0.1/private".to_owned(),
                    keywords: "webhook".to_owned(),
                    owner: "Nora".to_owned(),
                    version: "sdk 2.0".to_owned(),
                    command: "npm test".to_owned(),
                }],
            }),
        )
        .await;
        assert!(matches!(result, Err(ApiError(StatusCode::BAD_REQUEST, _))));
        let watches = list_watches(State(app), bearer(&token)).await.unwrap().0;
        assert_eq!(watches.len(), 1);
        assert_eq!(watches[0].vendor, "Keep me");
    }

    #[test]
    fn parses_every_matching_rss_and_preserves_notice_permalink() {
        let notices = parse_notices("<rss><channel><item><title>Webhook deprecation</title><description>Move now</description><link>https://vendor.example/one</link></item><item><title>Webhook retry change</title><description>Read this</description><link>https://vendor.example/two</link></item></channel></rss>", "https://vendor.example/feed.xml");
        assert_eq!(notices.len(), 2);
        assert_eq!(notices[1].url, "https://vendor.example/two");
    }

    #[tokio::test]
    async fn claim_hosted_scan_creates_action_card_from_controlled_fixture() {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        setup(&db).await.unwrap();
        let token = "f".repeat(64);
        sqlx::query("INSERT INTO workspaces(token_hash,created_at) VALUES(?,?)")
            .bind(token_hash(&token))
            .bind("now")
            .execute(&db)
            .await
            .unwrap();
        let workspace: i64 = sqlx::query_scalar("SELECT id FROM workspaces")
            .fetch_one(&db)
            .await
            .unwrap();
        sqlx::query("INSERT INTO workspace_watches(workspace_id,vendor,url,keywords,owner,version,command) VALUES(?,?,?,?,?,?,?)")
            .bind(workspace).bind("Fixture vendor").bind("https://vendor.example/feed.xml").bind("webhook,deprecation").bind("Maya · Payments").bind("fixture-sdk 2.4").bind("pnpm test:fixture")
            .execute(&db).await.unwrap();
        let watch = sqlx::query_as::<_, WatchRow>("SELECT id, vendor, url, keywords, owner, version, command, last_hash FROM workspace_watches")
            .fetch_one(&db).await.unwrap();
        let fixture = "<rss><channel><item><title>Webhook deprecation date</title><description>Update webhook signatures before June.</description><link>https://vendor.example/notice</link></item></channel></rss>";
        assert_eq!(
            record_matches(&db, workspace, &watch, parse_notices(fixture, &watch.url))
                .await
                .unwrap(),
            1
        );
        let mut actions = list_actions(
            State(App {
                db: db.clone(),
                build: "test".to_owned(),
                limiter: Arc::new(Mutex::new(HashMap::new())),
            }),
            bearer(&token),
        )
        .await
        .unwrap()
        .0;
        let action = actions.remove(0);
        assert_eq!(action.title, "Webhook deprecation date");
        assert_eq!(action.matched, "webhook");
        assert_eq!(action.owner, "Maya · Payments");
        assert_eq!(action.version, "fixture-sdk 2.4");
        assert_eq!(action.command, "pnpm test:fixture");
    }

    #[test]
    fn parses_atom_html_titles_and_html_changelog_headings() {
        let atom = parse_notices("<feed><entry><title type=\"html\">Token &amp; webhook update</title><summary>Deprecation notice</summary><link href=\"/notice\" /></entry></feed>", "https://vendor.example/feed");
        assert_eq!(atom[0].title, "Token & webhook update");
        assert_eq!(atom[0].url, "https://vendor.example/notice");
        let html = parse_notices(
            "<main><h2>Webhook deprecation</h2></main>",
            "https://vendor.example/changelog",
        );
        assert_eq!(html[0].title, "Webhook deprecation");
    }

    #[test]
    fn blocks_private_and_malformed_destinations() {
        assert!(forbidden_ip("127.0.0.1".parse().unwrap()));
        assert!(forbidden_ip("10.0.0.4".parse().unwrap()));
        assert!(Url::parse("httpjunk").is_err());
    }

    #[test]
    fn workspace_tokens_are_not_plaintext_rows() {
        let token = "a-very-long-workspace-token";
        assert_ne!(token_hash(token), token);
    }

    #[tokio::test]
    async fn default_database_survives_restart_when_data_is_mounted() {
        assert_eq!(
            default_database_url(),
            "sqlite:/data/changelog-watch.db?mode=rwc&vfs=unix-dotfile"
        );
        let root = std::env::temp_dir().join(format!("icw-data-test-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let database = root.join("changelog-watch.db");
        let url = format!("sqlite:{}?mode=rwc&vfs=unix-dotfile", database.display());
        let first = SqlitePoolOptions::new().connect(&url).await.unwrap();
        setup(&first).await.unwrap();
        sqlx::query("INSERT INTO workspaces(token_hash,created_at) VALUES(?,?)")
            .bind(token_hash("persisted-workspace-token"))
            .bind("now")
            .execute(&first)
            .await
            .unwrap();
        first.close().await;
        let second = SqlitePoolOptions::new().connect(&url).await.unwrap();
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM workspaces")
            .fetch_one(&second)
            .await
            .unwrap();
        assert_eq!(count, 1);
        second.close().await;
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn watch_limit_is_atomic_under_concurrent_creates() {
        let root = std::env::temp_dir().join(format!("icw-quota-test-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let database = root.join("changelog-watch.db");
        let url = format!("sqlite:{}?mode=rwc", database.display());
        let db = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await
            .unwrap();
        setup(&db).await.unwrap();
        sqlx::query("INSERT INTO workspaces(token_hash,created_at) VALUES(?,?)")
            .bind(token_hash("concurrent-workspace-token"))
            .bind("now")
            .execute(&db)
            .await
            .unwrap();
        let workspace: i64 = sqlx::query_scalar("SELECT id FROM workspaces")
            .fetch_one(&db)
            .await
            .unwrap();
        let watch = NewWatch {
            vendor: "Concurrent vendor".to_owned(),
            url: "https://1.1.1.1/feed".to_owned(),
            keywords: "webhook".to_owned(),
            owner: "Maya".to_owned(),
            version: "".to_owned(),
            command: "npm test".to_owned(),
        };
        let mut creates = tokio::task::JoinSet::new();
        for attempt in 0..10 {
            let db = db.clone();
            let mut watch = watch.clone();
            watch.vendor = format!("Concurrent vendor {attempt}");
            creates.spawn(async move {
                match insert_watch_under_limit(&db, workspace, &watch).await {
                    Ok(_) => StatusCode::CREATED,
                    Err(ApiError(status, _)) => status,
                }
            });
        }
        let mut created = 0;
        let mut limited = 0;
        while let Some(result) = creates.join_next().await {
            match result.unwrap() {
                StatusCode::CREATED => created += 1,
                StatusCode::CONFLICT => limited += 1,
                status => panic!("unexpected concurrent create response: {status}"),
            }
        }
        let stored: i64 =
            sqlx::query_scalar("SELECT count(*) FROM workspace_watches WHERE workspace_id=?")
                .bind(workspace)
                .fetch_one(&db)
                .await
                .unwrap();
        assert_eq!((created, limited, stored), (3, 7, 3));
        db.close().await;
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn ingress_client_ip_shares_one_bucket_across_connections_and_ignores_later_hops() {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let state = App {
            db,
            build: "test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(middleware::from_fn_with_state(state, rate_limit));
        let mut allowed = 0;
        let mut limited = 0;
        for request_number in 0..80 {
            let mut request = Request::builder()
                .uri("/")
                .body(axum::body::Body::empty())
                .unwrap();
            request.headers_mut().insert(
                "x-forwarded-for",
                format!("198.51.100.77, 192.0.2.{request_number}")
                    .parse()
                    .unwrap(),
            );
            request
                .extensions_mut()
                .insert(ConnectInfo(SocketAddr::from((
                    [10, 0, 0, request_number],
                    4567,
                ))));
            let response = app.clone().oneshot(request).await.unwrap();
            if response.status().is_success() {
                allowed += 1;
            } else {
                limited += 1;
                assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
                assert_eq!(response.headers().get(header::RETRY_AFTER).unwrap(), "1");
            }
        }
        assert_eq!((allowed, limited), (40, 40));
    }

    #[test]
    fn claim_port_only_startup_configuration() {
        assert_eq!(server_port(None), 8080);
        assert_eq!(server_port(Some("9090".to_owned())), 9090);
        assert_eq!(
            default_database_url(),
            "sqlite:/data/changelog-watch.db?mode=rwc&vfs=unix-dotfile"
        );
    }

    #[test]
    fn claim_single_replica_durable_topology() {
        let topology = include_str!("../deploy/containerapp.yaml");
        assert!(topology.contains("minReplicas: 1"));
        assert!(topology.contains("maxReplicas: 1"));
        assert!(topology.contains("storageType: AzureFile"));
        assert!(topology.contains("storageName: integration-changelog-watch-data"));
        assert!(topology.contains("mountPath: /data"));
        assert!(topology.contains("terminationGracePeriodSeconds: 30"));

        // This reproduces the generic factory deployment body from
        // verification 8 and proves the runtime guard restores the boundary.
        let generic = serde_json::json!({
            "properties": {"template": {
                "scale": {"minReplicas": 1, "maxReplicas": 3},
                "containers": [{
                    "name": "app",
                    "image": "registry.example/product:repair",
                    "resources": {"cpu": 0.5, "memory": "1Gi"},
                    "env": [{"name": "PORT", "value": "8080"}]
                }]
            }}
        });
        assert!(!topology_is_durable(&generic));
        let patch = durable_topology_patch(&generic).unwrap();
        assert_eq!(patch["properties"]["template"]["scale"]["maxReplicas"], 1);
        assert_eq!(
            patch["properties"]["template"]["containers"][0]["image"],
            "registry.example/product:repair"
        );
        assert_eq!(
            patch["properties"]["template"]["containers"][0]["volumeMounts"][0]["mountPath"],
            "/data"
        );
        assert!(mountinfo_has_data_mount(
            "43 31 0:42 / /data rw,relatime - cifs share rw"
        ));
        assert!(!mountinfo_has_data_mount(
            "43 31 0:42 / /app rw,relatime - overlay overlay rw"
        ));
    }

    #[tokio::test]
    async fn replica_local_sqlite_failure_is_reproduced() {
        // This is the exact production failure shape: a workspace created on
        // one independent SQLite replica is unknown to the next replica.
        let first_db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let second_db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        setup(&first_db).await.unwrap();
        setup(&second_db).await.unwrap();
        let token = "r".repeat(64);
        sqlx::query("INSERT INTO workspaces(token_hash,created_at) VALUES(?,?)")
            .bind(token_hash(&token))
            .bind("now")
            .execute(&first_db)
            .await
            .unwrap();
        let first = App {
            db: first_db,
            build: "test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };
        let second = App {
            db: second_db,
            build: "test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };
        assert!(workspace_id(&bearer(&token), &first).await.is_ok());
        assert!(matches!(
            workspace_id(&bearer(&token), &second).await,
            Err(ApiError(StatusCode::UNAUTHORIZED, _))
        ));
    }

    #[tokio::test]
    async fn replica_local_rate_limits_fragment_the_40_request_allowance() {
        let first = App {
            db: SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap(),
            build: "test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };
        let second = App {
            db: SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap(),
            build: "test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };
        let first_router = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(middleware::from_fn_with_state(first, rate_limit));
        let second_router = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(middleware::from_fn_with_state(second, rate_limit));
        let mut accepted = 0;
        for router in [first_router, second_router] {
            for _ in 0..40 {
                let mut request = Request::builder()
                    .uri("/")
                    .body(axum::body::Body::empty())
                    .unwrap();
                request
                    .headers_mut()
                    .insert("x-forwarded-for", "198.51.100.77".parse().unwrap());
                if router
                    .clone()
                    .oneshot(request)
                    .await
                    .unwrap()
                    .status()
                    .is_success()
                {
                    accepted += 1;
                }
            }
        }
        // Two independent replicas allow two full 40-request bursts. This is
        // the 120-request production failure in its smallest deterministic form.
        assert_eq!(accepted, 80);
    }

    #[tokio::test]
    async fn durable_single_replica_recovers_tokens_after_restart_and_keeps_one_40_request_burst() {
        let root = std::env::temp_dir().join(format!("icw-durable-topology-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let database = root.join("changelog-watch.db");
        let url = format!("sqlite:{}?mode=rwc", database.display());
        let token = "durable-workspace-token-".repeat(3);

        // Process one creates the workspace and exits. Process two opens the
        // same mounted path, which is the only allowed production replica.
        let first = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .unwrap();
        setup(&first).await.unwrap();
        sqlx::query("INSERT INTO workspaces(token_hash,created_at) VALUES(?,?)")
            .bind(token_hash(&token))
            .bind("now")
            .execute(&first)
            .await
            .unwrap();
        first.close().await;

        let restarted = App {
            db: SqlitePoolOptions::new()
                .max_connections(1)
                .connect(&url)
                .await
                .unwrap(),
            build: "test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };
        assert!(workspace_id(&bearer(&token), &restarted).await.is_ok());
        let router = Router::new()
            .route("/api/watches", get(list_watches))
            .layer(middleware::from_fn_with_state(
                restarted.clone(),
                rate_limit,
            ))
            .with_state(restarted.clone());
        let mut allowed = 0;
        let mut limited = 0;
        for _ in 0..80 {
            let mut request = Request::builder()
                .uri("/api/watches")
                .body(axum::body::Body::empty())
                .unwrap();
            request.headers_mut().extend(bearer(&token));
            request
                .headers_mut()
                .insert("x-forwarded-for", "198.51.100.88".parse().unwrap());
            let response = router.clone().oneshot(request).await.unwrap();
            match response.status() {
                StatusCode::OK => allowed += 1,
                StatusCode::TOO_MANY_REQUESTS => {
                    limited += 1;
                    assert_eq!(response.headers().get(header::RETRY_AFTER).unwrap(), "1");
                }
                status => panic!("unexpected response after restart: {status}"),
            }
        }
        assert_eq!((allowed, limited), (40, 40));
        restarted.db.close().await;
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn sigterm_shutdown_path_drains_an_active_server() {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let app = App {
            db,
            build: "shutdown-test".to_owned(),
            limiter: Arc::new(Mutex::new(HashMap::new())),
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let router = Router::new().route("/health", get(health)).with_state(app);
        let (send_shutdown, receive_shutdown) = tokio::sync::oneshot::channel::<()>();
        let server = tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    receive_shutdown.await.unwrap();
                })
                .await
                .unwrap();
        });
        let response = reqwest::get(format!("http://{address}/health"))
            .await
            .unwrap();
        assert!(response.status().is_success());
        send_shutdown.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(2), server)
            .await
            .expect("server must exit after its shutdown signal")
            .unwrap();
    }

    #[test]
    fn parses_standard_rss_cdata_as_readable_text() {
        let notices = parse_notices(
            "<rss><channel><item><title><![CDATA[Unix V4 Workshop at Low Resource Computing]]></title><description><![CDATA[<p>A webhook migration closes Friday.</p>]]></description><link>https://vendor.example/workshop</link></item></channel></rss>",
            "https://vendor.example/feed.xml",
        );
        assert_eq!(notices.len(), 1);
        assert_eq!(
            notices[0].title,
            "Unix V4 Workshop at Low Resource Computing"
        );
        assert_eq!(notices[0].excerpt, "A webhook migration closes Friday.");
    }

    #[test]
    fn claim_redirecting_feeds_are_rejected() {
        assert_eq!(
            ensure_feed_response_is_safe(StatusCode::TEMPORARY_REDIRECT),
            Err("This feed redirects. Use its final public HTTPS address instead.".to_owned())
        );
        assert_eq!(
            ensure_feed_response_is_safe(StatusCode::BAD_GATEWAY),
            Err("The feed returned an error response.".to_owned())
        );
        assert_eq!(ensure_feed_response_is_safe(StatusCode::OK), Ok(()));
    }

    #[tokio::test]
    async fn cli_scan_persists_deduplication_and_acknowledgements_for_local_repository_feeds() {
        let root = std::env::temp_dir().join(format!("icw-cli-test-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("feed.xml"), "<rss><channel><item><title><![CDATA[Webhook update]]></title><description><![CDATA[<p>Move webhook fixture</p>]]></description><link>https://example.com/notice</link></item></channel></rss>").unwrap();
        std::fs::write(root.join("watches.json"), r#"{"watches":[{"vendor":"Example","url":"feed.xml","keywords":"webhook","owner":"Maya","version":"example-sdk 4.2","command":"npm test"}]}"#).unwrap();
        let config = root.join("watches.json");
        cli_scan(config.to_str().unwrap()).await.unwrap();
        let state = read_cli_state(&root.join(".integration-changelog-watch")).unwrap();
        assert_eq!(state.actions.len(), 1);
        let id = state.actions[0].id.clone();
        let card_path = root
            .join(".integration-changelog-watch/actions")
            .join(format!("{id}.md"));
        assert!(card_path.exists());
        let card = std::fs::read_to_string(&card_path).unwrap();
        assert!(card.contains("# Webhook update"));
        assert!(card.contains("Move webhook fixture"));
        assert!(card.contains("**Affected dependency:** example-sdk 4.2"));
        assert!(card.contains("**Status:** Needs acknowledgement"));
        assert!(!card.contains("CDATA"));
        cli_scan(config.to_str().unwrap()).await.unwrap();
        assert_eq!(
            read_cli_state(&root.join(".integration-changelog-watch"))
                .unwrap()
                .actions
                .len(),
            1
        );
        cli_ack(config.to_str().unwrap(), &id).unwrap();
        assert!(
            read_cli_state(&root.join(".integration-changelog-watch"))
                .unwrap()
                .actions[0]
                .acknowledged
        );
        let card = std::fs::read_to_string(card_path).unwrap();
        assert!(card.contains("**Status:** Acknowledged"));
        assert!(!card.contains("**Status:** Needs acknowledgement"));
        std::fs::remove_dir_all(root).unwrap();
    }
}

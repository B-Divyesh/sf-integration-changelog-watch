use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use reqwest::{redirect::Policy, Client};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use std::{
    env,
    net::{IpAddr, SocketAddr},
    time::Duration,
};
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;
use url::Url;
use uuid::Uuid;

#[derive(Clone)]
struct App {
    db: SqlitePool,
    build: String,
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

#[derive(Debug, Deserialize)]
struct NewWatch {
    vendor: String,
    url: String,
    keywords: String,
    owner: String,
    version: String,
    command: String,
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

    tracing_subscriber::fmt()
        .json()
        .with_env_filter("info")
        .init();
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:/data/changelog-watch.db?mode=rwc".to_owned());
    let db = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(_) => SqlitePoolOptions::new()
            .connect("sqlite:changelog-watch.db?mode=rwc")
            .await
            .expect("SQLite starts"),
    };
    setup(&db).await.expect("schema");
    let state = App {
        db,
        build: env::var("BUILD_SHA").unwrap_or_else(|_| "dev".to_owned()),
    };
    info!(
        config = "DATABASE_URL defaulted when absent",
        "starting Integration Changelog Watch"
    );

    let governor = GovernorConfigBuilder::default()
        .per_second(20)
        .burst_size(40)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .expect("rate limiter config");
    let api = Router::new()
        .route("/health", get(health))
        .route("/api/workspaces", post(create_workspace))
        .route("/api/watches", get(list_watches).post(add_watch))
        .route("/api/actions", get(list_actions))
        .route("/api/actions/:id", post(ack_action))
        .route("/api/scan", post(scan))
        .layer(GovernorLayer {
            config: governor.into(),
        })
        .layer(middleware::from_fn(api_cache_headers))
        .with_state(state);

    let app = Router::new()
        .merge(api)
        .route_service("/", ServeFile::new("dist/index.html"))
        .route_service("/demo", ServeFile::new("dist/index.html"))
        .route_service("/privacy", ServeFile::new("dist/index.html"))
        .route_service("/terms", ServeFile::new("dist/index.html"))
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
        .fallback(not_found)
        .layer(middleware::from_fn(site_headers));
    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8080);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("bind PORT");
    info!(port, "listening");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("serve");
}

async fn setup(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workspaces(id INTEGER PRIMARY KEY, token_hash TEXT NOT NULL UNIQUE, created_at TEXT NOT NULL);
         CREATE TABLE IF NOT EXISTS workspace_watches(id INTEGER PRIMARY KEY, workspace_id INTEGER NOT NULL, vendor TEXT NOT NULL, url TEXT NOT NULL, keywords TEXT NOT NULL, owner TEXT NOT NULL, version TEXT NOT NULL, command TEXT NOT NULL, last_hash TEXT, last_scanned TEXT, FOREIGN KEY(workspace_id) REFERENCES workspaces(id));
         CREATE TABLE IF NOT EXISTS workspace_actions(id INTEGER PRIMARY KEY, workspace_id INTEGER NOT NULL, watch_id INTEGER NOT NULL, notice_key TEXT NOT NULL, title TEXT NOT NULL, excerpt TEXT NOT NULL, matched TEXT NOT NULL, url TEXT NOT NULL, owner TEXT NOT NULL, command TEXT NOT NULL, acknowledged INTEGER NOT NULL DEFAULT 0, seen_at TEXT NOT NULL, UNIQUE(workspace_id, watch_id, notice_key), FOREIGN KEY(workspace_id) REFERENCES workspaces(id));
         CREATE INDEX IF NOT EXISTS workspace_watches_owner ON workspace_watches(workspace_id);
         CREATE INDEX IF NOT EXISTS workspace_actions_owner ON workspace_actions(workspace_id);",
    )
    .execute(db)
    .await?;
    Ok(())
}

async fn health(State(app): State<App>) -> impl IntoResponse {
    Json(serde_json::json!({"ok": true, "build": app.build}))
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
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM workspace_watches WHERE workspace_id=?")
            .bind(workspace)
            .fetch_one(&app.db)
            .await
            .map_err(internal)?;
    if count >= 3 {
        return Err(ApiError(StatusCode::CONFLICT, "This workspace already has three watches. Edit an existing watch before adding another.".to_owned()));
    }
    let id = sqlx::query("INSERT INTO workspace_watches(workspace_id,vendor,url,keywords,owner,version,command) VALUES(?,?,?,?,?,?,?)")
        .bind(workspace).bind(&new.vendor).bind(&new.url).bind(&new.keywords).bind(&new.owner).bind(&new.version).bind(&new.command)
        .execute(&app.db).await.map_err(internal)?.last_insert_rowid();
    let watch = sqlx::query_as("SELECT id, vendor, url, keywords, owner, version, command, last_scanned FROM workspace_watches WHERE id=? AND workspace_id=?")
        .bind(id).bind(workspace).fetch_one(&app.db).await.map_err(internal)?;
    Ok((StatusCode::CREATED, Json(watch)))
}

async fn list_actions(State(app): State<App>, headers: HeaderMap) -> ApiResult<Json<Vec<Action>>> {
    let workspace = workspace_id(&headers, &app).await?;
    let actions = sqlx::query_as("SELECT id, watch_id, title, excerpt, matched, url, owner, command, acknowledged, seen_at FROM workspace_actions WHERE workspace_id=? ORDER BY acknowledged, id DESC")
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
    let action = sqlx::query_as("SELECT id, watch_id, title, excerpt, matched, url, owner, command, acknowledged, seen_at FROM workspace_actions WHERE id=? AND workspace_id=?")
        .bind(id).bind(workspace).fetch_one(&app.db).await.map_err(internal)?;
    Ok(Json(action))
}

async fn scan(State(app): State<App>, headers: HeaderMap) -> ApiResult<Json<ScanResult>> {
    let workspace = workspace_id(&headers, &app).await?;
    let watches: Vec<WatchRow> = sqlx::query_as("SELECT id, vendor, url, keywords, owner, command, last_hash FROM workspace_watches WHERE workspace_id=?")
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
            for notice in parse_notices(&text, &watch.url) {
                let body = format!("{} {}", notice.title, notice.excerpt).to_lowercase();
                if let Some(rule) = watch
                    .keywords
                    .split(',')
                    .map(str::trim)
                    .filter(|rule| !rule.is_empty())
                    .find(|rule| body.contains(&rule.to_lowercase()))
                {
                    let key = format!(
                        "{:x}",
                        Sha256::digest(format!("{}\n{}", notice.title, notice.url).as_bytes())
                    );
                    let title = if notice.title.is_empty() {
                        format!("Matched change from {}", watch.vendor)
                    } else {
                        notice.title
                    };
                    let inserted = sqlx::query("INSERT OR IGNORE INTO workspace_actions(workspace_id,watch_id,notice_key,title,excerpt,matched,url,owner,command,seen_at) VALUES(?,?,?,?,?,?,?,?,?,?)")
                        .bind(workspace).bind(watch.id).bind(key).bind(title).bind(notice.excerpt.chars().take(420).collect::<String>()).bind(rule).bind(notice.url).bind(&watch.owner).bind(&watch.command).bind(Utc::now().to_rfc3339())
                        .execute(&app.db).await.map_err(internal)?;
                    made += inserted.rows_affected() as usize;
                }
            }
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

#[derive(FromRow)]
struct WatchRow {
    id: i64,
    vendor: String,
    url: String,
    keywords: String,
    owner: String,
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
    let document = Html::parse_document(text);
    let entry = Selector::parse("item, entry").expect("valid selector");
    let title = Selector::parse("title").expect("valid selector");
    let description = Selector::parse("description, summary, content").expect("valid selector");
    let link = Selector::parse("link").expect("valid selector");
    let rss_links = rss_item_links(text);
    let mut notices: Vec<Notice> = document
        .select(&entry)
        .enumerate()
        .map(|(index, item)| {
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
            let item_url = if item_url == source_url {
                rss_links.get(index).cloned().unwrap_or(item_url)
            } else {
                item_url
            };
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

fn rss_item_links(text: &str) -> Vec<String> {
    text.split("<item")
        .skip(1)
        .filter_map(|item| item.split("</item>").next())
        .filter_map(|item| item.split("<link").nth(1))
        .filter_map(|item| item.split_once('>').map(|(_, value)| value))
        .filter_map(|item| item.split("</link>").next())
        .map(str::trim)
        .filter(|link| !link.is_empty())
        .map(str::to_owned)
        .collect()
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
            "Provide a vendor, public URL, rules, owner, and check command.".to_owned(),
        ));
    }
    resolve_public_url(&watch.url)
        .await
        .map(|_| ())
        .map_err(|message| ApiError(StatusCode::BAD_REQUEST, message))
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
    if response.status().is_redirection() {
        return Err("This feed redirects. Use its final public HTTPS address instead.".to_owned());
    }
    response
        .error_for_status()
        .map_err(|_| "The feed returned an error response.".to_owned())?
        .text()
        .await
        .map_err(|_| "Could not read this feed response.".to_owned())
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

async fn api_cache_headers(request: Request<axum::body::Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, private"),
    );
    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        response
            .headers_mut()
            .insert(header::RETRY_AFTER, HeaderValue::from_static("1"));
    }
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

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, [(header::CONTENT_TYPE, "text/html; charset=utf-8")], "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Page not found — Integration Changelog Watch</title></head><body><main id=\"main\"><h1>That page is not here</h1><p>Return to your integration action board.</p><p><a href=\"/\">Return home</a></p></main></body></html>")
}

fn print_help() {
    println!("Integration Changelog Watch\n\nUsage:\n  integration-changelog-watch                         Start the dashboard server\n  integration-changelog-watch demo                    Print shipped Markdown action cards\n  integration-changelog-watch scan --config FILE     Scan a repository watch mapping to Markdown\n  integration-changelog-watch --help                  Show this help\n\nThe dashboard uses a private browser workspace token. The demo command makes no network request.");
}

fn print_demo_markdown() {
    println!("# Integration changelog watch demo\n\n## Stripe retires legacy webhook event format\n\n- **Matched rule:** webhook\n- **Owner:** Maya · Payments\n- **Check:** `pnpm test:stripe`\n\nReview signature parsing and event fixtures.\n\n## Auth0 changes refresh token rotation defaults\n\n- **Matched rule:** token\n- **Owner:** Ishan · Identity\n- **Check:** `pnpm test:auth`\n\nCheck explicit configuration before the next environment.");
}

async fn cli_scan(path: &str) -> Result<(), String> {
    let source = std::fs::read_to_string(path).map_err(|_| format!("could not read {path}"))?;
    let config: CliConfig = serde_json::from_str(&source)
        .map_err(|_| "the watch mapping is not valid JSON".to_owned())?;
    let mut cards = Vec::new();
    for watch in config.watches {
        validate_watch(&watch).await.map_err(|error| error.1)?;
        let text = fetch_public(&watch.url).await?;
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
                cards.push(format!("## {title}\n\n- **Matched rule:** {rule}\n- **Owner:** {}\n- **Check:** `{}`\n- **Notice:** {}\n\n{}", watch.owner, watch.command, notice.url, notice.excerpt));
            }
        }
    }
    if cards.is_empty() {
        println!("# Integration changelog scan\n\nNo matching notices found.");
    } else {
        println!("# Integration changelog scan\n\n{}", cards.join("\n\n"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(json.get("source_url").is_none());
    }

    #[test]
    fn parses_every_matching_rss_and_preserves_notice_permalink() {
        let notices = parse_notices("<rss><channel><item><title>Webhook deprecation</title><description>Move now</description><link>https://vendor.example/one</link></item><item><title>Webhook retry change</title><description>Read this</description><link>https://vendor.example/two</link></item></channel></rss>", "https://vendor.example/feed.xml");
        assert_eq!(notices.len(), 2);
        assert_eq!(notices[1].url, "https://vendor.example/two");
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
}

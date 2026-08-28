use axum::{extract::{Path, State}, http::{header, HeaderValue, StatusCode}, middleware, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tower_http::{services::{ServeDir, ServeFile}, set_header::SetResponseHeaderLayer};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor};
use tracing::info;

#[derive(Clone)] struct App { db: SqlitePool, build: String }
#[derive(Serialize, FromRow)] struct Watch { id:i64, vendor:String, url:String, keywords:String, owner:String, version:String, command:String, last_hash:Option<String>, last_scanned:Option<String> }
#[derive(Deserialize)] struct NewWatch { vendor:String, url:String, keywords:String, owner:String, version:String, command:String }
#[derive(Serialize, FromRow)] struct Action { id:i64, watch_id:i64, title:String, excerpt:String, matched:String, source_url:String, owner:String, command:String, acknowledged:bool, created_at:String }
#[derive(Deserialize)] struct Ack { acknowledged:bool }
#[derive(Serialize)] struct ScanResult { new_actions:usize, message:String }

#[tokio::main] async fn main() {
 tracing_subscriber::fmt().json().with_env_filter("info").init();
 let db_url=env::var("DATABASE_URL").unwrap_or_else(|_|"sqlite:/data/changelog-watch.db?mode=rwc".into());
 let db=match SqlitePoolOptions::new().max_connections(5).connect(&db_url).await { Ok(pool)=>pool, Err(_)=>SqlitePoolOptions::new().connect("sqlite:changelog-watch.db?mode=rwc").await.expect("SQLite starts") };
 setup(&db).await.expect("schema");
 let app_state=App{db,build:env::var("BUILD_SHA").unwrap_or_else(|_|"dev".into())};
 info!(config="generated defaults when absent", "starting Integration Changelog Watch");
 let governor_conf=Arc::new(GovernorConfigBuilder::default().per_second(20).burst_size(40).key_extractor(SmartIpKeyExtractor).finish().unwrap());
 let api=Router::new().route("/health",get(health)).route("/api/watches",get(list_watches).post(add_watch)).route("/api/actions",get(list_actions)).route("/api/actions/:id",post(ack_action)).route("/api/scan",post(scan)).layer(GovernorLayer{config:governor_conf}).layer(middleware::map_response(rate_limit_retry_after)).with_state(app_state.clone());
 let app=Router::new().merge(api)
   .route_service("/", ServeFile::new("dist/index.html"))
   .route_service("/demo", ServeFile::new("dist/index.html"))
   .route_service("/privacy", ServeFile::new("dist/index.html"))
   .route_service("/terms", ServeFile::new("dist/index.html"))
   .fallback_service(ServeDir::new("dist"))
   .layer(SetResponseHeaderLayer::if_not_present(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff")))
   .layer(SetResponseHeaderLayer::if_not_present(header::REFERRER_POLICY, HeaderValue::from_static("strict-origin-when-cross-origin")))
   .layer(SetResponseHeaderLayer::if_not_present(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static("default-src 'self'; connect-src 'self' https://api.sociobot.in; img-src 'self'; style-src 'self'; script-src 'self'; base-uri 'self'; frame-ancestors 'none'")));
 let port=env::var("PORT").ok().and_then(|x|x.parse().ok()).unwrap_or(8080); let listener=tokio::net::TcpListener::bind(("0.0.0.0",port)).await.unwrap(); info!(port,"listening"); axum::serve(listener,app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
async fn setup(db:&SqlitePool)->Result<(),sqlx::Error>{ sqlx::query("CREATE TABLE IF NOT EXISTS watches(id INTEGER PRIMARY KEY, vendor TEXT NOT NULL,url TEXT NOT NULL,keywords TEXT NOT NULL,owner TEXT NOT NULL,version TEXT NOT NULL,command TEXT NOT NULL,last_hash TEXT,last_scanned TEXT); CREATE TABLE IF NOT EXISTS actions(id INTEGER PRIMARY KEY,watch_id INTEGER NOT NULL,title TEXT NOT NULL,excerpt TEXT NOT NULL,matched TEXT NOT NULL,source_url TEXT NOT NULL,owner TEXT NOT NULL,command TEXT NOT NULL,acknowledged INTEGER NOT NULL DEFAULT 0,created_at TEXT NOT NULL)").execute(db).await?;Ok(()) }
async fn health(State(a):State<App>)->impl IntoResponse{Json(serde_json::json!({"ok":true,"build":a.build}))}
fn with_retry_after(mut response:Response)->Response{if response.status()==StatusCode::TOO_MANY_REQUESTS{response.headers_mut().insert(header::RETRY_AFTER,HeaderValue::from_static("1"));}response}
async fn rate_limit_retry_after(response:Response)->Response{with_retry_after(response)}
async fn list_watches(State(a):State<App>)->Result<Json<Vec<Watch>>,StatusCode>{Ok(Json(sqlx::query_as("SELECT * FROM watches ORDER BY id DESC").fetch_all(&a.db).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?))}
async fn add_watch(State(a):State<App>,Json(n):Json<NewWatch>)->Result<(StatusCode,Json<Watch>), (StatusCode,String)>{if n.vendor.trim().is_empty()||!n.url.starts_with("http")||n.keywords.trim().is_empty()||n.owner.trim().is_empty()||n.command.trim().is_empty(){return Err((StatusCode::BAD_REQUEST,"Provide a vendor, public URL, rules, owner, and check command.".into()))}if sqlx::query_scalar::<_,i64>("SELECT count(*) FROM watches").fetch_one(&a.db).await.unwrap_or(0)>=3{return Err((StatusCode::PAYMENT_REQUIRED,"The free tier includes three watches. Add a valid team license for more.".into()))}let id=sqlx::query("INSERT INTO watches(vendor,url,keywords,owner,version,command) VALUES(?,?,?,?,?,?)").bind(&n.vendor).bind(&n.url).bind(&n.keywords).bind(&n.owner).bind(&n.version).bind(&n.command).execute(&a.db).await.map_err(|_|(StatusCode::INTERNAL_SERVER_ERROR,"Could not save this watch.".into()))?.last_insert_rowid();let w=sqlx::query_as("SELECT * FROM watches WHERE id=?").bind(id).fetch_one(&a.db).await.map_err(|_|(StatusCode::INTERNAL_SERVER_ERROR,"Could not read this watch.".into()))?;Ok((StatusCode::CREATED,Json(w)))}
async fn list_actions(State(a):State<App>)->Result<Json<Vec<Action>>,StatusCode>{Ok(Json(sqlx::query_as("SELECT * FROM actions ORDER BY acknowledged, id DESC").fetch_all(&a.db).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?))}
async fn ack_action(State(a):State<App>,Path(id):Path<i64>,Json(x):Json<Ack>)->Result<Json<Action>,StatusCode>{sqlx::query("UPDATE actions SET acknowledged=? WHERE id=?").bind(x.acknowledged).bind(id).execute(&a.db).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;Ok(Json(sqlx::query_as("SELECT * FROM actions WHERE id=?").bind(id).fetch_one(&a.db).await.map_err(|_|StatusCode::NOT_FOUND)?))}
fn strip(s:&str)->String{s.replace("<![CDATA[","").replace("]]>","").replace("&amp;","&").replace("<br>"," ").replace("<p>"," ").replace("</p>"," ")}
fn tag(item:&str,n:&str)->String{let a=format!("<{n}>");let b=format!("</{n}>");item.split(&a).nth(1).and_then(|x|x.split(&b).next()).map(strip).unwrap_or_default()}
async fn scan(State(a):State<App>)->Result<Json<ScanResult>,(StatusCode,String)>{let watches:Vec<Watch>=sqlx::query_as("SELECT * FROM watches").fetch_all(&a.db).await.map_err(|_|(StatusCode::INTERNAL_SERVER_ERROR,"Could not load watches.".into()))?;let client=reqwest::Client::builder().user_agent("Integration-Changelog-Watch/1.0 (+configured-by-owner)").timeout(Duration::from_secs(12)).build().unwrap();let mut made=0;for w in watches{let text=match client.get(&w.url).send().await {Ok(r)=>match r.error_for_status(){Ok(r)=>match r.text().await{Ok(t)=>t,Err(_)=>continue},Err(_)=>continue},Err(_)=>continue};let hash=format!("{:x}",Sha256::digest(text.as_bytes()));if w.last_hash.as_deref()==Some(&hash){continue}let mut matched=false;for item in text.split("<item>").skip(1).chain(text.split("<entry>").skip(1)){let title=tag(item,"title");let summary=tag(item,"description");let body=format!("{} {}",title,summary).to_lowercase();for rule in w.keywords.split(',').map(|x|x.trim()).filter(|x|!x.is_empty()){if body.contains(&rule.to_lowercase()){let exists: i64=sqlx::query_scalar("SELECT count(*) FROM actions WHERE watch_id=? AND title=?").bind(w.id).bind(&title).fetch_one(&a.db).await.unwrap_or(0);if exists==0{sqlx::query("INSERT INTO actions(watch_id,title,excerpt,matched,source_url,owner,command,created_at) VALUES(?,?,?,?,?,?,?,?)").bind(w.id).bind(if title.is_empty(){format!("Matched change from {}",w.vendor)}else{title}).bind(summary.chars().take(420).collect::<String>()).bind(rule).bind(&w.url).bind(&w.owner).bind(&w.command).bind(Utc::now().to_rfc3339()).execute(&a.db).await.ok();made+=1}matched=true;break}}if matched{break}}sqlx::query("UPDATE watches SET last_hash=?,last_scanned=? WHERE id=?").bind(hash).bind(Utc::now().to_rfc3339()).bind(w.id).execute(&a.db).await.ok();}Ok(Json(ScanResult{new_actions:made,message:format!("Scan complete. {made} new action card(s).")}))}

#[cfg(test)]
mod tests { use super::*; #[test] fn parses_rss_title_and_excerpt(){let item="<title>Webhook deprecation</title><description><![CDATA[Move before March]]></description>";assert_eq!(tag(item,"title"),"Webhook deprecation");assert_eq!(tag(item,"description"),"Move before March");} #[test] fn rate_limits_include_retry_after(){let response=with_retry_after(Response::builder().status(StatusCode::TOO_MANY_REQUESTS).body(axum::body::Body::empty()).unwrap());assert_eq!(response.headers().get(header::RETRY_AFTER).unwrap(),"1");} }

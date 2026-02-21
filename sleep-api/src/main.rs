mod app;
mod auth;
mod config;
mod db;
mod domain;
mod error;
mod handlers;
mod middleware;
mod models;
mod repository;
mod security;
mod time;
mod trends;

use crate::db::connect;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let pool = connect().await?;
    sqlx::migrate!("../migrations").run(&pool).await?;
    let app = app::router(pool);
    let bind_addr = config::api_bind_addr();
    let listener = TcpListener::bind(&bind_addr).await?;
    tracing::info!(%bind_addr, "API listening");
    axum::serve(listener, app).await?;
    Ok(())
}

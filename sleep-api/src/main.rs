mod app;
mod db;
mod error;
mod handlers;
mod models;

use crate::db::connect;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let pool = connect().await?;
    sqlx::migrate!("../migrations").run(&pool).await?;
    let app = app::router(pool);
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

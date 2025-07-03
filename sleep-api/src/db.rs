use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub type Db = Pool<Sqlite>;

pub async fn connect() -> Result<Db, sqlx::Error> {
    dotenvy::dotenv().ok();
    use std::io;
    let url = std::env::var("DATABASE_URL").map_err(|_| {
        sqlx::Error::Configuration(
            io::Error::new(io::ErrorKind::NotFound, "DATABASE_URL missing").into(),
        )
    })?;
    let pool = SqlitePoolOptions::new().connect(&url).await?;
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;
    Ok(pool)
}

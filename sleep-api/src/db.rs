use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub type Db = Pool<Sqlite>;

pub async fn connect() -> Result<Db, sqlx::Error> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    SqlitePoolOptions::new().connect(&url).await
}

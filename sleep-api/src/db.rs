#![doc = r#"Database utilities

Provides the shared Sqlite connection pool type [`Db`] and a helper to connect
and enforce SQLite foreign key constraints.

[`Db`]: crate::db::Db
"#]

use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

/// Pooled Sqlite connection handle used by the application.
///
/// This is a type alias for [`sqlx::Pool<sqlx::Sqlite>`].
pub type Db = Pool<Sqlite>;

#[doc = r#"Connect to the database and enable SQLite foreign keys (`PRAGMA foreign_keys = ON`).

Reads the `DATABASE_URL` environment variable (e.g., `sqlite::memory:` or a file path),
establishes a connection pool, and enables foreign key constraints.

# Example
```rust,no_run
# use std::error::Error;
# async fn demo() -> Result<(), Box<dyn Error>> {
// Use an in-memory database for demonstration.
# // DATABASE_URL should be configured in the environment for this example.
let db = sleep_api::db::connect().await?;

// Simple sanity query
sqlx::query("SELECT 1").execute(&db).await?;
# Ok(()) }
```

# Errors
- Returns [`sqlx::Error::Configuration`] if `DATABASE_URL` is missing.
- Returns other [`sqlx::Error`] variants if the connection or PRAGMA execution fails.

[`sqlx::Error::Configuration`]: sqlx::Error
"#]
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

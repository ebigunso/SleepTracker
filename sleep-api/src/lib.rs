#![doc = r#"
SleepTracker API library

This crate provides the building blocks of a small sleep‑tracking API built with Axum and SQLx.
It exposes modules for HTTP routing, persistence, domain models and time handling.

Key modules:
- [`app`] — HTTP router wiring all routes.
- [`db`] — database pool and connection utilities.
- [`models`] — input/output types with validation.
- [`repository`] — persistence operations.
- [`time`] — time and duration helpers including DST‑aware computations.
- [`trends`] and [`views`] — aggregation endpoints and templates.

Why: use this crate to embed the API server in your binary, or reuse its types and helpers like [`compute_duration_min`].

# Example

Bootstrapping a Router:

```rust,no_run
# use std::error::Error;
# async fn demo() -> Result<(), Box<dyn Error>> {
let db = sleep_api::db::connect().await?;
let app = sleep_api::app::router(db);
// axum::serve(listener, app).await?;
# Ok(())
# }
```

Additional references:
- OpenAPI specification: https://github.com/ebigunso/SleepTracker/blob/main/openapi.yaml
- API examples: https://github.com/ebigunso/SleepTracker/blob/main/docs/api_examples.md

See also: [`time`], [`repository`], and [`models`].

[`app`]: crate::app
[`db`]: crate::db
[`models`]: crate::models
[`repository`]: crate::repository
[`time`]: crate::time
[`trends`]: crate::trends
[`views`]: crate::views
[`compute_duration_min`]: crate::time::compute_duration_min
"#]

pub mod app;
pub mod config;
pub mod db;
pub mod domain;
mod error;
mod handlers;
pub mod models;
pub mod repository;
pub mod time;
pub mod trends;
pub mod views;

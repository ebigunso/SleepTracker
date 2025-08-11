#![doc = r#"Trends API

Aggregations over recorded sleep data, exposed as Axum handlers.

Endpoints:
- `GET /api/trends/sleep-bars`
- `GET /api/trends/summary`

For HTTP examples, see `docs/api_examples.md` and the OpenAPI spec.
"#]

use crate::middleware::auth_layer::RequireSessionJson;
use crate::{db::Db, error::ApiError};
use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{Datelike, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Sqlite};
use std::collections::BTreeMap;

/// Helper to parse a date string and return ApiError with field name
fn parse_date_field(s: &str, field: &str) -> Result<NaiveDate, ApiError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| ApiError::InvalidInput(format!("invalid {field} date")))
}

/// Helper to parse and validate a from/to date range (YYYY-MM-DD)
fn parse_and_validate_date_range(from: &str, to: &str) -> Result<(NaiveDate, NaiveDate), ApiError> {
    let from_date = parse_date_field(from, "from")?;
    let to_date = parse_date_field(to, "to")?;
    if to_date < from_date {
        return Err(ApiError::InvalidInput("to must be >= from".into()));
    }
    Ok((from_date, to_date))
}

#[derive(Deserialize)]
#[doc = r#"Query parameters for trends endpoints.

- `from`, `to`: inclusive date range `YYYY-MM-DD`.
- `bucket`: optional `"day"` or `"week"` (summary only). Defaults to `"day"`.
"#]
pub struct RangeQuery {
    pub from: String,
    pub to: String,
    pub bucket: Option<String>, // day|week (for summary)
}

#[derive(Serialize)]
#[doc = r#"Bar data point for per-day sleep: local bed/wake times, optional quality/duration."#]
pub struct SleepBar {
    pub date: NaiveDate, // wake date
    pub bed_time: NaiveTime,
    pub wake_time: NaiveTime,
    pub quality: Option<i32>,      // optional for coloring
    pub duration_min: Option<i32>, // optional
}

#[derive(FromRow)]
struct SleepBarRow {
    wake_date: NaiveDate,
    bed_time: NaiveTime,
    wake_time: NaiveTime,
    quality: Option<i32>,
    duration_min: Option<i32>,
}

#[doc = r#"Return per-day sleep bars over a date range.

Validates the date range and fetches rows from the `v_daily_sleep` view.

Examples:
- HTTP usage: see `docs/api_examples.md` and the OpenAPI spec.

Errors:
- Returns an API error for invalid dates or if `to < from`.
- Returns an API error on database failures.
"#]
pub async fn sleep_bars(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    Query(q): Query<RangeQuery>,
) -> Result<Json<Vec<SleepBar>>, ApiError> {
    let (from, to) = parse_and_validate_date_range(&q.from, &q.to)?;

    // Pull from view; rely on server-computed duration_min
    let rows = sqlx::query_as::<Sqlite, SleepBarRow>(
        r#"
        SELECT wake_date, bed_time, wake_time, quality, duration_min
        FROM v_daily_sleep
        WHERE wake_date BETWEEN ? AND ?
        ORDER BY wake_date ASC
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(&db)
    .await?;

    let out = rows
        .into_iter()
        .map(|r| SleepBar {
            date: r.wake_date,
            bed_time: r.bed_time,
            wake_time: r.wake_time,
            quality: r.quality,
            duration_min: r.duration_min,
        })
        .collect();

    Ok(Json(out))
}

#[derive(Serialize, Clone)]
#[doc = r#"Aggregated duration statistics per bucket (`bucket` is a date or ISO week)."#]
pub struct DurationBucket {
    pub bucket: String,
    pub avg_min: f64,
    pub min_min: i32,
    pub max_min: i32,
}

#[derive(Serialize, Clone)]
#[doc = r#"Average quality per bucket."#]
pub struct QualityBucket {
    pub bucket: String,
    pub avg: f64,
}

#[derive(Serialize, Clone)]
#[doc = r#"Median latency per bucket (computed via selection)."#]
pub struct LatencyBucket {
    pub bucket: String,
    pub median: f64,
}

#[derive(Serialize)]
#[doc = r#"Aggregated trends response combining duration, quality, and latency buckets."#]
pub struct SummaryResponse {
    pub duration_by_bucket: Vec<DurationBucket>,
    pub quality_by_bucket: Vec<QualityBucket>,
    pub latency_by_bucket: Vec<LatencyBucket>,
}

#[derive(FromRow)]
struct SummaryRow {
    wake_date: NaiveDate,
    duration_min: i32,
    quality: i32,
    latency_min: i32,
}

#[doc = r#"Return aggregated summary statistics over a date range.

When `bucket` is `"day"` (default), groups by date; when `"week"`, groups by ISO week (YYYY-Www).

Examples:
- HTTP usage: see `docs/api_examples.md` and the OpenAPI spec.

Errors:
- Returns an API error for invalid dates or invalid `bucket` values.
- Returns an API error on database failures.
"#]
pub async fn summary(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    Query(q): Query<RangeQuery>,
) -> Result<Json<SummaryResponse>, ApiError> {
    let (from, to) = parse_and_validate_date_range(&q.from, &q.to)?;

    let bucket = q.bucket.as_deref().unwrap_or("day");
    if bucket != "day" && bucket != "week" {
        return Err(ApiError::InvalidInput("bucket must be day or week".into()));
    }

    // Pull per-day rows; aggregate in Rust for day/week.
    let rows = sqlx::query_as::<Sqlite, SummaryRow>(
        r#"
        SELECT wake_date, duration_min, quality, latency_min
        FROM v_daily_sleep
        WHERE wake_date BETWEEN ? AND ?
        ORDER BY wake_date ASC
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(&db)
    .await?;

    // Group by bucket key
    let mut by_bucket: BTreeMap<String, Vec<(i32, i32, i32)>> = BTreeMap::new();
    for r in rows {
        let key = if bucket == "day" {
            r.wake_date.format("%Y-%m-%d").to_string()
        } else {
            // week: ISO week keyed to Monday; format "YYYY-Www"
            let iw = r.wake_date.iso_week();
            format!("{:04}-W{:02}", iw.year(), iw.week())
        };
        by_bucket
            .entry(key)
            .or_default()
            .push((r.duration_min, r.quality, r.latency_min));
    }

    let mut duration_buckets = Vec::new();
    let mut quality_buckets = Vec::new();
    let mut latency_buckets = Vec::new();

    for (bucket_key, vals) in by_bucket {
        if vals.is_empty() {
            continue;
        }
        let count = vals.len();
        let mut sum_dur = 0i64;
        let mut min_dur = i32::MAX;
        let mut max_dur = i32::MIN;

        let mut sum_quality = 0i64;
        let mut latencies = Vec::with_capacity(vals.len());

        for (dur, qual, lat) in vals {
            sum_dur += dur as i64;
            min_dur = min_dur.min(dur);
            max_dur = max_dur.max(dur);

            sum_quality += qual as i64;
            latencies.push(lat);
        }

        let avg_min = (sum_dur as f64) / (count as f64);
        let avg_quality = (sum_quality as f64) / (count as f64);

        // median latency in O(n) expected time using selection instead of full sort
        // Note: select_nth_unstable permutes the contents of `latencies`. This is acceptable here
        // because `latencies` is built per-bucket and not used after computing the median.
        // Cloning to avoid mutation would add O(n) time and memory per bucket and reduce the
        // performance benefit of using selection.
        let n = latencies.len();
        let median = if n % 2 == 1 {
            let mid = n / 2;
            let (_low, nth, _high) = latencies.select_nth_unstable(mid);
            *nth as f64
        } else {
            // For even n, select the upper middle, then average with max of lower partition
            let mid = n / 2;
            let (low, nth, _high) = latencies.select_nth_unstable(mid);
            debug_assert!(
                mid > 0 && low.len() == mid,
                "select_nth_unstable invariant: for even n, low partition must have mid elements"
            );
            let lower_max = *low
                .iter()
                .max()
                .expect("median: low.len() != mid or low empty (unexpected for even n)")
                as f64;
            let upper_min = *nth as f64;
            (lower_max + upper_min) / 2.0
        };

        duration_buckets.push(DurationBucket {
            bucket: bucket_key.clone(),
            avg_min,
            min_min: min_dur,
            max_min: max_dur,
        });
        quality_buckets.push(QualityBucket {
            bucket: bucket_key.clone(),
            avg: avg_quality,
        });
        latency_buckets.push(LatencyBucket {
            bucket: bucket_key,
            median,
        });
    }

    Ok(Json(SummaryResponse {
        duration_by_bucket: duration_buckets,
        quality_by_bucket: quality_buckets,
        latency_by_bucket: latency_buckets,
    }))
}

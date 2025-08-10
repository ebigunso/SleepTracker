use crate::{db::Db, error::ApiError};
use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{Datelike, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Sqlite};
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct RangeQuery {
    pub from: String,
    pub to: String,
    pub bucket: Option<String>, // day|week (for summary)
}

#[derive(Serialize)]
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

pub async fn sleep_bars(
    State(db): State<Db>,
    Query(q): Query<RangeQuery>,
) -> Result<Json<Vec<SleepBar>>, ApiError> {
    let from = NaiveDate::parse_from_str(&q.from, "%Y-%m-%d")
        .map_err(|_| ApiError::InvalidInput("invalid from date".into()))?;
    let to = NaiveDate::parse_from_str(&q.to, "%Y-%m-%d")
        .map_err(|_| ApiError::InvalidInput("invalid to date".into()))?;
    if to < from {
        return Err(ApiError::InvalidInput("to must be >= from".into()));
    }

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
pub struct DurationBucket {
    pub bucket: String,
    pub avg_min: f64,
    pub min_min: i32,
    pub max_min: i32,
}

#[derive(Serialize, Clone)]
pub struct QualityBucket {
    pub bucket: String,
    pub avg: f64,
}

#[derive(Serialize, Clone)]
pub struct LatencyBucket {
    pub bucket: String,
    pub median: f64,
}

#[derive(Serialize)]
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

pub async fn summary(
    State(db): State<Db>,
    Query(q): Query<RangeQuery>,
) -> Result<Json<SummaryResponse>, ApiError> {
    let from = NaiveDate::parse_from_str(&q.from, "%Y-%m-%d")
        .map_err(|_| ApiError::InvalidInput("invalid from date".into()))?;
    let to = NaiveDate::parse_from_str(&q.to, "%Y-%m-%d")
        .map_err(|_| ApiError::InvalidInput("invalid to date".into()))?;
    if to < from {
        return Err(ApiError::InvalidInput("to must be >= from".into()));
    }

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

        // median latency (use selection for O(n) average)
        let n = latencies.len();
        let median = if n % 2 == 1 {
            let mid = n / 2;
            let (_, v, _) = latencies.select_nth_unstable(mid);
            *v as f64
        } else {
            let mid = n / 2;
            let (less, v, _) = latencies.select_nth_unstable(mid);
            let hi = *v as f64;
            let low = *less.iter().max().unwrap() as f64;
            (low + hi) / 2.0
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

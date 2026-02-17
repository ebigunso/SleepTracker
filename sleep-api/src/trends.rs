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
use chrono::{Datelike, Duration as ChronoDuration, NaiveDate, NaiveTime, Timelike, Utc, Weekday};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Sqlite};
use std::collections::{BTreeMap, HashSet};

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

#[derive(Deserialize)]
#[doc = r#"Query parameters for personalization trends endpoint.

- `window_days`: optional rolling window size in days. Defaults to 28.
- `to`: optional inclusive end date `YYYY-MM-DD`. Defaults to server current UTC date.
"#]
pub struct PersonalizationQuery {
    pub window_days: Option<i64>,
    pub to: Option<String>,
}

#[derive(Serialize)]
pub struct PersonalizationWindow {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub logged_days: usize,
    pub missing_days: i64,
    pub missing_days_pct: f64,
}

#[derive(Serialize)]
pub struct DurationBaselineMetric {
    pub eligible: bool,
    pub sample_days: usize,
    pub p10_min: Option<f64>,
    pub p50_min: Option<f64>,
    pub p90_min: Option<f64>,
    pub iqr_min: Option<f64>,
    pub recent_out_of_range_incidence_pct: Option<f64>,
}

#[derive(Serialize)]
pub struct DayTypeTimingBaselineMetric {
    pub eligible: bool,
    pub weekday_sample_days: usize,
    pub weekend_sample_days: usize,
    pub weekday_bed_median_min: Option<f64>,
    pub weekday_wake_median_min: Option<f64>,
    pub weekend_bed_median_min: Option<f64>,
    pub weekend_wake_median_min: Option<f64>,
    pub midpoint_stable_across_windows: bool,
    pub recent_14_day_diverges_from_baseline: bool,
}

#[derive(Serialize)]
pub struct SocialJetlagMetric {
    pub eligible: bool,
    pub weekend_sample_days: usize,
    pub current_delta_min: Option<f64>,
    pub prior_delta_min: Option<f64>,
    pub sustained_two_windows: bool,
}

#[derive(Serialize)]
pub struct ScheduleVariabilityMetric {
    pub eligible: bool,
    pub current_variability_min: Option<f64>,
    pub prior_variability_min: Option<f64>,
    pub sustained_two_windows: bool,
    pub high_data_gap: bool,
}

#[derive(Serialize)]
pub struct RankedQualityFactor {
    pub factor: String,
    pub effect: f64,
}

#[derive(Serialize)]
pub struct QualityFactorRankingMetric {
    pub eligible: bool,
    pub sessions_with_quality: usize,
    pub distinct_quality_values: usize,
    pub stable_across_adjacent_windows: bool,
    pub ranked_factors: Vec<RankedQualityFactor>,
}

#[derive(Serialize)]
pub struct PersonalizationMetrics {
    pub duration_baseline: DurationBaselineMetric,
    pub day_type_timing_baseline: DayTypeTimingBaselineMetric,
    pub social_jetlag: SocialJetlagMetric,
    pub schedule_variability: ScheduleVariabilityMetric,
    pub quality_factor_ranking: QualityFactorRankingMetric,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationStatus {
    Recommended,
    Suppressed,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Serialize)]
pub struct ActionRecommendation {
    pub action_key: String,
    pub status: RecommendationStatus,
    pub confidence: Confidence,
    pub rationale: String,
    pub suppression_reasons: Vec<String>,
}

#[derive(Serialize)]
pub struct PersonalizationResponse {
    pub as_of: NaiveDate,
    pub window_days: i64,
    pub current_window: PersonalizationWindow,
    pub prior_window: PersonalizationWindow,
    pub metrics: PersonalizationMetrics,
    pub recommendations: Vec<ActionRecommendation>,
}

#[derive(FromRow)]
struct PersonalizationDailyRow {
    wake_date: NaiveDate,
    bed_time: NaiveTime,
    wake_time: NaiveTime,
    duration_min: i32,
    quality: Option<i32>,
}

#[derive(Clone)]
struct DaySample {
    wake_date: NaiveDate,
    bed_clock_min: f64,
    wake_clock_min: f64,
    bed_relative_min: f64,
    wake_relative_min: f64,
    midpoint_clock_min: f64,
    duration_min: f64,
    quality: Option<f64>,
    weekend: bool,
}

struct PersonalizationCalc {
    current_window: PersonalizationWindow,
    prior_window: PersonalizationWindow,
    metrics: PersonalizationMetrics,
    recommendations: Vec<ActionRecommendation>,
}

#[doc = r#"Return rolling-window personalization metrics and action recommendations.

Uses wake-date semantics through `v_daily_sleep` (daily aggregated view), compares the current
rolling window with the immediately previous window of equal length, and evaluates triggers and
guardrails from `docs/personalization-agent-action-map.md`.

Errors:
- Returns an API error for invalid dates or invalid `window_days` values.
- Returns an API error on database failures.
"#]
pub async fn personalization(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    Query(q): Query<PersonalizationQuery>,
) -> Result<Json<PersonalizationResponse>, ApiError> {
    let window_days = q.window_days.unwrap_or(28);
    if !(1..=365).contains(&window_days) {
        return Err(ApiError::InvalidInput(
            "window_days must be between 1 and 365".into(),
        ));
    }

    let as_of = match q.to.as_deref() {
        Some(s) => parse_date_field(s, "to")?,
        None => Utc::now().date_naive(),
    };

    let current_from = as_of
        .checked_sub_signed(ChronoDuration::days(window_days - 1))
        .ok_or_else(|| ApiError::InvalidInput("invalid date range".into()))?;
    let prior_to = current_from
        .pred_opt()
        .ok_or_else(|| ApiError::InvalidInput("invalid date range".into()))?;
    let prior_from = prior_to
        .checked_sub_signed(ChronoDuration::days(window_days - 1))
        .ok_or_else(|| ApiError::InvalidInput("invalid date range".into()))?;

    let rows = sqlx::query_as::<Sqlite, PersonalizationDailyRow>(
        r#"
        SELECT wake_date, bed_time, wake_time, duration_min, quality
        FROM v_daily_sleep
        WHERE wake_date BETWEEN ? AND ?
        ORDER BY wake_date ASC
        "#,
    )
    .bind(prior_from)
    .bind(as_of)
    .fetch_all(&db)
    .await?;

    let samples: Vec<DaySample> = rows.into_iter().map(to_day_sample).collect();

    let current_samples: Vec<DaySample> = samples
        .iter()
        .filter(|s| s.wake_date >= current_from && s.wake_date <= as_of)
        .cloned()
        .collect();
    let prior_samples: Vec<DaySample> = samples
        .iter()
        .filter(|s| s.wake_date >= prior_from && s.wake_date <= prior_to)
        .cloned()
        .collect();

    let calc = build_personalization_calc(
        &current_samples,
        &prior_samples,
        current_from,
        as_of,
        prior_from,
        prior_to,
        window_days,
    );

    Ok(Json(PersonalizationResponse {
        as_of,
        window_days,
        current_window: calc.current_window,
        prior_window: calc.prior_window,
        metrics: calc.metrics,
        recommendations: calc.recommendations,
    }))
}

fn to_day_sample(row: PersonalizationDailyRow) -> DaySample {
    let bed_clock_min = minutes_of_day(row.bed_time) as f64;
    let wake_clock_min = minutes_of_day(row.wake_time) as f64;
    let crosses_midnight = row.bed_time > row.wake_time;
    let bed_relative_min = if crosses_midnight {
        bed_clock_min - 24.0 * 60.0
    } else {
        bed_clock_min
    };
    let wake_relative_min = wake_clock_min;
    let midpoint_clock_min = normalize_minutes(wake_clock_min - (row.duration_min as f64) / 2.0);

    DaySample {
        wake_date: row.wake_date,
        bed_clock_min,
        wake_clock_min,
        bed_relative_min,
        wake_relative_min,
        midpoint_clock_min,
        duration_min: row.duration_min as f64,
        quality: row.quality.map(|q| q as f64),
        weekend: matches!(row.wake_date.weekday(), Weekday::Sat | Weekday::Sun),
    }
}

fn minutes_of_day(t: NaiveTime) -> i32 {
    (t.hour() as i32) * 60 + (t.minute() as i32)
}

fn normalize_minutes(min: f64) -> f64 {
    let day = 24.0 * 60.0;
    ((min % day) + day) % day
}

fn circular_minutes_diff(a: f64, b: f64) -> f64 {
    let day = 24.0 * 60.0;
    let diff = (a - b).rem_euclid(day);
    diff.min(day - diff)
}

fn circular_signed_minutes_delta(later: f64, earlier: f64) -> f64 {
    let day = 24.0 * 60.0;
    ((later - earlier + day / 2.0).rem_euclid(day)) - day / 2.0
}

fn within_clock_minute_band(value: f64, center: f64, threshold_min: f64) -> bool {
    circular_minutes_diff(value, center) <= threshold_min
}

fn percentile_linear(values: &[f64], p: f64) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let rank = p.clamp(0.0, 1.0) * ((sorted.len() - 1) as f64);
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    if lower == upper {
        return sorted.get(lower).copied();
    }
    let weight = rank - (lower as f64);
    Some(sorted[lower] * (1.0 - weight) + sorted[upper] * weight)
}

fn median(values: &[f64]) -> Option<f64> {
    percentile_linear(values, 0.5)
}

fn std_dev(values: &[f64]) -> Option<f64> {
    if values.len() < 2 {
        return None;
    }
    let mean = values.iter().sum::<f64>() / (values.len() as f64);
    let var = values
        .iter()
        .map(|v| {
            let d = *v - mean;
            d * d
        })
        .sum::<f64>()
        / (values.len() as f64);
    Some(var.sqrt())
}

fn abs_diff(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(x), Some(y)) => Some(circular_minutes_diff(x, y)),
        _ => None,
    }
}

fn pct(part: usize, total: usize) -> Option<f64> {
    if total == 0 {
        return None;
    }
    Some((part as f64) * 100.0 / (total as f64))
}

fn window_stats(
    from: NaiveDate,
    to: NaiveDate,
    samples: &[DaySample],
    window_days: i64,
) -> PersonalizationWindow {
    let mut unique_days = HashSet::new();
    for sample in samples {
        unique_days.insert(sample.wake_date);
    }
    let logged_days = unique_days.len();
    let missing_days = (window_days - (logged_days as i64)).max(0);
    let missing_days_pct = (missing_days as f64) * 100.0 / (window_days as f64);

    PersonalizationWindow {
        from,
        to,
        logged_days,
        missing_days,
        missing_days_pct,
    }
}

fn day_type_medians(
    samples: &[DaySample],
) -> (
    Option<f64>,
    Option<f64>,
    usize,
    Option<f64>,
    Option<f64>,
    usize,
) {
    let weekday: Vec<&DaySample> = samples.iter().filter(|s| !s.weekend).collect();
    let weekend: Vec<&DaySample> = samples.iter().filter(|s| s.weekend).collect();

    let weekday_bed = weekday.iter().map(|s| s.bed_clock_min).collect::<Vec<_>>();
    let weekday_wake = weekday.iter().map(|s| s.wake_clock_min).collect::<Vec<_>>();

    let weekend_bed = weekend.iter().map(|s| s.bed_clock_min).collect::<Vec<_>>();
    let weekend_wake = weekend.iter().map(|s| s.wake_clock_min).collect::<Vec<_>>();

    (
        median(&weekday_bed),
        median(&weekday_wake),
        weekday.len(),
        median(&weekend_bed),
        median(&weekend_wake),
        weekend.len(),
    )
}

fn social_jetlag_delta(samples: &[DaySample]) -> (Option<f64>, usize) {
    let weekday_mid = samples
        .iter()
        .filter(|s| !s.weekend)
        .map(|s| s.midpoint_clock_min)
        .collect::<Vec<_>>();
    let weekend = samples.iter().filter(|s| s.weekend).collect::<Vec<_>>();
    let weekend_mid = weekend
        .iter()
        .map(|s| s.midpoint_clock_min)
        .collect::<Vec<_>>();

    let delta = match (median(&weekday_mid), median(&weekend_mid)) {
        (Some(wd), Some(we)) => Some(circular_signed_minutes_delta(we, wd)),
        _ => None,
    };

    (delta, weekend.len())
}

fn build_personalization_calc(
    current_samples: &[DaySample],
    prior_samples: &[DaySample],
    current_from: NaiveDate,
    current_to: NaiveDate,
    prior_from: NaiveDate,
    prior_to: NaiveDate,
    window_days: i64,
) -> PersonalizationCalc {
    let current_window = window_stats(current_from, current_to, current_samples, window_days);
    let prior_window = window_stats(prior_from, prior_to, prior_samples, window_days);

    let prior_durations = prior_samples
        .iter()
        .map(|s| s.duration_min)
        .collect::<Vec<_>>();
    let current_durations = current_samples
        .iter()
        .map(|s| s.duration_min)
        .collect::<Vec<_>>();

    let p10 = percentile_linear(&prior_durations, 0.10);
    let p50 = percentile_linear(&prior_durations, 0.50);
    let p90 = percentile_linear(&prior_durations, 0.90);
    let p25 = percentile_linear(&prior_durations, 0.25);
    let p75 = percentile_linear(&prior_durations, 0.75);
    let iqr = match (p25, p75) {
        (Some(a), Some(b)) => Some(b - a),
        _ => None,
    };

    let out_of_range_count = match (p10, p90) {
        (Some(low), Some(high)) => current_durations
            .iter()
            .filter(|v| **v < low || **v > high)
            .count(),
        _ => 0,
    };
    let out_of_range_pct = pct(out_of_range_count, current_durations.len());

    let duration_eligible = prior_durations.len() >= 60;
    let duration_trigger = duration_eligible && out_of_range_pct.unwrap_or(0.0) >= 5.0;

    let (cwd_bed, cwd_wake, cwd_n, cwe_bed, cwe_wake, cwe_n) = day_type_medians(current_samples);

    let current_weekday_mid = current_samples
        .iter()
        .filter(|s| !s.weekend)
        .map(|s| s.midpoint_clock_min)
        .collect::<Vec<_>>();
    let current_weekend_mid = current_samples
        .iter()
        .filter(|s| s.weekend)
        .map(|s| s.midpoint_clock_min)
        .collect::<Vec<_>>();
    let prior_weekday_mid = prior_samples
        .iter()
        .filter(|s| !s.weekend)
        .map(|s| s.midpoint_clock_min)
        .collect::<Vec<_>>();
    let prior_weekend_mid = prior_samples
        .iter()
        .filter(|s| s.weekend)
        .map(|s| s.midpoint_clock_min)
        .collect::<Vec<_>>();

    let weekday_mid_stable = abs_diff(median(&current_weekday_mid), median(&prior_weekday_mid))
        .map(|d| d <= 30.0)
        .unwrap_or(false);
    let weekend_mid_stable = abs_diff(median(&current_weekend_mid), median(&prior_weekend_mid))
        .map(|d| d <= 30.0)
        .unwrap_or(false);
    let midpoint_stable_across_windows = weekday_mid_stable && weekend_mid_stable;

    let baseline_midpoint_by_daytype = (median(&prior_weekday_mid), median(&prior_weekend_mid));
    let recent_14 = current_samples
        .iter()
        .rev()
        .take(14)
        .cloned()
        .collect::<Vec<_>>();
    let recent_14_weekday_mid = recent_14
        .iter()
        .filter(|s| !s.weekend)
        .map(|s| s.midpoint_clock_min)
        .collect::<Vec<_>>();
    let recent_14_weekend_mid = recent_14
        .iter()
        .filter(|s| s.weekend)
        .map(|s| s.midpoint_clock_min)
        .collect::<Vec<_>>();
    let recent_diverges = abs_diff(
        median(&recent_14_weekday_mid),
        baseline_midpoint_by_daytype.0,
    )
    .map(|d| d > 90.0)
    .unwrap_or(false)
        || abs_diff(
            median(&recent_14_weekend_mid),
            baseline_midpoint_by_daytype.1,
        )
        .map(|d| d > 90.0)
        .unwrap_or(false);

    let day_type_eligible = cwd_n >= 8 && cwe_n >= 4 && midpoint_stable_across_windows;

    let (social_current, social_weekend_n) = social_jetlag_delta(current_samples);
    let (social_prior, social_prior_weekend_n) = social_jetlag_delta(prior_samples);
    let social_sustained = social_current.map(|d| d.abs() >= 30.0).unwrap_or(false)
        && social_prior.map(|d| d.abs() >= 30.0).unwrap_or(false);
    let social_eligible = social_weekend_n >= 4 && social_prior_weekend_n >= 4;

    let current_var = {
        let bed = current_samples
            .iter()
            .map(|s| s.bed_relative_min)
            .collect::<Vec<_>>();
        let wake = current_samples
            .iter()
            .map(|s| s.wake_relative_min)
            .collect::<Vec<_>>();
        match (std_dev(&bed), std_dev(&wake)) {
            (Some(b), Some(w)) => Some((b + w) / 2.0),
            _ => None,
        }
    };
    let prior_var = {
        let bed = prior_samples
            .iter()
            .map(|s| s.bed_relative_min)
            .collect::<Vec<_>>();
        let wake = prior_samples
            .iter()
            .map(|s| s.wake_relative_min)
            .collect::<Vec<_>>();
        match (std_dev(&bed), std_dev(&wake)) {
            (Some(b), Some(w)) => Some((b + w) / 2.0),
            _ => None,
        }
    };
    let variability_sustained = current_var.map(|v| v >= 60.0).unwrap_or(false)
        && prior_var.map(|v| v >= 60.0).unwrap_or(false);
    let high_gap = current_window.missing_days_pct > 30.0;
    let variability_eligible = current_var.is_some() && prior_var.is_some();

    let quality_samples_current = current_samples
        .iter()
        .filter_map(|s| s.quality.map(|q| (s, q)))
        .collect::<Vec<_>>();
    let quality_values = quality_samples_current
        .iter()
        .map(|(_, q)| *q as i32)
        .collect::<HashSet<_>>();
    let quality_base_eligible = quality_samples_current.len() >= 40 && quality_values.len() >= 3;

    let current_mid_median = median(
        &current_samples
            .iter()
            .map(|s| s.midpoint_clock_min)
            .collect::<Vec<_>>(),
    );
    let prior_mid_median = median(
        &prior_samples
            .iter()
            .map(|s| s.midpoint_clock_min)
            .collect::<Vec<_>>(),
    );
    let duration_window_p10 = percentile_linear(&current_durations, 0.10);
    let duration_window_p90 = percentile_linear(&current_durations, 0.90);
    let prior_duration_values = prior_samples
        .iter()
        .map(|s| s.duration_min)
        .collect::<Vec<_>>();
    let prior_duration_p10 = percentile_linear(&prior_duration_values, 0.10);
    let prior_duration_p90 = percentile_linear(&prior_duration_values, 0.90);

    let factor_effect = |samples: &[DaySample],
                         midpoint_median: Option<f64>,
                         p10: Option<f64>,
                         p90: Option<f64>,
                         factor: &str| {
        let mut favorable = Vec::new();
        let mut unfavorable = Vec::new();
        for s in samples {
            let Some(q) = s.quality else { continue };
            let is_favorable = match factor {
                "duration_in_personal_range" => match (p10, p90) {
                    (Some(low), Some(high)) => s.duration_min >= low && s.duration_min <= high,
                    _ => false,
                },
                "consistent_mid_sleep_timing" => midpoint_median
                    .map(|m| within_clock_minute_band(s.midpoint_clock_min, m, 60.0))
                    .unwrap_or(false),
                "weekday_schedule_alignment" => {
                    if s.weekend {
                        midpoint_median
                            .map(|m| within_clock_minute_band(s.midpoint_clock_min, m, 90.0))
                            .unwrap_or(false)
                    } else {
                        true
                    }
                }
                _ => false,
            };

            if is_favorable {
                favorable.push(q);
            } else {
                unfavorable.push(q);
            }
        }

        if favorable.len() < 5 || unfavorable.len() < 5 {
            return None;
        }
        let f_mean = favorable.iter().sum::<f64>() / (favorable.len() as f64);
        let u_mean = unfavorable.iter().sum::<f64>() / (unfavorable.len() as f64);
        Some(f_mean - u_mean)
    };

    let factors = [
        "duration_in_personal_range",
        "consistent_mid_sleep_timing",
        "weekday_schedule_alignment",
    ];

    let mut ranked_factors = Vec::new();
    let mut stable_effects = true;
    for factor in factors {
        let cur = factor_effect(
            current_samples,
            current_mid_median,
            duration_window_p10,
            duration_window_p90,
            factor,
        );
        let prv = factor_effect(
            prior_samples,
            prior_mid_median,
            prior_duration_p10,
            prior_duration_p90,
            factor,
        );

        match (cur, prv) {
            (Some(c), Some(p)) => {
                if c.signum() != p.signum() {
                    stable_effects = false;
                }
                ranked_factors.push(RankedQualityFactor {
                    factor: factor.to_string(),
                    effect: c,
                });
            }
            _ => {
                stable_effects = false;
            }
        }
    }
    ranked_factors.sort_by(|a, b| b.effect.abs().total_cmp(&a.effect.abs()));
    ranked_factors.truncate(3);

    let quality_eligible = quality_base_eligible && stable_effects;

    let duration_metric = DurationBaselineMetric {
        eligible: duration_eligible,
        sample_days: prior_durations.len(),
        p10_min: p10,
        p50_min: p50,
        p90_min: p90,
        iqr_min: iqr,
        recent_out_of_range_incidence_pct: out_of_range_pct,
    };
    let day_type_metric = DayTypeTimingBaselineMetric {
        eligible: day_type_eligible,
        weekday_sample_days: cwd_n,
        weekend_sample_days: cwe_n,
        weekday_bed_median_min: cwd_bed,
        weekday_wake_median_min: cwd_wake,
        weekend_bed_median_min: cwe_bed,
        weekend_wake_median_min: cwe_wake,
        midpoint_stable_across_windows,
        recent_14_day_diverges_from_baseline: recent_diverges,
    };
    let social_metric = SocialJetlagMetric {
        eligible: social_eligible,
        weekend_sample_days: social_weekend_n,
        current_delta_min: social_current,
        prior_delta_min: social_prior,
        sustained_two_windows: social_sustained,
    };
    let variability_metric = ScheduleVariabilityMetric {
        eligible: variability_eligible,
        current_variability_min: current_var,
        prior_variability_min: prior_var,
        sustained_two_windows: variability_sustained,
        high_data_gap: high_gap,
    };
    let quality_metric = QualityFactorRankingMetric {
        eligible: quality_eligible,
        sessions_with_quality: quality_samples_current.len(),
        distinct_quality_values: quality_values.len(),
        stable_across_adjacent_windows: stable_effects,
        ranked_factors,
    };

    let recommendations = build_recommendations(
        duration_trigger,
        &duration_metric,
        &day_type_metric,
        &social_metric,
        &variability_metric,
        &quality_metric,
    );

    PersonalizationCalc {
        current_window,
        prior_window,
        metrics: PersonalizationMetrics {
            duration_baseline: duration_metric,
            day_type_timing_baseline: day_type_metric,
            social_jetlag: social_metric,
            schedule_variability: variability_metric,
            quality_factor_ranking: quality_metric,
        },
        recommendations,
    }
}

fn build_recommendations(
    duration_trigger: bool,
    duration_metric: &DurationBaselineMetric,
    day_type_metric: &DayTypeTimingBaselineMetric,
    social_metric: &SocialJetlagMetric,
    variability_metric: &ScheduleVariabilityMetric,
    quality_metric: &QualityFactorRankingMetric,
) -> Vec<ActionRecommendation> {
    let mut out = Vec::new();

    let mut duration_suppression = Vec::new();
    if !duration_metric.eligible {
        duration_suppression
            .push("needs at least 60 baseline sessions in prior window".to_string());
    }
    if !duration_trigger {
        duration_suppression.push("recent out-of-range incidence is below 5% trigger".to_string());
    }
    duration_suppression.push(
        "schedule disruption guardrail unavailable from current data (travel/shift period not inferred)"
            .to_string(),
    );
    out.push(ActionRecommendation {
        action_key: "personal_duration_warning_tuning".to_string(),
        status: if duration_metric.eligible && duration_trigger {
            RecommendationStatus::Recommended
        } else {
            RecommendationStatus::Suppressed
        },
        confidence: if duration_metric.eligible && duration_trigger {
            Confidence::Medium
        } else {
            Confidence::Low
        },
        rationale: "Replace static unusual-duration warning with personal duration tails"
            .to_string(),
        suppression_reasons: if duration_metric.eligible && duration_trigger {
            Vec::new()
        } else {
            duration_suppression
        },
    });

    let mut day_type_suppression = Vec::new();
    if day_type_metric.weekday_sample_days < 8 || day_type_metric.weekend_sample_days < 4 {
        day_type_suppression
            .push("requires >=8 weekday and >=4 weekend sessions in current window".to_string());
    }
    if !day_type_metric.midpoint_stable_across_windows {
        day_type_suppression
            .push("day-type midpoint medians are not stable across adjacent windows".to_string());
    }
    if day_type_metric.recent_14_day_diverges_from_baseline {
        day_type_suppression
            .push("recent 14-day pattern strongly diverges from baseline".to_string());
    }
    out.push(ActionRecommendation {
        action_key: "day_type_default_prefill".to_string(),
        status: if day_type_metric.eligible && !day_type_metric.recent_14_day_diverges_from_baseline
        {
            RecommendationStatus::Recommended
        } else {
            RecommendationStatus::Suppressed
        },
        confidence: if day_type_metric.eligible {
            Confidence::Medium
        } else {
            Confidence::Low
        },
        rationale: "Offer weekday/weekend usual-time defaults when day-type baselines are stable"
            .to_string(),
        suppression_reasons: if day_type_metric.eligible
            && !day_type_metric.recent_14_day_diverges_from_baseline
        {
            Vec::new()
        } else {
            day_type_suppression
        },
    });

    let mut social_suppression = Vec::new();
    if social_metric.weekend_sample_days < 4 {
        social_suppression.push("weekend sample too small (<4 sessions)".to_string());
    }
    if !social_metric.sustained_two_windows {
        social_suppression
            .push("social jetlag delta is not >=30 min for two consecutive windows".to_string());
    }
    out.push(ActionRecommendation {
        action_key: "social_jetlag_schedule_shift_insight".to_string(),
        status: if social_metric.eligible && social_metric.sustained_two_windows {
            RecommendationStatus::Recommended
        } else {
            RecommendationStatus::Suppressed
        },
        confidence: if social_metric.eligible && social_metric.sustained_two_windows {
            Confidence::High
        } else {
            Confidence::Low
        },
        rationale: "Show weekend-vs-weekday midpoint shift insight when phase shift persists"
            .to_string(),
        suppression_reasons: if social_metric.eligible && social_metric.sustained_two_windows {
            Vec::new()
        } else {
            social_suppression
        },
    });

    let mut variability_suppression = Vec::new();
    if !variability_metric.sustained_two_windows {
        variability_suppression
            .push("schedule variability is not >=60 minutes across two windows".to_string());
    }
    if variability_metric.high_data_gap {
        variability_suppression.push("data gaps exceed 30% of current window".to_string());
    }
    out.push(ActionRecommendation {
        action_key: "regularity_insight_priority".to_string(),
        status: if variability_metric.eligible
            && variability_metric.sustained_two_windows
            && !variability_metric.high_data_gap
        {
            RecommendationStatus::Recommended
        } else {
            RecommendationStatus::Suppressed
        },
        confidence: if variability_metric.eligible && variability_metric.sustained_two_windows {
            Confidence::Medium
        } else {
            Confidence::Low
        },
        rationale: "Prioritize regularity insight when timing dispersion is persistently high"
            .to_string(),
        suppression_reasons: if variability_metric.eligible
            && variability_metric.sustained_two_windows
            && !variability_metric.high_data_gap
        {
            Vec::new()
        } else {
            variability_suppression
        },
    });

    let mut quality_suppression = Vec::new();
    if quality_metric.sessions_with_quality < 40 {
        quality_suppression.push("requires at least 40 sessions with quality".to_string());
    }
    if quality_metric.distinct_quality_values < 3 {
        quality_suppression.push("requires at least 3 distinct quality values".to_string());
    }
    if !quality_metric.stable_across_adjacent_windows {
        quality_suppression
            .push("factor effects are not stable across adjacent windows".to_string());
    }
    out.push(ActionRecommendation {
        action_key: "quality_aligned_factor_explanation".to_string(),
        status: if quality_metric.eligible {
            RecommendationStatus::Recommended
        } else {
            RecommendationStatus::Suppressed
        },
        confidence: if quality_metric.eligible {
            Confidence::Medium
        } else {
            Confidence::Low
        },
        rationale:
            "Surface top associated factors for higher-quality nights using directional language"
                .to_string(),
        suppression_reasons: if quality_metric.eligible {
            Vec::new()
        } else {
            quality_suppression
        },
    });

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_day_sample(
        wake_date: NaiveDate,
        weekend: bool,
        midpoint_clock_min: f64,
        quality: Option<f64>,
    ) -> DaySample {
        DaySample {
            wake_date,
            bed_clock_min: 23.0 * 60.0,
            wake_clock_min: 7.0 * 60.0,
            bed_relative_min: -60.0,
            wake_relative_min: 7.0 * 60.0,
            midpoint_clock_min: normalize_minutes(midpoint_clock_min),
            duration_min: 8.0 * 60.0,
            quality,
            weekend,
        }
    }

    #[test]
    fn circular_midpoint_diff_wraps_across_midnight() {
        let left = 23.0 * 60.0 + 55.0;
        let right = 5.0;
        assert_eq!(circular_minutes_diff(left, right), 10.0);
        assert!(within_clock_minute_band(left, right, 15.0));
        assert!(!within_clock_minute_band(left, right, 5.0));
    }

    #[test]
    fn midpoint_stability_treats_midnight_neighbors_as_stable() {
        let prior_from = NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date");
        let prior_to = NaiveDate::from_ymd_opt(2025, 1, 28).expect("valid date");
        let current_from = NaiveDate::from_ymd_opt(2025, 1, 29).expect("valid date");
        let current_to = NaiveDate::from_ymd_opt(2025, 2, 25).expect("valid date");

        let mut prior_samples = Vec::new();
        let mut current_samples = Vec::new();

        for i in 0..8 {
            let d = prior_from + ChronoDuration::days(i);
            prior_samples.push(make_day_sample(d, false, 23.0 * 60.0 + 55.0, Some(4.0)));
        }
        for i in 8..12 {
            let d = prior_from + ChronoDuration::days(i);
            prior_samples.push(make_day_sample(d, true, 23.0 * 60.0 + 55.0, Some(4.0)));
        }

        for i in 0..8 {
            let d = current_from + ChronoDuration::days(i);
            current_samples.push(make_day_sample(d, false, 5.0, Some(4.0)));
        }
        for i in 8..12 {
            let d = current_from + ChronoDuration::days(i);
            current_samples.push(make_day_sample(d, true, 5.0, Some(4.0)));
        }

        let calc = build_personalization_calc(
            &current_samples,
            &prior_samples,
            current_from,
            current_to,
            prior_from,
            prior_to,
            28,
        );

        assert!(
            calc.metrics
                .day_type_timing_baseline
                .midpoint_stable_across_windows
        );
        assert!(calc.metrics.day_type_timing_baseline.eligible);
    }

    #[test]
    fn social_jetlag_delta_wraps_across_midnight() {
        let base = NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date");

        let mut samples = Vec::new();
        for i in 0..5 {
            let d = base + ChronoDuration::days(i);
            samples.push(make_day_sample(d, false, 23.0 * 60.0 + 55.0, Some(4.0)));
        }
        for i in 5..9 {
            let d = base + ChronoDuration::days(i);
            samples.push(make_day_sample(d, true, 5.0, Some(4.0)));
        }

        let (delta, weekend_n) = social_jetlag_delta(&samples);
        assert_eq!(weekend_n, 4);
        assert_eq!(delta, Some(10.0));
    }
}

use chrono_tz::Tz;
use std::str::FromStr;

/// Returns the application timezone from APP_TZ or defaults to Asia/Tokyo.
/// Falls back to Asia/Tokyo if an unknown TZ name is provided.
pub fn app_tz() -> Tz {
    let name = std::env::var("APP_TZ").unwrap_or_else(|_| "Asia/Tokyo".to_string());
    Tz::from_str(&name).unwrap_or(chrono_tz::Asia::Tokyo)
}

#![doc = r#"Configuration utilities

Provides application configuration helpers such as the default timezone used by
time computations. See also: [`time::compute_duration_min`].

[`time::compute_duration_min`]: crate::time::compute_duration_min
"#]

use chrono_tz::Tz;
use std::str::FromStr;

#[doc = r#"Return the application timezone derived from the `APP_TZ` environment variable.

If `APP_TZ` is not set or contains an unknown zone name, the function falls back to `Asia/Tokyo`.

This timezone is used by functions like [`time::compute_duration_min`] to interpret local
bed/wake times in a consistent, DST-aware manner.

# Example

```rust,no_run
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {
# unsafe {
std::env::set_var("APP_TZ", "Asia/Tokyo");
let tz = sleep_api::config::app_tz();
assert_eq!(tz, chrono_tz::Asia::Tokyo);

// Unknown values also fall back to Asia/Tokyo.
std::env::set_var("APP_TZ", "Not/AZone");
let tz2 = sleep_api::config::app_tz();
assert_eq!(tz2, chrono_tz::Asia::Tokyo);
# }
# Ok(()) }
```

[`time::compute_duration_min`]: crate::time::compute_duration_min
"#]
pub fn app_tz() -> Tz {
    let name = std::env::var("APP_TZ").unwrap_or_else(|_| "Asia/Tokyo".to_string());
    Tz::from_str(&name).unwrap_or(chrono_tz::Asia::Tokyo)
}

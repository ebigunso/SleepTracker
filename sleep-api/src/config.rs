use base64::engine::general_purpose::STANDARD as BASE64_STD;
use base64::Engine;
use cookie::Key;
use chrono_tz::Tz;
use std::str::FromStr;

/// Returns the application timezone from APP_TZ or defaults to Asia/Tokyo.
/// Falls back to Asia/Tokyo if an unknown TZ name is provided.
pub fn app_tz() -> Tz {
    let name = std::env::var("APP_TZ").unwrap_or_else(|_| "Asia/Tokyo".to_string());
    Tz::from_str(&name).unwrap_or(chrono_tz::Asia::Tokyo)
}

/// Builds a cookie::Key from APP_SECRET_B64 (base64-encoded 64 random bytes).
/// Returns an error if the env var is missing, not valid base64, or not 64 bytes.
///
/// Generate securely, e.g.:
///   openssl rand -base64 64
pub fn cookie_key_from_b64() -> Result<Key, String> {
    let b64 = std::env::var("APP_SECRET_B64")
        .map_err(|_| "APP_SECRET_B64 not set".to_string())?;
    let bytes = BASE64_STD
        .decode(b64)
        .map_err(|_| "APP_SECRET_B64 is not valid base64".to_string())?;
    if bytes.len() != 64 {
        return Err(format!(
            "APP_SECRET_B64 must decode to 64 bytes, got {}",
            bytes.len()
        ));
    }
    Ok(Key::from(bytes.as_slice()))
}

use thiserror::Error;

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum DomainError {
    #[error("invalid intensity: {0}")]
    InvalidIntensity(String),
    #[error("quality must be between 1 and 5")]
    InvalidQuality,
    #[error("wake_time must be after bed_time")]
    InvalidSleepTimes,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

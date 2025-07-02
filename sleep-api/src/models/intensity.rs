use serde::{Deserialize, Serialize};
use crate::domain::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Intensity {
    None,
    Light,
    Hard,
}

impl std::fmt::Display for Intensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Intensity::None => "none",
            Intensity::Light => "light",
            Intensity::Hard => "hard",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for Intensity {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Intensity::None),
            "light" => Ok(Intensity::Light),
            "hard" => Ok(Intensity::Hard),
            other => Err(DomainError::InvalidIntensity(other.to_string())),
        }
    }
}

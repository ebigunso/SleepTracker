use crate::domain::DomainError;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Quality(pub u8);

impl<'de> Deserialize<'de> for Quality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u8::deserialize(deserializer)?;
        if (1..=5).contains(&v) {
            Ok(Quality(v))
        } else {
            Err(serde::de::Error::custom("quality must be between 1 and 5"))
        }
    }
}

impl Quality {
    pub fn value(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Quality {
    type Error = DomainError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        if (1..=5).contains(&v) {
            Ok(Quality(v))
        } else {
            Err(DomainError::InvalidQuality)
        }
    }
}

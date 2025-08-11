use crate::domain::DomainError;
use serde::{Deserialize, Deserializer, Serialize};

#[doc = r#"Sleep quality score (1..=5).

Semantic wrapper around `u8` validated during deserialization and conversion.
Higher values indicate better perceived sleep quality.

# Example

```rust
# use sleep_api::domain::DomainError;
use sleep_api::models::Quality;

// Construct directly
let q = Quality(4);
assert_eq!(q.value(), 4);

// Fallible construction from raw value
let q2 = Quality::try_from(5u8)?; // 1..=5 ok
assert_eq!(q2.value(), 5);
# Ok::<(), DomainError>(())
```
"#]
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
    #[doc = r#"Return the underlying 1..=5 score."#]
    pub fn value(self) -> u8 {
        self.0
    }
}

#[doc = r#"Attempt to convert a raw `u8` into a [`Quality`].

# Errors

Returns [`DomainError::InvalidQuality`] if the value is not in 1..=5.

[`DomainError::InvalidQuality`]: crate::domain::DomainError::InvalidQuality
"#]
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

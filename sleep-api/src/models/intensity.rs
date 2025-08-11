#![doc = r#"Exercise intensity levels

Represents qualitative intensity used by the exercise model. Values serialize as lowercase
strings and implement both `Display` and `FromStr` for ergonomic use.

- Serde representation: `"none" | "light" | "hard"`.
- `Display`: prints the lowercase string.
- `FromStr`: parses the lowercase string and returns a [`DomainError::InvalidIntensity`] on failure.

[`DomainError::InvalidIntensity`]: crate::domain::DomainError::InvalidIntensity
"#]

use crate::domain::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[doc = r#"Exercise intensity level.

# Example

```rust
# use sleep_api::domain::DomainError;
# fn main() -> Result<(), DomainError> {
use sleep_api::models::Intensity;

let level: Intensity = "light".parse()?;
assert_eq!(level.to_string(), "light");
# Ok(()) }
```

# Errors

Parsing with `FromStr` returns [`DomainError::InvalidIntensity`] when the input is not one of:
`"none"`, `"light"`, or `"hard"`.

[`DomainError::InvalidIntensity`]: crate::domain::DomainError::InvalidIntensity
"#]
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
        write!(f, "{s}")
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

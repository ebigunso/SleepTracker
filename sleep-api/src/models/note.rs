use crate::domain::DomainError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[doc = r#"User-provided note associated with a date.

Notes can be used to capture free-form observations that may help interpret sleep data.

- `date`: calendar date the note applies to.
- `body`: optional free text. Limited to 1000 characters.

# Example

```rust
# use sleep_api::domain::DomainError;
# use sleep_api::models::NoteInput;
# use chrono::NaiveDate;
# fn main() -> Result<(), DomainError> {
let note = NoteInput {
    date: NaiveDate::from_ymd_opt(2025, 6, 1).ok_or_else(|| DomainError::InvalidInput("invalid date".into()))?,
    body: Some("Felt refreshed".to_string()),
};
note.validate()?;
# Ok(()) }
```
"#]
#[derive(Serialize, Deserialize, Clone)]
pub struct NoteInput {
    pub date: NaiveDate,
    pub body: Option<String>,
}

impl NoteInput {
    #[doc = r#"Validate the note length (<= 1000 characters).

# Errors

Returns [`DomainError::InvalidInput`] if `body` is longer than 1000 characters.

[`DomainError::InvalidInput`]: crate::domain::DomainError::InvalidInput
"#]
    pub fn validate(&self) -> Result<(), DomainError> {
        if let Some(ref b) = self.body
            && (b.len() > 1000)
        {
            return Err(DomainError::InvalidInput("body too long".into()));
        }
        Ok(())
    }
}

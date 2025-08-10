use crate::domain::DomainError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NoteInput {
    pub date: NaiveDate,
    pub body: Option<String>,
}

impl NoteInput {
    pub fn validate(&self) -> Result<(), DomainError> {
        if let Some(ref b) = self.body && (b.len() > 1000) {
            return Err(DomainError::InvalidInput("body too long".into()));
        }
        Ok(())
    }
}

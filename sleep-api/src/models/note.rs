use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NoteInput {
    pub date: NaiveDate,
    pub body: Option<String>,
}

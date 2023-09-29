use chrono::{NaiveDate, NaiveTime, ParseError, ParseResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pass {
    pub name: String,
    pub description: String,
    pub date: String,
    pub time: String,
    pub location: String,
    pub code: String,
}

impl Pass {
    pub fn validate_date(&self) -> Result<(), ParseError> {
        NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")?;
        Ok(())
    }

    pub fn validate_time(&self) -> ParseResult<NaiveTime> {
        NaiveTime::parse_from_str(&self.time, "%H:%M")
    }
}

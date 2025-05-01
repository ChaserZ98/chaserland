use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdatedAt(DateTime<Utc>);

impl UpdatedAt {
    pub fn new(updated_at: DateTime<Utc>) -> Self {
        Self(updated_at)
    }
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
    pub fn update(&mut self) {
        self.0 = Utc::now();
    }
    pub fn set(&mut self, updated_at: DateTime<Utc>) {
        self.0 = updated_at;
    }
}

impl From<DateTime<Utc>> for UpdatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for UpdatedAt {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl TryFrom<&str> for UpdatedAt {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl std::fmt::Display for UpdatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

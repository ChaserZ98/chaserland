use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DeletedAt(DateTime<Utc>);

impl DeletedAt {
    pub fn new() -> Self {
        Self(Utc::now())
    }
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for DeletedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for DeletedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

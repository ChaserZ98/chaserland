use std::fmt::Display;

use serde::{Deserialize, Serialize};

/**
    ## ArticleVersion
    * A chrono::DateTime<chrono::Utc> that represents the version of the article
    * It is used for optimistic concurrency control
    * Any time the article aggregate is updated, the version should be updated with chrono::Utc::now()
*/
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Version(chrono::DateTime<chrono::Utc>);

impl Version {
    pub fn new(version: chrono::DateTime<chrono::Utc>) -> Self {
        Self(version)
    }
    pub fn value(&self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }
    pub fn bump(&mut self) {
        self.0 = chrono::Utc::now();
    }
}

impl Default for Version {
    fn default() -> Self {
        Self(chrono::Utc::now())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Version {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for Version {
    type Error = chrono::ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: chrono::DateTime<chrono::Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl TryFrom<&str> for Version {
    type Error = chrono::ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: chrono::DateTime<chrono::Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

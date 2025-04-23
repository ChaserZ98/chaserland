use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Content(String);

impl Content {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

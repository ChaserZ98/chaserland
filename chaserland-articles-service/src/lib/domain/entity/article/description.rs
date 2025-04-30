use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Description(String);

impl Description {
    pub fn new(description: impl Into<String>) -> Self {
        Self(description.into())
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for Description {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Description {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

use super::Slug;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Title(String);

impl Title {
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        Self::validate(&title).unwrap();
        Self(title)
    }
    pub fn as_slug(&self) -> Slug {
        self.clone().into()
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
    fn validate(title: &str) -> Result<(), String> {
        match title.trim().is_empty() {
            true => Err("title is empty".to_string()),
            false => Ok(()),
        }
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Title {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for Title {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

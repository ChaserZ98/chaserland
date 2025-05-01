use super::{Identifier, Title};
use serde::{Deserialize, Serialize};
use slugify::slugify;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Slug(String);

impl Slug {
    pub fn value(&self) -> String {
        self.0.clone()
    }
    pub fn as_identifier(&self) -> Identifier {
        self.clone().into()
    }
    fn validate(value: impl AsRef<str>) -> Result<(), String> {
        match value.as_ref().trim().is_empty() {
            true => Err("slug is empty".to_string()),
            false => Ok(()),
        }
    }
}

impl TryFrom<String> for Slug {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for Slug {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<Title> for Slug {
    fn from(value: Title) -> Self {
        Self(slugify!(&value.to_string(), separator = "-"))
    }
}

impl Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

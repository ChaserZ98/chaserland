use serde::{Deserialize, Serialize};
use slugify::slugify;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Id(i32);

impl Id {
    pub fn new(id: i32) -> Self {
        Self::validate(id).unwrap();
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    fn validate(id: i32) -> Result<(), String> {
        match id > 0 {
            true => Ok(()),
            false => Err("id must be greater than 0".to_string()),
        }
    }
}

impl TryFrom<i32> for Id {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Slug(String);

impl Slug {
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name(String);

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self::validate(&name).unwrap();
        Self(name)
    }
    pub fn value(&self) -> &str {
        &self.0
    }
    pub fn as_slug(&self) -> Slug {
        Slug(slugify!(&self.0, separator = "-"))
    }
    pub fn validate(name: impl Into<String>) -> Result<(), String> {
        let name = name.into();
        match name.len() > 0 {
            true => Ok(()),
            false => Err("name must not be empty".to_string()),
        }
    }
}

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub id: Id,
    pub slug: Slug,
    pub name: Name,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Identifier {
    Id(Id),
    Slug(Slug),
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Id(id) => write!(f, "id={}", id),
            Identifier::Slug(slug) => write!(f, "slug={}", slug),
        }
    }
}

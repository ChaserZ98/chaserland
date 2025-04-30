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

    pub fn as_identifier(&self) -> Identifier {
        self.clone().into()
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

impl TryFrom<&i32> for Id {
    type Error = String;

    fn try_from(value: &i32) -> Result<Self, Self::Error> {
        Self::try_from(*value)
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

    pub fn as_identifier(&self) -> Identifier {
        self.clone().into()
    }

    fn validate(slug: impl AsRef<str>) -> Result<(), String> {
        match slug.as_ref().trim().is_empty() {
            true => Err("slug must not be empty".to_string()),
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

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Name(String);

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();
        Self::validate(&name).unwrap();
        Self(name)
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
    pub fn as_slug(&self) -> Slug {
        Slug(slugify!(&self.0, separator = "-"))
    }
    pub fn validate(name: impl AsRef<str>) -> Result<(), String> {
        match name.as_ref().trim().is_empty() {
            true => Err("name must not be empty".to_string()),
            false => Ok(()),
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

impl TryFrom<&str> for Name {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Tag {
    pub id: Id,
    pub slug: Slug,
    pub name: Name,
}

impl Tag {
    pub fn new(id: Id, name: Name) -> Self {
        let slug = name.as_slug();
        Self { id, slug, name }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewTag {
    pub name: Name,
    slug: Slug,
}

impl NewTag {
    pub fn new(name: Name) -> Self {
        let slug = name.as_slug();
        Self { name, slug }
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Identifier {
    Id(Id),
    Slug(Slug),
}

impl Identifier {
    pub fn is_id(&self) -> bool {
        match self {
            Identifier::Id(_) => true,
            _ => false,
        }
    }
    pub fn is_slug(&self) -> bool {
        match self {
            Identifier::Slug(_) => true,
            _ => false,
        }
    }
}

impl From<Id> for Identifier {
    fn from(id: Id) -> Self {
        Self::Id(id)
    }
}

impl From<Slug> for Identifier {
    fn from(slug: Slug) -> Self {
        Self::Slug(slug)
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Id(id) => write!(f, "id={}", id),
            Identifier::Slug(slug) => write!(f, "slug={}", slug),
        }
    }
}

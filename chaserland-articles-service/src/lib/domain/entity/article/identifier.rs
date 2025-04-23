use super::{Id, Slug};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
    Id(Id),
    Slug(Slug),
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

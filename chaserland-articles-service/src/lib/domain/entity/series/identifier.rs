use super::{Id, Slug};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Identifier {
    Id(Id),
    Slug(Slug),
}

impl Identifier {
    pub fn is_id(&self) -> bool {
        matches!(self, Identifier::Id(_))
    }

    pub fn is_slug(&self) -> bool {
        matches!(self, Identifier::Slug(_))
    }

    pub fn as_id(&self) -> Option<&Id> {
        match self {
            Identifier::Id(id) => Some(id),
            Identifier::Slug(_) => None,
        }
    }

    pub fn as_slug(&self) -> Option<&Slug> {
        match self {
            Identifier::Id(_) => None,
            Identifier::Slug(slug) => Some(slug),
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

#[cfg(test)]
mod tests {
    use super::{Id, Identifier, Slug};

    #[test]
    fn identifier_case_id() {
        let id = Id::new(1);
        let identifier = Identifier::Id(id);

        assert!(identifier.is_id());

        let res = identifier.as_id();
        assert!(res.is_some());

        let res = res.unwrap();
        assert_eq!(res, &id);

        let slug = Slug::try_from("test-title").unwrap();
        let identifier = Identifier::Slug(slug);

        assert_eq!(identifier.is_id(), false);

        let res = identifier.as_id();
        assert!(res.is_none());
    }

    #[test]
    fn identifier_case_slug() {
        let slug = Slug::try_from("test-title").unwrap();
        let identifier = Identifier::Slug(slug.clone());

        assert!(identifier.is_slug());

        let res = identifier.as_slug();
        assert!(res.is_some());

        let res = res.unwrap();
        assert_eq!(res, &slug);

        let id = Id::new(1);
        let identifier = Identifier::Id(id);

        assert_eq!(identifier.is_slug(), false);

        let res = identifier.as_slug();
        assert!(res.is_none());
    }

    #[test]
    fn identifier_case_to_string() {
        let id = Id::new(1);
        let identifier = Identifier::Id(id);

        assert_eq!(identifier.to_string(), "id=1");

        let slug = Slug::try_from("test-title").unwrap();
        let identifier = Identifier::Slug(slug);

        assert_eq!(identifier.to_string(), "slug=test-title");
    }

    #[test]
    fn identifier_case_from_id() {
        let id = Id::new(1);
        let identifier = Identifier::from(id);

        assert_eq!(identifier, Identifier::Id(id));
    }

    #[test]
    fn identifier_case_from_slug() {
        let slug = Slug::try_from("test-title").unwrap();
        let identifier = Identifier::from(slug.clone());

        assert_eq!(identifier, Identifier::Slug(slug));
    }
}

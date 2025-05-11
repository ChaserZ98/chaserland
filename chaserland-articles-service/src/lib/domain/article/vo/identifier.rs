use super::{Id, Slug};

#[derive(Debug, Clone, PartialEq, Eq)]
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
            Self::Id(id) => Some(id),
            Self::Slug(_) => None,
        }
    }

    pub fn as_slug(&self) -> Option<&Slug> {
        match self {
            Self::Slug(slug) => Some(slug),
            Self::Id(_) => None,
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
    fn article_identifier_case_id() {
        let id = Id::new(1);
        let res = Identifier::Id(id);

        assert!(res.is_id());

        let res = res.as_id();

        assert!(res.is_some());

        let res = res.unwrap();

        assert_eq!(res, &id);

        let slug = Slug::try_from("test-title").unwrap();
        let res = Identifier::Slug(slug);

        assert_eq!(res.is_id(), false);

        let res = res.as_id();

        assert!(res.is_none());
    }

    #[test]
    fn article_identifier_case_slug() {
        let slug = Slug::try_from("test-title").unwrap();
        let res = Identifier::Slug(slug.clone());

        assert!(res.is_slug());

        let res = res.as_slug();

        assert!(res.is_some());

        let res = res.unwrap();

        assert_eq!(res, &slug);

        let id = Id::new(1);
        let res = Identifier::Id(id);

        assert_eq!(res.is_slug(), false);

        let res = res.as_slug();

        assert!(res.is_none());
    }

    #[test]
    fn article_identifier_case_to_string() {
        let id = Id::new(1);
        let res = Identifier::Id(id);

        assert_eq!(res.to_string(), "id=1");

        let slug = Slug::try_from("test-title").unwrap();
        let res = Identifier::Slug(slug);

        assert_eq!(res.to_string(), "slug=test-title");
    }

    #[test]
    fn article_identifier_case_from_id() {
        let id = Id::new(1);
        let res = Identifier::from(id);

        assert_eq!(res, Identifier::Id(id));
    }

    #[test]
    fn article_identifier_case_from_slug() {
        let slug = Slug::try_from("test-title").unwrap();
        let res = Identifier::from(slug.clone());

        assert_eq!(res, Identifier::Slug(slug));
    }
}

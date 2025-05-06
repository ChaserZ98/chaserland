use super::{Identifier, Name};
use serde::{Deserialize, Serialize};
use slugify::slugify;

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

impl From<Name> for Slug {
    fn from(name: Name) -> Self {
        Self(slugify!(name.value(), separator = "-"))
    }
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Identifier, Name, Slug};

    #[test]
    fn slug_case_from_name() {
        let slug = Slug::from(Name::new("name"));
        assert_eq!(slug.value(), "name");

        let slug = Slug::from(Name::new("default    name"));
        assert_eq!(slug.value(), "default-name");
    }

    #[test]
    fn slug_case_try_from_string() {
        let res = Slug::try_from(String::from("test-title"));

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), "test-title");

        let res = Slug::try_from(String::from("   "));

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert_eq!(res, "slug is empty");
    }

    #[test]
    fn slug_case_try_from_str_ref() {
        let res = Slug::try_from("test-title");

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), "test-title");

        let res = Slug::try_from("   ");

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert_eq!(res, "slug is empty");
    }

    #[test]
    fn slug_case_to_string() {
        let slug = Slug::try_from("test-title").unwrap();
        assert_eq!(slug.to_string(), "test-title");
    }

    #[test]
    fn slug_case_as_identifier() {
        let slug = Slug::try_from("test-title").unwrap();
        assert_eq!(slug.as_identifier(), Identifier::Slug(slug));
    }
}

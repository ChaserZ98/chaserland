use super::{Identifier, Name};
use serde::{Deserialize, Serialize};
use slugify::slugify;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Slug(String);

impl Slug {
    pub fn value(&self) -> &String {
        &self.0
    }

    pub fn as_identifier(&self) -> Identifier {
        self.clone().into()
    }

    fn validate<T>(slug: T) -> Result<(), String>
    where
        T: AsRef<str>,
    {
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

impl From<Name> for Slug {
    fn from(name: Name) -> Self {
        Self(slugify!(&name.to_string(), separator = "-"))
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
    }

    #[test]
    fn slug_case_try_from_string() {
        let res = Slug::try_from(String::from("test-title"));

        assert!(res.is_ok());

        let slug = res.unwrap();
        assert_eq!(slug.value(), "test-title");

        let res = Slug::try_from(String::from("   "));

        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, "slug must not be empty");
    }

    #[test]
    fn slug_case_try_from_str_ref() {
        let res = Slug::try_from("test-title");

        assert!(res.is_ok());

        let slug = res.unwrap();
        assert_eq!(slug.value(), "test-title");

        let res = Slug::try_from("   ");

        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, "slug must not be empty");
    }

    #[test]
    fn slug_case_as_identifier() {
        let slug = Slug::try_from("test-title").unwrap();
        assert_eq!(slug.as_identifier(), Identifier::Slug(slug));
    }
}

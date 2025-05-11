use super::{Identifier, Title};
use serde::{Deserialize, Serialize};
use slugify::slugify;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Slug(String);

impl Slug {
    pub fn value(&self) -> &String {
        &self.0
    }
    pub fn as_identifier(&self) -> Identifier {
        self.clone().into()
    }
    fn validate<T>(value: T) -> Result<(), String>
    where
        T: AsRef<str>,
    {
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

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Identifier, Slug, Title};

    #[test]
    fn article_slug_case_from_title() {
        let title = Title::new("test title");
        let slug = Slug::from(title);

        let target = Slug("test-title".to_string());

        assert_eq!(slug, target);

        let title = Title::new("test   title");
        let slug = Slug::from(title);

        let target = Slug("test-title".to_string());

        assert_eq!(slug, target);

        let target = &String::from("test-title");

        assert_eq!(slug.value(), target);
    }

    #[test]
    fn article_slug_case_to_string() {
        let slug = Slug("test-title".to_string());

        let target = String::from("test-title");

        assert_eq!(slug.to_string(), target);
    }

    #[test]
    fn article_slug_case_as_identifier() {
        let slug = Slug("test-title".to_string());
        let identifier = slug.as_identifier();

        let target = Identifier::Slug(slug.clone());

        assert_eq!(identifier, target);
    }

    #[test]
    fn article_slug_case_try_from_string() {
        let slug = Slug::try_from(String::from("test-title"));

        assert!(slug.is_ok());

        let slug = slug.unwrap();

        let target = Slug("test-title".to_string());

        assert_eq!(slug, target);

        let slug = Slug::try_from(String::from(""));

        assert!(slug.is_err());

        let slug = slug.unwrap_err();

        assert_eq!(slug, "slug is empty");
    }

    #[test]
    fn article_slug_case_try_from_str_ref() {
        let slug = Slug::try_from("test-title");

        assert!(slug.is_ok());

        let slug = slug.unwrap();

        let target = Slug("test-title".to_string());

        assert_eq!(slug, target);

        let slug = Slug::try_from("");

        assert!(slug.is_err());

        let slug = slug.unwrap_err();

        assert_eq!(slug, "slug is empty");
    }
}

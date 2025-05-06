use super::Slug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Title(String);

impl Title {
    pub fn new<T>(title: T) -> Self
    where
        T: AsRef<str>,
    {
        let title = title.as_ref();
        Self::validate(&title).unwrap();
        Self(title.into())
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

impl std::fmt::Display for Title {
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

#[cfg(test)]
mod tests {
    use super::Title;

    #[test]
    fn article_title_case_new() {
        let title = Title::new("title");
        assert_eq!(title.value(), "title");
        assert_eq!(title.as_slug().value(), "title");
    }

    #[test]
    #[should_panic(expected = "title is empty")]
    fn article_title_case_new_panic() {
        let _ = Title::new(String::from("  "));
    }

    #[test]
    fn article_title_case_to_string() {
        let title = Title::new("title");
        assert_eq!(title.to_string(), "title");
    }

    #[test]
    fn article_title_case_try_from_string() {
        let title = Title::try_from(String::from("title"));
        assert_eq!(title.is_ok(), true);

        let title = title.unwrap();

        let target = Title::new("title");

        assert_eq!(title, target);

        let title = Title::try_from(String::new());

        assert!(title.is_err());

        assert_eq!(title.unwrap_err(), "title is empty");
    }

    #[test]
    fn article_title_case_try_from_str_ref() {
        let title = Title::try_from("title");

        assert_eq!(title.is_ok(), true);

        let title = title.unwrap();

        let target = Title::new("title");

        assert_eq!(title, target);

        let title = Title::try_from("");

        assert!(title.is_err());

        assert_eq!(title.unwrap_err(), "title is empty");
    }
}

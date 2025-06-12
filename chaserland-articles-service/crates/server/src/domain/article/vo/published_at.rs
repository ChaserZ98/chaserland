use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct PublishedAt(DateTime<Utc>);

impl PublishedAt {
    pub fn new(published_at: DateTime<Utc>) -> Self {
        Self(published_at)
    }
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for PublishedAt {
    fn default() -> Self {
        Self::new(chrono::Utc::now())
    }
}

impl From<DateTime<Utc>> for PublishedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for PublishedAt {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl TryFrom<&str> for PublishedAt {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl std::fmt::Display for PublishedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::PublishedAt;

    #[test]
    fn article_published_at_case_new() {
        let time = chrono::Utc::now();
        let published_at = PublishedAt::new(time);

        assert_eq!(published_at.value(), time);
    }

    #[test]
    fn article_published_at_case_default() {
        let time = chrono::Utc::now();
        let published_at = PublishedAt::default();

        assert!(published_at.value() - time < chrono::Duration::seconds(1));
    }

    #[test]
    fn article_published_at_case_to_string() {
        let time = chrono::Utc::now();
        let published_at = PublishedAt::new(time);

        assert_eq!(published_at.to_string(), time.to_string());
    }

    #[test]
    fn article_published_at_case_from_chrono() {
        let time = chrono::Utc::now();
        let published_at = PublishedAt::from(time);

        assert_eq!(published_at.value(), time);
    }

    #[test]
    fn article_published_at_case_try_from_string() {
        let time = chrono::Utc::now();
        let res = PublishedAt::try_from(time.to_string());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), time);

        let res = PublishedAt::try_from(String::from("invalid string"));

        assert!(res.is_err());
    }

    #[test]
    fn article_published_at_case_try_from_str_ref() {
        let time = chrono::Utc::now();
        let res = PublishedAt::try_from(time.to_string().as_str());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), time);

        let res = PublishedAt::try_from("invalid string");

        assert!(res.is_err());
    }
}

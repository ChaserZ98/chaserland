use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct CreatedAt(DateTime<Utc>);

impl CreatedAt {
    pub fn new(created_at: DateTime<Utc>) -> Self {
        Self(created_at)
    }
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for CreatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for CreatedAt {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl TryFrom<&str> for CreatedAt {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl std::fmt::Display for CreatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::CreatedAt;

    #[test]
    fn article_created_at_case_new() {
        let time = chrono::Utc::now();
        let created_at = CreatedAt::new(time);

        let target = CreatedAt(time);

        assert_eq!(created_at, target);
    }

    #[test]
    fn article_created_at_case_to_string() {
        let time = chrono::Utc::now();
        let created_at = CreatedAt::new(time);

        assert_eq!(created_at.to_string(), time.to_string());
    }

    #[test]
    fn article_created_at_case_from_chrono() {
        let time = chrono::Utc::now();
        let created_at = CreatedAt::from(time);

        assert_eq!(created_at.value(), time);
    }

    #[test]
    fn article_created_at_case_try_from_string() {
        let time = chrono::Utc::now();
        let res = CreatedAt::try_from(time.to_string());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), time);

        let res = CreatedAt::try_from(String::from("invalid string"));

        assert!(res.is_err());
    }

    #[test]
    fn article_created_at_case_try_from_str_ref() {
        let time = chrono::Utc::now();
        let res = CreatedAt::try_from(time.to_string().as_str());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), time);

        let res = CreatedAt::try_from("invalid string");

        assert!(res.is_err());
    }
}

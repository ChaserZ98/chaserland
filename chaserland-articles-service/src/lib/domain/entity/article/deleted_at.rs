use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct DeletedAt(DateTime<Utc>);

impl DeletedAt {
    pub fn new() -> Self {
        Self(Utc::now())
    }
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for DeletedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for DeletedAt {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl TryFrom<&str> for DeletedAt {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl std::fmt::Display for DeletedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::DeletedAt;

    #[test]
    fn article_deleted_at_case_new() {
        let deleted_at = DeletedAt::new();
        let target = chrono::Utc::now();

        assert!(deleted_at.value() - target <= chrono::Duration::seconds(1));
    }

    #[test]
    fn article_deleted_at_case_to_string() {
        let deleted_at = DeletedAt::new();
        let target: Result<chrono::DateTime<chrono::Utc>, _> = deleted_at.to_string().parse();

        assert!(target.is_ok());

        let target = target.unwrap();

        assert_eq!(deleted_at.to_string(), target.to_string());
    }

    #[test]
    fn article_deleted_at_case_from_chrono() {
        let chrono = chrono::Utc::now();
        let deleted_at = DeletedAt::from(chrono);

        assert_eq!(deleted_at.value(), chrono);
    }

    #[test]
    fn article_deleted_at_case_try_from_string() {
        let chrono = chrono::Utc::now();
        let res = DeletedAt::try_from(chrono.to_string());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), chrono);

        let res = DeletedAt::try_from("invalid string");

        assert!(res.is_err());
    }

    #[test]
    fn article_deleted_at_case_try_from_str_ref() {
        let chrono = chrono::Utc::now();
        let res = DeletedAt::try_from(chrono.to_string().as_str());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), chrono);

        let res = DeletedAt::try_from("invalid string");

        assert!(res.is_err());
    }
}

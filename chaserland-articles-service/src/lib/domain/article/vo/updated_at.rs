use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct UpdatedAt(DateTime<Utc>);

impl UpdatedAt {
    pub fn new(updated_at: DateTime<Utc>) -> Self {
        Self(updated_at)
    }
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
    pub fn update(&mut self) {
        self.0 = Utc::now();
    }
    pub fn set(&mut self, updated_at: DateTime<Utc>) {
        self.0 = updated_at;
    }
}

impl From<DateTime<Utc>> for UpdatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for UpdatedAt {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl TryFrom<&str> for UpdatedAt {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl std::fmt::Display for UpdatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::UpdatedAt;

    #[test]
    fn article_updated_at_case_new() {
        let time = chrono::Utc::now();
        let updated_at = UpdatedAt::new(time);

        assert_eq!(updated_at.value(), time);
    }

    #[test]
    fn article_updated_at_case_update() {
        let time = "2022-01-01 00:00:00 UTC".parse().unwrap();
        let mut updated_at = UpdatedAt::new(time);

        assert_eq!(updated_at.value(), time);

        updated_at.update();
        let target = chrono::Utc::now();

        assert!(target - updated_at.value() < chrono::Duration::seconds(1));
    }

    #[test]
    fn article_updated_at_case_set() {
        let time = chrono::Utc::now();
        let mut updated_at = UpdatedAt::new(time);

        assert_eq!(updated_at.value(), time);

        let new_time = chrono::Utc::now();
        updated_at.set(new_time);

        assert_eq!(updated_at.value(), new_time);
    }

    #[test]
    fn article_updated_at_case_to_string() {
        let time = chrono::Utc::now();
        let updated_at = UpdatedAt::new(time);

        assert_eq!(updated_at.to_string(), time.to_string());
    }

    #[test]
    fn article_updated_at_case_from_chrono() {
        let time = chrono::Utc::now();
        let updated_at = UpdatedAt::from(time);

        assert_eq!(updated_at.value(), time);
    }

    #[test]
    fn article_updated_at_case_try_from_string() {
        let time = chrono::Utc::now();
        let res = UpdatedAt::try_from(time.to_string());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), time);

        let res = UpdatedAt::try_from(String::from("invalid string"));

        assert!(res.is_err());
    }

    #[test]
    fn article_updated_at_case_try_from_str_ref() {
        let time = chrono::Utc::now();
        let res = UpdatedAt::try_from(time.to_string().as_str());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), time);

        let res = UpdatedAt::try_from("invalid string");

        assert!(res.is_err());
    }
}

use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};

/**
    ## ArticleVersion
    * A chrono::DateTime<chrono::Utc> that represents the version of the article
    * It is used for optimistic concurrency control
    * Any time the article aggregate is updated, the version should be updated with chrono::Utc::now()
*/
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Version(DateTime<Utc>);

impl Version {
    pub fn new(version: DateTime<Utc>) -> Self {
        Self(version)
    }
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
    /**
        Set the version to chrono::Utc::now()
    */
    pub fn bump(&mut self) {
        self.0 = Utc::now();
    }
}

impl Default for Version {
    fn default() -> Self {
        Self(Utc::now())
    }
}

impl From<DateTime<Utc>> for Version {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for Version {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl TryFrom<&str> for Version {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: DateTime<Utc> = value.parse()?;
        Ok(value.into())
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn article_version_case_new() {
        let time = chrono::Utc::now();
        let version = Version::new(time);

        assert_eq!(version.value(), time);
    }

    #[test]
    fn article_version_case_default() {
        let time = chrono::Utc::now();
        let version = Version::default();

        assert!(version.value() - time < chrono::Duration::seconds(1));
    }

    #[test]
    fn article_version_case_to_string() {
        let time = chrono::Utc::now();
        let version = Version::new(time);

        assert_eq!(version.to_string(), time.to_string());
    }

    #[test]
    fn article_version_case_bump() {
        let time = "2022-01-01 00:00:00 UTC".parse().unwrap();
        let mut version = Version::new(time);

        assert_eq!(version.value(), time);

        version.bump();
        let target = chrono::Utc::now();
        assert!(target - version.value() < chrono::Duration::seconds(1));
    }

    #[test]
    fn article_version_case_from_chrono() {
        let time = chrono::Utc::now();
        let version = Version::from(time);

        assert_eq!(version.value(), time);
    }

    #[test]
    fn article_version_case_try_from_string() {
        let time = chrono::Utc::now();
        let res = Version::try_from(time.to_string());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), time);

        let res = Version::try_from(String::from("invalid string"));

        assert!(res.is_err());
    }

    #[test]
    fn article_version_case_try_from_str_ref() {
        let time = chrono::Utc::now();
        let res = Version::try_from(time.to_string().as_str());

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.value(), time);

        let res = Version::try_from("invalid string");

        assert!(res.is_err());
    }
}

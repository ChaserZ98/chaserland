use super::Slug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        let name = name.into();
        Self::validate(&name).unwrap();
        Self(name)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn as_slug(&self) -> Slug {
        self.clone().into()
    }

    fn validate<T>(name: T) -> Result<(), String>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        match name.trim().is_empty() {
            false => Ok(()),
            true => Err("name must not be empty".to_string()),
        }
    }
}

impl TryFrom<&str> for Name {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    fn name_case_new() {
        let name = Name::new("name");
        assert_eq!(name.value(), "name");
    }

    #[test]
    #[should_panic(expected = "name must not be empty")]
    fn name_case_new_panic() {
        let _ = Name::new("");
    }

    #[test]
    fn name_case_to_string() {
        let name = Name::new("name");
        assert_eq!(name.to_string(), "name");
    }

    #[test]
    fn name_case_as_slug() {
        let name = Name::new("default name");
        assert_eq!(name.as_slug().value(), "default-name");
    }

    #[test]
    fn name_case_try_from_string() {
        let res = Name::try_from(String::from("name"));

        assert!(res.is_ok());

        let name = res.unwrap();
        assert_eq!(name.value(), "name");

        let res = Name::try_from(String::from(""));

        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, "name must not be empty");
    }

    #[test]
    fn name_case_try_from_str_ref() {
        let res = Name::try_from("name");

        assert!(res.is_ok());

        let name = res.unwrap();
        assert_eq!(name.value(), "name");

        let res = Name::try_from("");

        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, "name must not be empty");
    }
}

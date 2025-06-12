use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Description(String);

impl Description {
    pub fn new<T>(description: T) -> Self
    where
        T: Into<String>,
    {
        Self(description.into())
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

impl From<String> for Description {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Description {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Description;

    #[test]
    fn article_description_case_new() {
        let description = Description::new("description");
        assert_eq!(description.value(), "description");
    }

    #[test]
    fn article_description_case_default() {
        let description = Description::default();
        assert_eq!(description.value(), "");
    }

    #[test]
    fn article_description_case_to_string() {
        let description = Description::new("description");
        assert_eq!(description.to_string(), "description");
    }

    #[test]
    fn article_description_case_from_string() {
        let description = Description::from(String::from("description"));

        assert_eq!(description.value(), "description");
    }

    #[test]
    fn article_description_case_from_str_ref() {
        let description = Description::from("description");
        assert_eq!(description.value(), "description");
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Content(String);

impl Content {
    pub fn new<T>(content: T) -> Self
    where
        T: Into<String>,
    {
        Self(content.into())
    }
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Content;

    #[test]
    fn article_content_case_new() {
        let content = Content::new("content");
        assert_eq!(content.value(), "content");
    }

    #[test]
    fn article_content_case_default() {
        let content = Content::default();
        assert_eq!(content.value(), "");
    }

    #[test]
    fn article_content_case_to_string() {
        let content = Content::new("content");
        assert_eq!(content.to_string(), "content");
    }

    #[test]
    fn article_content_from_string() {
        let content = Content::from(String::from("content"));
        assert_eq!(content.value(), "content");
    }

    #[test]
    fn article_content_from_str_ref() {
        let content = Content::from("content");
        assert_eq!(content.value(), "content");
    }
}

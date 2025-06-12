use super::{Content, Description, Title, Version};
use crate::domain::category::vo as category;
use crate::domain::series::vo as series;
use crate::domain::tag::vo as tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NewArticle {
    pub title: Title,
    pub description: Description,
    pub content: Option<Content>,
    pub series_id: Option<series::Id>,
    pub category_ids: Vec<category::Id>,
    pub tag_ids: Vec<tag::Id>,
    version: Version,
}

impl Default for NewArticle {
    fn default() -> Self {
        let title = Title::new("Default Title");
        let description = Description::default();
        let content = None;
        let series_id = None;
        let category_ids = vec![];
        let tag_ids = vec![];
        let version = Version::default();
        Self {
            title,
            description,
            content,
            series_id,
            category_ids,
            tag_ids,
            version,
        }
    }
}

impl NewArticle {
    pub fn new(
        title: Title,
        description: Description,
        content: Option<Content>,
        series_id: Option<series::Id>,
        category_ids: Vec<category::Id>,
        tag_ids: Vec<tag::Id>,
    ) -> Self {
        let version = Version::default();
        Self {
            title,
            description,
            content,
            series_id,
            category_ids,
            tag_ids,
            version,
        }
    }
    pub fn version(&self) -> Version {
        self.version
    }
}

#[cfg(test)]
mod tests {
    use super::{NewArticle, Version};

    #[test]
    fn new_article_case_new() {
        let article = NewArticle::new(
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            None,
            vec![1.try_into().unwrap(), 2.try_into().unwrap()],
            vec![3.try_into().unwrap(), 4.try_into().unwrap()],
        );

        let target = NewArticle {
            title: "title".try_into().unwrap(),
            description: "description".into(),
            content: Some("content".into()),
            series_id: None,
            category_ids: vec![1.try_into().unwrap(), 2.try_into().unwrap()],
            tag_ids: vec![3.try_into().unwrap(), 4.try_into().unwrap()],
            version: Version::default(),
        };

        assert_eq!(article.title, target.title);
        assert_eq!(article.description, target.description);
        assert_eq!(article.content, target.content);
        assert_eq!(article.series_id, target.series_id);
        assert_eq!(article.category_ids, target.category_ids);
        assert_eq!(article.tag_ids, target.tag_ids);
        assert!(target.version.value() - article.version.value() < chrono::Duration::seconds(1));
    }

    #[test]
    fn new_article_case_default() {
        let article = NewArticle::default();
        let target = NewArticle {
            title: "Default Title".try_into().unwrap(),
            description: "".into(),
            content: None,
            series_id: None,
            category_ids: vec![],
            tag_ids: vec![],
            version: Version::default(),
        };

        assert_eq!(article.title, target.title);
        assert_eq!(article.description, target.description);
        assert_eq!(article.content, target.content);
        assert_eq!(article.series_id, target.series_id);
        assert_eq!(article.category_ids, target.category_ids);
        assert_eq!(article.tag_ids, target.tag_ids);
        assert!(target.version.value() - article.version.value() < chrono::Duration::seconds(1));
    }
}

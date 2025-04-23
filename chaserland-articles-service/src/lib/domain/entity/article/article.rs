use super::{
    Content, CreatedAt, DeletedAt, Description, Id, PublishedAt, Slug, Title, UpdatedAt, Version,
};
use crate::domain::entity::{category, series, tag};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Article {
    pub id: Id,
    pub title: Title,
    slug: Slug,
    pub description: Description,
    pub content: Option<Content>,
    pub created_at: CreatedAt,
    pub published_at: Option<PublishedAt>,
    pub updated_at: UpdatedAt,
    pub deleted_at: Option<DeletedAt>,
    pub series_id: Option<series::SeriesId>,
    pub category_ids: Vec<category::CategoryId>,
    pub tag_ids: Vec<tag::TagId>,
    pub version: Version,
}

impl Article {
    pub fn new(
        id: Id,
        title: Title,
        description: Description,
        content: Option<Content>,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
        series_id: Option<series::SeriesId>,
        category_ids: Vec<category::CategoryId>,
        tag_ids: Vec<tag::TagId>,
        version: Version,
    ) -> Self {
        let slug = title.as_slug();
        Self {
            id,
            title,
            description,
            content,
            created_at,
            updated_at,
            deleted_at: None,
            published_at: None,
            slug,
            series_id,
            category_ids,
            tag_ids,
            version,
        }
    }
    pub fn set_title(&mut self, title: Title) {
        self.slug = title.as_slug();
        self.title = title;
    }
    pub fn set_description(&mut self, description: Description) {
        self.description = description;
    }
    pub fn set_content(&mut self, content: Option<Content>) {
        self.content = content;
    }
    pub fn slug(&self) -> &Slug {
        &self.slug
    }
    pub fn is_published(&self) -> bool {
        self.published_at.is_some()
    }
    pub fn publish(&mut self) -> Result<(), DomainError> {
        match self.published_at {
            Some(_) => Err(DomainError::AlreadyPublished(self.id)),
            None => {
                let utc_now = chrono::Utc::now();
                self.published_at = Some(PublishedAt::new(utc_now));
                Ok(())
            }
        }
    }
    pub fn unpublish(&mut self) -> Result<(), DomainError> {
        match self.published_at {
            Some(_) => {
                self.published_at = None;
                Ok(())
            }
            None => Err(DomainError::NotPublished(self.id)),
        }
    }
    pub fn soft_delete(&mut self) -> Result<(), DomainError> {
        match self.deleted_at {
            Some(_) => Err(DomainError::AlreadySoftDeleted(self.id)),
            None => {
                self.deleted_at = Some(DeletedAt::new());
                Ok(())
            }
        }
    }
    pub fn revoke_soft_delete(&mut self) -> Result<(), DomainError> {
        match self.deleted_at {
            Some(_) => {
                self.deleted_at = None;
                Ok(())
            }
            None => Err(DomainError::NotSoftDeleted(self.id)),
        }
    }
    pub fn bump_updated_at(&mut self) {
        self.updated_at.update();
    }
    pub fn bump_version(&mut self) {
        self.version.bump();
    }
}

impl Default for Article {
    fn default() -> Self {
        let id = Id::new(1);
        let title = Title::new("Default Title");
        let description = Description::new("");
        let content = None;
        let created_at = CreatedAt::new(chrono::Utc::now());
        let updated_at = UpdatedAt::new(chrono::Utc::now());
        let series_id = None;
        let category_ids = vec![];
        let tag_ids = vec![];
        let version = Version::new(chrono::Utc::now());
        Self::new(
            id,
            title,
            description,
            content,
            created_at,
            updated_at,
            series_id,
            category_ids,
            tag_ids,
            version,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleCreate {
    pub title: Title,
    pub description: Description,
    pub content: Option<Content>,
    pub series_id: Option<series::SeriesId>,
    pub category_ids: Vec<category::CategoryId>,
    pub tag_ids: Vec<tag::TagId>,
    pub version: Version,
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Article with id {0} is already published")]
    AlreadyPublished(Id),
    #[error("Article with id {0} has not been published")]
    NotPublished(Id),
    #[error("Article with id {0} is already soft deleted")]
    AlreadySoftDeleted(Id),
    #[error("Article with id {0} has not been soft deleted")]
    NotSoftDeleted(Id),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::Title;

    #[test]
    fn test_article_title() {
        let title = Title::new(String::from("title"));
        assert_eq!(title.value(), "title");
    }

    #[test]
    #[should_panic(expected = "title is empty")]
    fn test_article_title_panic() {
        let _ = Title::new(String::from("  "));
    }

    #[test]
    fn test_article_title_convert() {
        let title = TryInto::<Title>::try_into(String::from("title"));
        assert_eq!(title.is_ok(), true);
        assert_eq!(title.unwrap().value(), "title");

        let title: Result<Title, _> = String::from("  ").try_into();
        assert_eq!(title.is_err(), true);
        assert_eq!(title.unwrap_err(), "title is empty");
    }
}

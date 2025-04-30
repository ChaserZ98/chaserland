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
    pub series_id: Option<series::Id>,
    pub category_ids: Vec<category::Id>,
    pub tag_ids: Vec<tag::Id>,
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
        series_id: Option<series::Id>,
        category_ids: Vec<category::Id>,
        tag_ids: Vec<tag::Id>,
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
    pub fn set_content(&mut self, content: Content) {
        self.content = Some(content);
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
    pub fn set_series_id(&mut self, series_id: series::Id) {
        self.series_id = Some(series_id);
    }
    pub fn remove_series_id(&mut self) {
        self.series_id = None;
    }
    pub fn add_category_id(&mut self, category_id: category::Id) -> Result<(), DomainError> {
        if self.category_ids.contains(&category_id) {
            return Err(DomainError::CategoryAlreadyAttached(self.id, category_id));
        }
        self.category_ids.push(category_id);
        Ok(())
    }
    pub fn remove_category_id(&mut self, category_id: category::Id) -> Result<(), DomainError> {
        if !self.category_ids.contains(&category_id) {
            return Err(DomainError::CategoryNotFound(self.id, category_id));
        }
        self.category_ids.retain(|id| *id != category_id);
        Ok(())
    }
    pub fn add_tag_id(&mut self, tag_id: tag::Id) -> Result<(), DomainError> {
        if self.tag_ids.contains(&tag_id) {
            return Err(DomainError::TagAlreadyAttached(self.id, tag_id));
        }
        self.tag_ids.push(tag_id);
        Ok(())
    }
    pub fn remove_tag_id(&mut self, tag_id: tag::Id) -> Result<(), DomainError> {
        if !self.tag_ids.contains(&tag_id) {
            return Err(DomainError::TagNotFound(self.id, tag_id));
        }
        self.tag_ids.retain(|id| *id != tag_id);
        Ok(())
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct NewArticle {
    pub title: Title,
    pub description: Description,
    pub content: Option<Content>,
    pub series_id: Option<series::Id>,
    pub category_ids: Vec<category::Id>,
    pub tag_ids: Vec<tag::Id>,
    version: Version,
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
    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn bump_version(&mut self) {
        self.version.bump();
    }
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
    #[error("Article with id {0} already has category with id {1} attached")]
    CategoryAlreadyAttached(Id, category::Id),
    #[error("Article with id {0} does not have category with id {1} attached")]
    CategoryNotFound(Id, category::Id),
    #[error("Article with id {0} already has tag with id {1} attached")]
    TagAlreadyAttached(Id, tag::Id),
    #[error("Article with id {0} does not have tag with id {1} attached")]
    TagNotFound(Id, tag::Id),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl DomainError {
    pub fn is_already_published(&self) -> bool {
        matches!(self, DomainError::AlreadyPublished(_))
    }

    pub fn is_not_published(&self) -> bool {
        matches!(self, DomainError::NotPublished(_))
    }

    pub fn is_already_soft_deleted(&self) -> bool {
        matches!(self, DomainError::AlreadySoftDeleted(_))
    }

    pub fn is_not_soft_deleted(&self) -> bool {
        matches!(self, DomainError::NotSoftDeleted(_))
    }

    pub fn is_category_already_attached(&self) -> bool {
        matches!(self, DomainError::CategoryAlreadyAttached(_, _))
    }

    pub fn is_category_not_found(&self) -> bool {
        matches!(self, DomainError::CategoryNotFound(_, _))
    }

    pub fn is_tag_already_attached(&self) -> bool {
        matches!(self, DomainError::TagAlreadyAttached(_, _))
    }

    pub fn is_tag_not_found(&self) -> bool {
        matches!(self, DomainError::TagNotFound(_, _))
    }
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

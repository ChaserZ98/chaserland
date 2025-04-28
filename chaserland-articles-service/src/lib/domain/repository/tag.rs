use crate::domain::entity::tag;
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};

#[async_trait]
pub trait TagRepository: Send + Sync + 'static {
    async fn create(&self, name: tag::Name) -> Result<tag::Tag, TagRepositoryError>;
    async fn get_one(&self, identifier: tag::Identifier) -> Result<tag::Tag, TagRepositoryError>;
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
    ) -> Result<Vec<tag::Tag>, TagRepositoryError>;
    async fn delete(&self, identifier: tag::Identifier) -> Result<(), TagRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TagRepositoryError {
    #[error("Tag with identifier {0} not found")]
    TagNotFound(tag::Identifier),
    #[error("Tag {0} with slug {1} already exists")]
    DuplicateTagSlug(tag::Name, tag::Slug),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

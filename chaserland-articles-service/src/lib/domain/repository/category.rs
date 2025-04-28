use crate::domain::entity::category;
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};

#[async_trait]
pub trait CategoryRepository: Send + Sync + 'static {
    async fn create(
        &self,
        name: category::Name,
    ) -> Result<category::Category, CategoryRepositoryError>;
    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<category::Category, CategoryRepositoryError>;
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
    ) -> Result<Vec<category::Category>, CategoryRepositoryError>;
    async fn delete(&self, identifier: category::Identifier)
    -> Result<(), CategoryRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CategoryRepositoryError {
    #[error("Category with identifier {0} not found")]
    CategoryNotFound(category::Identifier),
    #[error("Series {0} with slug {1} already exists")]
    DuplicateCategorySlug(category::Name, category::Slug),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

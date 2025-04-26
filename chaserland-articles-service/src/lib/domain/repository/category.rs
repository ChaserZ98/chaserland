use crate::domain::entity::category;
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};
pub use error::*;

#[async_trait]
pub trait CategoryRepository {
    async fn create(
        &self,
        name: category::Name,
    ) -> Result<category::Category, error::CreateCategoryError>;
    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<category::Category, error::GetCategoryError>;
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
    ) -> Result<Vec<category::Category>, error::GetCategoryError>;
    async fn delete(
        &self,
        identifier: category::Identifier,
    ) -> Result<(), error::DeleteCategoryError>;
}

pub mod error {
    use super::category;

    #[derive(Debug, thiserror::Error)]
    pub enum CreateCategoryError {
        #[error("Category {0} with slug {1} already exists")]
        AlreadyExists(category::Name, category::Slug),
        #[error(transparent)]
        Unknown(#[from] anyhow::Error),
    }

    #[derive(Debug, thiserror::Error)]
    pub enum GetCategoryError {
        #[error("Category with identifier {0} not found")]
        NotFound(category::Identifier),
        #[error(transparent)]
        Unknown(#[from] anyhow::Error),
    }

    #[derive(Debug, thiserror::Error)]
    pub enum DeleteCategoryError {
        #[error("Category with identifier {0} not found")]
        NotFound(category::Identifier),
        #[error(transparent)]
        Unknown(#[from] anyhow::Error),
    }
}

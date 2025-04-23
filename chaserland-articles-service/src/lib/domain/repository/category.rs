use crate::domain::entity::category;
use async_trait::async_trait;
use chaserland_protos::article::v1::Category;

#[async_trait]
pub trait CategoryRepository {
    async fn create(&self, name: category::Name) -> Result<Category, CreateCategoryError>;
    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<category::Category, GetCategoryError>;
    async fn delete(&self, identifier: category::Identifier) -> Result<(), DeleteCategoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CreateCategoryError {
    #[error("Category with name {0} already exists")]
    AlreadyExists(category::Name),
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

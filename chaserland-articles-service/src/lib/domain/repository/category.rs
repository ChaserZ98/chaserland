use crate::domain::entity::category;
use async_trait::async_trait;
use chaserland_protos::article::v1::Category;

#[async_trait]
pub trait CategoryRepository {
    async fn create(&self, name: category::CategoryName) -> Result<Category, CreateCategoryError>;
    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<category::Category, GetCategoryError>;
    async fn delete(&self, identifier: category::Identifier) -> Result<(), DeleteCategoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CreateCategoryError {
    #[error("Category with name {name} already exists")]
    AlreadyExists { name: category::CategoryName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetCategoryError {
    #[error("Category with identifier {identifier} not found")]
    NotFound { identifier: category::Identifier },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteCategoryError {
    #[error("Category with identifier {identifier} not found")]
    NotFound { identifier: category::Identifier },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

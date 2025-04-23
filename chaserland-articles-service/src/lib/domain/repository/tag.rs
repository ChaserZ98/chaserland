use crate::domain::entity::tag;
use async_trait::async_trait;

#[async_trait]
pub trait TagRepository {
    async fn create(&self, name: tag::Name) -> Result<tag::Tag, CreateTagError>;
    async fn get_one(&self, identifier: tag::Identifier) -> Result<tag::Tag, GetTagError>;
    async fn delete(&self, identifier: tag::Identifier) -> Result<(), DeleteTagError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTagError {
    #[error("Tag with name {0} already exists")]
    AlreadyExists(tag::Name),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetTagError {
    #[error("Tag with identifier {0} not found")]
    NotFound(tag::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteTagError {
    #[error("Tag with identifier {0} not found")]
    NotFound(tag::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

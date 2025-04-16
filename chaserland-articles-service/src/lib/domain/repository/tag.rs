use crate::domain::model::tag;
use async_trait::async_trait;

#[async_trait]
pub trait TagRepository {
    type Tx;
    async fn create(
        &self,
        name: tag::TagName,
        tx: &mut Self::Tx,
    ) -> Result<tag::Tag, CreateTagError>;
    async fn get_one(&self, identifier: tag::Identifier) -> Result<tag::Tag, GetTagError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTagError {
    #[error("Tag {} already exists or slug {} is already taken", name, name.as_slug())]
    AlreadyExists { name: tag::TagName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetTagError {
    #[error("Tag with identifier {identifier} not found")]
    NotFound { identifier: tag::Identifier },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

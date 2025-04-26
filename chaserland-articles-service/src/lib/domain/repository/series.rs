use crate::domain::entity::series;
use async_trait::async_trait;

#[async_trait]
pub trait SeriesRepository {
    async fn create(&self, name: series::Name) -> Result<series::Series, CreateSeriesError>;
    async fn get_one(
        &self,
        identifier: series::Identifier,
    ) -> Result<series::Series, GetSeriesError>;
    async fn delete(&self, identifier: series::Identifier) -> Result<(), DeleteSeriesError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CreateSeriesError {
    #[error("Series {0} already exists or slug {1} is already taken")]
    AlreadyExists(series::Name, series::Slug),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetSeriesError {
    #[error("Series with identifier {0} not found")]
    NotFound(series::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSeriesError {
    #[error("Series with identifier {0} not found")]
    NotFound(series::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

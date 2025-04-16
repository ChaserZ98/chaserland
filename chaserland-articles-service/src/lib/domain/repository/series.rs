use async_trait::async_trait;

use crate::domain::entity::series;

#[async_trait]
pub trait SeriesRepository {
    type Tx;

    async fn create(
        &self,
        name: series::SeriesName,
        tx: &mut Self::Tx,
    ) -> Result<series::Series, CreateSeriesError>;
    async fn get_one(
        &self,
        identifier: series::Identifier,
        tx: &mut Self::Tx,
    ) -> Result<series::Series, GetSeriesError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CreateSeriesError {
    #[error("Series {} already exists or slug {} is already taken", name, name.as_slug())]
    AlreadyExists { name: series::SeriesName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetSeriesError {
    #[error("Series with identifier {identifier} not found")]
    NotFound { identifier: series::Identifier },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

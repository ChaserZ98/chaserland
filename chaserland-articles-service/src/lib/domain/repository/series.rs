use crate::domain::entity::series;
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};

#[async_trait]
pub trait SeriesRepository: Send + Sync + 'static {
    async fn create(&self, name: series::Name) -> Result<series::Series, SeriesRepositoryError>;
    async fn get_one(
        &self,
        identifier: series::Identifier,
    ) -> Result<series::Series, SeriesRepositoryError>;
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
    ) -> Result<Vec<series::Series>, SeriesRepositoryError>;
    async fn delete(&self, identifier: series::Identifier) -> Result<(), SeriesRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SeriesRepositoryError {
    #[error("Series with identifier {0} not found")]
    SeriesNotFound(series::Identifier),
    #[error("Series {0} with slug {1} already exists")]
    DuplicateSeriesSlug(series::Name, series::Slug),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

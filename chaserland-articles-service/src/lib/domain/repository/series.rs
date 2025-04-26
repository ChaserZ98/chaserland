use crate::domain::entity::series;
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};
pub use error::*;

#[async_trait]
pub trait SeriesRepository {
    async fn create(&self, name: series::Name) -> Result<series::Series, error::CreateSeriesError>;
    async fn get_one(
        &self,
        identifier: series::Identifier,
    ) -> Result<series::Series, error::GetSeriesError>;
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
    ) -> Result<Vec<series::Series>, error::GetSeriesError>;
    async fn delete(&self, identifier: series::Identifier) -> Result<(), error::DeleteSeriesError>;
}

pub mod error {
    use super::series;

    #[derive(Debug, thiserror::Error)]
    pub enum CreateSeriesError {
        #[error("Series {0} with slug {1} already exists")]
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
}

use super::{SeriesFilter, SeriesRepositoryError};
use crate::domain::series::entity::Series;
use crate::domain::series::vo as series;
use async_trait::async_trait;
use chaserland_common::pagination::Pagination;

#[async_trait]
pub trait SeriesRepository: Send + Sync + 'static {
    async fn create(&self, series: series::NewSeries) -> Result<Series, SeriesRepositoryError>;

    async fn get_one(
        &self,
        identifier: series::Identifier,
    ) -> Result<Series, SeriesRepositoryError>;

    async fn get_many(
        &self,
        filter: Option<SeriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Series>, SeriesRepositoryError>;

    async fn delete(&self, identifier: series::Identifier) -> Result<(), SeriesRepositoryError>;
}

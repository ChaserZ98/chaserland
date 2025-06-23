use super::{SeriesFilter, SeriesRepositoryError};
use crate::domain::series::entity::Series;
use crate::domain::series::vo as series;
use chaserland_common::pagination::Pagination;

#[trait_variant::make(SeriesRepository: Send)]
pub trait LocalSeriesRepository: Clone + Sync + 'static {
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

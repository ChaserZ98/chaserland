use super::SeriesRepositoryError;
use crate::domain::series::{entity::Series, vo as series};
use sqlx::{Database, Transaction};

#[trait_variant::make(SeriesRepository: Send)]
pub trait LocalSeriesRepository: Clone + Sync + 'static {
    type DB: Database;
    async fn create(
        &self,
        series: series::NewSeries,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Series, SeriesRepositoryError>;

    async fn get_one(
        &self,
        identifier: series::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Series, SeriesRepositoryError>;

    async fn delete(
        &self,
        identifier: series::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), SeriesRepositoryError>;
}

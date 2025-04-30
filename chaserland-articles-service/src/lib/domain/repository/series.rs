use crate::domain::entity::series;
use async_trait::async_trait;
use chaserland_common::pagination::Pagination;

#[async_trait]
pub trait SeriesRepository: Send + Sync + 'static {
    async fn create(
        &self,
        series: series::NewSeries,
    ) -> Result<series::Series, SeriesRepositoryError>;
    async fn get_one(
        &self,
        identifier: series::Identifier,
    ) -> Result<series::Series, SeriesRepositoryError>;
    async fn get_many(
        &self,
        filter: Option<SeriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<series::Series>, SeriesRepositoryError>;
    async fn delete(&self, identifier: series::Identifier) -> Result<(), SeriesRepositoryError>;
}

#[derive(Debug)]
pub struct SeriesFilter {
    series_ids: Vec<series::Id>,
}

impl SeriesFilter {
    pub fn new(series_ids: Vec<series::Id>) -> Self {
        Self::validate(&series_ids).unwrap();
        Self { series_ids }
    }

    pub fn series_ids(&self) -> &Vec<series::Id> {
        &self.series_ids
    }

    pub fn validate(series_ids: &Vec<series::Id>) -> Result<(), String> {
        match series_ids.is_empty() {
            true => Err("At least one series id must be specified".to_string()),
            false => Ok(()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SeriesRepositoryError {
    #[error("Series with identifier {0} not found")]
    SeriesNotFound(series::Identifier),
    #[error("Series {} with slug {} already exists", .0.name, .0.slug())]
    DuplicateSeriesSlug(series::NewSeries),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl SeriesRepositoryError {
    pub fn is_series_not_found(&self) -> bool {
        matches!(self, SeriesRepositoryError::SeriesNotFound(_))
    }

    pub fn is_duplicate_series_slug(&self) -> bool {
        matches!(self, SeriesRepositoryError::DuplicateSeriesSlug(_))
    }

    pub fn is_transaction_error(&self) -> bool {
        matches!(self, SeriesRepositoryError::Transaction(_))
    }

    pub fn is_do_conversion_error(&self) -> bool {
        matches!(self, SeriesRepositoryError::DOConversion(_))
    }
}

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

    fn validate(series_ids: &Vec<series::Id>) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    pub use super::{SeriesFilter, SeriesRepositoryError};

    mod series_filter {
        use super::SeriesFilter;

        #[test]
        fn series_filter_case_new() {
            let series_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

            let series_filter = SeriesFilter::new(series_ids.clone());

            assert_eq!(series_filter.series_ids(), &series_ids);
        }

        #[test]
        #[should_panic(expected = "At least one series id must be specified")]
        fn series_filter_case_new_panic() {
            SeriesFilter::new(vec![]);
        }
    }

    mod series_repository_error {
        use super::SeriesRepositoryError;
        use crate::domain::entity::series;

        #[test]
        fn series_repository_error_case_series_not_found() {
            let series_id: series::Id = 1.try_into().unwrap();
            let err = SeriesRepositoryError::SeriesNotFound(series_id.as_identifier());

            assert!(err.is_series_not_found());

            let err = SeriesRepositoryError::Unknown(anyhow::anyhow!("test"));

            assert_eq!(err.is_series_not_found(), false);
        }

        #[test]
        fn series_repository_error_case_duplicate_series_slug() {
            let new_series = series::NewSeries::new("name".try_into().unwrap());
            let err = SeriesRepositoryError::DuplicateSeriesSlug(new_series);

            assert!(err.is_duplicate_series_slug());

            let err = SeriesRepositoryError::Unknown(anyhow::anyhow!("test"));

            assert_eq!(err.is_duplicate_series_slug(), false);
        }

        #[test]
        fn series_repository_error_case_transaction() {
            let err = SeriesRepositoryError::Transaction("test".to_string());

            assert!(err.is_transaction_error());

            let err = SeriesRepositoryError::Unknown(anyhow::anyhow!("test"));

            assert_eq!(err.is_transaction_error(), false);
        }

        #[test]
        fn series_repository_error_case_do_conversion() {
            let err = SeriesRepositoryError::DOConversion("test".to_string());

            assert!(err.is_do_conversion_error());

            let err = SeriesRepositoryError::Unknown(anyhow::anyhow!("test"));

            assert_eq!(err.is_do_conversion_error(), false);
        }
    }
}

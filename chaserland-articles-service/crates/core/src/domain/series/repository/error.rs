use crate::domain::series::vo as series;

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
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
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
    use super::SeriesRepositoryError;
    use crate::domain::series::vo as series;

    #[test]
    fn series_repository_error_case_series_not_found() {
        let series_id: series::Id = 1.try_into().unwrap();
        let err = SeriesRepositoryError::SeriesNotFound(series_id.as_identifier());

        assert!(err.is_series_not_found());

        let new_series = series::NewSeries::new("name".try_into().unwrap());
        let err = SeriesRepositoryError::DuplicateSeriesSlug(new_series);

        assert_eq!(err.is_series_not_found(), false);
    }

    #[test]
    fn series_repository_error_case_duplicate_series_slug() {
        let new_series = series::NewSeries::new("name".try_into().unwrap());
        let err = SeriesRepositoryError::DuplicateSeriesSlug(new_series);

        assert!(err.is_duplicate_series_slug());

        let err = SeriesRepositoryError::Transaction("test".to_string());

        assert_eq!(err.is_duplicate_series_slug(), false);
    }

    #[test]
    fn series_repository_error_case_transaction() {
        let err = SeriesRepositoryError::Transaction("test".to_string());

        assert!(err.is_transaction_error());

        let err = SeriesRepositoryError::DOConversion("test".to_string());

        assert_eq!(err.is_transaction_error(), false);
    }

    #[test]
    fn series_repository_error_case_do_conversion() {
        let err = SeriesRepositoryError::DOConversion("test".to_string());

        assert!(err.is_do_conversion_error());

        let series_id: series::Id = 1.try_into().unwrap();
        let err = SeriesRepositoryError::SeriesNotFound(series_id.as_identifier());

        assert_eq!(err.is_do_conversion_error(), false);
    }
}

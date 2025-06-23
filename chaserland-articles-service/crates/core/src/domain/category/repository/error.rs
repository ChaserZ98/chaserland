use crate::domain::category::vo as category;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CategoryRepositoryError {
    #[error("Category with identifier {0} not found")]
    CategoryNotFound(category::Identifier),
    #[error("Series {} with slug {} already exists", .0.name, .0.slug())]
    DuplicateCategorySlug(category::NewCategory),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl CategoryRepositoryError {
    pub fn is_category_not_found(&self) -> bool {
        matches!(self, Self::CategoryNotFound(_))
    }

    pub fn is_duplicate_category_slug(&self) -> bool {
        matches!(self, Self::DuplicateCategorySlug(..))
    }

    pub fn is_transaction(&self) -> bool {
        matches!(self, Self::Transaction(_))
    }

    pub fn is_do_conversion(&self) -> bool {
        matches!(self, Self::DOConversion(_))
    }
}

#[cfg(test)]
mod tests {
    use super::CategoryRepositoryError;
    use crate::domain::category::vo as category;

    #[test]
    fn category_repository_error_case_category_not_found() {
        let category_id = category::Id::new(1);
        let err = CategoryRepositoryError::CategoryNotFound(category_id.as_identifier());

        assert!(err.is_category_not_found());

        let new_category = category::NewCategory::new("test".try_into().unwrap());
        let err = CategoryRepositoryError::DuplicateCategorySlug(new_category);

        assert_eq!(err.is_category_not_found(), false);
    }

    #[test]
    fn category_repository_error_case_duplicate_category_slug() {
        let new_category = category::NewCategory::new("test".try_into().unwrap());
        let err = CategoryRepositoryError::DuplicateCategorySlug(new_category);

        assert!(err.is_duplicate_category_slug());

        let err = CategoryRepositoryError::Transaction("test".to_string());

        assert_eq!(err.is_duplicate_category_slug(), false);
    }

    #[test]
    fn category_repository_error_case_transaction() {
        let err = CategoryRepositoryError::Transaction("test".to_string());

        assert!(err.is_transaction());

        let err = CategoryRepositoryError::DOConversion("test".to_string());

        assert_eq!(err.is_transaction(), false);
    }

    #[test]
    fn category_repository_error_case_do_conversion() {
        let err = CategoryRepositoryError::DOConversion("test".to_string());

        assert!(err.is_do_conversion());

        let category_id = category::Id::new(1);
        let err = CategoryRepositoryError::CategoryNotFound(category_id.as_identifier());

        assert_eq!(err.is_do_conversion(), false);
    }
}

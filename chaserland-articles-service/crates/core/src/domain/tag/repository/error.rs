use crate::domain::tag::vo as tag;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TagRepositoryError {
    #[error("Tag with identifier {0} not found")]
    TagNotFound(tag::Identifier),
    #[error("Tag {} with slug {} already exists", .0.name, .0.slug())]
    DuplicateTagSlug(tag::NewTag),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl TagRepositoryError {
    pub fn is_tag_not_found(&self) -> bool {
        matches!(self, TagRepositoryError::TagNotFound(_))
    }

    pub fn is_duplicate_tag_slug(&self) -> bool {
        matches!(self, TagRepositoryError::DuplicateTagSlug(_))
    }

    pub fn is_transaction_error(&self) -> bool {
        matches!(self, TagRepositoryError::Transaction(_))
    }

    pub fn is_do_conversion_error(&self) -> bool {
        matches!(self, TagRepositoryError::DOConversion(_))
    }
}

#[cfg(test)]
mod tests {
    use super::TagRepositoryError;
    use crate::domain::tag::vo as tag;

    #[test]
    fn tag_repository_error_case_tag_not_found() {
        let tag_id: tag::Id = 1.try_into().unwrap();
        let err = TagRepositoryError::TagNotFound(tag_id.as_identifier());

        assert!(err.is_tag_not_found());

        let new_tag = tag::NewTag::new("test".try_into().unwrap());
        let err = TagRepositoryError::DuplicateTagSlug(new_tag);

        assert_eq!(err.is_tag_not_found(), false);
    }

    #[test]
    fn tag_repository_error_case_duplicate_tag_slug() {
        let new_tag = tag::NewTag::new("test".try_into().unwrap());
        let err = TagRepositoryError::DuplicateTagSlug(new_tag);

        assert!(err.is_duplicate_tag_slug());

        let err = TagRepositoryError::Transaction("test".to_string());

        assert_eq!(err.is_duplicate_tag_slug(), false);
    }

    #[test]
    fn tag_repository_error_case_transaction() {
        let err = TagRepositoryError::Transaction("test".to_string());

        assert!(err.is_transaction_error());

        let err = TagRepositoryError::DOConversion("test".to_string());

        assert_eq!(err.is_transaction_error(), false);
    }

    #[test]
    fn tag_repository_error_case_do_conversion() {
        let err = TagRepositoryError::DOConversion("test".to_string());

        assert!(err.is_do_conversion_error());

        let tag_id: tag::Id = 1.try_into().unwrap();
        let err = TagRepositoryError::TagNotFound(tag_id.as_identifier());

        assert_eq!(err.is_do_conversion_error(), false);
    }
}

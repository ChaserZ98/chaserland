use crate::domain::entity::tag;
use async_trait::async_trait;
use chaserland_common::pagination::Pagination;

#[async_trait]
pub trait TagRepository: Send + Sync + 'static {
    async fn create(&self, new_tag: tag::NewTag) -> Result<tag::Tag, TagRepositoryError>;

    async fn get_one(&self, identifier: tag::Identifier) -> Result<tag::Tag, TagRepositoryError>;

    async fn get_many(
        &self,
        filter: Option<TagsFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<tag::Tag>, TagRepositoryError>;

    async fn delete(&self, identifier: tag::Identifier) -> Result<(), TagRepositoryError>;
}

#[derive(Debug)]
pub struct TagsFilter {
    tag_ids: Vec<tag::Id>,
}

impl TagsFilter {
    pub fn new(tag_ids: Vec<tag::Id>) -> Self {
        Self::validate(&tag_ids).unwrap();
        Self { tag_ids }
    }

    pub fn tag_ids(&self) -> &Vec<tag::Id> {
        &self.tag_ids
    }

    fn validate(tag_ids: &Vec<tag::Id>) -> Result<(), String> {
        match tag_ids.is_empty() {
            true => Err("At least one tag id must be specified".to_string()),
            false => Ok(()),
        }
    }
}

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
    Unknown(#[from] anyhow::Error),
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
    pub use super::{TagRepositoryError, TagsFilter};
    mod tags_filter {
        use super::TagsFilter;

        #[test]
        fn tags_filter_case_new() {
            let tag_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
            let tag_filter = TagsFilter::new(tag_ids.clone());

            assert_eq!(tag_filter.tag_ids(), &tag_ids);
        }

        #[test]
        #[should_panic(expected = "At least one tag id must be specified")]
        fn tags_filter_case_new_panic() {
            TagsFilter::new(vec![]);
        }
    }
    mod tag_repository_error {
        use super::TagRepositoryError;
        use crate::domain::entity::tag;

        #[test]
        fn tag_repository_error_case_tag_not_found() {
            let tag_id: tag::Id = 1.try_into().unwrap();
            let err = TagRepositoryError::TagNotFound(tag_id.as_identifier());

            assert!(err.is_tag_not_found());

            let err = TagRepositoryError::Unknown(anyhow::anyhow!("test"));

            assert_eq!(err.is_tag_not_found(), false);
        }

        #[test]
        fn tag_repository_error_case_duplicate_tag_slug() {
            let new_tag = tag::NewTag::new("test".try_into().unwrap());
            let err = TagRepositoryError::DuplicateTagSlug(new_tag);

            assert!(err.is_duplicate_tag_slug());

            let err = TagRepositoryError::Unknown(anyhow::anyhow!("test"));

            assert_eq!(err.is_duplicate_tag_slug(), false);
        }

        #[test]
        fn tag_repository_error_case_transaction() {
            let err = TagRepositoryError::Transaction("test".to_string());

            assert!(err.is_transaction_error());

            let err = TagRepositoryError::Unknown(anyhow::anyhow!("test"));

            assert_eq!(err.is_transaction_error(), false);
        }

        #[test]
        fn tag_repository_error_case_do_conversion() {
            let err = TagRepositoryError::DOConversion("test".to_string());

            assert!(err.is_do_conversion_error());

            let err = TagRepositoryError::Unknown(anyhow::anyhow!("test"));

            assert_eq!(err.is_do_conversion_error(), false);
        }
    }
}

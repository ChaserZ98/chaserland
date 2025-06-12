use super::vo::Id;
use crate::domain::category::vo as category;
use crate::domain::tag::vo as tag;

#[derive(Debug, thiserror::Error)]
pub enum ArticleDomainError {
    #[error("Article with id {0} is already published")]
    AlreadyPublished(Id),
    #[error("Article with id {0} has not been published")]
    NotPublished(Id),
    #[error("Article with id {0} is already soft deleted")]
    AlreadySoftDeleted(Id),
    #[error("Article with id {0} has not been soft deleted")]
    NotSoftDeleted(Id),
    #[error("Article with id {0} already has category with id {1} attached")]
    CategoryAlreadyAttached(Id, category::Id),
    #[error("Article with id {0} does not have category with id {1} attached")]
    CategoryNotFound(Id, category::Id),
    #[error("Article with id {0} already has tag with id {1} attached")]
    TagAlreadyAttached(Id, tag::Id),
    #[error("Article with id {0} does not have tag with id {1} attached")]
    TagNotFound(Id, tag::Id),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl ArticleDomainError {
    pub fn is_already_published(&self) -> bool {
        matches!(self, ArticleDomainError::AlreadyPublished(_))
    }

    pub fn is_not_published(&self) -> bool {
        matches!(self, ArticleDomainError::NotPublished(_))
    }

    pub fn is_already_soft_deleted(&self) -> bool {
        matches!(self, ArticleDomainError::AlreadySoftDeleted(_))
    }

    pub fn is_not_soft_deleted(&self) -> bool {
        matches!(self, ArticleDomainError::NotSoftDeleted(_))
    }

    pub fn is_category_already_attached(&self) -> bool {
        matches!(self, ArticleDomainError::CategoryAlreadyAttached(_, _))
    }

    pub fn is_category_not_found(&self) -> bool {
        matches!(self, ArticleDomainError::CategoryNotFound(_, _))
    }

    pub fn is_tag_already_attached(&self) -> bool {
        matches!(self, ArticleDomainError::TagAlreadyAttached(_, _))
    }

    pub fn is_tag_not_found(&self) -> bool {
        matches!(self, ArticleDomainError::TagNotFound(_, _))
    }
}

#[cfg(test)]
mod tests {
    use super::ArticleDomainError;

    #[test]
    fn article_domain_error_case_already_published() {
        let err = ArticleDomainError::AlreadyPublished(1.try_into().unwrap());

        assert!(err.is_already_published());
    }

    #[test]
    fn article_domain_error_case_not_published() {
        let err = ArticleDomainError::NotPublished(1.try_into().unwrap());

        assert!(err.is_not_published());
    }

    #[test]
    fn article_domain_error_case_already_soft_deleted() {
        let err = ArticleDomainError::AlreadySoftDeleted(1.try_into().unwrap());

        assert!(err.is_already_soft_deleted());
    }

    #[test]
    fn article_domain_error_case_not_soft_deleted() {
        let err = ArticleDomainError::NotSoftDeleted(1.try_into().unwrap());

        assert!(err.is_not_soft_deleted());
    }

    #[test]
    fn article_domain_error_case_category_already_attached() {
        let err = ArticleDomainError::CategoryAlreadyAttached(
            1.try_into().unwrap(),
            2.try_into().unwrap(),
        );

        assert!(err.is_category_already_attached());
    }

    #[test]
    fn article_domain_error_case_category_not_found() {
        let err =
            ArticleDomainError::CategoryNotFound(1.try_into().unwrap(), 2.try_into().unwrap());

        assert!(err.is_category_not_found());
    }

    #[test]
    fn article_domain_error_case_tag_already_attached() {
        let err =
            ArticleDomainError::TagAlreadyAttached(1.try_into().unwrap(), 2.try_into().unwrap());

        assert!(err.is_tag_already_attached());
    }

    #[test]
    fn article_domain_error_case_tag_not_found() {
        let err = ArticleDomainError::TagNotFound(1.try_into().unwrap(), 2.try_into().unwrap());

        assert!(err.is_tag_not_found());
    }
}

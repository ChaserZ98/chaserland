use crate::domain::{
    article::{error::ArticleDomainError, repository::ArticleRepositoryError, vo as article},
    category::{repository::CategoryRepositoryError, vo as category},
    error::{DomainError, RepositoryError},
    series::{repository::SeriesRepositoryError, vo as series},
    tag::{repository::TagRepositoryError, vo as tag},
};

#[derive(Debug, thiserror::Error)]
pub enum CreateArticleError {
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
    #[error("Series with identifier {0} not found")]
    SeriesNotFound(series::Identifier),
    #[error("Category with identifier {0} not found")]
    CategoryNotFound(category::Identifier),
    #[error("Tag with identifier {0} not found")]
    TagNotFound(tag::Identifier),
    #[error("Article with slug {0} already exists")]
    DuplicateSlug(article::Slug),
    #[error("Version conflict: {0}")]
    DataVersionConflict(String),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

impl From<SeriesRepositoryError> for CreateArticleError {
    fn from(value: SeriesRepositoryError) -> Self {
        match value {
            SeriesRepositoryError::SeriesNotFound(identifier) => Self::SeriesNotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

impl From<CategoryRepositoryError> for CreateArticleError {
    fn from(value: CategoryRepositoryError) -> Self {
        match value {
            CategoryRepositoryError::CategoryNotFound(identifier) => {
                Self::CategoryNotFound(identifier)
            }
            _ => Self::Repository(value.into()),
        }
    }
}

impl From<TagRepositoryError> for CreateArticleError {
    fn from(value: TagRepositoryError) -> Self {
        match value {
            TagRepositoryError::TagNotFound(identifier) => Self::TagNotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

impl From<ArticleRepositoryError> for CreateArticleError {
    fn from(value: ArticleRepositoryError) -> Self {
        match value {
            ArticleRepositoryError::DuplicateArticleSlug(slug) => Self::DuplicateSlug(slug),
            ArticleRepositoryError::SeriesNotFound(identifier) => {
                Self::DataVersionConflict(format!(
                    "Series with identifier {identifier} not found in database when creating article"
                ))
            }
            ArticleRepositoryError::CategoryNotFound(identifier) => {
                Self::DataVersionConflict(format!(
                    "Category with identifier {identifier} not found in database when creating article"
                ))
            }
            ArticleRepositoryError::TagNotFound(identifier) => Self::DataVersionConflict(format!(
                "Tag with identifier {identifier} not found in database when creating article"
            )),
            _ => Self::Repository(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateSeriesError {
    #[error("Series with slug {0} already exists")]
    DuplicateSlug(series::Slug),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<SeriesRepositoryError> for CreateSeriesError {
    fn from(value: SeriesRepositoryError) -> Self {
        match value {
            SeriesRepositoryError::DuplicateSeriesSlug(new_series) => {
                Self::DuplicateSlug(new_series.slug().clone())
            }
            _ => Self::Repository(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateCategoryError {
    #[error("Category with slug {0} already exists")]
    DuplicateSlug(category::Slug),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<CategoryRepositoryError> for CreateCategoryError {
    fn from(value: CategoryRepositoryError) -> Self {
        match value {
            CategoryRepositoryError::DuplicateCategorySlug(new_category) => {
                Self::DuplicateSlug(new_category.slug().clone())
            }
            _ => Self::Repository(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTagError {
    #[error("Tag with slug {0} already exists")]
    DuplicateSlug(tag::Slug),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<TagRepositoryError> for CreateTagError {
    fn from(value: TagRepositoryError) -> Self {
        match value {
            TagRepositoryError::DuplicateTagSlug(new_tag) => {
                Self::DuplicateSlug(new_tag.slug().clone())
            }
            _ => Self::Repository(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateArticleError {
    #[error("Article with identifier {0} not found")]
    ArticleNotFound(article::Identifier),
    #[error("Series with identifier {0} not found")]
    SeriesNotFound(series::Identifier),
    #[error("Category with identifier {0} not found")]
    CategoryNotFound(category::Identifier),
    #[error("Tag with identifier {0} not found")]
    TagNotFound(tag::Identifier),
    #[error("Article with slug {0} already exists")]
    DuplicateSlug(article::Slug),
    #[error("Version conflict: {0}")]
    DataVersionConflict(String),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<ArticleRepositoryError> for UpdateArticleError {
    fn from(value: ArticleRepositoryError) -> Self {
        match value {
            ArticleRepositoryError::ArticleNotFound(identifier) => {
                Self::ArticleNotFound(identifier)
            }
            _ => Self::Repository(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PublishArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error("Article with identifier {0} already published")]
    AlreadyPublished(article::Identifier),
    #[error("Version conflict: {0}")]
    DataVersionConflict(String),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<ArticleRepositoryError> for PublishArticleError {
    fn from(value: ArticleRepositoryError) -> Self {
        match value {
            ArticleRepositoryError::ArticleNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

impl From<ArticleDomainError> for PublishArticleError {
    fn from(value: ArticleDomainError) -> Self {
        match value {
            ArticleDomainError::AlreadyPublished(id) => {
                Self::AlreadyPublished(article::Identifier::Id(id))
            }
            _ => Self::Domain(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UnpublishArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error("Article with identifier {0} not published")]
    NotPublished(article::Identifier),
    #[error("Version conflict: {0}")]
    DataVersionConflict(String),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<ArticleRepositoryError> for UnpublishArticleError {
    fn from(value: ArticleRepositoryError) -> Self {
        match value {
            ArticleRepositoryError::ArticleNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

impl From<ArticleDomainError> for UnpublishArticleError {
    fn from(value: ArticleDomainError) -> Self {
        match value {
            ArticleDomainError::NotPublished(id) => Self::NotPublished(article::Identifier::Id(id)),
            _ => Self::Domain(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SoftDeleteArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error("Article with identifier {0} already soft deleted")]
    AlreadySoftDeleted(article::Identifier),
    #[error("Version conflict: {0}")]
    DataVersionConflict(String),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<ArticleRepositoryError> for SoftDeleteArticleError {
    fn from(value: ArticleRepositoryError) -> Self {
        match value {
            ArticleRepositoryError::ArticleNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

impl From<ArticleDomainError> for SoftDeleteArticleError {
    fn from(value: ArticleDomainError) -> Self {
        match value {
            ArticleDomainError::AlreadySoftDeleted(id) => {
                Self::AlreadySoftDeleted(article::Identifier::Id(id))
            }
            _ => Self::Domain(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RevokeSoftDeleteError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error("Article with identifier {0} not soft deleted")]
    NotSoftDeleted(article::Identifier),
    #[error("Version conflict: {0}")]
    DataVersionConflict(String),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<ArticleRepositoryError> for RevokeSoftDeleteError {
    fn from(value: ArticleRepositoryError) -> Self {
        match value {
            ArticleRepositoryError::ArticleNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

impl From<ArticleDomainError> for RevokeSoftDeleteError {
    fn from(value: ArticleDomainError) -> Self {
        match value {
            ArticleDomainError::NotSoftDeleted(id) => {
                Self::NotSoftDeleted(article::Identifier::Id(id))
            }
            _ => Self::Domain(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<ArticleRepositoryError> for DeleteArticleError {
    fn from(value: ArticleRepositoryError) -> Self {
        match value {
            ArticleRepositoryError::ArticleNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSeriesError {
    #[error("Series with identifier {0} not found")]
    NotFound(series::Identifier),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<SeriesRepositoryError> for DeleteSeriesError {
    fn from(value: SeriesRepositoryError) -> Self {
        match value {
            SeriesRepositoryError::SeriesNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteCategoryError {
    #[error("Category with identifier {0} not found")]
    NotFound(category::Identifier),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<CategoryRepositoryError> for DeleteCategoryError {
    fn from(value: CategoryRepositoryError) -> Self {
        match value {
            CategoryRepositoryError::CategoryNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteTagError {
    #[error("Tag with identifier {0} not found")]
    NotFound(tag::Identifier),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl From<TagRepositoryError> for DeleteTagError {
    fn from(value: TagRepositoryError) -> Self {
        match value {
            TagRepositoryError::TagNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::Repository(value.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_article_error_from_series_repository_error() {
        let err = SeriesRepositoryError::SeriesNotFound(series::Id::new(1).as_identifier());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::SeriesNotFound(_)));

        let err = SeriesRepositoryError::DOConversion("test".to_string());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::Repository(_)));
    }

    #[test]
    fn create_article_error_from_category_repository_error() {
        let err = CategoryRepositoryError::CategoryNotFound(category::Id::new(1).as_identifier());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::CategoryNotFound(_)));

        let err = CategoryRepositoryError::DOConversion("test".to_string());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::Repository(_)));
    }

    #[test]
    fn create_article_error_from_tag_repository_error() {
        let err = TagRepositoryError::TagNotFound(tag::Id::new(1).as_identifier());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::TagNotFound(_)));

        let err = TagRepositoryError::DOConversion("test".to_string());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::Repository(_)));
    }

    #[test]
    fn create_article_error_from_article_repository_error() {
        let err = ArticleRepositoryError::DuplicateArticleSlug(
            article::Title::new("test".to_string()).as_slug(),
        );

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::DuplicateSlug(_)));

        let err = ArticleRepositoryError::SeriesNotFound(series::Id::new(1).as_identifier());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::DataVersionConflict(_)));

        let err = ArticleRepositoryError::CategoryNotFound(category::Id::new(1).as_identifier());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::DataVersionConflict(_)));

        let err = ArticleRepositoryError::TagNotFound(tag::Id::new(1).as_identifier());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::DataVersionConflict(_)));

        let err = ArticleRepositoryError::DOConversion("test".to_string());

        let err = CreateArticleError::from(err);

        assert!(matches!(err, CreateArticleError::Repository(_)));
    }

    #[test]
    fn create_series_error_from_series_repository_error() {
        let err = SeriesRepositoryError::DuplicateSeriesSlug(series::NewSeries::new(
            "test".try_into().unwrap(),
        ));

        let err = CreateSeriesError::from(err);

        assert!(matches!(err, CreateSeriesError::DuplicateSlug(_)));

        let err = SeriesRepositoryError::DOConversion("test".to_string());

        let err = CreateSeriesError::from(err);

        assert!(matches!(err, CreateSeriesError::Repository(_)));
    }

    #[test]
    fn create_category_error_from_category_repository_error() {
        let err = CategoryRepositoryError::DuplicateCategorySlug(category::NewCategory::new(
            "test".try_into().unwrap(),
        ));

        let err = CreateCategoryError::from(err);

        assert!(matches!(err, CreateCategoryError::DuplicateSlug(_)));

        let err = CategoryRepositoryError::DOConversion("test".to_string());

        let err = CreateCategoryError::from(err);

        assert!(matches!(err, CreateCategoryError::Repository(_)));
    }

    #[test]
    fn create_tag_error_from_category_repository_error() {
        let err =
            TagRepositoryError::DuplicateTagSlug(tag::NewTag::new("test".try_into().unwrap()));

        let err = CreateTagError::from(err);

        assert!(matches!(err, CreateTagError::DuplicateSlug(_)));

        let err = TagRepositoryError::DOConversion("test".to_string());

        let err = CreateTagError::from(err);

        assert!(matches!(err, CreateTagError::Repository(_)));
    }

    #[test]
    fn update_article_error_from_article_repository_error() {
        let err = ArticleRepositoryError::ArticleNotFound(article::Id::new(1).as_identifier());

        let err = UpdateArticleError::from(err);

        assert!(matches!(err, UpdateArticleError::ArticleNotFound(_)));

        let err = ArticleRepositoryError::DOConversion("test".to_string());

        let err = UpdateArticleError::from(err);

        assert!(matches!(err, UpdateArticleError::Repository(_)));
    }

    #[test]
    fn publish_article_error_from_article_repository_error() {
        let err = ArticleRepositoryError::ArticleNotFound(article::Id::new(1).as_identifier());

        let err = PublishArticleError::from(err);

        assert!(matches!(err, PublishArticleError::NotFound(_)));

        let err = ArticleRepositoryError::DOConversion("test".to_string());

        let err = PublishArticleError::from(err);

        assert!(matches!(err, PublishArticleError::Repository(_)));
    }

    #[test]
    fn publish_article_error_from_article_domain_error() {
        let err = ArticleDomainError::AlreadyPublished(1.try_into().unwrap());

        let err = PublishArticleError::from(err);

        assert!(matches!(err, PublishArticleError::AlreadyPublished(_)));

        let err = ArticleDomainError::Unknown(anyhow::anyhow!("test"));

        let err = PublishArticleError::from(err);

        assert!(matches!(err, PublishArticleError::Domain(_)));
    }

    #[test]
    fn unpublish_article_error_from_article_repository_error() {
        let err = ArticleRepositoryError::ArticleNotFound(article::Id::new(1).as_identifier());

        let err = UnpublishArticleError::from(err);

        assert!(matches!(err, UnpublishArticleError::NotFound(_)));

        let err = ArticleRepositoryError::DOConversion("test".to_string());

        let err = UnpublishArticleError::from(err);

        assert!(matches!(err, UnpublishArticleError::Repository(_)));
    }

    #[test]
    fn unpublish_article_error_from_article_domain_error() {
        let err = ArticleDomainError::NotPublished(1.try_into().unwrap());

        let err = UnpublishArticleError::from(err);

        assert!(matches!(err, UnpublishArticleError::NotPublished(_)));

        let err = ArticleDomainError::Unknown(anyhow::anyhow!("test"));

        let err = UnpublishArticleError::from(err);

        assert!(matches!(err, UnpublishArticleError::Domain(_)));
    }

    #[test]
    fn soft_delete_article_error_from_article_repository_error() {
        let err = ArticleRepositoryError::ArticleNotFound(article::Id::new(1).as_identifier());

        let err = SoftDeleteArticleError::from(err);

        assert!(matches!(err, SoftDeleteArticleError::NotFound(_)));

        let err = ArticleRepositoryError::DOConversion("test".to_string());

        let err = SoftDeleteArticleError::from(err);

        assert!(matches!(err, SoftDeleteArticleError::Repository(_)));
    }

    #[test]
    fn soft_delete_article_error_from_article_domain_error() {
        let err = ArticleDomainError::AlreadySoftDeleted(1.try_into().unwrap());

        let err = SoftDeleteArticleError::from(err);

        assert!(matches!(err, SoftDeleteArticleError::AlreadySoftDeleted(_)));

        let err = ArticleDomainError::Unknown(anyhow::anyhow!("test"));

        let err = SoftDeleteArticleError::from(err);

        assert!(matches!(err, SoftDeleteArticleError::Domain(_)));
    }

    #[test]
    fn revoke_soft_delete_error_from_article_repository_error() {
        let err = ArticleRepositoryError::ArticleNotFound(article::Id::new(1).as_identifier());

        let err = RevokeSoftDeleteError::from(err);

        assert!(matches!(err, RevokeSoftDeleteError::NotFound(_)));

        let err = ArticleRepositoryError::DOConversion("test".to_string());

        let err = RevokeSoftDeleteError::from(err);

        assert!(matches!(err, RevokeSoftDeleteError::Repository(_)));
    }

    #[test]
    fn revoke_soft_delete_error_from_article_domain_error() {
        let err = ArticleDomainError::NotSoftDeleted(1.try_into().unwrap());

        let err = RevokeSoftDeleteError::from(err);

        assert!(matches!(err, RevokeSoftDeleteError::NotSoftDeleted(_)));

        let err = ArticleDomainError::Unknown(anyhow::anyhow!("test"));

        let err = RevokeSoftDeleteError::from(err);

        assert!(matches!(err, RevokeSoftDeleteError::Domain(_)));
    }

    #[test]
    fn delete_article_error_from_article_repository_error() {
        let err = ArticleRepositoryError::ArticleNotFound(article::Id::new(1).as_identifier());

        let err = DeleteArticleError::from(err);

        assert!(matches!(err, DeleteArticleError::NotFound(_)));

        let err = ArticleRepositoryError::DOConversion("test".to_string());

        let err = DeleteArticleError::from(err);

        assert!(matches!(err, DeleteArticleError::Repository(_)));
    }

    #[test]
    fn delete_series_error_from_series_repository_error() {
        let err = SeriesRepositoryError::SeriesNotFound(series::Id::new(1).as_identifier());

        let err = DeleteSeriesError::from(err);

        assert!(matches!(err, DeleteSeriesError::NotFound(_)));

        let err = SeriesRepositoryError::DOConversion("test".to_string());

        let err = DeleteSeriesError::from(err);

        assert!(matches!(err, DeleteSeriesError::Repository(_)));
    }

    #[test]
    fn delete_category_error_from_category_repository_error() {
        let err = CategoryRepositoryError::CategoryNotFound(category::Id::new(1).as_identifier());

        let err = DeleteCategoryError::from(err);

        assert!(matches!(err, DeleteCategoryError::NotFound(_)));

        let err = CategoryRepositoryError::DOConversion("test".to_string());

        let err = DeleteCategoryError::from(err);

        assert!(matches!(err, DeleteCategoryError::Repository(_)));
    }

    #[test]
    fn delete_tag_error_from_tag_repository_error() {
        let err = TagRepositoryError::TagNotFound(tag::Id::new(1).as_identifier());

        let err = DeleteTagError::from(err);

        assert!(matches!(err, DeleteTagError::NotFound(_)));

        let err = TagRepositoryError::DOConversion("test".to_string());

        let err = DeleteTagError::from(err);

        assert!(matches!(err, DeleteTagError::Repository(_)));
    }
}

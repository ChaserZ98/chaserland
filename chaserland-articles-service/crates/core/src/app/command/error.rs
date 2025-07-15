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

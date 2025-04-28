use super::entity::article;
use super::repository;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DomainError {
    #[error(transparent)]
    Article(#[from] article::DomainError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RepositoryError {
    #[error(transparent)]
    ArticleRepository(#[from] repository::article::ArticleRepositoryError),
    #[error(transparent)]
    SeriesRepository(#[from] repository::series::SeriesRepositoryError),
    #[error(transparent)]
    CategoryRepository(#[from] repository::category::CategoryRepositoryError),
    #[error(transparent)]
    TagRepository(#[from] repository::tag::TagRepositoryError),
}

impl RepositoryError {
    pub fn is_article_repository_error(&self) -> bool {
        matches!(self, RepositoryError::ArticleRepository(_))
    }

    pub fn is_series_repository_error(&self) -> bool {
        matches!(self, RepositoryError::SeriesRepository(_))
    }

    pub fn is_category_repository_error(&self) -> bool {
        matches!(self, RepositoryError::CategoryRepository(_))
    }

    pub fn is_tag_repository_error(&self) -> bool {
        matches!(self, RepositoryError::TagRepository(_))
    }

    pub fn as_article_repository_error(
        &self,
    ) -> Option<&repository::article::ArticleRepositoryError> {
        match self {
            RepositoryError::ArticleRepository(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_series_repository_error(&self) -> Option<&repository::series::SeriesRepositoryError> {
        match self {
            RepositoryError::SeriesRepository(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_category_repository_error(
        &self,
    ) -> Option<&repository::category::CategoryRepositoryError> {
        match self {
            RepositoryError::CategoryRepository(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_tag_repository_error(&self) -> Option<&repository::tag::TagRepositoryError> {
        match self {
            RepositoryError::TagRepository(e) => Some(e),
            _ => None,
        }
    }
}

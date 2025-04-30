use crate::domain::error::{DomainError, RepositoryError};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ArticleServiceError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("DO to DTO conversion error: {0}")]
    DTOConversion(String),
}

impl ArticleServiceError {
    pub fn is_domain_error(&self) -> bool {
        matches!(self, ArticleServiceError::Domain(_))
    }
    pub fn is_repository_error(&self) -> bool {
        matches!(self, ArticleServiceError::Repository(_))
    }
    pub fn as_domain_error(&self) -> Option<&DomainError> {
        match self {
            ArticleServiceError::Domain(e) => Some(e),
            _ => None,
        }
    }
    pub fn as_repository_error(&self) -> Option<&RepositoryError> {
        match self {
            ArticleServiceError::Repository(e) => Some(e),
            _ => None,
        }
    }
}

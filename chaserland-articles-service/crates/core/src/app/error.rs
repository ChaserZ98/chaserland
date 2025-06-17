use super::dto::DTOError;
use crate::domain::error::{DomainError, RepositoryError};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ArticleServiceError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("DO to DTO conversion error: {0}")]
    DTOConversion(#[from] DTOError),
}

impl ArticleServiceError {
    pub fn is_domain_error(&self) -> bool {
        matches!(self, ArticleServiceError::Domain(_))
    }
    pub fn is_repository_error(&self) -> bool {
        matches!(self, ArticleServiceError::Repository(_))
    }
    pub fn is_dto_conversion_error(&self) -> bool {
        matches!(self, ArticleServiceError::DTOConversion(_))
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
    pub fn as_dto_conversion_error(&self) -> Option<&DTOError> {
        match self {
            ArticleServiceError::DTOConversion(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArticleServiceError;
    use crate::app::dto::{ArticleDTOError, DTOError};
    use crate::domain::article::{error::ArticleDomainError, repository::ArticleRepositoryError};
    use crate::domain::error::{DomainError, RepositoryError};
    use anyhow::anyhow;

    #[test]
    fn article_service_error_case_domain_error() {
        let err = ArticleServiceError::Domain(DomainError::Article(ArticleDomainError::Unknown(
            anyhow!("test"),
        )));

        assert!(err.is_domain_error());

        let err = err.as_domain_error();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, DomainError::Article(_)));

        let err = ArticleServiceError::DTOConversion(DTOError::Article(
            crate::app::dto::ArticleDTOError::ArticleNotDefined,
        ));

        assert_eq!(err.is_domain_error(), false);

        let err = err.as_domain_error();

        assert!(err.is_none());
    }

    #[test]
    fn article_service_error_case_repository_error() {
        let err = ArticleServiceError::Repository(RepositoryError::ArticleRepository(
            ArticleRepositoryError::Unknown(anyhow!("test")),
        ));

        assert!(err.is_repository_error());

        let err = err.as_repository_error();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, RepositoryError::ArticleRepository(_)));

        let err = ArticleServiceError::Domain(DomainError::Article(ArticleDomainError::Unknown(
            anyhow!("test"),
        )));

        assert_eq!(err.is_repository_error(), false);

        let err = err.as_repository_error();

        assert!(err.is_none());
    }

    #[test]
    fn article_service_error_case_dto_conversion_error() {
        let article_dto_error = ArticleDTOError::ArticleNotDefined;
        let err = ArticleServiceError::DTOConversion(DTOError::Article(article_dto_error));

        assert!(err.is_dto_conversion_error());

        let err = err.as_dto_conversion_error();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(err.is_article_error());

        let err = err.as_article_error();

        assert!(err.is_some());

        let err = err.unwrap();

        assert!(matches!(err, ArticleDTOError::ArticleNotDefined));

        let err = ArticleServiceError::Repository(RepositoryError::ArticleRepository(
            ArticleRepositoryError::Unknown(anyhow!("test")),
        ));

        assert_eq!(err.is_dto_conversion_error(), false);

        let err = err.as_dto_conversion_error();

        assert!(err.is_none());
    }
}

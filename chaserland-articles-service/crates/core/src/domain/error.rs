use super::article::error::ArticleDomainError;
use super::article::repository::ArticleRepositoryError;
use super::category::repository::CategoryRepositoryError;
use super::series::repository::SeriesRepositoryError;
use super::tag::repository::TagRepositoryError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DomainError {
    #[error(transparent)]
    Article(#[from] ArticleDomainError),
}

impl DomainError {
    pub fn is_article_domain_error(&self) -> bool {
        matches!(self, DomainError::Article(_))
    }

    pub fn as_article_domain_error(&self) -> Option<&ArticleDomainError> {
        let DomainError::Article(e) = self;
        Some(e)
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RepositoryError {
    #[error(transparent)]
    ArticleRepository(#[from] ArticleRepositoryError),
    #[error(transparent)]
    SeriesRepository(#[from] SeriesRepositoryError),
    #[error(transparent)]
    CategoryRepository(#[from] CategoryRepositoryError),
    #[error(transparent)]
    TagRepository(#[from] TagRepositoryError),
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

    pub fn as_article_repository_error(&self) -> Option<&ArticleRepositoryError> {
        match self {
            RepositoryError::ArticleRepository(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_series_repository_error(&self) -> Option<&SeriesRepositoryError> {
        match self {
            RepositoryError::SeriesRepository(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_category_repository_error(&self) -> Option<&CategoryRepositoryError> {
        match self {
            RepositoryError::CategoryRepository(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_tag_repository_error(&self) -> Option<&TagRepositoryError> {
        match self {
            RepositoryError::TagRepository(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DomainError;
    use super::RepositoryError;
    use crate::domain::article::repository::ArticleRepositoryError;
    use crate::domain::category::repository::CategoryRepositoryError;
    use crate::domain::error::ArticleDomainError;
    use crate::domain::series::repository::SeriesRepositoryError;
    use crate::domain::tag::repository::TagRepositoryError;

    #[test]
    fn domain_err_case_article_domain_error() {
        let err = DomainError::Article(ArticleDomainError::Unknown(anyhow::anyhow!("test")));

        assert!(err.is_article_domain_error());

        let err = err.as_article_domain_error().unwrap();

        assert!(matches!(err, ArticleDomainError::Unknown(_)));
    }

    #[test]
    fn repository_err_case_article_repository_error() {
        let err =
            RepositoryError::ArticleRepository(ArticleRepositoryError::DOConversion("test".into()));

        assert!(err.is_article_repository_error());

        let err = err.as_article_repository_error().unwrap();

        assert!(matches!(err, ArticleRepositoryError::DOConversion(_)));

        let err = RepositoryError::TagRepository(TagRepositoryError::DOConversion("test".into()));

        assert_eq!(err.is_article_repository_error(), false);

        let err = err.as_article_repository_error();

        assert!(err.is_none());
    }

    #[test]
    fn repository_err_case_series_repository_error() {
        let err =
            RepositoryError::SeriesRepository(SeriesRepositoryError::DOConversion("test".into()));

        assert!(err.is_series_repository_error());

        let err = err.as_series_repository_error().unwrap();

        assert!(matches!(err, SeriesRepositoryError::DOConversion(_)));

        let err =
            RepositoryError::ArticleRepository(ArticleRepositoryError::DOConversion("test".into()));

        assert_eq!(err.is_series_repository_error(), false);

        let err = err.as_series_repository_error();

        assert!(err.is_none());
    }

    #[test]
    fn repository_err_case_category_repository_error() {
        let err = RepositoryError::CategoryRepository(CategoryRepositoryError::DOConversion(
            "test".into(),
        ));

        assert!(err.is_category_repository_error());

        let err = err.as_category_repository_error().unwrap();

        assert!(matches!(err, CategoryRepositoryError::DOConversion(_)));

        let err =
            RepositoryError::SeriesRepository(SeriesRepositoryError::DOConversion("test".into()));

        assert_eq!(err.is_category_repository_error(), false);

        let err = err.as_category_repository_error();

        assert!(err.is_none());
    }

    #[test]
    fn repository_err_case_tag_repository_error() {
        let err = RepositoryError::TagRepository(TagRepositoryError::DOConversion("test".into()));

        assert!(err.is_tag_repository_error());

        let err = err.as_tag_repository_error().unwrap();

        assert!(matches!(err, TagRepositoryError::DOConversion(_)));

        let err = RepositoryError::CategoryRepository(CategoryRepositoryError::DOConversion(
            "test".into(),
        ));

        assert_eq!(err.is_tag_repository_error(), false);

        let err = err.as_tag_repository_error();

        assert!(err.is_none());
    }
}

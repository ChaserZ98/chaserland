use super::entity::article;
use super::repository;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DomainError {
    #[error(transparent)]
    Article(#[from] article::DomainError),
}

impl DomainError {
    pub fn is_article_domain_error(&self) -> bool {
        matches!(self, DomainError::Article(_))
    }

    pub fn as_article_domain_error(&self) -> Option<&article::DomainError> {
        let DomainError::Article(e) = self;
        Some(e)
    }
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

#[cfg(test)]
mod tests {
    use super::DomainError;
    use super::RepositoryError;
    use crate::domain::entity::article;
    use crate::domain::repository;

    #[test]
    fn domain_err_case_article_domain_error() {
        let err = DomainError::Article(article::DomainError::Unknown(anyhow::anyhow!("test")));

        assert!(err.is_article_domain_error());

        let err = err.as_article_domain_error().unwrap();

        assert!(matches!(err, article::DomainError::Unknown(_)));
    }

    #[test]
    fn repository_err_case_article_repository_error() {
        let err = RepositoryError::ArticleRepository(
            repository::article::ArticleRepositoryError::Unknown(anyhow::anyhow!("test")),
        );

        assert!(err.is_article_repository_error());

        let err = err.as_article_repository_error().unwrap();

        assert!(matches!(
            err,
            repository::article::ArticleRepositoryError::Unknown(_)
        ));

        let err = RepositoryError::TagRepository(repository::tag::TagRepositoryError::Unknown(
            anyhow::anyhow!("test"),
        ));

        assert_eq!(err.is_article_repository_error(), false);

        let err = err.as_article_repository_error();

        assert!(err.is_none());
    }

    #[test]
    fn repository_err_case_series_repository_error() {
        let err = RepositoryError::SeriesRepository(
            repository::series::SeriesRepositoryError::Unknown(anyhow::anyhow!("test")),
        );

        assert!(err.is_series_repository_error());

        let err = err.as_series_repository_error().unwrap();

        assert!(matches!(
            err,
            repository::series::SeriesRepositoryError::Unknown(_)
        ));

        let err = RepositoryError::ArticleRepository(
            repository::article::ArticleRepositoryError::Unknown(anyhow::anyhow!("test")),
        );

        assert_eq!(err.is_series_repository_error(), false);

        let err = err.as_series_repository_error();

        assert!(err.is_none());
    }

    #[test]
    fn repository_err_case_category_repository_error() {
        let err = RepositoryError::CategoryRepository(
            repository::category::CategoryRepositoryError::Unknown(anyhow::anyhow!("test")),
        );

        assert!(err.is_category_repository_error());

        let err = err.as_category_repository_error().unwrap();

        assert!(matches!(
            err,
            repository::category::CategoryRepositoryError::Unknown(_)
        ));

        let err = RepositoryError::SeriesRepository(
            repository::series::SeriesRepositoryError::Unknown(anyhow::anyhow!("test")),
        );

        assert_eq!(err.is_category_repository_error(), false);

        let err = err.as_category_repository_error();

        assert!(err.is_none());
    }

    #[test]
    fn repository_err_case_tag_repository_error() {
        let err = RepositoryError::TagRepository(repository::tag::TagRepositoryError::Unknown(
            anyhow::anyhow!("test"),
        ));

        assert!(err.is_tag_repository_error());

        let err = err.as_tag_repository_error().unwrap();

        assert!(matches!(
            err,
            repository::tag::TagRepositoryError::Unknown(_)
        ));

        let err = RepositoryError::CategoryRepository(
            repository::category::CategoryRepositoryError::Unknown(anyhow::anyhow!("test")),
        );

        assert_eq!(err.is_tag_repository_error(), false);

        let err = err.as_tag_repository_error();

        assert!(err.is_none());
    }
}

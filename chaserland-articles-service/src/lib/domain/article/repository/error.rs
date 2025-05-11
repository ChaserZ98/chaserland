use crate::domain::article::vo as article;
use crate::domain::category::vo as category;
use crate::domain::series::vo as series;
use crate::domain::tag::vo as tag;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ArticleRepositoryError {
    #[error("Article with identifier {0} not found")]
    ArticleNotFound(article::Identifier),
    #[error("Article with slug {0} already exists")]
    DuplicateArticleSlug(article::Slug),
    #[error("Series with identifier {0} not found")]
    SeriesNotFound(series::Identifier),
    #[error("Category with identifier {0} not found")]
    CategoryNotFound(category::Identifier),
    #[error("Tag with identifier {0} not found")]
    TagNotFound(tag::Identifier),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl ArticleRepositoryError {
    pub fn is_article_not_found(&self) -> bool {
        matches!(self, ArticleRepositoryError::ArticleNotFound(_))
    }

    pub fn is_duplicate_article_slug(&self) -> bool {
        matches!(self, ArticleRepositoryError::DuplicateArticleSlug(_))
    }

    pub fn is_series_not_found(&self) -> bool {
        matches!(self, ArticleRepositoryError::SeriesNotFound(_))
    }

    pub fn is_category_not_found(&self) -> bool {
        matches!(self, ArticleRepositoryError::CategoryNotFound(_))
    }

    pub fn is_tag_not_found(&self) -> bool {
        matches!(self, ArticleRepositoryError::TagNotFound(_))
    }

    pub fn is_version_mismatch(&self) -> bool {
        matches!(self, ArticleRepositoryError::VersionMismatch { .. })
    }

    pub fn is_transaction_error(&self) -> bool {
        matches!(self, ArticleRepositoryError::Transaction(_))
    }

    pub fn is_do_conversion_error(&self) -> bool {
        matches!(self, ArticleRepositoryError::DOConversion(_))
    }

    pub fn as_article_not_found(&self) -> Option<&article::Identifier> {
        match self {
            ArticleRepositoryError::ArticleNotFound(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_duplicate_article_slug(&self) -> Option<&article::Slug> {
        match self {
            ArticleRepositoryError::DuplicateArticleSlug(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_series_not_found(&self) -> Option<&series::Identifier> {
        match self {
            ArticleRepositoryError::SeriesNotFound(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_category_not_found(&self) -> Option<&category::Identifier> {
        match self {
            ArticleRepositoryError::CategoryNotFound(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_tag_not_found(&self) -> Option<&tag::Identifier> {
        match self {
            ArticleRepositoryError::TagNotFound(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_version_mismatch(
        &self,
    ) -> Option<(&article::Id, &article::Version, &article::Version)> {
        match self {
            ArticleRepositoryError::VersionMismatch {
                id,
                current_version,
                db_version,
            } => Some((id, current_version, db_version)),
            _ => None,
        }
    }

    pub fn as_transaction_error(&self) -> Option<&String> {
        match self {
            ArticleRepositoryError::Transaction(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_do_conversion_error(&self) -> Option<&String> {
        match self {
            ArticleRepositoryError::DOConversion(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArticleRepositoryError;
    use crate::domain::article::vo as article;
    use crate::domain::category::vo as category;
    use crate::domain::series::vo as series;
    use crate::domain::tag::vo as tag;

    #[test]
    fn article_repository_error_article_not_found() {
        let article_id = article::Id::new(1);

        let err = ArticleRepositoryError::ArticleNotFound(article_id.as_identifier());

        assert!(err.is_article_not_found());

        let err = err.as_article_not_found();

        assert!(err.is_some());

        let err = err.unwrap();

        assert_eq!(err, &article_id.as_identifier());

        let err = ArticleRepositoryError::Unknown(anyhow::anyhow!("test"));

        assert_eq!(err.is_article_not_found(), false);

        let err = err.as_article_not_found();

        assert!(err.is_none());
    }

    #[test]
    fn article_repository_error_duplicate_article_slug() {
        let slug: article::Slug = "test".try_into().unwrap();
        let err = ArticleRepositoryError::DuplicateArticleSlug(slug.clone());

        assert!(err.is_duplicate_article_slug());

        let err = err.as_duplicate_article_slug();

        assert!(err.is_some());

        let err = err.unwrap();

        assert_eq!(err, &slug);

        let err = ArticleRepositoryError::Unknown(anyhow::anyhow!("test"));

        assert_eq!(err.is_duplicate_article_slug(), false);

        let err = err.as_duplicate_article_slug();

        assert!(err.is_none());
    }

    #[test]
    fn article_repository_error_series_not_found() {
        let series_id = series::Id::new(1);

        let err = ArticleRepositoryError::SeriesNotFound(series_id.as_identifier());

        assert!(err.is_series_not_found());

        let err = err.as_series_not_found();

        assert!(err.is_some());

        let err = err.unwrap();

        assert_eq!(err, &series_id.as_identifier());

        let err = ArticleRepositoryError::Unknown(anyhow::anyhow!("test"));

        assert_eq!(err.is_series_not_found(), false);

        let err = err.as_series_not_found();

        assert!(err.is_none());
    }

    #[test]
    fn article_repository_error_category_not_found() {
        let category_id = category::Id::new(1);

        let err = ArticleRepositoryError::CategoryNotFound(category_id.as_identifier());

        assert!(err.is_category_not_found());

        let err = err.as_category_not_found();

        assert!(err.is_some());

        let err = err.unwrap();

        assert_eq!(err, &category_id.as_identifier());

        let err = ArticleRepositoryError::Unknown(anyhow::anyhow!("test"));

        assert_eq!(err.is_category_not_found(), false);

        let err = err.as_category_not_found();

        assert!(err.is_none());
    }

    #[test]
    fn article_repository_error_tag_not_found() {
        let tag_id = tag::Id::new(1);

        let err = ArticleRepositoryError::TagNotFound(tag_id.as_identifier());

        assert!(err.is_tag_not_found());

        let err = err.as_tag_not_found();

        assert!(err.is_some());

        let err = err.unwrap();

        assert_eq!(err, &tag_id.as_identifier());

        let err = ArticleRepositoryError::Unknown(anyhow::anyhow!("test"));

        assert_eq!(err.is_tag_not_found(), false);

        let err = err.as_tag_not_found();

        assert!(err.is_none());
    }

    #[test]
    fn article_repository_error_version_mismatch() {
        let article_id = article::Id::new(1);
        let current_version = article::Version::new("2020-01-01 00:00:00 UTC".parse().unwrap());
        let db_version = article::Version::new("2021-01-01 00:00:00 UTC".parse().unwrap());

        let err = ArticleRepositoryError::VersionMismatch {
            id: article_id.clone(),
            current_version: current_version.clone(),
            db_version: db_version.clone(),
        };

        assert!(err.is_version_mismatch());

        let err = err.as_version_mismatch();

        assert!(err.is_some());

        let err = err.unwrap();

        assert_eq!(err, (&article_id, &current_version, &db_version));

        let err = ArticleRepositoryError::Unknown(anyhow::anyhow!("test"));

        assert_eq!(err.is_version_mismatch(), false);

        let err = err.as_version_mismatch();

        assert!(err.is_none());
    }

    #[test]
    fn article_repository_error_transaction() {
        let err = ArticleRepositoryError::Transaction("test".to_string());

        assert!(err.is_transaction_error());

        let err = err.as_transaction_error();

        assert!(err.is_some());

        let err = err.unwrap();

        assert_eq!(err, "test");

        let err = ArticleRepositoryError::Unknown(anyhow::anyhow!("test"));

        assert_eq!(err.is_transaction_error(), false);

        let err = err.as_transaction_error();

        assert!(err.is_none());
    }

    #[test]
    fn article_repository_error_do_conversion() {
        let err = ArticleRepositoryError::DOConversion("test".to_string());

        assert!(err.is_do_conversion_error());

        let err = err.as_do_conversion_error();

        assert!(err.is_some());

        let err = err.unwrap();

        assert_eq!(err, "test");

        let err = ArticleRepositoryError::Unknown(anyhow::anyhow!("test"));

        assert_eq!(err.is_do_conversion_error(), false);

        let err = err.as_do_conversion_error();

        assert!(err.is_none());
    }
}

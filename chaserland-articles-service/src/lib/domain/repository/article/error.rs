use crate::domain::entity::{article, category, series, tag};

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

    pub fn article_not_found(&self) -> Option<&article::Identifier> {
        match self {
            ArticleRepositoryError::ArticleNotFound(e) => Some(e),
            _ => None,
        }
    }

    pub fn duplicate_article_slug(&self) -> Option<&article::Slug> {
        match self {
            ArticleRepositoryError::DuplicateArticleSlug(e) => Some(e),
            _ => None,
        }
    }

    pub fn series_not_found(&self) -> Option<&series::Identifier> {
        match self {
            ArticleRepositoryError::SeriesNotFound(e) => Some(e),
            _ => None,
        }
    }

    pub fn category_not_found(&self) -> Option<&category::Identifier> {
        match self {
            ArticleRepositoryError::CategoryNotFound(e) => Some(e),
            _ => None,
        }
    }

    pub fn tag_not_found(&self) -> Option<&tag::Identifier> {
        match self {
            ArticleRepositoryError::TagNotFound(e) => Some(e),
            _ => None,
        }
    }

    pub fn version_mismatch(&self) -> Option<(&article::Id, &article::Version, &article::Version)> {
        match self {
            ArticleRepositoryError::VersionMismatch {
                id,
                current_version,
                db_version,
            } => Some((id, current_version, db_version)),
            _ => None,
        }
    }

    pub fn transaction_error(&self) -> Option<&String> {
        match self {
            ArticleRepositoryError::Transaction(e) => Some(e),
            _ => None,
        }
    }

    pub fn do_conversion_error(&self) -> Option<&String> {
        match self {
            ArticleRepositoryError::DOConversion(e) => Some(e),
            _ => None,
        }
    }
}

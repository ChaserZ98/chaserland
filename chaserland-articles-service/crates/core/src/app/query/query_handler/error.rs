use crate::domain::{
    article::vo as article, category::vo as category, series::vo as series, tag::vo as tag,
};

#[derive(Debug, thiserror::Error)]
pub enum QueryHandlerError {
    #[error(transparent)]
    Article(#[from] ArticleQueryHandlerError),
    #[error(transparent)]
    Series(#[from] SeriesQueryHandlerError),
    #[error(transparent)]
    Category(#[from] CategoryQueryHandlerError),
    #[error(transparent)]
    Tag(#[from] TagQueryHandlerError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ArticleQueryHandlerError {
    #[error("Article with identifier {0} not found")]
    ArticleNotFound(article::Identifier),
    // #[error("Series with identifier {0} not found")]
    // SeriesNotFound(series::Identifier),
    // #[error("Category with identifier {0} not found")]
    // CategoryNotFound(category::Identifier),
    // #[error("Tag with identifier {0} not found")]
    // TagNotFound(tag::Identifier),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SeriesQueryHandlerError {
    #[error("Series with identifier {0} not found")]
    SeriesNotFound(series::Identifier),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CategoryQueryHandlerError {
    #[error("Category with identifier {0} not found")]
    CategoryNotFound(category::Identifier),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TagQueryHandlerError {
    #[error("Tag with identifier {0} not found")]
    TagNotFound(tag::Identifier),
    #[error("Transaction error: {0}")]
    Transaction(String),
    #[error("PO to DO conversion error: {0}")]
    DOConversion(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

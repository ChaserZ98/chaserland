use crate::{
    app::query::query_handler::error::{
        ArticleQueryHandlerError, CategoryQueryHandlerError, QueryHandlerError,
        SeriesQueryHandlerError, TagQueryHandlerError,
    },
    domain::{
        article::vo as article, category::vo as category, series::vo as series, tag::vo as tag,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum GetArticleOneError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    // #[error("Version conflict: {0}")]
    // DataVersionConflict(String),
    // #[error("DO to DTO conversion error: {0}")]
    // DTOConversion(String),
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<ArticleQueryHandlerError> for GetArticleOneError {
    fn from(value: ArticleQueryHandlerError) -> Self {
        match value {
            ArticleQueryHandlerError::ArticleNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::QueryHandler(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetArticleContentError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<ArticleQueryHandlerError> for GetArticleContentError {
    fn from(value: ArticleQueryHandlerError) -> Self {
        match value {
            ArticleQueryHandlerError::ArticleNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::QueryHandler(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetArticleManyError {
    #[error("DO to DTO conversion error: {0}")]
    DTOConversion(String),
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<ArticleQueryHandlerError> for GetArticleManyError {
    fn from(value: ArticleQueryHandlerError) -> Self {
        Self::QueryHandler(value.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetSeriesOneError {
    #[error("Series with identifier {0} not found")]
    NotFound(series::Identifier),
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<SeriesQueryHandlerError> for GetSeriesOneError {
    fn from(value: SeriesQueryHandlerError) -> Self {
        match value {
            SeriesQueryHandlerError::SeriesNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::QueryHandler(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetSeriesManyError {
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<SeriesQueryHandlerError> for GetSeriesManyError {
    fn from(value: SeriesQueryHandlerError) -> Self {
        Self::QueryHandler(value.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetCategoryOneError {
    #[error("Category with identifier {0} not found")]
    NotFound(category::Identifier),
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<CategoryQueryHandlerError> for GetCategoryOneError {
    fn from(value: CategoryQueryHandlerError) -> Self {
        match value {
            CategoryQueryHandlerError::CategoryNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::QueryHandler(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetCategoryManyError {
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<CategoryQueryHandlerError> for GetCategoryManyError {
    fn from(value: CategoryQueryHandlerError) -> Self {
        Self::QueryHandler(value.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetTagOneError {
    #[error("Tag with identifier {0} not found")]
    NotFound(tag::Identifier),
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<TagQueryHandlerError> for GetTagOneError {
    fn from(value: TagQueryHandlerError) -> Self {
        match value {
            TagQueryHandlerError::TagNotFound(identifier) => Self::NotFound(identifier),
            _ => Self::QueryHandler(value.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetTagManyError {
    #[error(transparent)]
    QueryHandler(#[from] QueryHandlerError),
}

impl From<TagQueryHandlerError> for GetTagManyError {
    fn from(value: TagQueryHandlerError) -> Self {
        Self::QueryHandler(value.into())
    }
}

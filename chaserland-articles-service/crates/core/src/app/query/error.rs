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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_article_one_error_from_article_query_handler_error() {
        let error = ArticleQueryHandlerError::ArticleNotFound(article::Identifier::Id(
            1.try_into().unwrap(),
        ));

        let res = GetArticleOneError::from(error);

        assert!(match res {
            GetArticleOneError::NotFound(identifier) =>
                identifier == article::Identifier::Id(1.try_into().unwrap()),
            _ => false,
        });

        let error = ArticleQueryHandlerError::DOConversion("error".into());

        let res = GetArticleOneError::from(error);

        assert!(matches!(res, GetArticleOneError::QueryHandler(_)));
    }

    #[test]
    fn get_article_content_error_from_article_query_handler_error() {
        let identifier = article::Identifier::Id(1.try_into().unwrap());
        let error = ArticleQueryHandlerError::ArticleNotFound(identifier.clone());

        let res = GetArticleContentError::from(error);

        assert!(match res {
            GetArticleContentError::NotFound(identifier) => identifier == identifier,
            _ => false,
        });

        let error = ArticleQueryHandlerError::DOConversion("error".into());

        let res = GetArticleContentError::from(error);

        assert!(matches!(res, GetArticleContentError::QueryHandler(_)));
    }

    #[test]
    fn get_article_many_error_from_article_query_handler_error() {
        let error = ArticleQueryHandlerError::DOConversion("error".into());

        let res = GetArticleManyError::from(error);

        assert!(matches!(res, GetArticleManyError::QueryHandler(_)));
    }

    #[test]
    fn get_series_one_error_from_series_query_handler_error() {
        let identifier = series::Identifier::Id(1.try_into().unwrap());
        let error = SeriesQueryHandlerError::SeriesNotFound(identifier.clone());

        let res = GetSeriesOneError::from(error);

        assert!(match res {
            GetSeriesOneError::NotFound(identifier) => identifier == identifier,
            _ => false,
        });

        let error = SeriesQueryHandlerError::DOConversion("error".into());

        let res = GetSeriesOneError::from(error);

        assert!(matches!(res, GetSeriesOneError::QueryHandler(_)));
    }

    #[test]
    fn get_series_many_error_from_series_query_handler_error() {
        let error = SeriesQueryHandlerError::DOConversion("error".into());

        let res = GetSeriesManyError::from(error);

        assert!(matches!(res, GetSeriesManyError::QueryHandler(_)));
    }

    #[test]
    fn get_category_one_error_from_category_query_handler_error() {
        let identifier = category::Identifier::Id(1.try_into().unwrap());
        let error = CategoryQueryHandlerError::CategoryNotFound(identifier.clone());

        let res = GetCategoryOneError::from(error);

        assert!(match res {
            GetCategoryOneError::NotFound(identifier) => identifier == identifier,
            _ => false,
        });

        let error = CategoryQueryHandlerError::DOConversion("error".into());

        let res = GetCategoryOneError::from(error);

        assert!(matches!(res, GetCategoryOneError::QueryHandler(_)));
    }

    #[test]
    fn get_category_many_error_from_category_query_handler_error() {
        let error = CategoryQueryHandlerError::DOConversion("error".into());

        let res = GetCategoryManyError::from(error);

        assert!(matches!(res, GetCategoryManyError::QueryHandler(_)));
    }

    #[test]
    fn get_tag_one_error_from_tag_query_handler_error() {
        let identifier = tag::Identifier::Id(1.try_into().unwrap());
        let error = TagQueryHandlerError::TagNotFound(identifier.clone());

        let res = GetTagOneError::from(error);

        assert!(match res {
            GetTagOneError::NotFound(identifier) => identifier == identifier,
            _ => false,
        });

        let error = TagQueryHandlerError::DOConversion("error".into());

        let res = GetTagOneError::from(error);

        assert!(matches!(res, GetTagOneError::QueryHandler(_)));
    }

    #[test]
    fn get_tag_many_error_from_tag_query_handler_error() {
        let error = TagQueryHandlerError::DOConversion("error".into());

        let res = GetTagManyError::from(error);

        assert!(matches!(res, GetTagManyError::QueryHandler(_)));
    }
}

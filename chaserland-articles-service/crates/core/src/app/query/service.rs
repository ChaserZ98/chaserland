use super::{dto, error, interface::ArticleQueryService as ArticleQueryServiceInterface, query};
use crate::app::query::query_handler::interface::{
    ArticleQueryHandler, CategoryQueryHandler, SeriesQueryHandler, TagQueryHandler,
};

#[derive(Clone)]
pub struct ArticleQueryService<A, S, C, T>
where
    A: ArticleQueryHandler,
    S: SeriesQueryHandler,
    C: CategoryQueryHandler,
    T: TagQueryHandler,
{
    article_query_handler: A,
    series_query_handler: S,
    category_query_handler: C,
    tag_query_handler: T,
}

impl<A, S, C, T> ArticleQueryService<A, S, C, T>
where
    A: ArticleQueryHandler,
    S: SeriesQueryHandler,
    C: CategoryQueryHandler,
    T: TagQueryHandler,
{
    pub fn new(
        article_query_handler: A,
        series_query_handler: S,
        category_query_handler: C,
        tag_query_handler: T,
    ) -> Self {
        Self {
            article_query_handler,
            series_query_handler,
            category_query_handler,
            tag_query_handler,
        }
    }
}

impl<A, S, C, T> ArticleQueryServiceInterface for ArticleQueryService<A, S, C, T>
where
    A: ArticleQueryHandler,
    S: SeriesQueryHandler,
    C: CategoryQueryHandler,
    T: TagQueryHandler,
{
    async fn get_article_one(
        &self,
        query: query::GetArticleOneQuery,
    ) -> Result<dto::ArticleDTO, error::GetArticleOneError> {
        let article = self
            .article_query_handler
            .get_one(query.identifier, query.public_only, query.with_content)
            .await?;
        Ok(article)
    }

    async fn get_article_content(
        &self,
        query: query::GetArticleContentQuery,
    ) -> Result<String, error::GetArticleContentError> {
        let article = self
            .article_query_handler
            .get_one(query.identifier, query.public_only, true)
            .await?;

        let content = match article.content {
            None => String::from(""),
            Some(content) => content,
        };
        Ok(content)
    }

    async fn get_article_many(
        &self,
        query: query::GetArticleManyQuery,
    ) -> Result<Vec<dto::ArticleDTO>, error::GetArticleManyError> {
        let articles = self
            .article_query_handler
            .get_many(
                query.pagination,
                query.public_only,
                query.with_content,
                query.filter,
            )
            .await?;

        Ok(articles)
    }

    async fn get_series_one(
        &self,
        query: query::GetSeriesOneQuery,
    ) -> Result<dto::SeriesDTO, error::GetSeriesOneError> {
        let series = self.series_query_handler.get_one(query.identifier).await?;

        Ok(series)
    }

    async fn get_series_many(
        &self,
        query: query::GetSeriesManyQuery,
    ) -> Result<Vec<dto::SeriesDTO>, error::GetSeriesManyError> {
        let series = self
            .series_query_handler
            .get_many(query.filter, query.pagination)
            .await?;

        Ok(series)
    }

    async fn get_category_one(
        &self,
        query: query::GetCategoryOneQuery,
    ) -> Result<dto::CategoryDTO, error::GetCategoryOneError> {
        let category = self
            .category_query_handler
            .get_one(query.identifier)
            .await?;

        Ok(category)
    }

    async fn get_category_many(
        &self,
        query: query::GetCategoryManyQuery,
    ) -> Result<Vec<dto::CategoryDTO>, error::GetCategoryManyError> {
        let categories = self
            .category_query_handler
            .get_many(query.filter, query.pagination)
            .await?;

        Ok(categories)
    }

    async fn get_tag_one(
        &self,
        query: query::GetTagOneQuery,
    ) -> Result<dto::TagDTO, error::GetTagOneError> {
        let tag = self.tag_query_handler.get_one(query.identifier).await?;

        Ok(tag)
    }

    async fn get_tag_many(
        &self,
        query: query::GetTagManyQuery,
    ) -> Result<Vec<dto::TagDTO>, error::GetTagManyError> {
        let tags = self
            .tag_query_handler
            .get_many(query.filter, query.pagination)
            .await?;

        Ok(tags)
    }
}

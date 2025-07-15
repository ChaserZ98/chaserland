use super::{dto, error, query};

#[trait_variant::make(ArticleQueryService: Send)]
pub trait LocalArticleQueryService: Clone + Sync + 'static {
    async fn get_article_one(
        &self,
        query: query::GetArticleOneQuery,
    ) -> Result<dto::ArticleDTO, error::GetArticleOneError>;

    async fn get_article_content(
        &self,
        query: query::GetArticleContentQuery,
    ) -> Result<String, error::GetArticleContentError>;

    async fn get_article_many(
        &self,
        query: query::GetArticleManyQuery,
    ) -> Result<Vec<dto::ArticleDTO>, error::GetArticleManyError>;

    async fn get_series_one(
        &self,
        query: query::GetSeriesOneQuery,
    ) -> Result<dto::SeriesDTO, error::GetSeriesOneError>;

    async fn get_series_many(
        &self,
        query: query::GetSeriesManyQuery,
    ) -> Result<Vec<dto::SeriesDTO>, error::GetSeriesManyError>;

    async fn get_category_one(
        &self,
        query: query::GetCategoryOneQuery,
    ) -> Result<dto::CategoryDTO, error::GetCategoryOneError>;

    async fn get_category_many(
        &self,
        query: query::GetCategoryManyQuery,
    ) -> Result<Vec<dto::CategoryDTO>, error::GetCategoryManyError>;

    async fn get_tag_one(
        &self,
        query: query::GetTagOneQuery,
    ) -> Result<dto::TagDTO, error::GetTagOneError>;

    async fn get_tag_many(
        &self,
        query: query::GetTagManyQuery,
    ) -> Result<Vec<dto::TagDTO>, error::GetTagManyError>;
}

use crate::app::{command, dto, error, query};

#[trait_variant::make(ArticleService: Send)]
pub trait LocalArticleService: Clone + Sync + 'static {
    async fn create_article(
        &self,
        command: command::CreateArticleCommand,
    ) -> Result<dto::ArticleDTO, error::CreateArticleError>;

    async fn create_series(
        &self,
        command: command::CreateSeriesCommand,
    ) -> Result<dto::SeriesDTO, error::CreateSeriesError>;

    async fn create_category(
        &self,
        command: command::CreateCategoryCommand,
    ) -> Result<dto::CategoryDTO, error::CreateCategoryError>;

    async fn create_tag(
        &self,
        command: command::CreateTagCommand,
    ) -> Result<dto::TagDTO, error::CreateTagError>;

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

    async fn publish_article(
        &self,
        command: command::PublishArticleCommand,
    ) -> Result<(), error::PublishArticleError>;

    async fn unpublish_article(
        &self,
        command: command::UnpublishArticleCommand,
    ) -> Result<(), error::UnpublishArticleError>;

    async fn soft_delete_article(
        &self,
        command: command::SoftDeleteArticleCommand,
    ) -> Result<(), error::SoftDeleteArticleError>;

    async fn revoke_soft_delete_article(
        &self,
        command: command::RevokeSoftDeleteArticleCommand,
    ) -> Result<(), error::RevokeSoftDeleteError>;

    async fn delete_article(
        &self,
        command: command::DeleteArticleCommand,
    ) -> Result<(), error::DeleteArticleError>;

    async fn delete_series(
        &self,
        command: command::DeleteSeriesCommand,
    ) -> Result<(), error::DeleteSeriesError>;

    async fn delete_category(
        &self,
        command: command::DeleteCategoryCommand,
    ) -> Result<(), error::DeleteCategoryError>;

    async fn delete_tag(
        &self,
        command: command::DeleteTagCommand,
    ) -> Result<(), error::DeleteTagError>;
}

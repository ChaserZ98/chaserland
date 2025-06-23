use super::{ArticleRepositoryError, ArticlesFilter};
use crate::domain::article::{entity::Article, vo as article};
use crate::domain::category::vo as category;
use crate::domain::series::vo as series;
use crate::domain::tag::vo as tag;
use chaserland_common::pagination::Pagination;

#[trait_variant::make(ArticleRepository: Send)]
pub trait LocalArticleRepository: Clone + Sync + 'static {
    async fn create(&self, article: article::NewArticle)
    -> Result<Article, ArticleRepositoryError>;

    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<Article, ArticleRepositoryError>;

    async fn get_many(
        &self,
        pagination: Pagination,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<Article>, ArticleRepositoryError>;

    async fn set_series(
        &self,
        id: article::Id,
        series_id: series::Id,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn remove_series(
        &self,
        id: article::Id,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn add_category(
        &self,
        article_id: article::Id,
        category_id: category::Id,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn remove_category(
        &self,
        article_id: article::Id,
        category_id: category::Id,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn add_tag(
        &self,
        article_id: article::Id,
        tag_id: tag::Id,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn remove_tag(
        &self,
        article_id: article::Id,
        tag_id: tag::Id,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn publish(
        &self,
        id: article::Id,
        published_at: article::PublishedAt,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn unpublish(
        &self,
        id: article::Id,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn soft_delete(
        &self,
        id: article::Id,
        deleted_at: article::DeletedAt,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;

    async fn revoke_soft_delete(
        &self,
        id: article::Id,
        version: article::Version,
    ) -> Result<(), ArticleRepositoryError>;
    async fn delete(&self, identifier: article::Identifier) -> Result<(), ArticleRepositoryError>;
}

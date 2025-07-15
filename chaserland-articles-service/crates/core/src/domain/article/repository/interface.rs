use super::ArticleRepositoryError;
use crate::domain::article::vo::TimestampVersion;
use crate::domain::article::{entity::Article, vo as article};
use crate::domain::category::vo as category;
use crate::domain::series::vo as series;
use crate::domain::tag::vo as tag;
use sqlx::{Database, Transaction};

#[trait_variant::make(ArticleRepository: Send)]
pub trait LocalArticleRepository: Clone + Sync + 'static {
    type DB: Database;
    async fn create(
        &self,
        article: article::NewArticle,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Article, ArticleRepositoryError>;

    async fn get_one(
        &self,
        identifier: article::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(Article, TimestampVersion), ArticleRepositoryError>;

    async fn save(
        &self,
        article: Article,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn set_series(
        &self,
        id: article::Id,
        series_id: series::Id,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn remove_series(
        &self,
        id: article::Id,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn add_category(
        &self,
        article_id: article::Id,
        category_id: category::Id,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn remove_category(
        &self,
        article_id: article::Id,
        category_id: category::Id,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn add_tag(
        &self,
        article_id: article::Id,
        tag_id: tag::Id,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn remove_tag(
        &self,
        article_id: article::Id,
        tag_id: tag::Id,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn publish(
        &self,
        id: article::Id,
        published_at: article::PublishedAt,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn unpublish(
        &self,
        id: article::Id,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn soft_delete(
        &self,
        id: article::Id,
        deleted_at: article::DeletedAt,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;

    async fn revoke_soft_delete(
        &self,
        id: article::Id,
        version: article::TimestampVersion,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;
    async fn delete(
        &self,
        identifier: article::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;
}

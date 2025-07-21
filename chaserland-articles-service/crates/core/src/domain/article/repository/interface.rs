use super::ArticleRepositoryError;
use crate::domain::{
    article::{
        entity::Article,
        vo::{self as article, TimestampVersion},
    },
    category::vo as category,
    tag::vo as tag,
};
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

    async fn delete(
        &self,
        identifier: article::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), ArticleRepositoryError>;
}

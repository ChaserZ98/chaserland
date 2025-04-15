use super::model::{article, category, tag};
use async_trait::async_trait;

#[async_trait]
pub trait ArticleRepository {
    type Transaction;

    async fn create(
        &self,
        article: article::ArticleCreate,
        transaction: &mut Self::Transaction,
    ) -> Result<article::Article, article::CreateArticleError>;
    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<article::Article, article::GetArticleError>;
    async fn get_many(
        &self,
        page: i32,
        page_size: i32,
        public_only: bool,
        with_content: bool,
        filter: article::ArticlesFilter,
    ) -> Result<Vec<article::Article>, article::GetArticleError>;
    async fn delete_by_id(
        &self,
        id: article::ArticleId,
        transaction: &mut Self::Transaction,
    ) -> Result<(), article::DeleteArticleError>;
}

#[async_trait]
pub trait CategoryRepository {
    type Transaction;

    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<category::Category, category::GetCategoryError>;
}

#[async_trait]
pub trait TagRepository {
    async fn get_one(&self, identifier: tag::Identifier) -> Result<tag::Tag, tag::GetTagError>;
}

#[async_trait]
pub trait SeriesRepository {}

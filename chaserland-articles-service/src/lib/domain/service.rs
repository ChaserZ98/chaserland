use super::model::article;
use super::repository::article::{ArticlesFilter, GetArticleError};
use chaserland_common::pagination::{Page, PageSize};
use tonic::async_trait;

#[async_trait]
pub trait ArticleService {
    async fn get_article(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<article::Article, GetArticleError>;

    async fn get_articles(
        &self,
        page: Page,
        page_size: PageSize,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<article::Article>, GetArticleError>;
}

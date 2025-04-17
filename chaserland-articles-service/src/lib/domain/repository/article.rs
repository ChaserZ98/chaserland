use crate::domain::entity::{article, category, series, tag};
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};

#[async_trait]
pub trait ArticleRepository {
    async fn create(
        &self,
        article: article::ArticleCreate,
    ) -> Result<article::Article, CreateArticleError>;
    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<article::Article, GetArticleError>;
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<article::Article>, GetArticleError>;
    async fn delete(&self, identifier: article::Identifier) -> Result<(), DeleteArticleError>;
}

#[derive(Debug, Default)]
pub struct ArticlesFilter {
    pub series_id: Option<series::SeriesId>,
    pub category_ids: Vec<category::CategoryId>,
    pub tag_ids: Vec<tag::TagId>,
}

#[derive(Debug, thiserror::Error)]
pub enum GetArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateArticleError {
    #[error("Article with slug {0} already exists")]
    DuplicateSlug(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

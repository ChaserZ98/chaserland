use crate::domain::model::article;
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};

#[async_trait]
pub trait ArticleRepository {
    type Tx;

    async fn create(
        &self,
        article: article::ArticleCreate,
        transaction: &mut Self::Tx,
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
    async fn delete_by_id(
        &self,
        id: article::ArticleId,
        transaction: &mut Self::Tx,
    ) -> Result<(), DeleteArticleError>;
}

#[derive(Debug, thiserror::Error)]
pub enum GetArticleError {
    #[error("Article with identifier not found: {identifier}")]
    NotFound { identifier: article::Identifier },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateArticleError {
    #[error("Article with slug {slug} already exists")]
    DuplicateSlug { slug: article::ArticleSlug },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteArticleError {
    #[error("Article with identifier not found: {identifier}")]
    NotFound { identifier: article::Identifier },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug)]
pub enum ArticlesFilter {
    SeriesFilter(String),
    CategoryFilter {
        category_slugs: Vec<String>,
        tag_slugs: Vec<String>,
    },
}

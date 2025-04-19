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
    async fn delete(
        &self,
        identifier: article::Identifier,
        hard: bool,
    ) -> Result<(), DeleteArticleError>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArticlesFilter {
    series_id: Option<series::Identifier>,
    category_ids: Vec<category::CategoryId>,
    tag_ids: Vec<tag::TagId>,
}

impl ArticlesFilter {
    pub fn new(
        series_id: Option<series::Identifier>,
        category_ids: Vec<category::CategoryId>,
        tag_ids: Vec<tag::TagId>,
    ) -> Self {
        Self::validate(&series_id, &category_ids, &tag_ids).unwrap();
        Self {
            series_id,
            category_ids,
            tag_ids,
        }
    }
    pub fn series_id(&self) -> &Option<series::Identifier> {
        &self.series_id
    }
    pub fn category_ids(&self) -> &Vec<category::CategoryId> {
        &self.category_ids
    }
    pub fn tag_ids(&self) -> &Vec<tag::TagId> {
        &self.tag_ids
    }
    fn validate(
        series_id: &Option<series::Identifier>,
        category_ids: &Vec<category::CategoryId>,
        tag_ids: &Vec<tag::TagId>,
    ) -> Result<(), String> {
        if series_id.is_none() && category_ids.is_empty() && tag_ids.is_empty() {
            return Err("At least one filter must be specified".to_string());
        }
        Ok(())
    }
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

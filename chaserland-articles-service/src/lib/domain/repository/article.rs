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
    async fn publish(
        &self,
        identifier: article::Identifier,
    ) -> Result<article::Article, PublishArticleError>;
    async fn soft_delete(
        &self,
        identifier: article::Identifier,
    ) -> Result<(), SoftDeleteArticleError>;
    async fn delete(&self, identifier: article::Identifier) -> Result<(), DeleteArticleError>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArticlesFilter {
    series_identifier: Option<series::Identifier>,
    category_ids: Vec<category::CategoryId>,
    tag_ids: Vec<tag::TagId>,
}

impl ArticlesFilter {
    pub fn new(
        series_identifier: Option<series::Identifier>,
        category_ids: Vec<category::CategoryId>,
        tag_ids: Vec<tag::TagId>,
    ) -> Self {
        Self::validate(&series_identifier, &category_ids, &tag_ids).unwrap();
        Self {
            series_identifier,
            category_ids,
            tag_ids,
        }
    }
    pub fn series_identifier(&self) -> &Option<series::Identifier> {
        &self.series_identifier
    }
    pub fn category_ids(&self) -> &Vec<category::CategoryId> {
        &self.category_ids
    }
    pub fn tag_ids(&self) -> &Vec<tag::TagId> {
        &self.tag_ids
    }
    fn validate(
        series_identifier: &Option<series::Identifier>,
        category_ids: &Vec<category::CategoryId>,
        tag_ids: &Vec<tag::TagId>,
    ) -> Result<(), String> {
        if series_identifier.is_none() && category_ids.is_empty() && tag_ids.is_empty() {
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
pub enum PublishArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error("Article with identifier {0} is already published")]
    AlreadyPublished(article::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SoftDeleteArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
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

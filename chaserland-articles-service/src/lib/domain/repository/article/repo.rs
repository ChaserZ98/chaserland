use super::ArticleRepositoryError;
use crate::domain::entity::{article, category, series, tag};
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};

#[async_trait]
pub trait ArticleRepository: Send + Sync + 'static {
    async fn create(
        &self,
        article: article::ArticleCreate,
    ) -> Result<article::Article, ArticleRepositoryError>;
    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<article::Article, ArticleRepositoryError>;
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<article::Article>, ArticleRepositoryError>;
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArticlesFilter {
    series_identifier: Option<series::Identifier>,
    category_ids: Vec<category::Id>,
    tag_ids: Vec<tag::Id>,
}

impl ArticlesFilter {
    pub fn new(
        series_identifier: Option<series::Identifier>,
        category_ids: Vec<category::Id>,
        tag_ids: Vec<tag::Id>,
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
    pub fn category_ids(&self) -> &Vec<category::Id> {
        &self.category_ids
    }
    pub fn tag_ids(&self) -> &Vec<tag::Id> {
        &self.tag_ids
    }
    fn validate(
        series_identifier: &Option<series::Identifier>,
        category_ids: &Vec<category::Id>,
        tag_ids: &Vec<tag::Id>,
    ) -> Result<(), String> {
        if series_identifier.is_none() && category_ids.is_empty() && tag_ids.is_empty() {
            return Err("At least one filter must be specified".to_string());
        }
        Ok(())
    }
}

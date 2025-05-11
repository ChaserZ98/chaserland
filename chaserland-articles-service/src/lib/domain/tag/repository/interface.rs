use super::{TagRepositoryError, TagsFilter};
use crate::domain::tag::{entity::Tag, vo as tag};
use async_trait::async_trait;
use chaserland_common::pagination::Pagination;

#[async_trait]
pub trait TagRepository: Send + Sync + 'static {
    async fn create(&self, new_tag: tag::NewTag) -> Result<Tag, TagRepositoryError>;

    async fn get_one(&self, identifier: tag::Identifier) -> Result<Tag, TagRepositoryError>;

    async fn get_many(
        &self,
        filter: Option<TagsFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Tag>, TagRepositoryError>;

    async fn delete(&self, identifier: tag::Identifier) -> Result<(), TagRepositoryError>;
}

use super::TagRepositoryError;
use crate::domain::tag::{entity::Tag, vo as tag};
use sqlx::{Database, Transaction};

#[trait_variant::make(TagRepository: Send)]
pub trait LocalTagRepository: Clone + Sync + 'static {
    type DB: Database;
    async fn create(
        &self,
        new_tag: tag::NewTag,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Tag, TagRepositoryError>;

    async fn get_one(
        &self,
        identifier: tag::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Tag, TagRepositoryError>;

    // async fn get_many(
    //     &self,
    //     filter: Option<TagsFilter>,
    //     pagination: Option<Pagination>,
    // ) -> Result<Vec<Tag>, TagRepositoryError>;

    async fn delete(
        &self,
        identifier: tag::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), TagRepositoryError>;
}

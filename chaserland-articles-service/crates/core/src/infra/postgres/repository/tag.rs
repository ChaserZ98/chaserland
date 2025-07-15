use crate::{
    domain::tag::{
        entity::Tag,
        repository::{TagRepository, TagRepositoryError},
        vo as tag,
    },
    infra::postgres::po::PgTag,
};
use sqlx::{Postgres, QueryBuilder, Transaction};

#[derive(Clone)]
pub struct PgTagRepository {}

impl PgTagRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl TagRepository for PgTagRepository {
    type DB = Postgres;

    async fn create(
        &self,
        new_tag: tag::NewTag,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Tag, TagRepositoryError> {
        let tag: PgTag =
            sqlx::query_as("INSERT INTO article.tags (name, slug) VALUES ($1, $2) RETURNING *")
                .bind(&new_tag.name.value())
                .bind(&new_tag.slug().value())
                .fetch_one(&mut **tx)
                .await
                .map_err(|why| match why {
                    sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                        TagRepositoryError::DuplicateTagSlug(new_tag)
                    }
                    _ => TagRepositoryError::Sqlx(why.into()),
                })?;

        let tag = tag
            .try_into()
            .map_err(|why: String| TagRepositoryError::DOConversion(why))?;

        Ok(tag)
    }
    async fn get_one(
        &self,
        identifier: tag::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Tag, TagRepositoryError> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM article.tags WHERE ");
        match &identifier {
            tag::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            tag::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        }

        let tag: Option<PgTag> = query.build_query_as().fetch_optional(&mut **tx).await?;

        if tag.is_none() {
            return Err(TagRepositoryError::TagNotFound(identifier));
        }

        let tag = tag.unwrap();

        let tag = tag
            .try_into()
            .map_err(|why: String| TagRepositoryError::DOConversion(why))?;

        Ok(tag)
    }

    async fn delete(
        &self,
        identifier: tag::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), TagRepositoryError> {
        let mut query = QueryBuilder::<Postgres>::new("DELETE FROM article.tags WHERE ");
        match &identifier {
            tag::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            tag::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        };

        let rows_affected = query.build().execute(&mut **tx).await?.rows_affected();

        if rows_affected == 0 {
            return Err(TagRepositoryError::TagNotFound(identifier));
        }

        Ok(())
    }
}

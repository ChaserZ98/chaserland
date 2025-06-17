use crate::domain::tag::{
    entity::Tag,
    repository::{TagRepository, TagRepositoryError, TagsFilter},
    vo as tag,
};
use async_trait::async_trait;
use chaserland_common::pagination::Pagination;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

#[derive(FromRow)]
pub struct PgTag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl TryInto<Tag> for PgTag {
    type Error = String;

    fn try_into(self) -> Result<Tag, Self::Error> {
        let id = self.id.try_into()?;
        let name = self.name.try_into()?;

        let tag = Tag::new(id, name);

        Ok(tag)
    }
}

pub struct PgTagRepository {
    pool: PgPool,
}

impl PgTagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepository for PgTagRepository {
    async fn create(&self, new_tag: tag::NewTag) -> Result<Tag, TagRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| TagRepositoryError::Transaction(why.to_string()))?;
        let tag: PgTag =
            sqlx::query_as("INSERT INTO article.tags (name, slug) VALUES ($1, $2) RETURNING *")
                .bind(&new_tag.name.value())
                .bind(&new_tag.slug().value())
                .fetch_one(&mut *tx)
                .await
                .map_err(|why| match why {
                    sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                        TagRepositoryError::DuplicateTagSlug(new_tag)
                    }
                    _ => TagRepositoryError::Unknown(why.into()),
                })?;

        let tag = tag
            .try_into()
            .map_err(|why: String| TagRepositoryError::DOConversion(why))?;

        tx.commit()
            .await
            .map_err(|why| TagRepositoryError::Transaction(why.to_string()))?;

        Ok(tag)
    }
    async fn get_one(&self, identifier: tag::Identifier) -> Result<Tag, TagRepositoryError> {
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

        let tag: Option<PgTag> = query
            .build_query_as()
            .fetch_optional(&self.pool)
            .await
            .map_err(|why| TagRepositoryError::Unknown(why.into()))?;

        if tag.is_none() {
            return Err(TagRepositoryError::TagNotFound(identifier));
        }

        let tag = tag.unwrap();

        let tag = tag
            .try_into()
            .map_err(|why: String| TagRepositoryError::DOConversion(why))?;

        Ok(tag)
    }
    async fn get_many(
        &self,
        filter: Option<TagsFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Tag>, TagRepositoryError> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM article.tags");

        if let Some(filter) = filter {
            query.push(" WHERE id = ANY(");
            query.push_bind(
                filter
                    .tag_ids()
                    .iter()
                    .map(|id| id.value())
                    .collect::<Vec<i32>>(),
            );
            query.push(")");
        }
        if let Some(pagination) = pagination {
            query.push(" LIMIT ");
            query.push_bind(pagination.page_size.value());
            query.push(" OFFSET ");
            query.push_bind(pagination.as_offset().value());
        }

        let tags: Vec<PgTag> = query
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|why| TagRepositoryError::Unknown(why.into()))?;

        let tags = tags
            .into_iter()
            .map(|x| {
                x.try_into()
                    .map_err(|why: String| TagRepositoryError::DOConversion(why))
            })
            .collect::<Result<Vec<Tag>, TagRepositoryError>>()?;

        Ok(tags)
    }
    async fn delete(&self, identifier: tag::Identifier) -> Result<(), TagRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| TagRepositoryError::Transaction(why.to_string()))?;

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

        let rows_affected = query
            .build()
            .execute(&mut *tx)
            .await
            .map_err(|why| TagRepositoryError::Unknown(why.into()))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(TagRepositoryError::TagNotFound(identifier));
        }

        tx.commit()
            .await
            .map_err(|why| TagRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
}

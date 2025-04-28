use crate::domain::entity::tag;
use crate::domain::repository::tag::{TagRepository, TagRepositoryError};
use async_trait::async_trait;
use chaserland_common::pagination::{Offset, Page, PageSize};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

#[derive(FromRow)]
pub struct PgTag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl TryInto<tag::Tag> for PgTag {
    type Error = String;

    fn try_into(self) -> Result<tag::Tag, Self::Error> {
        let id = self.id.try_into()?;
        let name = self.name.try_into()?;

        let tag = tag::Tag::new(id, name);

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
    async fn create(&self, name: tag::Name) -> Result<tag::Tag, TagRepositoryError> {
        let slug = name.as_slug();
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| TagRepositoryError::Transaction(why.to_string()))?;
        let tag: PgTag =
            sqlx::query_as("INSERT INTO article.tags (name, slug) VALUES ($1, $2) RETURNING *")
                .bind(&name.value())
                .bind(&name.as_slug().value())
                .fetch_one(&mut *tx)
                .await
                .map_err(|why| match why {
                    sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                        TagRepositoryError::DuplicateTagSlug(name, slug)
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
    async fn get_one(&self, identifier: tag::Identifier) -> Result<tag::Tag, TagRepositoryError> {
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
        page: Page,
        page_size: PageSize,
    ) -> Result<Vec<tag::Tag>, TagRepositoryError> {
        let offset = Offset::from((page, page_size));
        let tags: Vec<PgTag> = sqlx::query_as("SELECT * FROM article.tags LIMIT $1 OFFSET $2")
            .bind(page_size.value())
            .bind(offset.value())
            .fetch_all(&self.pool)
            .await
            .map_err(|why| TagRepositoryError::Unknown(why.into()))?;

        let tags = tags
            .into_iter()
            .map(|x| {
                x.try_into()
                    .map_err(|why: String| TagRepositoryError::DOConversion(why))
            })
            .collect::<Result<Vec<tag::Tag>, TagRepositoryError>>()?;

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

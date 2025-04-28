use crate::domain::entity::category;
use crate::domain::repository::category::{CategoryRepository, CategoryRepositoryError};
use async_trait::async_trait;
use chaserland_common::pagination::{Offset, Page, PageSize};
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(sqlx::FromRow)]
pub struct PgCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl TryInto<category::Category> for PgCategory {
    type Error = String;

    fn try_into(self) -> Result<category::Category, Self::Error> {
        let id = self.id.try_into()?;
        let name = self.name.try_into()?;
        let category = category::Category::new(id, name);
        Ok(category)
    }
}

pub struct PgCategoryRepository {
    pool: PgPool,
}

impl PgCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for PgCategoryRepository {
    async fn create(
        &self,
        name: category::Name,
    ) -> Result<category::Category, CategoryRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| CategoryRepositoryError::Transaction(why.to_string()))?;

        let slug = name.as_slug();

        let category: PgCategory = sqlx::query_as(
            "INSERT INTO article.categories (name, slug) VALUES ($1, $2) RETURNING *",
        )
        .bind(&name.value())
        .bind(&slug.value())
        .fetch_one(&mut *tx)
        .await
        .map_err(|why| match why {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                CategoryRepositoryError::DuplicateCategorySlug(name, slug)
            }
            _ => CategoryRepositoryError::Unknown(why.into()),
        })?;

        let category = category
            .try_into()
            .map_err(|why| CategoryRepositoryError::DOConversion(why))?;

        tx.commit()
            .await
            .map_err(|why| CategoryRepositoryError::Transaction(why.to_string()))?;

        Ok(category)
    }
    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<category::Category, CategoryRepositoryError> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM article.categories WHERE ");

        match &identifier {
            category::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            category::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        };

        let category: Option<PgCategory> = query
            .build_query_as()
            .fetch_optional(&self.pool)
            .await
            .map_err(|why| CategoryRepositoryError::Unknown(why.into()))?;

        if category.is_none() {
            return Err(CategoryRepositoryError::CategoryNotFound(identifier));
        }

        let category = category.unwrap();

        let category = category
            .try_into()
            .map_err(|why: String| CategoryRepositoryError::DOConversion(why))?;

        Ok(category)
    }
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
    ) -> Result<Vec<category::Category>, CategoryRepositoryError> {
        let offset = Offset::from((page, page_size));

        let categories: Vec<PgCategory> =
            sqlx::query_as("SELECT * FROM article.categories LIMIT $1 OFFSET $2")
                .bind(page_size.value())
                .bind(offset.value())
                .fetch_all(&self.pool)
                .await
                .map_err(|why| CategoryRepositoryError::Unknown(why.into()))?;

        let categories = categories
            .into_iter()
            .map(|x| {
                x.try_into()
                    .map_err(|why: String| CategoryRepositoryError::DOConversion(why))
            })
            .collect::<Result<Vec<category::Category>, CategoryRepositoryError>>()?;

        Ok(categories)
    }
    async fn delete(
        &self,
        identifier: category::Identifier,
    ) -> Result<(), CategoryRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| CategoryRepositoryError::Transaction(why.to_string()))?;

        let mut query = QueryBuilder::<Postgres>::new("DELETE FROM article.categories WHERE ");

        match &identifier {
            category::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            category::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        };

        let rows_affected = query
            .build()
            .execute(&mut *tx)
            .await
            .map_err(|why| CategoryRepositoryError::Unknown(why.into()))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(CategoryRepositoryError::CategoryNotFound(identifier));
        }

        tx.commit()
            .await
            .map_err(|why| CategoryRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
}

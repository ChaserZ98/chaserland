use crate::{
    domain::category::{
        entity::Category,
        repository::{CategoryRepository, CategoryRepositoryError},
        vo as category,
    },
    infra::postgres::po::PgCategory,
};
use sqlx::{Postgres, QueryBuilder, Transaction};

#[derive(Clone)]
pub struct PgCategoryRepository {}

impl PgCategoryRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl CategoryRepository for PgCategoryRepository {
    type DB = Postgres;

    async fn create(
        &self,
        new_category: category::NewCategory,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Category, CategoryRepositoryError> {
        let category: PgCategory = sqlx::query_as(
            "INSERT INTO article.categories (name, slug) VALUES ($1, $2) RETURNING *",
        )
        .bind(&new_category.name.value())
        .bind(&new_category.slug().value())
        .fetch_one(&mut **tx)
        .await
        .map_err(|why| match why {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                CategoryRepositoryError::DuplicateCategorySlug(new_category)
            }
            _ => CategoryRepositoryError::Sqlx(why.into()),
        })?;

        let category = category
            .try_into()
            .map_err(|why| CategoryRepositoryError::DOConversion(why))?;

        Ok(category)
    }

    async fn get_one(
        &self,
        identifier: category::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Category, CategoryRepositoryError> {
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

        let category: Option<PgCategory> = query.build_query_as().fetch_optional(&mut **tx).await?;

        if category.is_none() {
            return Err(CategoryRepositoryError::CategoryNotFound(identifier));
        }

        let category = category.unwrap();

        let category = category
            .try_into()
            .map_err(|why: String| CategoryRepositoryError::DOConversion(why))?;

        Ok(category)
    }

    async fn delete(
        &self,
        identifier: category::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), CategoryRepositoryError> {
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

        let rows_affected = query.build().execute(&mut **tx).await?.rows_affected();

        if rows_affected == 0 {
            return Err(CategoryRepositoryError::CategoryNotFound(identifier));
        }

        Ok(())
    }
}

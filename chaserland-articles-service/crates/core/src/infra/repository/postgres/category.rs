use crate::domain::category::{
    entity::Category,
    repository::{CategoriesFilter, CategoryRepository, CategoryRepositoryError},
    vo as category,
};
use chaserland_common::pagination::Pagination;
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(sqlx::FromRow)]
pub struct PgCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl TryInto<Category> for PgCategory {
    type Error = String;

    fn try_into(self) -> Result<Category, Self::Error> {
        let id = self.id.try_into()?;
        let name = self.name.try_into()?;
        let category = Category::new(id, name);
        Ok(category)
    }
}

#[derive(Clone)]
pub struct PgCategoryRepository {
    pool: PgPool,
}

impl PgCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl CategoryRepository for PgCategoryRepository {
    async fn create(
        &self,
        new_category: category::NewCategory,
    ) -> Result<Category, CategoryRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| CategoryRepositoryError::Transaction(why.to_string()))?;

        let category: PgCategory = sqlx::query_as(
            "INSERT INTO article.categories (name, slug) VALUES ($1, $2) RETURNING *",
        )
        .bind(&new_category.name.value())
        .bind(&new_category.slug().value())
        .fetch_one(&mut *tx)
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

        tx.commit()
            .await
            .map_err(|why| CategoryRepositoryError::Transaction(why.to_string()))?;

        Ok(category)
    }
    async fn get_one(
        &self,
        identifier: category::Identifier,
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

        let category: Option<PgCategory> =
            query.build_query_as().fetch_optional(&self.pool).await?;

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
        filter: Option<CategoriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Category>, CategoryRepositoryError> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM article.categories");

        if let Some(filter) = filter {
            query.push(" WHERE id = ANY(");
            query.push_bind(
                filter
                    .category_ids()
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

        let categories: Vec<PgCategory> = query.build_query_as().fetch_all(&self.pool).await?;

        let categories = categories
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<Category>, _>>()
            .map_err(|e| CategoryRepositoryError::DOConversion(e))?;

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

        let rows_affected = query.build().execute(&mut *tx).await?.rows_affected();

        if rows_affected == 0 {
            return Err(CategoryRepositoryError::CategoryNotFound(identifier));
        }

        tx.commit()
            .await
            .map_err(|why| CategoryRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
}

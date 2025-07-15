use crate::{
    app::query::{
        dto::CategoryDTO,
        query_handler::{error::CategoryQueryHandlerError, interface::CategoryQueryHandler},
    },
    domain::category::{repository::CategoriesFilter, vo as category},
    infra::postgres::po::PgCategory,
};
use chaserland_common::pagination::Pagination;
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Clone)]
pub struct PgCategoryQueryHandler {
    pool: PgPool,
}

impl PgCategoryQueryHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl CategoryQueryHandler for PgCategoryQueryHandler {
    async fn get_one(
        &self,
        identifier: category::Identifier,
    ) -> Result<CategoryDTO, CategoryQueryHandlerError> {
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
            return Err(CategoryQueryHandlerError::CategoryNotFound(identifier));
        }

        let category = category.unwrap();

        Ok(category.into())
    }

    async fn get_many(
        &self,
        filter: Option<CategoriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<CategoryDTO>, CategoryQueryHandlerError> {
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
            .map(|x| x.into())
            .collect::<Vec<CategoryDTO>>();

        Ok(categories)
    }
}

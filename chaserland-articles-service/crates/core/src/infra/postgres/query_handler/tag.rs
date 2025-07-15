use crate::{
    app::query::{
        dto::TagDTO,
        query_handler::{error::TagQueryHandlerError, interface::TagQueryHandler},
    },
    domain::tag::{repository::TagsFilter, vo as tag},
    infra::postgres::po::PgTag,
};
use chaserland_common::pagination::Pagination;
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Clone)]
pub struct PgTagQueryHandler {
    pool: PgPool,
}

impl PgTagQueryHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TagQueryHandler for PgTagQueryHandler {
    async fn get_one(&self, identifier: tag::Identifier) -> Result<TagDTO, TagQueryHandlerError> {
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

        let tag: Option<PgTag> = query.build_query_as().fetch_optional(&self.pool).await?;

        if tag.is_none() {
            return Err(TagQueryHandlerError::TagNotFound(identifier));
        }

        let tag = tag.unwrap();

        let tag = tag.into();

        Ok(tag)
    }

    async fn get_many(
        &self,
        filter: Option<TagsFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<TagDTO>, TagQueryHandlerError> {
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

        let tags: Vec<PgTag> = query.build_query_as().fetch_all(&self.pool).await?;

        let tags = tags.into_iter().map(|x| x.into()).collect::<Vec<TagDTO>>();

        Ok(tags)
    }
}

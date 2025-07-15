use crate::{
    app::query::{
        dto::SeriesDTO,
        query_handler::{error::SeriesQueryHandlerError, interface::SeriesQueryHandler},
    },
    domain::series::{repository::SeriesFilter, vo as series},
    infra::postgres::po::PgSeries,
};
use chaserland_common::pagination::Pagination;
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Clone)]
pub struct PgSeriesQueryHandler {
    pool: PgPool,
}

impl PgSeriesQueryHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SeriesQueryHandler for PgSeriesQueryHandler {
    async fn get_one(
        &self,
        identifier: series::Identifier,
    ) -> Result<SeriesDTO, SeriesQueryHandlerError> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM article.series WHERE ");
        match &identifier {
            series::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            series::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        };

        let series: Option<PgSeries> = query.build_query_as().fetch_optional(&self.pool).await?;

        if series.is_none() {
            return Err(SeriesQueryHandlerError::SeriesNotFound(identifier));
        }

        let series = series.unwrap();

        let series = series.into();

        Ok(series)
    }

    async fn get_many(
        &self,
        filter: Option<SeriesFilter>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<SeriesDTO>, SeriesQueryHandlerError> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM article.series");
        if let Some(filter) = filter {
            query.push(" WHERE id = ANY(");
            query.push_bind(
                filter
                    .series_ids()
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
        let series: Vec<PgSeries> = query.build_query_as().fetch_all(&self.pool).await?;

        let series = series
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<SeriesDTO>>();

        Ok(series)
    }
}

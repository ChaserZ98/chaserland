use crate::{
    domain::series::{
        entity::Series,
        repository::{SeriesRepository, SeriesRepositoryError},
        vo as series,
    },
    infra::postgres::po::PgSeries,
};
use sqlx::{Postgres, QueryBuilder, Transaction};

#[derive(Clone)]
pub struct PgSeriesRepository {}

impl PgSeriesRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl SeriesRepository for PgSeriesRepository {
    type DB = Postgres;

    async fn create(
        &self,
        series: series::NewSeries,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Series, SeriesRepositoryError> {
        let series: PgSeries =
            sqlx::query_as("INSERT INTO article.series (name, slug) VALUES ($1, $2) RETURNING *")
                .bind(&series.name.value())
                .bind(&series.slug().value())
                .fetch_one(&mut **tx)
                .await
                .map_err(|e| match e {
                    sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                        SeriesRepositoryError::DuplicateSeriesSlug(series)
                    }
                    _ => SeriesRepositoryError::Sqlx(e.into()),
                })?;

        let series = series
            .try_into()
            .map_err(|why: String| SeriesRepositoryError::DOConversion(why))?;

        Ok(series)
    }
    async fn get_one(
        &self,
        identifier: series::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Series, SeriesRepositoryError> {
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

        let series: Option<PgSeries> = query.build_query_as().fetch_optional(&mut **tx).await?;

        if series.is_none() {
            return Err(SeriesRepositoryError::SeriesNotFound(identifier));
        }

        let series = series.unwrap();

        let series = series
            .try_into()
            .map_err(|why: String| SeriesRepositoryError::DOConversion(why))?;

        Ok(series)
    }

    async fn delete(
        &self,
        identifier: series::Identifier,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<(), SeriesRepositoryError> {
        let mut query = QueryBuilder::<Postgres>::new("DELETE FROM article.series WHERE ");

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

        let rows_affected = query.build().execute(&mut **tx).await?.rows_affected();

        if rows_affected == 0 {
            return Err(SeriesRepositoryError::SeriesNotFound(identifier));
        }

        Ok(())
    }
}

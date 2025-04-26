use crate::domain::entity::series;
use crate::domain::repository::series::{SeriesRepository, error};
use async_trait::async_trait;
use chaserland_common::pagination::{Offset, Page, PageSize};
use sqlx::{Postgres, QueryBuilder};

#[derive(sqlx::FromRow)]
pub struct PgSeries {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl TryInto<series::Series> for PgSeries {
    type Error = String;

    fn try_into(self) -> Result<series::Series, Self::Error> {
        let id = self.id.try_into()?;
        let name = self.name.try_into()?;

        let series = series::Series::new(id, name);

        Ok(series)
    }
}

pub struct PgSeriesRepository {
    pool: sqlx::PgPool,
}

impl PgSeriesRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SeriesRepository for PgSeriesRepository {
    async fn create(&self, name: series::Name) -> Result<series::Series, error::CreateSeriesError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| error::CreateSeriesError::Unknown(why.into()))?;

        let slug = name.as_slug();

        let series: PgSeries =
            sqlx::query_as("INSERT INTO article.series (name, slug) VALUES ($1, $2) RETURNING *")
                .bind(&name.value())
                .bind(&slug.value())
                .fetch_one(&mut *tx)
                .await
                .map_err(|why| match why {
                    sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                        error::CreateSeriesError::AlreadyExists(name, slug)
                    }
                    _ => error::CreateSeriesError::Unknown(why.into()),
                })?;

        let series = series
            .try_into()
            .map_err(|why: String| error::CreateSeriesError::Unknown(anyhow::anyhow!(why)))?;

        tx.commit()
            .await
            .map_err(|why| error::CreateSeriesError::Unknown(why.into()))?;

        Ok(series)
    }
    async fn get_one(
        &self,
        identifier: series::Identifier,
    ) -> Result<series::Series, error::GetSeriesError> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM article.series WHERE ");
        match identifier.clone() {
            series::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            series::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        };

        let series: Option<PgSeries> = query
            .build_query_as()
            .fetch_optional(&self.pool)
            .await
            .map_err(|why| error::GetSeriesError::Unknown(why.into()))?;

        if series.is_none() {
            return Err(error::GetSeriesError::NotFound(identifier));
        }

        let series = series.unwrap();

        let series = series
            .try_into()
            .map_err(|why: String| error::GetSeriesError::Unknown(anyhow::anyhow!(why)))?;

        Ok(series)
    }
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
    ) -> Result<Vec<series::Series>, error::GetSeriesError> {
        let offset = Offset::from((page, page_size));

        let series: Vec<PgSeries> =
            sqlx::query_as("SELECT * FROM article.series LIMIT $1 OFFSET $2")
                .bind(page_size.value())
                .bind(offset.value())
                .fetch_all(&self.pool)
                .await
                .map_err(|why| error::GetSeriesError::Unknown(why.into()))?;

        let series = series
            .into_iter()
            .map(|x| {
                x.try_into()
                    .map_err(|why: String| error::GetSeriesError::Unknown(anyhow::anyhow!(why)))
            })
            .collect::<Result<Vec<series::Series>, error::GetSeriesError>>()?;

        Ok(series)
    }
    async fn delete(&self, identifier: series::Identifier) -> Result<(), error::DeleteSeriesError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| error::DeleteSeriesError::Unknown(why.into()))?;

        let mut query = QueryBuilder::<Postgres>::new("DELETE FROM article.series WHERE ");

        match identifier.clone() {
            series::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            series::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        };

        let rows_affected = query
            .build()
            .execute(&mut *tx)
            .await
            .map_err(|why| error::DeleteSeriesError::Unknown(why.into()))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(error::DeleteSeriesError::NotFound(identifier));
        }

        tx.commit()
            .await
            .map_err(|why| error::DeleteSeriesError::Unknown(why.into()))?;

        Ok(())
    }
}

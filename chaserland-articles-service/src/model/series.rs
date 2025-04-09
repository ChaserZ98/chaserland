use anyhow::Result;
use chaserland_protos::article::v1::SeriesCreate;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use sqlx::{FromRow, PgPool, Postgres, Transaction};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Series {
    pub id: i32,
    pub slug: String,
    pub name: String,
}

#[allow(dead_code)]
impl Series {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        series: SeriesCreate,
    ) -> Result<Self> {
        let name = series.name;
        let slug = slugify!(&name, separator = "-");
        let series = sqlx::query_as("INSERT INTO article.series (slug, name) VALUES ($1, $2) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *")
            .bind(slug)
            .bind(name)
            .fetch_one(&mut **transaction)
            .await?;
        Ok(series)
    }
    pub async fn get(db: &PgPool) -> Result<Vec<Self>> {
        let res = sqlx::query_as("SELECT * FROM article.series")
            .fetch_all(db)
            .await?;
        Ok(res)
    }
    pub async fn get_by_id(db: &PgPool, id: i32) -> Result<Option<Self>> {
        let res = sqlx::query_as("SELECT * FROM article.series WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;
        Ok(res)
    }
    pub async fn get_by_article_id(db: &PgPool, id: i32) -> Result<Option<Self>> {
        let res = sqlx::query_as("SELECT a.id, a.slug, a.name FROM article.series AS a JOIN article.articles AS b ON a.id = b.series_id WHERE b.id = $1").bind(id).fetch_optional(db).await?;
        Ok(res)
    }
    pub async fn delete_by_id(transaction: &mut Transaction<'_, Postgres>, id: i32) -> Result<u64> {
        let row_count = sqlx::query("DELETE FROM article.series WHERE id = $1")
            .bind(id)
            .execute(&mut **transaction)
            .await?
            .rows_affected();
        println!("row_count: {}", row_count);
        Ok(row_count)
    }
}

impl Into<chaserland_protos::article::v1::Series> for Series {
    fn into(self) -> chaserland_protos::article::v1::Series {
        chaserland_protos::article::v1::Series {
            id: self.id,
            slug: self.slug,
            name: self.name,
        }
    }
}

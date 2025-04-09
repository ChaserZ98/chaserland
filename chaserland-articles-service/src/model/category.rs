use anyhow::Result;
use chaserland_protos::article::v1::CategoryCreate;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use sqlx::{FromRow, PgPool, Postgres, Transaction};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub slug: String,
    pub name: String,
}

#[allow(dead_code)]
impl Category {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        category: CategoryCreate,
    ) -> Result<Self> {
        let name = category.name;
        let slug = slugify!(&name, separator = "-");
        let category = sqlx::query_as("INSERT INTO article.categories (slug, name) VALUES ($1, $2) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *").bind(slug).bind(name).fetch_one(&mut **transaction).await?;
        Ok(category)
    }

    pub async fn get(db: &PgPool) -> Result<Vec<Self>> {
        let res = sqlx::query_as("SELECT * FROM article.categories")
            .fetch_all(db)
            .await?;
        Ok(res)
    }

    pub async fn get_all_by_article_id(db: &PgPool, id: i32) -> Result<Vec<Category>> {
        let res = sqlx::query_as("SELECT a.id, a.slug, a.name FROM article.categories AS a JOIN article.article_categories AS b ON a.id = b.category_id WHERE b.article_id = $1").bind(id).fetch_all(db).await?;
        Ok(res)
    }

    pub async fn delete_by_id(transaction: &mut Transaction<'_, Postgres>, id: i32) -> Result<u64> {
        let row_count = sqlx::query("DELETE FROM article.categories WHERE id = $1")
            .bind(id)
            .execute(&mut **transaction)
            .await?
            .rows_affected();
        Ok(row_count)
    }
}

impl Into<chaserland_protos::article::v1::Category> for Category {
    fn into(self) -> chaserland_protos::article::v1::Category {
        chaserland_protos::article::v1::Category {
            id: self.id,
            slug: self.slug,
            name: self.name,
        }
    }
}

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Series {
    id: i32,
    slug: String,
    name: String,
}

#[allow(dead_code)]
impl Series {
    pub async fn get_articles(&self, db: &PgPool) -> Result<Vec<Article>> {
        let res = sqlx::query_as("SELECT * FROM article.articles WHERE series_id = $1")
            .bind(self.id)
            .fetch_all(db)
            .await?;
        Ok(res)
    }

    pub async fn get_by_article_id(db: &PgPool, id: i32) -> Result<Option<Self>> {
        let res = sqlx::query_as("SELECT a.id, a.slug, a.name FROM article.series AS a JOIN article.articles AS b ON a.id = b.series_id WHERE b.id = $1").bind(id).fetch_optional(db).await?;
        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    id: i32,
    title: String,
    slug: String,
    description: String,
    content: String,
    series_id: Option<i32>,
    created_at: chrono::DateTime<chrono::Utc>,
    published_at: Option<chrono::DateTime<chrono::Utc>>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[allow(dead_code)]
impl Article {
    pub async fn get(db: &PgPool, page: i32, page_size: i32) -> Result<Vec<Self>> {
        let offset = (page - 1) * page_size;
        let res = sqlx::query_as("SELECT * FROM article.articles LIMIT $1 OFFSET $2")
            .bind(page_size)
            .bind(offset)
            .fetch_all(db)
            .await?;
        Ok(res)
    }
    pub async fn get_meta(db: &PgPool, page: i32, page_size: i32) -> Result<Vec<Self>> {}
    pub async fn get_meta_by_slug(db: &PgPool, slug: &str) -> Result<Option<Self>> {
        let article = sqlx::query_as("SELECT id, title, slug, description, series_id, created_at, published_at, updated_at, deleted_at FROM article.articles WHERE slug = $1").bind(slug).fetch_optional(db).await?;
        Ok(article)
    }
    pub async fn get_by_slug(db: &PgPool, id: i32) -> Result<Option<Self>> {
        let article = sqlx::query_as("SELECT * FROM article.articles WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;
        Ok(article)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    id: i32,
    slug: String,
    name: String,
}

#[allow(dead_code)]
impl Category {
    pub async fn get_articles(
        &self,
        db: &PgPool,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Article>> {
        let offset = (page - 1) * page_size;
        let res = sqlx::query_as(
            "SELECT * FROM article.articles WHERE category_id = $1 LIMIT $2 OFFSET $3",
        )
        .bind(self.id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(db)
        .await?;
        Ok(res)
    }

    pub async fn get_all_by_article_id(db: &PgPool, id: i32) -> Result<Vec<Category>> {
        let res = sqlx::query_as("SELECT a.id, a.slug, a.name FROM article.categories AS a JOIN article.article_categories AS b ON a.id = b.category_id WHERE b.article_id = $1").bind(id).fetch_all(db).await?;
        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    id: i32,
    slug: String,
    name: String,
}

#[allow(dead_code)]
impl Tag {
    pub async fn get_articles(
        &self,
        db: &PgPool,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Article>> {
        let offset = (page - 1) * page_size;
        let res =
            sqlx::query_as("SELECT * FROM article.articles WHERE tag_id = $1 LIMIT $2 OFFSET $3")
                .bind(self.id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(db)
                .await?;
        Ok(res)
    }
    pub async fn get_all_by_article_id(db: &PgPool, id: i32) -> Result<Vec<Tag>> {
        let res = sqlx::query_as("SELECT a.id, a.slug, a.name FROM article.tags AS a JOIN article.article_tags AS b ON a.id = b.tag_id WHERE b.article_id = $1").bind(id).fetch_all(db).await?;
        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleMeta {
    id: i32,
    title: String,
    slug: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
    published_at: Option<chrono::DateTime<chrono::Utc>>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    series: Option<Series>,
    categories: Vec<Category>,
    tags: Vec<Tag>,
}

#[allow(dead_code)]
impl ArticleMeta {
    // pub async fn get(db:&PgPool, page: i32, page_size: i32) -> Result<Vec<Self>> {
    //     let offset = (page - 1) * page_size;
    //     let articles =
    // }
    pub async fn get_by_slug(db: &PgPool, slug: &str) -> Result<Option<Self>> {
        let article = Article::get_meta_by_slug(db, slug).await?;
        if article.is_none() {
            return Ok(None);
        }
        let article = article.unwrap();

        let tags = Tag::get_all_by_article_id(db, article.id).await?;

        let categories = Category::get_all_by_article_id(db, article.id).await?;

        let series = Series::get_by_article_id(db, article.id).await?;

        let res = Self {
            id: article.id,
            title: article.title,
            slug: article.slug,
            description: article.description,
            created_at: article.created_at,
            published_at: article.published_at,
            updated_at: article.updated_at,
            deleted_at: article.deleted_at,
            series,
            categories,
            tags,
        };
        Ok(Some(res))
    }
}

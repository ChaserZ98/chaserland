use anyhow::Result;
use chaserland_protos::article::{ArticleCreate, CategoryCreate, SeriesCreate, TagCreate};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, Transaction};

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
        let series = sqlx::query_as("INSERT INTO article.series (slug, name) VALUES ($1, $2) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *")
            .bind(series.slug)
            .bind(series.name)
            .fetch_one(&mut **transaction)
            .await?;
        Ok(series)
    }

    pub async fn get_by_id(db: &PgPool, id: i32) -> Result<Option<Self>> {
        let res = sqlx::query_as("SELECT * FROM article.series WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;
        Ok(res)
    }

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

impl Into<chaserland_protos::article::Series> for Series {
    fn into(self) -> chaserland_protos::article::Series {
        chaserland_protos::article::Series {
            id: self.id,
            slug: self.slug,
            name: self.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: String,
    pub series_id: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[allow(dead_code)]
impl Article {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        article: ArticleCreate,
    ) -> Result<Self> {
        let series_id = article.series_id.clone();

        let article = sqlx::query_as(
            "INSERT INTO article.articles (title, slug, description, content, series_id) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *",
        )
        .bind(article.title)
        .bind(article.slug)
        .bind(article.description)
        .bind(article.content)
        .bind(series_id)
        .fetch_one(&mut **transaction)
        .await?;

        Ok(article)
    }
    pub async fn get(db: &PgPool, page: i32, page_size: i32) -> Result<Vec<Self>> {
        let offset = (page - 1) * page_size;
        let res = sqlx::query_as("SELECT * FROM article.articles LIMIT $1 OFFSET $2")
            .bind(page_size)
            .bind(offset)
            .fetch_all(db)
            .await?;
        Ok(res)
    }
    pub async fn get_by_slug(db: &PgPool, id: i32) -> Result<Option<Self>> {
        let article = sqlx::query_as("SELECT * FROM article.articles WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;
        Ok(article)
    }
    pub async fn get_content_by_slug(db: &PgPool, slug: String) -> Result<Option<String>> {
        let content = sqlx::query_scalar("SELECT content FROM article.articles WHERE slug = $1")
            .bind(slug)
            .fetch_optional(db)
            .await?;
        Ok(content)
    }
    pub async fn add_tags(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
        tag_ids: Vec<i32>,
    ) -> Result<Vec<Tag>> {
        let mut query = QueryBuilder::new("INSERT INTO article.article_tags (article_id, tag_id) ");
        query.push_values(tag_ids.clone(), |mut b, tag_id| {
            b.push_bind(article_id).push_bind(tag_id);
        });
        query.push(" ON CONFLICT (article_id, tag_id) DO NOTHING");
        let query = query.build();

        query.execute(&mut **transaction).await?;

        let tags = sqlx::query_as("SELECT * FROM article.tags WHERE id = ANY($1)")
            .bind(tag_ids)
            .fetch_all(&mut **transaction)
            .await?;

        Ok(tags)
    }
    pub async fn add_categories(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
        category_ids: Vec<i32>,
    ) -> Result<Vec<Category>> {
        let mut query =
            QueryBuilder::new("INSERT INTO article.article_categories (article_id, category_id) ");
        query.push_values(category_ids.clone(), |mut b, category_id| {
            b.push_bind(article_id).push_bind(category_id);
        });
        query.push(" ON CONFLICT (article_id, category_id) DO NOTHING");
        let query = query.build();

        query.execute(&mut **transaction).await?;

        let categories = sqlx::query_as("SELECT * FROM article.categories WHERE id = ANY($1)")
            .bind(category_ids)
            .fetch_all(&mut **transaction)
            .await?;

        Ok(categories)
    }
    pub async fn update_series(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
        series_id: i32,
    ) -> Result<Series> {
        sqlx::query("UPDATE article.articles SET series_id = $1 WHERE id = $2")
            .bind(series_id)
            .bind(article_id)
            .execute(&mut **transaction)
            .await?;
        let series = sqlx::query_as("SELECT * FROM article.series WHERE id = $1")
            .bind(series_id)
            .fetch_one(&mut **transaction)
            .await?;
        Ok(series)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ArticleMeta {
    id: i32,
    title: String,
    slug: String,
    description: String,
    series_id: Option<i32>,
    created_at: chrono::DateTime<chrono::Utc>,
    published_at: Option<chrono::DateTime<chrono::Utc>>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ArticleMeta {
    pub async fn get(db: &PgPool, page: i32, page_size: i32) -> Result<Vec<Self>> {
        let offset = (page - 1) * page_size;
        let res = sqlx::query_as("SELECT id, title, slug, description, series_id, created_at, published_at, updated_at, deleted_at FROM article.articles LIMIT $1 OFFSET $2")
            .bind(page_size)
            .bind(offset)
            .fetch_all(db)
            .await?;
        Ok(res)
    }
    pub async fn get_by_slug(db: &PgPool, slug: &str) -> Result<Option<Self>> {
        let article = sqlx::query_as("SELECT id, title, slug, description, series_id, created_at, published_at, updated_at, deleted_at FROM article.articles WHERE slug = $1").bind(slug).fetch_optional(db).await?;
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
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        category: CategoryCreate,
    ) -> Result<Self> {
        let category = sqlx::query_as("INSERT INTO article.categories (slug, name) VALUES ($1, $2) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *").bind(category.slug).bind(category.name).fetch_one(&mut **transaction).await?;
        Ok(category)
    }

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

impl Into<chaserland_protos::article::Category> for Category {
    fn into(self) -> chaserland_protos::article::Category {
        chaserland_protos::article::Category {
            id: self.id,
            slug: self.slug,
            name: self.name,
        }
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
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        tag: TagCreate,
    ) -> Result<Self> {
        let tag = sqlx::query_as("INSERT INTO article.tags (slug, name) VALUES ($1, $2) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *").bind(tag.slug).bind(tag.name).fetch_one(&mut **transaction).await?;
        Ok(tag)
    }

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

impl Into<chaserland_protos::article::Tag> for Tag {
    fn into(self) -> chaserland_protos::article::Tag {
        chaserland_protos::article::Tag {
            id: self.id,
            slug: self.slug,
            name: self.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullArticle {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub series: Option<Series>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
}

impl Into<chaserland_protos::article::Article> for FullArticle {
    fn into(self) -> chaserland_protos::article::Article {
        chaserland_protos::article::Article {
            id: self.id,
            title: self.title,
            slug: self.slug,
            description: self.description,
            content: self.content,
            created_at: Some(prost_types::Timestamp {
                seconds: self.created_at.timestamp(),
                nanos: self.created_at.timestamp_subsec_nanos() as i32,
            }),
            published_at: self.published_at.map(|dt| prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: self.updated_at.timestamp(),
                nanos: self.updated_at.timestamp_subsec_nanos() as i32,
            }),
            deleted_at: self.deleted_at.map(|dt| prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            series: self.series.map(|x| x.into()),
            categories: self.categories.into_iter().map(|x| x.into()).collect(),
            tags: self.tags.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullArticleMeta {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub series: Option<Series>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
}

#[allow(dead_code)]
impl FullArticleMeta {
    pub async fn get(db: &PgPool, page: i32, page_size: i32) -> Result<Vec<Self>> {
        let article_metas = ArticleMeta::get(db, page, page_size).await?;
        let mut res = Vec::new();
        for article_meta in article_metas {
            let series = Series::get_by_article_id(db, article_meta.id).await?;
            let categories = Category::get_all_by_article_id(db, article_meta.id).await?;
            let tags = Tag::get_all_by_article_id(db, article_meta.id).await?;
            res.push(FullArticleMeta {
                id: article_meta.id,
                title: article_meta.title,
                slug: article_meta.slug,
                description: article_meta.description,
                created_at: article_meta.created_at,
                published_at: article_meta.published_at,
                updated_at: article_meta.updated_at,
                deleted_at: article_meta.deleted_at,
                series,
                categories,
                tags,
            });
        }
        Ok(res)
    }
    pub async fn get_by_slug(db: &PgPool, slug: &str) -> Result<Option<Self>> {
        let article_meta = ArticleMeta::get_by_slug(db, slug).await?;
        if article_meta.is_none() {
            return Ok(None);
        }
        let article_meta = article_meta.unwrap();

        let tags = Tag::get_all_by_article_id(db, article_meta.id).await?;

        let categories = Category::get_all_by_article_id(db, article_meta.id).await?;

        let series = Series::get_by_article_id(db, article_meta.id).await?;

        let res = Self {
            id: article_meta.id,
            title: article_meta.title,
            slug: article_meta.slug,
            description: article_meta.description,
            created_at: article_meta.created_at,
            published_at: article_meta.published_at,
            updated_at: article_meta.updated_at,
            deleted_at: article_meta.deleted_at,
            series,
            categories,
            tags,
        };
        Ok(Some(res))
    }
}

impl Into<chaserland_protos::article::ArticleMeta> for FullArticleMeta {
    fn into(self) -> chaserland_protos::article::ArticleMeta {
        chaserland_protos::article::ArticleMeta {
            id: self.id,
            title: self.title,
            slug: self.slug,
            description: self.description,
            series: self.series.map(|x| x.into()),
            categories: self.categories.into_iter().map(|x| x.into()).collect(),
            tags: self.tags.into_iter().map(|x| x.into()).collect(),
            created_at: Some(prost_types::Timestamp {
                seconds: self.created_at.timestamp(),
                nanos: self.created_at.timestamp_subsec_nanos() as i32,
            }),
            published_at: self.published_at.map(|dt| prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: self.updated_at.timestamp(),
                nanos: self.updated_at.timestamp_subsec_nanos() as i32,
            }),
            deleted_at: self.deleted_at.map(|dt| prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
        }
    }
}

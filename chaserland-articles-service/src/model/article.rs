use super::{Category, Series, Tag};
use anyhow::Result;
use chaserland_protos::article::v1::ArticleCreate;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use sqlx::types::chrono;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, Transaction};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: Option<String>,
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
        let slug = slugify!(&article.title, separator = "-");

        let article = sqlx::query_as(
            "INSERT INTO article.articles (title, slug, description, content, series_id) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *",
        )
        .bind(article.title)
        .bind(slug)
        .bind(article.description)
        .bind(article.content)
        .bind(series_id)
        .fetch_one(&mut **transaction)
        .await?;

        Ok(article)
    }
    pub async fn get_many(
        db: &PgPool,
        page: i32,
        page_size: i32,
        public_only: bool,
        with_content: bool,
        filter: Option<chaserland_protos::article::v1::get_articles_request::Filter>,
    ) -> Result<Vec<Self>> {
        let offset = (page - 1) * page_size;
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM ");

        match (public_only, with_content) {
            (true, true) => {
                query.push("article.public_articles");
            }
            (true, false) => {
                query.push("article.public_articles_meta ");
            }
            (false, true) => {
                query.push("article.articles ");
            }
            (false, false) => {
                query.push("article.articles_meta ");
            }
        }
        query.push("AS a ");

        match filter {
            Some(chaserland_protos::article::v1::get_articles_request::Filter::SeriesFilter(
                series_filter,
            )) => {
                let series_slug = series_filter.series_slug;
                query.push("JOIN article.series AS b ON a.series_id = b.id WHERE b.slug = ANY(");
                query.push_bind(series_slug);
                query.push(")");
            }
            Some(
                chaserland_protos::article::v1::get_articles_request::Filter::CategoryTagFilter(
                    category_tag_filter,
                ),
            ) => {
                let category_slugs = category_tag_filter.category_slugs;
                let tag_slugs = category_tag_filter.tag_slugs;

                match (!category_slugs.is_empty(), !tag_slugs.is_empty()) {
                    (true, true) => {
                        query.push("JOIN article.categories AS c ON a.id = c.article_id JOIN article.tags AS d ON d.id = c.tag_id WHERE c.category_slug = ANY(");
                        query.push_bind(category_slugs);
                        query.push(") AND d.tag_slug = ANY(");
                        query.push_bind(tag_slugs);
                        query.push(")");
                    }
                    (true, false) => {
                        query.push("JOIN article.categories AS c ON a.id = c.article_id WHERE c.category_slug = ANY(");
                        query.push_bind(category_slugs);
                        query.push(")");
                    }
                    (false, true) => {
                        query.push(
                            "JOIN article.tags AS d ON a.id = d.article_id WHERE d.tag_slug = ANY(",
                        );
                        query.push_bind(tag_slugs);
                        query.push(")");
                    }
                    (false, false) => {}
                }
            }
            None => {}
        }

        query.push(" LIMIT ");
        query.push_bind(page_size);
        query.push(" OFFSET ");
        query.push_bind(offset);
        let res = query.build_query_as().fetch_all(db).await?;
        Ok(res)
    }
    pub async fn get_one(
        db: &PgPool,
        identifier: chaserland_protos::article::v1::get_article_request::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<Option<Self>> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM ");
        match (public_only, with_content) {
            (true, true) => {
                query.push("article.public_articles ");
            }
            (true, false) => {
                query.push("article.public_articles_meta ");
            }
            (false, true) => {
                query.push("article.articles ");
            }
            (false, false) => {
                query.push("article.articles_meta ");
            }
        }
        match identifier {
            chaserland_protos::article::v1::get_article_request::Identifier::Id(id) => {
                query.push("WHERE id = ");
                query.push_bind(id);
            }
            chaserland_protos::article::v1::get_article_request::Identifier::Slug(slug) => {
                query.push("WHERE slug = ");
                query.push_bind(slug);
            }
        }

        let article = query.build_query_as().fetch_optional(db).await?;
        Ok(article)
    }
    pub async fn get_content(
        db: &PgPool,
        identifier: chaserland_protos::article::v1::get_article_content_request::Identifier,
    ) -> Result<Option<String>> {
        let mut query =
            QueryBuilder::<Postgres>::new("SELECT content FROM article.articles WHERE ");
        match identifier {
            chaserland_protos::article::v1::get_article_content_request::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug);
            }
            chaserland_protos::article::v1::get_article_content_request::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id);
            }
        };
        let content = query.build_query_scalar().fetch_optional(db).await?;
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

        query.build().execute(&mut **transaction).await?;

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

        query.build().execute(&mut **transaction).await?;

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
    pub async fn publish_by_id(
        transaction: &mut Transaction<'_, Postgres>,
        id: i32,
    ) -> Result<u64> {
        let row_count = sqlx::query(
            "UPDATE article.articles SET published_at = CURRENT_TIMESTAMP WHERE id = $1",
        )
        .bind(id)
        .execute(&mut **transaction)
        .await?
        .rows_affected();
        Ok(row_count)
    }
    pub async fn delete_by_id(transaction: &mut Transaction<'_, Postgres>, id: i32) -> Result<u64> {
        let row_count =
            sqlx::query("UPDATE article.articles SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(id)
                .execute(&mut **transaction)
                .await?
                .rows_affected();
        Ok(row_count)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullArticle {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub series: Option<Series>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
}

impl FullArticle {
    pub async fn get_one(
        db: &PgPool,
        identifier: chaserland_protos::article::v1::get_article_request::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<Option<Self>> {
        let article = match Article::get_one(db, identifier, public_only, with_content).await? {
            Some(val) => val,
            None => {
                return Ok(None);
            }
        };

        let series = Series::get_by_article_id(db, article.id).await?;
        let categories = Category::get_all_by_article_id(db, article.id).await?;
        let tags = Tag::get_many_by_article_id(db, article.id).await?;

        let res = Self {
            id: article.id,
            title: article.title,
            slug: article.slug,
            description: article.description,
            content: article.content,
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
    pub async fn get_many(
        db: &PgPool,
        page: i32,
        page_size: i32,
        public_only: bool,
        with_content: bool,
        filter: Option<chaserland_protos::article::v1::get_articles_request::Filter>,
    ) -> Result<Vec<Self>> {
        let articles =
            Article::get_many(db, page, page_size, public_only, with_content, filter).await?;

        let mut res = Vec::new();
        for article in articles {
            let series = Series::get_by_article_id(db, article.id).await?;
            let categories = Category::get_all_by_article_id(db, article.id).await?;
            let tags = Tag::get_many_by_article_id(db, article.id).await?;
            res.push(FullArticle {
                id: article.id,
                title: article.title,
                slug: article.slug,
                description: article.description,
                content: article.content,
                created_at: article.created_at,
                published_at: article.published_at,
                updated_at: article.updated_at,
                deleted_at: article.deleted_at,
                series,
                categories,
                tags,
            });
        }

        Ok(res)
    }
}

impl Into<chaserland_protos::article::v1::Article> for FullArticle {
    fn into(self) -> chaserland_protos::article::v1::Article {
        chaserland_protos::article::v1::Article {
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

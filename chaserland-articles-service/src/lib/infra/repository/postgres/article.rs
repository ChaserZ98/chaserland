use crate::domain::entity::{article, series, category, tag};
use crate::domain::repository::article::ArticlesFilter;
use crate::domain::repository::article::error;
use async_trait::async_trait;
use chaserland_common::pagination::{Offset, Page, PageSize};
use sqlx::{PgPool, Postgres, QueryBuilder};
use crate::domain::repository::article::ArticleRepository;

#[derive(sqlx::FromRow)]
pub struct PgArticle {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    #[sqlx(default)]
    pub content: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub series_id: Option<i32>,
    #[sqlx(default)]
    pub category_ids: Vec<i32>,
    #[sqlx(default)]
    pub tag_ids: Vec<i32>,
    pub version: chrono::DateTime<chrono::Utc>,
}

impl TryInto<article::Article> for PgArticle {
    type Error = String;

    fn try_into(self) -> Result<article::Article, Self::Error> {
        let id = self.id.try_into()?;
        let title = self.title.try_into()?;
        let description = self.description.into();
        let content = match self.content {
            Some(content) => Some(content.into()),
            None => None,
        };
        let created_at = self.created_at.into();
        let published_at = self.published_at.map(|published_at| published_at.into());
        let updated_at = self.updated_at.into();
        let deleted_at = self.deleted_at.map(|deleted_at| deleted_at.into());
        let series_id = match self.series_id {
            Some(series_id) => Some(series_id.try_into()?),
            None => None
        };

        let category_ids = self.category_ids.iter().map(|category_id| category_id.try_into()).collect::<Result<Vec<_>, _>>()?;
        let tag_ids = self.tag_ids.iter().map(|tag_id| tag_id.try_into()).collect::<Result<Vec<_>, _>>()?;
        let version = self.version.into();

        let mut article = article::Article::new(id, title, description, content, created_at, updated_at, series_id, category_ids, tag_ids, version);
        article.published_at = published_at;
        article.deleted_at = deleted_at;

        Ok(article)
    }
}

pub struct PgArticleRepository {
    pool: PgPool,
}

impl PgArticleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticleRepository for PgArticleRepository {
    async fn create(
        &self,
        article: article::ArticleCreate,
    ) -> Result<article::Article, error::CreateArticleError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| error::CreateArticleError::Unknown(why.into()))?;

        let title = article.title.value();
        let slug = article.title.as_slug().value();
        let description = article.description.value();
        let content = match article.content {
            Some(content) => content.value(),
            None => "".to_string(),
        };
        let series_id = match article.series_id {
            Some(series_id) => Some(series_id.value()),
            None => None,
        };
        let version = article.version.value();

        let pg_article: PgArticle = sqlx::query_as(
            "INSERT INTO article.articles (title, slug, description, content, series_id, version) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        ).bind(title)
        .bind(slug.clone())
        .bind(description)
        .bind(content)
        .bind(series_id)
        .bind(version)
        .fetch_one(&mut *tx)
        .await.map_err(|why| 
            match why {
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => 
                    error::CreateArticleError::DuplicateSlug(slug),
                _ => error::CreateArticleError::Unknown(why.into()),
            }
        )?;

        if !article.category_ids.is_empty() {
            let mut query = QueryBuilder::new(
                "INSERT INTO article.article_categories (article_id, category_id) ",
            );
            query.push_values(&article.category_ids, |mut b, category_id| {
                b.push_bind(pg_article.id).push_bind(category_id.value());
            });

            query.push(" ON CONFLICT (article_id, category_id) DO NOTHING");

            query
                .build()
                .execute(&mut *tx)
                .await
                .map_err(|why| error::CreateArticleError::Unknown(why.into()))?;
        }

        if !article.tag_ids.is_empty() {
            let mut query =
                QueryBuilder::new("INSERT INTO article.article_tags (article_id, tag_id) ");
            query.push_values(&article.tag_ids, |mut b, tag_id| {
                b.push_bind(pg_article.id).push_bind(tag_id.value());
            });

            query.push(" ON CONFLICT (article_id, tag_id) DO NOTHING");

            query
                .build()
                .execute(&mut *tx)
                .await
                .map_err(|why| error::CreateArticleError::Unknown(why.into()))?;
        }

        tx.commit()
            .await
            .map_err(|why| error::CreateArticleError::Unknown(why.into()))?;

        let mut new_article: article::Article = pg_article.try_into().map_err(|why: String| error::CreateArticleError::Unknown(anyhow::anyhow!(why)))?;
        new_article.category_ids = article.category_ids;
        new_article.tag_ids = article.tag_ids;

        Ok(new_article)
    }
    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<article::Article, error::GetArticleError> {
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
        match identifier.clone() {
            article::Identifier::Id(id) => {
                query.push("WHERE id = ");
                query.push_bind(id.value());
            }
            article::Identifier::Slug(slug) => {
                query.push("WHERE slug = ");
                query.push_bind(slug.value());
            }
        }

        let pg_article: Option<PgArticle> = query.build_query_as().fetch_optional(&self.pool).await.map_err(|why| error::GetArticleError::Unknown(why.into()))?;
        if pg_article.is_none() {
            return Err(error::GetArticleError::NotFound(identifier));
        }

        let pg_article = pg_article.unwrap();

        let category_ids: Vec<i32> = sqlx::query_scalar(
            "SELECT category_id FROM article.article_categories WHERE article_id = $1").bind(pg_article.id).fetch_all(&self.pool).await.map_err(|why| error::GetArticleError::Unknown(why.into()))?;

        let tag_ids: Vec<i32> = sqlx::query_scalar("SELECT tag_id FROM article.article_tags WHERE article_id = $1").bind(pg_article.id).fetch_all(&self.pool).await.map_err(|why| error::GetArticleError::Unknown(why.into()))?;


        let mut article: article::Article = pg_article.try_into().map_err(|why: String| error::GetArticleError::Unknown(anyhow::anyhow!(why)))?;
        article.category_ids = category_ids.iter().map(|v| v.try_into().unwrap()).collect();
        article.tag_ids = tag_ids.iter().map(|v| v.try_into().unwrap()).collect();

        Ok(article)
    }
    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<article::Article>, error::GetArticleError> {
        let offset = Offset::from((page, page_size));
        let mut query = QueryBuilder::<Postgres>::new("SELECT a.*, COALESCE(c.category_ids, '{}') as category_ids, COALESCE(d.tag_ids, '{}') as tag_ids FROM ");

        match (public_only, with_content) {
            (true, true) => {
                query.push("article.public_articles");
            }
            (true, false) => {
                query.push("article.public_articles_meta");
            }
            (false, true) => {
                query.push("article.articles");
            }
            (false, false) => {
                query.push("article.articles_meta");
            }
        }
        query.push(" AS a");
        query.push(" LEFT JOIN article.series AS b on a.series_id = b.id");
        query.push(" LEFT JOIN (SELECT article_id, ARRAY_AGG(DISTINCT category_id) AS category_ids FROM article.article_categories GROUP BY article_id) AS c ON a.id = c.article_id");
        query.push(" LEFT JOIN (SELECT article_id, ARRAY_AGG(DISTINCT tag_id) AS tag_ids FROM article.article_tags GROUP BY article_id) AS d ON a.id = d.article_id");

        let mut has_prev_condition = false;

        if let Some(filter) = filter {
            query.push(" WHERE ");
            if let Some(series_identifier) = filter.series_identifier() {
                has_prev_condition = true;
                match series_identifier {
                    series::Identifier::Id(id) => {
                        query.push("b.id = ");
                        query.push_bind(id.value());
                    }
                    series::Identifier::Slug(slug) => {
                        query.push("b.slug = ");
                        query.push_bind(slug.value());
                    }
                }
            }
            if !filter.category_ids().is_empty() {
                match has_prev_condition {
                    true => {
                        query.push(" AND ");
                    },
                    false => {
                        has_prev_condition = true;
                    }
                }
                query.push(" c.category_ids @> ");
                query.push_bind(filter.category_ids().iter().map(|v| v.value()).collect::<Vec<_>>());
            }

            if !filter.tag_ids().is_empty() {
                match has_prev_condition {
                    true => {
                        query.push(" AND ");
                    },
                    _ => {}
                }
                query.push(" d.tag_ids @> ");
                query.push_bind(filter.tag_ids().iter().map(|v| v.value()).collect::<Vec<_>>());
            }
        }

        query.push(" LIMIT ");
        query.push_bind(page_size.value());
        query.push(" OFFSET ");
        query.push_bind(offset.value());

        let articles: Vec<PgArticle> = query.build_query_as().fetch_all(&self.pool).await.map_err(|why| error::GetArticleError::Unknown(why.into()))?;

        let res = articles.into_iter().map(|v| v.try_into()).collect::<Result<Vec<article::Article>, String>>().map_err(|why| error::GetArticleError::Unknown(anyhow::anyhow!(why)))?;

        Ok(res)
    }
    async fn set_series(&self, id: article::Id, series_id: series::Id, version: article::Version) -> Result<(), error::SetSeriesError> {
        let mut tx = self.pool.begin().await.map_err(|why| error::SetSeriesError::Unknown(why.into()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET series_id = $1, version = NOW() WHERE id = $2 RETURNING (SELECT version FROM article.articles WHERE id = $2) as version").bind(series_id.value()).bind(id.value()).fetch_optional(&mut *tx).await.map_err(|why|
            match why {
                sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => error::SetSeriesError::SeriesNotFound(series_id),
                _ => error::SetSeriesError::Unknown(why.into())
            }
        )?;

        if db_article_version.is_none() {
            return Err(error::SetSeriesError::NotFound(id));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(error::SetSeriesError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            });
        }

        tx.commit().await.map_err(|why| error::SetSeriesError::Unknown(why.into()))?;

        Ok(())
    }
    async fn remove_series(&self, id: article::Id, version: article::Version) -> Result<(), error::RemoveSeriesError> {
        let mut tx = self.pool.begin().await.map_err(|why| error::RemoveSeriesError::Unknown(why.into()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET series_id = NULL, version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_optional(&mut *tx).await.map_err(|why| error::RemoveSeriesError::Unknown(why.into()))?;

        if db_article_version.is_none() {
            return Err(error::RemoveSeriesError::NotFound(id));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(error::RemoveSeriesError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            });
        }

        tx.commit().await.map_err(|why| error::RemoveSeriesError::Unknown(why.into()))?;

        Ok(())
    }
    async fn add_category(&self, id: article::Id, category_id: category::Id, version: article::Version) -> Result<(), error::AddCategoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| error::AddCategoryError::Unknown(why.into()))?;

        sqlx::query("INSERT INTO article.article_categories (article_id, category_id) VALUES ($1, $2) ON CONFLICT (article_id, category_id) DO NOTHING")
        .bind(id.value())
        .bind(category_id.value())
        .execute(&mut *tx)
        .await
        .map_err(
            |why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return error::AddCategoryError::Unknown(why.into());
                }
                let db_err = why.as_database_error().unwrap();
                if !db_err.is_foreign_key_violation() {
                    return error::AddCategoryError::Unknown(why.into());
                }
                match db_err.constraint() {
                    Some("article_categories_article_id_fkey") => error::AddCategoryError::ArticleNotFound(id),
                    Some("article_categories_category_id_fkey") => error::AddCategoryError::CategoryNotFound(category_id),
                    _ => error::AddCategoryError::Unknown(why.into())
                }
            }
        )?;
        
        let db_article_version: chrono::DateTime<chrono::Utc> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_one(&mut *tx).await.map_err(|why| error::AddCategoryError::Unknown(why.into()))?;

        if db_article_version != version.value() {
            return Err(error::AddCategoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            });
        }

        tx.commit().await.map_err(|why| error::AddCategoryError::Unknown(why.into()))?;

        Ok(())
    }
    async fn remove_category(&self, id: article::Id, category_id: category::Id, version: article::Version) -> Result<(), error::RemoveCategoryError> {
        todo!()
    }
    async fn add_tag(&self, id: article::Id, tag_id: tag::Id, version: article::Version) -> Result<(), error::AddTagError> {
        todo!()
    }
    async fn remove_tag(&self, id: article::Id, tag_id: tag::Id, version: article::Version) -> Result<(), error::RemoveTagError> {
        todo!()
    }
    async fn publish(&self, id: article::Id, published_at: article::PublishedAt, version: article::Version) -> Result<(), error::PublishArticleError> {
        let mut tx = self.pool.begin().await.map_err(|why| error::PublishArticleError::Unknown(why.into()))?;
        
        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET published_at = $1, version = NOW() WHERE id = $2 RETURNING (SELECT version FROM article.articles WHERE id = $2) as version")
        .bind(published_at.value())
        .bind(id.value())
        .fetch_optional(&mut *tx)
        .await.map_err(|why| error::PublishArticleError::Unknown(why.into()))?;

        if db_article_version.is_none() {
            return Err(error::PublishArticleError::NotFound(id));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(error::PublishArticleError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            });
        }

        tx.commit().await.map_err(|why| error::PublishArticleError::Unknown(why.into()))?;

        Ok(())
    }
    async fn unpublish(&self, id: article::Id, version: article::Version) -> Result<(), error::UnpublishArticleError> {
        let mut tx = self.pool.begin().await.map_err(|why| error::UnpublishArticleError::Unknown(why.into()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET published_at = NULL, version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version")
        .bind(id.value())
        .fetch_optional(&mut *tx)
        .await.map_err(|why| error::UnpublishArticleError::Unknown(why.into()))?;

        if db_article_version.is_none() {
            return Err(error::UnpublishArticleError::NotFound(id));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(error::UnpublishArticleError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            });
        }

        tx.commit().await.map_err(|why| error::UnpublishArticleError::Unknown(why.into()))?;

        Ok(())
    }
    async fn soft_delete(&self, id: article::Id, deleted_at: article::DeletedAt, version: article::Version) -> Result<(), error::SoftDeleteArticleError> {
        let mut tx = self.pool.begin().await.map_err(|why| error::SoftDeleteArticleError::Unknown(why.into()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET deleted_at = $1, version = NOW() WHERE id = $2 RETURNING (SELECT version FROM article.articles WHERE id = $2) as version")
        .bind(deleted_at.value())
        .bind(id.value())
        .fetch_optional(&mut *tx)
        .await.map_err(|why| error::SoftDeleteArticleError::Unknown(why.into()))?;

        if db_article_version.is_none() {
            return Err(error::SoftDeleteArticleError::NotFound(id));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(error::SoftDeleteArticleError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            });
        }

        tx.commit().await.map_err(|why| error::SoftDeleteArticleError::Unknown(why.into()))?;

        Ok(())
    }
    async fn revoke_soft_delete(&self, id: article::Id, version: article::Version) -> Result<(), error::RevokeSoftDeleteError> {
        let mut tx = self.pool.begin().await.map_err(|why| error::RevokeSoftDeleteError::Unknown(why.into()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET deleted_at = NULL, version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version")
        .bind(id.value())
        .fetch_optional(&mut *tx)
        .await.map_err(|why| error::RevokeSoftDeleteError::Unknown(why.into()))?;

        if db_article_version.is_none() {
            return Err(error::RevokeSoftDeleteError::NotFound(id));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(error::RevokeSoftDeleteError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            });
        }

        tx.commit().await.map_err(|why| error::RevokeSoftDeleteError::Unknown(why.into()))?;

        Ok(())
    }
    async fn delete(
        &self,
        identifier: article::Identifier,
    ) -> Result<(), error::DeleteArticleError> {
        let mut tx = self.pool.begin().await.map_err(|why| error::DeleteArticleError::Unknown(why.into()))?;

        let mut query = QueryBuilder::<Postgres>::new("DELETE FROM article.articles WHERE ");

        match identifier.clone() {
            article::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            article::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        };

        let res = query.build().execute(&mut *tx).await.map_err(|why| error::DeleteArticleError::Unknown(why.into()))?;
        
        if res.rows_affected() == 0 {
            return Err(error::DeleteArticleError::NotFound(identifier));
        }

        tx.commit().await.map_err(|why| error::DeleteArticleError::Unknown(why.into()))?;

        Ok(())
    }
}

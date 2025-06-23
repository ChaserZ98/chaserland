use crate::domain::tag::vo as tag;
use crate::domain::category::vo as category;
use crate::domain::series::vo as series;
use chaserland_common::pagination::Pagination;
use sqlx::{PgPool, Postgres, QueryBuilder};
use crate::domain::article::{repository::{ArticleRepository, ArticleRepositoryError, ArticlesFilter}, vo as article, entity::Article};

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

impl TryInto<Article> for PgArticle {
    type Error = String;

    fn try_into(self) -> Result<Article, Self::Error> {
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

        let category_ids = self.category_ids.iter().map(|category_id| (*category_id).try_into()).collect::<Result<Vec<_>, _>>()?;
        let tag_ids = self.tag_ids.iter().map(|tag_id| (*tag_id).try_into()).collect::<Result<Vec<_>, _>>()?;
        let version = self.version.into();

        let mut article = Article::new(id, title, description, content, created_at, updated_at, series_id, category_ids, tag_ids, version);
        article.published_at = published_at;
        article.deleted_at = deleted_at;

        Ok(article)
    }
}

#[derive(Clone)]
pub struct PgArticleRepository {
    pool: PgPool,
}

impl PgArticleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ArticleRepository for PgArticleRepository {
    async fn create(
        &self,
        article: article::NewArticle,
    ) -> Result<Article, ArticleRepositoryError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let title = article.title.value();
        let slug = article.title.as_slug();
        let description = article.description.value();
        let content = match &article.content {
            Some(content) => content.value(),
            None => &"".to_string(),
        };
        let series_id = match &article.series_id {
            Some(series_id) => Some(series_id.value()),
            None => None,
        };
        let version = article.version().value();

        let pg_article: PgArticle = sqlx::query_as(
            "INSERT INTO article.articles (title, slug, description, content, series_id, version) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(title)
        .bind(slug.value())
        .bind(description)
        .bind(content)
        .bind(series_id)
        .bind(version)
        .fetch_one(&mut *tx)
        .await.map_err(|why| 
            {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why.into());
                }
                let db_err = why.as_database_error().unwrap();
                if db_err.is_unique_violation() {
                    return ArticleRepositoryError::DuplicateArticleSlug(article.title.as_slug());
                }
                if db_err.is_foreign_key_violation() && db_err.constraint() == Some("articles_series_id_fkey") {
                    return ArticleRepositoryError::SeriesNotFound(article.series_id.unwrap().as_identifier());
                } 
                ArticleRepositoryError::Sqlx(why.into())
            }
        )?;

        for category_id in &article.category_ids {
            sqlx::query("INSERT INTO article.article_categories (article_id, category_id) VALUES ($1, $2) ON CONFLICT (article_id, category_id) DO NOTHING")
            .bind(pg_article.id)
            .bind(category_id.value())
            .execute(&mut *tx)
            .await
            .map_err(|why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why.into());
                }
                let db_err = why.as_database_error().unwrap();
                if db_err.is_foreign_key_violation() && db_err.constraint() == Some("article_categories_category_id_fkey") {
                    return ArticleRepositoryError::CategoryNotFound(category_id.as_identifier());
                }
                ArticleRepositoryError::Sqlx(why.into())
            })?;
        }

        for tag_id in &article.tag_ids {
            sqlx::query("INSERT INTO article.article_tags (article_id, tag_id) VALUES ($1, $2) ON CONFLICT (article_id, tag_id) DO NOTHING")
            .bind(pg_article.id)
            .bind(tag_id.value())
            .execute(&mut *tx)
            .await
            .map_err(|why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why.into());
                }
                let db_err = why.as_database_error().unwrap();
                if db_err.is_foreign_key_violation() && db_err.constraint() == Some("article_tags_tag_id_fkey") {
                    return ArticleRepositoryError::TagNotFound(tag_id.as_identifier());
                }
                ArticleRepositoryError::Sqlx(why.into())
            })?;
        }

        tx.commit()
            .await
            .map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let mut new_article: Article = pg_article.try_into().map_err(|why| ArticleRepositoryError::DOConversion(why))?;
        new_article.category_ids = article.category_ids;
        new_article.tag_ids = article.tag_ids;

        Ok(new_article)
    }
    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<Article, ArticleRepositoryError> {
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
        match &identifier {
            article::Identifier::Id(id) => {
                query.push("WHERE id = ");
                query.push_bind(id.value());
            }
            article::Identifier::Slug(slug) => {
                query.push("WHERE slug = ");
                query.push_bind(slug.value());
            }
        }

        let pg_article: Option<PgArticle> = query.build_query_as().fetch_optional(&self.pool).await?;

        if pg_article.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(identifier));
        }

        let pg_article = pg_article.unwrap();

        let category_ids: Vec<i32> = sqlx::query_scalar(
            "SELECT category_id FROM article.article_categories WHERE article_id = $1").bind(pg_article.id).fetch_all(&self.pool).await?;

        let tag_ids: Vec<i32> = sqlx::query_scalar("SELECT tag_id FROM article.article_tags WHERE article_id = $1").bind(pg_article.id).fetch_all(&self.pool).await?;

        let mut article: Article = pg_article.try_into().map_err(|why| ArticleRepositoryError::DOConversion(why))?;
        article.category_ids = category_ids.iter().map(|v| (*v).try_into()).collect::<Result<Vec<_>, _>>().map_err(|why| ArticleRepositoryError::DOConversion(why))?;
        article.tag_ids = tag_ids.iter().map(|v| (*v).try_into()).collect::<Result<Vec<_>, _>>().map_err(|why| ArticleRepositoryError::DOConversion(why))?;

        Ok(article)
    }
    async fn get_many(
        &self,
        pagination: Pagination,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<Article>, ArticleRepositoryError> {
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
        query.push_bind(pagination.page_size.value());
        query.push(" OFFSET ");
        query.push_bind(pagination.as_offset().value());

        let articles: Vec<PgArticle> = query.build_query_as().fetch_all(&self.pool).await?;

        let res = articles.into_iter().map(|v| v.try_into()).collect::<Result<Vec<Article>, String>>().map_err(|why| ArticleRepositoryError::DOConversion(why))?;

        Ok(res)
    }
    async fn set_series(&self, id: article::Id, series_id: series::Id, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET series_id = $1, version = NOW() WHERE id = $2 RETURNING (SELECT version FROM article.articles WHERE id = $2) as version").bind(series_id.value()).bind(id.value()).fetch_optional(&mut *tx).await.map_err(|why|
            match why {
                sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => ArticleRepositoryError::SeriesNotFound(series_id.as_identifier()),
                _ => ArticleRepositoryError::Sqlx(why.into())
            }
        )?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn remove_series(&self, id: article::Id, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET series_id = NULL, version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_optional(&mut *tx).await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn add_category(&self, id: article::Id, category_id: category::Id, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        sqlx::query("INSERT INTO article.article_categories (article_id, category_id) VALUES ($1, $2) ON CONFLICT (article_id, category_id) DO NOTHING")
        .bind(id.value())
        .bind(category_id.value())
        .execute(&mut *tx)
        .await
        .map_err(
            |why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why.into());
                }
                let db_err = why.as_database_error().unwrap();
                if !db_err.is_foreign_key_violation() {
                    return ArticleRepositoryError::Sqlx(why.into());
                }
                match db_err.constraint() {
                    Some("article_categories_article_id_fkey") => ArticleRepositoryError::ArticleNotFound(id.as_identifier()),
                    Some("article_categories_category_id_fkey") => ArticleRepositoryError::CategoryNotFound(category_id.as_identifier()),
                    _ => ArticleRepositoryError::Sqlx(why.into())
                }
            }
        )?;
        
        let db_article_version: chrono::DateTime<chrono::Utc> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_one(&mut *tx).await?;

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn remove_category(&self, id: article::Id, category_id: category::Id, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_optional(&mut *tx).await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        let rows_affected = sqlx::query("DELETE FROM article.article_categories WHERE article_id = $1 AND category_id = $2")
        .bind(id.value())
        .bind(category_id.value())
        .execute(&mut *tx)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(ArticleRepositoryError::CategoryNotFound(category_id.as_identifier()).into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn add_tag(&self, id: article::Id, tag_id: tag::Id, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        sqlx::query("INSERT INTO article.article_tags (article_id, tag_id) VALUES ($1, $2) ON CONFLICT (article_id, tag_id) DO NOTHING")
        .bind(id.value())
        .bind(tag_id.value())
        .execute(&mut *tx)
        .await
        .map_err(
            |why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why.into());
                }
                let db_err = why.as_database_error().unwrap();
                if !db_err.is_foreign_key_violation() {
                    return ArticleRepositoryError::Sqlx(why.into());
                }
                match db_err.constraint() {
                    Some("article_tags_article_id_fkey") => ArticleRepositoryError::ArticleNotFound(id.as_identifier()),
                    Some("article_tags_tag_id_fkey") => ArticleRepositoryError::TagNotFound(tag_id.as_identifier()),
                    _ => ArticleRepositoryError::Sqlx(why.into())
                }
            }
        )?;
        
        let db_article_version: chrono::DateTime<chrono::Utc> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_one(&mut *tx).await?;

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn remove_tag(&self, id: article::Id, tag_id: tag::Id, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_optional(&mut *tx).await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        let rows_affected = sqlx::query("DELETE FROM article.article_tags WHERE article_id = $1 AND tag_id = $2")
        .bind(id.value())
        .bind(tag_id.value())
        .execute(&mut *tx)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(ArticleRepositoryError::TagNotFound(tag_id.as_identifier()).into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn publish(&self, id: article::Id, published_at: article::PublishedAt, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;
        
        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET published_at = $1, version = NOW() WHERE id = $2 RETURNING (SELECT version FROM article.articles WHERE id = $2) as version")
        .bind(published_at.value())
        .bind(id.value())
        .fetch_optional(&mut *tx)
        .await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn unpublish(&self, id: article::Id, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET published_at = NULL, version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version")
        .bind(id.value())
        .fetch_optional(&mut *tx)
        .await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn soft_delete(&self, id: article::Id, deleted_at: article::DeletedAt, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET deleted_at = $1, version = NOW() WHERE id = $2 RETURNING (SELECT version FROM article.articles WHERE id = $2) as version")
        .bind(deleted_at.value())
        .bind(id.value())
        .fetch_optional(&mut *tx)
        .await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn revoke_soft_delete(&self, id: article::Id, version: article::Version) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET deleted_at = NULL, version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version")
        .bind(id.value())
        .fetch_optional(&mut *tx)
        .await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::VersionMismatch{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
    async fn delete(
        &self,
        identifier: article::Identifier,
    ) -> Result<(), ArticleRepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        let mut query = QueryBuilder::<Postgres>::new("DELETE FROM article.articles WHERE ");

        match &identifier {
            article::Identifier::Id(id) => {
                query.push("id = ");
                query.push_bind(id.value());
            }
            article::Identifier::Slug(slug) => {
                query.push("slug = ");
                query.push_bind(slug.value());
            }
        };

        let res = query.build().execute(&mut *tx).await?;
        
        if res.rows_affected() == 0 {
            return Err(ArticleRepositoryError::ArticleNotFound(identifier));
        }

        tx.commit().await.map_err(|why| ArticleRepositoryError::Transaction(why.to_string()))?;

        Ok(())
    }
}

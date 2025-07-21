use crate::{
    domain::{article::{vo::TimestampVersion, repository::{ArticleRepository, ArticleRepositoryError}, vo as article, entity::Article}, tag::vo as tag, category::vo as category}, infra::postgres::po::PgArticle
};
use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder, Transaction};

#[derive(Clone)]
pub struct PgArticleRepository {}

impl PgArticleRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl ArticleRepository for PgArticleRepository {
    type DB = Postgres;

    async fn create(
        &self,
        article: article::NewArticle,
        tx: &mut Transaction<'static, Self::DB>,
    ) -> Result<Article, ArticleRepositoryError> {
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
        let created_at = article.created_at.value();
        let updated_at = article.updated_at.value();
        let version = article.version().value();

        let pg_article: PgArticle = sqlx::query_as(
            "INSERT INTO article.articles (title, slug, description, content, series_id, created_at, updated_at, version) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(title)
        .bind(slug.value())
        .bind(description)
        .bind(content)
        .bind(series_id)
        .bind(created_at)
        .bind(updated_at)
        .bind(version)
        .fetch_one(&mut **tx)
        .await.map_err(|why| 
            {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why);
                }
                let db_err = why.as_database_error().unwrap();
                if db_err.is_unique_violation() {
                    return ArticleRepositoryError::DuplicateArticleSlug(article.title.as_slug());
                }
                if db_err.is_foreign_key_violation() && db_err.constraint() == Some("articles_series_id_fkey") {
                    return ArticleRepositoryError::SeriesNotFound(article.series_id.unwrap().as_identifier());
                } 
                ArticleRepositoryError::Sqlx(why)
            }
        )?;

        for category_id in &article.category_ids {
            sqlx::query("INSERT INTO article.article_categories (article_id, category_id) VALUES ($1, $2) ON CONFLICT (article_id, category_id) DO NOTHING")
            .bind(pg_article.id)
            .bind(category_id.value())
            .execute(&mut **tx)
            .await
            .map_err(|why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why);
                }
                let db_err = why.as_database_error().unwrap();
                if db_err.is_foreign_key_violation() && db_err.constraint() == Some("article_categories_category_id_fkey") {
                    return ArticleRepositoryError::CategoryNotFound(category_id.as_identifier());
                }
                ArticleRepositoryError::Sqlx(why)
            })?;
        }

        for tag_id in &article.tag_ids {
            sqlx::query("INSERT INTO article.article_tags (article_id, tag_id) VALUES ($1, $2) ON CONFLICT (article_id, tag_id) DO NOTHING")
            .bind(pg_article.id)
            .bind(tag_id.value())
            .execute(&mut **tx)
            .await
            .map_err(|why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why);
                }
                let db_err = why.as_database_error().unwrap();
                if db_err.is_foreign_key_violation() && db_err.constraint() == Some("article_tags_tag_id_fkey") {
                    return ArticleRepositoryError::TagNotFound(tag_id.as_identifier());
                }
                ArticleRepositoryError::Sqlx(why)
            })?;
        }

        let mut new_article: Article = pg_article.try_into().map_err(|why| ArticleRepositoryError::DOConversion(why))?;
        new_article.category_ids = article.category_ids;
        new_article.tag_ids = article.tag_ids;

        Ok(new_article)
    }
    
    async fn get_one(&self,identifier: article::Identifier, tx: &mut Transaction<'static, Self::DB>) -> Result<(Article,TimestampVersion),ArticleRepositoryError> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM article.articles ");
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

        let pg_article: Option<PgArticle> = query.build_query_as().fetch_optional(&mut **tx).await?;

        if pg_article.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(identifier));
        }

        let pg_article = pg_article.unwrap();

        let category_ids: Vec<i32> = sqlx::query_scalar(
            "SELECT category_id FROM article.article_categories WHERE article_id = $1").bind(pg_article.id).fetch_all(&mut **tx).await?;

        let tag_ids: Vec<i32> = sqlx::query_scalar("SELECT tag_id FROM article.article_tags WHERE article_id = $1").bind(pg_article.id).fetch_all(&mut **tx).await?;

        let (mut article, version) = pg_article.try_into().map_err(|why| ArticleRepositoryError::DOConversion(why))?;
        article.category_ids = category_ids.iter().map(|v| (*v).try_into()).collect::<Result<Vec<_>, _>>().map_err(|why| ArticleRepositoryError::DOConversion(why))?;
        article.tag_ids = tag_ids.iter().map(|v| (*v).try_into()).collect::<Result<Vec<_>, _>>().map_err(|why| ArticleRepositoryError::DOConversion(why))?;

        Ok((article, version))
    }

    async fn save(&self, article: Article, version: article::TimestampVersion, tx: &mut Transaction<'static, Self::DB>) -> Result<(), ArticleRepositoryError> {
        let id = article.id.value();
        let title = article.title.value();
        let slug = article.slug().value();
        let description = article.description.value();
        let content = article.content.as_ref().map(|x| x.value());
        let series_id = article.series_id.map(|x| x.value());
        let published_at = article.published_at.map(|x| x.value());
        let updated_at = article.updated_at.value();
        let deleted_at = article.deleted_at.map(|x| x.value());

        let db_article_version: Option<DateTime<Utc>> = sqlx::query_scalar("UPDATE article.articles SET title = $1, slug = $2, description = $3, content = $4, series_id = $5, published_at = $6, updated_at = $7, deleted_at = $8, version = NOW() WHERE id = $9 RETURNING (SELECT version FROM article.articles WHERE id = $9) as version").bind(title).bind(slug).bind(description).bind(content).bind(series_id).bind(published_at).bind(updated_at).bind(deleted_at).bind(id).fetch_optional(&mut **tx).await.map_err(|e| {
            if !matches!(e, sqlx::Error::Database(_)) {
                return ArticleRepositoryError::Sqlx(e);
            }
            let db_err = e.as_database_error().unwrap();
            if db_err.is_unique_violation() && db_err.constraint() == Some("articles_slug_key") {
                return ArticleRepositoryError::DuplicateArticleSlug(article.slug().clone());
            }
            if db_err.is_foreign_key_violation() && db_err.constraint() == Some("articles_series_id_fkey") {
                return ArticleRepositoryError::SeriesNotFound(article.series_id.unwrap().as_identifier());
            }
            e.into()
        })?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(article.id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::ConcurrentConflict{
                id: article.id,
                current_version: version,
                db_version: db_article_version.into()
            });
        }

        Ok(())
    }

    async fn add_category(&self, id: article::Id, category_id: category::Id, version: article::TimestampVersion, tx: &mut Transaction<'static, Self::DB>) -> Result<(), ArticleRepositoryError> {
        sqlx::query("INSERT INTO article.article_categories (article_id, category_id) VALUES ($1, $2) ON CONFLICT (article_id, category_id) DO NOTHING")
        .bind(id.value())
        .bind(category_id.value())
        .execute(&mut **tx)
        .await
        .map_err(
            |why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why);
                }
                let db_err = why.as_database_error().unwrap();
                if !db_err.is_foreign_key_violation() {
                    return ArticleRepositoryError::Sqlx(why);
                }
                match db_err.constraint() {
                    Some("article_categories_article_id_fkey") => ArticleRepositoryError::ArticleNotFound(id.as_identifier()),
                    Some("article_categories_category_id_fkey") => ArticleRepositoryError::CategoryNotFound(category_id.as_identifier()),
                    _ => ArticleRepositoryError::Sqlx(why)
                }
            }
        )?;
        
        let db_article_version: chrono::DateTime<chrono::Utc> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_one(&mut **tx).await?;

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::ConcurrentConflict{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        Ok(())
    }

    async fn remove_category(&self, id: article::Id, category_id: category::Id, version: article::TimestampVersion, tx: &mut Transaction<'static, Self::DB>) -> Result<(), ArticleRepositoryError> {
        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_optional(&mut **tx).await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::ConcurrentConflict{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        let rows_affected = sqlx::query("DELETE FROM article.article_categories WHERE article_id = $1 AND category_id = $2")
        .bind(id.value())
        .bind(category_id.value())
        .execute(&mut **tx)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(ArticleRepositoryError::CategoryNotFound(category_id.as_identifier()).into());
        }

        Ok(())
    }

    async fn add_tag(&self, id: article::Id, tag_id: tag::Id, version: article::TimestampVersion, tx: &mut Transaction<'static, Self::DB>) -> Result<(), ArticleRepositoryError> {
        sqlx::query("INSERT INTO article.article_tags (article_id, tag_id) VALUES ($1, $2) ON CONFLICT (article_id, tag_id) DO NOTHING")
        .bind(id.value())
        .bind(tag_id.value())
        .execute(&mut **tx)
        .await
        .map_err(
            |why| {
                if !matches!(why, sqlx::Error::Database(_)) {
                    return ArticleRepositoryError::Sqlx(why);
                }
                let db_err = why.as_database_error().unwrap();
                if !db_err.is_foreign_key_violation() {
                    return ArticleRepositoryError::Sqlx(why);
                }
                match db_err.constraint() {
                    Some("article_tags_article_id_fkey") => ArticleRepositoryError::ArticleNotFound(id.as_identifier()),
                    Some("article_tags_tag_id_fkey") => ArticleRepositoryError::TagNotFound(tag_id.as_identifier()),
                    _ => ArticleRepositoryError::Sqlx(why)
                }
            }
        )?;
        
        let db_article_version: chrono::DateTime<chrono::Utc> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_one(&mut **tx).await?;

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::ConcurrentConflict{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        Ok(())
    }

    async fn remove_tag(&self, id: article::Id, tag_id: tag::Id, version: article::TimestampVersion, tx: &mut Transaction<'static, Self::DB>) -> Result<(), ArticleRepositoryError> {
        let db_article_version: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("UPDATE article.articles SET version = NOW() WHERE id = $1 RETURNING (SELECT version FROM article.articles WHERE id = $1) as version").bind(id.value()).fetch_optional(&mut **tx).await?;

        if db_article_version.is_none() {
            return Err(ArticleRepositoryError::ArticleNotFound(id.as_identifier()));
        }

        let db_article_version = db_article_version.unwrap();

        if db_article_version != version.value() {
            return Err(ArticleRepositoryError::ConcurrentConflict{
                id,
                current_version: version,
                db_version: db_article_version.into()
            }.into());
        }

        let rows_affected = sqlx::query("DELETE FROM article.article_tags WHERE article_id = $1 AND tag_id = $2")
        .bind(id.value())
        .bind(tag_id.value())
        .execute(&mut **tx)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(ArticleRepositoryError::TagNotFound(tag_id.as_identifier()).into());
        }

        Ok(())
    }

    async fn delete(
        &self,
        identifier: article::Identifier,
        tx: &mut Transaction<'static, Self::DB>
    ) -> Result<(), ArticleRepositoryError> {
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

        let res = query.build().execute(&mut **tx).await?;
        
        if res.rows_affected() == 0 {
            return Err(ArticleRepositoryError::ArticleNotFound(identifier));
        }

        Ok(())
    }
}

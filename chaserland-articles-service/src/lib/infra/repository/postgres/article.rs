use crate::domain::entity::article;
use crate::domain::repository::article as domain_repo;
use async_trait::async_trait;
use chaserland_common::pagination::{Page, PageSize};
use sqlx::{PgPool, QueryBuilder};
use crate::domain::repository::article::ArticleRepository;

pub struct PgArticleRepository {
    pub pool: PgPool,
}

#[derive(sqlx::FromRow)]
pub struct PgArticle {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub series_id: Option<i32>,
}

#[async_trait]
impl ArticleRepository for PgArticleRepository {
    async fn create(
        &self,
        article: article::ArticleCreate,
    ) -> Result<article::Article, domain_repo::CreateArticleError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| domain_repo::CreateArticleError::Unknown(why.into()))?;

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

        let pg_article: PgArticle = sqlx::query_as(
            "INSERT INTO article.articles (title, slug, description, content, series_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        ).bind(title)
        .bind(slug.clone())
        .bind(description)
        .bind(content)
        .bind(series_id)
        .fetch_one(&mut *tx)
        .await.map_err(|why| 
            match why {
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => 
                    domain_repo::CreateArticleError::DuplicateSlug(slug),
                _ => domain_repo::CreateArticleError::Unknown(why.into()),
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
                .map_err(|why| domain_repo::CreateArticleError::Unknown(why.into()))?;
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
                .map_err(|why| domain_repo::CreateArticleError::Unknown(why.into()))?;
        }

        tx.commit()
            .await
            .map_err(|why| domain_repo::CreateArticleError::Unknown(why.into()))?;

        Ok(article::Article {
            id: pg_article.id.try_into().unwrap(),
            title: pg_article.title.try_into().unwrap(),
            slug: pg_article.slug.try_into().unwrap(),
            description: pg_article.description.into(),
            content: Some(pg_article.content.into()),
            created_at: pg_article.created_at.into(),
            published_at: pg_article.published_at.map(|v| v.into()),
            updated_at: pg_article.updated_at.into(),
            deleted_at: pg_article.deleted_at.map(|v| v.into()),
            series_id: pg_article.series_id.map(|v| v.try_into().unwrap()),
            category_ids: article.category_ids,
            tag_ids: article.tag_ids,
        })
    }

    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<article::Article, domain_repo::GetArticleError> {
        // Implement the logic for fetching a single article from the database
        unimplemented!()
    }

    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
        public_only: bool,
        with_content: bool,
        filter: Option<domain_repo::ArticlesFilter>,
    ) -> Result<Vec<article::Article>, domain_repo::GetArticleError> {
        // Implement the logic for fetching multiple articles from the database
        unimplemented!()
    }

    async fn delete(
        &self,
        identifier: article::Identifier,
    ) -> Result<(), domain_repo::DeleteArticleError> {
        // Implement the logic for deleting an article from the database
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::PgArticleRepository;
    use crate::domain::entity::article;
    use crate::domain::repository::article::{ArticleRepository, CreateArticleError};

    #[sqlx::test(fixtures(
        path = "../../../../../tests/fixtures",
        scripts("tags", "series", "categories")
    ))]
    async fn create_article_case_1(pool: sqlx::PgPool) {
        let repo = PgArticleRepository { pool };

        let article = article::ArticleCreate {
            title: "title".try_into().unwrap(),
            description: "description".into(),
            content: Some("content".into()),
            series_id: None,
            category_ids: vec![],
            tag_ids: vec![],
        };

        let res = repo.create(article.clone()).await;

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.title, article.title);
        assert_eq!(res.slug, article.title.as_slug());
        assert_eq!(res.description, article.description);
        assert_eq!(res.content, article.content);
        assert!(chrono::Utc::now() - res.created_at.value() <= chrono::Duration::seconds(5));
        assert!(chrono::Utc::now() - res.updated_at.value() <= chrono::Duration::seconds(5));
        assert_eq!(res.deleted_at, None);
        assert_eq!(res.published_at, None);
        assert_eq!(res.series_id, article.series_id);
        assert_eq!(res.category_ids, article.category_ids);
        assert_eq!(res.tag_ids, article.tag_ids);
    }

    #[sqlx::test(fixtures(
        path = "../../../../../tests/fixtures",
        scripts("tags", "series", "categories")
    ))]
    async fn create_article_case_2(pool: sqlx::PgPool) {
        let repo = PgArticleRepository { pool };

        let article = article::ArticleCreate {
            title: "title 1".try_into().unwrap(),
            description: "description".into(),
            content: Some("content".into()),
            series_id: Some(1.try_into().unwrap()),
            category_ids: vec![1.try_into().unwrap(), 2.try_into().unwrap()],
            tag_ids: vec![2.try_into().unwrap(), 3.try_into().unwrap()],
        };

        let res = repo.create(article.clone()).await;

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.title, article.title);
        assert_eq!(res.slug, article.title.as_slug());
        assert_eq!(res.description, article.description);
        assert_eq!(res.content, article.content);
        assert!(chrono::Utc::now() - res.created_at.value() <= chrono::Duration::seconds(5));
        assert!(chrono::Utc::now() - res.updated_at.value() <= chrono::Duration::seconds(5));
        assert_eq!(res.deleted_at, None);
        assert_eq!(res.published_at, None);
        assert_eq!(res.series_id, article.series_id);
        assert_eq!(res.category_ids, article.category_ids);
        assert_eq!(res.tag_ids, article.tag_ids);
    }

    #[sqlx::test(fixtures(
        path = "../../../../../tests/fixtures",
        scripts("tags", "series", "categories", "articles")
    ))]
    async fn create_article_case_3(pool: sqlx::PgPool) {
        let repo = PgArticleRepository { pool };

        let article = article::ArticleCreate {
            title: "article title".try_into().unwrap(),
            description: "description".into(),
            content: Some("content".into()),
            series_id: Some(1.try_into().unwrap()),
            category_ids: vec![1.try_into().unwrap(), 2.try_into().unwrap()],
            tag_ids: vec![2.try_into().unwrap(), 3.try_into().unwrap()],
        };

        let res = repo.create(article.clone()).await;

        assert!(res.is_err());

        let err = res.unwrap_err();

        assert!(matches!(err, CreateArticleError::DuplicateSlug(_)));
    }
}

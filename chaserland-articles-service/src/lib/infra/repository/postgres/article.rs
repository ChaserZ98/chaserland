use crate::domain::entity::{article, series};
use crate::domain::repository::article::{CreateArticleError, GetArticleError, DeleteArticleError, ArticlesFilter};
use async_trait::async_trait;
use chaserland_common::pagination::{Offset, Page, PageSize};
use sqlx::{PgPool, Postgres, QueryBuilder};
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
}

impl TryInto<article::Article> for PgArticle {
    type Error = String;

    fn try_into(self) -> Result<article::Article, Self::Error> {
        let id = self.id.try_into()?;
        let title = self.title.try_into()?;
        let slug = self.slug.try_into()?;
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

        Ok(article::Article {
            id,
            title,
            slug,
            description,
            content,
            created_at,
            published_at,
            updated_at,
            deleted_at,
            series_id,
            category_ids,
            tag_ids,
        })
    }
}

#[async_trait]
impl ArticleRepository for PgArticleRepository {
    async fn create(
        &self,
        article: article::ArticleCreate,
    ) -> Result<article::Article, CreateArticleError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|why| CreateArticleError::Unknown(why.into()))?;

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
                    CreateArticleError::DuplicateSlug(slug),
                _ => CreateArticleError::Unknown(why.into()),
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
                .map_err(|why| CreateArticleError::Unknown(why.into()))?;
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
                .map_err(|why| CreateArticleError::Unknown(why.into()))?;
        }

        tx.commit()
            .await
            .map_err(|why| CreateArticleError::Unknown(why.into()))?;

        Ok(article::Article {
            id: pg_article.id.try_into().unwrap(),
            title: pg_article.title.try_into().unwrap(),
            slug: pg_article.slug.try_into().unwrap(),
            description: pg_article.description.into(),
            content: pg_article.content.map(|v| v.into()),
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
    ) -> Result<article::Article, GetArticleError> {
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

        let pg_article: Option<PgArticle> = query.build_query_as().fetch_optional(&self.pool).await.map_err(|why| GetArticleError::Unknown(why.into()))?;
        if pg_article.is_none() {
            return Err(GetArticleError::NotFound(identifier));
        }

        let pg_article = pg_article.unwrap();

        let category_ids: Vec<i32> = sqlx::query_scalar(
            "SELECT category_id FROM article.article_categories WHERE article_id = $1").bind(pg_article.id).fetch_all(&self.pool).await.map_err(|why| GetArticleError::Unknown(why.into()))?;

        let tag_ids: Vec<i32> = sqlx::query_scalar("SELECT tag_id FROM article.article_tags WHERE article_id = $1").bind(pg_article.id).fetch_all(&self.pool).await.map_err(|why| GetArticleError::Unknown(why.into()))?;

        Ok(article::Article {
            id: pg_article.id.try_into().unwrap(),
            title: pg_article.title.try_into().unwrap(),
            slug: pg_article.slug.try_into().unwrap(),
            description: pg_article.description.into(),
            content: pg_article.content.map(|v| v.into()),
            created_at: pg_article.created_at.into(),
            published_at: pg_article.published_at.map(|v| v.into()),
            updated_at: pg_article.updated_at.into(),
            deleted_at: pg_article.deleted_at.map(|v| v.into()),
            series_id: pg_article.series_id.map(|v| v.try_into().unwrap()),
            category_ids: category_ids.iter().map(|v| v.try_into().unwrap()).collect(),
            tag_ids: tag_ids.iter().map(|v| v.try_into().unwrap()).collect(),
        })
    }

    async fn get_many(
        &self,
        page: Page,
        page_size: PageSize,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<article::Article>, GetArticleError> {
        let offset = Offset::from((page, page_size));
        let mut query = QueryBuilder::<Postgres>::new("SELECT a.*, ARRAY_REMOVE(ARRAY_AGG(DISTINCT c.category_id), NULL) AS category_ids, ARRAY_REMOVE(ARRAY_AGG(DISTINCT d.tag_id), NULL) AS tag_ids FROM ");

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
        query.push(" LEFT JOIN article.article_categories AS c ON a.id = c.article_id");
        query.push(" LEFT JOIN article.article_tags AS d ON a.id = d.article_id");

        if let Some(filter) = filter {
            query.push(" WHERE ");
            if let Some(series_identifier) = filter.series_id() {
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
                query.push(" AND c.category_id = ANY(");
                query.push_bind(filter.category_ids().iter().map(|v| v.value()).collect::<Vec<_>>());
                query.push(")");
            }

            if !filter.tag_ids().is_empty() {
                query.push(" AND d.id = ANY(");
                query.push_bind(filter.tag_ids().iter().map(|v| v.value()).collect::<Vec<_>>());
                query.push(")");
            }
        }

        query.push(" GROUP BY a.id ");

        query.push(" LIMIT ");
        query.push_bind(page_size.value());
        query.push(" OFFSET ");
        query.push_bind(offset.value());

        let articles: Vec<PgArticle> = query.build_query_as().fetch_all(&self.pool).await.map_err(|why| GetArticleError::Unknown(why.into()))?;

        let res = articles.into_iter().map(|v| v.try_into()).collect::<Result<Vec<article::Article>, String>>().map_err(|why| GetArticleError::Unknown(anyhow::anyhow!(why)))?;

        Ok(res)
    }

    async fn delete(
        &self,
        identifier: article::Identifier,
        hard: bool
    ) -> Result<(), DeleteArticleError> {
        let mut tx = self.pool.begin().await.map_err(|why| DeleteArticleError::Unknown(why.into()))?;

        let mut query = QueryBuilder::<Postgres>::new("");
        match hard {
            true => {
                query.push("DELETE FROM article.articles WHERE ");
            }
            false => {
                query.push("UPDATE article.articles SET deleted_at = now() WHERE ");
            }
        };

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

        let res = query.build().execute(&mut *tx).await.map_err(|why| DeleteArticleError::Unknown(why.into()))?;
        
        if res.rows_affected() == 0 {
            return Err(DeleteArticleError::NotFound(identifier));
        }

        tx.commit().await.map_err(|why| DeleteArticleError::Unknown(why.into()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // create test case
    mod test_create {
        use crate::infra::repository::postgres::article::PgArticleRepository;
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
        async fn create_article_case_duplicate_slug(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };
    
            let article = article::ArticleCreate {
                title: "article title 1".try_into().unwrap(),
                description: "description".into(),
                content: Some("content".into()),
                series_id: Some(1.try_into().unwrap()),
                category_ids: vec![1.try_into().unwrap(), 2.try_into().unwrap()],
                tag_ids: vec![2.try_into().unwrap(), 3.try_into().unwrap()],
            };
    
            let res = repo.create(article.clone()).await;

            println!("{:?}", res);
    
            assert!(res.is_err());
    
            let err = res.unwrap_err();
    
            assert!(matches!(err, CreateArticleError::DuplicateSlug(_)));
        }
    }
    // get_one test case
    mod test_get_one{
        use crate::domain::entity::article;
        use crate::infra::repository::postgres::article::PgArticleRepository;
        use crate::domain::repository::article::{ArticleRepository, GetArticleError};

        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn get_one_case_1(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let res = repo.get_one(article::Identifier::Id(1.try_into().unwrap()), false, true).await;

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.id, 1.try_into().unwrap());
            assert_eq!(res.title, "article title 1".try_into().unwrap());
            assert_eq!(res.slug, "article-title-1".try_into().unwrap());
            assert_eq!(res.description, "article description 1".into());

            assert_eq!(res.content, Some("article content 1".into()));
            assert!(chrono::Utc::now() - res.created_at.value() <= chrono::Duration::seconds(5));
            assert!(chrono::Utc::now() - res.updated_at.value() <= chrono::Duration::seconds(5));
            assert_eq!(res.deleted_at, None);
            assert_eq!(res.published_at, None);
            assert_eq!(res.series_id, Some(1.try_into().unwrap()));
            assert_eq!(res.category_ids, vec![]);
            assert_eq!(res.tag_ids, vec![]);
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn get_one_case_2(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let res = repo.get_one(article::Identifier::Id(2.try_into().unwrap()), false, true).await;

            let target = article::Article::new(
                2.try_into().unwrap(),
                "article title 2".try_into().unwrap(),
                "article description 2".into(),
                Some("article content 2".into()),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
                Some(2.try_into().unwrap()),
                vec![1.try_into().unwrap(), 2.try_into().unwrap()],
                vec![2.try_into().unwrap(), 3.try_into().unwrap()],
            );

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.id, target.id);
            assert_eq!(res.title, target.title);
            assert_eq!(res.slug, target.slug);
            assert_eq!(res.description, target.description);
            assert_eq!(res.content, target.content);
            assert!(res.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
            assert!(res.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
            assert_eq!(res.deleted_at, target.deleted_at);
            assert_eq!(res.published_at, target.published_at);
            assert_eq!(res.series_id, target.series_id);
            assert_eq!(res.category_ids, target.category_ids);
            assert_eq!(res.tag_ids, target.tag_ids);
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn get_one_case_3(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let identifier = article::Identifier::Id(3.try_into().unwrap());

            let res = repo.get_one(identifier.clone(), false, true).await;
            
            assert!(res.is_err());

            let err = res.unwrap_err();

            assert!(match err {
                GetArticleError::NotFound(value) => value == identifier,
                _ => false
            });
        }
    }
    mod test_get_many {
        use crate::infra::repository::postgres::article::PgArticleRepository;
        use crate::domain::repository::article::{ArticleRepository, ArticlesFilter};
        use crate::domain::entity::article;

        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn test_get_many_case_1(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let page = 1.try_into().unwrap();
            let page_size = 10.try_into().unwrap();
            let public_only = false;
            let with_content = true;
            let filter = None;

            let res = repo.get_many(page, page_size, public_only, with_content, filter).await;

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.len(), 2);

            let target_1 = article::Article::new(
                1.try_into().unwrap(),
                "article title 1".try_into().unwrap(),
                "article description 1".into(),
                Some("article content 1".into()),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
                Some(1.try_into().unwrap()),
                vec![],
                vec![],
            );

            let target_2 = article::Article::new(
                2.try_into().unwrap(),
                "article title 2".try_into().unwrap(),
                "article description 2".into(),
                Some("article content 2".into()),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
                Some(2.try_into().unwrap()),
                vec![1.try_into().unwrap(), 2.try_into().unwrap()],
                vec![2.try_into().unwrap(), 3.try_into().unwrap()],
            );

            let article_1 = &res[0];
            assert_eq!(article_1.id, target_1.id);
            assert_eq!(article_1.title, target_1.title);
            assert_eq!(article_1.slug, target_1.slug);
            assert_eq!(article_1.description, target_1.description);
            assert_eq!(article_1.content, target_1.content);
            assert!(article_1.created_at.value() - target_1.created_at.value() <= chrono::Duration::seconds(5));
            assert!(article_1.updated_at.value() - target_1.updated_at.value() <= chrono::Duration::seconds(5));
            assert_eq!(article_1.deleted_at, target_1.deleted_at);
            assert_eq!(article_1.published_at, target_1.published_at);
            assert_eq!(article_1.series_id, target_1.series_id);
            assert_eq!(article_1.category_ids, target_1.category_ids);
            assert_eq!(article_1.tag_ids, target_1.tag_ids);

            let article_2 = &res[1];
            assert_eq!(article_2.id, target_2.id);
            assert_eq!(article_2.title, target_2.title);
            assert_eq!(article_2.slug, target_2.slug);
            assert_eq!(article_2.description, target_2.description);
            assert_eq!(article_2.content, target_2.content);
            assert!(article_2.created_at.value() - target_2.created_at.value() <= chrono::Duration::seconds(5));
            assert!(article_2.updated_at.value() - target_2.updated_at.value() <= chrono::Duration::seconds(5));
            assert_eq!(article_2.deleted_at, target_2.deleted_at);
            assert_eq!(article_2.published_at, target_2.published_at);
            assert_eq!(article_2.series_id, target_2.series_id);
            assert_eq!(article_2.category_ids, target_2.category_ids);
            assert_eq!(article_2.tag_ids, target_2.tag_ids);
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn test_get_many_case_empty(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let page = 1.try_into().unwrap();
            let page_size = 10.try_into().unwrap();
            let public_only = false;
            let with_content = false;
            let filter = None;

            let res = repo.get_many(page, page_size, public_only, with_content, filter).await;

            println!("{:#?}", res);

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.len(), 0);
        }

    }
    mod test_delete {
        use crate::domain::entity::article;
        use crate::domain::repository::article::{ArticleRepository, DeleteArticleError, GetArticleError};
        use crate::infra::repository::postgres::article::PgArticleRepository;

        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn delete_hard_case_id(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };
            let id = 1.try_into().unwrap();
            let identifier = article::Identifier::Id(id);

            let res = repo.get_one(identifier.clone(), false, true).await;

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.id, id);
            
            let res = repo.delete(identifier.clone(), true).await;

            assert!(res.is_ok());

            let res = repo.get_one(identifier.clone(), false, true).await;

            assert!(res.is_err());

            let err = res.unwrap_err();

            assert!(match err {
                GetArticleError::NotFound(value) => value == identifier,
                _ => false
            });
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn delete_hard_case_slug(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let slug: article::ArticleSlug = "article-title-1".try_into().unwrap();
            let identifier = article::Identifier::Slug(slug.clone());

            let res = repo.get_one(identifier.clone(), false, true).await;

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.slug, slug);

            let res = repo.delete(identifier.clone(), true).await;

            assert!(res.is_ok());

            let res = repo.get_one(identifier.clone(), false, true).await;

            assert!(res.is_err());

            let err = res.unwrap_err();

            assert!(match err {
                GetArticleError::NotFound(value) => value == identifier,
                _ => false
            });
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn delete_hard_case_id_not_found(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let id = 4.try_into().unwrap();
            let identifier = article::Identifier::Id(id);

            let res = repo.delete(identifier.clone(), true).await;

            assert!(res.is_err());

            let err = res.unwrap_err();

            assert!(match err {
                DeleteArticleError::NotFound(value) => value == identifier,
                _ => false
            });

            let slug = "article-title-4".try_into().unwrap();
            let identifier = article::Identifier::Slug(slug);

            let res = repo.delete(identifier.clone(), true).await;

            assert!(res.is_err());

            let err = res.unwrap_err();

            assert!(match err {
                DeleteArticleError::NotFound(value) => value == identifier,
                _ => false
            });
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn delete_hard_case_slug_not_found(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let slug = "article-title-4".try_into().unwrap();
            let identifier = article::Identifier::Slug(slug);

            let res = repo.delete(identifier.clone(), true).await;

            assert!(res.is_err());

            let err = res.unwrap_err();

            assert!(match err {
                DeleteArticleError::NotFound(value) => value == identifier,
                _ => false
            });
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn delete_soft_case_id(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let id = 1.try_into().unwrap();
            let identifier = article::Identifier::Id(id);

            let res = repo.get_one(identifier.clone(), false, true).await;

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.id, id);
            assert_eq!(res.deleted_at.is_none(), true);

            let res = repo.delete(identifier.clone(), false).await;

            assert!(res.is_ok());

            let res = repo.get_one(identifier.clone(), false, true).await;

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.id, id);
            assert_eq!(res.deleted_at.is_some(), true);
            assert!(res.deleted_at.unwrap().value() - chrono::Utc::now() <= chrono::Duration::seconds(5));
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn delete_soft_case_slug(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let slug: article::ArticleSlug = "article-title-1".try_into().unwrap();
            let identifier = article::Identifier::Slug(slug.clone());

            let res = repo.get_one(identifier.clone(), false, true).await;

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.slug, slug);

            let res = repo.delete(identifier.clone(), false).await;

            assert!(res.is_ok());

            let res = repo.get_one(identifier.clone(), false, true).await;

            assert!(res.is_ok());

            let res = res.unwrap();

            assert_eq!(res.slug, slug);
            assert_eq!(res.deleted_at.is_some(), true);
            assert!(res.deleted_at.unwrap().value() - chrono::Utc::now() <= chrono::Duration::seconds(5));
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn delete_soft_case_id_not_found(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let id = 4.try_into().unwrap();
            let identifier = article::Identifier::Id(id);

            let res = repo.delete(identifier.clone(), false).await;

            assert!(res.is_err());

            let err = res.unwrap_err();

            assert!(match err {
                DeleteArticleError::NotFound(value) => value == identifier,
                _ => false
            });
        }
        #[sqlx::test(fixtures(
            path = "../../../../../tests/fixtures",
            scripts("tags", "series", "categories", "articles", "article_categories", "article_tags")
        ))]
        async fn delete_soft_case_slug_not_found(pool: sqlx::PgPool) {
            let repo = PgArticleRepository { pool };

            let slug = "article-title-4".try_into().unwrap();
            let identifier = article::Identifier::Slug(slug);

            let res = repo.delete(identifier.clone(), false).await;

            assert!(res.is_err());

            let err = res.unwrap_err();

            assert!(match err {
                DeleteArticleError::NotFound(value) => value == identifier,
                _ => false
            });
        }
    }
}

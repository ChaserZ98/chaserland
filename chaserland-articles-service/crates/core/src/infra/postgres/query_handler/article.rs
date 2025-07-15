use crate::{
    app::query::{
        dto::ArticleDTO,
        query_handler::{error::ArticleQueryHandlerError, interface::ArticleQueryHandler},
    },
    domain::{
        article::{repository::ArticlesFilter, vo as article},
        series::vo as series,
    },
    infra::postgres::po::{PgArticle, PgCategory, PgSeries, PgTag},
};
use chaserland_common::pagination::Pagination;
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Clone)]
pub struct PgArticleQueryHandler {
    pool: PgPool,
}

impl PgArticleQueryHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ArticleQueryHandler for PgArticleQueryHandler {
    async fn get_one(
        &self,
        identifier: article::Identifier,
        public_only: bool,
        with_content: bool,
    ) -> Result<ArticleDTO, ArticleQueryHandlerError> {
        let mut tx = self
            .pool
            .begin_with("BEGIN TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
            .await
            .map_err(|why| ArticleQueryHandlerError::Transaction(why.to_string()))?;

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

        let pg_article: Option<PgArticle> = query.build_query_as().fetch_optional(&mut *tx).await?;

        if pg_article.is_none() {
            return Err(ArticleQueryHandlerError::ArticleNotFound(identifier));
        }

        let pg_article = pg_article.unwrap();

        let pg_series = match pg_article.series_id {
            None => None,
            Some(series_id) => {
                let pg_series: PgSeries =
                    sqlx::query_as("SELECT * FROM article.series WHERE id = $1")
                        .bind(series_id)
                        .fetch_one(&mut *tx)
                        .await?;
                Some(pg_series)
            }
        };

        let categories: Vec<PgCategory> = sqlx::query_as(
            "SELECT b.id, b.name, b.slug FROM article.article_categories AS a LEFT JOIN article.categories AS b ON a.category_id = b.id WHERE a.article_id = $1").bind(pg_article.id).fetch_all(&mut *tx).await?;

        let tags: Vec<PgTag> = sqlx::query_as("SELECT b.id, b.slug, b.name FROM article.article_tags AS a LEFT JOIN article.tags AS b ON a.tag_id = b.id WHERE a.article_id = $1").bind(pg_article.id).fetch_all(&mut *tx).await?;

        let article_dto = ArticleDTO {
            id: pg_article.id,
            title: pg_article.title,
            slug: pg_article.slug,
            description: pg_article.description,
            content: pg_article.content,
            created_at: pg_article.created_at,
            published_at: pg_article.published_at,
            updated_at: pg_article.updated_at,
            deleted_at: pg_article.deleted_at,
            series: pg_series.map(|x| x.into()),
            categories: categories.into_iter().map(|x| x.into()).collect(),
            tags: tags.into_iter().map(|x| x.into()).collect(),
        };

        Ok(article_dto)
    }

    async fn get_many(
        &self,
        pagination: Pagination,
        public_only: bool,
        with_content: bool,
        filter: Option<ArticlesFilter>,
    ) -> Result<Vec<ArticleDTO>, ArticleQueryHandlerError> {
        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT a.*, COALESCE(c.category_ids, '{}') as category_ids, COALESCE(d.tag_ids, '{}') as tag_ids FROM ",
        );

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
                    }
                    false => {
                        has_prev_condition = true;
                    }
                }
                query.push(" c.category_ids @> ");
                query.push_bind(
                    filter
                        .category_ids()
                        .iter()
                        .map(|v| v.value())
                        .collect::<Vec<_>>(),
                );
            }

            if !filter.tag_ids().is_empty() {
                match has_prev_condition {
                    true => {
                        query.push(" AND ");
                    }
                    _ => {}
                }
                query.push(" d.tag_ids @> ");
                query.push_bind(
                    filter
                        .tag_ids()
                        .iter()
                        .map(|v| v.value())
                        .collect::<Vec<_>>(),
                );
            }
        }

        query.push(" LIMIT ");
        query.push_bind(pagination.page_size.value());
        query.push(" OFFSET ");
        query.push_bind(pagination.as_offset().value());

        let articles: Vec<PgArticle> = query.build_query_as().fetch_all(&self.pool).await?;

        let mut res = vec![];

        for article in articles {
            let series = match article.series_id {
                None => None,
                Some(series_id) => {
                    let pg_series: PgSeries =
                        sqlx::query_as("SELECT * FROM article.series WHERE id = $1")
                            .bind(series_id)
                            .fetch_one(&self.pool)
                            .await?;
                    Some(pg_series)
                }
            };

            let categories: Vec<PgCategory> =
                sqlx::query_as("SELECT * FROM article.categories WHERE id = ANY($1)")
                    .bind(&article.category_ids)
                    .fetch_all(&self.pool)
                    .await?;

            let tags: Vec<PgTag> = sqlx::query_as("SELECT * FROM article.tags WHERE id = ANY($1)")
                .bind(&article.tag_ids)
                .fetch_all(&self.pool)
                .await?;

            let article_dto = ArticleDTO {
                id: article.id,
                title: article.title,
                slug: article.slug,
                description: article.description,
                content: article.content,
                created_at: article.created_at,
                published_at: article.published_at,
                updated_at: article.updated_at,
                deleted_at: article.deleted_at,
                series: series.map(|x| x.into()),
                categories: categories.into_iter().map(|x| x.into()).collect(),
                tags: tags.into_iter().map(|x| x.into()).collect(),
            };

            res.push(article_dto);
        }

        Ok(res)
    }
}

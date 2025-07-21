use crate::common::{init_command_service, init_query_service};
use chaserland_articles_service_core::{
    app::{
        command::{self, error::UnpublishArticleError, interface::ArticleCommandService},
        query::{self, error::GetArticleOneError, interface::ArticleQueryService},
    },
    domain::article::vo as article,
    migrator::MIGRATOR,
};
use chrono::{Duration, Utc};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../tests/fixtures",
        scripts(
            "series",
            "categories",
            "tags",
            "articles",
            "article_categories",
            "article_tags",
            "publish_article"
        )
    )
)]
async fn unpublish_article_case_1(pool: PgPool) {
    let command_service = init_command_service(pool.clone());
    let query_service = init_query_service(pool.clone());

    let id = article::Id::new(1);

    let query = query::GetArticleOneQuery {
        identifier: id.as_identifier(),
        public_only: true,
        with_content: true,
    };

    let target = query_service.get_article_one(query).await;

    assert!(target.is_ok());

    let mut target = target.unwrap();
    target.published_at = None;
    target.updated_at = Utc::now();

    let command = command::UnpublishArticleCommand {
        identifier: id.as_identifier(),
    };

    let res = command_service.unpublish_article(command).await;

    assert!(res.is_ok());

    let query = query::GetArticleOneQuery {
        identifier: id.as_identifier(),
        public_only: true,
        with_content: true,
    };

    let res = query_service.get_article_one(query).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(matches!(res, GetArticleOneError::NotFound(_)));

    let query = query::GetArticleOneQuery {
        identifier: id.as_identifier(),
        public_only: false,
        with_content: true,
    };
    let res = query_service.get_article_one(query).await;

    assert!(res.is_ok(), "res: {:#?}", res);

    let res = res.unwrap();

    assert_eq!(res.id, target.id);
    assert_eq!(res.title, target.title);
    assert_eq!(res.slug, target.slug);
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert_eq!(res.created_at, target.created_at);
    assert!(res.updated_at - target.updated_at < Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    // assert!(res.published_at.unwrap() - target.published_at.unwrap() < Duration::seconds(5));
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series, target.series);
    assert_eq!(res.categories, target.categories);
    assert_eq!(res.tags, target.tags);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../tests/fixtures",
        scripts(
            "series",
            "categories",
            "tags",
            "articles",
            "article_categories",
            "article_tags",
        )
    )
)]
async fn unpublish_article_case_2(pool: PgPool) {
    let command_service = init_command_service(pool.clone());
    let query_service = init_query_service(pool.clone());

    let id = article::Id::new(1);

    let query = query::GetArticleOneQuery {
        identifier: id.as_identifier(),
        public_only: false,
        with_content: true,
    };

    let target = query_service.get_article_one(query).await;

    assert!(target.is_ok());

    let target = target.unwrap();

    assert!(target.published_at.is_none());

    let command = command::UnpublishArticleCommand {
        identifier: id.as_identifier(),
    };

    let res = command_service.unpublish_article(command).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(matches!(res, UnpublishArticleError::NotPublished(_)));
}

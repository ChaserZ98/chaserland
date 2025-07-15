use chaserland_articles_service_core::{
    domain::{
        article::{
            entity::Article,
            repository::{ArticleRepository, ArticleRepositoryError},
        },
        category::vo as category,
    },
    infra::postgres::repository::PgArticleRepository,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../../../tests/fixtures",
        scripts(
            "tags",
            "series",
            "categories",
            "articles",
            "article_categories",
            "article_tags"
        )
    )
)]
async fn remove_category_case_1(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(1);
    article.remove_category_id(category_id).unwrap();

    let res = repo
        .remove_category(article.id, category_id, version, &mut tx)
        .await;

    assert!(res.is_ok());
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../../../tests/fixtures",
        scripts(
            "tags",
            "series",
            "categories",
            "articles",
            "article_categories",
            "article_tags"
        )
    )
)]
async fn remove_category_case_article_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    article.category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(1);
    article.remove_category_id(category_id).unwrap();

    let res = repo
        .remove_category(article.id, category_id, version, &mut tx)
        .await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        ArticleRepositoryError::ArticleNotFound(id) => id == article.id.as_identifier(),
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../../../tests/fixtures",
        scripts(
            "tags",
            "series",
            "categories",
            "articles",
            "article_categories",
            "article_tags"
        )
    )
)]
async fn remove_category_case_category_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.category_ids = vec![1.try_into().unwrap(), 5.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(5);

    let res = repo
        .remove_category(article.id, category_id, version, &mut tx)
        .await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        ArticleRepositoryError::CategoryNotFound(id) => id == category_id.as_identifier(),
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../../../tests/fixtures",
        scripts(
            "tags",
            "series",
            "categories",
            "articles",
            "article_categories",
            "article_tags"
        )
    )
)]
async fn remove_category_case_concurrent_conflict(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(1);
    article.remove_category_id(category_id).unwrap();

    let res = repo
        .remove_category(article.id, category_id, version, &mut tx)
        .await;

    assert!(res.is_ok());

    let res = repo
        .remove_category(article.id, category_id, version, &mut tx)
        .await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        ArticleRepositoryError::ConcurrentConflict {
            id,
            current_version,
            db_version,
        } => id == article.id && current_version == version && db_version != version,
        _ => false,
    });
}

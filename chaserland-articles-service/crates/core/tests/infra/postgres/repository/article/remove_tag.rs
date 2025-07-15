use chaserland_articles_service_core::{
    domain::{
        article::{
            entity::Article,
            repository::{ArticleRepository, ArticleRepositoryError},
        },
        tag::vo as tag,
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
async fn remove_tag_case_1(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 3.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let tag_id = tag::Id::new(2);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, version, &mut tx).await;

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
async fn remove_tag_case_article_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 3.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let tag_id = tag::Id::new(2);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, version, &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
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
async fn remove_tag_case_tag_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 4.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let tag_id = tag::Id::new(4);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, version, &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::TagNotFound(id) => id == tag_id.as_identifier(),
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
async fn remove_tag_case_both_not_found(pool: PgPool) {
    // Note: when both not found, article id is checked first
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 4.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let tag_id = tag::Id::new(4);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, version, &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
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
async fn remove_tag_case_concurrent_conflict(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 3.try_into().unwrap()];
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let tag_id = tag::Id::new(2);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, version, &mut tx).await;

    assert!(res.is_ok());

    let res = repo.remove_tag(article.id, tag_id, version, &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ConcurrentConflict {
            id,
            current_version,
            db_version,
        } => {
            id == article.id && current_version == version && db_version != version
        }
        _ => false,
    });
}

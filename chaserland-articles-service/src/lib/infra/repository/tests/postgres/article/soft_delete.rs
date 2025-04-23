use crate::domain::entity::article;
use crate::domain::repository::article::{ArticleRepository, SoftDeleteArticleError};
use crate::infra::repository::postgres::article::PgArticleRepository;

#[sqlx::test(fixtures(
    path = "../../../../../../../tests/fixtures",
    scripts(
        "tags",
        "series",
        "categories",
        "articles",
        "article_categories",
        "article_tags"
    )
))]
async fn soft_delete_case_id(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 1.try_into().unwrap();
    article.soft_delete().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let res = repo
        .soft_delete(
            article.id,
            article.deleted_at.clone().unwrap(),
            article.version,
        )
        .await;

    assert!(res.is_ok());

    let res = repo.get_one(article.id.as_identifier(), false, true).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, article.id);
    assert_eq!(res.deleted_at.is_some(), true);
    assert!(res.deleted_at.unwrap().value() - chrono::Utc::now() <= chrono::Duration::seconds(5));
}
#[sqlx::test(fixtures(
    path = "../../../../../../../tests/fixtures",
    scripts(
        "tags",
        "series",
        "categories",
        "articles",
        "article_categories",
        "article_tags"
    )
))]
async fn soft_delete_case_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 4.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.soft_delete().unwrap();

    let res = repo
        .soft_delete(
            article.id,
            article.deleted_at.clone().unwrap(),
            article.version,
        )
        .await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        SoftDeleteArticleError::NotFound(id) => id == article.id,
        _ => false,
    });
}
#[sqlx::test(fixtures(
    path = "../../../../../../../tests/fixtures",
    scripts(
        "tags",
        "series",
        "categories",
        "articles",
        "article_categories",
        "article_tags",
        "soft_delete_article"
    )
))]
async fn soft_delete_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 2.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.soft_delete().unwrap();

    let res = repo
        .soft_delete(
            article.id,
            article.deleted_at.clone().unwrap(),
            article.version,
        )
        .await;

    assert!(res.is_ok());

    let res = repo
        .soft_delete(
            article.id,
            article.deleted_at.clone().unwrap(),
            article.version,
        )
        .await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        SoftDeleteArticleError::VersionMismatch {
            id,
            current_version,
            db_version,
        } =>
            id == article.id && current_version == article.version && db_version != article.version,
        _ => false,
    });
}

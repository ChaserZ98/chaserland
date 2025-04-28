use crate::domain::entity::article;
use crate::domain::repository::article::{ArticleRepository, ArticleRepositoryError};
use crate::infra::repository::postgres::article::PgArticleRepository;

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
async fn revoke_soft_delete_case_id(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 1.try_into().unwrap();
    article.deleted_at = Some(chrono::Utc::now().into());
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.revoke_soft_delete().unwrap();

    let res = repo.revoke_soft_delete(article.id, article.version).await;

    println!("{:?}", res);

    assert!(res.is_ok());
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
async fn revoke_soft_delete_case_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 4.try_into().unwrap();
    article.deleted_at = Some(chrono::Utc::now().into());
    article.revoke_soft_delete().unwrap();

    let res = repo.revoke_soft_delete(article.id, article.version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(id) => id == article.id.as_identifier(),
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
async fn revoke_soft_delete_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 2.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.deleted_at = Some(chrono::Utc::now().into());
    article.revoke_soft_delete().unwrap();

    let res = repo.revoke_soft_delete(article.id, article.version).await;

    assert!(res.is_ok());

    let res = repo.revoke_soft_delete(article.id, article.version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::VersionMismatch {
            id,
            current_version,
            db_version,
        } =>
            id == article.id && current_version == article.version && db_version != article.version,
        _ => false,
    });
}

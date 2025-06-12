use crate::domain::article::{
    entity::Article,
    repository::{ArticleRepository, ArticleRepositoryError},
};
use crate::infra::repository::postgres::article::PgArticleRepository;

#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts(
            "tags",
            "series",
            "categories",
            "articles",
            "article_categories",
            "article_tags",
            "publish_article"
        )
    )
)]
async fn unpublish_case_id(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 1.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.published_at = Some(chrono::Utc::now().into());
    article.unpublish().unwrap();

    let res = repo.unpublish(article.id, article.version).await;

    assert!(res.is_ok());
}
#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts(
            "tags",
            "series",
            "categories",
            "articles",
            "article_categories",
            "article_tags",
            "publish_article"
        )
    )
)]
async fn unpublish_case_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    article.published_at = Some(chrono::Utc::now().into());
    article.unpublish().unwrap();

    let res = repo.unpublish(article.id, article.version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == article.id.as_identifier(),
        _ => false,
    });
}
#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts(
            "tags",
            "series",
            "categories",
            "articles",
            "article_categories",
            "article_tags",
            "publish_article"
        )
    )
)]
async fn unpublish_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 1.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.published_at = Some(chrono::Utc::now().into());
    article.unpublish().unwrap();

    let res = repo.unpublish(article.id, article.version).await;

    assert!(res.is_ok());

    let res = repo.unpublish(article.id, article.version).await;

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

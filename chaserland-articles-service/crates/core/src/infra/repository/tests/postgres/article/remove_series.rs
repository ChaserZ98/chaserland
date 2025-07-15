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
        )
    )
)]
async fn remove_series_case_id_with_series_id(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 1.try_into().unwrap();
    article.series_id = Some(1.try_into().unwrap());
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.remove_series_id();

    let res = repo.remove_series(article.id, version).await;

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
        )
    )
)]
async fn remove_series_case_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    article.series_id = Some(1.try_into().unwrap());
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.remove_series_id();

    let res = repo.remove_series(article.id, version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(id) => id == article.id.as_identifier(),
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
        )
    )
)]
async fn remove_series_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 1.try_into().unwrap();
    article.series_id = Some(1.try_into().unwrap());
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.remove_series_id();

    let res = repo.remove_series(article.id, version).await;

    assert!(res.is_ok());

    let res = repo.remove_series(article.id, version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::VersionMismatch {
            id,
            current_version,
            db_version,
        } => id == article.id && current_version == version && db_version != version,
        _ => false,
    });
}

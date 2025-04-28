use crate::domain::entity::{article, series};
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
    )
))]
async fn set_series_case_id_with_series_id_originally_not_null(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let series_id = series::Id::new(2);

    let mut article = article::Article::default();
    article.id = 1.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.set_series_id(series_id);

    let res = repo
        .set_series(article.id, series_id, article.version)
        .await;

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
        "article_tags",
        "remove_article_series"
    )
))]
async fn set_series_case_id_with_series_id_originally_null(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let series_id = series::Id::new(1);

    let mut article = article::Article::default();
    article.id = 1.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.set_series_id(series_id);

    let res = repo
        .set_series(article.id, series_id, article.version)
        .await;

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
async fn set_series_case_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let series_id = series::Id::new(1);

    let mut article = article::Article::default();
    article.id = 4.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.set_series_id(series_id);

    let res = repo
        .set_series(article.id, series_id, article.version)
        .await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == article.id.as_identifier(),
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
        "remove_article_series"
    )
))]
async fn set_series_case_series_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let series_id = series::Id::new(4);

    let mut article = article::Article::default();
    article.id = 1.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.set_series_id(series_id);

    let res = repo
        .set_series(article.id, series_id, article.version)
        .await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::SeriesNotFound(value) => value == series_id.as_identifier(),
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
        "remove_article_series"
    )
))]
async fn set_series_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let series_id = series::Id::new(1);

    let mut article = article::Article::default();
    article.id = 1.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.set_series_id(series_id);

    let res = repo
        .set_series(article.id, series_id, article.version)
        .await;

    assert!(res.is_ok());

    let res = repo
        .set_series(article.id, series_id, article.version)
        .await;

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

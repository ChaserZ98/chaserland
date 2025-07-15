use crate::domain::article::{
    entity::Article,
    repository::{ArticleRepository, ArticleRepositoryError},
};
use crate::domain::category::vo as category;
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
            "article_tags"
        )
    )
)]
async fn add_category_case_1(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 1.try_into().unwrap();
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(1);
    article.add_category_id(category_id).unwrap();

    let res = repo.add_category(article.id, category_id, version).await;

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
            "article_tags"
        )
    )
)]
async fn add_category_case_article_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(1);
    article.add_category_id(category_id).unwrap();

    let res = repo.add_category(article.id, category_id, version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    let err = err.try_into().unwrap();

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
            "article_tags"
        )
    )
)]
async fn add_category_case_category_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 1.try_into().unwrap();
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(4);
    article.add_category_id(category_id).unwrap();

    let res = repo.add_category(article.id, category_id, version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::CategoryNotFound(value) => value == category_id.as_identifier(),
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
            "article_tags"
        )
    )
)]
async fn add_category_case_both_not_found(pool: sqlx::PgPool) {
    // Note: when both not found, article id is checked first

    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(4);
    article.add_category_id(category_id).unwrap();

    let res = repo.add_category(article.id, category_id, version).await;

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
            "article_tags"
        )
    )
)]
async fn add_category_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 1.try_into().unwrap();
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let category_id = category::Id::new(1);
    article.add_category_id(category_id).unwrap();

    let res = repo.add_category(article.id, category_id, version).await;

    assert!(res.is_ok());

    let res = repo.add_category(article.id, category_id, version).await;

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

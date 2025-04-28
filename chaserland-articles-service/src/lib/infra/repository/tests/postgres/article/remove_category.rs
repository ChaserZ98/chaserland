use crate::domain::entity::{article, category};
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
        "article_tags"
    )
))]
async fn remove_category_case_1(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 2.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

    let category_id = category::Id::new(1);
    article.remove_category_id(category_id).unwrap();

    let res = repo
        .remove_category(article.id, category_id, article.version)
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
async fn remove_category_case_article_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 4.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

    let category_id = category::Id::new(1);
    article.remove_category_id(category_id).unwrap();

    let res = repo
        .remove_category(article.id, category_id, article.version)
        .await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
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
        "article_tags"
    )
))]
async fn remove_category_case_category_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 2.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.category_ids = vec![1.try_into().unwrap(), 5.try_into().unwrap()];

    let category_id = category::Id::new(5);

    let res = repo
        .remove_category(article.id, category_id, article.version)
        .await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        ArticleRepositoryError::CategoryNotFound(id) => id == category_id.as_identifier(),
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
        "article_tags"
    )
))]
async fn remove_category_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 2.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

    let category_id = category::Id::new(1);
    article.remove_category_id(category_id).unwrap();

    let res = repo
        .remove_category(article.id, category_id, article.version)
        .await;

    assert!(res.is_ok());

    let res = repo
        .remove_category(article.id, category_id, article.version)
        .await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        ArticleRepositoryError::VersionMismatch {
            id,
            current_version,
            db_version,
        } =>
            id == article.id && current_version == article.version && db_version != article.version,
        _ => false,
    });
}

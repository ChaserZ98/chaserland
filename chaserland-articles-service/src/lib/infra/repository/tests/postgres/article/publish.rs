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
        "article_tags"
    )
))]
async fn publish_case_id(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        "2020-01-01 00:00:00 UTC".try_into().unwrap(),
    );

    article.publish().unwrap();

    let res = repo
        .publish(
            article.id,
            article.published_at.clone().unwrap(),
            article.version,
        )
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
async fn publish_case_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::new(
        4.try_into().unwrap(),
        "article title 4".try_into().unwrap(),
        "article description 4".into(),
        Some("article content 4".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        chrono::Utc::now().into(),
    );

    article.publish().unwrap();

    let res = repo
        .publish(
            article.id,
            article.published_at.clone().unwrap(),
            article.version,
        )
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
        "article_tags"
    )
))]
async fn publish_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = article::Article::default();
    article.id = 1.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.publish().unwrap();

    let res = repo
        .publish(
            article.id,
            article.published_at.clone().unwrap(),
            article.version,
        )
        .await;

    assert!(res.is_ok());

    let res = repo
        .publish(
            article.id,
            article.published_at.clone().unwrap(),
            article.version,
        )
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

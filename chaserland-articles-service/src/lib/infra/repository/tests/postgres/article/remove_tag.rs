use crate::domain::article::{
    entity::Article,
    repository::{ArticleRepository, ArticleRepositoryError},
};
use crate::domain::tag::vo as tag;
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
async fn remove_tag_case_1(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 3.try_into().unwrap()];

    let tag_id = tag::Id::new(2);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, article.version).await;

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
async fn remove_tag_case_article_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 3.try_into().unwrap()];

    let tag_id = tag::Id::new(2);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, article.version).await;

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
        "article_tags"
    )
))]
async fn remove_tag_case_tag_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 4.try_into().unwrap()];

    let tag_id = tag::Id::new(4);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, article.version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::TagNotFound(id) => id == tag_id.as_identifier(),
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
async fn remove_tag_case_both_not_found(pool: sqlx::PgPool) {
    // Note: when both not found, article id is checked first
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 4.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 4.try_into().unwrap()];

    let tag_id = tag::Id::new(4);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, article.version).await;

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
        "article_tags"
    )
))]
async fn remove_tag_case_version_mismatch(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut article = Article::default();
    article.id = 2.try_into().unwrap();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();
    article.tag_ids = vec![2.try_into().unwrap(), 3.try_into().unwrap()];

    let tag_id = tag::Id::new(2);
    article.remove_tag_id(tag_id).unwrap();

    let res = repo.remove_tag(article.id, tag_id, article.version).await;

    assert!(res.is_ok());

    let res = repo.remove_tag(article.id, tag_id, article.version).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::VersionMismatch {
            id,
            current_version,
            db_version,
        } => {
            id == article.id && current_version == article.version && db_version != article.version
        }
        _ => false,
    });
}

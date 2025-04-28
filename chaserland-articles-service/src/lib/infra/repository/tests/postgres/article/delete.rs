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
async fn delete_hard_case_id(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);
    let id = 1.try_into().unwrap();
    let identifier = article::Identifier::Id(id);

    let res = repo.get_one(identifier.clone(), false, true).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, id);

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_ok());

    let res = repo.get_one(identifier.clone(), false, true).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
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
async fn delete_hard_case_slug(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let slug: article::Slug = "article-title-1".try_into().unwrap();
    let identifier = article::Identifier::Slug(slug.clone());

    let res = repo.get_one(identifier.clone(), false, true).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.slug(), &slug);

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_ok());

    let res = repo.get_one(identifier.clone(), false, true).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
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
async fn delete_hard_case_id_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let id = 4.try_into().unwrap();
    let identifier = article::Identifier::Id(id);

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });

    let slug = "article-title-4".try_into().unwrap();
    let identifier = article::Identifier::Slug(slug);

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
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
async fn delete_hard_case_slug_not_found(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let slug = "article-title-4".try_into().unwrap();
    let identifier = article::Identifier::Slug(slug);

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });
}

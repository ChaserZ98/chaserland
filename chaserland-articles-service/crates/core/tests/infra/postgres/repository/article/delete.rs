use chaserland_articles_service_core::{
    domain::article::{
        repository::{ArticleRepository, ArticleRepositoryError},
        vo as article,
    },
    infra::postgres::repository::PgArticleRepository,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../../../tests/fixtures",
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
async fn delete_hard_case_id(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id = 1.try_into().unwrap();
    let identifier = article::Identifier::Id(id);

    let res = repo.get_one(identifier.clone(), &mut tx).await;

    assert!(res.is_ok());

    let (res, _) = res.unwrap();

    assert_eq!(res.id, id);

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_ok());

    let res = repo.get_one(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../../../tests/fixtures",
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
async fn delete_hard_case_slug(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let slug: article::Slug = "article-title-1".try_into().unwrap();
    let identifier = article::Identifier::Slug(slug.clone());

    let res = repo.get_one(identifier.clone(), &mut tx).await;

    assert!(res.is_ok());

    let (res, _) = res.unwrap();

    assert_eq!(res.slug(), &slug);

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_ok());

    let res = repo.get_one(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../../../tests/fixtures",
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
async fn delete_hard_case_id_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id = 4.try_into().unwrap();
    let identifier = article::Identifier::Id(id);

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });

    let slug = "article-title-4".try_into().unwrap();
    let identifier = article::Identifier::Slug(slug);

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../../../tests/fixtures",
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
async fn delete_hard_case_slug_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let slug = "article-title-4".try_into().unwrap();
    let identifier = article::Identifier::Slug(slug);

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });
}

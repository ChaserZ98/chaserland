use crate::domain::article::{
    repository::{ArticleRepository, ArticleRepositoryError},
    vo as article,
};
use crate::infra::repository::postgres::article::PgArticleRepository;
use sqlx::PgPool;

#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts("tags", "series", "categories")
    )
)]
async fn create_article_case_1(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::NewArticle::new(
        "title".try_into().unwrap(),
        "description".into(),
        Some("content".into()),
        None,
        vec![],
        vec![],
    );

    let res = repo.create(article.clone()).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.title, article.title);
    assert_eq!(res.slug(), &article.title.as_slug());
    assert_eq!(res.description, article.description);
    assert_eq!(res.content, article.content);
    assert!(chrono::Utc::now() - res.created_at.value() <= chrono::Duration::seconds(5));
    assert!(chrono::Utc::now() - res.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, None);
    assert_eq!(res.published_at, None);
    assert_eq!(res.series_id, article.series_id);
    assert_eq!(res.category_ids, article.category_ids);
    assert_eq!(res.tag_ids, article.tag_ids);
}

#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts("tags", "series", "categories")
    )
)]
async fn create_article_case_2(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::NewArticle::new(
        "title 1".try_into().unwrap(),
        "description".into(),
        Some("content".into()),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let res = repo.create(article.clone()).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.title, article.title);
    assert_eq!(res.slug(), &article.title.as_slug());
    assert_eq!(res.description, article.description);
    assert_eq!(res.content, article.content);
    assert!(chrono::Utc::now() - res.created_at.value() <= chrono::Duration::seconds(5));
    assert!(chrono::Utc::now() - res.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, None);
    assert_eq!(res.published_at, None);
    assert_eq!(res.series_id, article.series_id);
    assert_eq!(res.category_ids, article.category_ids);
    assert_eq!(res.tag_ids, article.tag_ids);
}

#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts("tags", "series", "categories")
    )
)]
async fn create_article_case_series_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::NewArticle::new(
        "title 3".try_into().unwrap(),
        "description 3".into(),
        Some("content 3".into()),
        Some(4.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let res = repo.create(article.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::SeriesNotFound(id) =>
            id == article.series_id.clone().unwrap().as_identifier(),
        _ => false,
    });
}

#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts("tags", "series", "categories")
    )
)]
async fn create_article_case_category_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::NewArticle::new(
        "title 4".try_into().unwrap(),
        "description 4".into(),
        Some("content 4".into()),
        Some(1.try_into().unwrap()),
        vec![4.try_into().unwrap(), 5.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );

    let res = repo.create(article.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::CategoryNotFound(id) =>
            id == article.category_ids[0].as_identifier(),
        _ => false,
    });

    let article = article::NewArticle::new(
        "title 4".try_into().unwrap(),
        "description 4".into(),
        Some("content 4".into()),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 5.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );

    let res = repo.create(article.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::CategoryNotFound(id) =>
            id == article.category_ids[1].as_identifier(),
        _ => false,
    });
}

#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts("tags", "series", "categories")
    )
)]
async fn create_article_case_tag_not_found(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::NewArticle::new(
        "title 4".try_into().unwrap(),
        "description 4".into(),
        Some("content 4".into()),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![4.try_into().unwrap(), 5.try_into().unwrap()],
    );

    let res = repo.create(article.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::TagNotFound(id) => id == article.tag_ids[0].as_identifier(),
        _ => false,
    });

    let article = article::NewArticle::new(
        "title 4".try_into().unwrap(),
        "description 4".into(),
        Some("content 4".into()),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 5.try_into().unwrap()],
    );

    let res = repo.create(article.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::TagNotFound(id) => id == article.tag_ids[1].as_identifier(),
        _ => false,
    });
}

#[sqlx::test(
    migrator = "crate::migrator::MIGRATOR",
    fixtures(
        path = "../../../../../../tests/fixtures",
        scripts("tags", "series", "categories", "articles")
    )
)]
async fn create_article_case_duplicate_slug(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::NewArticle::new(
        "article title 1".try_into().unwrap(),
        "description".into(),
        Some("content".into()),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let res = repo.create(article.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::DuplicateArticleSlug(slug) => slug == article.title.as_slug(),
        _ => false,
    });
}

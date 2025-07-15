use chaserland_articles_service_core::{
    domain::article::{
        entity::Article,
        repository::{ArticleRepository, ArticleRepositoryError},
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
async fn save_case_title(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.set_title("new article title 1".try_into().unwrap());

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
async fn save_case_title_duplicate(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.set_title("article title 2".try_into().unwrap());

    let res = repo.save(article.clone(), version, &mut tx).await;

    assert!(res.is_err(), "{:?}", res);

    let err = res.unwrap_err();

    assert!(
        match &err {
            ArticleRepositoryError::DuplicateArticleSlug(slug) => slug == article.slug(),
            _ => false,
        },
        "{:?}",
        err
    );
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
async fn save_case_description(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.set_description("new article description 1".into());

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
async fn save_case_content(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.set_content("new article content 1".into());

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
async fn save_case_series(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.set_series_id(2.try_into().unwrap());

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
async fn save_case_series_none(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.remove_series_id();

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
async fn save_case_series_not_found(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.set_series_id(4.try_into().unwrap());

    let res = repo.save(article.clone(), version, &mut tx).await;

    assert!(res.is_err(), "{:?}", res);

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::SeriesNotFound(id) =>
            id == article.series_id.unwrap().as_identifier(),
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
async fn save_case_publish(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.publish().unwrap();

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
            "article_tags",
            "publish_article"
        )
    )
)]
async fn save_case_unpublish(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    article.published_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.unpublish().unwrap();

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
async fn save_case_soft_delete(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let _ = article.soft_delete();

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
            "article_tags",
            "soft_delete_article"
        )
    )
)]
async fn save_case_revoke_soft_delete(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    article.deleted_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let _ = article.revoke_soft_delete();

    let res = repo.save(article, version, &mut tx).await;

    assert!(res.is_ok(), "{:?}", res);
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
async fn save_case_not_found(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let repo = PgArticleRepository::new();

    let mut article = Article::new(
        4.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
    );
    let version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    article.publish().unwrap();

    let res = repo.save(article.clone(), version, &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(id) => id == article.id.as_identifier(),
        _ => false,
    });
}

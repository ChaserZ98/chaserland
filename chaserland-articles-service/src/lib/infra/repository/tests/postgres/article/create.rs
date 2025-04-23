use crate::domain::entity::article;
use crate::domain::repository::article::{ArticleRepository, CreateArticleError};
use crate::infra::repository::postgres::article::PgArticleRepository;

#[sqlx::test(fixtures(
    path = "../../../../../../../tests/fixtures",
    scripts("tags", "series", "categories")
))]
async fn create_article_case_1(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::ArticleCreate {
        title: "title".try_into().unwrap(),
        description: "description".into(),
        content: Some("content".into()),
        series_id: None,
        category_ids: vec![],
        tag_ids: vec![],
        version: chrono::Utc::now().into(),
    };

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
#[sqlx::test(fixtures(
    path = "../../../../../../../tests/fixtures",
    scripts("tags", "series", "categories")
))]
async fn create_article_case_2(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::ArticleCreate {
        title: "title 1".try_into().unwrap(),
        description: "description".into(),
        content: Some("content".into()),
        series_id: Some(1.try_into().unwrap()),
        category_ids: vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        tag_ids: vec![2.try_into().unwrap(), 3.try_into().unwrap()],
        version: chrono::Utc::now().into(),
    };

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
#[sqlx::test(fixtures(
    path = "../../../../../../../tests/fixtures",
    scripts("tags", "series", "categories", "articles")
))]
async fn create_article_case_duplicate_slug(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let article = article::ArticleCreate {
        title: "article title 1".try_into().unwrap(),
        description: "description".into(),
        content: Some("content".into()),
        series_id: Some(1.try_into().unwrap()),
        category_ids: vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        tag_ids: vec![2.try_into().unwrap(), 3.try_into().unwrap()],
        version: chrono::Utc::now().into(),
    };

    let res = repo.create(article.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(matches!(err, CreateArticleError::DuplicateSlug(_)));
}

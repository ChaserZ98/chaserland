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
async fn get_one_case_1(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let res = repo
        .get_one(article::Identifier::Id(1.try_into().unwrap()), false, true)
        .await;

    let target = article::Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![],
        vec![],
        chrono::Utc::now().into(),
    );

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, target.id);
    assert_eq!(res.title, target.title);
    assert_eq!(res.slug(), target.slug());
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert!(target.created_at.value() - res.created_at.value() <= chrono::Duration::seconds(5));
    assert!(target.updated_at.value() - res.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series_id, target.series_id);
    assert_eq!(res.category_ids, target.category_ids);
    assert_eq!(res.tag_ids, target.tag_ids);
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
async fn get_one_case_2(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let res = repo
        .get_one(article::Identifier::Id(2.try_into().unwrap()), false, true)
        .await;

    let target = article::Article::new(
        2.try_into().unwrap(),
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        Some("article content 2".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(2.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
        chrono::Utc::now().into(),
    );

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, target.id);
    assert_eq!(res.title, target.title);
    assert_eq!(res.slug(), target.slug());
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert!(res.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(res.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series_id, target.series_id);
    assert_eq!(res.category_ids, target.category_ids);
    assert_eq!(res.tag_ids, target.tag_ids);
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
async fn get_one_case_3(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let identifier = article::Identifier::Id(3.try_into().unwrap());

    let res = repo.get_one(identifier.clone(), false, true).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleRepositoryError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });
}

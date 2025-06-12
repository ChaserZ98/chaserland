use crate::domain::article::{
    entity::Article,
    repository::{ArticleRepository, ArticleRepositoryError},
    vo as article,
};
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
async fn get_one_case_id_1(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let res = repo
        .get_one(article::Identifier::Id(1.try_into().unwrap()), false, true)
        .await;

    let target = Article::new(
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
async fn get_one_case_id_2(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let id = article::Id::new(2);
    let public_only = false;
    let with_content = false;

    let res = repo
        .get_one(id.as_identifier(), public_only, with_content)
        .await;

    let target = Article::new(
        id,
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        None,
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
async fn get_one_case_all_with_content_id(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let title = article::Title::new("article title 1");
    let public_only = false;
    let with_content = true;

    let target = Article::new(
        1.try_into().unwrap(),
        title.clone(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![],
        vec![],
        "2020-01-01 00:00:00 UTC".try_into().unwrap(),
    );

    let res = repo
        .get_one(title.as_slug().as_identifier(), public_only, with_content)
        .await;

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
    assert_eq!(res.version, target.version);
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
            "article_tags",
            "publish_article"
        )
    )
)]
async fn get_one_case_public_with_content_slug(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let title = article::Title::new("article title 1");
    let public_only = true;
    let with_content = true;

    let mut target = Article::new(
        1.try_into().unwrap(),
        title.clone(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![],
        vec![],
        "2020-01-01 00:00:00 UTC".try_into().unwrap(),
    );
    target.published_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());

    let res = repo
        .get_one(title.as_slug().as_identifier(), public_only, with_content)
        .await;

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
    assert_eq!(res.version, target.version);
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
            "article_tags",
            "publish_article"
        )
    )
)]
async fn get_one_case_public_without_content_slug(pool: sqlx::PgPool) {
    let repo = PgArticleRepository::new(pool);

    let title = article::Title::new("article title 1");
    let public_only = true;
    let with_content = false;

    let mut target = Article::new(
        1.try_into().unwrap(),
        title.clone(),
        "article description 1".into(),
        None,
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![],
        vec![],
        "2020-01-01 00:00:00 UTC".try_into().unwrap(),
    );
    target.published_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());

    let res = repo
        .get_one(title.as_slug().as_identifier(), public_only, with_content)
        .await;

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
    assert_eq!(res.version, target.version);
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
async fn get_one_case_all_id_not_found(pool: sqlx::PgPool) {
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

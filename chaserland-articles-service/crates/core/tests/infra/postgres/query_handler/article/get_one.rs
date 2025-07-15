use chaserland_articles_service_core::{
    app::query::{
        dto::{ArticleDTO, CategoryDTO, SeriesDTO, TagDTO},
        query_handler::{error::ArticleQueryHandlerError, interface::ArticleQueryHandler},
    },
    domain::article::vo as article,
    infra::postgres::query_handler::PgArticleQueryHandler,
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
async fn get_one_case_id_1(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let res = query_handler
        .get_one(article::Identifier::Id(1.try_into().unwrap()), false, true)
        .await;

    let target = ArticleDTO {
        id: 1.try_into().unwrap(),
        title: "article title 1".try_into().unwrap(),
        slug: "article-title-1".try_into().unwrap(),
        description: "article description 1".into(),
        content: Some("article content 1".into()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        deleted_at: None,
        published_at: None,
        series: Some(SeriesDTO {
            id: 1.try_into().unwrap(),
            slug: "series-1".into(),
            name: "series 1".into(),
        }),
        categories: vec![],
        tags: vec![],
    };

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, target.id);
    assert_eq!(res.title, target.title);
    assert_eq!(res.slug, target.slug);
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert!(res.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(res.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series, target.series);
    assert_eq!(res.categories, target.categories);
    assert_eq!(res.tags, target.tags);
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
async fn get_one_case_id_2(pool: sqlx::PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let id = article::Id::new(2);
    let public_only = false;
    let with_content = false;

    let res = query_handler
        .get_one(id.as_identifier(), public_only, with_content)
        .await;

    let target = ArticleDTO {
        id: 2.try_into().unwrap(),
        title: "article title 2".try_into().unwrap(),
        slug: "article-title-2".try_into().unwrap(),
        description: "article description 2".into(),
        content: None,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        deleted_at: None,
        published_at: None,
        series: Some(SeriesDTO {
            id: 2.try_into().unwrap(),
            slug: "series-2".into(),
            name: "series 2".into(),
        }),
        categories: vec![
            CategoryDTO {
                id: 1.try_into().unwrap(),
                slug: "category-1".into(),
                name: "Category 1".into(),
            },
            CategoryDTO {
                id: 2.try_into().unwrap(),
                slug: "category-2".into(),
                name: "Category 2".into(),
            },
        ],
        tags: vec![
            TagDTO {
                id: 2.try_into().unwrap(),
                slug: "tag-2".into(),
                name: "Tag 2".into(),
            },
            TagDTO {
                id: 3.try_into().unwrap(),
                slug: "tag-3".into(),
                name: "Tag 3".into(),
            },
        ],
    };

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, target.id);
    assert_eq!(res.title, target.title);
    assert_eq!(res.slug, target.slug);
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert!(res.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(res.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series, target.series);
    assert_eq!(res.categories, target.categories);
    assert_eq!(res.tags, target.tags);
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
async fn get_one_case_all_with_content_id(pool: sqlx::PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let title = article::Title::new("article title 1");
    let public_only = false;
    let with_content = true;

    let target = ArticleDTO {
        id: 1.try_into().unwrap(),
        title: "article title 1".try_into().unwrap(),
        slug: "article-title-1".try_into().unwrap(),
        description: "article description 1".into(),
        content: Some("article content 1".into()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        deleted_at: None,
        published_at: None,
        series: Some(SeriesDTO {
            id: 1.try_into().unwrap(),
            slug: "series-1".into(),
            name: "series 1".into(),
        }),
        categories: vec![],
        tags: vec![],
    };

    let res = query_handler
        .get_one(title.as_slug().as_identifier(), public_only, with_content)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, target.id);
    assert_eq!(res.title, target.title);
    assert_eq!(res.slug, target.slug);
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert!(res.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(res.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series, target.series);
    assert_eq!(res.categories, target.categories);
    assert_eq!(res.tags, target.tags);
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
async fn get_one_case_public_with_content_slug(pool: sqlx::PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let title = article::Title::new("article title 1");
    let public_only = true;
    let with_content = true;

    let mut target = ArticleDTO {
        id: 1.try_into().unwrap(),
        title: "article title 1".try_into().unwrap(),
        slug: "article-title-1".try_into().unwrap(),
        description: "article description 1".into(),
        content: Some("article content 1".into()),
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        deleted_at: None,
        published_at: None,
        series: Some(SeriesDTO {
            id: 1.try_into().unwrap(),
            slug: "series-1".into(),
            name: "series 1".into(),
        }),
        categories: vec![],
        tags: vec![],
    };
    target.published_at = Some("2020-01-01 00:00:00 UTC".parse().unwrap());

    let res = query_handler
        .get_one(title.as_slug().as_identifier(), public_only, with_content)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, target.id);
    assert_eq!(res.title, target.title);
    assert_eq!(res.slug, target.slug);
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert!(res.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(res.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series, target.series);
    assert_eq!(res.categories, target.categories);
    assert_eq!(res.tags, target.tags);
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
async fn get_one_case_public_without_content_slug(pool: sqlx::PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let title = article::Title::new("article title 1");
    let public_only = true;
    let with_content = false;

    let mut target = ArticleDTO {
        id: 1.try_into().unwrap(),
        title: "article title 1".try_into().unwrap(),
        slug: "article-title-1".try_into().unwrap(),
        description: "article description 1".into(),
        content: None,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        deleted_at: None,
        published_at: None,
        series: Some(SeriesDTO {
            id: 1.try_into().unwrap(),
            slug: "series-1".into(),
            name: "series 1".into(),
        }),
        categories: vec![],
        tags: vec![],
    };
    target.published_at = Some("2020-01-01 00:00:00 UTC".parse().unwrap());

    let res = query_handler
        .get_one(title.as_slug().as_identifier(), public_only, with_content)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, target.id);
    assert_eq!(res.title, target.title);
    assert_eq!(res.slug, target.slug);
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert!(res.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(res.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series, target.series);
    assert_eq!(res.categories, target.categories);
    assert_eq!(res.tags, target.tags);
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
async fn get_one_case_all_id_not_found(pool: sqlx::PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let identifier = article::Identifier::Id(3.try_into().unwrap());

    let res = query_handler.get_one(identifier.clone(), false, true).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        ArticleQueryHandlerError::ArticleNotFound(value) => value == identifier,
        _ => false,
    });
}

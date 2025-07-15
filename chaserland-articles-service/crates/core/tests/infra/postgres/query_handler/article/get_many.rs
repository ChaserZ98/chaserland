use chaserland_articles_service_core::{
    app::query::{
        dto::{ArticleDTO, CategoryDTO, SeriesDTO, TagDTO},
        query_handler::interface::ArticleQueryHandler,
    },
    domain::{article::repository::ArticlesFilter, series::vo as series},
    infra::postgres::query_handler::PgArticleQueryHandler,
    migrator::MIGRATOR,
};
use chaserland_common::pagination::Pagination;
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
async fn get_many_case_pagination(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let mut pagination = Pagination::new(1.try_into().unwrap(), 1.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let filter = None;

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

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

    assert_eq!(res[0].id, target.id);
    assert_eq!(res[0].title, target.title);
    assert_eq!(res[0].slug, target.slug);
    assert_eq!(res[0].description, target.description);
    assert_eq!(res[0].content, target.content);
    assert!(res[0].created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(res[0].updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(res[0].deleted_at, target.deleted_at);
    assert_eq!(res[0].published_at, target.published_at);
    assert_eq!(res[0].series, target.series);
    assert_eq!(res[0].categories, target.categories);
    assert_eq!(res[0].tags, target.tags);

    pagination.page = 2.try_into().unwrap();
    let public_only = false;
    let with_content = true;
    let filter = None;

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = ArticleDTO {
        id: 2.try_into().unwrap(),
        title: "article title 2".try_into().unwrap(),
        slug: "article-title-2".try_into().unwrap(),
        description: "article description 2".into(),
        content: Some("article content 2".into()),
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

    let article = &res[0];

    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);
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
async fn get_many_case_all_without_content_no_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = false;
    let filter = None;

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = ArticleDTO {
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

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);

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

    let article = &res[1];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);
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
async fn get_many_case_all_with_content_no_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let filter = None;

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok(), "{:?}", res);

    let res = res.unwrap();

    assert_eq!(res.len(), 2, "{:?}", res);

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

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);

    let target = ArticleDTO {
        id: 2.try_into().unwrap(),
        title: "article title 2".try_into().unwrap(),
        slug: "article-title-2".try_into().unwrap(),
        description: "article description 2".into(),
        content: Some("article content 2".into()),
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

    let article = &res[1];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);
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
async fn get_many_case_all_with_content_with_series_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let target = ArticleDTO {
        id: 2.try_into().unwrap(),
        title: "article title 2".try_into().unwrap(),
        slug: "article-title-2".try_into().unwrap(),
        description: "article description 2".into(),
        content: Some("article content 2".into()),
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

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let series_identifier = Some(series::Identifier::Id(2.try_into().unwrap()));
    let filter = Some(ArticlesFilter::new(series_identifier, vec![], vec![]));

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let series_identifier = Some(series::Identifier::Slug(
        series::Name::new("series 2").as_slug(),
    ));
    let filter = Some(ArticlesFilter::new(series_identifier, vec![], vec![]));

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);
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
async fn get_many_case_all_with_content_with_category_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let category_ids = vec![1.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(None, category_ids, vec![]));

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = ArticleDTO {
        id: 2.try_into().unwrap(),
        title: "article title 2".try_into().unwrap(),
        slug: "article-title-2".try_into().unwrap(),
        description: "article description 2".into(),
        content: Some("article content 2".into()),
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

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let category_ids = vec![1.try_into().unwrap(), 3.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(None, category_ids, vec![]));

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
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
async fn get_many_case_all_with_content_with_tag_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let tag_ids = vec![2.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(None, vec![], tag_ids));

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = ArticleDTO {
        id: 2.try_into().unwrap(),
        title: "article title 2".try_into().unwrap(),
        slug: "article-title-2".try_into().unwrap(),
        description: "article description 2".into(),
        content: Some("article content 2".into()),
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

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let tag_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(None, vec![], tag_ids));

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
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
async fn get_many_case_all_with_content_with_series_category_tag_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let series_id = Some(series::Id::new(2).as_identifier());
    let category_ids = vec![1.try_into().unwrap()];
    let tag_ids = vec![2.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(series_id, category_ids, tag_ids));

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = ArticleDTO {
        id: 2.try_into().unwrap(),
        title: "article title 2".try_into().unwrap(),
        slug: "article-title-2".try_into().unwrap(),
        description: "article description 2".into(),
        content: Some("article content 2".into()),
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

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);
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
async fn get_many_case_public_no_content_no_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = true;
    let with_content = false;
    let filter = None;

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
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
async fn get_many_case_public_with_content_no_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = true;
    let with_content = true;
    let filter = None;

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

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

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);
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
async fn get_many_case_public_with_content_with_series_filter(pool: PgPool) {
    let query_handler = PgArticleQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = true;
    let with_content = true;
    let series_identifier = series::Identifier::Id(1.try_into().unwrap());
    let filter = Some(ArticlesFilter::new(Some(series_identifier), vec![], vec![]));

    let res = query_handler
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

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

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug, target.slug);
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at - target.created_at <= chrono::Duration::seconds(5));
    assert!(article.updated_at - target.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series, target.series);
    assert_eq!(article.categories, target.categories);
    assert_eq!(article.tags, target.tags);
}

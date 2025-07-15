use crate::common::init_query_service;
use chaserland_articles_service_core::{
    app::query::{self as query, dto, interface::ArticleQueryService},
    domain::article::vo as article,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../tests/fixtures",
        scripts(
            "series",
            "categories",
            "tags",
            "articles",
            "article_categories",
            "article_tags"
        )
    )
)]
async fn get_article_case_1(pool: PgPool) {
    let query_service = init_query_service(pool);

    let article_dto = dto::ArticleDTO {
        id: 1,
        title: "article title 1".to_string(),
        slug: "article-title-1".to_string(),
        description: "article description 1".to_string(),
        content: Some("article content 1".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        deleted_at: None,
        published_at: None,
        series: Some(dto::SeriesDTO {
            id: 1,
            name: "series 1".to_string(),
            slug: "series-1".to_string(),
        }),
        tags: vec![],
        categories: vec![],
    };

    let query = query::GetArticleOneQuery {
        identifier: article::Identifier::Id(article_dto.id.try_into().unwrap()),
        public_only: false,
        with_content: true,
    };

    let res = query_service.get_article_one(query).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, article_dto.id);
    assert_eq!(res.title, article_dto.title);
    assert_eq!(res.description, article_dto.description);
    assert_eq!(res.content, article_dto.content);
    assert_eq!(res.series, article_dto.series);
    assert_eq!(res.categories, article_dto.categories);
    assert_eq!(res.tags, article_dto.tags);
    assert!(article_dto.created_at - res.created_at <= chrono::Duration::seconds(5));
    assert!(article_dto.updated_at - res.updated_at <= chrono::Duration::seconds(5));
    assert_eq!(res.published_at, article_dto.published_at);
    assert_eq!(res.deleted_at, article_dto.deleted_at);
}

use crate::common::{init_command_service, init_query_service};
use chaserland_articles_service_core::{
    app::{
        command::{self, interface::ArticleCommandService},
        query::{self, dto::ArticleDTO, interface::ArticleQueryService},
    },
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
async fn create_article_case_1(pool: PgPool) {
    let command_service = init_command_service(pool.clone());
    let query_service = init_query_service(pool.clone());

    let command = command::CreateArticleCommand {
        title: "article title 3".try_into().unwrap(),
        description: "article description 3".into(),
        content: Some("article content 3".into()),
        series_id: None,
        category_ids: vec![],
        tag_ids: vec![],
    };

    let res = command_service.create_article(command).await;

    assert!(res.is_ok(), "res: {:#?}", res);

    let query = query::GetArticleOneQuery {
        identifier: article::Id::new(3).as_identifier(),
        public_only: false,
        with_content: true,
    };

    let res = query_service.get_article_one(query).await;

    assert!(res.is_ok(), "res: {:#?}", res);

    let res = res.unwrap();
    let target = ArticleDTO {
        id: 3,
        title: "article title 3".to_string(),
        slug: "article-title-3".to_string(),
        description: "article description 3".to_string(),
        content: Some("article content 3".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        deleted_at: None,
        published_at: None,
        series: None,
        categories: vec![],
        tags: vec![],
    };

    assert_eq!(res.title, target.title);
    assert_eq!(res.slug, target.slug);
    assert_eq!(res.description, target.description);
    assert_eq!(res.content, target.content);
    assert!(target.created_at - res.created_at < chrono::Duration::seconds(5));
    assert!(target.created_at - res.updated_at < chrono::Duration::seconds(5));
    assert_eq!(res.deleted_at, target.deleted_at);
    assert_eq!(res.published_at, target.published_at);
    assert_eq!(res.series, target.series);
    assert_eq!(res.categories, target.categories);
    assert_eq!(res.tags, target.tags);
}

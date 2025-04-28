use crate::{
    app::service::ArticleService,
    domain::entity::article,
    infra::repository::postgres::{
        article::PgArticleRepository, category::PgCategoryRepository, series::PgSeriesRepository,
        tag::PgTagRepository,
    },
};
use sqlx::PgPool;

#[sqlx::test(fixtures(
    path = "../../../../../tests/fixtures",
    scripts(
        "series",
        "categories",
        "tags",
        "articles",
        "article_categories",
        "article_tags"
    )
))]
async fn get_article(pool: PgPool) {
    let article_repository = PgArticleRepository::new(pool.clone());
    let series_repository = PgSeriesRepository::new(pool.clone());
    let category_repository = PgCategoryRepository::new(pool.clone());
    let tag_repository = PgTagRepository::new(pool.clone());
    let service = ArticleService::new(
        article_repository,
        series_repository,
        category_repository,
        tag_repository,
    );
    let mut article = article::Article::default();
    article.id = 1.try_into().unwrap();
    article.set_title("article title 1".try_into().unwrap());
    article.set_description("article description 1".try_into().unwrap());
    article.set_content("article content 1".into());
    article.set_series_id(1.try_into().unwrap());
    article.category_ids = vec![];
    article.tag_ids = vec![];
    article.created_at = chrono::Utc::now().into();
    article.updated_at = chrono::Utc::now().into();
    article.version = "2020-01-01 00:00:00 UTC".try_into().unwrap();

    let res = service
        .get_article(article.id.as_identifier(), false, true)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.id, article.id);
    assert_eq!(res.title, article.title);
    assert_eq!(res.description, article.description);
    assert_eq!(res.content, article.content);
    assert_eq!(res.series_id, article.series_id);
    assert_eq!(res.category_ids, article.category_ids);
    assert_eq!(res.tag_ids, article.tag_ids);
    assert!(article.created_at.value() - res.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - res.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(res.published_at, article.published_at);
    assert_eq!(res.deleted_at, article.deleted_at);
    assert_eq!(res.version, article.version);
}

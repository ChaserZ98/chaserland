use crate::domain::article::{
    entity::Article,
    repository::{ArticleRepository, ArticlesFilter},
};
use crate::domain::series::vo as series;
use crate::infra::repository::postgres::article::PgArticleRepository;
use chaserland_common::pagination::Pagination;
use sqlx::PgPool;

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
async fn get_many_case_pagination(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let mut pagination = Pagination::new(1.try_into().unwrap(), 1.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let filter = None;

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

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
    );

    assert_eq!(res[0].id, target.id);
    assert_eq!(res[0].title, target.title);
    assert_eq!(res[0].slug(), target.slug());
    assert_eq!(res[0].description, target.description);
    assert_eq!(res[0].content, target.content);
    assert!(res[0].created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(res[0].updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(res[0].deleted_at, target.deleted_at);
    assert_eq!(res[0].published_at, target.published_at);
    assert_eq!(res[0].series_id, target.series_id);
    assert_eq!(res[0].category_ids, target.category_ids);
    assert_eq!(res[0].tag_ids, target.tag_ids);

    pagination.page = 2.try_into().unwrap();
    let public_only = false;
    let with_content = true;
    let filter = None;

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = Article::new(
        2.try_into().unwrap(),
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        Some("article content 2".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(2.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let article = &res[0];

    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug(), target.slug());
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series_id, target.series_id);
    assert_eq!(article.category_ids, target.category_ids);
    assert_eq!(article.tag_ids, target.tag_ids);
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
async fn get_many_case_all_without_content_no_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = false;
    let filter = None;

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target_1 = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        None,
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![],
        vec![],
    );

    let target_2 = Article::new(
        2.try_into().unwrap(),
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        None,
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(2.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let article_1 = &res[0];
    assert_eq!(article_1.id, target_1.id);
    assert_eq!(article_1.title, target_1.title);
    assert_eq!(article_1.slug(), target_1.slug());
    assert_eq!(article_1.description, target_1.description);
    assert_eq!(article_1.content, target_1.content);
    assert!(
        article_1.created_at.value() - target_1.created_at.value() <= chrono::Duration::seconds(5)
    );
    assert!(
        article_1.updated_at.value() - target_1.updated_at.value() <= chrono::Duration::seconds(5)
    );
    assert_eq!(article_1.deleted_at, target_1.deleted_at);
    assert_eq!(article_1.published_at, target_1.published_at);
    assert_eq!(article_1.series_id, target_1.series_id);
    assert_eq!(article_1.category_ids, target_1.category_ids);
    assert_eq!(article_1.tag_ids, target_1.tag_ids);

    let article_2 = &res[1];
    assert_eq!(article_2.id, target_2.id);
    assert_eq!(article_2.title, target_2.title);
    assert_eq!(article_2.slug(), target_2.slug());
    assert_eq!(article_2.description, target_2.description);
    assert_eq!(article_2.content, target_2.content);
    assert!(
        article_2.created_at.value() - target_2.created_at.value() <= chrono::Duration::seconds(5)
    );
    assert!(
        article_2.updated_at.value() - target_2.updated_at.value() <= chrono::Duration::seconds(5)
    );
    assert_eq!(article_2.deleted_at, target_2.deleted_at);
    assert_eq!(article_2.published_at, target_2.published_at);
    assert_eq!(article_2.series_id, target_2.series_id);
    assert_eq!(article_2.category_ids, target_2.category_ids);
    assert_eq!(article_2.tag_ids, target_2.tag_ids);
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
async fn get_many_case_all_with_content_no_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let filter = None;

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target_1 = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![],
        vec![],
    );

    let target_2 = Article::new(
        2.try_into().unwrap(),
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        Some("article content 2".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(2.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let article_1 = &res[0];
    assert_eq!(article_1.id, target_1.id);
    assert_eq!(article_1.title, target_1.title);
    assert_eq!(article_1.slug(), target_1.slug());
    assert_eq!(article_1.description, target_1.description);
    assert_eq!(article_1.content, target_1.content);
    assert!(
        article_1.created_at.value() - target_1.created_at.value() <= chrono::Duration::seconds(5)
    );
    assert!(
        article_1.updated_at.value() - target_1.updated_at.value() <= chrono::Duration::seconds(5)
    );
    assert_eq!(article_1.deleted_at, target_1.deleted_at);
    assert_eq!(article_1.published_at, target_1.published_at);
    assert_eq!(article_1.series_id, target_1.series_id);
    assert_eq!(article_1.category_ids, target_1.category_ids);
    assert_eq!(article_1.tag_ids, target_1.tag_ids);

    let article_2 = &res[1];
    assert_eq!(article_2.id, target_2.id);
    assert_eq!(article_2.title, target_2.title);
    assert_eq!(article_2.slug(), target_2.slug());
    assert_eq!(article_2.description, target_2.description);
    assert_eq!(article_2.content, target_2.content);
    assert!(
        article_2.created_at.value() - target_2.created_at.value() <= chrono::Duration::seconds(5)
    );
    assert!(
        article_2.updated_at.value() - target_2.updated_at.value() <= chrono::Duration::seconds(5)
    );
    assert_eq!(article_2.deleted_at, target_2.deleted_at);
    assert_eq!(article_2.published_at, target_2.published_at);
    assert_eq!(article_2.series_id, target_2.series_id);
    assert_eq!(article_2.category_ids, target_2.category_ids);
    assert_eq!(article_2.tag_ids, target_2.tag_ids);
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
async fn get_many_case_all_with_content_with_series_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let target = Article::new(
        2.try_into().unwrap(),
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        Some("article content 2".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(2.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let series_identifier = Some(series::Identifier::Id(2.try_into().unwrap()));
    let filter = Some(ArticlesFilter::new(series_identifier, vec![], vec![]));

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug(), target.slug());
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series_id, target.series_id);
    assert_eq!(article.category_ids, target.category_ids);
    assert_eq!(article.tag_ids, target.tag_ids);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let series_identifier = Some(series::Identifier::Slug(
        series::Name::new("series 2").as_slug(),
    ));
    let filter = Some(ArticlesFilter::new(series_identifier, vec![], vec![]));

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug(), target.slug());
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series_id, target.series_id);
    assert_eq!(article.category_ids, target.category_ids);
    assert_eq!(article.tag_ids, target.tag_ids);
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
async fn get_many_case_all_with_content_with_category_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let category_ids = vec![1.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(None, category_ids, vec![]));

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = Article::new(
        2.try_into().unwrap(),
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        Some("article content 2".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(2.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug(), target.slug());
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series_id, target.series_id);
    assert_eq!(article.category_ids, target.category_ids);
    assert_eq!(article.tag_ids, target.tag_ids);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let category_ids = vec![1.try_into().unwrap(), 3.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(None, category_ids, vec![]));

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
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
async fn get_many_case_all_with_content_with_tag_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let tag_ids = vec![2.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(None, vec![], tag_ids));

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = Article::new(
        2.try_into().unwrap(),
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        Some("article content 2".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(2.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug(), target.slug());
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series_id, target.series_id);
    assert_eq!(article.category_ids, target.category_ids);
    assert_eq!(article.tag_ids, target.tag_ids);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let tag_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(None, vec![], tag_ids));

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
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
async fn get_many_case_all_with_content_with_series_category_tag_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = false;
    let with_content = true;
    let series_id = Some(series::Id::new(2).as_identifier());
    let category_ids = vec![1.try_into().unwrap()];
    let tag_ids = vec![2.try_into().unwrap()];
    let filter = Some(ArticlesFilter::new(series_id, category_ids, tag_ids));

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = Article::new(
        2.try_into().unwrap(),
        "article title 2".try_into().unwrap(),
        "article description 2".into(),
        Some("article content 2".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(2.try_into().unwrap()),
        vec![1.try_into().unwrap(), 2.try_into().unwrap()],
        vec![2.try_into().unwrap(), 3.try_into().unwrap()],
    );

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug(), target.slug());
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series_id, target.series_id);
    assert_eq!(article.category_ids, target.category_ids);
    assert_eq!(article.tag_ids, target.tag_ids);
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
async fn get_many_case_public_no_content_no_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = true;
    let with_content = false;
    let filter = None;

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
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
async fn get_many_case_public_with_content_no_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = true;
    let with_content = true;
    let filter = None;

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let mut target = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![],
        vec![],
    );
    target.published_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug(), target.slug());
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series_id, target.series_id);
    assert_eq!(article.category_ids, target.category_ids);
    assert_eq!(article.tag_ids, target.tag_ids);
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
async fn get_many_case_public_with_content_with_series_filter(pool: PgPool) {
    let repo = PgArticleRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());
    let public_only = true;
    let with_content = true;
    let series_identifier = series::Identifier::Id(1.try_into().unwrap());
    let filter = Some(ArticlesFilter::new(Some(series_identifier), vec![], vec![]));

    let res = repo
        .get_many(pagination, public_only, with_content, filter)
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let mut target = Article::new(
        1.try_into().unwrap(),
        "article title 1".try_into().unwrap(),
        "article description 1".into(),
        Some("article content 1".into()),
        chrono::Utc::now().into(),
        chrono::Utc::now().into(),
        Some(1.try_into().unwrap()),
        vec![],
        vec![],
    );
    target.published_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());

    let article = &res[0];
    assert_eq!(article.id, target.id);
    assert_eq!(article.title, target.title);
    assert_eq!(article.slug(), target.slug());
    assert_eq!(article.description, target.description);
    assert_eq!(article.content, target.content);
    assert!(article.created_at.value() - target.created_at.value() <= chrono::Duration::seconds(5));
    assert!(article.updated_at.value() - target.updated_at.value() <= chrono::Duration::seconds(5));
    assert_eq!(article.deleted_at, target.deleted_at);
    assert_eq!(article.published_at, target.published_at);
    assert_eq!(article.series_id, target.series_id);
    assert_eq!(article.category_ids, target.category_ids);
    assert_eq!(article.tag_ids, target.tag_ids);
}

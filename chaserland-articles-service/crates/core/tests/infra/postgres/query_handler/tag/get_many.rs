use chaserland_articles_service_core::{
    app::query::{dto::TagDTO, query_handler::interface::TagQueryHandler},
    domain::tag::repository::TagsFilter,
    infra::postgres::query_handler::PgTagQueryHandler,
    migrator::MIGRATOR,
};
use chaserland_common::pagination::Pagination;
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_many_case_pagination_1(pool: PgPool) {
    let query_handler = PgTagQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());

    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 3);

    let target = TagDTO {
        id: 1,
        name: "Tag 1".into(),
        slug: "tag-1".into(),
    };
    assert_eq!(res[0], target);

    let target = TagDTO {
        id: 2,
        name: "Tag 2".into(),
        slug: "tag-2".into(),
    };
    assert_eq!(res[1], target);

    let target = TagDTO {
        id: 3,
        name: "Tag 3".into(),
        slug: "tag-3".into(),
    };
    assert_eq!(res[2], target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_many_case_pagination_2(pool: PgPool) {
    let query_handler = PgTagQueryHandler::new(pool);

    let mut pagination = Pagination::new(1.try_into().unwrap(), 2.try_into().unwrap());

    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = TagDTO {
        id: 1,
        name: "Tag 1".into(),
        slug: "tag-1".into(),
    };
    assert_eq!(res[0], target);

    let target = TagDTO {
        id: 2,
        name: "Tag 2".into(),
        slug: "tag-2".into(),
    };
    assert_eq!(res[1], target);

    pagination.page = 2.try_into().unwrap();

    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = TagDTO {
        id: 3,
        name: "Tag 3".into(),
        slug: "tag-3".into(),
    };
    assert_eq!(res[0], target);

    pagination.page = 3.try_into().unwrap();

    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_many_case_filter_1(pool: PgPool) {
    let query_handler = PgTagQueryHandler::new(pool);

    let filter = TagsFilter::new(vec![1.try_into().unwrap(), 2.try_into().unwrap()]);

    let res = query_handler.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = TagDTO {
        id: 1,
        name: "Tag 1".into(),
        slug: "tag-1".into(),
    };
    assert_eq!(res[0], target);

    let target = TagDTO {
        id: 2,
        name: "Tag 2".into(),
        slug: "tag-2".into(),
    };
    assert_eq!(res[1], target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_many_case_filter_2(pool: PgPool) {
    let query_handler = PgTagQueryHandler::new(pool);

    let filter = TagsFilter::new(vec![1.try_into().unwrap(), 3.try_into().unwrap()]);

    let res = query_handler.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = TagDTO {
        id: 1,
        name: "Tag 1".into(),
        slug: "tag-1".into(),
    };
    assert_eq!(res[0], target);

    let target = TagDTO {
        id: 3,
        name: "Tag 3".into(),
        slug: "tag-3".into(),
    };
    assert_eq!(res[1], target);
}

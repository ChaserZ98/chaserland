use chaserland_articles_service_core::{
    app::query::{dto::CategoryDTO, query_handler::interface::CategoryQueryHandler},
    domain::category::repository::CategoriesFilter,
    infra::postgres::query_handler::PgCategoryQueryHandler,
    migrator::MIGRATOR,
};
use chaserland_common::pagination::Pagination;
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn get_many_case_pagination_1(pool: PgPool) {
    let query_handler = PgCategoryQueryHandler::new(pool);

    let pagination = Some(Pagination::new(
        1.try_into().unwrap(),
        10.try_into().unwrap(),
    ));

    let res = query_handler.get_many(None, pagination).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 3);

    let target = CategoryDTO {
        id: 1,
        name: "Category 1".into(),
        slug: "category-1".into(),
    };
    assert_eq!(res[0], target);

    let target = CategoryDTO {
        id: 2,
        name: "Category 2".into(),
        slug: "category-2".into(),
    };
    assert_eq!(res[1], target);

    let target = CategoryDTO {
        id: 3,
        name: "Category 3".into(),
        slug: "category-3".into(),
    };
    assert_eq!(res[2], target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn get_many_case_pagination_2(pool: PgPool) {
    let query_handler = PgCategoryQueryHandler::new(pool);

    let mut pagination = Pagination::new(1.try_into().unwrap(), 2.try_into().unwrap());

    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = CategoryDTO {
        id: 1,
        name: "Category 1".into(),
        slug: "category-1".into(),
    };
    assert_eq!(res[0], target);

    let target = CategoryDTO {
        id: 2,
        name: "Category 2".into(),
        slug: "category-2".into(),
    };
    assert_eq!(res[1], target);

    pagination.page = 2.try_into().unwrap();

    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = CategoryDTO {
        id: 3,
        name: "Category 3".into(),
        slug: "category-3".into(),
    };
    assert_eq!(res[0], target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn get_many_case_pagination_filter_1(pool: PgPool) {
    let query_handler = PgCategoryQueryHandler::new(pool);

    let filter = CategoriesFilter::new(vec![
        1.try_into().unwrap(),
        2.try_into().unwrap(),
        3.try_into().unwrap(),
    ]);

    let res = query_handler.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 3);

    let target = CategoryDTO {
        id: 1,
        name: "Category 1".into(),
        slug: "category-1".into(),
    };
    assert_eq!(res[0], target);

    let target = CategoryDTO {
        id: 2,
        name: "Category 2".into(),
        slug: "category-2".into(),
    };
    assert_eq!(res[1], target);

    let target = CategoryDTO {
        id: 3,
        name: "Category 3".into(),
        slug: "category-3".into(),
    };
    assert_eq!(res[2], target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn get_many_case_pagination_filter_2(pool: PgPool) {
    let query_handler = PgCategoryQueryHandler::new(pool);

    let filter = CategoriesFilter::new(vec![
        1.try_into().unwrap(),
        3.try_into().unwrap(),
        4.try_into().unwrap(),
    ]);

    let res = query_handler.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = CategoryDTO {
        id: 1,
        name: "Category 1".into(),
        slug: "category-1".into(),
    };
    assert_eq!(res[0], target);

    let target = CategoryDTO {
        id: 3,
        name: "Category 3".into(),
        slug: "category-3".into(),
    };
    assert_eq!(res[1], target);
}

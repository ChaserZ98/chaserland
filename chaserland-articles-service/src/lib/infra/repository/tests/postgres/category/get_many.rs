use crate::domain::category::{
    entity::Category,
    repository::{CategoriesFilter, CategoryRepository},
};
use crate::infra::repository::postgres::category::PgCategoryRepository;
use chaserland_common::pagination::Pagination;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn get_many_case_pagination_1(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let pagination = Some(Pagination::new(
        1.try_into().unwrap(),
        10.try_into().unwrap(),
    ));

    let res = repo.get_many(None, pagination).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 3);

    let target = Category::new(1.try_into().unwrap(), "Category 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = Category::new(2.try_into().unwrap(), "Category 2".try_into().unwrap());
    assert_eq!(res[1], target);

    let target = Category::new(3.try_into().unwrap(), "Category 3".try_into().unwrap());
    assert_eq!(res[2], target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn get_many_case_pagination_2(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let mut pagination = Pagination::new(1.try_into().unwrap(), 2.try_into().unwrap());

    let res = repo.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = Category::new(1.try_into().unwrap(), "Category 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = Category::new(2.try_into().unwrap(), "Category 2".try_into().unwrap());
    assert_eq!(res[1], target);

    pagination.page = 2.try_into().unwrap();

    let res = repo.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = Category::new(3.try_into().unwrap(), "Category 3".try_into().unwrap());
    assert_eq!(res[0], target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn get_many_case_pagination_filter_1(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let filter = CategoriesFilter::new(vec![
        1.try_into().unwrap(),
        2.try_into().unwrap(),
        3.try_into().unwrap(),
    ]);

    let res = repo.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 3);

    let target = Category::new(1.try_into().unwrap(), "Category 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = Category::new(2.try_into().unwrap(), "Category 2".try_into().unwrap());
    assert_eq!(res[1], target);

    let target = Category::new(3.try_into().unwrap(), "Category 3".try_into().unwrap());
    assert_eq!(res[2], target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn get_many_case_pagination_filter_2(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let filter = CategoriesFilter::new(vec![
        1.try_into().unwrap(),
        3.try_into().unwrap(),
        4.try_into().unwrap(),
    ]);

    let res = repo.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = Category::new(1.try_into().unwrap(), "Category 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = Category::new(3.try_into().unwrap(), "Category 3".try_into().unwrap());
    assert_eq!(res[1], target);
}

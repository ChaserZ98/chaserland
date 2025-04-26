use crate::domain::entity::category;
use crate::domain::repository::category::CategoryRepository;
use crate::infra::repository::postgres::category::PgCategoryRepository;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn get_many_case_1(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let page = 1.try_into().unwrap();
    let page_size = 10.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 3);

    let target = category::Category::new(1.try_into().unwrap(), "Category 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = category::Category::new(2.try_into().unwrap(), "Category 2".try_into().unwrap());
    assert_eq!(res[1], target);

    let target = category::Category::new(3.try_into().unwrap(), "Category 3".try_into().unwrap());
    assert_eq!(res[2], target);
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn get_many_case_2(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let page = 1.try_into().unwrap();
    let page_size = 2.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = category::Category::new(1.try_into().unwrap(), "Category 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = category::Category::new(2.try_into().unwrap(), "Category 2".try_into().unwrap());
    assert_eq!(res[1], target);

    let page = 2.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = category::Category::new(3.try_into().unwrap(), "Category 3".try_into().unwrap());
    assert_eq!(res[0], target);
}

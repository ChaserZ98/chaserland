use crate::{
    domain::{entity::tag, repository::tag::TagRepository},
    infra::repository::postgres::tag::PgTagRepository,
};
use chaserland_common::pagination::{Page, PageSize};
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn get_many_case_1(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let page: Page = 1.try_into().unwrap();
    let page_size: PageSize = 10.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 3);

    let target = tag::Tag::new(1.try_into().unwrap(), "Tag 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = tag::Tag::new(2.try_into().unwrap(), "Tag 2".try_into().unwrap());
    assert_eq!(res[1], target);

    let target = tag::Tag::new(3.try_into().unwrap(), "Tag 3".try_into().unwrap());
    assert_eq!(res[2], target);
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn get_many_case_2(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let page: Page = 1.try_into().unwrap();
    let page_size: PageSize = 2.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = tag::Tag::new(1.try_into().unwrap(), "Tag 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = tag::Tag::new(2.try_into().unwrap(), "Tag 2".try_into().unwrap());
    assert_eq!(res[1], target);

    let page: Page = 2.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = tag::Tag::new(3.try_into().unwrap(), "Tag 3".try_into().unwrap());
    assert_eq!(res[0], target);

    let page: Page = 3.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
}

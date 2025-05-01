use crate::{
    domain::{
        entity::tag,
        repository::tag::{TagRepository, TagsFilter},
    },
    infra::repository::postgres::tag::PgTagRepository,
};
use chaserland_common::pagination::Pagination;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn get_many_case_pagination_1(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());

    let res = repo.get_many(None, Some(pagination)).await;

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
async fn get_many_case_pagination_2(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let mut pagination = Pagination::new(1.try_into().unwrap(), 2.try_into().unwrap());

    let res = repo.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = tag::Tag::new(1.try_into().unwrap(), "Tag 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = tag::Tag::new(2.try_into().unwrap(), "Tag 2".try_into().unwrap());
    assert_eq!(res[1], target);

    pagination.page = 2.try_into().unwrap();

    let res = repo.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = tag::Tag::new(3.try_into().unwrap(), "Tag 3".try_into().unwrap());
    assert_eq!(res[0], target);

    pagination.page = 3.try_into().unwrap();

    let res = repo.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 0);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn get_many_case_filter_1(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let filter = TagsFilter::new(vec![1.try_into().unwrap(), 2.try_into().unwrap()]);

    let res = repo.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = tag::Tag::new(1.try_into().unwrap(), "Tag 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = tag::Tag::new(2.try_into().unwrap(), "Tag 2".try_into().unwrap());
    assert_eq!(res[1], target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn get_many_case_filter_2(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let filter = TagsFilter::new(vec![1.try_into().unwrap(), 3.try_into().unwrap()]);

    let res = repo.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = tag::Tag::new(1.try_into().unwrap(), "Tag 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = tag::Tag::new(3.try_into().unwrap(), "Tag 3".try_into().unwrap());
    assert_eq!(res[1], target);
}

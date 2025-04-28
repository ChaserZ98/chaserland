use crate::domain::entity::tag;
use crate::domain::repository::tag::{TagRepository, TagRepositoryError};
use crate::infra::repository::postgres::tag::PgTagRepository;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn create_tag_case_1(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let id = 4.try_into().unwrap();
    let name = "tag 4".try_into().unwrap();
    let target = tag::Tag::new(id, name);

    let res = repo.create(target.name.clone()).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn create_tag_case_already_exists(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let tag = tag::Tag::new(4.try_into().unwrap(), "tag 1".try_into().unwrap());

    let res = repo.create(tag.name.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagRepositoryError::DuplicateTagSlug(name, slug) => tag.name == name && tag.slug == slug,
        _ => false,
    });
}

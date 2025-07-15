use chaserland_articles_service_core::{
    domain::tag::{
        entity::Tag,
        repository::{TagRepository, TagRepositoryError},
        vo as tag,
    },
    infra::postgres::repository::PgTagRepository,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn create_tag_case_1(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id = 4.try_into().unwrap();
    let name = tag::Name::new("tag 4");
    let target = Tag::new(id, name.clone());
    let new_tag = tag::NewTag::new(name);

    let res = repo.create(new_tag.clone(), &mut tx).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn create_tag_case_already_exists(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let name = tag::Name::new("tag 1");
    let new_tag = tag::NewTag::new(name);

    let res = repo.create(new_tag.clone(), &mut tx).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagRepositoryError::DuplicateTagSlug(value) => value == new_tag,
        _ => false,
    });
}

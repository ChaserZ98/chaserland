use chaserland_articles_service_core::{
    domain::tag::{
        entity::Tag,
        repository::{TagRepository, TagRepositoryError},
    },
    infra::postgres::repository::PgTagRepository,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_one_case_id(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let tag = Tag::new(1.try_into().unwrap(), "Tag 1".try_into().unwrap());

    let res = repo.get_one(tag.id.as_identifier(), &mut tx).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(tag, res);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_one_case_slug(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let tag = Tag::new(1.try_into().unwrap(), "Tag 1".try_into().unwrap());

    let res = repo.get_one(tag.slug().as_identifier(), &mut tx).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(tag, res);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_one_case_id_not_found(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let tag = Tag::new(4.try_into().unwrap(), "tag 4".try_into().unwrap());
    let identifier = tag.id.as_identifier();

    let res = repo.get_one(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagRepositoryError::TagNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn get_one_case_slug_not_found(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let tag = Tag::new(4.try_into().unwrap(), "tag 4".try_into().unwrap());
    let identifier = tag.slug().as_identifier();

    let res = repo.get_one(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagRepositoryError::TagNotFound(value) => value == identifier,
        _ => false,
    });
}

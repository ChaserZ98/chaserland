use chaserland_articles_service_core::{
    domain::tag::{
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
async fn delete_tag_case_id(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id = tag::Id::new(1);
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_ok());
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn delete_tag_case_slug(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let name = tag::Name::new("Tag 1");
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_ok());
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("tags"))
)]
async fn delete_tag_case_id_not_found(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id = tag::Id::new(4);
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone(), &mut tx).await;

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
async fn delete_tag_case_slug_not_found(pool: PgPool) {
    let repo = PgTagRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let name = tag::Name::new("Tag 4");
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagRepositoryError::TagNotFound(value) => value == identifier,
        _ => false,
    });
}

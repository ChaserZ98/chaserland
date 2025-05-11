use crate::domain::tag::{
    repository::{TagRepository, TagRepositoryError},
    vo as tag,
};
use crate::infra::repository::postgres::tag::PgTagRepository;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn delete_tag_case_id(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let id = tag::Id::new(1);
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_ok());
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn delete_tag_case_slug(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let name = tag::Name::new("Tag 1");
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_ok());
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn delete_tag_case_id_not_found(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let id = tag::Id::new(4);
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagRepositoryError::TagNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("tags")))]
async fn delete_tag_case_slug_not_found(pool: PgPool) {
    let repo = PgTagRepository::new(pool);

    let name = tag::Name::new("Tag 4");
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        TagRepositoryError::TagNotFound(value) => value == identifier,
        _ => false,
    });
}

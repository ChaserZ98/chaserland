use crate::domain::category::{
    repository::{CategoryRepository, CategoryRepositoryError},
    vo as category,
};
use crate::infra::repository::postgres::category::PgCategoryRepository;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn delete_category_case_id(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let id: category::Id = 1.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = repo.delete(identifier).await;

    assert!(res.is_ok());
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn delete_category_case_slug(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let name: category::Name = "Category 1".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier).await;

    assert!(res.is_ok());
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn delete_category_case_id_not_found(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let id: category::Id = 4.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        CategoryRepositoryError::CategoryNotFound(value) => value == identifier,
        _ => false,
    });
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn delete_category_case_slug_not_found(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let name: category::Name = "Category 4".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        CategoryRepositoryError::CategoryNotFound(value) => value == identifier,
        _ => false,
    });
}

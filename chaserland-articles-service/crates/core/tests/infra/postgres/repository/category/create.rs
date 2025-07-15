use chaserland_articles_service_core::{
    domain::category::{
        entity::Category,
        repository::{CategoryRepository, CategoryRepositoryError},
        vo as category,
    },
    infra::postgres::repository::PgCategoryRepository,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn create_category_case_1(pool: PgPool) {
    let repo = PgCategoryRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id = 4.try_into().unwrap();
    let name = category::Name::new("Category 4");
    let target = Category::new(id, name.clone());
    let new_category = category::NewCategory::new(name);

    let res = repo.create(new_category, &mut tx).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn create_category_case_already_exists(pool: PgPool) {
    let repo = PgCategoryRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let name = category::Name::new("Category 1");
    let new_category = category::NewCategory::new(name);

    let res = repo.create(new_category.clone(), &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        CategoryRepositoryError::DuplicateCategorySlug(value) => value == new_category,
        _ => false,
    });
}

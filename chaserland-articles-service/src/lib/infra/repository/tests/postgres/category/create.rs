use crate::domain::category::{
    entity::Category,
    repository::{CategoryRepository, CategoryRepositoryError},
    vo as category,
};
use crate::infra::repository::postgres::category::PgCategoryRepository;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn create_category_case_1(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let id = 4.try_into().unwrap();
    let name = category::Name::new("Category 4");
    let target = Category::new(id, name.clone());
    let new_category = category::NewCategory::new(name);

    let res = repo.create(new_category).await;

    println!("{:?}", res);

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn create_category_case_already_exists(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let name = category::Name::new("Category 1");
    let new_category = category::NewCategory::new(name);

    let res = repo.create(new_category.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        CategoryRepositoryError::DuplicateCategorySlug(value) => value == new_category,
        _ => false,
    });
}

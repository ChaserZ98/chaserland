use crate::domain::entity::category;
use crate::domain::repository::category::{CategoryRepository, CategoryRepositoryError};
use crate::infra::repository::postgres::category::PgCategoryRepository;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn create_category_case_1(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let id = 4.try_into().unwrap();
    let name = "Category 4".try_into().unwrap();
    let target = category::Category::new(id, name);

    let res = repo.create(target.name.clone()).await;

    println!("{:?}", res);

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("categories")))]
async fn create_category_case_already_exists(pool: PgPool) {
    let repo = PgCategoryRepository::new(pool);

    let id = 1.try_into().unwrap();
    let name = "Category 1".try_into().unwrap();
    let category = category::Category::new(id, name);

    let res = repo.create(category.name.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        CategoryRepositoryError::DuplicateCategorySlug(name, slug) =>
            name == category.name && slug == category.slug,
        _ => false,
    });
}

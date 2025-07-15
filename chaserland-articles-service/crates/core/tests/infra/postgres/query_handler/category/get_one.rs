use chaserland_articles_service_core::{
    app::query::{
        dto::CategoryDTO,
        query_handler::{error::CategoryQueryHandlerError, interface::CategoryQueryHandler},
    },
    domain::category::vo as category,
    infra::postgres::query_handler::PgCategoryQueryHandler,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn get_one_case_id(pool: PgPool) {
    let query_handler = PgCategoryQueryHandler::new(pool);

    let target = CategoryDTO {
        id: 1,
        name: "Category 1".into(),
        slug: "category-1".into(),
    };

    let res = query_handler
        .get_one(category::Id::new(target.id).as_identifier())
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn get_one_case_slug(pool: PgPool) {
    let query_handler = PgCategoryQueryHandler::new(pool);

    let target = CategoryDTO {
        id: 1,
        name: "Category 1".into(),
        slug: "category-1".into(),
    };

    let res = query_handler
        .get_one(
            category::Name::new(target.name.clone())
                .as_slug()
                .as_identifier(),
        )
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn get_one_case_id_not_found(pool: PgPool) {
    let query_handler = PgCategoryQueryHandler::new(pool);

    let id: category::Id = 4.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = query_handler.get_one(identifier.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        CategoryQueryHandlerError::CategoryNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("categories"))
)]
async fn get_one_case_slug_not_found(pool: PgPool) {
    let query_handler = PgCategoryQueryHandler::new(pool);

    let name: category::Name = "Category 4".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = query_handler.get_one(identifier.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        CategoryQueryHandlerError::CategoryNotFound(value) => value == identifier,
        _ => false,
    });
}

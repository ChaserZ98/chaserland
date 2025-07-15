use chaserland_articles_service_core::{
    app::{
        command::{self as command, interface::ArticleCommandService},
        query::{self as query, dto::CategoryDTO, interface::ArticleQueryService},
    },
    domain::category::vo as category,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

use crate::common::{init_command_service, init_query_service};

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(
        path = "../../../tests/fixtures",
        scripts(
            "series",
            "categories",
            "tags",
            "articles",
            "article_categories",
            "article_tags"
        )
    )
)]
async fn create_category_case_1(pool: PgPool) {
    let command_service = init_command_service(pool.clone());
    let query_service = init_query_service(pool);

    let name = "Category 4".try_into().unwrap();
    let command = command::CreateCategoryCommand { name };
    let res = command_service.create_category(command).await;

    assert!(res.is_ok());

    let query = query::GetCategoryOneQuery {
        identifier: category::Id::new(4).as_identifier(),
    };
    let res = query_service.get_category_one(query).await;
    let target = CategoryDTO {
        id: 4,
        name: "Category 4".into(),
        slug: "category-4".into(),
    };

    assert!(res.is_ok());

    let res = res.unwrap();
    assert_eq!(res, target);
}

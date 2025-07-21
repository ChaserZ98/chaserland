use crate::common::{init_command_service, init_query_service};
use chaserland_articles_service_core::{
    app::{
        command::{self, interface::ArticleCommandService},
        query::{self, error::GetCategoryOneError, interface::ArticleQueryService},
    },
    domain::category::vo as category,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

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
async fn delete_category_case_1(pool: PgPool) {
    let command_service = init_command_service(pool.clone());
    let query_service = init_query_service(pool.clone());

    let command = command::DeleteCategoryCommand {
        identifier: category::Id::new(1).as_identifier(),
    };

    let res = command_service.delete_category(command).await;

    assert!(res.is_ok(), "res: {:#?}", res);

    let query = query::GetCategoryOneQuery {
        identifier: category::Id::new(1).as_identifier(),
    };

    let res = query_service.get_category_one(query).await;

    assert!(res.is_err(), "res: {:#?}", res);

    let res = res.unwrap_err();

    assert!(matches!(res, GetCategoryOneError::NotFound(_)));
}

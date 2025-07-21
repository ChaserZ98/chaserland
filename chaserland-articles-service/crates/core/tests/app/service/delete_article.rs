use crate::common::{init_command_service, init_query_service};
use chaserland_articles_service_core::{
    app::{
        command::{self, interface::ArticleCommandService},
        query::{self, error::GetArticleOneError, interface::ArticleQueryService},
    },
    domain::article::vo as article,
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
async fn delete_article_case_1(pool: PgPool) {
    let command_service = init_command_service(pool.clone());
    let query_service = init_query_service(pool.clone());

    let command = command::DeleteArticleCommand {
        identifier: article::Id::new(1).as_identifier(),
    };

    let res = command_service.delete_article(command).await;

    assert!(res.is_ok(), "res: {:#?}", res);

    let query = query::GetArticleOneQuery {
        identifier: article::Id::new(1).as_identifier(),
        public_only: false,
        with_content: true,
    };

    let res = query_service.get_article_one(query).await;

    assert!(res.is_err(), "res: {:#?}", res);

    let res = res.unwrap_err();

    assert!(matches!(res, GetArticleOneError::NotFound(_)));
}

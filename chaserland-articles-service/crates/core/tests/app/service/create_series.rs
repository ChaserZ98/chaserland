use chaserland_articles_service_core::{
    app::{
        command::{self as command, interface::ArticleCommandService},
        query::{self as query, dto::SeriesDTO, interface::ArticleQueryService},
    },
    domain::series::vo as series,
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
async fn create_series_case_1(pool: PgPool) {
    let command_service = init_command_service(pool.clone());
    let query_service = init_query_service(pool);

    let name = "series 3".try_into().unwrap();
    let command = command::CreateSeriesCommand { name };
    let res = command_service.create_series(command).await;

    assert!(res.is_ok());

    let query = query::GetSeriesOneQuery {
        identifier: series::Id::new(3).as_identifier(),
    };
    let res = query_service.get_series_one(query).await;
    let target = SeriesDTO {
        id: 3,
        name: "series 3".into(),
        slug: "series-3".into(),
    };

    assert!(res.is_ok());

    let series = res.unwrap();
    assert_eq!(series, target);
}

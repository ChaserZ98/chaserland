use chaserland_articles_service_core::{
    app::query::{
        dto::SeriesDTO,
        query_handler::{error::SeriesQueryHandlerError, interface::SeriesQueryHandler},
    },
    domain::series::vo as series,
    infra::postgres::query_handler::PgSeriesQueryHandler,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn get_one_case_id(pool: PgPool) {
    let query_handler = PgSeriesQueryHandler::new(pool);

    let target = SeriesDTO {
        id: 1,
        name: "series 1".into(),
        slug: "series-1".into(),
    };

    let res = query_handler
        .get_one(series::Id::new(target.id).as_identifier())
        .await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn get_one_case_slug(pool: PgPool) {
    let query_handler = PgSeriesQueryHandler::new(pool);

    let target = SeriesDTO {
        id: 1,
        name: "series 1".into(),
        slug: "series-1".into(),
    };

    let res = query_handler
        .get_one(
            series::Name::new(target.name.clone())
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
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn get_one_case_id_not_found(pool: PgPool) {
    let query_handler = PgSeriesQueryHandler::new(pool);

    let id: series::Id = 4.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = query_handler.get_one(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        SeriesQueryHandlerError::SeriesNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn get_one_case_slug_not_found(pool: PgPool) {
    let query_handler = PgSeriesQueryHandler::new(pool);

    let name: series::Name = "series 4".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = query_handler.get_one(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        SeriesQueryHandlerError::SeriesNotFound(value) => value == identifier,
        _ => false,
    });
}

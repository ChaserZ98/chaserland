use chaserland_articles_service_core::{
    app::query::{dto::SeriesDTO, query_handler::interface::SeriesQueryHandler},
    domain::series::repository::SeriesFilter,
    infra::postgres::query_handler::PgSeriesQueryHandler,
    migrator::MIGRATOR,
};
use chaserland_common::pagination::Pagination;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn get_many_case_pagination_1(pool: sqlx::PgPool) {
    let query_handler = PgSeriesQueryHandler::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());

    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let series = res[0].clone();
    let target = SeriesDTO {
        id: 1,
        name: "series 1".into(),
        slug: "series-1".into(),
    };
    assert_eq!(series, target);

    let series = res[1].clone();
    let target = SeriesDTO {
        id: 2,
        name: "series 2".into(),
        slug: "series-2".into(),
    };
    assert_eq!(series, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn get_many_case_pagination_2(pool: sqlx::PgPool) {
    let query_handler = PgSeriesQueryHandler::new(pool);

    let mut pagination = Pagination::new(1.try_into().unwrap(), 1.try_into().unwrap());

    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let series = res[0].clone();
    let target = SeriesDTO {
        id: 1,
        name: "series 1".into(),
        slug: "series-1".into(),
    };
    assert_eq!(series, target);

    pagination.page = 2.try_into().unwrap();
    let res = query_handler.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let series = res[0].clone();
    let target = SeriesDTO {
        id: 2,
        name: "series 2".into(),
        slug: "series-2".into(),
    };
    assert_eq!(series, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn get_many_case_filter_1(pool: sqlx::PgPool) {
    let query_handler = PgSeriesQueryHandler::new(pool);

    let filter = SeriesFilter::new(vec![1.try_into().unwrap(), 2.try_into().unwrap()]);

    let res = query_handler.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = SeriesDTO {
        id: 1,
        name: "series 1".into(),
        slug: "series-1".into(),
    };
    assert_eq!(res[0], target);

    let target = SeriesDTO {
        id: 2,
        name: "series 2".into(),
        slug: "series-2".into(),
    };
    assert_eq!(res[1], target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn get_many_case_filter_2(pool: sqlx::PgPool) {
    let query_handler = PgSeriesQueryHandler::new(pool);

    let filter = SeriesFilter::new(vec![1.try_into().unwrap(), 4.try_into().unwrap()]);

    let res = query_handler.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = SeriesDTO {
        id: 1,
        name: "series 1".into(),
        slug: "series-1".into(),
    };
    assert_eq!(res[0], target);
}

use chaserland_articles_service_core::{
    domain::series::{
        entity::Series,
        repository::{SeriesRepository, SeriesRepositoryError},
        vo as series,
    },
    infra::postgres::repository::PgSeriesRepository,
    migrator::MIGRATOR,
};

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn create_series_case_1(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id = 3.try_into().unwrap();
    let name = series::Name::new("series 3");
    let target = Series::new(id, name.clone());
    let new_series = series::NewSeries::new(name);

    let res = repo.create(new_series, &mut tx).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn create_series_case_2(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id = 3.try_into().unwrap();
    let name = series::Name::new("series 3");
    let target = Series::new(id, name.clone());
    let new_series = series::NewSeries::new(name);

    let res = repo.create(new_series, &mut tx).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);

    let id = 4.try_into().unwrap();
    let name = series::Name::new("series 4");
    let target = Series::new(id, name.clone());
    let new_series = series::NewSeries::new(name);

    let res = repo.create(new_series, &mut tx).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn create_series_case_already_exists(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let name = series::Name::new("series 1");
    let new_series = series::NewSeries::new(name);

    let res = repo.create(new_series.clone(), &mut tx).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        SeriesRepositoryError::DuplicateSeriesSlug(value) => value == new_series,
        _ => false,
    })
}

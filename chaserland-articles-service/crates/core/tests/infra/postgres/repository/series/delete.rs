use chaserland_articles_service_core::{
    domain::series::{
        repository::{SeriesRepository, SeriesRepositoryError},
        vo as series,
    },
    infra::postgres::repository::PgSeriesRepository,
    migrator::MIGRATOR,
};
use sqlx::PgPool;

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn delete_series_case_id(pool: PgPool) {
    let repo = PgSeriesRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id: series::Id = 1.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_ok());
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn delete_series_case_slug(pool: PgPool) {
    let repo = PgSeriesRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let name: series::Name = "series 1".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_ok());
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn delete_series_case_id_not_found(pool: PgPool) {
    let repo = PgSeriesRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let id: series::Id = 3.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        SeriesRepositoryError::SeriesNotFound(value) => value == identifier,
        _ => false,
    });
}

#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "../../../../../tests/fixtures", scripts("series"))
)]
async fn delete_series_case_slug_not_found(pool: PgPool) {
    let repo = PgSeriesRepository::new();

    let mut tx = pool.begin().await.unwrap();

    let name: series::Name = "series 3".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone(), &mut tx).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        SeriesRepositoryError::SeriesNotFound(value) => value == identifier,
        _ => false,
    });
}

use crate::domain::entity::series;
use crate::domain::repository::series::{DeleteSeriesError, SeriesRepository};
use crate::infra::repository::postgres::series::PgSeriesRepository;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn delete_series_case_id(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let id: series::Id = 1.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_ok());
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn delete_series_case_slug(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let name: series::Name = "series 1".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_ok());
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn delete_series_case_id_not_found(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let id: series::Id = 3.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        DeleteSeriesError::NotFound(value) => value == identifier,
        _ => false,
    });
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn delete_series_case_slug_not_found(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let name: series::Name = "series 3".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = repo.delete(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        DeleteSeriesError::NotFound(value) => value == identifier,
        _ => false,
    });
}

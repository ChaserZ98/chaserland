use crate::domain::entity::series;
use crate::domain::repository::series::{CreateSeriesError, SeriesRepository};
use crate::infra::repository::postgres::series::PgSeriesRepository;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn create_series_case_1(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let id = 3.try_into().unwrap();
    let name = "series 3".try_into().unwrap();
    let target = series::Series::new(id, name);

    let res = repo.create(target.name.clone()).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn create_series_case_2(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let id = 3.try_into().unwrap();
    let name = "series 3".try_into().unwrap();
    let target = series::Series::new(id, name);

    let res = repo.create(target.name.clone()).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);

    let id = 4.try_into().unwrap();
    let name = "series 4".try_into().unwrap();
    let target = series::Series::new(id, name);

    let res = repo.create(target.name.clone()).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn create_series_case_already_exists(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let id = 1.try_into().unwrap();
    let name = "series 1".try_into().unwrap();
    let series = series::Series::new(id, name);

    let res = repo.create(series.name.clone()).await;

    assert!(res.is_err());

    let err = res.unwrap_err();

    assert!(match err {
        CreateSeriesError::AlreadyExists(name, slug) => name == series.name && slug == series.slug,
        _ => false,
    })
}

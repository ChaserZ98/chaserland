use crate::domain::series::{
    entity::Series,
    repository::{SeriesRepository, SeriesRepositoryError},
    vo as series,
};
use crate::infra::repository::postgres::series::PgSeriesRepository;
use sqlx::PgPool;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_one_case_id(pool: PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let id = 1.try_into().unwrap();
    let name = "series 1".try_into().unwrap();
    let target = Series::new(id, name);

    let res = repo.get_one(target.id.as_identifier()).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_one_case_slug(pool: PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let id = 1.try_into().unwrap();
    let name = "series 1".try_into().unwrap();
    let target = Series::new(id, name);

    let res = repo.get_one(target.slug().as_identifier()).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res, target);
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_one_case_id_not_found(pool: PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let id: series::Id = 4.try_into().unwrap();
    let identifier = id.as_identifier();

    let res = repo.get_one(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        SeriesRepositoryError::SeriesNotFound(value) => value == identifier,
        _ => false,
    });
}
#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_one_case_slug_not_found(pool: PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let name: series::Name = "series 4".try_into().unwrap();
    let identifier = name.as_slug().as_identifier();

    let res = repo.get_one(identifier.clone()).await;

    assert!(res.is_err());

    let res = res.unwrap_err();

    assert!(match res {
        SeriesRepositoryError::SeriesNotFound(value) => value == identifier,
        _ => false,
    });
}

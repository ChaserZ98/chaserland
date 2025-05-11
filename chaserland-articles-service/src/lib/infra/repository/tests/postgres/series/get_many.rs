use crate::domain::series::{
    entity::Series,
    repository::{SeriesFilter, SeriesRepository},
};
use crate::infra::repository::postgres::series::PgSeriesRepository;
use chaserland_common::pagination::Pagination;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_many_case_pagination_1(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let pagination = Pagination::new(1.try_into().unwrap(), 10.try_into().unwrap());

    let res = repo.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let series = res[0].clone();
    let target = Series::new(1.try_into().unwrap(), "series 1".try_into().unwrap());
    assert_eq!(series, target);

    let series = res[1].clone();
    let target = Series::new(2.try_into().unwrap(), "series 2".try_into().unwrap());
    assert_eq!(series, target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_many_case_pagination_2(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let mut pagination = Pagination::new(1.try_into().unwrap(), 1.try_into().unwrap());

    let res = repo.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let series = res[0].clone();
    let target = Series::new(1.try_into().unwrap(), "series 1".try_into().unwrap());
    assert_eq!(series, target);

    pagination.page = 2.try_into().unwrap();
    let res = repo.get_many(None, Some(pagination)).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let series = res[0].clone();
    let target = Series::new(2.try_into().unwrap(), "series 2".try_into().unwrap());
    assert_eq!(series, target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_many_case_filter_1(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let filter = SeriesFilter::new(vec![1.try_into().unwrap(), 2.try_into().unwrap()]);

    let res = repo.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let target = Series::new(1.try_into().unwrap(), "series 1".try_into().unwrap());
    assert_eq!(res[0], target);

    let target = Series::new(2.try_into().unwrap(), "series 2".try_into().unwrap());
    assert_eq!(res[1], target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_many_case_filter_2(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let filter = SeriesFilter::new(vec![1.try_into().unwrap(), 4.try_into().unwrap()]);

    let res = repo.get_many(Some(filter), None).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let target = Series::new(1.try_into().unwrap(), "series 1".try_into().unwrap());
    assert_eq!(res[0], target);
}

use crate::domain::entity::series;
use crate::domain::repository::series::SeriesRepository;
use crate::infra::repository::postgres::series::PgSeriesRepository;

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_many_case_1(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let page = 1.try_into().unwrap();
    let page_size = 10.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 2);

    let series = res[0].clone();
    let target = series::Series::new(1.try_into().unwrap(), "series 1".try_into().unwrap());
    assert_eq!(series, target);

    let series = res[1].clone();
    let target = series::Series::new(2.try_into().unwrap(), "series 2".try_into().unwrap());
    assert_eq!(series, target);
}

#[sqlx::test(fixtures(path = "../../../../../../../tests/fixtures", scripts("series")))]
async fn get_many_case_2(pool: sqlx::PgPool) {
    let repo = PgSeriesRepository::new(pool);

    let page = 1.try_into().unwrap();
    let page_size = 1.try_into().unwrap();

    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let series = res[0].clone();
    let target = series::Series::new(1.try_into().unwrap(), "series 1".try_into().unwrap());
    assert_eq!(series, target);

    let page = 2.try_into().unwrap();
    let res = repo.get_many(page, page_size).await;

    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 1);

    let series = res[0].clone();
    let target = series::Series::new(2.try_into().unwrap(), "series 2".try_into().unwrap());
    assert_eq!(series, target);
}

use chaserland_articles_service::model;
use chaserland_protos::article::v1::SeriesCreate;

#[sqlx::test(fixtures("series"))]
async fn test_get_series(db: sqlx::PgPool) {
    let series = model::Series::get(&db).await.unwrap();
    assert_eq!(series.len(), 2);

    let s = &series[0];
    assert_eq!(s.id, 1);
    assert_eq!(
        s.slug,
        "thrown-softly-avoid-friendly-distant-previous-wrong"
    );
    assert_eq!(
        s.name,
        "thrown softly avoid friendly distant previous wrong"
    );
}

#[sqlx::test(fixtures("series"))]
async fn test_create_series(db: sqlx::PgPool) {
    let series = SeriesCreate {
        name: "new series".to_string(),
    };
    let mut transaction = db.begin().await.unwrap();
    let series = model::Series::create(&mut transaction, series)
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    assert_eq!(series.id, 3);
    assert_eq!(series.slug, "new-series");
    assert_eq!(series.name, "new series");

    let series = model::Series::get(&db).await.unwrap();
    assert_eq!(series.len(), 3);

    let s = &series[2];
    assert_eq!(s.id, 3);
    assert_eq!(s.slug, "new-series");
    assert_eq!(s.name, "new series");
}

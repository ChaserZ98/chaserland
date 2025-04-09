use chaserland_articles_service::model;
use chaserland_protos::article::v1::TagCreate;

#[sqlx::test(fixtures("tags"))]
async fn test_get_tags(db: sqlx::PgPool) {
    let tags = model::Tag::get(&db).await.unwrap();
    assert_eq!(tags.len(), 2);

    let tag = &tags[0];
    assert_eq!(tag.id, 1);
    assert_eq!(tag.slug, "tag-1");
    assert_eq!(tag.name, "Tag 1");
}

#[sqlx::test(fixtures("tags"))]
async fn test_create_tag(db: sqlx::PgPool) {
    let tag = TagCreate {
        name: "Tag 3".to_string(),
    };
    let mut transaction = db.begin().await.unwrap();
    let tag = model::Tag::create(&mut transaction, tag).await.unwrap();
    transaction.commit().await.unwrap();
    assert_eq!(tag.id, 3);
    assert_eq!(tag.slug, "tag-3");
    assert_eq!(tag.name, "Tag 3");
}

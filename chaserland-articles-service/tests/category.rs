// use chaserland_articles_service::model;
// use chaserland_protos::article::v1::CategoryCreate;

// #[sqlx::test(fixtures("categories"))]
// async fn test_get_categories(db: sqlx::PgPool) {
//     let categories = model::Category::get(&db).await.unwrap();
//     assert_eq!(categories.len(), 2);

//     let category = &categories[0];
//     assert_eq!(category.id, 1);
//     assert_eq!(category.slug, "category-1");
//     assert_eq!(category.name, "Category 1");
// }

// #[sqlx::test(fixtures("categories"))]
// async fn test_create_category(db: sqlx::PgPool) {
//     let category = CategoryCreate {
//         name: "Category 3".to_string(),
//     };
//     let mut transaction = db.begin().await.unwrap();
//     let category = model::Category::create(&mut transaction, category)
//         .await
//         .unwrap();
//     transaction.commit().await.unwrap();
//     assert_eq!(category.id, 3);
//     assert_eq!(category.slug, "category-3");
//     assert_eq!(category.name, "Category 3");

//     let categories = model::Category::get(&db).await.unwrap();

//     assert_eq!(categories.len(), 3);

//     let category = &categories[2];
//     assert_eq!(category.id, 3);
//     assert_eq!(category.slug, "category-3");
//     assert_eq!(category.name, "Category 3");
// }

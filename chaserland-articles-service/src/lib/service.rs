// use crate::domain::model;
// use anyhow::Result;
// use chaserland_protos::article::v1::{
//     ArticleCreate, CategoryCreate, SeriesCreate, TagCreate,
//     article_service_server::ArticleServiceServer, get_article_content_request, get_article_request,
//     get_articles_request,
// };
// use sqlx::PgPool;

// pub struct ArticleService {
//     db: PgPool,
// }

// impl ArticleService {
//     pub fn new(db: PgPool) -> Self {
//         Self { db }
//     }
//     pub fn into_tonic_service(self) -> ArticleServiceServer<Self> {
//         ArticleServiceServer::new(self)
//     }
//     pub async fn create_article(&self, article: ArticleCreate) -> Result<model::FullArticle> {
//         let series_id = article.series_id.clone();
//         let category_ids = article.category_ids.clone();
//         let tag_ids = article.tag_ids.clone();

//         let mut transaction = self.db.begin().await?;

//         let article = model::Article::create(&mut transaction, article).await?;

//         let series = match series_id {
//             Some(series_id) => {
//                 Some(model::Article::update_series(&mut transaction, article.id, series_id).await?)
//             }
//             None => None,
//         };

//         let categories = match category_ids.is_empty() {
//             true => vec![],
//             false => {
//                 model::Article::add_categories(&mut transaction, article.id, category_ids.clone())
//                     .await?
//             }
//         };

//         let tags = match tag_ids.is_empty() {
//             true => vec![],
//             false => {
//                 model::Article::add_tags(&mut transaction, article.id, tag_ids.clone()).await?
//             }
//         };

//         transaction.commit().await?;

//         Ok(model::FullArticle {
//             id: article.id,
//             title: article.title,
//             slug: article.slug,
//             description: article.description,
//             content: article.content,
//             created_at: article.created_at,
//             published_at: article.published_at,
//             updated_at: article.updated_at,
//             deleted_at: article.deleted_at,
//             series,
//             categories,
//             tags,
//         })
//     }
//     pub async fn create_series(&self, series: SeriesCreate) -> Result<model::Series> {
//         let mut transaction = self.db.begin().await?;
//         let series = model::Series::create(&mut transaction, series).await?;
//         transaction.commit().await?;
//         Ok(series)
//     }
//     pub async fn create_category(&self, category: CategoryCreate) -> Result<model::Category> {
//         let mut transaction = self.db.begin().await?;
//         let category = model::Category::create(&mut transaction, category).await?;
//         transaction.commit().await?;
//         Ok(category)
//     }
//     pub async fn create_tag(&self, tag: TagCreate) -> Result<model::Tag> {
//         let mut transaction = self.db.begin().await?;
//         let tag = model::Tag::create(&mut transaction, tag).await?;
//         transaction.commit().await?;
//         Ok(tag)
//     }
//     pub async fn get_article(
//         &self,
//         identifier: get_article_request::Identifier,
//         public_only: bool,
//         with_content: bool,
//     ) -> Result<Option<model::FullArticle>> {
//         model::FullArticle::get_one(&self.db, identifier, public_only, with_content).await
//     }
//     pub async fn get_article_content(
//         &self,
//         identifer: get_article_content_request::Identifier,
//         public_only: bool,
//     ) -> Result<Option<String>> {
//         model::Article::get_content(&self.db, identifer, public_only).await
//     }
//     pub async fn get_articles(
//         &self,
//         page: i32,
//         page_size: i32,
//         public_only: bool,
//         with_content: bool,
//         filter: Option<get_articles_request::Filter>,
//     ) -> Result<Vec<model::FullArticle>> {
//         model::FullArticle::get_many(&self.db, page, page_size, public_only, with_content, filter)
//             .await
//     }
//     pub async fn get_series(&self) -> Result<Vec<model::Series>> {
//         model::Series::get(&self.db).await
//     }
//     pub async fn get_categories(&self) -> Result<Vec<model::Category>> {
//         model::Category::get(&self.db).await
//     }
//     pub async fn get_tags(&self) -> Result<Vec<model::Tag>> {
//         model::Tag::get(&self.db).await
//     }
//     pub async fn publish_article_by_id(&self, id: i32) -> Result<u64> {
//         let mut transaction = self.db.begin().await?;

//         let row_count = model::Article::publish_by_id(&mut transaction, id).await?;

//         transaction.commit().await?;

//         Ok(row_count)
//     }
//     pub async fn delete_article_by_id(&self, id: i32) -> Result<u64> {
//         let mut transaction = self.db.begin().await?;

//         let row_count = model::Article::delete_by_id(&mut transaction, id).await?;

//         transaction.commit().await?;

//         Ok(row_count)
//     }
//     pub async fn delete_series_by_id(&self, id: i32) -> Result<u64> {
//         let mut transaction = self.db.begin().await?;

//         let row_count = model::Series::delete_by_id(&mut transaction, id).await?;

//         transaction.commit().await?;

//         Ok(row_count)
//     }
//     pub async fn delete_category_by_id(&self, id: i32) -> Result<u64> {
//         let mut transaction = self.db.begin().await?;

//         let row_count = model::Category::delete_by_id(&mut transaction, id).await?;

//         transaction.commit().await?;

//         Ok(row_count)
//     }
//     pub async fn delete_tag_by_id(&self, id: i32) -> Result<u64> {
//         let mut transaction = self.db.begin().await?;

//         let row_count = model::Tag::delete_by_id(&mut transaction, id).await?;

//         transaction.commit().await?;

//         Ok(row_count)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use chaserland_protos::article::v1::{CategoryCreate, SeriesCreate, TagCreate};

//     use super::ArticleService;

//     #[sqlx::test(fixtures(path = "../tests/fixtures", scripts("series")))]
//     async fn create_series(db: sqlx::PgPool) {
//         let service = ArticleService { db };
//         let series = service
//             .create_series(SeriesCreate {
//                 name: "Series 3".to_string(),
//             })
//             .await
//             .unwrap();
//         assert_eq!(series.id, 3);
//         assert_eq!(series.slug, "series-3");
//         assert_eq!(series.name, "Series 3");
//     }

//     #[sqlx::test(fixtures(path = "../tests/fixtures", scripts("categories")))]
//     async fn create_category(db: sqlx::PgPool) {
//         let service = ArticleService { db };
//         let category = service
//             .create_category(CategoryCreate {
//                 name: "Category 3".to_string(),
//             })
//             .await
//             .unwrap();
//         assert_eq!(category.id, 3);
//         assert_eq!(category.slug, "category-3");
//         assert_eq!(category.name, "Category 3");
//     }

//     #[sqlx::test(fixtures(path = "../tests/fixtures", scripts("tags")))]
//     async fn create_tag(db: sqlx::PgPool) {
//         let service = ArticleService { db };
//         let tag = service
//             .create_tag(TagCreate {
//                 name: "Tag 3".to_string(),
//             })
//             .await
//             .unwrap();
//         assert_eq!(tag.id, 3);
//         assert_eq!(tag.slug, "tag-3");
//         assert_eq!(tag.name, "Tag 3");
//     }
// }

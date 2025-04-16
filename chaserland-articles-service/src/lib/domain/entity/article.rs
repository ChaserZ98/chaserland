use super::{category, series, tag};
use serde::{Deserialize, Serialize};
use slugify::slugify;
use std::{fmt::Display, ops::Sub};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleId(i32);

impl ArticleId {
    pub fn new(id: i32) -> Self {
        Self::validate(id).unwrap();
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
    pub fn validate(id: i32) -> Result<(), String> {
        match id > 0 {
            true => Ok(()),
            false => Err("id must be greater than 0".to_string()),
        }
    }
}

impl Display for ArticleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<i32> for ArticleId {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(ArticleId(value))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticleTitle(String);

impl ArticleTitle {
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        Self::validate(&title).unwrap();
        Self(title)
    }
    pub fn as_slug(&self) -> ArticleSlug {
        self.clone().into()
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
    fn validate(title: &str) -> Result<(), String> {
        match title.trim().is_empty() {
            true => Err("title is empty".to_string()),
            false => Ok(()),
        }
    }
}

impl Display for ArticleTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ArticleTitle {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(ArticleTitle(value))
    }
}

impl TryFrom<&str> for ArticleTitle {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticleSlug(String);

impl ArticleSlug {
    pub fn value(&self) -> String {
        self.0.clone()
    }
    fn validate(slug: &String) -> Result<(), String> {
        match slug.trim().is_empty() {
            true => Err("slug is empty".to_string()),
            false => Ok(()),
        }
    }
}

impl TryFrom<String> for ArticleSlug {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(ArticleSlug(value))
    }
}

impl TryFrom<&str> for ArticleSlug {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<ArticleTitle> for ArticleSlug {
    fn from(value: ArticleTitle) -> Self {
        Self(slugify!(&value.to_string(), separator = "-"))
    }
}

impl Display for ArticleSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticleDescription(String);

impl ArticleDescription {
    pub fn new(description: impl Into<String>) -> Self {
        Self(description.into())
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for ArticleDescription {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ArticleDescription {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl Display for ArticleDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticleContent(String);

impl ArticleContent {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for ArticleContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ArticleContent {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl Display for ArticleContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticleCreatedAt(chrono::DateTime<chrono::Utc>);

impl ArticleCreatedAt {
    pub fn new(created_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self(created_at)
    }
    pub fn value(&self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }
}

impl From<chrono::DateTime<chrono::Utc>> for ArticleCreatedAt {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl Display for ArticleCreatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticlePublishedAt(chrono::DateTime<chrono::Utc>);

impl ArticlePublishedAt {
    pub fn new(published_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self(published_at)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for ArticlePublishedAt {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl Display for ArticlePublishedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticleUpdatedAt(chrono::DateTime<chrono::Utc>);

impl ArticleUpdatedAt {
    pub fn new(updated_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self(updated_at)
    }
    pub fn value(&self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }
}

impl From<chrono::DateTime<chrono::Utc>> for ArticleUpdatedAt {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl Display for ArticleUpdatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArticleDeletedAt(chrono::DateTime<chrono::Utc>);

impl ArticleDeletedAt {
    pub fn new(deleted_at: chrono::DateTime<chrono::Utc>) -> Self {
        Self(deleted_at)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for ArticleDeletedAt {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl Display for ArticleDeletedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: ArticleId,
    pub title: ArticleTitle,
    pub slug: ArticleSlug,
    pub description: ArticleDescription,
    pub content: Option<ArticleContent>,
    pub created_at: ArticleCreatedAt,
    pub published_at: Option<ArticlePublishedAt>,
    pub updated_at: ArticleUpdatedAt,
    pub deleted_at: Option<ArticleDeletedAt>,
    pub series_id: Option<series::SeriesId>,
    pub category_ids: Vec<category::CategoryId>,
    pub tag_ids: Vec<tag::TagId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleCreate {
    pub title: ArticleTitle,
    pub description: ArticleDescription,
    pub content: Option<ArticleContent>,
    pub series_id: Option<series::SeriesId>,
    pub category_ids: Vec<category::CategoryId>,
    pub tag_ids: Vec<tag::TagId>,
}

// #[allow(dead_code)]
// impl Article {
//     pub async fn create(
//         transaction: &mut Transaction<'_, Postgres>,
//         article: ArticleCreate,
//     ) -> Result<Self> {
//         let series_id = article.series_id.clone();
//         let slug = slugify!(&article.title, separator = "-");

//         let article = sqlx::query_as(
//             "INSERT INTO article.articles (title, slug, description, content, series_id) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *",
//         )
//         .bind(article.title)
//         .bind(slug)
//         .bind(article.description)
//         .bind(article.content)
//         .bind(series_id)
//         .fetch_one(&mut **transaction)
//         .await?;

//         Ok(article)
//     }
//     pub async fn get_many(
//         db: &PgPool,
//         page: i32,
//         page_size: i32,
//         public_only: bool,
//         with_content: bool,
//         filter: Option<chaserland_protos::article::v1::get_articles_request::Filter>,
//     ) -> Result<Vec<Self>> {
//         let offset = (page - 1) * page_size;
//         let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM ");

//         match (public_only, with_content) {
//             (true, true) => {
//                 query.push("article.public_articles");
//             }
//             (true, false) => {
//                 query.push("article.public_articles_meta ");
//             }
//             (false, true) => {
//                 query.push("article.articles ");
//             }
//             (false, false) => {
//                 query.push("article.articles_meta ");
//             }
//         }
//         query.push("AS a ");

//         match filter {
//             Some(chaserland_protos::article::v1::get_articles_request::Filter::SeriesFilter(
//                 series_filter,
//             )) => {
//                 let series_slug = series_filter.series_slug;
//                 query.push("JOIN article.series AS b ON a.series_id = b.id WHERE b.slug = ANY(");
//                 query.push_bind(series_slug);
//                 query.push(")");
//             }
//             Some(
//                 chaserland_protos::article::v1::get_articles_request::Filter::CategoryTagFilter(
//                     category_tag_filter,
//                 ),
//             ) => {
//                 let category_slugs = category_tag_filter.category_slugs;
//                 let tag_slugs = category_tag_filter.tag_slugs;

//                 match (!category_slugs.is_empty(), !tag_slugs.is_empty()) {
//                     (true, true) => {
//                         query.push("JOIN article.categories AS c ON a.id = c.article_id JOIN article.tags AS d ON d.id = c.tag_id WHERE c.category_slug = ANY(");
//                         query.push_bind(category_slugs);
//                         query.push(") AND d.tag_slug = ANY(");
//                         query.push_bind(tag_slugs);
//                         query.push(")");
//                     }
//                     (true, false) => {
//                         query.push("JOIN article.categories AS c ON a.id = c.article_id WHERE c.category_slug = ANY(");
//                         query.push_bind(category_slugs);
//                         query.push(")");
//                     }
//                     (false, true) => {
//                         query.push(
//                             "JOIN article.tags AS d ON a.id = d.article_id WHERE d.tag_slug = ANY(",
//                         );
//                         query.push_bind(tag_slugs);
//                         query.push(")");
//                     }
//                     (false, false) => {}
//                 }
//             }
//             None => {}
//         }

//         query.push(" LIMIT ");
//         query.push_bind(page_size);
//         query.push(" OFFSET ");
//         query.push_bind(offset);
//         let res = query.build_query_as().fetch_all(db).await?;
//         Ok(res)
//     }
//     pub async fn get_one(
//         db: &PgPool,
//         identifier: chaserland_protos::article::v1::get_article_request::Identifier,
//         public_only: bool,
//         with_content: bool,
//     ) -> Result<Option<Self>> {
//         let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM ");
//         match (public_only, with_content) {
//             (true, true) => {
//                 query.push("article.public_articles ");
//             }
//             (true, false) => {
//                 query.push("article.public_articles_meta ");
//             }
//             (false, true) => {
//                 query.push("article.articles ");
//             }
//             (false, false) => {
//                 query.push("article.articles_meta ");
//             }
//         }
//         match identifier {
//             chaserland_protos::article::v1::get_article_request::Identifier::Id(id) => {
//                 query.push("WHERE id = ");
//                 query.push_bind(id);
//             }
//             chaserland_protos::article::v1::get_article_request::Identifier::Slug(slug) => {
//                 query.push("WHERE slug = ");
//                 query.push_bind(slug);
//             }
//         }

//         let article = query.build_query_as().fetch_optional(db).await?;
//         Ok(article)
//     }
//     pub async fn get_content(
//         db: &PgPool,
//         identifier: chaserland_protos::article::v1::get_article_content_request::Identifier,
//         public_only: bool,
//     ) -> Result<Option<String>> {
//         let mut query = QueryBuilder::<Postgres>::new("SELECT content FROM ");
//         match public_only {
//             true => {
//                 query.push("article.public_articles ");
//             }
//             false => {
//                 query.push("article.articles ");
//             }
//         }
//         query.push("WHERE ");
//         match identifier {
//             chaserland_protos::article::v1::get_article_content_request::Identifier::Slug(slug) => {
//                 query.push("slug = ");
//                 query.push_bind(slug);
//             }
//             chaserland_protos::article::v1::get_article_content_request::Identifier::Id(id) => {
//                 query.push("id = ");
//                 query.push_bind(id);
//             }
//         };
//         let content = query.build_query_scalar().fetch_optional(db).await?;
//         Ok(content)
//     }
//     pub async fn add_tags(
//         transaction: &mut Transaction<'_, Postgres>,
//         article_id: i32,
//         tag_ids: Vec<i32>,
//     ) -> Result<Vec<Tag>> {
//         let mut query = QueryBuilder::new("INSERT INTO article.article_tags (article_id, tag_id) ");
//         query.push_values(tag_ids.clone(), |mut b, tag_id| {
//             b.push_bind(article_id).push_bind(tag_id);
//         });
//         query.push(" ON CONFLICT (article_id, tag_id) DO NOTHING");

//         query.build().execute(&mut **transaction).await?;

//         let tags = sqlx::query_as("SELECT * FROM article.tags WHERE id = ANY($1)")
//             .bind(tag_ids)
//             .fetch_all(&mut **transaction)
//             .await?;

//         Ok(tags)
//     }
//     pub async fn add_categories(
//         transaction: &mut Transaction<'_, Postgres>,
//         article_id: i32,
//         category_ids: Vec<i32>,
//     ) -> Result<Vec<Category>> {
//         let mut query =
//             QueryBuilder::new("INSERT INTO article.article_categories (article_id, category_id) ");
//         query.push_values(category_ids.clone(), |mut b, category_id| {
//             b.push_bind(article_id).push_bind(category_id);
//         });
//         query.push(" ON CONFLICT (article_id, category_id) DO NOTHING");

//         query.build().execute(&mut **transaction).await?;

//         let categories = sqlx::query_as("SELECT * FROM article.categories WHERE id = ANY($1)")
//             .bind(category_ids)
//             .fetch_all(&mut **transaction)
//             .await?;

//         Ok(categories)
//     }
//     pub async fn update_series(
//         transaction: &mut Transaction<'_, Postgres>,
//         article_id: i32,
//         series_id: i32,
//     ) -> Result<Series> {
//         sqlx::query("UPDATE article.articles SET series_id = $1 WHERE id = $2")
//             .bind(series_id)
//             .bind(article_id)
//             .execute(&mut **transaction)
//             .await?;
//         let series = sqlx::query_as("SELECT * FROM article.series WHERE id = $1")
//             .bind(series_id)
//             .fetch_one(&mut **transaction)
//             .await?;
//         Ok(series)
//     }
//     pub async fn publish_by_id(
//         transaction: &mut Transaction<'_, Postgres>,
//         id: i32,
//     ) -> Result<u64> {
//         let row_count = sqlx::query(
//             "UPDATE article.articles SET published_at = CURRENT_TIMESTAMP WHERE id = $1",
//         )
//         .bind(id)
//         .execute(&mut **transaction)
//         .await?
//         .rows_affected();
//         Ok(row_count)
//     }
//     pub async fn delete_by_id(transaction: &mut Transaction<'_, Postgres>, id: i32) -> Result<u64> {
//         let row_count =
//             sqlx::query("UPDATE article.articles SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1")
//                 .bind(id)
//                 .execute(&mut **transaction)
//                 .await?
//                 .rows_affected();
//         Ok(row_count)
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct FullArticle {
//     pub id: i32,
//     pub title: String,
//     pub slug: String,
//     pub description: String,
//     pub content: Option<String>,
//     pub created_at: chrono::DateTime<chrono::Utc>,
//     pub published_at: Option<chrono::DateTime<chrono::Utc>>,
//     pub updated_at: chrono::DateTime<chrono::Utc>,
//     pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
//     pub series: Option<Series>,
//     pub categories: Vec<Category>,
//     pub tags: Vec<Tag>,
// }

// impl FullArticle {
//     pub async fn get_one(
//         db: &PgPool,
//         identifier: chaserland_protos::article::v1::get_article_request::Identifier,
//         public_only: bool,
//         with_content: bool,
//     ) -> Result<Option<Self>> {
//         let article = match Article::get_one(db, identifier, public_only, with_content).await? {
//             Some(val) => val,
//             None => {
//                 return Ok(None);
//             }
//         };

//         let series = Series::get_by_article_id(db, article.id).await?;
//         let categories = Category::get_all_by_article_id(db, article.id).await?;
//         let tags = Tag::get_many_by_article_id(db, article.id).await?;

//         let res = Self {
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
//         };

//         Ok(Some(res))
//     }
//     pub async fn get_many(
//         db: &PgPool,
//         page: i32,
//         page_size: i32,
//         public_only: bool,
//         with_content: bool,
//         filter: Option<chaserland_protos::article::v1::get_articles_request::Filter>,
//     ) -> Result<Vec<Self>> {
//         let articles =
//             Article::get_many(db, page, page_size, public_only, with_content, filter).await?;

//         let mut res = Vec::new();
//         for article in articles {
//             let series = Series::get_by_article_id(db, article.id).await?;
//             let categories = Category::get_all_by_article_id(db, article.id).await?;
//             let tags = Tag::get_many_by_article_id(db, article.id).await?;
//             res.push(FullArticle {
//                 id: article.id,
//                 title: article.title,
//                 slug: article.slug,
//                 description: article.description,
//                 content: article.content,
//                 created_at: article.created_at,
//                 published_at: article.published_at,
//                 updated_at: article.updated_at,
//                 deleted_at: article.deleted_at,
//                 series,
//                 categories,
//                 tags,
//             });
//         }

//         Ok(res)
//     }
// }

// impl Into<chaserland_protos::article::v1::Article> for FullArticle {
//     fn into(self) -> chaserland_protos::article::v1::Article {
//         chaserland_protos::article::v1::Article {
//             id: self.id,
//             title: self.title,
//             slug: self.slug,
//             description: self.description,
//             content: self.content,
//             created_at: Some(prost_types::Timestamp {
//                 seconds: self.created_at.timestamp(),
//                 nanos: self.created_at.timestamp_subsec_nanos() as i32,
//             }),
//             published_at: self.published_at.map(|dt| prost_types::Timestamp {
//                 seconds: dt.timestamp(),
//                 nanos: dt.timestamp_subsec_nanos() as i32,
//             }),
//             updated_at: Some(prost_types::Timestamp {
//                 seconds: self.updated_at.timestamp(),
//                 nanos: self.updated_at.timestamp_subsec_nanos() as i32,
//             }),
//             deleted_at: self.deleted_at.map(|dt| prost_types::Timestamp {
//                 seconds: dt.timestamp(),
//                 nanos: dt.timestamp_subsec_nanos() as i32,
//             }),
//             series: self.series.map(|x| x.into()),
//             categories: self.categories.into_iter().map(|x| x.into()).collect(),
//             tags: self.tags.into_iter().map(|x| x.into()).collect(),
//         }
//     }
// }

#[derive(Debug)]
pub enum Identifier {
    Id(ArticleId),
    Slug(ArticleSlug),
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Id(id) => write!(f, "id={}", id),
            Identifier::Slug(slug) => write!(f, "slug={}", slug),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArticleTitle;

    #[test]
    fn test_article_title() {
        let title = ArticleTitle::new(String::from("title"));
        assert_eq!(title.value(), "title");
    }

    #[test]
    #[should_panic(expected = "title is empty")]
    fn test_article_title_panic() {
        let _ = ArticleTitle::new(String::from("  "));
    }

    #[test]
    fn test_article_title_convert() {
        let title = TryInto::<ArticleTitle>::try_into(String::from("title"));
        assert_eq!(title.is_ok(), true);
        assert_eq!(title.unwrap().value(), "title");

        let title: Result<ArticleTitle, _> = String::from("  ").try_into();
        assert_eq!(title.is_err(), true);
        assert_eq!(title.unwrap_err(), "title is empty");
    }
}

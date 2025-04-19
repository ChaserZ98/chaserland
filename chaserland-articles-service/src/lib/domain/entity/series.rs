use serde::{Deserialize, Serialize};
use slugify::slugify;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct SeriesId(i32);

impl SeriesId {
    pub fn new(id: i32) -> Self {
        Self::validate(id).unwrap();
        SeriesId(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    fn validate(id: i32) -> Result<(), String> {
        match id > 0 {
            true => Ok(()),
            false => Err("id must be greater than 0".to_string()),
        }
    }
}

impl TryFrom<i32> for SeriesId {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(SeriesId(value))
    }
}

impl Display for SeriesId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SeriesSlug(String);

impl SeriesSlug {
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl Display for SeriesSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesName(String);

impl SeriesName {
    pub fn as_slug(&self) -> SeriesSlug {
        SeriesSlug(slugify!(&self.0, separator = "-"))
    }
}

impl Display for SeriesName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub id: SeriesId,
    pub slug: SeriesSlug,
    pub name: SeriesName,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Identifier {
    Id(SeriesId),
    Slug(SeriesSlug),
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Id(id) => write!(f, "id={}", id),
            Identifier::Slug(slug) => write!(f, "slug={}", slug),
        }
    }
}

// #[allow(dead_code)]
// impl Series {
//     pub async fn create(
//         transaction: &mut Transaction<'_, Postgres>,
//         series: SeriesCreate,
//     ) -> Result<Self> {
//         let name = series.name;
//         let slug = slugify!(&name, separator = "-");
//         let series = sqlx::query_as("INSERT INTO article.series (slug, name) VALUES ($1, $2) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *")
//             .bind(slug)
//             .bind(name)
//             .fetch_one(&mut **transaction)
//             .await?;
//         Ok(series)
//     }
//     pub async fn get(db: &PgPool) -> Result<Vec<Self>> {
//         let res = sqlx::query_as("SELECT * FROM article.series")
//             .fetch_all(db)
//             .await?;
//         Ok(res)
//     }
//     pub async fn get_by_id(db: &PgPool, id: i32) -> Result<Option<Self>> {
//         let res = sqlx::query_as("SELECT * FROM article.series WHERE id = $1")
//             .bind(id)
//             .fetch_optional(db)
//             .await?;
//         Ok(res)
//     }
//     pub async fn get_by_article_id(db: &PgPool, id: i32) -> Result<Option<Self>> {
//         let res = sqlx::query_as("SELECT a.id, a.slug, a.name FROM article.series AS a JOIN article.articles AS b ON a.id = b.series_id WHERE b.id = $1").bind(id).fetch_optional(db).await?;
//         Ok(res)
//     }
//     pub async fn delete_by_id(transaction: &mut Transaction<'_, Postgres>, id: i32) -> Result<u64> {
//         let row_count = sqlx::query("DELETE FROM article.series WHERE id = $1")
//             .bind(id)
//             .execute(&mut **transaction)
//             .await?
//             .rows_affected();
//         println!("row_count: {}", row_count);
//         Ok(row_count)
//     }
// }

// impl Into<chaserland_protos::article::v1::Series> for Series {
//     fn into(self) -> chaserland_protos::article::v1::Series {
//         chaserland_protos::article::v1::Series {
//             id: self.id,
//             slug: self.slug,
//             name: self.name,
//         }
//     }
// }

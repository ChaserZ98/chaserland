use std::fmt::Display;

use serde::{Deserialize, Serialize};
use slugify::slugify;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagId(i32);

impl Display for TagId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagSlug(String);

impl Display for TagSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagName(String);

impl TagName {
    pub fn as_slug(&self) -> TagSlug {
        TagSlug(slugify!(&self.0, separator = "-"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub slug: TagSlug,
    pub name: TagName,
}

// #[allow(dead_code)]
// impl Tag {
//     pub async fn create(
//         transaction: &mut Transaction<'_, Postgres>,
//         tag: TagCreate,
//     ) -> Result<Self> {
//         let name = tag.name;
//         let slug = slugify!(&name, separator = "-");
//         let tag = sqlx::query_as("INSERT INTO article.tags (slug, name) VALUES ($1, $2) ON CONFLICT (slug) DO UPDATE SET slug = EXCLUDED.slug RETURNING *").bind(slug).bind(name).fetch_one(&mut **transaction).await?;
//         Ok(tag)
//     }
//     pub async fn get(db: &PgPool) -> Result<Vec<Self>> {
//         let res = sqlx::query_as("SELECT * FROM article.tags")
//             .fetch_all(db)
//             .await?;
//         Ok(res)
//     }
//     pub async fn get_many_by_article_id(db: &PgPool, id: i32) -> Result<Vec<Tag>> {
//         let res = sqlx::query_as("SELECT a.id, a.slug, a.name FROM article.tags AS a JOIN article.article_tags AS b ON a.id = b.tag_id WHERE b.article_id = $1").bind(id).fetch_all(db).await?;
//         Ok(res)
//     }
//     pub async fn delete_by_id(transaction: &mut Transaction<'_, Postgres>, id: i32) -> Result<u64> {
//         let row_count = sqlx::query("DELETE FROM article.tags WHERE id = $1")
//             .bind(id)
//             .execute(&mut **transaction)
//             .await?
//             .rows_affected();
//         Ok(row_count)
//     }
// }

// impl Into<chaserland_protos::article::v1::Tag> for Tag {
//     fn into(self) -> chaserland_protos::article::v1::Tag {
//         chaserland_protos::article::v1::Tag {
//             id: self.id,
//             slug: self.slug,
//             name: self.name,
//         }
//     }
// }

#[derive(Debug)]
pub enum Identifier {
    Id(TagId),
    Slug(TagSlug),
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Id(id) => write!(f, "id={}", id),
            Identifier::Slug(slug) => write!(f, "slug={}", slug),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetTagError {
    #[error("Tag with identifier not found: {identifier}")]
    NotFound { identifier: Identifier },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

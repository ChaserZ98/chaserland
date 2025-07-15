use crate::domain::article::{entity::Article, vo::TimestampVersion};

#[derive(sqlx::FromRow)]
pub struct PgArticle {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    #[sqlx(default)]
    pub content: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub series_id: Option<i32>,
    #[sqlx(default)]
    pub category_ids: Vec<i32>,
    #[sqlx(default)]
    pub tag_ids: Vec<i32>,
    pub version: chrono::DateTime<chrono::Utc>,
}

impl TryInto<Article> for PgArticle {
    type Error = String;

    fn try_into(self) -> Result<Article, Self::Error> {
        let id = self.id.try_into()?;
        let title = self.title.try_into()?;
        let description = self.description.into();
        let content = match self.content {
            Some(content) => Some(content.into()),
            None => None,
        };
        let created_at = self.created_at.into();
        let published_at = self.published_at.map(|published_at| published_at.into());
        let updated_at = self.updated_at.into();
        let deleted_at = self.deleted_at.map(|deleted_at| deleted_at.into());
        let series_id = match self.series_id {
            Some(series_id) => Some(series_id.try_into()?),
            None => None,
        };

        let category_ids = self
            .category_ids
            .iter()
            .map(|category_id| (*category_id).try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let tag_ids = self
            .tag_ids
            .iter()
            .map(|tag_id| (*tag_id).try_into())
            .collect::<Result<Vec<_>, _>>()?;

        let mut article = Article::new(
            id,
            title,
            description,
            content,
            created_at,
            updated_at,
            series_id,
            category_ids,
            tag_ids,
        );
        article.published_at = published_at;
        article.deleted_at = deleted_at;

        Ok(article)
    }
}

impl TryInto<(Article, TimestampVersion)> for PgArticle {
    type Error = String;
    fn try_into(self) -> Result<(Article, TimestampVersion), Self::Error> {
        let version = self.version.into();
        let article = self.try_into()?;

        Ok((article, version))
    }
}

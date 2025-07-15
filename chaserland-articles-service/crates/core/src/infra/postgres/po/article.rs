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
            .map(|&category_id| category_id.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let tag_ids = self
            .tag_ids
            .iter()
            .map(|&tag_id| tag_id.try_into())
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

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::PgArticle;
    use crate::domain::article::{entity::Article, vo::TimestampVersion};

    #[test]
    fn pg_article_to_article() {
        let pg_article = PgArticle {
            id: 1,
            title: "title".to_string(),
            slug: "slug".to_string(),
            description: "description".to_string(),
            content: None,
            created_at: Utc::now(),
            published_at: None,
            updated_at: Utc::now(),
            deleted_at: None,
            series_id: None,
            category_ids: vec![],
            tag_ids: vec![],
            version: Utc::now(),
        };

        let target = Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            None,
            Utc::now().into(),
            Utc::now().into(),
            None,
            vec![],
            vec![],
        );

        let res: Result<Article, _> = pg_article.try_into();

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(res.id, target.id);
        assert_eq!(res.title, target.title);
        assert_eq!(res.description, target.description);
        assert_eq!(res.content, target.content);
        assert!(target.created_at.value() - res.created_at.value() < Duration::seconds(1));
        assert_eq!(res.published_at, target.published_at);
        assert!(target.updated_at.value() - res.updated_at.value() < Duration::seconds(1));
        assert_eq!(res.deleted_at, target.deleted_at);
        assert_eq!(res.series_id, target.series_id);
        assert_eq!(res.category_ids, target.category_ids);
        assert_eq!(res.tag_ids, target.tag_ids);
    }

    #[test]
    fn pg_article_to_article_timestamp_version() {
        let pg_article = PgArticle {
            id: 1,
            title: "title".to_string(),
            slug: "slug".to_string(),
            description: "description".to_string(),
            content: Some("content".into()),
            created_at: Utc::now(),
            published_at: None,
            updated_at: Utc::now(),
            deleted_at: None,
            series_id: Some(1),
            category_ids: vec![],
            tag_ids: vec![],
            version: Utc::now(),
        };

        let target = Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            Utc::now().into(),
            Utc::now().into(),
            Some(1.try_into().unwrap()),
            vec![],
            vec![],
        );
        let target_version = TimestampVersion::new(Utc::now());

        let res: Result<(Article, TimestampVersion), _> = pg_article.try_into();

        assert!(res.is_ok());

        let (article, version) = res.unwrap();

        assert_eq!(article.id, target.id);
        assert_eq!(article.title, target.title);
        assert_eq!(article.description, target.description);
        assert_eq!(article.content, target.content);
        assert!(target.created_at.value() - article.created_at.value() < Duration::seconds(1));
        assert_eq!(article.published_at, target.published_at);
        assert!(target.updated_at.value() - article.updated_at.value() < Duration::seconds(1));
        assert_eq!(article.deleted_at, target.deleted_at);
        assert_eq!(article.series_id, target.series_id);
        assert_eq!(article.category_ids, target.category_ids);
        assert_eq!(article.tag_ids, target.tag_ids);
        assert!(target_version.value() - version.value() < Duration::seconds(1));
    }

    #[test]
    fn pg_article_to_article_case_category_id_error() {
        let pg_article = PgArticle {
            id: 1,
            title: "title".to_string(),
            slug: "slug".to_string(),
            description: "description".to_string(),
            content: None,
            created_at: Utc::now(),
            published_at: None,
            updated_at: Utc::now(),
            deleted_at: None,
            series_id: None,
            category_ids: vec![-1, -2],
            tag_ids: vec![],
            version: Utc::now(),
        };

        let res: Result<Article, _> = pg_article.try_into();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert_eq!(res, "id must be greater than 0");
    }

    #[test]
    fn pg_article_to_article_case_tag_id_error() {
        let pg_article = PgArticle {
            id: 1,
            title: "title".to_string(),
            slug: "slug".to_string(),
            description: "description".to_string(),
            content: None,
            created_at: Utc::now(),
            published_at: None,
            updated_at: Utc::now(),
            deleted_at: None,
            series_id: None,
            category_ids: vec![],
            tag_ids: vec![-1, -2],
            version: Utc::now(),
        };

        let res: Result<Article, _> = pg_article.try_into();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert_eq!(res, "id must be greater than 0");
    }
}

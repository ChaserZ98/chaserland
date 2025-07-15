use super::error::ArticleDomainError;
use super::vo::{
    Content, CreatedAt, DeletedAt, Description, Id, PublishedAt, Slug, Title, UpdatedAt,
};
use crate::domain::category::vo as category;
use crate::domain::series::vo as series;
use crate::domain::tag::vo as tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Article {
    pub id: Id,
    pub title: Title,
    slug: Slug,
    pub description: Description,
    pub content: Option<Content>,
    pub created_at: CreatedAt,
    pub published_at: Option<PublishedAt>,
    pub updated_at: UpdatedAt,
    pub deleted_at: Option<DeletedAt>,
    pub series_id: Option<series::Id>,
    pub category_ids: Vec<category::Id>,
    pub tag_ids: Vec<tag::Id>,
}

impl Article {
    pub fn new(
        id: Id,
        title: Title,
        description: Description,
        content: Option<Content>,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
        series_id: Option<series::Id>,
        category_ids: Vec<category::Id>,
        tag_ids: Vec<tag::Id>,
    ) -> Self {
        let slug = title.as_slug();
        Self {
            id,
            title,
            description,
            content,
            created_at,
            updated_at,
            deleted_at: None,
            published_at: None,
            slug,
            series_id,
            category_ids,
            tag_ids,
        }
    }

    pub fn set_title(&mut self, title: Title) {
        self.slug = title.as_slug();
        self.title = title;
        self.bump_updated_at();
    }

    pub fn set_description(&mut self, description: Description) {
        self.description = description;
        self.bump_updated_at();
    }

    pub fn set_content(&mut self, content: Content) {
        self.content = Some(content);
        self.bump_updated_at();
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    pub fn is_published(&self) -> bool {
        self.published_at.is_some()
    }

    pub fn publish(&mut self) -> Result<(), ArticleDomainError> {
        match self.published_at {
            Some(_) => Err(ArticleDomainError::AlreadyPublished(self.id)),
            None => {
                let utc_now = chrono::Utc::now();
                self.published_at = Some(PublishedAt::new(utc_now));
                self.bump_updated_at();
                Ok(())
            }
        }
    }

    pub fn unpublish(&mut self) -> Result<(), ArticleDomainError> {
        match self.published_at {
            Some(_) => {
                self.published_at = None;
                Ok(())
            }
            None => Err(ArticleDomainError::NotPublished(self.id)),
        }
    }

    pub fn soft_delete(&mut self) -> Result<(), ArticleDomainError> {
        match self.deleted_at {
            Some(_) => Err(ArticleDomainError::AlreadySoftDeleted(self.id)),
            None => {
                self.deleted_at = Some(DeletedAt::new());
                Ok(())
            }
        }
    }

    pub fn revoke_soft_delete(&mut self) -> Result<(), ArticleDomainError> {
        match self.deleted_at {
            Some(_) => {
                self.deleted_at = None;
                Ok(())
            }
            None => Err(ArticleDomainError::NotSoftDeleted(self.id)),
        }
    }

    fn bump_updated_at(&mut self) {
        self.updated_at.update();
    }

    pub fn set_series_id(&mut self, series_id: series::Id) {
        self.series_id = Some(series_id);
    }

    pub fn remove_series_id(&mut self) {
        self.series_id = None;
    }

    pub fn add_category_id(&mut self, category_id: category::Id) -> Result<(), ArticleDomainError> {
        if self.category_ids.contains(&category_id) {
            return Err(ArticleDomainError::CategoryAlreadyAttached(
                self.id,
                category_id,
            ));
        }
        self.category_ids.push(category_id);
        Ok(())
    }

    pub fn remove_category_id(
        &mut self,
        category_id: category::Id,
    ) -> Result<(), ArticleDomainError> {
        if !self.category_ids.contains(&category_id) {
            return Err(ArticleDomainError::CategoryNotFound(self.id, category_id));
        }
        self.category_ids.retain(|id| *id != category_id);
        Ok(())
    }

    pub fn add_tag_id(&mut self, tag_id: tag::Id) -> Result<(), ArticleDomainError> {
        if self.tag_ids.contains(&tag_id) {
            return Err(ArticleDomainError::TagAlreadyAttached(self.id, tag_id));
        }
        self.tag_ids.push(tag_id);
        Ok(())
    }

    pub fn remove_tag_id(&mut self, tag_id: tag::Id) -> Result<(), ArticleDomainError> {
        if !self.tag_ids.contains(&tag_id) {
            return Err(ArticleDomainError::TagNotFound(self.id, tag_id));
        }
        self.tag_ids.retain(|id| *id != tag_id);
        Ok(())
    }
}

impl Default for Article {
    fn default() -> Self {
        let id = Id::new(1);
        let title = Title::new("Default Title");
        let description = "".into();
        let content = None;
        let now = chrono::Utc::now();
        let created_at = now.into();
        let updated_at = now.into();
        let series_id = None;
        let category_ids = vec![];
        let tag_ids = vec![];
        Self::new(
            id,
            title,
            description,
            content,
            created_at,
            updated_at,
            series_id,
            category_ids,
            tag_ids,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Article;
    use crate::domain::article::error::ArticleDomainError;
    use crate::domain::article::vo::{Content, Description, PublishedAt, Title};

    #[test]
    fn article_case_new() {
        let now = chrono::Utc::now();
        let article = Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            now.into(),
            now.into(),
            None,
            vec![1.try_into().unwrap(), 2.try_into().unwrap()],
            vec![3.try_into().unwrap(), 4.try_into().unwrap()],
        );

        let target = Article {
            id: 1.try_into().unwrap(),
            title: "title".try_into().unwrap(),
            slug: "title".try_into().unwrap(),
            description: "description".into(),
            content: Some("content".into()),
            created_at: now.into(),
            updated_at: now.into(),
            published_at: None,
            deleted_at: None,
            series_id: None,
            category_ids: vec![1.try_into().unwrap(), 2.try_into().unwrap()],
            tag_ids: vec![3.try_into().unwrap(), 4.try_into().unwrap()],
        };

        assert_eq!(article.id, target.id);
        assert_eq!(article.title, target.title);
        assert_eq!(article.slug(), target.slug());
        assert_eq!(article.description, target.description);
        assert_eq!(article.content, target.content);
        assert_eq!(article.created_at, target.created_at);
        assert_eq!(article.updated_at, target.updated_at);
        assert_eq!(article.published_at, target.published_at);
        assert_eq!(article.deleted_at, target.deleted_at);
        assert_eq!(article.series_id, target.series_id);
        assert_eq!(article.category_ids, target.category_ids);
        assert_eq!(article.tag_ids, target.tag_ids);
    }

    #[test]
    fn article_case_default() {
        let article = Article::default();
        let now = chrono::Utc::now();
        let target = Article {
            id: 1.try_into().unwrap(),
            title: "Default Title".try_into().unwrap(),
            slug: "default-title".try_into().unwrap(),
            description: "".into(),
            content: None,
            created_at: now.into(),
            updated_at: now.into(),
            published_at: None,
            deleted_at: None,
            series_id: None,
            category_ids: vec![],
            tag_ids: vec![],
        };

        assert_eq!(article.id, target.id);
        assert_eq!(article.title, target.title);
        assert_eq!(article.slug(), target.slug());
        assert_eq!(article.description, target.description);
        assert_eq!(article.content, target.content);
        assert!(
            target.created_at.value() - article.created_at.value() < chrono::Duration::seconds(1)
        );
        assert!(
            target.updated_at.value() - article.updated_at.value() < chrono::Duration::seconds(1)
        );
        assert_eq!(article.published_at, target.published_at);
        assert_eq!(article.deleted_at, target.deleted_at);
        assert_eq!(article.series_id, target.series_id);
        assert_eq!(article.category_ids, target.category_ids);
        assert_eq!(article.tag_ids, target.tag_ids);
    }

    #[test]
    fn article_case_set_title() {
        let mut article = Article::default();

        let target_title: Title = "Default Title".try_into().unwrap();

        assert_eq!(article.title, target_title);
        assert_eq!(article.slug(), &target_title.as_slug());

        let new_title: Title = "New Title".try_into().unwrap();

        article.set_title(new_title.clone());

        assert_eq!(article.title, new_title);
        assert_eq!(article.slug(), &new_title.as_slug());
    }

    #[test]
    fn article_case_set_description() {
        let mut article = Article::default();

        let target_description = Description::default();

        assert_eq!(article.description, target_description);

        let new_description = Description::from("New Description");

        article.set_description(new_description.clone());

        assert_eq!(article.description, new_description);
    }

    #[test]
    fn article_case_set_content() {
        let mut article = Article::default();

        assert_eq!(article.content, None);

        let new_content = Content::from("New Content");

        article.set_content(new_content.clone());

        assert_eq!(article.content, Some(new_content));
    }

    #[test]
    fn article_case_is_published() {
        let mut article = Article::default();

        assert_eq!(article.is_published(), false);

        article.published_at = Some(PublishedAt::from(chrono::Utc::now()));

        assert!(article.is_published());
    }

    #[test]
    fn article_case_publish() {
        let mut article = Article::default();

        assert_eq!(article.published_at, None);

        let res = article.publish();

        assert!(res.is_ok());
        assert!(article.published_at.is_some());

        let now = chrono::Utc::now();

        assert!(article.published_at.unwrap().value() - now < chrono::Duration::seconds(1));

        let res = article.publish();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDomainError::AlreadyPublished(_) => true,
            _ => false,
        });
    }

    #[test]
    fn article_case_unpublish() {
        let mut article = Article::default();

        let res = article.unpublish();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDomainError::NotPublished(_) => true,
            _ => false,
        });

        article.published_at = Some(chrono::Utc::now().into());

        let res = article.unpublish();

        assert!(res.is_ok());
        assert!(article.published_at.is_none());
    }

    #[test]
    fn article_case_soft_delete() {
        let mut article = Article::default();

        let res = article.soft_delete();

        assert!(res.is_ok());
        assert!(article.deleted_at.is_some());

        let now = chrono::Utc::now();

        assert!(article.deleted_at.unwrap().value() - now < chrono::Duration::seconds(1));

        let res = article.soft_delete();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDomainError::AlreadySoftDeleted(_) => true,
            _ => false,
        });
    }

    #[test]
    fn article_case_revoke_soft_delete() {
        let mut article = Article::default();

        let res = article.revoke_soft_delete();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDomainError::NotSoftDeleted(_) => true,
            _ => false,
        });

        article.deleted_at = Some(chrono::Utc::now().into());

        let res = article.revoke_soft_delete();

        assert!(res.is_ok());
        assert!(article.deleted_at.is_none());
    }

    #[test]
    fn article_case_bump_updated_at() {
        let mut article = Article::default();

        let previous_updated_at = article.updated_at;

        article.bump_updated_at();

        assert!(article.updated_at != previous_updated_at);
        assert!(
            article.updated_at.value() - previous_updated_at.value() < chrono::Duration::seconds(1)
        );
    }

    #[test]
    fn article_case_set_series_id() {
        let mut article = Article::default();

        assert!(article.series_id.is_none());

        let new_series_id = 1.try_into().unwrap();

        article.set_series_id(new_series_id);

        assert!(article.series_id.is_some());
        assert_eq!(article.series_id.unwrap(), new_series_id);
    }

    #[test]
    fn article_case_remove_series_id() {
        let mut article = Article::default();

        article.series_id = Some(1.try_into().unwrap());

        assert!(article.series_id.is_some());

        article.remove_series_id();

        assert!(article.series_id.is_none());
    }

    #[test]
    fn article_case_add_category_id() {
        let mut article = Article::default();

        assert!(article.category_ids.is_empty());

        let new_category_id = 1.try_into().unwrap();

        let res = article.add_category_id(new_category_id);

        assert!(res.is_ok());
        assert_eq!(article.category_ids.len(), 1);
        assert!(article.category_ids.contains(&new_category_id));

        let new_category_id = 2.try_into().unwrap();

        let res = article.add_category_id(new_category_id);

        assert!(res.is_ok());
        assert_eq!(article.category_ids.len(), 2);
        assert!(article.category_ids.contains(&new_category_id));

        let new_category_id = 1.try_into().unwrap();

        let res = article.add_category_id(new_category_id);

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDomainError::CategoryAlreadyAttached(_, _) => true,
            _ => false,
        });
    }

    #[test]
    fn article_case_remove_category_id() {
        let mut article = Article::default();

        article.category_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

        assert_eq!(article.category_ids.len(), 2);

        let res = article.remove_category_id(1.try_into().unwrap());

        assert!(res.is_ok());
        assert_eq!(article.category_ids.len(), 1);
        assert!(!article.category_ids.contains(&1.try_into().unwrap()));

        let res = article.remove_category_id(1.try_into().unwrap());

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDomainError::CategoryNotFound(_, _) => true,
            _ => false,
        });

        let res = article.remove_category_id(2.try_into().unwrap());

        assert!(res.is_ok());
        assert_eq!(article.category_ids.len(), 0);
    }

    #[test]
    fn article_case_add_tag_id() {
        let mut article = Article::default();

        assert!(article.tag_ids.is_empty());

        let new_tag_id = 1.try_into().unwrap();

        let res = article.add_tag_id(new_tag_id);

        assert!(res.is_ok());
        assert_eq!(article.tag_ids.len(), 1);
        assert!(article.tag_ids.contains(&new_tag_id));

        let new_tag_id = 2.try_into().unwrap();

        let res = article.add_tag_id(new_tag_id);

        assert!(res.is_ok());
        assert_eq!(article.tag_ids.len(), 2);
        assert!(article.tag_ids.contains(&new_tag_id));

        let new_tag_id = 1.try_into().unwrap();

        let res = article.add_tag_id(new_tag_id);

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDomainError::TagAlreadyAttached(_, _) => true,
            _ => false,
        });
    }

    #[test]
    fn article_case_remove_tag_id() {
        let mut article = Article::default();

        article.tag_ids = vec![1.try_into().unwrap(), 2.try_into().unwrap()];

        assert_eq!(article.tag_ids.len(), 2);

        let res = article.remove_tag_id(1.try_into().unwrap());

        assert!(res.is_ok());
        assert_eq!(article.tag_ids.len(), 1);
        assert!(!article.tag_ids.contains(&1.try_into().unwrap()));

        let res = article.remove_tag_id(1.try_into().unwrap());

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDomainError::TagNotFound(_, _) => true,
            _ => false,
        });

        let res = article.remove_tag_id(2.try_into().unwrap());

        assert!(res.is_ok());
        assert_eq!(article.tag_ids.len(), 0);
    }
}

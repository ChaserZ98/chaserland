use crate::domain::entity::{article, category, series, tag};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::ArticleDTOError;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ArticleDTO {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub series: Option<SeriesDTO>,
    pub categories: Vec<CategoryDTO>,
    pub tags: Vec<TagDTO>,
}

#[derive(Debug, Default)]
pub struct ArticleDTOBuilder {
    article: Option<article::Article>,
    series: Option<series::Series>,
    categories: Vec<category::Category>,
    tags: Vec<tag::Tag>,
}

impl ArticleDTOBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_article(mut self, article: article::Article) -> Self {
        self.article = Some(article);
        self
    }
    pub fn with_series(mut self, series: series::Series) -> Self {
        self.series = Some(series);
        self
    }
    pub fn with_categories(mut self, categories: Vec<category::Category>) -> Self {
        self.categories = categories;
        self
    }
    pub fn with_tags(mut self, tags: Vec<tag::Tag>) -> Self {
        self.tags = tags;
        self
    }
    pub fn try_build(self) -> Result<ArticleDTO, ArticleDTOError> {
        let mut article_dto = ArticleDTO::default();
        if self.article.is_none() {
            return Err(ArticleDTOError::ArticleNotDefined);
        }
        let article = self.article.unwrap();
        article_dto.id = article.id.value();
        article_dto.title = article.title.value();
        article_dto.slug = article.slug().value();
        article_dto.description = article.description.value();
        article_dto.content = article.content.map(|x| x.value());
        article_dto.created_at = article.created_at.value();
        article_dto.updated_at = article.updated_at.value();
        article_dto.published_at = article.published_at.map(|x| x.value());
        article_dto.deleted_at = article.deleted_at.map(|x| x.value());

        let series_id = self.series.as_ref().map(|x| x.id);
        if article.series_id != series_id {
            return Err(ArticleDTOError::SeriesMismatch {
                article_series_id: article.series_id,
                series_id: series_id,
            });
        }
        article_dto.series = self.series.map(|x| x.into());

        if article.category_ids.len() != self.categories.len() {
            return Err(ArticleDTOError::CategoriesLengthMismatch {
                article_categories_length: article.category_ids.len(),
                categories_length: self.categories.len(),
            });
        }
        let mut categories_map = HashMap::new();
        for category in self.categories {
            categories_map.insert(category.id.value(), category);
        }
        for category_id in &article.category_ids {
            let category = categories_map.get(&category_id.value());
            if category.is_none() {
                return Err(ArticleDTOError::CategoriesElementMismatch(*category_id));
            }
            let category = category.unwrap().clone();
            article_dto.categories.push(category.into());
        }

        if article.tag_ids.len() != self.tags.len() {
            return Err(ArticleDTOError::TagsLengthMismatch {
                article_tags_length: article.tag_ids.len(),
                tags_length: self.tags.len(),
            });
        }
        let mut tags_map = HashMap::new();
        for tag in self.tags {
            tags_map.insert(tag.id.value(), tag);
        }
        for tag_id in &article.tag_ids {
            let tag = tags_map.get(&tag_id.value());
            if tag.is_none() {
                return Err(ArticleDTOError::TagsElementMismatch(*tag_id));
            }
            let tag = tag.unwrap().clone();
            article_dto.tags.push(tag.into());
        }

        Ok(article_dto)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SeriesDTO {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<series::Series> for SeriesDTO {
    fn from(value: series::Series) -> Self {
        Self {
            id: value.id.value(),
            name: value.name.value().into(),
            slug: value.slug.value(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CategoryDTO {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<category::Category> for CategoryDTO {
    fn from(value: category::Category) -> Self {
        Self {
            id: value.id.value(),
            name: value.name.value(),
            slug: value.slug.value(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TagDTO {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<tag::Tag> for TagDTO {
    fn from(value: tag::Tag) -> Self {
        Self {
            id: value.id.value(),
            name: value.name.value(),
            slug: value.slug.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArticleDTOBuilder;
    use crate::{
        app::dto::{ArticleDTOError, CategoryDTO, SeriesDTO, TagDTO},
        domain::entity::{article, category, series, tag},
    };

    #[test]
    fn into_article_dto_case_1() {
        let series = series::Series::new(1.try_into().unwrap(), "series".try_into().unwrap());
        let categories = vec![
            category::Category::new(1.try_into().unwrap(), "category-1".try_into().unwrap()),
            category::Category::new(2.try_into().unwrap(), "category-2".try_into().unwrap()),
        ];
        let tags = vec![
            tag::Tag::new(1.try_into().unwrap(), "tag-1".try_into().unwrap()),
            tag::Tag::new(2.try_into().unwrap(), "tag-2".try_into().unwrap()),
        ];
        let mut article = article::Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            Some(series.id),
            categories.iter().map(|x| x.id).collect(),
            tags.iter().map(|x| x.id).collect(),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
        );
        article.published_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());
        article.deleted_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());

        let article_dto_builder = ArticleDTOBuilder::new()
            .with_article(article.clone())
            .with_series(series.clone())
            .with_categories(categories.clone())
            .with_tags(tags.clone());

        let article_dto = article_dto_builder.try_build();

        assert!(article_dto.is_ok());

        let article_dto = article_dto.unwrap();

        assert_eq!(article_dto.id, article.id.value());
        assert_eq!(article_dto.title, article.title.value());
        assert_eq!(article_dto.slug, article.slug().value());
        assert_eq!(article_dto.description, article.description.value());
        assert_eq!(article_dto.content, article.content.map(|x| x.value()));
        assert_eq!(article_dto.created_at, article.created_at.value());
        assert_eq!(article_dto.updated_at, article.updated_at.value());
        assert_eq!(
            article_dto.published_at,
            article.published_at.map(|x| x.value())
        );
        assert_eq!(
            article_dto.deleted_at,
            article.deleted_at.map(|x| x.value())
        );

        let series_dto = SeriesDTO {
            id: series.id.value(),
            name: series.name.value().to_string(),
            slug: series.slug.value(),
        };
        assert_eq!(article_dto.series, Some(series_dto));

        let categories_dto = categories
            .iter()
            .map(|x| CategoryDTO {
                id: x.id.value(),
                name: x.name.value().to_string(),
                slug: x.slug.value(),
            })
            .collect::<Vec<CategoryDTO>>();
        assert_eq!(article_dto.categories, categories_dto);

        let tags_dto = tags
            .iter()
            .map(|x| TagDTO {
                id: x.id.value(),
                name: x.name.value().to_string(),
                slug: x.slug.value(),
            })
            .collect::<Vec<TagDTO>>();
        assert_eq!(article_dto.tags, tags_dto);
    }

    #[test]
    fn into_article_dto_case_article_not_defined() {
        let article_dto_builder = ArticleDTOBuilder::new();
        let res = article_dto_builder.try_build();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(matches!(res, ArticleDTOError::ArticleNotDefined));
    }

    #[test]
    fn into_article_dto_case_series_mismatch() {
        let series = series::Series::new(1.try_into().unwrap(), "series".try_into().unwrap());

        let mut article = article::Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            None,
            vec![],
            vec![],
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
        );
        article.published_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());
        article.deleted_at = Some("2020-01-01 00:00:00 UTC".try_into().unwrap());

        let article_dto_builder = ArticleDTOBuilder::new()
            .with_article(article.clone())
            .with_series(series.clone());

        let res = article_dto_builder.try_build();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDTOError::SeriesMismatch {
                article_series_id,
                series_id,
            } => article_series_id == article.series_id && series_id == Some(series.id),
            _ => false,
        });
    }

    #[test]
    fn into_article_dto_case_categories_length_mismatch() {
        let categories = vec![category::Category::new(
            1.try_into().unwrap(),
            "category-1".try_into().unwrap(),
        )];
        let article = article::Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            None,
            vec![1.try_into().unwrap(), 2.try_into().unwrap()],
            vec![],
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
        );

        let article_dto_builder = ArticleDTOBuilder::new()
            .with_article(article.clone())
            .with_categories(categories.clone());

        let res = article_dto_builder.try_build();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDTOError::CategoriesLengthMismatch {
                article_categories_length,
                categories_length,
            } =>
                article_categories_length == article.category_ids.len()
                    && categories_length == categories.len(),
            _ => false,
        });
    }

    #[test]
    fn into_article_dto_case_categories_element_mismatch() {
        let categories = vec![category::Category::new(
            1.try_into().unwrap(),
            "category-1".try_into().unwrap(),
        )];
        let article = article::Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            None,
            vec![2.try_into().unwrap()],
            vec![],
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
        );

        let article_dto_builder = ArticleDTOBuilder::new()
            .with_article(article.clone())
            .with_categories(categories.clone());

        let res = article_dto_builder.try_build();

        assert!(res.is_err());

        let res = res.unwrap_err();

        let missing_category_id = 2.try_into().unwrap();
        assert!(match res {
            ArticleDTOError::CategoriesElementMismatch(id) => id == missing_category_id,
            _ => false,
        });
    }

    #[test]
    fn into_article_dto_case_tags_length_mismatch() {
        let tags = vec![tag::Tag::new(
            1.try_into().unwrap(),
            "tag-1".try_into().unwrap(),
        )];
        let article = article::Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            None,
            vec![],
            vec![1.try_into().unwrap(), 2.try_into().unwrap()],
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
        );

        let article_dto_builder = ArticleDTOBuilder::new()
            .with_article(article.clone())
            .with_tags(tags.clone());

        let res = article_dto_builder.try_build();

        assert!(res.is_err());

        let res = res.unwrap_err();

        assert!(match res {
            ArticleDTOError::TagsLengthMismatch {
                article_tags_length,
                tags_length,
            } => article_tags_length == article.tag_ids.len() && tags_length == tags.len(),
            _ => false,
        });
    }

    #[test]
    fn into_article_dto_case_tags_element_mismatch() {
        let tags = vec![tag::Tag::new(
            1.try_into().unwrap(),
            "tag-1".try_into().unwrap(),
        )];
        let article = article::Article::new(
            1.try_into().unwrap(),
            "title".try_into().unwrap(),
            "description".into(),
            Some("content".into()),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
            None,
            vec![],
            vec![2.try_into().unwrap()],
            "2020-01-01 00:00:00 UTC".try_into().unwrap(),
        );

        let article_dto_builder = ArticleDTOBuilder::new()
            .with_article(article.clone())
            .with_tags(tags.clone());

        let res = article_dto_builder.try_build();

        assert!(res.is_err());

        let res = res.unwrap_err();

        let missing_tag_id = 2.try_into().unwrap();

        assert!(match res {
            ArticleDTOError::TagsElementMismatch(id) => id == missing_tag_id,
            _ => false,
        });
    }
}

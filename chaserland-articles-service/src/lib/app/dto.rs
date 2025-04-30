use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::domain::entity::{article, category, series, tag};

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
    pub fn try_build(self) -> Result<ArticleDTO, String> {
        let mut article_dto = ArticleDTO::default();
        if self.article.is_none() {
            return Err("Article is not defined.".to_string());
        }
        let article = self.article.unwrap();
        article_dto.id = article.id.value();
        article_dto.title = article.title.value();
        article_dto.slug = article.slug().value();
        article_dto.description = article.description.value();
        article_dto.content = article.content.map(|x| x.value());
        match (article.series_id, self.series) {
            (Some(series_id), Some(series)) if series.id == series_id => {
                article_dto.series = Some(series.into());
            }
            (None, None) => {}
            _ => {
                return Err("Series mismatch".to_string());
            }
        };
        if article.category_ids.len() != self.categories.len() {
            return Err("Categories mismatch".to_string());
        }
        let mut categories_map = HashMap::new();
        for category in self.categories {
            categories_map.insert(category.id.value(), category);
        }
        for category_id in &article.category_ids {
            let category = categories_map.get(&category_id.value());
            if category.is_none() {
                return Err("Categories mismatch".to_string());
            }
            let category = category.unwrap().clone();
            article_dto.categories.push(category.into());
        }

        if article.tag_ids.len() != self.tags.len() {
            return Err("Tags mismatch".to_string());
        }
        let mut tags_map = HashMap::new();
        for tag in self.tags {
            tags_map.insert(tag.id.value(), tag);
        }
        for tag_id in &article.tag_ids {
            let tag = tags_map.get(&tag_id.value());
            if tag.is_none() {
                return Err("Tags mismatch".to_string());
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

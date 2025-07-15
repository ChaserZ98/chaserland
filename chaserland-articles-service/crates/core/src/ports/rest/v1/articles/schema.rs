use crate::{
    app::{
        command::CreateArticleCommand,
        query::dto::{ArticleDTO, CategoryDTO, TagDTO},
    },
    ports::rest::{response::ErrorResponse, v1::series::schema as series_schema},
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ArticleCreate {
    pub title: String,
    pub description: String,
    pub content: Option<String>,
    pub series_id: Option<i32>,
    pub category_ids: Vec<i32>,
    pub tag_ids: Vec<i32>,
}

impl TryInto<CreateArticleCommand> for ArticleCreate {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<CreateArticleCommand, Self::Error> {
        let title = self
            .title
            .try_into()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
        let description = self.description.into();
        let content = self.content.map(|content| content.into());
        let series_id = self
            .series_id
            .map(|id| id.try_into())
            .transpose()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
        let category_ids = self
            .category_ids
            .into_iter()
            .map(|id| id.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
        let tag_ids = self
            .tag_ids
            .into_iter()
            .map(|id| id.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;

        let command = CreateArticleCommand {
            title,
            description,
            content,
            series_id,
            category_ids,
            tag_ids,
        };

        Ok(command)
    }
}

#[derive(Serialize, ToSchema)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: Option<String>,
    pub series: Option<series_schema::Series>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub published_at: Option<String>,
}

impl From<ArticleDTO> for Article {
    fn from(value: ArticleDTO) -> Self {
        let id = value.id;
        let title = value.title;
        let slug = value.slug;
        let description = value.description;
        let content = value.content;
        let series = value.series.map(|x| x.into());
        let categories = value.categories.into_iter().map(|x| x.into()).collect();
        let tags = value.tags.into_iter().map(|x| x.into()).collect();
        let created_at = value.created_at.to_string();
        let updated_at = value.updated_at.to_string();
        let deleted_at = value.deleted_at.map(|x| x.to_string());
        let published_at = value.published_at.map(|x| x.to_string());

        Self {
            id,
            title,
            slug,
            description,
            content,
            series,
            categories,
            tags,
            created_at,
            updated_at,
            deleted_at,
            published_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<CategoryDTO> for Category {
    fn from(value: CategoryDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<TagDTO> for Tag {
    fn from(value: TagDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}

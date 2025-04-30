use crate::app::dto::{ArticleDTO, CategoryDTO, SeriesDTO, TagDTO};
use crate::domain::entity::{article, category, series, tag};
use chaserland_protos::article::v1::CreateArticleRequest;
use chrono::{DateTime, Utc};
use tonic::Status;

impl TryInto<article::ArticleCreate> for CreateArticleRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<article::ArticleCreate, Self::Error> {
        if self.article.is_none() {
            return Err(Status::invalid_argument("Article is required"));
        }
        let article = self.article.unwrap();
        let title: article::Title = article
            .title
            .try_into()
            .map_err(|why| Status::invalid_argument(why))?;
        let description: article::Description = article.description.into();
        let content: Option<article::Content> = Some(article.content.into());
        let series_id: Option<series::Id> = article
            .series_id
            .map(|id| id.try_into())
            .transpose()
            .map_err(|why| Status::invalid_argument(why))?;
        let category_ids: Vec<category::Id> = article
            .category_ids
            .into_iter()
            .map(|id| id.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|why| Status::invalid_argument(why))?;

        let tag_ids: Vec<tag::Id> = article
            .tag_ids
            .into_iter()
            .map(|id| id.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|why| Status::invalid_argument(why))?;

        let mut article_create = article::ArticleCreate::default();
        article_create.title = title;
        article_create.description = description;
        article_create.content = content;
        article_create.series_id = series_id;
        article_create.category_ids = category_ids;
        article_create.tag_ids = tag_ids;

        Ok(article_create)
    }
}
pub struct ProstTimestampWrapper(prost_types::Timestamp);

impl ProstTimestampWrapper {
    pub fn into_inner(self) -> prost_types::Timestamp {
        self.0
    }
}

impl From<DateTime<Utc>> for ProstTimestampWrapper {
    fn from(value: DateTime<Utc>) -> Self {
        Self(prost_types::Timestamp {
            seconds: value.timestamp(),
            nanos: value.timestamp_subsec_nanos() as i32,
        })
    }
}

impl From<ArticleDTO> for chaserland_protos::article::v1::Article {
    fn from(value: ArticleDTO) -> Self {
        Self {
            id: value.id,
            title: value.title,
            slug: value.slug,
            description: value.description,
            content: value.content,
            created_at: Some(ProstTimestampWrapper::from(value.created_at).into_inner()),
            updated_at: Some(ProstTimestampWrapper::from(value.updated_at).into_inner()),
            deleted_at: value
                .deleted_at
                .map(|x| ProstTimestampWrapper::from(x).into_inner()),
            published_at: value
                .published_at
                .map(|x| ProstTimestampWrapper::from(x).into_inner()),
            series: value.series.map(|x| x.into()),
            categories: value.categories.into_iter().map(|x| x.into()).collect(),
            tags: value.tags.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<SeriesDTO> for chaserland_protos::article::v1::Series {
    fn from(value: SeriesDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}

impl From<CategoryDTO> for chaserland_protos::article::v1::Category {
    fn from(value: CategoryDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}

impl From<TagDTO> for chaserland_protos::article::v1::Tag {
    fn from(value: TagDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}

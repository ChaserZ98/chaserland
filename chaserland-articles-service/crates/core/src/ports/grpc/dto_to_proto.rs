use crate::app::query::dto::{ArticleDTO, CategoryDTO, SeriesDTO, TagDTO};
use chaserland_protos::article::v1;
use chrono::{DateTime, Utc};

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

impl From<ArticleDTO> for v1::Article {
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

// impl From<ArticleDTO> for v1::CreateArticleResponse {
//     fn from(value: ArticleDTO) -> Self {
//         let article = v1::Article::from(value);
//         Self {
//             article: Some(article),
//         }
//     }
// }

impl From<SeriesDTO> for v1::Series {
    fn from(value: SeriesDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}

// impl From<SeriesDTO> for v1::CreateSeriesResponse {
//     fn from(value: SeriesDTO) -> Self {
//         Self {
//             series: Some(v1::Series::from(value)),
//         }
//     }
// }

impl From<CategoryDTO> for v1::Category {
    fn from(value: CategoryDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}

// impl From<CategoryDTO> for v1::CreateCategoryResponse {
//     fn from(value: CategoryDTO) -> Self {
//         Self {
//             category: Some(v1::Category::from(value)),
//         }
//     }
// }

impl From<TagDTO> for v1::Tag {
    fn from(value: TagDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            slug: value.slug,
        }
    }
}

// impl From<TagDTO> for v1::CreateTagResponse {
//     fn from(value: TagDTO) -> Self {
//         Self {
//             tag: Some(v1::Tag::from(value)),
//         }
//     }
// }

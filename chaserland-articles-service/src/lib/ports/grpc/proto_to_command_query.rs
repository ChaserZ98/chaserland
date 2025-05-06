use crate::app::{command, query};
use crate::domain::entity::{article, category, series, tag};
use crate::domain::repository::article::ArticlesFilter;
use chaserland_common::pagination::Pagination;
use chaserland_protos::article::v1;
use tonic::Status;

impl TryInto<command::CreateArticleCommand> for v1::CreateArticleRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::CreateArticleCommand, Self::Error> {
        if self.article.is_none() {
            return Err(Status::invalid_argument("Field article is required"));
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

        let command = command::CreateArticleCommand {
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

impl TryInto<command::CreateSeriesCommand> for v1::CreateSeriesRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::CreateSeriesCommand, Self::Error> {
        if self.series.is_none() {
            return Err(Status::invalid_argument("Field series is required"));
        }

        let series = self.series.unwrap();
        let name = series
            .name
            .try_into()
            .map_err(|why| Status::invalid_argument(why))?;

        Ok(command::CreateSeriesCommand { name })
    }
}

impl TryInto<command::CreateCategoryCommand> for v1::CreateCategoryRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::CreateCategoryCommand, Self::Error> {
        if self.category.is_none() {
            return Err(Status::invalid_argument("Field category is required"));
        }

        let category = self.category.unwrap();

        let name = category
            .name
            .try_into()
            .map_err(|why| Status::invalid_argument(why))?;

        Ok(command::CreateCategoryCommand { name })
    }
}

impl TryInto<command::CreateTagCommand> for v1::CreateTagRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::CreateTagCommand, Self::Error> {
        if self.tag.is_none() {
            return Err(Status::invalid_argument("Field tag is required"));
        }

        let tag = self.tag.unwrap();

        let name = tag
            .name
            .try_into()
            .map_err(|why| Status::invalid_argument(why))?;

        Ok(command::CreateTagCommand { name })
    }
}

impl TryInto<article::Identifier> for v1::ArticleIdentifier {
    type Error = tonic::Status;

    fn try_into(self) -> Result<article::Identifier, Self::Error> {
        let identifier = self.identifier;
        if identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }
        let identifier = identifier.unwrap();
        match identifier {
            v1::article_identifier::Identifier::Id(id) => {
                let id = id.try_into().map_err(|why| Status::invalid_argument(why))?;
                Ok(article::Identifier::Id(id))
            }
            v1::article_identifier::Identifier::Slug(slug) => {
                let slug = slug
                    .try_into()
                    .map_err(|why| Status::invalid_argument(why))?;
                Ok(article::Identifier::Slug(slug))
            }
        }
    }
}

impl TryInto<query::GetArticleOneQuery> for v1::GetArticleOneRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetArticleOneQuery, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(query::GetArticleOneQuery {
            identifier,
            public_only: self.public_only,
            with_content: self.with_content,
        })
    }
}

impl TryInto<query::GetArticleContentQuery> for v1::GetArticleContentRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetArticleContentQuery, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(query::GetArticleContentQuery {
            identifier,
            public_only: self.public_only,
        })
    }
}

impl TryInto<series::Identifier> for v1::SeriesIdentifier {
    type Error = tonic::Status;

    fn try_into(self) -> Result<series::Identifier, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }
        let identifier = self.identifier.unwrap();
        match identifier {
            v1::series_identifier::Identifier::Id(id) => {
                let id = id.try_into().map_err(|why| Status::invalid_argument(why))?;
                Ok(series::Identifier::Id(id))
            }
            v1::series_identifier::Identifier::Slug(slug) => {
                let slug = slug
                    .try_into()
                    .map_err(|why| Status::invalid_argument(why))?;
                Ok(series::Identifier::Slug(slug))
            }
        }
    }
}

impl TryInto<ArticlesFilter> for v1::ArticlesFilter {
    type Error = tonic::Status;

    fn try_into(self) -> Result<ArticlesFilter, Self::Error> {
        if self.series_identifier.is_none()
            && self.category_ids.is_empty()
            && self.tag_ids.is_empty()
        {
            return Err(Status::invalid_argument(
                "At least one filter must be specified",
            ));
        }

        let series_identifier = match self.series_identifier {
            None => None,
            Some(value) => Some(value.try_into()?),
        };
        let category_ids = self
            .category_ids
            .iter()
            .map(|v| (*v).try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|why| Status::invalid_argument(why))?;
        let tag_ids = self
            .tag_ids
            .iter()
            .map(|v| (*v).try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|why| Status::invalid_argument(why))?;

        Ok(ArticlesFilter::new(
            series_identifier,
            category_ids,
            tag_ids,
        ))
    }
}

impl TryInto<query::GetArticleManyQuery> for v1::GetArticleManyRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetArticleManyQuery, Self::Error> {
        let page = self
            .page
            .try_into()
            .map_err(|why| Status::invalid_argument(why))?;
        let page_size = self
            .page_size
            .try_into()
            .map_err(|why| Status::invalid_argument(why))?;
        let pagination = Pagination::new(page, page_size);

        let filter = match self.filter {
            None => None,
            Some(value) => Some(value.try_into()?),
        };

        Ok(query::GetArticleManyQuery {
            filter,
            pagination,
            public_only: self.public_only,
            with_content: self.with_content,
        })
    }
}

impl TryInto<query::GetSeriesOneQuery> for v1::GetSeriesOneRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetSeriesOneQuery, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(query::GetSeriesOneQuery { identifier })
    }
}

impl TryInto<query::GetSeriesManyQuery> for v1::GetSeriesManyRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetSeriesManyQuery, Self::Error> {
        Ok(query::GetSeriesManyQuery {
            filter: None,
            pagination: None,
        })
    }
}

impl TryInto<category::Identifier> for v1::CategoryIdentifier {
    type Error = tonic::Status;

    fn try_into(self) -> Result<category::Identifier, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }
        let identifier = self.identifier.unwrap();
        match identifier {
            v1::category_identifier::Identifier::Id(id) => {
                let id = id.try_into().map_err(|why| Status::invalid_argument(why))?;
                Ok(category::Identifier::Id(id))
            }
            v1::category_identifier::Identifier::Slug(slug) => {
                let slug = slug
                    .try_into()
                    .map_err(|why| Status::invalid_argument(why))?;
                Ok(category::Identifier::Slug(slug))
            }
        }
    }
}

impl TryInto<query::GetCategoryOneQuery> for v1::GetCategoryOneRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetCategoryOneQuery, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(query::GetCategoryOneQuery { identifier })
    }
}

impl TryInto<query::GetCategoryManyQuery> for v1::GetCategoryManyRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetCategoryManyQuery, Self::Error> {
        Ok(query::GetCategoryManyQuery {
            filter: None,
            pagination: None,
        })
    }
}

impl TryInto<tag::Identifier> for v1::TagIdentifier {
    type Error = tonic::Status;

    fn try_into(self) -> Result<tag::Identifier, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }
        let identifier = self.identifier.unwrap();
        match identifier {
            v1::tag_identifier::Identifier::Id(id) => {
                let id = id.try_into().map_err(|why| Status::invalid_argument(why))?;
                Ok(tag::Identifier::Id(id))
            }
            v1::tag_identifier::Identifier::Slug(slug) => {
                let slug = slug
                    .try_into()
                    .map_err(|why| Status::invalid_argument(why))?;
                Ok(tag::Identifier::Slug(slug))
            }
        }
    }
}

impl TryInto<query::GetTagOneQuery> for v1::GetTagOneRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetTagOneQuery, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(query::GetTagOneQuery { identifier })
    }
}

impl TryInto<query::GetTagManyQuery> for v1::GetTagManyRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<query::GetTagManyQuery, Self::Error> {
        Ok(query::GetTagManyQuery {
            filter: None,
            pagination: None,
        })
    }
}

impl TryInto<command::PublishArticleCommand> for v1::PublishArticleRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::PublishArticleCommand, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(command::PublishArticleCommand { identifier })
    }
}

impl TryInto<command::UnpublishArticleCommand> for v1::UnpublishArticleRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::UnpublishArticleCommand, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(command::UnpublishArticleCommand { identifier })
    }
}

impl TryInto<command::SoftDeleteArticleCommand> for v1::SoftDeleteArticleRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::SoftDeleteArticleCommand, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(command::SoftDeleteArticleCommand { identifier })
    }
}

impl TryInto<command::RevokeSoftDeleteArticleCommand> for v1::RevokeSoftDeleteArticleRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::RevokeSoftDeleteArticleCommand, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(command::RevokeSoftDeleteArticleCommand { identifier })
    }
}

impl TryInto<command::DeleteArticleCommand> for v1::DeleteArticleRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::DeleteArticleCommand, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(command::DeleteArticleCommand { identifier })
    }
}

impl TryInto<command::DeleteSeriesCommand> for v1::DeleteSeriesRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::DeleteSeriesCommand, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(command::DeleteSeriesCommand { identifier })
    }
}

impl TryInto<command::DeleteCategoryCommand> for v1::DeleteCategoryRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::DeleteCategoryCommand, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(command::DeleteCategoryCommand { identifier })
    }
}

impl TryInto<command::DeleteTagCommand> for v1::DeleteTagRequest {
    type Error = tonic::Status;

    fn try_into(self) -> Result<command::DeleteTagCommand, Self::Error> {
        if self.identifier.is_none() {
            return Err(Status::invalid_argument("Field identifier is required"));
        }

        let identifier = self.identifier.unwrap().try_into()?;

        Ok(command::DeleteTagCommand { identifier })
    }
}

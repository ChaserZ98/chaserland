use crate::{
    app::{command, query},
    domain::{
        article::{repository::ArticlesFilter, vo as article},
        category::vo as category,
        series::vo as series,
        tag::vo as tag,
    },
    ports::rest::response::ErrorResponse,
};
use chaserland_common::pagination::Pagination;
use http::StatusCode;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
pub struct GetArticleOneQuery {
    /// Search in public articles only
    #[param(default = false)]
    pub public_only: Option<bool>,
    /// Include article content in response body
    #[param(default = false)]
    pub with_content: Option<bool>,
}

impl TryInto<query::GetArticleOneQuery> for (String, GetArticleOneQuery) {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<query::GetArticleOneQuery, Self::Error> {
        let (identifier, query) = self;

        let identifier = match identifier.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| article::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => identifier
                .try_into()
                .map(|slug| article::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;
        let public_only = query.public_only.unwrap_or(false);
        let with_content = query.with_content.unwrap_or(false);

        Ok(query::GetArticleOneQuery {
            identifier,
            public_only,
            with_content,
        })
    }
}

#[derive(IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
pub struct GetArticleContentQuery {
    /// Search in public articles only
    #[param(default = false)]
    pub public_only: Option<bool>,
}

impl TryInto<query::GetArticleContentQuery> for (String, GetArticleContentQuery) {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<query::GetArticleContentQuery, Self::Error> {
        let (identifier, query) = self;

        let identifier = match identifier.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| article::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => identifier
                .try_into()
                .map(|slug| article::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;
        let public_only = query.public_only.unwrap_or(false);

        Ok(query::GetArticleContentQuery {
            identifier,
            public_only,
        })
    }
}

#[derive(IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
pub struct GetArticleManyQuery {
    /// Page number
    #[param(default = 1, minimum = 1)]
    pub page: Option<i32>,
    /// Page size
    #[param(default = 10, minimum = 1, maximum = 10)]
    pub page_size: Option<i32>,
    /// Search in public articles only
    #[param(default = false)]
    pub public_only: Option<bool>,
    /// Include article content in response body
    #[param(default = false)]
    pub with_content: Option<bool>,
    /// Filter by series identifier
    pub series_identifier: Option<String>,
    /// Filter by category identifiers
    pub category_ids: Option<Vec<i32>>,
    /// Filter by tag identifiers
    pub tag_ids: Option<Vec<i32>>,
}

impl TryInto<query::GetArticleManyQuery> for GetArticleManyQuery {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<query::GetArticleManyQuery, Self::Error> {
        let page = self
            .page
            .unwrap_or(1)
            .try_into()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
        let page_size = self
            .page
            .unwrap_or(10)
            .try_into()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
        let pagination = Pagination::new(page, page_size);

        let public_only = self.public_only.unwrap_or(false);
        let with_content = self.with_content.unwrap_or(false);

        let filter = match (&self.series_identifier, &self.category_ids, &self.tag_ids) {
            (None, None, None) => None,
            _ => {
                let series_identifier = match self.series_identifier {
                    None => None,
                    Some(identifier) => match identifier.parse::<i32>() {
                        Ok(id) => {
                            let id = id
                                .try_into()
                                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                            Some(series::Identifier::Id(id))
                        }
                        _ => {
                            let slug = identifier
                                .try_into()
                                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                            Some(series::Identifier::Slug(slug))
                        }
                    },
                };
                let category_ids = match self.category_ids {
                    None => vec![],
                    Some(ids) => ids
                        .into_iter()
                        .map(|id| {
                            id.try_into()
                                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))
                        })
                        .collect::<Result<Vec<category::Id>, ErrorResponse>>()?,
                };
                let tag_ids = match self.tag_ids {
                    None => vec![],
                    Some(ids) => ids
                        .into_iter()
                        .map(|id| {
                            id.try_into()
                                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))
                        })
                        .collect::<Result<Vec<tag::Id>, ErrorResponse>>()?,
                };
                let filter = ArticlesFilter::try_new(series_identifier, category_ids, tag_ids)
                    .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                Some(filter)
            }
        };

        Ok(query::GetArticleManyQuery {
            pagination,
            public_only,
            with_content,
            filter,
        })
    }
}

impl TryInto<command::DeleteArticleCommand> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<command::DeleteArticleCommand, Self::Error> {
        let identifier = self
            .parse()
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;

        Ok(command::DeleteArticleCommand { identifier })
    }
}

impl TryInto<command::PublishArticleCommand> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<command::PublishArticleCommand, Self::Error> {
        let identifier = match self.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| article::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => self
                .try_into()
                .map(|slug| article::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;

        Ok(command::PublishArticleCommand { identifier })
    }
}

impl TryInto<command::UnpublishArticleCommand> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<command::UnpublishArticleCommand, Self::Error> {
        let identifier = match self.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| article::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => self
                .try_into()
                .map(|slug| article::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;

        Ok(command::UnpublishArticleCommand { identifier })
    }
}

impl TryInto<command::SoftDeleteArticleCommand> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<command::SoftDeleteArticleCommand, Self::Error> {
        let identifier = match self.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| article::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => self
                .try_into()
                .map(|slug| article::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;

        Ok(command::SoftDeleteArticleCommand { identifier })
    }
}

impl TryInto<command::RevokeSoftDeleteArticleCommand> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<command::RevokeSoftDeleteArticleCommand, Self::Error> {
        let identifier = match self.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| article::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => self
                .try_into()
                .map(|slug| article::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;

        Ok(command::RevokeSoftDeleteArticleCommand { identifier })
    }
}

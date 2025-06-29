use crate::{
    app::{command, query},
    domain::tag::{repository::TagsFilter, vo as tag},
    ports::rest::response::ErrorResponse,
};
use chaserland_common::pagination::Pagination;
use http::StatusCode;
use serde::Deserialize;
use utoipa::IntoParams;

impl TryInto<query::GetTagOneQuery> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<query::GetTagOneQuery, Self::Error> {
        let identifier = match self.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| tag::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => self
                .try_into()
                .map(|slug| tag::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;

        Ok(query::GetTagOneQuery { identifier })
    }
}

#[derive(IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
pub struct GetTagManyQuery {
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub page_size: Option<i32>,
    /// series ids
    pub series_ids: Option<Vec<i32>>,
}

impl TryInto<query::GetTagManyQuery> for GetTagManyQuery {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<query::GetTagManyQuery, Self::Error> {
        let pagination = match (self.page, self.page_size) {
            (None, None) => None,
            (Some(page), Some(page_size)) => {
                let page = page
                    .try_into()
                    .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                let page_size = page_size
                    .try_into()
                    .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                Some(Pagination::new(page, page_size))
            }
            (Some(_), None) => {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    "Page size is required when page is specified".into(),
                ));
            }
            (None, Some(_)) => {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    "Page is required when page size is specified".into(),
                ));
            }
        };

        let filter = match self.series_ids {
            None => None,
            Some(ids) => {
                let ids = ids
                    .into_iter()
                    .map(|id| id.try_into())
                    .collect::<Result<Vec<tag::Id>, _>>()
                    .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                let filter = TagsFilter::try_new(ids)
                    .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                Some(filter)
            }
        };

        Ok(query::GetTagManyQuery { pagination, filter })
    }
}

impl TryInto<command::DeleteTagCommand> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<command::DeleteTagCommand, Self::Error> {
        let identifier = match self.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| tag::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => self
                .try_into()
                .map(|slug| tag::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;

        Ok(command::DeleteTagCommand { identifier })
    }
}

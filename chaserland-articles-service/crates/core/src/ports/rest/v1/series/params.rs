use crate::{
    app::{command, query},
    domain::series::{repository::SeriesFilter, vo as series},
    ports::rest::response::ErrorResponse,
};
use chaserland_common::pagination::Pagination;
use http::StatusCode;
use serde::Deserialize;
use utoipa::IntoParams;

impl TryInto<query::GetSeriesOneQuery> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<query::GetSeriesOneQuery, Self::Error> {
        let identifier = match self.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| series::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => self
                .try_into()
                .map(|slug| series::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;

        Ok(query::GetSeriesOneQuery { identifier })
    }
}

#[derive(IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
pub struct GetSeriesManyQuery {
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub page_size: Option<i32>,
    /// series ids
    pub series_ids: Option<Vec<i32>>,
}

impl TryInto<query::GetSeriesManyQuery> for GetSeriesManyQuery {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<query::GetSeriesManyQuery, Self::Error> {
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
                    .collect::<Result<Vec<series::Id>, _>>()
                    .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                let filter = SeriesFilter::try_new(ids)
                    .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e))?;
                Some(filter)
            }
        };

        Ok(query::GetSeriesManyQuery { pagination, filter })
    }
}

impl TryInto<command::DeleteSeriesCommand> for String {
    type Error = ErrorResponse;
    fn try_into(self) -> Result<command::DeleteSeriesCommand, Self::Error> {
        let identifier = match self.parse::<i32>() {
            Ok(id) => id
                .try_into()
                .map(|id| series::Identifier::Id(id))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
            _ => self
                .try_into()
                .map(|slug| series::Identifier::Slug(slug))
                .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e)),
        }?;

        Ok(command::DeleteSeriesCommand { identifier })
    }
}

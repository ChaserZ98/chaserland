use crate::{
    app::{command::error as command_error, query::error as query_error},
    ports::rest::response::ErrorResponse,
};
use http::StatusCode;

impl From<command_error::CreateSeriesError> for ErrorResponse {
    fn from(value: command_error::CreateSeriesError) -> Self {
        match value {
            command_error::CreateSeriesError::DuplicateSlug(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetSeriesOneError> for ErrorResponse {
    fn from(value: query_error::GetSeriesOneError) -> Self {
        match value {
            query_error::GetSeriesOneError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetSeriesManyError> for ErrorResponse {
    fn from(_value: query_error::GetSeriesManyError) -> Self {
        Self::new_default(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<command_error::DeleteSeriesError> for ErrorResponse {
    fn from(value: command_error::DeleteSeriesError) -> Self {
        match value {
            command_error::DeleteSeriesError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

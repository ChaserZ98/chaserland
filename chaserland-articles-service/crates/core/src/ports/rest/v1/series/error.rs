use crate::{app::error as app_error, ports::rest::response::ErrorResponse};
use http::StatusCode;

impl From<app_error::CreateSeriesError> for ErrorResponse {
    fn from(value: app_error::CreateSeriesError) -> Self {
        match value {
            app_error::CreateSeriesError::DuplicateSlug(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::GetSeriesOneError> for ErrorResponse {
    fn from(value: app_error::GetSeriesOneError) -> Self {
        match value {
            app_error::GetSeriesOneError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::GetSeriesManyError> for ErrorResponse {
    fn from(_value: app_error::GetSeriesManyError) -> Self {
        Self::new_default(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<app_error::DeleteSeriesError> for ErrorResponse {
    fn from(value: app_error::DeleteSeriesError) -> Self {
        match value {
            app_error::DeleteSeriesError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

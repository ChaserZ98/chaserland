use crate::{app::error as app_error, ports::rest::response::ErrorResponse};
use http::StatusCode;

impl From<app_error::CreateArticleError> for ErrorResponse {
    fn from(value: app_error::CreateArticleError) -> Self {
        match value {
            app_error::CreateArticleError::CategoryNotFound(_) => {
                Self::new(StatusCode::BAD_REQUEST, value.to_string())
            }
            app_error::CreateArticleError::SeriesNotFound(_) => {
                Self::new(StatusCode::BAD_REQUEST, value.to_string())
            }
            app_error::CreateArticleError::TagNotFound(_) => {
                Self::new(StatusCode::BAD_REQUEST, value.to_string())
            }
            app_error::CreateArticleError::DuplicateSlug(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            app_error::CreateArticleError::DataVersionConflict(message) => {
                Self::new(StatusCode::CONFLICT, message)
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::GetArticleOneError> for ErrorResponse {
    fn from(value: app_error::GetArticleOneError) -> Self {
        match value {
            app_error::GetArticleOneError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            app_error::GetArticleOneError::DataVersionConflict(message) => {
                Self::new(StatusCode::CONFLICT, message)
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::GetArticleManyError> for ErrorResponse {
    fn from(value: app_error::GetArticleManyError) -> Self {
        match value {
            app_error::GetArticleManyError::DataVersionConflict(message) => {
                Self::new(StatusCode::CONFLICT, message)
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

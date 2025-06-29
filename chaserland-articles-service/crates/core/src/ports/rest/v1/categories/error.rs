use crate::{app::error as app_error, ports::rest::response::ErrorResponse};
use http::StatusCode;

impl From<app_error::CreateCategoryError> for ErrorResponse {
    fn from(value: app_error::CreateCategoryError) -> Self {
        match value {
            app_error::CreateCategoryError::DuplicateSlug(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::GetCategoryOneError> for ErrorResponse {
    fn from(value: app_error::GetCategoryOneError) -> Self {
        match value {
            app_error::GetCategoryOneError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::GetCategoryManyError> for ErrorResponse {
    fn from(_value: app_error::GetCategoryManyError) -> Self {
        Self::new_default(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<app_error::DeleteCategoryError> for ErrorResponse {
    fn from(value: app_error::DeleteCategoryError) -> Self {
        match value {
            app_error::DeleteCategoryError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

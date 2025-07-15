use crate::{
    app::{command::error as command_error, query::error as query_error},
    ports::rest::response::ErrorResponse,
};
use http::StatusCode;

impl From<command_error::CreateCategoryError> for ErrorResponse {
    fn from(value: command_error::CreateCategoryError) -> Self {
        match value {
            command_error::CreateCategoryError::DuplicateSlug(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetCategoryOneError> for ErrorResponse {
    fn from(value: query_error::GetCategoryOneError) -> Self {
        match value {
            query_error::GetCategoryOneError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetCategoryManyError> for ErrorResponse {
    fn from(_value: query_error::GetCategoryManyError) -> Self {
        Self::new_default(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<command_error::DeleteCategoryError> for ErrorResponse {
    fn from(value: command_error::DeleteCategoryError) -> Self {
        match value {
            command_error::DeleteCategoryError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

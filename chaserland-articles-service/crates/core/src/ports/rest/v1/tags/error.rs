use crate::{
    app::{command::error as command_error, query::error as query_error},
    ports::rest::response::ErrorResponse,
};
use http::StatusCode;

impl From<command_error::CreateTagError> for ErrorResponse {
    fn from(value: command_error::CreateTagError) -> Self {
        match value {
            command_error::CreateTagError::DuplicateSlug(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetTagOneError> for ErrorResponse {
    fn from(value: query_error::GetTagOneError) -> Self {
        match value {
            query_error::GetTagOneError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetTagManyError> for ErrorResponse {
    fn from(_value: query_error::GetTagManyError) -> Self {
        Self::new_default(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<command_error::DeleteTagError> for ErrorResponse {
    fn from(value: command_error::DeleteTagError) -> Self {
        match value {
            command_error::DeleteTagError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

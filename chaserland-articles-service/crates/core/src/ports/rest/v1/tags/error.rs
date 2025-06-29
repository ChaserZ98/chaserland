use crate::{app::error as app_error, ports::rest::response::ErrorResponse};
use http::StatusCode;

impl From<app_error::CreateTagError> for ErrorResponse {
    fn from(value: app_error::CreateTagError) -> Self {
        match value {
            app_error::CreateTagError::DuplicateSlug(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::GetTagOneError> for ErrorResponse {
    fn from(value: app_error::GetTagOneError) -> Self {
        match value {
            app_error::GetTagOneError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::GetTagManyError> for ErrorResponse {
    fn from(_value: app_error::GetTagManyError) -> Self {
        Self::new_default(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<app_error::DeleteTagError> for ErrorResponse {
    fn from(value: app_error::DeleteTagError) -> Self {
        match value {
            app_error::DeleteTagError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

use crate::{app::error as app_error, ports::rest::response::ErrorResponse};
use http::StatusCode;

impl From<app_error::CreateArticleError> for ErrorResponse {
    fn from(value: app_error::CreateArticleError) -> Self {
        match value {
            app_error::CreateArticleError::SeriesNotFound(_)
            | app_error::CreateArticleError::CategoryNotFound(_)
            | app_error::CreateArticleError::TagNotFound(_) => {
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

impl From<app_error::GetArticleContentError> for ErrorResponse {
    fn from(value: app_error::GetArticleContentError) -> Self {
        match value {
            app_error::GetArticleContentError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
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

impl From<app_error::DeleteArticleError> for ErrorResponse {
    fn from(value: app_error::DeleteArticleError) -> Self {
        match value {
            app_error::DeleteArticleError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::PublishArticleError> for ErrorResponse {
    fn from(value: app_error::PublishArticleError) -> Self {
        match value {
            app_error::PublishArticleError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            app_error::PublishArticleError::AlreadyPublished(_)
            | app_error::PublishArticleError::DataVersionConflict(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::UnpublishArticleError> for ErrorResponse {
    fn from(value: app_error::UnpublishArticleError) -> Self {
        match value {
            app_error::UnpublishArticleError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            app_error::UnpublishArticleError::NotPublished(_)
            | app_error::UnpublishArticleError::DataVersionConflict(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::SoftDeleteArticleError> for ErrorResponse {
    fn from(value: app_error::SoftDeleteArticleError) -> Self {
        match value {
            app_error::SoftDeleteArticleError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            app_error::SoftDeleteArticleError::AlreadySoftDeleted(_)
            | app_error::SoftDeleteArticleError::DataVersionConflict(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<app_error::RevokeSoftDeleteError> for ErrorResponse {
    fn from(value: app_error::RevokeSoftDeleteError) -> Self {
        match value {
            app_error::RevokeSoftDeleteError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            app_error::RevokeSoftDeleteError::NotSoftDeleted(_)
            | app_error::RevokeSoftDeleteError::DataVersionConflict(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

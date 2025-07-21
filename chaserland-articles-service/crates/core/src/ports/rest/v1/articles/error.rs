use crate::{
    app::{command::error as command_error, query::error as query_error},
    ports::rest::response::ErrorResponse,
};
use http::StatusCode;

impl From<command_error::CreateArticleError> for ErrorResponse {
    fn from(value: command_error::CreateArticleError) -> Self {
        match value {
            command_error::CreateArticleError::SeriesNotFound(_)
            | command_error::CreateArticleError::CategoryNotFound(_)
            | command_error::CreateArticleError::TagNotFound(_) => {
                Self::new(StatusCode::BAD_REQUEST, value.to_string())
            }
            command_error::CreateArticleError::DuplicateSlug(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            command_error::CreateArticleError::DataVersionConflict(message) => {
                Self::new(StatusCode::CONFLICT, message)
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetArticleOneError> for ErrorResponse {
    fn from(value: query_error::GetArticleOneError) -> Self {
        match value {
            query_error::GetArticleOneError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetArticleContentError> for ErrorResponse {
    fn from(value: query_error::GetArticleContentError) -> Self {
        match value {
            query_error::GetArticleContentError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<query_error::GetArticleManyError> for ErrorResponse {
    fn from(_value: query_error::GetArticleManyError) -> Self {
        Self::new_default(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<command_error::DeleteArticleError> for ErrorResponse {
    fn from(value: command_error::DeleteArticleError) -> Self {
        match value {
            command_error::DeleteArticleError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<command_error::UpdateArticleError> for ErrorResponse {
    fn from(value: command_error::UpdateArticleError) -> Self {
        match value {
            command_error::UpdateArticleError::ArticleNotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            command_error::UpdateArticleError::DuplicateSlug(_)
            | command_error::UpdateArticleError::SeriesNotFound(_)
            | command_error::UpdateArticleError::CategoryNotFound(_)
            | command_error::UpdateArticleError::TagNotFound(_) => {
                Self::new(StatusCode::BAD_REQUEST, value.to_string())
            }
            command_error::UpdateArticleError::DataVersionConflict(message) => {
                Self::new(StatusCode::CONFLICT, message)
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<command_error::PublishArticleError> for ErrorResponse {
    fn from(value: command_error::PublishArticleError) -> Self {
        match value {
            command_error::PublishArticleError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            command_error::PublishArticleError::AlreadyPublished(_)
            | command_error::PublishArticleError::DataVersionConflict(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<command_error::UnpublishArticleError> for ErrorResponse {
    fn from(value: command_error::UnpublishArticleError) -> Self {
        match value {
            command_error::UnpublishArticleError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            command_error::UnpublishArticleError::NotPublished(_)
            | command_error::UnpublishArticleError::DataVersionConflict(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<command_error::SoftDeleteArticleError> for ErrorResponse {
    fn from(value: command_error::SoftDeleteArticleError) -> Self {
        match value {
            command_error::SoftDeleteArticleError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            command_error::SoftDeleteArticleError::AlreadySoftDeleted(_)
            | command_error::SoftDeleteArticleError::DataVersionConflict(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<command_error::RevokeSoftDeleteError> for ErrorResponse {
    fn from(value: command_error::RevokeSoftDeleteError) -> Self {
        match value {
            command_error::RevokeSoftDeleteError::NotFound(_) => {
                Self::new(StatusCode::NOT_FOUND, value.to_string())
            }
            command_error::RevokeSoftDeleteError::NotSoftDeleted(_)
            | command_error::RevokeSoftDeleteError::DataVersionConflict(_) => {
                Self::new(StatusCode::CONFLICT, value.to_string())
            }
            _ => Self::new_default(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

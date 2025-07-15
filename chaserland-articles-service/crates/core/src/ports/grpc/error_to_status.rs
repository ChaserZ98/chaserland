use crate::app::{command::error as command_error, query::error as query_error};
use tonic::Status;

impl From<command_error::CreateArticleError> for Status {
    fn from(value: command_error::CreateArticleError) -> Self {
        match value {
            command_error::CreateArticleError::CategoryNotFound(_) => {
                Self::invalid_argument(value.to_string())
            }
            command_error::CreateArticleError::SeriesNotFound(_) => {
                Self::invalid_argument(value.to_string())
            }
            command_error::CreateArticleError::TagNotFound(_) => {
                Self::invalid_argument(value.to_string())
            }
            command_error::CreateArticleError::DuplicateSlug(_) => {
                Self::already_exists(value.to_string())
            }
            command_error::CreateArticleError::DataVersionConflict(message) => {
                Self::failed_precondition(message)
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::CreateSeriesError> for Status {
    fn from(value: command_error::CreateSeriesError) -> Self {
        match value {
            command_error::CreateSeriesError::DuplicateSlug(_) => {
                Self::already_exists(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::CreateCategoryError> for Status {
    fn from(value: command_error::CreateCategoryError) -> Self {
        match value {
            command_error::CreateCategoryError::DuplicateSlug(_) => {
                Self::already_exists(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::CreateTagError> for Status {
    fn from(value: command_error::CreateTagError) -> Self {
        match value {
            command_error::CreateTagError::DuplicateSlug(_) => {
                Self::already_exists(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<query_error::GetArticleOneError> for Status {
    fn from(value: query_error::GetArticleOneError) -> Self {
        match value {
            query_error::GetArticleOneError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<query_error::GetArticleContentError> for Status {
    fn from(value: query_error::GetArticleContentError) -> Self {
        match value {
            query_error::GetArticleContentError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<query_error::GetArticleManyError> for Status {
    fn from(value: query_error::GetArticleManyError) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<query_error::GetSeriesOneError> for Status {
    fn from(value: query_error::GetSeriesOneError) -> Self {
        match value {
            query_error::GetSeriesOneError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<query_error::GetSeriesManyError> for Status {
    fn from(value: query_error::GetSeriesManyError) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<query_error::GetCategoryOneError> for Status {
    fn from(value: query_error::GetCategoryOneError) -> Self {
        match value {
            query_error::GetCategoryOneError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<query_error::GetCategoryManyError> for Status {
    fn from(value: query_error::GetCategoryManyError) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<query_error::GetTagOneError> for Status {
    fn from(value: query_error::GetTagOneError) -> Self {
        match value {
            query_error::GetTagOneError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<query_error::GetTagManyError> for Status {
    fn from(value: query_error::GetTagManyError) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<command_error::PublishArticleError> for Status {
    fn from(value: command_error::PublishArticleError) -> Self {
        match value {
            command_error::PublishArticleError::NotFound(_) => Self::not_found(value.to_string()),
            command_error::PublishArticleError::AlreadyPublished(_)
            | command_error::PublishArticleError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::UnpublishArticleError> for Status {
    fn from(value: command_error::UnpublishArticleError) -> Self {
        match value {
            command_error::UnpublishArticleError::NotFound(_) => Self::not_found(value.to_string()),
            command_error::UnpublishArticleError::NotPublished(_)
            | command_error::UnpublishArticleError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::SoftDeleteArticleError> for Status {
    fn from(value: command_error::SoftDeleteArticleError) -> Self {
        match value {
            command_error::SoftDeleteArticleError::NotFound(_) => {
                Self::not_found(value.to_string())
            }
            command_error::SoftDeleteArticleError::AlreadySoftDeleted(_)
            | command_error::SoftDeleteArticleError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::RevokeSoftDeleteError> for Status {
    fn from(value: command_error::RevokeSoftDeleteError) -> Self {
        match value {
            command_error::RevokeSoftDeleteError::NotFound(_) => Self::not_found(value.to_string()),
            command_error::RevokeSoftDeleteError::NotSoftDeleted(_)
            | command_error::RevokeSoftDeleteError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::DeleteArticleError> for Status {
    fn from(value: command_error::DeleteArticleError) -> Self {
        match value {
            command_error::DeleteArticleError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::DeleteSeriesError> for Status {
    fn from(value: command_error::DeleteSeriesError) -> Self {
        match value {
            command_error::DeleteSeriesError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::DeleteCategoryError> for Status {
    fn from(value: command_error::DeleteCategoryError) -> Self {
        match value {
            command_error::DeleteCategoryError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<command_error::DeleteTagError> for Status {
    fn from(value: command_error::DeleteTagError) -> Self {
        match value {
            command_error::DeleteTagError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

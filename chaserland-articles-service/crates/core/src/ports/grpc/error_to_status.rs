use crate::app::error as app_error;
use tonic::Status;

impl From<app_error::CreateArticleError> for Status {
    fn from(value: app_error::CreateArticleError) -> Self {
        match value {
            app_error::CreateArticleError::CategoryNotFound(_) => {
                Self::invalid_argument(value.to_string())
            }
            app_error::CreateArticleError::SeriesNotFound(_) => {
                Self::invalid_argument(value.to_string())
            }
            app_error::CreateArticleError::TagNotFound(_) => {
                Self::invalid_argument(value.to_string())
            }
            app_error::CreateArticleError::DuplicateSlug(_) => {
                Self::already_exists(value.to_string())
            }
            app_error::CreateArticleError::DataVersionConflict(message) => {
                Self::failed_precondition(message)
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::CreateSeriesError> for Status {
    fn from(value: app_error::CreateSeriesError) -> Self {
        match value {
            app_error::CreateSeriesError::DuplicateSlug(_) => {
                Self::already_exists(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::CreateCategoryError> for Status {
    fn from(value: app_error::CreateCategoryError) -> Self {
        match value {
            app_error::CreateCategoryError::DuplicateSlug(_) => {
                Self::already_exists(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::CreateTagError> for Status {
    fn from(value: app_error::CreateTagError) -> Self {
        match value {
            app_error::CreateTagError::DuplicateSlug(_) => Self::already_exists(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::GetArticleOneError> for Status {
    fn from(value: app_error::GetArticleOneError) -> Self {
        match value {
            app_error::GetArticleOneError::NotFound(_) => Self::not_found(value.to_string()),
            app_error::GetArticleOneError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::GetArticleContentError> for Status {
    fn from(value: app_error::GetArticleContentError) -> Self {
        match value {
            app_error::GetArticleContentError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::GetArticleManyError> for Status {
    fn from(value: app_error::GetArticleManyError) -> Self {
        match value {
            app_error::GetArticleManyError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::GetSeriesOneError> for Status {
    fn from(value: app_error::GetSeriesOneError) -> Self {
        match value {
            app_error::GetSeriesOneError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::GetSeriesManyError> for Status {
    fn from(value: app_error::GetSeriesManyError) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<app_error::GetCategoryOneError> for Status {
    fn from(value: app_error::GetCategoryOneError) -> Self {
        match value {
            app_error::GetCategoryOneError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::GetCategoryManyError> for Status {
    fn from(value: app_error::GetCategoryManyError) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<app_error::GetTagOneError> for Status {
    fn from(value: app_error::GetTagOneError) -> Self {
        match value {
            app_error::GetTagOneError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::GetTagManyError> for Status {
    fn from(value: app_error::GetTagManyError) -> Self {
        Self::internal(value.to_string())
    }
}

impl From<app_error::PublishArticleError> for Status {
    fn from(value: app_error::PublishArticleError) -> Self {
        match value {
            app_error::PublishArticleError::NotFound(_) => Self::not_found(value.to_string()),
            app_error::PublishArticleError::AlreadyPublished(_)
            | app_error::PublishArticleError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::UnpublishArticleError> for Status {
    fn from(value: app_error::UnpublishArticleError) -> Self {
        match value {
            app_error::UnpublishArticleError::NotFound(_) => Self::not_found(value.to_string()),
            app_error::UnpublishArticleError::NotPublished(_)
            | app_error::UnpublishArticleError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::SoftDeleteArticleError> for Status {
    fn from(value: app_error::SoftDeleteArticleError) -> Self {
        match value {
            app_error::SoftDeleteArticleError::NotFound(_) => Self::not_found(value.to_string()),
            app_error::SoftDeleteArticleError::AlreadySoftDeleted(_)
            | app_error::SoftDeleteArticleError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::RevokeSoftDeleteError> for Status {
    fn from(value: app_error::RevokeSoftDeleteError) -> Self {
        match value {
            app_error::RevokeSoftDeleteError::NotFound(_) => Self::not_found(value.to_string()),
            app_error::RevokeSoftDeleteError::NotSoftDeleted(_)
            | app_error::RevokeSoftDeleteError::DataVersionConflict(_) => {
                Self::failed_precondition(value.to_string())
            }
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::DeleteArticleError> for Status {
    fn from(value: app_error::DeleteArticleError) -> Self {
        match value {
            app_error::DeleteArticleError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::DeleteSeriesError> for Status {
    fn from(value: app_error::DeleteSeriesError) -> Self {
        match value {
            app_error::DeleteSeriesError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::DeleteCategoryError> for Status {
    fn from(value: app_error::DeleteCategoryError) -> Self {
        match value {
            app_error::DeleteCategoryError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

impl From<app_error::DeleteTagError> for Status {
    fn from(value: app_error::DeleteTagError) -> Self {
        match value {
            app_error::DeleteTagError::NotFound(_) => Self::not_found(value.to_string()),
            _ => Self::internal(value.to_string()),
        }
    }
}

use crate::domain::entity::{article, category, series, tag};

#[derive(Debug, thiserror::Error)]
pub enum CreateArticleError {
    #[error("Article with slug {0} already exists")]
    DuplicateSlug(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SetSeriesError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error("Series with id {0} not found")]
    SeriesNotFound(series::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveSeriesError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AddCategoryError {
    #[error("Article with id {0} not found")]
    ArticleNotFound(article::Id),
    #[error("Category with id {0} not found")]
    CategoryNotFound(category::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveCategoryError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error("Category with id {0} not found")]
    CategoryNotFound(category::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AddTagError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error("Tag with id {0} not found")]
    TagNotFound(tag::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveTagError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error("Tag with id {0} not found")]
    TagNotFound(tag::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PublishArticleError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UnpublishArticleError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SoftDeleteArticleError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RevokeSoftDeleteError {
    #[error("Article with id {0} not found")]
    NotFound(article::Id),
    #[error(
        "Article with id {id} has mismatched version: current version {current_version} != database version {db_version}"
    )]
    VersionMismatch {
        id: article::Id,
        current_version: article::Version,
        db_version: article::Version,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteArticleError {
    #[error("Article with identifier {0} not found")]
    NotFound(article::Identifier),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

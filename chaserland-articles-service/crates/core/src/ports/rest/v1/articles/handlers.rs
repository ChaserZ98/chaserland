use super::{TAG, params, schema};
use crate::{
    app::{command::interface::ArticleCommandService, query::interface::ArticleQueryService},
    ports::rest::{
        response::{ErrorBody, ErrorResponse},
        state::AppState,
    },
};
use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response, Result},
};
use axum_extra::extract::Query;
use http::StatusCode;

#[utoipa::path(
    post,
    tag = TAG,
    summary = "Create article",
    description = "Create article",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    request_body(content = schema::ArticleCreate, description = "Request body for create article", content_type = "application/json"),
    responses(
        (
            status = 201,
            description = "Created",
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Already exists",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn create_article<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Json(payload): Json<schema::ArticleCreate>,
) -> Result<Response, ErrorResponse> {
    let command = payload.try_into()?;

    state
        .article_command_service
        .create_article(command)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create article: {e}");
            e
        })?;

    let res = StatusCode::CREATED.into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "Get article",
    description = "Get article by article identifier",
    path = "/{identifier}",
    params(
        ("accept" = Option<String>, Header, description = "Accept header"),
        params::GetArticleOneQuery
    ),
    responses(
        (
            status = 200,
            description = "OK",
            content((schema::Article = "application/json"))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn get_article_one<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
    Query(query): Query<params::GetArticleOneQuery>,
) -> Result<Response, ErrorResponse> {
    let query = (identifier, query).try_into()?;

    let article = state.article_query_service.get_article_one(query).await?;

    let res = Json(schema::Article::from(article)).into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "Get article content",
    description = "Get article content by article identifier",
    path = "/{identifier}/content",
    responses(
        (
            status = 200,
            description = "OK",
            content((String = "text/plain"))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn get_article_content<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
    Query(query): Query<params::GetArticleContentQuery>,
) -> Result<Response, ErrorResponse> {
    let query = (identifier, query).try_into()?;

    let content = state
        .article_query_service
        .get_article_content(query)
        .await?;

    let res = content.into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "List articles",
    description = "List articles",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header"),
        params::GetArticleManyQuery
    ),
    responses(
        (
            status = 200,
            description = "OK",
            content((Vec<schema::Article> = "application/json"))
        )
    )
)]
pub async fn get_article_many<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Query(query): Query<params::GetArticleManyQuery>,
) -> Result<Response, ErrorResponse> {
    let query = query.try_into()?;

    let articles = state.article_query_service.get_article_many(query).await?;

    let res = Json(
        articles
            .into_iter()
            .map(schema::Article::from)
            .collect::<Vec<_>>(),
    )
    .into_response();

    Ok(res)
}

#[utoipa::path(
    delete,
    tag = TAG,
    summary = "Delete article",
    description = "Delete article by article identifier",
    path = "/{identifier}",
    responses(
        (
            status = StatusCode::OK,
            description = "OK",
            content((()))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn delete_article<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state
        .article_command_service
        .delete_article(command)
        .await?;

    Ok(().into_response())
}

#[utoipa::path(
    put,
    tag = TAG,
    summary = "Publish article",
    description = "Publish article by article identifier",
    path = "/{identifier}/publish",
    responses(
        (
            status = StatusCode::OK,
            description = "OK",
            content((()))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Already published or database state changed during request",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn publish_article<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state
        .article_command_service
        .publish_article(command)
        .await?;

    Ok(().into_response())
}

#[utoipa::path(
    delete,
    tag = TAG,
    summary = "Unpublish article",
    description = "Unpublish article by article identifier",
    path = "/{identifier}/publish",
    responses(
        (
            status = StatusCode::OK,
            description = "OK",
            content((()))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Not published or database state changed during request",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn unpublish_article<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state
        .article_command_service
        .unpublish_article(command)
        .await?;

    Ok(().into_response())
}

#[utoipa::path(
    put,
    tag = TAG,
    summary = "Soft delete article",
    description = "Soft delete article by article identifier",
    path = "/{identifier}/soft_delete",
    responses(
        (
            status = StatusCode::OK,
            description = "OK",
            content((()))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Already soft-deleted or database state changed during request",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn soft_delete_article<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state
        .article_command_service
        .soft_delete_article(command)
        .await?;

    Ok(().into_response())
}

#[utoipa::path(
    delete,
    tag = TAG,
    summary = "Revoke soft delete article",
    description = "Revoke soft delete article by article identifier",
    path = "/{identifier}/soft_delete",
    responses(
        (
            status = StatusCode::OK,
            description = "OK",
            content((()))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Not soft-deleted or database state changed during request",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn revoke_soft_delete_article<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state
        .article_command_service
        .revoke_soft_delete_article(command)
        .await?;

    Ok(().into_response())
}

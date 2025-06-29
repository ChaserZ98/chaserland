use super::{TAG, params, schema};
use crate::{
    app::interface::ArticleService,
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

#[utoipa::path(
    post,
    tag = TAG,
    summary = "Create tag",
    description = "Create tag",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    request_body(content = schema::TagCreate, description = "Request body for create tag", content_type = "application/json"),
    responses(
        (
            status = 201,
            description = "Created",
            content((schema::Tag = "application/json"))
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Already exists",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn create_tag<T: ArticleService>(
    State(state): State<AppState<T>>,
    Json(payload): Json<schema::TagCreate>,
) -> Result<Response, ErrorResponse> {
    let command = payload.try_into()?;

    let tag = state
        .article_service
        .create_tag(command)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create tag: {e}");
            e
        })?;

    let res = Json(schema::Tag::from(tag)).into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "Get tag",
    description = "Get tag by tag identifier",
    path = "/{identifier}",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    responses(
        (
            status = StatusCode::OK,
            description = "Ok",
            content((schema::Tag = "application/json"))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn get_tag_one<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let query = identifier.try_into()?;

    let tag = state.article_service.get_tag_one(query).await?;

    let res = Json(schema::Tag::from(tag)).into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "List tag",
    description = "List tag",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header"),
        params::GetTagManyQuery
    ),
    responses(
        (
            status = 200,
            description = "OK",
            content((Vec<schema::Tag> = "application/json"))
        )
    )
)]
pub async fn get_tag_many<T: ArticleService>(
    State(state): State<AppState<T>>,
    Query(query): Query<params::GetTagManyQuery>,
) -> Result<Response, ErrorResponse> {
    let query = query.try_into()?;

    let tag = state.article_service.get_tag_many(query).await?;

    let res = Json(tag.into_iter().map(schema::Tag::from).collect::<Vec<_>>()).into_response();

    Ok(res)
}

#[utoipa::path(
    delete,
    tag = TAG,
    summary = "Delete tag",
    description = "Delete tag by tag identifier",
    path = "/{identifier}",
    responses(
        (
            status = StatusCode::OK,
            description = "Ok"
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn delete_tag<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state.article_service.delete_tag(command).await?;

    Ok(().into_response())
}

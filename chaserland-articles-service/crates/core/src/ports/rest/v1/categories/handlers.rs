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
    summary = "Create category",
    description = "Create category",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    request_body(content = schema::CategoryCreate, description = "Request body for create category", content_type = "application/json"),
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
pub async fn create_category<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Json(payload): Json<schema::CategoryCreate>,
) -> Result<Response, ErrorResponse> {
    let command = payload.try_into()?;

    state
        .article_command_service
        .create_category(command)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create category: {e}");
            e
        })?;

    let res = StatusCode::CREATED.into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "Get category",
    description = "Get category by category identifier",
    path = "/{identifier}",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    responses(
        (
            status = StatusCode::OK,
            description = "Ok",
            content((schema::Category = "application/json"))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn get_category_one<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let query = identifier.try_into()?;

    let category = state.article_query_service.get_category_one(query).await?;

    let res = Json(schema::Category::from(category)).into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "List categories",
    description = "List categories",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header"),
        params::GetCategoryManyQuery
    ),
    responses(
        (
            status = 200,
            description = "OK",
            content((Vec<schema::Category> = "application/json"))
        )
    )
)]
pub async fn get_category_many<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Query(query): Query<params::GetCategoryManyQuery>,
) -> Result<Response, ErrorResponse> {
    let query = query.try_into()?;

    let category = state.article_query_service.get_category_many(query).await?;

    let res = Json(
        category
            .into_iter()
            .map(schema::Category::from)
            .collect::<Vec<_>>(),
    )
    .into_response();

    Ok(res)
}

#[utoipa::path(
    delete,
    tag = TAG,
    summary = "Delete category",
    description = "Delete category by category identifier",
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
pub async fn delete_category<C: ArticleCommandService, Q: ArticleQueryService>(
    State(state): State<AppState<C, Q>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state
        .article_command_service
        .delete_category(command)
        .await?;

    Ok(().into_response())
}

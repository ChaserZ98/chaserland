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
            content((schema::Category = "application/json"))
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Already exists",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn create_category<T: ArticleService>(
    State(state): State<AppState<T>>,
    Json(payload): Json<schema::CategoryCreate>,
) -> Result<Response, ErrorResponse> {
    let command = payload.try_into()?;

    let category = state
        .article_service
        .create_category(command)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create category: {e}");
            e
        })?;

    let res = Json(schema::Category::from(category)).into_response();

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
pub async fn get_category_one<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let query = identifier.try_into()?;

    let category = state.article_service.get_category_one(query).await?;

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
pub async fn get_category_many<T: ArticleService>(
    State(state): State<AppState<T>>,
    Query(query): Query<params::GetCategoryManyQuery>,
) -> Result<Response, ErrorResponse> {
    let query = query.try_into()?;

    let category = state.article_service.get_category_many(query).await?;

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
pub async fn delete_category<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state.article_service.delete_category(command).await?;

    Ok(().into_response())
}

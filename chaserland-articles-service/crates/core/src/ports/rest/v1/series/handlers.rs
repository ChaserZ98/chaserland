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
    summary = "Create series",
    description = "Create series",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    request_body(content = schema::SeriesCreate, description = "Request body for create series", content_type = "application/json"),
    responses(
        (
            status = 201,
            description = "Created",
            content((schema::Series = "application/json"))
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Already exists",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn create_series<T: ArticleService>(
    State(state): State<AppState<T>>,
    Json(payload): Json<schema::SeriesCreate>,
) -> Result<Response, ErrorResponse> {
    let command = payload.try_into()?;

    let series = state
        .article_service
        .create_series(command)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create series: {e}");
            e
        })?;

    let res = Json(schema::Series::from(series)).into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "Get series",
    description = "Get series by series identifier",
    path = "/{identifier}",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    responses(
        (
            status = StatusCode::OK,
            description = "Ok",
            content((schema::Series = "application/json"))
        ),
        (
            status = StatusCode::NOT_FOUND,
            description = "Not found",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn get_series_one<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let query = identifier.try_into()?;

    let series = state.article_service.get_series_one(query).await?;

    let res = Json(schema::Series::from(series)).into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "List series",
    description = "List series",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header"),
        params::GetSeriesManyQuery
    ),
    responses(
        (
            status = 200,
            description = "OK",
            content((Vec<schema::Series> = "application/json"))
        )
    )
)]
pub async fn get_series_many<T: ArticleService>(
    State(state): State<AppState<T>>,
    Query(query): Query<params::GetSeriesManyQuery>,
) -> Result<Response, ErrorResponse> {
    let query = query.try_into()?;

    let series = state.article_service.get_series_many(query).await?;

    let res = Json(
        series
            .into_iter()
            .map(schema::Series::from)
            .collect::<Vec<_>>(),
    )
    .into_response();

    Ok(res)
}

#[utoipa::path(
    delete,
    tag = TAG,
    summary = "Delete series",
    description = "Delete series by series identifier",
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
pub async fn delete_series<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(identifier): Path<String>,
) -> Result<Response, ErrorResponse> {
    let command = identifier.try_into()?;

    state.article_service.delete_series(command).await?;

    Ok(().into_response())
}

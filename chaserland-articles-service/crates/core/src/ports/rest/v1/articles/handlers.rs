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
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    tag = TAG,
    request_body(content = schema::ArticleCreate, description = "Request body for create article", content_type = "application/json"),
    responses(
        (
            status = 201,
            description = "Created",
            content((schema::Article = "application/json"))
        ),
        (
            status = StatusCode::CONFLICT,
            description = "Already exists",
            content((ErrorBody = "application/json"))
        )
    )
)]
pub async fn create_article<T: ArticleService>(
    State(state): State<AppState<T>>,
    Json(payload): Json<schema::ArticleCreate>,
) -> Result<Response, ErrorResponse> {
    let command = payload.try_into()?;

    let article = state
        .article_service
        .create_article(command)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create article: {e}");
            e
        })?;

    let res = Json(schema::Article::from(article)).into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    path = "/{identifier}",
    tag = TAG,
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
pub async fn get_article_one<T: ArticleService>(
    State(state): State<AppState<T>>,
    Path(identifier): Path<String>,
    Query(query): Query<params::GetArticleOneQuery>,
) -> Result<Response, ErrorResponse> {
    let query = (identifier, query).try_into()?;

    let article = state.article_service.get_article_one(query).await?;

    let res = Json(schema::Article::from(article)).into_response();

    Ok(res)
}

#[utoipa::path(
    get,
    path = "",
    tag = TAG,
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
pub async fn get_article_many<T: ArticleService>(
    State(state): State<AppState<T>>,
    Query(query): Query<params::GetArticleManyQuery>,
) -> Result<Response, ErrorResponse> {
    let query = query.try_into()?;

    let articles = state.article_service.get_article_many(query).await?;

    let res = Json(
        articles
            .into_iter()
            .map(schema::Article::from)
            .collect::<Vec<_>>(),
    )
    .into_response();

    Ok(res)
}

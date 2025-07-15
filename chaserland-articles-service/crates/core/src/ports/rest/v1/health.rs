use crate::{
    app::{command::interface::ArticleCommandService, query::interface::ArticleQueryService},
    ports::rest::{response::ErrorResponse, state::AppState},
};
use axum::{
    Json,
    response::{IntoResponse, Response, Result},
};
use http::{HeaderMap, StatusCode, header::ACCEPT};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

pub static TAG: &str = "Health";

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Health {
    status: String,
}

impl Health {
    pub fn text() -> String {
        String::from("OK")
    }

    pub fn json() -> Self {
        Self::default()
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            status: String::from("OK"),
        }
    }
}

#[utoipa::path(
    get,
    tag = TAG,
    summary = "Get health",
    description = "Get health",
    path = "",
    params(
        ("accept" = Option<String>, Header, description = "Accept header")
    ),
    responses(
        (
            status = StatusCode::OK,
            description = "OK",
            content(
                (Health = "application/json"),
                (String = "text/plain")
            )
        ),
        (
            status = StatusCode::BAD_REQUEST,
            description = "Unsupported Accept header",
            content(
                (String = "text/plain")
            )
        )
    )
)]
pub async fn get_health(headers: HeaderMap) -> Result<Response, ErrorResponse> {
    match headers.get(ACCEPT).map(|x| x.as_bytes()) {
        Some(b"application/json") | Some(b"*/*") | None => {
            Ok(Json(Health::default()).into_response())
        }
        Some(b"text/plain") => Ok(Health::text().into_response()),
        _ => Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Unsupported Accept header".into(),
        )),
    }
}

pub fn router<C, Q>() -> OpenApiRouter<AppState<C, Q>>
where
    C: ArticleCommandService,
    Q: ArticleQueryService,
{
    OpenApiRouter::new().routes(routes!(get_health))
}

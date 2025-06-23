use crate::{app::interface::ArticleService, ports::rest::state::AppState};
use axum::{
    Json,
    response::{IntoResponse, Response},
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
    path = "",
    tag = TAG,
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
pub async fn get_health(headers: HeaderMap) -> Response {
    match headers.get(ACCEPT).map(|x| x.as_bytes()) {
        Some(b"application/json") | Some(b"*/*") | None => Json(Health::default()).into_response(),
        Some(b"text/plain") => Health::text().into_response(),
        _ => (StatusCode::BAD_REQUEST, "Unsupported Accept header").into_response(),
    }
}

pub fn router<T>() -> OpenApiRouter<AppState<T>>
where
    T: ArticleService,
{
    OpenApiRouter::new().routes(routes!(get_health))
}

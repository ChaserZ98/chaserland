pub mod error;
pub mod handlers;
pub mod params;
pub mod schema;

use crate::{
    app::{command::interface::ArticleCommandService, query::interface::ArticleQueryService},
    ports::rest::state::AppState,
};
use utoipa_axum::{router::OpenApiRouter, routes};

pub static TAG: &str = "Series";

pub fn router<C: ArticleCommandService, Q: ArticleQueryService>() -> OpenApiRouter<AppState<C, Q>> {
    let router = OpenApiRouter::new()
        .routes(routes!(handlers::create_series, handlers::get_series_many))
        .routes(routes!(handlers::get_series_one, handlers::delete_series));
    router
}

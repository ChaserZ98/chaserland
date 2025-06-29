pub mod error;
pub mod handlers;
pub mod params;
pub mod schema;

use crate::{app::interface::ArticleService, ports::rest::state::AppState};
use utoipa_axum::{router::OpenApiRouter, routes};

pub static TAG: &str = "Categories";

pub fn router<T: ArticleService>() -> OpenApiRouter<AppState<T>> {
    let router = OpenApiRouter::new()
        .routes(routes!(
            handlers::create_category,
            handlers::get_category_many
        ))
        .routes(routes!(
            handlers::get_category_one,
            handlers::delete_category
        ));
    router
}

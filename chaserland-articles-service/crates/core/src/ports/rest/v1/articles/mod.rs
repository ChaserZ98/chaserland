pub mod error;
pub mod handlers;
pub mod params;
pub mod schema;

use crate::{
    app::{command::interface::ArticleCommandService, query::interface::ArticleQueryService},
    ports::rest::state::AppState,
};
use utoipa_axum::{router::OpenApiRouter, routes};

pub static TAG: &str = "Articles";

pub fn router<C: ArticleCommandService, Q: ArticleQueryService>() -> OpenApiRouter<AppState<C, Q>> {
    let router = OpenApiRouter::new()
        .routes(routes!(
            handlers::create_article,
            handlers::get_article_many,
        ))
        .routes(routes!(handlers::get_article_one, handlers::delete_article))
        .routes(routes!(handlers::get_article_content))
        .routes(routes!(
            handlers::publish_article,
            handlers::unpublish_article
        ))
        .routes(routes!(
            handlers::soft_delete_article,
            handlers::revoke_soft_delete_article
        ));
    router
}

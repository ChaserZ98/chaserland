use super::health;
use crate::{
    app::{command::interface::ArticleCommandService, query::interface::ArticleQueryService},
    ports::rest::{
        state::AppState,
        v1::{articles, categories, series, tags},
    },
};
use utoipa_axum::router::OpenApiRouter;

pub fn router<C, Q>() -> OpenApiRouter<AppState<C, Q>>
where
    C: ArticleCommandService,
    Q: ArticleQueryService,
{
    let router = OpenApiRouter::new()
        .nest("/health", health::router())
        .nest("/articles", articles::router())
        .nest("/series", series::router())
        .nest("/categories", categories::router())
        .nest("/tags", tags::router());
    router
}

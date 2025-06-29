use super::health;
use crate::{
    app::interface::ArticleService,
    ports::rest::{
        state::AppState,
        v1::{articles, categories, series, tags},
    },
};
use utoipa_axum::router::OpenApiRouter;

pub fn router<T>() -> OpenApiRouter<AppState<T>>
where
    T: ArticleService,
{
    let router = OpenApiRouter::new()
        .nest("/health", health::router())
        .nest("/articles", articles::router())
        .nest("/series", series::router())
        .nest("/categories", categories::router())
        .nest("/tags", tags::router());
    router
}

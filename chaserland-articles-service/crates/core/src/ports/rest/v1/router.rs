use super::health;
use crate::{
    app::interface::ArticleService,
    ports::rest::{state::AppState, v1::articles},
};
use utoipa_axum::router::OpenApiRouter;

pub fn router<T>() -> OpenApiRouter<AppState<T>>
where
    T: ArticleService,
{
    let router = OpenApiRouter::new()
        .nest("/health", health::router())
        .nest("/articles", articles::router());
    router
}

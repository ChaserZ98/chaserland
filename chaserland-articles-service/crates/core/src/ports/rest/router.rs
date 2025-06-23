use super::v1;
use crate::{app::interface::ArticleService, ports::rest::state::AppState};
use axum::Router;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKeyValue, SecurityScheme},
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

pub fn router<T>() -> Router<AppState<T>>
where
    T: ArticleService,
{
    let v1_router = v1::router();
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/v1", v1_router)
        .split_for_parts();
    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api));
    router
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(
        title = "chaserland article service",
        description = "my description",
        version = "0.2.0",
        license(name = "Apache 2.0", url = "https://www.apache.org/licenses/LICENSE-2.0")
    ),
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Header(
                    ApiKeyValue::new("api_key"),
                )),
            );
        }
    }
}

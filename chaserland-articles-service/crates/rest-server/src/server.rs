use super::{MetricHandler, Metrics, ServerConfig};
use anyhow::Result;
use axum::http::{self, Request};
use chaserland_articles_service_core::{
    app::{command::service::ArticleCommandService, query::service::ArticleQueryService},
    db::connect_db,
    infra::postgres::{
        query_handler::{
            PgArticleQueryHandler, PgCategoryQueryHandler, PgSeriesQueryHandler, PgTagQueryHandler,
        },
        repository::{
            PgArticleRepository, PgCategoryRepository, PgSeriesRepository, PgTagRepository,
        },
    },
    ports::rest::{router::router, state::AppState},
};
use chaserland_observability::Observability;
use opentelemetry::{
    global,
    trace::{SpanKind, Status},
};
use opentelemetry_http::HeaderExtractor;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::{
    DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::Level;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Default)]
pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config(&mut self, config: ServerConfig) {
        self.config = config
    }

    pub async fn run(&self) -> Result<()> {
        let otel_provider = Observability::default()
            .with_otel_config(self.config.otel_config.clone())
            .init()
            .map_err(|e| {
                tracing::error!("Failed to initialize observability: {}", e);
                e
            })?;

        let pool = connect_db(self.config.db_config.clone())
            .await
            .map_err(|e| {
                tracing::error!("Failed to create db connection pool: {}", e);
                e
            })?;

        let article_repository = PgArticleRepository::new();
        let series_repository = PgSeriesRepository::new();
        let category_repository = PgCategoryRepository::new();
        let tag_repository = PgTagRepository::new();

        let article_command_service = ArticleCommandService::new(
            article_repository,
            series_repository,
            category_repository,
            tag_repository,
            pool.clone(),
        );

        let article_query_handler = PgArticleQueryHandler::new(pool.clone());
        let series_query_handler = PgSeriesQueryHandler::new(pool.clone());
        let category_query_handler = PgCategoryQueryHandler::new(pool.clone());
        let tag_query_handler = PgTagQueryHandler::new(pool.clone());

        let article_query_service = ArticleQueryService::new(
            article_query_handler,
            series_query_handler,
            category_query_handler,
            tag_query_handler,
        );

        let addr = format!("{}:{}", self.config.host, self.config.port);
        tracing::info!("Server binding address set to {}", addr);

        let listener = TcpListener::bind(addr).await.map_err(|e| {
            tracing::error!("Failed to bind to address: {}", e);
            e
        })?;

        let service_name = self.config.otel_config.service_name.clone();
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(move |req: &Request<_>| {
                let parent_context = global::get_text_map_propagator(|propagator| {
                    propagator.extract(&HeaderExtractor(req.headers()))
                });
                let path = req.uri().path();
                let method = req.method().as_str();
                let span = tracing::info_span!(
                    "request",
                    "otel.kind" = ?SpanKind::Server,
                    "otel.name" = service_name,
                    "otel.status_code" = ?Status::Ok,
                    "request.path" = path,
                    "request.method" = method
                );
                span.set_parent(parent_context);
                span
            })
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(tower_http::LatencyUnit::Micros),
            )
            .on_failure(DefaultOnFailure::new().level(Level::ERROR));

        let metric_handler = MetricHandler::new(Metrics::new());
        let metric_layer = tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(metric_handler.clone())
            .on_request(metric_handler.clone())
            .on_response(metric_handler.clone())
            .on_failure(metric_handler)
            .on_eos(DefaultOnEos::new().level(Level::TRACE));

        let cors_layer = tower_http::cors::CorsLayer::new()
            .allow_credentials(true)
            .allow_methods([http::Method::POST]);

        let middlewares = ServiceBuilder::new().layer(trace_layer).layer(metric_layer);

        let state = AppState::new(article_command_service, article_query_service);

        let router = router().with_state(state).layer(middlewares);

        let shutdown_future = async {
            if let Err(e) = tokio::signal::ctrl_c().await {
                tracing::error!("Error while listening for shutdown signal: {}", e);
                return;
            }
            tracing::info!("Shutdown signal received. Starting graceful shutdown...");
        };

        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(shutdown_future)
            .await?;

        otel_provider.shutdown_all()?;

        Ok(())
    }
}

impl From<ServerConfig> for Server {
    fn from(config: ServerConfig) -> Self {
        Self { config }
    }
}

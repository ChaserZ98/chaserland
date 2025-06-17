use super::ServerConfig;
use crate::metrics::{MetricHandler, Metrics};
use anyhow::{Result, anyhow};
use chaserland_articles_service_core::{
    app::service::ArticleService,
    db::connect_db,
    infra::repository::postgres::{
        article::PgArticleRepository, category::PgCategoryRepository, series::PgSeriesRepository,
        tag::PgTagRepository,
    },
    ports::grpc::service::GrpcArticleService,
};
use chaserland_observability::Observability;
use chaserland_protos::article::v1::article_service_server::ArticleServiceServer;
use opentelemetry::{
    global,
    trace::{SpanKind, Status},
};
use opentelemetry_http::HeaderExtractor;
use tonic::transport::Server as TonicServer;
use tonic_health::server::health_reporter;
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
        self.config = config;
    }

    pub async fn run(&self) -> Result<()> {
        let otel_provider = Observability::default()
            .with_otel_config(self.config.otel_config.clone())
            .init()
            .map_err(|e| {
                tracing::error!("Failed to initialize observability: {}", e);
                anyhow!("Failed to initialize observability: {}", e)
            })?;

        let pool = connect_db(self.config.db_config.clone())
            .await
            .map_err(|e| {
                tracing::error!("Failed to create db connection pool: {}", e);
                e
            })?;

        let article_repository = PgArticleRepository::new(pool.clone());
        let series_repository = PgSeriesRepository::new(pool.clone());
        let category_repository = PgCategoryRepository::new(pool.clone());
        let tag_repository = PgTagRepository::new(pool.clone());

        let article_service = GrpcArticleService::new(ArticleService::new(
            article_repository,
            series_repository,
            category_repository,
            tag_repository,
        ))
        .into_tonic_service();

        let (health_reporter, health_service) = health_reporter();

        health_reporter
            .set_serving::<ArticleServiceServer<
                GrpcArticleService<
                    PgArticleRepository,
                    PgSeriesRepository,
                    PgCategoryRepository,
                    PgTagRepository,
                >,
            >>()
            .await;

        let addr = format!("{}:{}", self.config.host, self.config.port).parse()?;
        tracing::info!("Server binding address set to {}", addr);

        let service_name = self.config.otel_config.service_name.clone();
        let trace_layer = TraceLayer::new_for_grpc()
            .make_span_with(move |req: &http::Request<_>| {
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
        let metric_layer = tower_http::trace::TraceLayer::new_for_grpc()
            .make_span_with(metric_handler.clone())
            .on_request(metric_handler.clone())
            .on_response(metric_handler.clone())
            .on_failure(metric_handler)
            .on_eos(DefaultOnEos::new().level(Level::TRACE));

        TonicServer::builder()
            .layer(trace_layer)
            .layer(metric_layer)
            .add_service(health_service)
            .add_service(article_service)
            .serve_with_shutdown(addr, self.shutdown())
            .await?;

        tracing::info!("Server stopped.");

        tracing::info!("Starting resource cleanup...");

        tracing::info!("Closing db connection pool...");
        pool.close().await;

        tracing::info!("Resource cleanup completed.");

        tracing::info!("Shutting down observability...");
        otel_provider.shutdown_all()?;

        Ok(())
    }

    async fn shutdown(&self) {
        tracing::info!("Listening for shutdown signal...");
        if let Err(why) = tokio::signal::ctrl_c().await {
            tracing::error!("Error while shutting down: {}", why);
            return;
        }
        tracing::info!("Shutdown signal received. Starting graceful shutdown...");
    }
}

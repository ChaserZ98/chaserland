use crate::bootstrap::{ServerConfig, Service, ServiceClient, State};
use anyhow::Result;
use axum::{
    Router,
    extract::{MatchedPath, Request},
    routing::get,
};
use chaserland_observability::{Observability, util};
use chaserland_protos::article::v1::article_service_client::ArticleServiceClient;
use opentelemetry::{
    global,
    trace::{SpanKind, Status},
};
use opentelemetry_http::{HeaderExtractor, HeaderInjector, Response};
use std::time::Duration;
use tokio::net::TcpListener;
use tonic::{body::Body, transport::Endpoint};
use tower::{BoxError, ServiceBuilder, ServiceExt, util::BoxService};
use tower_http::{
    LatencyUnit,
    classify::GrpcFailureClass,
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
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

    pub async fn run(&self) -> Result<()> {
        let otel_provider = Observability::default()
            .with_otel_config(self.config.otel_config.clone())
            .init()
            .map_err(|e| {
                tracing::error!("Failed to initialize observability: {}", e);
                anyhow::anyhow!("Failed to initialize observability: {}", e)
            })?;

        let service_name = self.config.otel_config.service_name.clone();
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(move |req: &Request<_>| {
                let parent_context = global::get_text_map_propagator(|propagator| {
                    propagator.extract(&HeaderExtractor(req.headers()))
                });

                let path = req
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
                let method = req.method().as_str();

                let span = tracing::info_span!(
                    "request",
                    "otel.kind" = ?SpanKind::Server,
                    "otel.name" = service_name,
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
                    .latency_unit(LatencyUnit::Micros),
            )
            .on_failure(DefaultOnFailure::new().level(Level::ERROR));

        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(trace_layer);

        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(addr).await.map_err(|why| {
            tracing::error!("Failed to bind to address: {}", why);
            anyhow::anyhow!("Failed to bind to address: {}", why)
        })?;

        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown())
            .await
            .map_err(|why| {
                tracing::error!("Failed to start server: {}", why);
                anyhow::anyhow!("Failed to start server: {}", why)
            })?;

        tracing::info!("Server stopped.");

        tracing::info!("Shutting down observability...");
        otel_provider.shutdown_all()?;

        Ok(())
    }

    async fn shutdown() {
        tracing::info!("Listening for shutdown signal...");
        if let Err(why) = tokio::signal::ctrl_c().await {
            tracing::error!("Error while shutting down: {}", why);
            return;
        }
        tracing::info!("Shutdown signal received. Starting graceful shutdown...");
    }
}

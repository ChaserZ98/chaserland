use anyhow::Result;
use axum::{body::HttpBody, extract::Request, http, response::Response};
use std::time::Duration;
// use chaserland_gateway::bootstrap::Server;
use chaserland_observability::{Observability, OtelConfig};
use chaserland_protos::article::v1::{
    GetSeriesManyRequest, article_service_client::ArticleServiceClient,
};
use opentelemetry::{
    global,
    trace::{SpanKind, Status},
};
use opentelemetry_http::HeaderInjector;
use tonic::{client::GrpcService, transport::Endpoint};
use tower::{Layer, Service, ServiceBuilder, ServiceExt};
use tower_http::{classify::GrpcFailureClass, trace::TraceLayer};
use tracing::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut otel_config = OtelConfig::default()
        .with_trace()
        .with_logging()
        .with_metric();

    otel_config.service_name = "chaserland gateway".into();

    let otel_provider = Observability::default()
        .with_otel_config(otel_config)
        .init()
        .map_err(|e| {
            tracing::error!("Failed to initialize observability: {}", e);
            anyhow::anyhow!("Failed to initialize observability: {}", e)
        })?;

    let span = tracing::info_span!("main", "otel.kind" = ?SpanKind::Server, "otel.name" = "main", "otel.status_code" = ?Status::Ok);

    test().instrument(span).await?;

    // let server = Server::new();
    // server.run().await?;

    otel_provider.shutdown_all()?;

    Ok(())
}

async fn test() -> Result<()> {
    let trace_layer = TraceLayer::new_for_grpc()
    .make_span_with(|req: &Request<_>| {
        let parent_span = tracing::Span::current();
        let parent_context = parent_span.context();

        let path = req.uri().path();
        let method = req.method().as_str();
        let span = tracing::info_span!("chaserland article service client", "otel.kind" = ?SpanKind::Client, "otel.name" = "chaserland article service client", "request.path" = path, "request.method" = method);
        span.set_parent(parent_context);
        span
    })
    .on_request(|_: &Request<_>, _: &tracing::Span| {
        tracing::info!("Sending request to article service");
    })
    .on_response(|_: &Response<_>, _: Duration, span: &tracing::Span| {
        tracing::info!("Received response from article service");
        span.set_status(Status::Ok);
    })
    .on_failure(|class: GrpcFailureClass, _: Duration, span: &tracing::Span| {
        tracing::error!("Request to article service failed: {}", class);
        span.set_status(Status::Error { description: class.to_string().into() });
    });

    let article_service_channel = ServiceBuilder::new()
        .layer(trace_layer)
        .map_request(|mut req: Request<_>| {
            let cx = &tracing::Span::current().context();
            global::get_text_map_propagator(|propogator| {
                propogator.inject_context(cx, &mut HeaderInjector(req.headers_mut()))
            });
            req
        })
        .service(Endpoint::from_static("http://[::1]:8080").connect().await?)
        .boxed();

    tracing::info!("Connected to article service");

    let mut article_service_client = ArticleServiceClient::new(article_service_channel);

    let _ = article_service_client
        .get_series_many(tonic::Request::new(GetSeriesManyRequest {}))
        .await;

    Ok(())
}

#[derive(Default, Debug, Clone)]
pub struct GrpcTraceLayer;

impl<S> Layer<S> for GrpcTraceLayer {
    type Service = GrpcTraceService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        GrpcTraceService { inner }
    }
}

#[derive(Debug, Clone)]
pub struct GrpcTraceService<S> {
    inner: S,
}

impl<S, B, B2> Service<http::Request<B>> for GrpcTraceService<S>
where
    S: GrpcService<B, ResponseBody = B2> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: std::error::Error + 'static,
    B: Send + 'static,
    B2: HttpBody,
{
    type Response = Response<B2>;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        let mut req = req;
        let span = tracing::Span::current();
        let cx = span.context();

        global::get_text_map_propagator(|propogator| {
            propogator.inject_context(&cx, &mut HeaderInjector(req.headers_mut()))
        });

        self.inner.call(req)
    }
}

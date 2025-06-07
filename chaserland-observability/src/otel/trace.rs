use super::config::TraceConfig;
use anyhow::Result;
use opentelemetry::global;
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    trace::{RandomIdGenerator, SdkTracerProvider},
};

pub fn init_tracer_provider<T>(trace_config: T, resource: Resource) -> Result<SdkTracerProvider>
where
    T: AsRef<TraceConfig>,
{
    let trace_config = trace_config.as_ref();
    let endpoint = trace_config.endpoint.clone().unwrap();

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource)
        .build();
    global::set_tracer_provider(provider.clone());

    Ok(provider)
}

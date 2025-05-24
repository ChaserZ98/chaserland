use super::config::TraceConfig;
use crate::util::Environment;
use anyhow::Result;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    trace::{RandomIdGenerator, SdkTracerProvider},
};
use opentelemetry_semantic_conventions::resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION};

pub fn init_tracer_provider<T>(trace_config: T) -> Result<SdkTracerProvider>
where
    T: AsRef<TraceConfig>,
{
    let trace_config = trace_config.as_ref();
    let endpoint = trace_config.endpoint.clone().unwrap();
    let service_name = trace_config.service_name.clone().unwrap();
    let service_version = trace_config.service_version.clone().unwrap();

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;

    let environment_name = String::from(Environment::from_env());

    let resource = Resource::builder_empty()
        .with_service_name(service_name)
        .with_attributes(vec![
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, environment_name),
            KeyValue::new(SERVICE_VERSION, service_version),
        ])
        .build();

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource)
        .build();
    global::set_tracer_provider(provider.clone());

    Ok(provider)
}

use super::config::LogConfig;
use crate::util::Environment;
use anyhow::Result;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{LogExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider};
use opentelemetry_semantic_conventions::resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION};

pub fn init_logger_provider<T>(log_config: T) -> Result<SdkLoggerProvider>
where
    T: AsRef<LogConfig>,
{
    let log_config = log_config.as_ref();

    let endpoint = log_config.endpoint.clone().unwrap();
    let service_name = log_config.service_name.clone().unwrap();
    let service_version = log_config.service_version.clone().unwrap();

    let exporter = LogExporter::builder()
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

    let provider = SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();

    Ok(provider)
}

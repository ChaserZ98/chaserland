use super::config::LogConfig;
use anyhow::Result;
use opentelemetry_otlp::{LogExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider};

pub fn init_logger_provider<T>(log_config: T, resource: Resource) -> Result<SdkLoggerProvider>
where
    T: AsRef<LogConfig>,
{
    let log_config = log_config.as_ref();

    let endpoint = log_config.endpoint.clone().unwrap();

    let exporter = LogExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;

    let provider = SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();

    Ok(provider)
}

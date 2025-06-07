use super::config::MetricConfig;
use anyhow::Result;
use opentelemetry::global;
use opentelemetry_otlp::{MetricExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider, Temporality},
};
use std::time::Duration;

pub fn init_meter_provider(
    metric_config: impl AsRef<MetricConfig>,
    resource: Resource,
) -> Result<SdkMeterProvider> {
    let config = metric_config.as_ref();

    let endpoint = config.endpoint.clone().unwrap();

    let exporter = MetricExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .with_temporality(Temporality::Cumulative)
        .build()?;

    let periodic_reader = PeriodicReader::builder(exporter)
        .with_interval(Duration::from_secs(30))
        .build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource)
        .with_reader(periodic_reader)
        .build();

    global::set_meter_provider(meter_provider.clone());

    Ok(meter_provider)
}

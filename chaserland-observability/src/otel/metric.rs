use super::config::MetricConfig;
use crate::util::Environment;
use anyhow::Result;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::{MetricExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider, Temporality},
};
use opentelemetry_semantic_conventions::resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION};
use std::time::Duration;

pub fn init_meter_provider(metric_config: impl AsRef<MetricConfig>) -> Result<SdkMeterProvider> {
    let config = metric_config.as_ref();

    let endpoint = config.endpoint.clone().unwrap();
    let service_name = config.service_name.clone().unwrap();
    let service_version = config.service_version.clone().unwrap();

    let exporter = MetricExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .with_temporality(Temporality::Cumulative)
        .build()?;

    let periodic_reader = PeriodicReader::builder(exporter)
        .with_interval(Duration::from_secs(30))
        .build();

    let environment_name = String::from(Environment::from_env());

    let resource = Resource::builder_empty()
        .with_service_name(service_name)
        .with_attributes(vec![
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, environment_name),
            KeyValue::new(SERVICE_VERSION, service_version),
        ])
        .build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource)
        .with_reader(periodic_reader)
        .build();

    global::set_meter_provider(meter_provider.clone());

    Ok(meter_provider)
}

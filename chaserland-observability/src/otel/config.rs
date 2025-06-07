use super::{log::init_logger_provider, metric::init_meter_provider, trace::init_tracer_provider};
use crate::util::Environment;
use anyhow::Result;
use opentelemetry::{trace::TracerProvider, KeyValue};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::{
    logs::{SdkLogger, SdkLoggerProvider},
    metrics::SdkMeterProvider,
    trace::{SdkTracer, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION};
use serde::{Deserialize, Serialize};
use tracing_core::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{filter::Filtered, registry::LookupSpan, EnvFilter, Layer};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LogConfig {
    pub endpoint: Option<String>,
}

impl AsRef<LogConfig> for LogConfig {
    fn as_ref(&self) -> &LogConfig {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TraceConfig {
    pub endpoint: Option<String>,
}

impl AsRef<TraceConfig> for TraceConfig {
    fn as_ref(&self) -> &TraceConfig {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MetricConfig {
    pub endpoint: Option<String>,
}

impl AsRef<MetricConfig> for MetricConfig {
    fn as_ref(&self) -> &MetricConfig {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtelConfig {
    pub service_name: String,
    pub service_version: String,
    pub endpoint: Option<String>,
    #[serde(rename = "log")]
    log_config: Option<LogConfig>,
    #[serde(rename = "metric")]
    metric_config: Option<MetricConfig>,
    #[serde(rename = "trace")]
    trace_config: Option<TraceConfig>,
}

impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            service_name: String::from("default-service"),
            service_version: String::from("0.0.1"),
            endpoint: String::from("http://localhost:4317").into(),
            log_config: None,
            metric_config: None,
            trace_config: None,
        }
    }
}

impl OtelConfig {
    pub fn log_config(&self) -> Option<&LogConfig> {
        self.log_config.as_ref()
    }

    pub fn trace_config(&self) -> Option<&TraceConfig> {
        self.trace_config.as_ref()
    }

    pub fn metric_config(&self) -> Option<&MetricConfig> {
        self.metric_config.as_ref()
    }

    pub fn init_trace<S>(
        &self,
    ) -> Result<(
        Option<SdkTracerProvider>,
        Option<OpenTelemetryLayer<S, SdkTracer>>,
    )>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        if self.trace_config.is_none() {
            return Ok((None, None));
        }
        let trace_config = self.trace_config.as_ref().unwrap();
        let endpoint = trace_config
            .endpoint
            .clone()
            .or(self.endpoint.clone())
            .unwrap_or("http://localhost:4317".into());

        let trace_config = TraceConfig {
            endpoint: Some(endpoint.clone()),
        };

        let provider = init_tracer_provider(trace_config, self.resource())?;

        let tracer = provider.tracer(self.service_name.clone());
        let tracing_layer = OpenTelemetryLayer::new(tracer);

        Ok((Some(provider), Some(tracing_layer)))
    }

    pub fn init_logger<S>(
        &self,
    ) -> Result<(
        Option<SdkLoggerProvider>,
        Option<Filtered<OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger>, EnvFilter, S>>,
    )>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        if self.log_config.is_none() {
            return Ok((None, None));
        }
        let log_config = self.log_config.as_ref().unwrap();
        let endpoint = log_config
            .endpoint
            .clone()
            .or(self.endpoint.clone())
            .unwrap_or("http://localhost:4317".into());

        let log_config = LogConfig {
            endpoint: Some(endpoint.clone()),
        };

        let provider = init_logger_provider(log_config, self.resource())?;

        let filter_otel = EnvFilter::new("info")
            .add_directive("hyper=off".parse().unwrap())
            .add_directive("tonic=off".parse().unwrap())
            .add_directive("h2=off".parse().unwrap())
            .add_directive("reqwest=off".parse().unwrap());

        let logging_layer = OpenTelemetryTracingBridge::new(&provider).with_filter(filter_otel);

        Ok((Some(provider), Some(logging_layer)))
    }

    pub fn init_meter(&self) -> Result<Option<SdkMeterProvider>> {
        if self.metric_config.is_none() {
            return Ok(None);
        }
        let metric_config = self.metric_config.as_ref().unwrap();
        let endpoint = metric_config
            .endpoint
            .clone()
            .or(self.endpoint.clone())
            .unwrap_or("http://localhost:4317".into());

        let metric_config = MetricConfig {
            endpoint: Some(endpoint.clone()),
        };

        let meter_provider = init_meter_provider(metric_config, self.resource())?;

        Ok(Some(meter_provider))
    }

    pub fn with_trace(&mut self) -> &mut Self {
        self.trace_config = TraceConfig::default().into();
        self
    }

    pub fn with_logging(&mut self) -> &mut Self {
        self.log_config = LogConfig::default().into();
        self
    }

    pub fn with_metric(&mut self) -> &mut Self {
        self.metric_config = MetricConfig::default().into();
        self
    }

    fn resource(&self) -> Resource {
        Resource::builder_empty()
            .with_service_name(self.service_name.clone())
            .with_attributes(vec![
                KeyValue::new(
                    DEPLOYMENT_ENVIRONMENT_NAME,
                    String::from(Environment::from_env()),
                ),
                KeyValue::new(SERVICE_VERSION, self.service_version.clone()),
            ])
            .build()
    }
}

use super::{log::init_logger_provider, trace::init_tracer_provider};
use anyhow::Result;
use opentelemetry::trace::TracerProvider;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::{
    logs::{SdkLogger, SdkLoggerProvider},
    trace::{SdkTracer, SdkTracerProvider},
};
use serde::{Deserialize, Serialize};
use tracing_core::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, Layer, filter::Filtered, registry::LookupSpan};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LogConfig {
    pub service_name: Option<String>,
    pub service_version: Option<String>,
    pub endpoint: Option<String>,
}

impl AsRef<LogConfig> for LogConfig {
    fn as_ref(&self) -> &LogConfig {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TraceConfig {
    pub service_name: Option<String>,
    pub service_version: Option<String>,
    pub endpoint: Option<String>,
}

impl AsRef<TraceConfig> for TraceConfig {
    fn as_ref(&self) -> &TraceConfig {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricConfig {
    pub service_name: String,
    pub service_version: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtelConfig {
    pub service_name: Option<String>,
    pub service_version: Option<String>,
    pub endpoint: Option<String>,
    #[serde(rename = "log")]
    log_config: Option<LogConfig>,
    #[serde(rename = "metric")]
    metric_config: Option<MetricConfig>,
    #[serde(rename = "trace")]
    trace_config: Option<TraceConfig>,
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
        let service_name = trace_config
            .service_name
            .clone()
            .or(self.service_name.clone())
            .unwrap_or("default-service".into());
        let service_version = trace_config
            .service_version
            .clone()
            .or(self.service_version.clone())
            .unwrap_or("0.0.1".into());

        let trace_config = TraceConfig {
            service_name: Some(service_name.clone()),
            service_version: Some(service_version.clone()),
            endpoint: Some(endpoint.clone()),
        };

        let provider = init_tracer_provider(trace_config)?;

        let tracer = provider.tracer(service_name.clone());
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
        let service_name = log_config
            .service_name
            .clone()
            .or(self.service_name.clone())
            .unwrap_or("default-service".into());
        let service_version = log_config
            .service_version
            .clone()
            .or(self.service_version.clone())
            .unwrap_or("0.0.1".into());

        let log_config = LogConfig {
            service_name: Some(service_name.clone()),
            service_version: Some(service_version.clone()),
            endpoint: Some(endpoint.clone()),
        };

        let provider = init_logger_provider(log_config)?;

        let filter_otel = EnvFilter::new("info")
            .add_directive("hyper=off".parse().unwrap())
            .add_directive("tonic=off".parse().unwrap())
            .add_directive("h2=off".parse().unwrap())
            .add_directive("reqwest=off".parse().unwrap());

        let logging_layer = OpenTelemetryTracingBridge::new(&provider).with_filter(filter_otel);

        Ok((Some(provider), Some(logging_layer)))
    }

    pub fn with_trace(&mut self) -> &mut Self {
        self.trace_config = TraceConfig::default().into();
        self
    }

    pub fn with_logging(&mut self) -> &mut Self {
        self.log_config = LogConfig::default().into();
        self
    }
}

impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            service_name: String::from("default-service").into(),
            service_version: String::from("0.0.1").into(),
            endpoint: String::from("http://localhost:4317").into(),
            log_config: None,
            metric_config: None,
            trace_config: None,
        }
    }
}

use crate::{OtelConfig, stdout_log::LogFormat, util::global_filter_layer};
use anyhow::Result;
use opentelemetry::global;
use opentelemetry_sdk::{
    logs::SdkLoggerProvider, metrics::SdkMeterProvider, propagation::TraceContextPropagator,
    trace::SdkTracerProvider,
};
use tracing_subscriber::{Registry, layer::SubscriberExt, util::SubscriberInitExt};

pub struct OtelProvider {
    logger_provider: Option<SdkLoggerProvider>,
    tracer_provider: Option<SdkTracerProvider>,
    meter_provider: Option<SdkMeterProvider>,
}

impl OtelProvider {
    pub fn logger_provider(&self) -> Option<&SdkLoggerProvider> {
        self.logger_provider.as_ref()
    }

    pub fn tracer_provider(&self) -> Option<&SdkTracerProvider> {
        self.tracer_provider.as_ref()
    }

    pub fn meter_provider(&self) -> Option<&SdkMeterProvider> {
        self.meter_provider.as_ref()
    }

    pub fn shutdown_all(&self) -> Result<()> {
        if let Some(tracer_provider) = &self.tracer_provider {
            tracer_provider.shutdown()?;
        }
        if let Some(meter_provider) = &self.meter_provider {
            meter_provider.shutdown()?;
        }
        if let Some(logger_provider) = &self.logger_provider {
            logger_provider.shutdown()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Observability {
    otel_config: OtelConfig,
    stdout_format: LogFormat,
}

impl Observability {
    pub fn init(&mut self) -> Result<OtelProvider> {
        let propagator = TraceContextPropagator::new();
        global::set_text_map_propagator(propagator);

        let subscriber = Registry::default();
        let global_filter_layer = global_filter_layer();
        let stdout_layer = LogFormat::from_env().layer();

        let (tracer_provider, tracing_layer) = self.otel_config.init_trace()?;

        let (logger_provider, logging_layer) = self.otel_config.init_logger()?;

        subscriber
            .with(global_filter_layer)
            .with(logging_layer)
            .with(stdout_layer)
            .with(tracing_layer)
            .try_init()?;

        let otel_provider = OtelProvider {
            logger_provider,
            tracer_provider,
            meter_provider: None,
        };

        Ok(otel_provider)
    }

    pub fn with_otel_config(&mut self, config: OtelConfig) -> &mut Self {
        self.otel_config = config;
        self
    }

    pub fn with_stdout_format(&mut self, format: LogFormat) -> &mut Self {
        self.stdout_format = format;
        self
    }
}

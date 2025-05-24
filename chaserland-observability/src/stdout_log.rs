use std::env;
use tracing_subscriber::{Layer, fmt, registry::LookupSpan};

#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Json,
    Compact,
    Pretty,
    Full,
    None,
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Full
    }
}

impl LogFormat {
    pub fn from_env() -> Self {
        match env::var("LOG_FORMAT")
            .unwrap_or("full".to_string())
            .as_str()
        {
            "json" => Self::Json,
            "compact" => Self::Compact,
            "pretty" => Self::Pretty,
            "none" => Self::None,
            _ => Self::Full,
        }
    }
    pub fn layer<S>(&self) -> Option<Box<dyn Layer<S> + Send + Sync + 'static>>
    where
        S: tracing_core::Subscriber + for<'span> LookupSpan<'span>,
    {
        if let Self::None = self {
            return None;
        }

        let layer = fmt::layer()
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(true)
            .with_line_number(true)
            .with_ansi(true);

        match self {
            Self::Json => Some(
                layer
                    .json()
                    .with_current_span(true)
                    .with_span_list(true)
                    .boxed(),
            ),
            Self::Compact => Some(layer.compact().boxed()),
            Self::Pretty => Some(layer.pretty().boxed()),
            Self::None => None,
            _ => Some(layer.boxed()),
        }
    }
}

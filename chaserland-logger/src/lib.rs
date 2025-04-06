use std::env;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt, registry};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Json,
    Compact,
    Pretty,
    Full,
}

pub fn init_logger() {
    let app_env = env::var("APP_ENV").unwrap_or(if cfg!(debug_assertions) {
        "dev".to_string()
    } else {
        "prod".to_string()
    });

    let log_format = match env::var("LOG_FORMAT")
        .unwrap_or("full".to_string())
        .as_str()
    {
        "json" => LogFormat::Json,
        "compact" => LogFormat::Compact,
        "pretty" => LogFormat::Pretty,
        "full" => LogFormat::Full,
        _ => LogFormat::Full,
    };

    let level_filter = match app_env.as_str() {
        "dev" => LevelFilter::DEBUG,
        _ => LevelFilter::INFO,
    };

    let subscriber = registry();
    let layer = fmt::layer()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true)
        .with_line_number(true);

    let boxed_layer = match log_format {
        LogFormat::Json => layer
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .with_filter(level_filter)
            .boxed(),
        LogFormat::Compact => layer
            .compact()
            .with_ansi(true)
            .with_filter(level_filter)
            .boxed(),
        LogFormat::Pretty => layer
            .pretty()
            .with_ansi(true)
            .with_filter(level_filter)
            .boxed(),
        _ => layer.with_ansi(true).with_filter(level_filter).boxed(),
    };

    subscriber.with(boxed_layer).init();
}

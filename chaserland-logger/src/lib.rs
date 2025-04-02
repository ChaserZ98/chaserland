use tracing_subscriber::Layer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Json,
    Compact,
    Pretty,
    Full,
}

pub fn init_logger(log_format: LogFormat, level_filter: LevelFilter) {
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

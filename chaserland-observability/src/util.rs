use std::env;
use tracing_core::LevelFilter;
use tracing_subscriber::EnvFilter;

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn from_env() -> Self {
        match env::var("APP_ENV").unwrap_or_default().as_str() {
            "development" => Self::Development,
            "production" => Self::Production,
            _ => match cfg!(debug_assertions) {
                true => Self::Development,
                false => Self::Production,
            },
        }
    }
}

impl From<Environment> for String {
    fn from(value: Environment) -> Self {
        match value {
            Environment::Development => "development".into(),
            Environment::Production => "production".into(),
        }
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Production => write!(f, "production"),
        }
    }
}

pub fn global_filter_layer() -> EnvFilter {
    let default_level = match Environment::from_env() {
        Environment::Development => LevelFilter::DEBUG,
        Environment::Production => LevelFilter::INFO,
    };

    let global_filter_layer = EnvFilter::builder()
        .with_default_directive(default_level.into())
        .from_env_lossy()
        .add_directive("opentelemetry=debug".parse().unwrap())
        .add_directive("opentelemetry_sdk=off".parse().unwrap());

    global_filter_layer
}

use anyhow::Result;
use chaserland_articles_service::db::DBConfig;
use chaserland_common::figment::FileProvider;
use chaserland_observability::OtelConfig;
use figment::{
    Figment,
    providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    #[serde(rename = "otel")]
    pub otel_config: OtelConfig,
    #[serde(rename = "db")]
    pub db_config: DBConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        let host = if cfg!(debug_assertions) {
            "[::1]".to_string()
        } else {
            "[::]".to_string()
        };

        Self {
            host,
            port: "8080".to_string(),
            otel_config: OtelConfig::default(),
            db_config: DBConfig::default(),
        }
    }
}

pub struct AppConfigLoader<P>
where
    P: AsRef<Path>,
{
    path: Option<P>,
}

impl<P> AppConfigLoader<P>
where
    P: AsRef<Path>,
{
    pub fn new() -> Self {
        AppConfigLoader { path: None }
    }

    pub fn with_path(self, path: P) -> Self {
        AppConfigLoader { path: Some(path) }
    }

    pub fn load(self) -> Result<AppConfig> {
        let mut figment = Figment::from(Serialized::defaults(AppConfig::default()))
            .merge(Env::prefixed("APP__").split("__"));

        if let Some(path) = self.path {
            if let Err(e) = OpenOptions::new().read(true).open(&path) {
                return Err(anyhow::anyhow!(
                    "Unable to open config file {}: {}",
                    path.as_ref().display(),
                    e
                ));
            }
            let file_provider = FileProvider::try_from(path)?;
            figment = figment.merge(file_provider);
        }

        let config = figment.extract()?;

        Ok(config)
    }
}

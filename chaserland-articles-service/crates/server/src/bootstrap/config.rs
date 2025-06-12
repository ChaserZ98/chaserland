use crate::db::DBConfig;
use anyhow::Result;
use chaserland_common::figment::FileProvider;
use chaserland_observability::OtelConfig;
use figment::{
    Figment,
    providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
    #[serde(rename = "otel")]
    pub otel_config: OtelConfig,
    #[serde(rename = "db")]
    pub db_config: DBConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let host = if cfg!(debug_assertions) {
            String::from("[::1]")
        } else {
            String::from("[::]")
        };

        let port = 8080;

        Self {
            host,
            port,
            otel_config: OtelConfig::default(),
            db_config: DBConfig::default(),
        }
    }
}

pub struct ServerConfigLoader<P>
where
    P: AsRef<Path>,
{
    path: Option<P>,
}

impl<P> ServerConfigLoader<P>
where
    P: AsRef<Path>,
{
    pub fn new() -> Self {
        Self { path: None }
    }

    pub fn with_path(self, path: P) -> Self {
        Self { path: Some(path) }
    }

    pub fn load(self) -> Result<ServerConfig> {
        let mut figment = Figment::from(Serialized::defaults(ServerConfig::default()))
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

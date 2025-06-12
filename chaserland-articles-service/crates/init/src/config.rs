use anyhow::Result;
use chaserland_articles_service_server::db::DBConfig;
use figment::{
    Figment,
    providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(rename = "service")]
    pub service_db_config: ServiceDBConfig,
    #[serde(rename = "db")]
    pub db_config: DBConfig,
}

impl AppConfig {
    pub fn try_load() -> Result<Self> {
        let figment = Self::figment();
        let config = figment.extract()?;

        Ok(config)
    }

    pub fn figment() -> Figment {
        Figment::from(Serialized::defaults(AppConfig::default()))
            .merge(Env::prefixed("APP__").split("__"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceDBConfig {
    pub name: String,
    pub username: String,
    pub password: String,
}

impl Default for ServiceDBConfig {
    fn default() -> Self {
        Self {
            name: "chaserland_article".to_string(),
            username: "chaserland_article".to_string(),
            password: "chaserland_article".to_string(),
        }
    }
}

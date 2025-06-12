use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use sqlx::ConnectOptions;
use sqlx::Postgres;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{PgPool, migrate::MigrateDatabase};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DBConfig {
    pub url: String,
    pub max_connections: u32,
    pub slow_threshold: Option<u64>,
}

impl DBConfig {
    pub fn new(url: String, max_connections: u32, slow_threshold: Option<u64>) -> Self {
        Self {
            url,
            max_connections,
            slow_threshold,
        }
    }
    pub fn parse_db_url(&self) -> (String, u16, String, String, String) {
        let connection_options = PgConnectOptions::from_str(&self.url).unwrap();
        let url = connection_options.to_url_lossy();
        let host = url.host_str().unwrap_or("localhost").into();
        let port = url.port().unwrap_or(5432);
        let username = url.username().into();
        let password = url.password().unwrap_or("").into();
        let mut database = url.path().trim_start_matches('/');
        if database.is_empty() {
            database = "postgres";
        }
        let database = database.into();

        (host, port, username, password, database)
    }
}

impl Default for DBConfig {
    fn default() -> Self {
        Self::new(
            "postgres://postgres:postgres@localhost:5432".into(),
            5,
            Some(100),
        )
    }
}

pub async fn connect_db(db_config: DBConfig) -> Result<PgPool> {
    let db_url = &db_config.url;

    match Postgres::database_exists(db_url).await {
        Err(e) => {
            return Err(e.into());
        }
        Ok(false) => {
            return Err(anyhow::anyhow!(
                "Database does not exist. Please make sure the database exists and migrations have been run."
            ));
        }
        _ => (),
    }

    let mut opts = PgConnectOptions::from_str(db_url)?;
    if let Some(slow_threshold) = db_config.slow_threshold {
        opts = opts.log_slow_statements(
            log::LevelFilter::Warn,
            std::time::Duration::from_millis(slow_threshold),
        );
    }

    let db = PgPoolOptions::new()
        .max_connections(db_config.max_connections)
        .connect_with(opts)
        .await?;

    tracing::info!("Connection to database established.");

    Ok(db)
}

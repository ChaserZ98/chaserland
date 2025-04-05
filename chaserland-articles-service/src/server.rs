use crate::db;
use crate::service::ArticleService;
use anyhow::Result;
use chaserland_logger::init_logger;
use tonic::transport::Server as TonicServer;
use tracing::level_filters::LevelFilter;

#[derive(Default)]
pub struct Server {}

#[allow(dead_code)]
impl Server {
    pub async fn run(&self, addr: &str) -> Result<()> {
        let level_filter = if cfg!(debug_assertions) {
            LevelFilter::DEBUG
        } else {
            LevelFilter::INFO
        };
        init_logger(chaserland_logger::LogFormat::Full, level_filter);

        let db = match db::connect_db().await {
            Ok(db) => db,
            Err(why) => {
                tracing::error!("Failed to create db connection pool: {}", why);
                return Err(why.into());
            }
        };

        let addr = addr.parse()?;
        tracing::info!("Server binding to address {}", addr);

        TonicServer::builder()
            .add_service(ArticleService::new(db))
            .serve_with_shutdown(addr, self.shutdown())
            .await?;

        Ok(())
    }
    async fn shutdown(&self) {
        tracing::info!("Listening for shutdown signal...");
        if let Err(why) = tokio::signal::ctrl_c().await {
            tracing::error!("Error while shutting down: {}", why);
        } else {
            tracing::info!("Shutting down...");
        }
    }
}

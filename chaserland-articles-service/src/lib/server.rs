use crate::db;
use crate::service::ArticleService;
use anyhow::Result;
use tonic::transport::Server as TonicServer;

#[derive(Default)]
pub struct Server {}

#[allow(dead_code)]
impl Server {
    pub async fn run(&self, addr: &str) -> Result<()> {
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
            .add_service(ArticleService::new(db.clone()).into_tonic_service())
            .serve_with_shutdown(addr, self.shutdown())
            .await?;

        tracing::info!("Server stopped.");

        tracing::info!("Closing db connection pool...");
        db.close().await;
        tracing::info!("Db connection pool closed.");

        tracing::info!("Cleanup completed. Server shutdown complete.");

        Ok(())
    }
    async fn shutdown(&self) {
        tracing::info!("Listening for shutdown signal...");
        if let Err(why) = tokio::signal::ctrl_c().await {
            tracing::error!("Error while shutting down: {}", why);
            return;
        }
        tracing::info!("Shutdown signal received. Starting graceful shutdown...");
    }
}

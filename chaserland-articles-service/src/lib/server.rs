use crate::{
    app::service::ArticleService,
    db,
    infra::repository::postgres::{
        article::PgArticleRepository, category::PgCategoryRepository, series::PgSeriesRepository,
        tag::PgTagRepository,
    },
    ports::grpc::service::GrpcArticleService,
};
use anyhow::Result;
use tonic::transport::Server as TonicServer;

#[derive(Default)]
pub struct Server {}

#[allow(dead_code)]
impl Server {
    pub async fn run(&self, addr: &str) -> Result<()> {
        let pool = match db::connect_db().await {
            Ok(db) => db,
            Err(why) => {
                tracing::error!("Failed to create db connection pool: {}", why);
                return Err(why.into());
            }
        };

        let article_repository = PgArticleRepository::new(pool.clone());
        let series_repository = PgSeriesRepository::new(pool.clone());
        let category_repository = PgCategoryRepository::new(pool.clone());
        let tag_repository = PgTagRepository::new(pool.clone());

        let article_service = GrpcArticleService::new(ArticleService::new(
            article_repository,
            series_repository,
            category_repository,
            tag_repository,
        ))
        .into_tonic_service();

        let addr = addr.parse()?;
        tracing::info!("Server binding to address {}", addr);

        TonicServer::builder()
            .add_service(article_service)
            .serve_with_shutdown(addr, self.shutdown())
            .await?;

        tracing::info!("Server stopped.");

        tracing::info!("Closing db connection pool...");
        pool.close().await;
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

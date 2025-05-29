use anyhow::Result;
use chaserland_articles_service::migrator::MIGRATOR;
use chaserland_observability::Observability;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let otel_provider = Observability::default().init()?;

    let default_url = String::from(
        "postgres://chaserland_article:chaserland_article@localhost:5432/chaserland_article",
    );
    let db_url = env::var("DATABASE_URL").unwrap_or(default_url);

    let db = PgPoolOptions::new().connect(&db_url).await.map_err(|e| {
        tracing::error!("Failed to connect to database: {}", e);
        e
    })?;

    tracing::info!("Migrating database...");

    MIGRATOR.run(&db).await.map_err(|e| {
        tracing::error!("Failed to run migrations: {}", e);
        e
    })?;

    otel_provider.shutdown_all()?;

    Ok(())
}

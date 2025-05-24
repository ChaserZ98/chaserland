use anyhow::Result;
use chaserland_articles_service::migrator::MIGRATOR;
use chaserland_observability::Observability;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let otel_provider = Observability::default().init()?;

    let username = env::var("POSTGRES_USER").unwrap_or("chaserland_article".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or("chaserland_article".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string());
    let db_name = env::var("POSTGRES_DB").unwrap_or("chaserland_article".to_string());

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, db_name
    );

    let db = match PgPoolOptions::new().connect(&db_url).await {
        Ok(db) => db,
        Err(why) => {
            tracing::error!("Failed to connect to database: {}", why);
            return Err(why.into());
        }
    };

    tracing::info!("Migrating database...");

    if let Err(e) = MIGRATOR.run(&db).await {
        tracing::error!("Failed to migrate database: {}", e);
        return Err(e.into());
    }

    otel_provider.shutdown_all()?;

    Ok(())
}

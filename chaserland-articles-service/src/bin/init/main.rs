mod config;

use anyhow::Result;
use chaserland_observability::Observability;
use config::AppConfig;
use sqlx::{Connection, Executor, PgConnection};

#[tokio::main]
pub async fn main() -> Result<()> {
    let otel_provider = Observability::default().init()?;

    let config = AppConfig::try_load().map_err(|e| {
        tracing::error!("Failed to load configuration: {}", e);
        e
    })?;

    let db_config = config.db_config;
    let service_db_config = config.service_db_config;

    let mut db = sqlx::postgres::PgConnection::connect(&db_config.url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to database: {}", e);
            e
        })?;

    create_db(&mut db, &service_db_config.name)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create database: {}", e);
            e
        })?;

    db.close().await.map_err(|e| {
        tracing::error!("Failed to close database connection: {}", e);
        e
    })?;

    let (host, port, username, password, _) = db_config.parse_db_url();

    let new_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, service_db_config.name
    );

    let mut db = sqlx::postgres::PgConnection::connect(&new_url).await?;

    create_user(
        &mut db,
        &service_db_config.username,
        &service_db_config.password,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {}", e);
        e
    })?;

    grant_permissions(
        &mut db,
        &service_db_config.username,
        &service_db_config.name,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to grant permissions: {}", e);
        e
    })?;

    tracing::info!("Database initialization complete.");

    otel_provider.shutdown_all()?;

    Ok(())
}

async fn create_db(db: &mut PgConnection, db_name: impl AsRef<str>) -> Result<()> {
    let db_name = db_name.as_ref();

    let exists = sqlx::query("SELECT 1 FROM pg_database WHERE datname = $1")
        .bind(db_name)
        .fetch_optional(&mut *db)
        .await?
        .is_some();

    if exists {
        tracing::info!("Database {} already exist.", db_name);
        return Ok(());
    }

    tracing::info!("Creating database {}...", db_name);

    let query = format!("CREATE DATABASE {}", db_name);

    db.execute(query.as_str()).await?;

    Ok(())
}

async fn create_user(
    db: &mut PgConnection,
    username: impl AsRef<str>,
    password: impl AsRef<str>,
) -> Result<()> {
    let (username, password) = (username.as_ref(), password.as_ref());

    let exists = sqlx::query("SELECT 1 FROM pg_roles WHERE rolname = $1")
        .bind(username)
        .fetch_optional(&mut *db)
        .await?
        .is_some();

    if exists {
        tracing::info!("User {} already exist.", username);
        return Ok(());
    }

    tracing::info!("Creating user {}...", username);

    let query = format!("CREATE USER {} WITH PASSWORD '{}'", username, password);
    db.execute(query.as_str()).await?;

    Ok(())
}

async fn grant_permissions(db: &mut PgConnection, username: &str, db_name: &str) -> Result<()> {
    tracing::info!(
        "Granting permissions to user {} on database {}...",
        username,
        db_name
    );

    let query = format!(
        "GRANT ALL PRIVILEGES ON DATABASE {} TO {}",
        db_name, username
    );
    db.execute(query.as_str()).await?;

    let query = format!("GRANT ALL PRIVILEGES ON SCHEMA public TO {}", username);
    db.execute(query.as_str()).await?;

    Ok(())
}

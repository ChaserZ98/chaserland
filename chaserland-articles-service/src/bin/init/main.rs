use anyhow::Result;
use chaserland_observability::Observability;
use sqlx::{Connection, Executor, PgConnection};
use std::env;

#[tokio::main]
pub async fn main() -> Result<()> {
    let otel_provider = Observability::default().init()?;

    let username = env::var("POSTGRES_USER").unwrap_or("postgres".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string());

    let service_db_name = env::var("ARTICLE_DB_NAME").unwrap_or("chaserland_article".to_string());
    let service_username = env::var("ARTICLE_DB_USER").unwrap_or("chaserland_article".to_string());
    let service_password =
        env::var("ARTICLE_DB_PASSWORD").unwrap_or("chaserland_article".to_string());

    let mut db = sqlx::postgres::PgConnection::connect(
        format!("postgres://{}:{}@{}:{}", username, password, host, port).as_str(),
    )
    .await?;

    if let Err(e) = create_db(&mut db, &service_db_name).await {
        tracing::error!("Failed to create db: {}", e);
        return Err(e.into());
    }

    db.close().await?;

    db = sqlx::postgres::PgConnection::connect(
        format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, service_db_name
        )
        .as_str(),
    )
    .await?;

    if let Err(e) = create_user(&mut db, &service_username, &service_password).await {
        tracing::error!("Failed to create user: {}", e);
        return Err(e.into());
    }

    if let Err(e) = grant_permissions(&mut db, &service_username, &service_db_name).await {
        tracing::error!("Failed to grant permissions: {}", e);
        return Err(e.into());
    }

    tracing::info!("Database initialization complete.");

    otel_provider.shutdown_all()?;

    Ok(())
}

async fn create_db(db: &mut PgConnection, db_name: &str) -> Result<()> {
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

    db.execute(format!("CREATE DATABASE {}", db_name).as_str())
        .await?;

    Ok(())
}

async fn create_user(db: &mut PgConnection, username: &str, password: &str) -> Result<()> {
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

    db.execute(format!("CREATE USER {} WITH PASSWORD '{}'", username, password).as_str())
        .await?;

    Ok(())
}

async fn grant_permissions(db: &mut PgConnection, username: &str, db_name: &str) -> Result<()> {
    tracing::info!(
        "Granting permissions to user {} on database {}...",
        username,
        db_name
    );

    db.execute(
        format!(
            "GRANT ALL PRIVILEGES ON DATABASE {} TO {}",
            db_name, username
        )
        .as_str(),
    )
    .await?;

    db.execute(format!("GRANT ALL PRIVILEGES ON SCHEMA public TO {}", username).as_str())
        .await?;

    Ok(())
}

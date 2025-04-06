use anyhow::Result;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, migrate::MigrateDatabase};
use std::env;

pub async fn connect_db() -> Result<PgPool> {
    let username = env::var("POSTGRES_USER").unwrap_or("postgres".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string());
    let db_name = env::var("POSTGRES_DB").unwrap_or("chaserland_article".to_string());

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, db_name
    );

    match Postgres::database_exists(&db_url).await {
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

    let db = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(db) => db,
        Err(why) => {
            return Err(why.into());
        }
    };

    tracing::info!("Connection to database established.");

    Ok(db)
}

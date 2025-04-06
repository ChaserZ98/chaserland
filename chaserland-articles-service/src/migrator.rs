use anyhow::Result;
use chaserland_logger::init_logger;
use sqlx::{Postgres, migrate::MigrateDatabase, postgres::PgPoolOptions};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let username = env::var("POSTGRES_USER").unwrap_or("postgres".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string());
    let db_name = env::var("POSTGRES_DB").unwrap_or("chaserland_article".to_string());

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, db_name
    );

    let is_db_exist = match Postgres::database_exists(&db_url).await {
        Err(e) => {
            tracing::error!("Failed to check if db exist: {}", e);
            return Err(e.into());
        }
        Ok(value) => value,
    };
    if !is_db_exist {
        tracing::info!("Database does not exist. Creating database...");
        match Postgres::create_database(&db_url).await {
            Err(e) => {
                tracing::error!("Failed to create database: {}", e);
                return Err(e.into());
            }
            Ok(_) => (),
        };
    }

    let db = match PgPoolOptions::new().connect(&db_url).await {
        Ok(db) => db,
        Err(why) => {
            tracing::error!("Failed to connect to database: {}", why);
            return Err(why.into());
        }
    };

    tracing::info!("Migrating database...");

    match sqlx::migrate!("./migrations").run(&db).await {
        Err(e) => {
            tracing::error!("Failed to migrate database: {}", e);
            return Err(e.into());
        }
        Ok(_) => (),
    }

    Ok(())
}

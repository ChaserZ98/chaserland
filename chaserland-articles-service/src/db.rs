use anyhow::Result;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, migrate::MigrateDatabase};

pub async fn connect_db() -> Result<PgPool> {
    let db_url = "postgres://postgres:123456@localhost:5432/chaserland_article";

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
        .connect(db_url)
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

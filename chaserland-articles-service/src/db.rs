use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, migrate::MigrateDatabase};

pub async fn connect_db() -> Result<PgPool, Box<dyn std::error::Error>> {
    let db_url = "postgres://postgres:123456@localhost:5432/chaserland-article";
    if !Postgres::database_exists(&db_url).await? {
        tracing::info!("Creating database...");
        Postgres::create_database(&db_url).await?
    }

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    tracing::info!("Connection to database established.");

    Ok(db)
}

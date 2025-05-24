mod config;
use anyhow::Result;
use chaserland_articles_service::server::Server;
use chaserland_observability::Observability;
use clap::Parser;
use config::{AppConfig, AppConfigLoader};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "config.json")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config: AppConfig = AppConfigLoader::new().with_path(args.config).load()?;

    let otel_provider = Observability::default()
        .with_otel_config(config.otel_config)
        .init()?;

    let addr = format!("{}:{}", config.host, config.port);

    let server = Server::default();
    server.run(&addr, config.db_config).await?;

    otel_provider.shutdown_all()?;

    Ok(())
}

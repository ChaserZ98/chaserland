use anyhow::Result;
use chaserland_articles_service::bootstrap::{Server, ServerConfig, ServerConfigLoader};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "config.json")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config: ServerConfig = ServerConfigLoader::new().with_path(args.config).load()?;
    let mut server = Server::default();
    server.set_config(config);
    server.run().await?;

    Ok(())
}

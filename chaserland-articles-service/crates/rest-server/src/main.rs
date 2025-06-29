use anyhow::Result;
use chaserland_articles_service_rest_server::{Server, ServerConfigLoader};
use clap::Parser;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "config.json")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = ServerConfigLoader::new().with_path(args.config).load()?;
    let server = Server::from(config);
    server.run().await?;

    Ok(())
}

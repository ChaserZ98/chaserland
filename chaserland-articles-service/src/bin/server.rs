use anyhow::Result;
use chaserland_articles_service::server::Server;
use chaserland_logger::init_logger;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let host = if cfg!(debug_assertions) {
        "[::1]".to_string()
    } else {
        "[::]".to_string()
    };
    let port = env::var("APP_PORT").unwrap_or("8080".to_string());
    let addr = format!("{}:{}", host, port);

    let server = Server::default();
    server.run(&addr).await?;
    Ok(())
}

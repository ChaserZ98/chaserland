use anyhow::Result;

mod db;
mod model;
mod server;
mod service;

use server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::default();
    server.run("[::1]:50051").await?;
    Ok(())
}

mod db;
mod model;
mod server;

use server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new();
    server.run("[::1]:50051").await?;
    Ok(())
}

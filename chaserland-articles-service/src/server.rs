use chaserland_logger::init_logger;
use chaserland_protos::article::{
    GetArticlesRequest, GetArticlesResponse,
    article_server::{Article, ArticleServer},
};
use tonic::transport::Server as TonicServer;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct Server {}

#[allow(dead_code)]
impl Server {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn run(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        if cfg!(debug_assertions) {
            println!("dev")
        } else {
            println!("prod")
        }
        init_logger(chaserland_logger::LogFormat::Full);
        let addr = addr.parse()?;
        tracing::info!("Server binding to address {}", addr);
        TonicServer::builder()
            .add_service(ArticleService::new())
            .serve_with_shutdown(addr, self.shutdown())
            .await?;
        Ok(())
    }
    async fn shutdown(&self) {
        tracing::info!("Listening for shutdown signal...");
        if let Err(why) = tokio::signal::ctrl_c().await {
            tracing::error!("Error while shutting down: {}", why);
        } else {
            tracing::info!("Shutting down...");
        }
    }
}

pub struct ArticleService {}

impl ArticleService {
    pub fn new() -> ArticleServer<ArticleService> {
        ArticleServer::new(ArticleService {})
    }
}

#[tonic::async_trait]
impl Article for ArticleService {
    async fn get_articles(
        &self,
        request: Request<GetArticlesRequest>,
    ) -> Result<Response<GetArticlesResponse>, Status> {
        let message = request.get_ref();
        let reply = GetArticlesResponse {
            message: format!("Hello {}!", message.name),
        };
        Ok(Response::new(reply))
    }
}

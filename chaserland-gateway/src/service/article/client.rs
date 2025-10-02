use crate::bootstrap::ServiceConfig;
use chaserland_protos::article::v1::article_service_client::ArticleServiceClient;
use tonic::transport::{Channel, Endpoint};

pub struct Client(ArticleServiceClient<Channel>);

impl Client {
    pub async fn try_from_config(config: ServiceConfig) -> Result<Self, tonic::transport::Error> {
        let endpoint = Endpoint::from_shared(config.endpoint)?.connect().await?;
        let client = ArticleServiceClient::new(endpoint);
        Ok(Client(client))
    }
}

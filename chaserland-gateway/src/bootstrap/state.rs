use chaserland_protos::article::v1::article_service_client::ArticleServiceClient;
use std::collections::HashMap;
use std::sync::RwLock;
use tonic::transport::Channel;

pub struct State {
    pub services: RwLock<HashMap<Service, ServiceClient>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Service {
    Article,
}

pub enum ServiceClient {
    Article(ArticleServiceClient<Channel>),
}

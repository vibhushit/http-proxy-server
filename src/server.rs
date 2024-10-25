use hyper::{Client, Request, Response, Body, Server};
use tokio::net::TcpListener;
use crate::cache::Cache;
use crate::load_balancer::LoadBalancer;
use std::sync::Arc;
use tracing::info;

pub struct ProxyServer {
    client: Client<hyper::client::HttpConnector>,
    cache: Arc<Cache>,
    load_balancer: Arc<LoadBalancer>,
}

impl ProxyServer {
    pub fn new(backends: Vec<String>) -> Self {
        Self {
            client: Client::new(),
            cache: Arc::new(Cache::new()),
            load_balancer: Arc::new(LoadBalancer::new(backends)),
        }
    }

    pub async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let uri = req.uri().to_string();

        // Check cache
        if let Some(response) = self.cache.get(&uri) {
            info!("Cache hit for {}", uri);
            return Ok(response);
        }

        // Get backend server using load balancer
        let backend_url = self.load_balancer.get_next_backend();
        let mut proxied_req = req;
        *proxied_req.uri_mut() = backend_url.parse().unwrap();

        // Forward request
        let response = self.client.request(proxied_req).await?;
        self.cache.insert(uri, response.clone());
        Ok(response)
    }

    pub async fn run(self, listener: TcpListener) {
        let service = hyper::service::make_service_fn(|_| {
            let server = self.clone();
            async { Ok::<_, hyper::Error>(hyper::service::service_fn(move |req| server.handle_request(req))) }
        });

        Server::from_tcp(listener).unwrap().serve(service).await.unwrap();
    }
}

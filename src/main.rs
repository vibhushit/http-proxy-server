mod server;
mod cache;
mod load_balancer;

use server::ProxyServer;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // Initialize logger

    let addr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    let server = ProxyServer::new(vec![
        "http://localhost:4001".to_string(),
        "http://localhost:4002".to_string(),
    ]);

    info!("Starting proxy server on http://{}", addr);
    server.run(listener).await;
}


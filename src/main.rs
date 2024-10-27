use cache::Cache;
use tokio::net::TcpListener;
use std::sync::Arc;
// use dashmap::DashMap;

mod proxy;
mod cache;
mod load_balancer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Arc::new(Cache::new());
    let load_balancer = load_balancer::LoadBalancer::new(vec![
        "http://localhost:8081".to_string(),
        "http://localhost:8082".to_string(),
    ]);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Proxy server listening on http://127.0.0.1:3000");

    loop {
        let (stream, _) = listener.accept().await?;
        let cache = Arc::clone(&cache);
        let load_balancer = load_balancer.clone();

        tokio::spawn(async move {
            if let Err(e) = proxy::handle_client(stream, cache, load_balancer).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

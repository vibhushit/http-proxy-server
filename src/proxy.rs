use tokio::net::TcpStream;
use std::sync::Arc;
// use dashmap::DashMap;
use crate::cache::Cache;
use crate::load_balancer::LoadBalancer;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn handle_client(
    mut stream: TcpStream,
    cache: Arc<Cache>,
    load_balancer: LoadBalancer,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..n]);

    // Use a borrowed reference to `request` here
    if let Some(response) = cache.get(request.as_ref()) {
        stream.write_all(response.as_bytes()).await?;
    } else {
        let backend_url = load_balancer.next_backend();
        let response = reqwest::get(format!("{}/{}", backend_url, request))
            .await?
            .text()
            .await?;

        // Store the response in cache
        cache.insert(request.to_string(), response.clone());
        stream.write_all(response.as_bytes()).await?;
    }
    Ok(())
}

use hyper::{Body, Client, Request, Response, Server, StatusCode};
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use tokio::sync::Mutex;
use dashmap::DashMap;
use std::sync::Arc;

type Cache = Arc<DashMap<String, String>>;

#[tokio::main]
async fn main() {
    // Set up the cache and client
    let cache: Cache = Arc::new(DashMap::new());
    let client = Client::builder().build::<_, Body>(HttpConnector::new());

    // Define the server address and service
    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = make_service_fn(move |_| {
        let cache = cache.clone();
        let client = client.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                proxy_request(req, client.clone(), cache.clone())
            }))
        }
    });

    // Start the server
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn proxy_request(
    req: Request<Body>,
    client: Client<HttpConnector, Body>,
    cache: Cache,
) -> Result<Response<Body>, hyper::Error> {
    let uri_string = req.uri().to_string();

    // Check if response is cached
    if let Some(response_body) = cache.get(&uri_string) {
        return Ok(Response::new(Body::from(response_body.clone())));
    }

    // Forward request to the upstream server
    match client.request(req).await {
        Ok(mut response) => {
            // Extract the response body
            let body_bytes = hyper::body::to_bytes(response.body_mut()).await?;
            let body_string = String::from_utf8(body_bytes.to_vec()).unwrap_or_default();

            // Cache the response
            cache.insert(uri_string.clone(), body_string.clone());

            // Return the response
            Ok(Response::new(Body::from(body_string)))
        }
        Err(_) => {
            // If there’s an error, respond with a 500 status
            let mut response = Response::new(Body::from("Internal Server Error"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(response)
        }
    }
}

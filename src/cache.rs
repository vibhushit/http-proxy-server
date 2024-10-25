use dashmap::DashMap;
use hyper::{Response, Body};
use std::time::{Instant, Duration};

struct CacheItem {
    response: Response<Body>,
    expiration: Instant,
}

pub struct Cache {
    store: DashMap<String, CacheItem>,
    ttl: Duration,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
            ttl: Duration::from_secs(60),
        }
    }

    pub fn get(&self, key: &str) -> Option<Response<Body>> {
        if let Some(item) = self.store.get(key) {
            if item.expiration > Instant::now() {
                return Some(item.response.clone());
            }
            self.store.remove(key);
        }
        None
    }

    pub fn insert(&self, key: String, response: Response<Body>) {
        self.store.insert(key, CacheItem {
            response,
            expiration: Instant::now() + self.ttl,
        });
    }
}

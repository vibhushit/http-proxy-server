use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::Mutex;

pub struct LoadBalancer {
    backends: Vec<String>,
    index: AtomicUsize,
}

impl LoadBalancer {
    pub fn new(backends: Vec<String>) -> Self {
        Self {
            backends,
            index: AtomicUsize::new(0),
        }
    }

    pub fn get_next_backend(&self) -> &str {
        let idx = self.index.fetch_add(1, Ordering::SeqCst) % self.backends.len();
        &self.backends[idx]
    }
}

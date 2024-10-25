use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct LoadBalancer {
    backends: Vec<String>,
    counter: Arc<AtomicUsize>,
}

impl LoadBalancer {
    pub fn new(backends: Vec<String>) -> Self {
        LoadBalancer {
            backends,
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn next_backend(&self) -> String {
        let index = self.counter.fetch_add(1, Ordering::SeqCst) % self.backends.len();
        self.backends[index].clone()
    }
}

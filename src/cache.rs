use dashmap::DashMap;
use std::sync::Arc;

pub struct Cache {
    inner: Arc<DashMap<String, String>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            inner: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).map(|entry| entry.value().clone())
    }

    pub fn insert(&self, key: String, value: String) {
        self.inner.insert(key, value);
    }
}


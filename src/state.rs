use crate::store::Store;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<RwLock<Store>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            store: Arc::new(RwLock::new(Store::new())),
        }
    }
}

use std::sync::Arc;

use tokio::sync::RwLock;

pub type Cache<T> = Arc<RwLock<Option<T>>>;
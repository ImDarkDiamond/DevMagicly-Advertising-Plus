use sled::Db;
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: Arc<Db>,
    pub http: Arc<Client>,
    pub cache: Arc<InMemoryCache>,
}
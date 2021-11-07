use std::{error::Error, sync::Arc};
use futures::StreamExt;
use tracing::info;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{Intents, Shard};
use twilight_http::Client;

use crate::{config::Config, context::Context};

mod config;
mod context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    info!("Loading configuration for Advertising-Plus");
    dotenv::dotenv().ok();
    let config = Config::from_env();
    info!("Loaded configuration for Advertsing-Plus");


    let (shard, mut events) = Shard::new(config.token.clone(), Intents::all());
    let cache = Arc::new(InMemoryCache::builder().message_cache_size(10).build());
    let http = Arc::new(Client::new(config.token));

    info!("Loading database");
    let db = Arc::new(sled::open("db").expect("Failed to open"));
    info!("Loaded database");

    let context = Context {
        db,
        http,
        cache,
    };

    tokio::spawn(async move {
        shard.start().await.expect("Failed to start shard");
    });

    while let Some(event) = events.next().await {
        tokio::spawn(async move {
            // println!("{:?}", event);
        });
    }

    Ok(())
}

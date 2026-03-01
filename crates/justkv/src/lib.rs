pub mod commands;
pub mod config;
pub mod engine;
pub mod net;
pub mod protocol;

#[global_allocator]
static GLOBAL_ALLOCATOR: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use crate::config::Config;
use crate::net::listener::run_listener;

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_listener(config).await
}

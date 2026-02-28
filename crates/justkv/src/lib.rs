pub mod commands;
pub mod config;
pub mod engine;
pub mod net;
pub mod protocol;

#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

use crate::config::Config;
use crate::net::listener::run_listener;

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_listener(config).await
}

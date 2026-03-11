#![allow(clippy::result_unit_err)]

pub mod auth;
pub mod backup;
pub mod config;
pub mod connection;
pub mod listener;
pub mod logging;
pub mod persistence;

#[global_allocator]
static GLOBAL_ALLOCATOR: betterkv_alloc::BetterKvAllocator = betterkv_alloc::BetterKvAllocator;

pub async fn run(config: config::Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    listener::run_listener(config).await
}

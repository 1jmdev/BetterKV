mod accept;
mod background;
mod shutdown;

use std::io;
use tokio::task::JoinSet;
use tokio::time::Duration;

use crate::auth::AuthService;
use crate::config::Config;
use crate::profile::ProfileHub;
use crate::{backup, backup::SnapshotStats};
use accept::{bind_reuse_port_listeners, run_accept_loop};
use background::{
    spawn_cached_clock_updater, spawn_expiry_sweeper, spawn_periodic_snapshot,
    write_snapshot_with_log,
};
use engine::pubsub::PubSubHub;
use engine::store::Store;
use shutdown::shutdown_signal;

pub async fn run_listener(config: Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_listener_with_profile(config, None).await
}

pub async fn run_listener_with_profile(
    config: Config,
    profile_hub: Option<ProfileHub>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _trace = profiler::scope("server::listener::run_listener");
    let bind_addr = config.addr();
    let listeners = bind_reuse_port_listeners(bind_addr.clone(), config.io_threads).await?;
    let store = Store::new(config.shards);
    let pubsub = PubSubHub::new();
    let profiler = profile_hub.unwrap_or_else(ProfileHub::disabled);
    let auth = AuthService::from_config(&config).map_err(io::Error::other)?;
    let snapshot_path = config.snapshot_path();

    if snapshot_path.exists() {
        match backup::load_snapshot(&store, &snapshot_path).await {
            Ok(stats) => {
                tracing::info!(
                    keys_loaded = stats.keys_loaded,
                    path = %snapshot_path.display(),
                    "loaded snapshot"
                );
            }
            Err(err) => {
                tracing::error!(
                    error = %err,
                    path = %snapshot_path.display(),
                    "failed to load snapshot"
                );
            }
        }
    }

    spawn_expiry_sweeper(
        store.clone(),
        Duration::from_millis(config.sweep_interval_ms),
    );
    spawn_cached_clock_updater(store.clone());
    if config.snapshot_interval_secs > 0 {
        spawn_periodic_snapshot(
            store.clone(),
            snapshot_path.clone(),
            Duration::from_secs(config.snapshot_interval_secs),
        );
    }
    tracing::info!(
        bind = %bind_addr,
        io_threads = config.io_threads,
        sweep_interval_ms = config.sweep_interval_ms,
        snapshot_interval_secs = config.snapshot_interval_secs,
        "listener ready"
    );

    let mut accept_tasks = JoinSet::new();
    for listener in listeners {
        let shared_store = store.clone();
        let shared_pubsub = pubsub.clone();
        let shared_auth = auth.clone();
        let shared_profiler = profiler;
        accept_tasks.spawn(async move {
            run_accept_loop(
                listener,
                shared_store,
                shared_pubsub,
                shared_auth,
                shared_profiler,
            )
            .await
        });
    }

    loop {
        tokio::select! {
            task = accept_tasks.join_next() => {
                let Some(task_result) = task else {
                    return Ok(());
                };

                match task_result {
                    Ok(Ok(())) => {}
                    Ok(Err(err)) => return Err(err),
                    Err(err) => return Err(err.into()),
                }
            }
            _ = shutdown_signal() => {
                tracing::warn!("shutdown signal received");
                if config.snapshot_on_shutdown {
                    let stats = write_snapshot_with_log(store.clone(), snapshot_path.clone(), "shutdown").await;
                    if stats.is_none() {
                        tracing::error!("shutdown snapshot failed");
                    }
                }
                return Ok(());
            }
        }
    }
}

pub(crate) type ListenerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub(crate) type SnapshotResult = Option<SnapshotStats>;

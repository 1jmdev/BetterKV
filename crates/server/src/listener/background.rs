use std::path::PathBuf;

use tokio::task::block_in_place;
use tokio::time::{Duration, sleep};

use crate::listener::SnapshotResult;
use crate::{backup, backup::SnapshotStats};
use engine::store::Store;

pub(crate) fn spawn_expiry_sweeper(store: Store, interval: Duration) {
    let _trace = profiler::scope("server::listener::spawn_expiry_sweeper");
    tokio::spawn(async move {
        loop {
            sleep(interval).await;
            let _ = block_in_place(|| store.sweep_expired());
        }
    });
}

pub(crate) fn spawn_cached_clock_updater(store: Store) {
    let _trace = profiler::scope("server::listener::spawn_cached_clock_updater");
    tokio::spawn(async move {
        loop {
            store.refresh_cached_time();
            sleep(Duration::from_millis(1)).await;
        }
    });
}

pub(crate) fn spawn_periodic_snapshot(store: Store, snapshot_path: PathBuf, interval: Duration) {
    let _trace = profiler::scope("server::listener::spawn_periodic_snapshot");
    tokio::spawn(async move {
        loop {
            sleep(interval).await;
            let _ = write_snapshot_with_log(store.clone(), snapshot_path.clone(), "periodic").await;
        }
    });
}

pub(crate) async fn write_snapshot_with_log(
    store: Store,
    snapshot_path: PathBuf,
    reason: &str,
) -> SnapshotResult {
    let _trace = profiler::scope("server::listener::write_snapshot_with_log");
    let path_for_log = snapshot_path.clone();
    let result =
        tokio::task::spawn_blocking(move || backup::write_snapshot(&store, &snapshot_path)).await;
    match result {
        Ok(Ok(stats)) => {
            log_snapshot_success(stats, &path_for_log, reason);
            Some(stats)
        }
        Ok(Err(err)) => {
            tracing::error!(
                reason,
                error = %err,
                path = %path_for_log.display(),
                "snapshot failed"
            );
            None
        }
        Err(err) => {
            tracing::error!(
                reason,
                error = %err,
                path = %path_for_log.display(),
                "snapshot task failed"
            );
            None
        }
    }
}

fn log_snapshot_success(stats: SnapshotStats, snapshot_path: &std::path::Path, reason: &str) {
    tracing::info!(
        reason,
        keys_written = stats.keys_written,
        bytes_written = stats.bytes_written,
        path = %snapshot_path.display(),
        "snapshot completed"
    );
}

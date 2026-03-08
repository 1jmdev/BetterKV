use std::collections::BTreeSet;

use betterkv_server::config::Config;
use betterkv_server::logging::init_logging;

use crate::cli::{RuntimeArgs, apply_directive, load_config_directives};

pub(crate) fn run(runtime: RuntimeArgs) -> Result<(), String> {
    let _trace = profiler::scope("server::main::run");
    let mut config = Config::default();
    let mut ignored = BTreeSet::new();

    if let Some(config_input) = runtime.config_file {
        let config_directives = load_config_directives(&config_input)?;
        for directive in config_directives {
            apply_directive(
                &directive.name,
                &directive.values,
                &mut config,
                &mut ignored,
            )?;
        }
    }

    for directive in runtime.directives {
        apply_directive(
            &directive.name,
            &directive.values,
            &mut config,
            &mut ignored,
        )?;
    }

    if !ignored.is_empty() {
        let names = ignored.into_iter().collect::<Vec<_>>().join(", ");
        eprintln!("betterkv-server: accepted but ignored directives: {names}");
    }

    let logging_guard = init_logging(&config)?;
    let _ = &logging_guard.file_guard;
    tracing::info!(
        bind = %config.bind,
        port = config.port,
        io_threads = config.io_threads,
        shards = config.shards,
        requirepass = config.requirepass.is_some(),
        acl_users = config.user_directives.len(),
        snapshot_path = %config.snapshot_path().display(),
        snapshot_interval_secs = config.snapshot_interval_secs,
        "starting betterkv server"
    );

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.io_threads)
        .enable_all()
        .build()
        .map_err(|err| format!("failed to create runtime: {err}"))?;

    let result = runtime
        .block_on(betterkv_server::run(config))
        .map_err(|err| format!("server error: {err}"));
    if let Err(err) = &result {
        tracing::error!(error = %err, "server exited with error");
    } else {
        tracing::info!("server shutdown complete");
    }
    result
}

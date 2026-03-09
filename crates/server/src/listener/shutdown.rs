pub(crate) async fn shutdown_signal() {
    let _trace = profiler::scope("server::listener::shutdown_signal");
    #[cfg(unix)]
    {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut sigterm) => {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {},
                    _ = sigterm.recv() => {},
                }
            }
            Err(_) => {
                if let Err(err) = tokio::signal::ctrl_c().await {
                    tracing::warn!(error = %err, "failed to listen for ctrl-c signal");
                }
            }
        }
    }

    #[cfg(not(unix))]
    {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::warn!(error = %err, "failed to listen for ctrl-c signal");
        }
    }
}

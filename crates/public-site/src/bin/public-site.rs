use public_site::{
    Config, prerender,
    runtime::RuntimeConfig,
    server::{Readiness, internal_probe_router, router_with_options},
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = RuntimeConfig::from_env()?;
    let config = Config {
        physician_sign_in: runtime.physician_proxy.is_some(),
        ..Config::from_env()?
    };
    let addr = config.listen_addr;
    let probe_addr = runtime.internal_probe_addr;
    let ready = Readiness::ready();
    let app = router_with_options(prerender(&config)?, runtime, [], ready.clone());
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let probe = if let Some(addr) = probe_addr {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let app = internal_probe_router(ready.clone());
        let mut shutdown = shutdown_rx.clone();
        Some(tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = shutdown.wait_for(|stopped| *stopped).await;
                })
                .await
        }))
    } else {
        None
    };
    let mut public_shutdown = shutdown_rx;
    let public = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = public_shutdown.wait_for(|stopped| *stopped).await;
            })
            .await
    });
    let mut public = public;
    tokio::select! {
        result = &mut public => { ready.stop(); let _ = shutdown_tx.send(true); result??; }
        result = shutdown_signal() => {
            result?;
            ready.stop();
            let _ = shutdown_tx.send(true);
            // Bound connection draining even if an HTTP peer never finishes.
            match tokio::time::timeout(Duration::from_secs(15), &mut public).await {
                Ok(result) => { result??; }
                Err(_) => public.abort(),
            }
        }
    }
    if let Some(mut probe) = probe
        && tokio::time::timeout(Duration::from_secs(1), &mut probe)
            .await
            .is_err()
    {
        probe.abort();
    }
    Ok(())
}

async fn shutdown_signal() -> std::io::Result<()> {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        tokio::select! { result = tokio::signal::ctrl_c() => result, _ = terminate.recv() => Ok(()) }
    }
    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await
}

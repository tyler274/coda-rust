//! Serves the issuer on `PORT` (default 8080).
use std::{net::SocketAddr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `main`'s `Err` prints `Debug`; operators read the message and its cause.
    let config = match langfuse_sso::Config::from_env() {
        Ok(config) => config,
        Err(error) => {
            let mut message = error.to_string();
            let mut cause = std::error::Error::source(&error);
            while let Some(source) = cause {
                message.push_str(&format!(": {source}"));
                cause = source.source();
            }
            eprintln!("langfuse-sso: {message}");
            std::process::exit(1);
        }
    };
    let port: u16 = std::env::var("PORT")
        .ok()
        .filter(|value| !value.is_empty())
        .map_or(Ok(8080), |value| value.parse())?;
    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await?;
    let (stop, mut stopped) = tokio::sync::watch::channel(false);
    let mut server = tokio::spawn(async move {
        axum::serve(listener, langfuse_sso::router(config))
            .with_graceful_shutdown(async move {
                let _ = stopped.wait_for(|stopped| *stopped).await;
            })
            .await
    });
    tokio::select! {
        result = &mut server => result??,
        result = shutdown_signal() => {
            result?;
            let _ = stop.send(true);
            match tokio::time::timeout(Duration::from_secs(15), &mut server).await {
                Ok(result) => result??,
                Err(_) => server.abort(),
            }
        }
    }
    Ok(())
}

async fn shutdown_signal() -> std::io::Result<()> {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    tokio::select! { result = tokio::signal::ctrl_c() => result, _ = terminate.recv() => Ok(()) }
}

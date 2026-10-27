//! IAM-protected Cloud Run adapter for Coda's Workspace SMTP relay.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sender = std::env::var("MAIL_FROM").unwrap_or_else(|_| "signin@codahealth.ai".into());
    let relay = gmail::SmtpRelay::google(&sender, "Coda Health")?;
    let notification_sender = std::env::var("MAIL_NOTIFICATION_FROM")
        .unwrap_or_else(|_| "notifications@codahealth.ai".into());
    let notifications = relay.with_sender(&notification_sender, "Coda Health")?;
    let address = std::env::var("MAIL_LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into());
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, gmail::service::router(relay, notifications))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

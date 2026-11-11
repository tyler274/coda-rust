use std::env;

#[tokio::main]
async fn main() {
    if let Err(reason) = physician_sample::hosting::validate_startup(|key| env::var(key).ok()) {
        eprintln!("{reason}");
        std::process::exit(1);
    }
    let _telemetry = telemetry::init("physician-sample");
    let bind_addr = env::var("PHYSICIAN_LISTEN_ADDR").unwrap_or_else(|_| {
        env::var("PORT")
            .map(|port| format!("0.0.0.0:{port}"))
            .unwrap_or_else(|_| "127.0.0.1:8790".to_owned())
    });
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind {bind_addr}: {err}"));
    tracing::info!(
        "physician site on http://{}",
        listener.local_addr().unwrap()
    );
    physician_sample::serve(listener).await;
}

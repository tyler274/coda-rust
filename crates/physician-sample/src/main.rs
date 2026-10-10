#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8790")
        .await
        .expect("127.0.0.1:8790");
    eprintln!("physician sample on http://127.0.0.1:8790");
    physician_sample::serve(listener).await;
}

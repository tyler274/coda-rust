//! Same-origin assets for the dedicated Duet web build (never the mode shell).
use std::path::PathBuf;

use axum::{
    Router,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};

/// Exposes only known build outputs. Unknown API paths remain 404, not HTML.
pub fn router(directory: PathBuf) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/index.html", get(index))
        .route("/duet-app.js", get(javascript))
        .route("/duet-app_bg.wasm", get(wasm))
        .route("/snippets/{*path}", get(snippet))
        .with_state(directory)
}

async fn asset(directory: PathBuf, name: &str, mime: &'static str) -> Response {
    match tokio::fs::read(directory.join(name)).await {
        Ok(bytes) => ([(header::CONTENT_TYPE, mime)], bytes).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn index(State(directory): State<PathBuf>) -> Response {
    asset(directory, "index.html", "text/html; charset=utf-8").await
}

async fn javascript(State(directory): State<PathBuf>) -> Response {
    asset(directory, "duet-app.js", "text/javascript; charset=utf-8").await
}

async fn wasm(State(directory): State<PathBuf>) -> Response {
    asset(directory, "duet-app_bg.wasm", "application/wasm").await
}

async fn snippet(State(directory): State<PathBuf>, Path(path): Path<String>) -> Response {
    // wasm-bindgen emits nested JS imports. Never let their request escape the
    // immutable asset tree, including through percent-decoded path components.
    if !path.ends_with(".js")
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == ".." || part.contains('\\'))
    {
        return StatusCode::NOT_FOUND.into_response();
    }
    asset(
        directory.join("snippets"),
        &path,
        "text/javascript; charset=utf-8",
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn serves_only_named_assets_with_wasm_mime() {
        let dir = std::env::temp_dir().join(format!("duet-site-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(dir.join("duet-app_bg.wasm"), b"wasm").unwrap();
        std::fs::create_dir_all(dir.join("snippets/interpreter/src")).unwrap();
        std::fs::write(dir.join("snippets/interpreter/src/inline.js"), b"export {}").unwrap();
        let app = router(dir.clone());
        for (path, status) in [
            ("/duet-app_bg.wasm", StatusCode::OK),
            ("/snippets/interpreter/src/inline.js", StatusCode::OK),
            ("/snippets/%2e%2e/secret.js", StatusCode::NOT_FOUND),
            ("/snippets/interpreter/secret.txt", StatusCode::NOT_FOUND),
            ("/index.html", StatusCode::NOT_FOUND),
            ("/api/missing", StatusCode::NOT_FOUND),
            ("/../Cargo.toml", StatusCode::NOT_FOUND),
        ] {
            let response = app
                .clone()
                .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), status);
            if path.ends_with(".wasm") && status == StatusCode::OK {
                assert_eq!(response.headers()[header::CONTENT_TYPE], "application/wasm");
            }
        }
        std::fs::remove_dir_all(dir).unwrap();
    }
}

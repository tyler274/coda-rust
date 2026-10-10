//! Read-only physician projections for the Roger synthetic practice.
//!
//! The FHIR corpus stays in coda-agents. These files are the physician JSON the
//! React site already renders. Commands are rejected here; they do not run a model.
//! The same server serves the Dioxus page, which mounts into `#main`.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::Query;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use serde_json::{Value, json};

const INDEX: &str = include_str!("../../physician-app/index.html");

const TODAY: &str = include_str!("../fixtures/today.json");
const PRACTICE_DAY: &str = include_str!("../fixtures/practice-day.json");
const UPDATES: &str = include_str!("../fixtures/updates.json");
const RECENT: &str = include_str!("../fixtures/recent-conversations.json");
const WORKSPACES: &str = include_str!("../fixtures/patient-workspaces.json");
const ROSTER: &str = include_str!("../fixtures/roster.json");

const WORKSPACE_IDS: [&str; 5] = [
    "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee",
    "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d",
    "pw-f5404d89-acde-4bb5-8c42-2fa191766811",
    "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971",
    "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e",
];

pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { Json(json!({"status": "ok"})) }))
        .route("/today", get(today))
        .route("/practice-day", get(|| async { json_body(PRACTICE_DAY) }))
        .route("/updates", get(|| async { json_body(UPDATES) }))
        .route("/recent-conversations", get(|| async { json_body(RECENT) }))
        .route(
            "/patient-workspaces",
            get(|| async { json_body(WORKSPACES) }),
        )
        .route("/patient-workspaces/{id}", get(workspace))
        .route("/patient-roster", get(roster))
        .route("/commands", post(reject_command))
        .route("/dictation", post(|| async { dictation_unavailable() }))
        .route(
            "/physician-state/drafts/{workspace}/{chat}",
            get(empty_draft).put(draft_not_saved),
        )
        .route(
            "/patient-workspaces/{id}/chats/{chat}/snapshot",
            get(empty_snapshot),
        )
        .route("/", get(site_index))
        .route("/index.html", get(site_index))
        .fallback(site_file)
        .layer(middleware::from_fn(cors))
}

/// Directory of `wasm-bindgen --target web` output. `PHYSICIAN_SITE_DIR` wins,
/// then `target/physician-web` beside the workspace.
pub fn site_dir() -> Option<PathBuf> {
    if let Some(dir) = env::var_os("PHYSICIAN_SITE_DIR") {
        let path = PathBuf::from(dir);
        if path.is_dir() {
            return Some(path);
        }
    }
    let fallback = PathBuf::from("target/physician-web");
    fallback.is_dir().then_some(fallback)
}

async fn site_index() -> Response {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        INDEX,
    )
        .into_response()
}

async fn empty_snapshot(
    axum::extract::Path((id, _chat)): axum::extract::Path<(String, String)>,
) -> Response {
    if workspace_body(&id).is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    Json(json!({
        "snapshot": { "conversation": [] }
    }))
    .into_response()
}

async fn site_file(request: Request<Body>) -> Response {
    let Some(dir) = site_dir() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let path = request.uri().path().trim_start_matches('/');
    if path.is_empty() || path.contains("..") || path.contains('\\') {
        return StatusCode::NOT_FOUND.into_response();
    }
    let file = dir.join(path);
    if !file.starts_with(&dir) {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Ok(bytes) = fs::read(&file) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(mime) = site_mime(&file) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, mime)],
        bytes,
    )
        .into_response()
}

fn site_mime(file: &Path) -> Option<&'static str> {
    match file.extension().and_then(|ext| ext.to_str()) {
        Some("js") => Some("text/javascript"),
        Some("wasm") => Some("application/wasm"),
        Some("png") => Some("image/png"),
        Some("css") => Some("text/css"),
        _ => None,
    }
}

pub async fn serve(listener: tokio::net::TcpListener) {
    axum::serve(listener, router())
        .await
        .expect("sample server");
}

async fn today(Query(query): Query<TodayQuery>) -> Response {
    if let Some(id) = query.patient_workspace_id {
        return match patient_today(&id) {
            Some(body) => json_body(body),
            None => StatusCode::NOT_FOUND.into_response(),
        };
    }
    json_body(TODAY)
}

async fn workspace(axum::extract::Path(id): axum::extract::Path<String>) -> Response {
    match workspace_body(&id) {
        Some(body) => json_body(body),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn roster(Query(query): Query<RosterQuery>) -> Response {
    let patients = search_roster(&query.query.unwrap_or_default());
    Json(json!({ "patients": patients, "next_after": null })).into_response()
}

async fn reject_command(body: Option<Json<Value>>) -> Response {
    let command_type = body
        .as_ref()
        .and_then(|json| json.0.get("type"))
        .and_then(Value::as_str)
        .unwrap_or("");
    (
        StatusCode::OK,
        Json(json!({
            "outcome": "rejected",
            "message": rejection_message(command_type),
        })),
    )
        .into_response()
}

async fn empty_draft() -> Response {
    Json(json!({ "revision": 0, "text": "" })).into_response()
}

async fn draft_not_saved() -> Response {
    StatusCode::CONFLICT.into_response()
}

fn dictation_unavailable() -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        "Dictation is unavailable. Check microphone support and server configuration.",
    )
        .into_response()
}

fn json_body(body: &'static str) -> Response {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        body,
    )
        .into_response()
}

#[derive(Deserialize)]
struct TodayQuery {
    patient_workspace_id: Option<String>,
}

#[derive(Deserialize)]
struct RosterQuery {
    query: Option<String>,
}

pub fn search_roster(query: &str) -> Vec<Value> {
    let words: Vec<String> = query
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .filter(|word| !word.is_empty())
        .collect();
    if words.is_empty() {
        return Vec::new();
    }
    let roster: Value = serde_json::from_str(ROSTER).expect("roster");
    roster["patients"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|patient| {
            let name = patient["patient_display_name"].as_str().unwrap_or("");
            let birth = patient["birth_date"].as_str().unwrap_or("");
            let haystack = format!("{name} {birth}").to_lowercase();
            words.iter().all(|word| haystack.contains(word))
        })
        .take(50)
        .collect()
}

pub fn rejection_message(command_type: &str) -> &'static str {
    match command_type {
        "message.submit" => "Coda couldn’t accept that message.",
        "message.withdraw" => "Coda couldn’t remove that queued message.",
        "work.resume" => "Coda couldn’t resume that work.",
        "work.done" => "Coda couldn’t mark that done.",
        "chat.open" => "Coda couldn’t open that chat.",
        "discussion.open" => "Coda couldn’t open that discussion.",
        "run.cancel" => "Coda couldn’t stop that work.",
        "home.clear" => "Coda couldn’t clear that item.",
        "reply.close" | "reply.use_update" => "Coda couldn’t change that reply.",
        "reply.reopen" => "Coda couldn’t reopen that reply.",
        "reply.request" => "Coda couldn’t ask for that reply.",
        "reply.revise" => "Coda couldn’t revise that reply.",
        "follow_up.review" => "Coda couldn’t review that follow-up now.",
        "follow_up.end" => "Coda couldn’t end that follow-up.",
        "conversation.archive" => "Coda couldn’t archive that conversation.",
        "conversation.restore" => "Coda couldn’t restore that conversation.",
        _ => "Coda couldn’t confirm that request. Reconnect to check its status.",
    }
}

fn workspace_body(id: &str) -> Option<&'static str> {
    Some(match id {
        "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee" => {
            include_str!("../fixtures/workspaces/pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee.json")
        }
        "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d" => {
            include_str!("../fixtures/workspaces/pw-82c44e9b-58c0-4af1-bd21-771bf77e101d.json")
        }
        "pw-f5404d89-acde-4bb5-8c42-2fa191766811" => {
            include_str!("../fixtures/workspaces/pw-f5404d89-acde-4bb5-8c42-2fa191766811.json")
        }
        "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971" => {
            include_str!("../fixtures/workspaces/pw-d11a35f7-0d8d-466f-a48e-5812df0a4971.json")
        }
        "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e" => {
            include_str!("../fixtures/workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e.json")
        }
        _ => return None,
    })
}

fn patient_today(id: &str) -> Option<&'static str> {
    if !WORKSPACE_IDS.contains(&id) {
        return None;
    }
    Some(match id {
        "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee" => {
            include_str!(
                "../fixtures/today-by-patient/pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee.json"
            )
        }
        "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d" => {
            include_str!(
                "../fixtures/today-by-patient/pw-82c44e9b-58c0-4af1-bd21-771bf77e101d.json"
            )
        }
        "pw-f5404d89-acde-4bb5-8c42-2fa191766811" => {
            include_str!(
                "../fixtures/today-by-patient/pw-f5404d89-acde-4bb5-8c42-2fa191766811.json"
            )
        }
        "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971" => {
            include_str!(
                "../fixtures/today-by-patient/pw-d11a35f7-0d8d-466f-a48e-5812df0a4971.json"
            )
        }
        "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e" => {
            include_str!(
                "../fixtures/today-by-patient/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e.json"
            )
        }
        _ => return None,
    })
}

async fn cors(request: Request<Body>, next: Next) -> Response {
    if request.method() == Method::OPTIONS {
        return cors_headers(StatusCode::NO_CONTENT.into_response());
    }
    cors_headers(next.run(request).await)
}

fn cors_headers(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, OPTIONS"),
    );
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("content-type"),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    async fn get(path: &str) -> (StatusCode, String) {
        let response = router()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
            .await
            .unwrap();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn serves_the_recorded_practice() {
        let (status, today) = get("/today").await;
        assert_eq!(status, StatusCode::OK);
        assert!(today.contains("not_yet_read"));
        let (status, day) = get("/practice-day").await;
        assert_eq!(status, StatusCode::OK);
        assert!(day.contains("Mary Louis"));
        let (status, mary) =
            get("/patient-workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e").await;
        assert_eq!(status, StatusCode::OK);
        assert!(mary.contains("authored_text"));
        let (status, found) = get("/patient-roster?query=Mary").await;
        assert_eq!(status, StatusCode::OK);
        assert!(found.contains("1972-03-18"));
        let (status, updates) = get("/updates").await;
        assert_eq!(status, StatusCode::OK);
        assert!(updates.contains("\"rows\":[]"));
        let (status, page) = get("/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(page.contains(r#"<main id="main"></main>"#));
        let (status, snapshot) =
            get("/patient-workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e/chats/none/snapshot")
                .await;
        assert_eq!(status, StatusCode::OK);
        assert!(snapshot.contains("\"conversation\":[]"));
    }

    #[tokio::test]
    async fn commands_are_rejected_without_a_model() {
        let response = router()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/commands")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"type":"work.resume"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 4_000)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["outcome"], "rejected");
        assert_eq!(body["message"], "Coda couldn’t resume that work.");
    }
}

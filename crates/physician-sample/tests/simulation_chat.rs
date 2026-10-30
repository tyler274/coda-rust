//! The local simulation's chat says who wrote each turn, so the page never
//! attributes the physician's words, or its own acknowledgement, to Coda.

use axum::body::{Body, to_bytes};
use axum::http::{Request, header};
use serde_json::{Value, json};
use tower::ServiceExt;

const WORKSPACE: &str = "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee";

#[tokio::test]
async fn a_sent_message_comes_back_as_the_physicians_and_the_acknowledgement_unattributed() {
    let app = physician_sample::simulation_router();
    let sent = app
        .clone()
        .oneshot(
            Request::post("/commands")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "type": "message.submit",
                        "payload": {
                            "patient_workspace_id": WORKSPACE,
                            "chat_id": "chat-1",
                            "text": "Hold the metoprolol until Friday.",
                        },
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let sent: Value =
        serde_json::from_slice(&to_bytes(sent.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(sent["outcome"], "accepted");

    let snapshot = app
        .oneshot(
            Request::get(format!(
                "/patient-workspaces/{WORKSPACE}/chats/chat-1/snapshot"
            ))
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();
    let snapshot: Value =
        serde_json::from_slice(&to_bytes(snapshot.into_body(), usize::MAX).await.unwrap()).unwrap();
    let turns = snapshot["snapshot"]["conversation"].as_array().unwrap();
    assert_eq!(turns.len(), 2);
    assert_eq!(turns[0]["kind"], "physician");
    assert_eq!(turns[0]["text"], "Hold the metoprolol until Friday.");
    assert!(turns[1].get("kind").is_none());
}

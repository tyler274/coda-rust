//! Read-only checks against the Spruce test organization.
//!
//! Ignored so routine tests never call Spruce. Run, from the repo, in the Nix shell:
//!
//! ```text
//! doppler run --project coda-rust --config dev -- \
//!   cargo test -p spruce --test live -- --ignored --nocapture
//! ```
//!
//! `SPRUCE_OBSERVE=before` writes `target/verification/spruce/before.json`.
//! After messages are created, edited and deleted by hand in Spruce,
//! `SPRUCE_OBSERVE=after` writes `after.json` and prints what changed.
//! Neither file is committed. The test returns without calling Spruce when
//! `SPRUCE_API_KEY` is unset.

use std::path::PathBuf;

use spruce::api::{Client, Error, ListQuery};
use spruce::changes::{ConversationRead, Observed, Reader};
use spruce::payload::{ConversationEntry, ItemEntry};

fn credential() -> Option<String> {
    std::env::var("SPRUCE_API_KEY")
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn output_dir() -> PathBuf {
    let base = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target"));
    let dir = base.join("verification/spruce");
    std::fs::create_dir_all(&dir).expect("verification directory");
    dir
}

#[tokio::test]
#[ignore = "reads the Spruce test organization"]
async fn access() {
    let Some(credential) = credential() else {
        eprintln!("SPRUCE_API_KEY unset; skipping the live Spruce access check");
        return;
    };
    let client = Client::new(credential).expect("credential");
    let organization = client.organization().await.expect("organization");
    if let Ok(expected) = std::env::var("SPRUCE_ORGANIZATION_ID") {
        assert_eq!(organization.id, expected, "organization id");
    }
    assert!(
        organization
            .name
            .as_deref()
            .is_some_and(|name| !name.is_empty())
    );
    let endpoints = client
        .internal_endpoints()
        .await
        .expect("internal endpoints");
    eprintln!(
        "spruce organization {} ({}) internal endpoints {}",
        organization.id,
        organization.name.unwrap_or_default(),
        endpoints.len()
    );
    assert!(
        !endpoints.is_empty(),
        "the test organization has an internal endpoint"
    );
}

#[tokio::test]
#[ignore = "snapshots the Spruce test organization"]
async fn observe() {
    let Some(credential) = credential() else {
        eprintln!("SPRUCE_API_KEY unset; skipping the live Spruce snapshot");
        return;
    };
    let Ok(phase) = std::env::var("SPRUCE_OBSERVE") else {
        eprintln!("SPRUCE_OBSERVE unset; skipping the snapshot (set before or after)");
        return;
    };
    assert!(
        phase == "before" || phase == "after",
        "SPRUCE_OBSERVE must be before or after"
    );
    let client = Client::new(credential).expect("credential");
    let organization = client.organization().await.expect("organization");
    let snapshot = snapshot(&client, &organization.id).await;
    let dir = output_dir();
    let path = dir.join(format!("{phase}.json"));
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&snapshot).expect("snapshot json"),
    )
    .expect("write snapshot");
    eprintln!(
        "wrote {} conversations to {}",
        snapshot["conversations"]
            .as_array()
            .map(Vec::len)
            .unwrap_or(0),
        path.display()
    );

    let stale = stale_token(&client).await;
    eprintln!("pagination token: {stale}");
    let mut findings = format!("phase {phase}\norganization {}\n{stale}\n", organization.id);

    if phase == "after" {
        let before_path = dir.join("before.json");
        let before: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&before_path).unwrap_or_else(|_| {
                panic!(
                    "run with SPRUCE_OBSERVE=before before comparing ({})",
                    before_path.display()
                )
            }))
            .expect("before snapshot");
        let report = compare(&client, &before, &snapshot).await;
        eprintln!("{report}");
        findings.push_str(&report);
    }
    std::fs::write(dir.join("findings.txt"), findings).expect("findings");
}

async fn snapshot(client: &Client, organization_id: &str) -> serde_json::Value {
    let mut conversations = Vec::new();
    let mut token: Option<String> = None;
    for _ in 0..50 {
        let query = match token.as_deref() {
            Some(page) => ListQuery {
                page_size: 20,
                start_from: None,
                pagination_token: Some(page),
            },
            None => ListQuery::first(20),
        };
        let page = client
            .list_conversations(query)
            .await
            .expect("conversations");
        for entry in page.entries {
            let ConversationEntry::Conversation(conversation) = entry else {
                conversations.push(serde_json::json!({"malformed": true}));
                continue;
            };
            conversations.push(serde_json::json!({
                "id": conversation.id,
                "kind": conversation.kind,
                "last_message_at": conversation.last_message_at,
                "items": conversation_items(client, &conversation.id).await,
            }));
        }
        if !page.has_more {
            break;
        }
        token = page.pagination_token;
    }
    serde_json::json!({"organization_id": organization_id, "conversations": conversations})
}

async fn conversation_items(client: &Client, id: &str) -> Vec<serde_json::Value> {
    let reader = Reader::new(client.clone());
    let mut page = None;
    let mut items = Vec::new();
    for _ in 0..50 {
        match reader
            .read_conversation(id, page.as_deref())
            .await
            .expect("items")
        {
            ConversationRead::Messages {
                items: observed,
                next_page,
                undecodable,
            } => {
                for observed in observed {
                    items.push(match observed {
                        Observed::Message(item) => serde_json::json!({
                            "id": item.id(),
                            "created_at": item.envelope.created_at,
                            "modified_at": item.envelope.modified_at,
                            "version": item.version(),
                            "deleted": item.deleted,
                            "direction": item.envelope.direction,
                            "text_len": item.text.len(),
                        }),
                        Observed::Unreadable(envelope) => serde_json::json!({
                            "id": envelope.id,
                            "unreadable": true,
                            "created_at": envelope.created_at,
                            "modified_at": envelope.modified_at,
                        }),
                    });
                }
                if undecodable {
                    items.push(serde_json::json!({"malformed": true}));
                }
                match next_page {
                    Some(next) => page = Some(next),
                    None => break,
                }
            }
            ConversationRead::Missing => {
                items.push(serde_json::json!({"missing": true}));
                break;
            }
            ConversationRead::Undecodable => {
                items.push(serde_json::json!({"undecodable": true}));
                break;
            }
            ConversationRead::ReconnectRequired => panic!("credential refused during the snapshot"),
            ConversationRead::Unavailable => {
                items.push(serde_json::json!({"unavailable": true}));
                break;
            }
        }
    }
    items
}

async fn stale_token(client: &Client) -> String {
    let unknown = match client
        .list_conversations(ListQuery {
            page_size: 1,
            start_from: None,
            pagination_token: Some("expired-observation-token"),
        })
        .await
    {
        Ok(page) => format!(
            "unknown token accepted, {} entries, has_more {}",
            page.entries.len(),
            page.has_more
        ),
        Err(Error::Rejected { status }) => format!("unknown token rejected ({status})"),
        Err(Error::Temporary { status }) => format!("unknown token temporary ({status})"),
        Err(Error::NotFound) => "unknown token not found".to_owned(),
        Err(Error::Contract) => "unknown token unreadable".to_owned(),
        Err(Error::AuthorizationRejected) => "unknown token refused the credential".to_owned(),
        Err(Error::Unavailable) => "unknown token did not answer".to_owned(),
        Err(Error::InvalidRequest) => "unknown token was not sent".to_owned(),
    };
    let fresh = match client.list_conversations(ListQuery::first(1)).await {
        Ok(page) if page.has_more => {
            let token = page.pagination_token.clone().unwrap_or_default();
            match client
                .list_conversations(ListQuery {
                    page_size: 1,
                    start_from: None,
                    pagination_token: Some(token.as_str()),
                })
                .await
            {
                Ok(next) => format!("fresh token reused, {} entries", next.entries.len()),
                Err(error) => format!("fresh token refused on reuse ({error})"),
            }
        }
        Ok(_) => "one page, no token to reuse".to_owned(),
        Err(error) => format!("first page failed ({error})"),
    };
    format!("{fresh}; {unknown}")
}

async fn compare(client: &Client, before: &serde_json::Value, after: &serde_json::Value) -> String {
    let mut lines = Vec::new();
    let before_rows = rows(before);
    let after_rows = rows(after);
    for (id, earlier) in &before_rows {
        match after_rows.get(id) {
            None => lines.push(format!("conversation {id} gone")),
            Some(later) => {
                if earlier.last_message_at != later.last_message_at {
                    lines.push(format!(
                        "conversation {id} lastMessageAt {} -> {}",
                        display(&earlier.last_message_at),
                        display(&later.last_message_at)
                    ));
                }
                for (item_id, item) in &earlier.items {
                    match later.items.get(item_id) {
                        None => {
                            let appearance = item_appearance(client, item_id).await;
                            lines.push(format!("item {item_id} absent from the later read ({appearance})"));
                        }
                        Some(next) if next.version != item.version || next.deleted != item.deleted => lines.push(format!(
                            "item {item_id} version changed, deleted {} -> {}, modified {} -> {}",
                            item.deleted,
                            next.deleted,
                            display(&item.modified_at),
                            display(&next.modified_at)
                        )),
                        Some(_) => {}
                    }
                }
                for item_id in later.items.keys() {
                    if !earlier.items.contains_key(item_id) {
                        lines.push(format!("item {item_id} is new"));
                    }
                }
            }
        }
    }
    for id in after_rows.keys() {
        if !before_rows.contains_key(id) {
            lines.push(format!("conversation {id} is new"));
        }
    }
    if lines.is_empty() {
        lines.push(
            "no conversation, item, edit or deletion changed between the snapshots".to_owned(),
        );
    }
    lines.join("\n")
}

async fn item_appearance(client: &Client, id: &str) -> String {
    match client.get_item(id).await {
        Err(Error::NotFound) => "404".to_owned(),
        Ok(ItemEntry::Item(item)) if item.deleted => "still returned, marked deleted".to_owned(),
        Ok(ItemEntry::Item(_)) => "still returned, not marked deleted".to_owned(),
        Ok(ItemEntry::Unreadable(_)) => "still returned, unreadable".to_owned(),
        Ok(ItemEntry::Malformed) => "still returned, no id".to_owned(),
        Err(error) => format!("error {error}"),
    }
}

struct Row {
    last_message_at: Option<String>,
    items: std::collections::BTreeMap<String, ItemRow>,
}

struct ItemRow {
    version: Option<String>,
    deleted: bool,
    modified_at: Option<String>,
}

fn rows(snapshot: &serde_json::Value) -> std::collections::BTreeMap<String, Row> {
    let mut rows = std::collections::BTreeMap::new();
    for conversation in snapshot
        .get("conversations")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
    {
        let Some(id) = conversation.get("id").and_then(|value| value.as_str()) else {
            continue;
        };
        let mut items = std::collections::BTreeMap::new();
        for item in conversation
            .get("items")
            .and_then(|value| value.as_array())
            .into_iter()
            .flatten()
        {
            let Some(item_id) = item.get("id").and_then(|value| value.as_str()) else {
                continue;
            };
            items.insert(
                item_id.to_owned(),
                ItemRow {
                    version: item
                        .get("version")
                        .and_then(|value| value.as_str())
                        .map(str::to_owned),
                    deleted: item
                        .get("deleted")
                        .and_then(|value| value.as_bool())
                        .unwrap_or(false),
                    modified_at: item
                        .get("modified_at")
                        .and_then(|value| value.as_str())
                        .map(str::to_owned),
                },
            );
        }
        rows.insert(
            id.to_owned(),
            Row {
                last_message_at: conversation
                    .get("last_message_at")
                    .and_then(|value| value.as_str())
                    .map(str::to_owned),
                items,
            },
        );
    }
    rows
}

fn display(value: &Option<String>) -> String {
    value.clone().unwrap_or_else(|| "none".to_owned())
}

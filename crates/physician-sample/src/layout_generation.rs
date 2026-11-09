//! Makes a source's layout for one record type the first time a record of that type from that
//! source is cited, and saves it for every later citation. The model sees the record's type, the
//! cited record as a sample and the layout schema, and replies with a layout: headings, labels,
//! roles and pointers, never a value. The layout is checked against that sample before it is
//! saved, so a layout that is refused, or a model call that fails, saves nothing, and the record
//! type keeps reading through the general view.
use std::collections::HashSet;
use std::fmt;
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::Duration;

use harness::engine::{Context, Item, Limits, Provider, Request};
use local_product::physician_chat::Citation;
use local_product::physician_layouts::{LayoutKey, LayoutStoreError, Saved};
use local_product::runtime::Store;
use serde_json::{Value, json};

use crate::source_layout::{Layout, Refusal};

/// What the model is told: its task, the roles and the rule that it never writes a value.
pub const INSTRUCTIONS: &str = include_str!("layout_generation.md");

/// The JSON schema of [`Layout`], sent to the model with each sample record.
pub const LAYOUT_SCHEMA: &str = include_str!("source_layout.schema.json");

static SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(LAYOUT_SCHEMA).expect("source_layout.schema.json is JSON")
});

/// Bounds for the one model call that makes a layout. The request carries one retained record,
/// which the chart reader caps at 64 KiB, beside the schema and instructions. The call ends
/// inside the worker's 100 s layout phase.
pub fn limits() -> Limits {
    Limits {
        model_calls: 1,
        tool_calls: 0,
        calls_per_turn: 0,
        request_bytes: 262_144,
        response_bytes: 131_072,
        tool_result_bytes: 0,
        total_tool_result_bytes: 0,
        output_tokens: 16_384,
        elapsed: Duration::from_secs(90),
    }
}

/// How a layout request ended without saving anything new, short of failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The key already had a layout, so the model was not asked.
    AlreadyStored,
    /// The model's layout passed its checks and is stored now.
    Stored {
        /// SHA-256 of the stored layout.
        sha256: String,
    },
    /// Another writer stored these exact bytes first.
    Unchanged,
    /// Another writer stored a different layout first; that one stays.
    LostRace {
        /// SHA-256 of the layout the key holds.
        stored_sha256: String,
    },
}

/// Why no layout was saved. The record type keeps reading through the general view.
#[derive(Debug)]
pub enum Unsaved {
    /// The cited record is not JSON, so there is nothing for pointers to bind.
    RecordNotJson(serde_json::Error),
    /// The request would exceed its byte bound.
    RequestTooLarge {
        /// The request's serialized size.
        bytes: usize,
    },
    /// The model call failed or returned something other than one text reply.
    Model(String),
    /// The model call ran past its deadline.
    TimedOut,
    /// The reply is not a layout.
    NotALayout(serde_json::Error),
    /// The layout does not read the record it was made from.
    Refused(Refusal),
    /// The layout store could not be read or written.
    Store(LayoutStoreError),
}

impl Unsaved {
    /// Why nothing was saved, as a fixed name that logs can carry.
    pub fn category(&self) -> &'static str {
        match self {
            Self::RecordNotJson(_) => "record_not_json",
            Self::RequestTooLarge { .. } => "request_too_large",
            Self::Model(_) => "model",
            Self::TimedOut => "timed_out",
            Self::NotALayout(_) => "not_a_layout",
            Self::Refused(_) => "refused",
            Self::Store(_) => "store",
        }
    }
}

/// Names no record or model text: a model failure's reason is the provider adapter's own fixed
/// wording, and a refusal's or JSON error's detail stays in its source (see
/// [`crate::source_layout::Loggable`]).
impl fmt::Display for Unsaved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecordNotJson(_) => f.write_str("the cited record is not JSON"),
            Self::RequestTooLarge { bytes } => {
                write!(f, "the layout request is {bytes} bytes, over its bound")
            }
            Self::Model(reason) => write!(f, "the layout model call failed: {reason}"),
            Self::TimedOut => f.write_str("the layout model call ran past its deadline"),
            Self::NotALayout(_) => f.write_str("the model's reply is not a layout"),
            Self::Refused(_) => f.write_str("the model's layout was refused by its checks"),
            Self::Store(_) => f.write_str("the layout store failed"),
        }
    }
}

impl std::error::Error for Unsaved {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RecordNotJson(error) | Self::NotALayout(error) => Some(error),
            Self::Refused(refusal) => Some(refusal),
            Self::Store(error) => Some(error),
            Self::RequestTooLarge { .. } | Self::Model(_) | Self::TimedOut => None,
        }
    }
}

/// The citations whose `(source, record type)` a layout would read, the first of each key in the
/// order cited. A citation retained without its source has no key and is left out.
pub fn first_of_each_key(citations: &[Citation]) -> Vec<(LayoutKey<'_>, &Citation)> {
    let mut seen = HashSet::new();
    citations
        .iter()
        .filter_map(|citation| {
            let key = LayoutKey {
                source_id: citation.source_id.as_deref()?,
                record_type: &citation.record_type,
            };
            seen.insert((key.source_id, key.record_type))
                .then_some((key, citation))
        })
        .collect()
}

/// Keys this process is making a layout for right now, so two answers citing the same new key
/// on one instance ask the model once. Writers on other instances settle through the store,
/// where the first layout stored wins.
#[derive(Debug, Default)]
pub struct InFlight(Mutex<HashSet<(String, String)>>);

/// This process's hold on one key while it makes that key's layout; released on drop.
#[derive(Debug)]
pub struct Held<'a> {
    in_flight: &'a InFlight,
    key: (String, String),
}

impl InFlight {
    /// Holds `key` unless another task on this process already does. Only the task that
    /// inserted the key gets a [`Held`], so only it releases the key.
    pub fn hold(&self, key: LayoutKey<'_>) -> Option<Held<'_>> {
        let key = (key.source_id.to_owned(), key.record_type.to_owned());
        let inserted = self
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(key.clone());
        if !inserted {
            return None;
        }
        Some(Held {
            in_flight: self,
            key,
        })
    }
}

impl Drop for Held<'_> {
    fn drop(&mut self) {
        self.in_flight
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(&self.key);
    }
}

/// Makes and saves `key`'s layout from `cited` unless the key already has one. `attempt_id`
/// names the model call for the provider's own records.
pub async fn ensure_layout<P: Provider + ?Sized>(
    store: &Store,
    provider: &mut P,
    key: LayoutKey<'_>,
    cited: &Citation,
    attempt_id: &str,
) -> Result<Outcome, Unsaved> {
    if store
        .layout_for(key)
        .await
        .map_err(Unsaved::Store)?
        .is_some()
    {
        return Ok(Outcome::AlreadyStored);
    }
    let record: Value = serde_json::from_str(&cited.content).map_err(Unsaved::RecordNotJson)?;
    let reply = request_layout(provider, key.record_type, &record, attempt_id).await?;
    let layout = Layout::parse(&reply).map_err(Unsaved::NotALayout)?;
    layout.validate(&record).map_err(Unsaved::Refused)?;
    match store.save_layout(key, &reply).await {
        Ok(Saved::Stored(stored)) => Ok(Outcome::Stored {
            sha256: stored.sha256,
        }),
        Ok(Saved::Unchanged(_)) => Ok(Outcome::Unchanged),
        Err(LayoutStoreError::Conflict { stored_sha256 }) => {
            Ok(Outcome::LostRace { stored_sha256 })
        }
        Err(error) => Err(Unsaved::Store(error)),
    }
}

/// One model call for a layout of `record_type`, with `record` as its sample. Returns the reply's
/// text, trimmed and otherwise exactly as the model wrote it.
async fn request_layout<P: Provider + ?Sized>(
    provider: &mut P,
    record_type: &str,
    record: &Value,
    attempt_id: &str,
) -> Result<String, Unsaved> {
    let sample = json!({
        "record_type": record_type,
        "sample_record": record,
        "layout_schema": *SCHEMA,
    });
    let context = Context {
        instructions: INSTRUCTIONS.to_owned(),
        items: vec![Item::Message {
            role: "user".into(),
            text: sample.to_string(),
        }],
        tools: Vec::new(),
    };
    let limits = limits();
    let request = Request {
        attempt_id,
        ordinal: 1,
        context: &context,
        limits: &limits,
    };
    let bytes = provider.request_bytes(&request).map_err(Unsaved::Model)?;
    if bytes > limits.request_bytes {
        return Err(Unsaved::RequestTooLarge { bytes });
    }
    let turn = tokio::time::timeout(limits.elapsed, provider.complete(request))
        .await
        .map_err(|_elapsed| Unsaved::TimedOut)?
        .map_err(Unsaved::Model)?;
    let mut text = String::new();
    for item in turn.items {
        match item {
            Item::Message { role, text: part } if role == "assistant" => text.push_str(&part),
            // Reasoning and other continuation metadata carry no reply.
            Item::Opaque(_) => {}
            Item::Message { .. } | Item::Call(_) | Item::Result { .. } => {
                return Err(Unsaved::Model("the reply is not one text message".into()));
            }
        }
    }
    let text = text.trim();
    if text.is_empty() {
        return Err(Unsaved::Model("the reply has no text".into()));
    }
    Ok(text.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::live::{cited_layouts, durable_snapshot_body};
    use crate::source_inspection::inspection;
    use crate::source_inspection::tests::{general_citation, observation};
    use crate::source_layout::{LayoutRefusal, RoleMismatchReason, SlotRole};
    use harness::engine::Turn;
    use local_product::runtime::{Conversation, Message, async_trait};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    const LAYOUT: &str = r#"{"record_type":"Observation",
        "statuses":[{"role":"status","pointer":"/status"}],
        "sections":[{"heading":"result","slots":[
            {"role":"code","pointer":"/code/text"},
            {"role":"result","pointer":"/valueQuantity/value"},
            {"role":"unit","pointer":"/valueQuantity/unit"}]}]}"#;

    /// A scripted layout model: each call returns its reply, or fails, and is counted.
    struct Scripted {
        reply: Result<String, String>,
        calls: Arc<AtomicU32>,
        seen: Option<Context>,
    }

    impl Scripted {
        fn replying(reply: &str) -> Self {
            Self {
                reply: Ok(reply.to_owned()),
                calls: Arc::default(),
                seen: None,
            }
        }

        fn failing(reason: &str) -> Self {
            Self {
                reply: Err(reason.to_owned()),
                calls: Arc::default(),
                seen: None,
            }
        }
    }

    #[async_trait]
    impl Provider for Scripted {
        fn request_bytes(&self, request: &Request<'_>) -> Result<usize, String> {
            Ok(serde_json::to_vec(request.context)
                .map_err(|e| e.to_string())?
                .len())
        }

        async fn complete(&mut self, request: Request<'_>) -> Result<Turn, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.seen = Some(request.context.clone());
            let text = self.reply.clone()?;
            Ok(Turn {
                items: vec![
                    Item::Opaque(json!({"type":"reasoning"})),
                    Item::Message {
                        role: "assistant".into(),
                        text,
                    },
                ],
                receipt: json!({"provider":"scripted"}),
            })
        }
    }

    async fn store() -> Store {
        let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL")
            .expect("PHYSICIAN_TEST_DATABASE_URL required for the layout generation proof");
        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        store
    }

    /// A cited observation from its own new source, so each test has a key with no layout.
    fn cited() -> Citation {
        let mut cited = general_citation(&observation());
        cited.id = "cited".into();
        cited.source_id = Some(format!("ehr-{}", uuid::Uuid::new_v4()));
        cited
    }

    fn key(cited: &Citation) -> LayoutKey<'_> {
        LayoutKey {
            source_id: cited.source_id.as_deref().unwrap(),
            record_type: &cited.record_type,
        }
    }

    /// The citation's inspection in the snapshot of an answer that cites it.
    async fn snapshot_view(store: &Store, cited: &Citation) -> Value {
        let conversation = Conversation {
            messages: vec![Message {
                role: "assistant".into(),
                text: format!("[[patient_source:{}]]", cited.id),
                citations: vec![cited.clone()],
            }],
            ..Default::default()
        };
        let layouts = cited_layouts(store, &conversation).await;
        let body = durable_snapshot_body(Some(&conversation), &layouts);
        body["conversation"][0]["segments"]
            .as_array()
            .unwrap()
            .iter()
            .find(|segment| segment["kind"] == "citation")
            .map(|segment| segment["source_presentation"]["inspection"].clone())
            .unwrap()
    }

    fn general(cited: &Citation) -> Value {
        serde_json::to_value(inspection(cited)).unwrap()
    }

    #[test]
    fn the_schema_is_json_and_names_every_role() {
        let roles = SCHEMA["$defs"]["slot"]["properties"]["role"]["enum"]
            .as_array()
            .unwrap();
        assert_eq!(roles.len(), 11);
        for role in roles {
            serde_json::from_value::<SlotRole>(role.clone()).unwrap();
        }
        assert!(Layout::parse(LAYOUT).is_ok());
    }

    #[tokio::test]
    async fn the_model_sees_the_type_the_sample_and_the_schema_and_its_layout_is_stored() {
        let store = store().await;
        let cited = cited();
        let mut model = Scripted::replying(LAYOUT);
        let outcome = ensure_layout(&store, &mut model, key(&cited), &cited, "a1")
            .await
            .unwrap();
        let Outcome::Stored { sha256 } = outcome else {
            panic!("{outcome:?}");
        };
        let stored = store.layout_for(key(&cited)).await.unwrap().unwrap();
        assert_eq!(stored.sha256, sha256);
        assert_eq!(stored.layout, LAYOUT);

        let seen = model.seen.unwrap();
        assert_eq!(seen.instructions, INSTRUCTIONS);
        assert!(seen.tools.is_empty());
        let [Item::Message { role, text }] = seen.items.as_slice() else {
            panic!("{:?}", seen.items);
        };
        assert_eq!(role, "user");
        let sent: Value = serde_json::from_str(text).unwrap();
        assert_eq!(sent["record_type"], "Observation");
        assert_eq!(sent["sample_record"], observation());
        assert_eq!(sent["layout_schema"], *SCHEMA);

        let view = snapshot_view(&store, &cited).await;
        assert_eq!(view["layout_sections"][0]["heading"], "Result", "{view}");
        assert_ne!(view, general(&cited));
    }

    #[tokio::test]
    async fn a_refused_layout_is_not_stored_and_the_record_reads_through_the_general_view() {
        let store = store().await;
        let typed_in = LAYOUT.replacen("\"sections\"", "\"value\":\"7.2\",\"sections\"", 1);
        let missing = LAYOUT.replace("/valueQuantity/unit", "/valueQuantity/missing");
        type Expected = fn(&Unsaved) -> bool;
        let cases: [(&str, Expected); 4] = [
            (&typed_in, |e| matches!(e, Unsaved::NotALayout(_))),
            (&missing, |e| {
                matches!(
                    e,
                    Unsaved::Refused(Refusal {
                        cause: LayoutRefusal::Unresolved { .. },
                        ..
                    })
                )
            }),
            ("Here is the layout you asked for.", |e| {
                matches!(e, Unsaved::NotALayout(_))
            }),
            ("   ", |e| matches!(e, Unsaved::Model(_))),
        ];
        for (reply, expected) in cases {
            let cited = cited();
            let mut model = Scripted::replying(reply);
            let error = ensure_layout(&store, &mut model, key(&cited), &cited, "a1")
                .await
                .unwrap_err();
            assert!(expected(&error), "{reply}: {error:?}");
            assert_eq!(store.layout_for(key(&cited)).await.unwrap(), None);
            assert_eq!(snapshot_view(&store, &cited).await, general(&cited));
        }
    }

    /// The AC10 planted error: the pointer is valid and its value is copied exactly, but it binds
    /// the reference range's low limit as the result. The role check refuses it before saving.
    #[tokio::test]
    async fn a_reference_range_low_labelled_as_the_result_is_refused_and_not_stored() {
        let store = store().await;
        let cited = cited();
        let planted = LAYOUT.replace("/valueQuantity/value", "/referenceRange/0/low/value");
        let mut model = Scripted::replying(&planted);
        let error = ensure_layout(&store, &mut model, key(&cited), &cited, "a1")
            .await
            .unwrap_err();
        let Unsaved::Refused(Refusal {
            cause:
                LayoutRefusal::RoleMismatch {
                    pointer,
                    role,
                    reason,
                },
            ..
        }) = error
        else {
            panic!("{error:?}");
        };
        assert_eq!(pointer, "/referenceRange/0/low/value");
        assert_eq!(role, SlotRole::Result);
        assert_eq!(reason, RoleMismatchReason::UnderReferenceRange);
        assert_eq!(store.layout_for(key(&cited)).await.unwrap(), None);
        assert_eq!(snapshot_view(&store, &cited).await, general(&cited));
    }

    #[tokio::test]
    async fn a_failed_model_call_stores_nothing() {
        let store = store().await;
        let cited = cited();
        let mut model = Scripted::failing("provider HTTP 500; no automatic retry");
        let error = ensure_layout(&store, &mut model, key(&cited), &cited, "a1")
            .await
            .unwrap_err();
        assert!(matches!(&error, Unsaved::Model(reason) if reason.contains("HTTP 500")));
        assert_eq!(store.layout_for(key(&cited)).await.unwrap(), None);
        assert_eq!(snapshot_view(&store, &cited).await, general(&cited));
    }

    #[tokio::test]
    async fn a_record_that_is_not_json_asks_no_model() {
        let store = store().await;
        let mut cited = cited();
        cited.content = "Potassium 4.1 mmol/L".into();
        let mut model = Scripted::replying(LAYOUT);
        let error = ensure_layout(&store, &mut model, key(&cited), &cited, "a1")
            .await
            .unwrap_err();
        assert!(matches!(error, Unsaved::RecordNotJson(_)), "{error:?}");
        assert_eq!(model.calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn a_key_with_a_layout_asks_no_model_again() {
        let store = store().await;
        let cited = cited();
        let mut model = Scripted::replying(LAYOUT);
        for expected in [true, false] {
            let outcome = ensure_layout(&store, &mut model, key(&cited), &cited, "a1")
                .await
                .unwrap();
            assert_eq!(matches!(outcome, Outcome::Stored { .. }), expected);
        }
        assert_eq!(model.calls.load(Ordering::SeqCst), 1);
    }

    /// Two writers that both found no layout race to save different ones: the first stored
    /// stays, and the other learns it lost without failing.
    #[tokio::test]
    async fn racing_writers_settle_on_the_first_stored_layout() {
        let store = store().await;
        let cited = cited();
        let relabelled = LAYOUT.replace("\"heading\":\"result\"", "\"heading\":\"results\"");
        assert_ne!(relabelled, LAYOUT);
        let mut first = Scripted::replying(LAYOUT);
        let mut second = Scripted::replying(&relabelled);
        let (a, b) = tokio::join!(
            ensure_layout(&store, &mut first, key(&cited), &cited, "a1"),
            ensure_layout(&store, &mut second, key(&cited), &cited, "a2"),
        );
        let outcomes = [a.unwrap(), b.unwrap()];
        let stored = store.layout_for(key(&cited)).await.unwrap().unwrap();
        let winners: Vec<_> = outcomes
            .iter()
            .filter(|o| matches!(o, Outcome::Stored { .. }))
            .collect();
        assert_eq!(winners.len(), 1, "{outcomes:?}");
        assert!(
            outcomes.iter().any(|o| matches!(
                o,
                Outcome::LostRace { stored_sha256 } if *stored_sha256 == stored.sha256
            ) || matches!(o, Outcome::AlreadyStored)),
            "{outcomes:?}"
        );
        assert!(stored.layout == LAYOUT || stored.layout == relabelled);
    }

    #[test]
    fn one_key_is_held_once_on_a_process_until_released() {
        let in_flight = InFlight::default();
        let key = LayoutKey {
            source_id: "ehr",
            record_type: "Observation",
        };
        let held = in_flight.hold(key).unwrap();
        assert!(in_flight.hold(key).is_none());
        let other = LayoutKey {
            source_id: "ehr",
            record_type: "Condition",
        };
        assert!(in_flight.hold(other).is_some());
        drop(held);
        assert!(in_flight.hold(key).is_some());
    }

    /// A refused hold must not release the holder's key: every later attempt is refused too,
    /// until the holder itself lets go.
    #[test]
    fn a_refused_hold_leaves_the_holders_key_held() {
        let in_flight = InFlight::default();
        let key = LayoutKey {
            source_id: "ehr",
            record_type: "Observation",
        };
        let held = in_flight.hold(key).unwrap();
        for _ in 0..3 {
            assert!(in_flight.hold(key).is_none());
        }
        drop(held);
        assert!(in_flight.hold(key).is_some());
    }

    #[test]
    fn each_cited_key_is_taken_once_from_its_first_citation() {
        let mut first = cited();
        first.id = "first".into();
        let mut again = first.clone();
        again.id = "again".into();
        let mut unsourced = cited();
        unsourced.source_id = None;
        let other = cited();
        let citations = [first, again, unsourced, other.clone()];
        let ids: Vec<_> = first_of_each_key(&citations)
            .into_iter()
            .map(|(_, citation)| citation.id.as_str())
            .collect();
        assert_eq!(ids, ["first", other.id.as_str()]);
    }
}

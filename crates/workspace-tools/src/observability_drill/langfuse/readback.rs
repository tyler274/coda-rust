//! Reading one rehearsal turn back from Langfuse and the operations
//! backends. Parsing is pure so the drill's judgments are unit-tested; the
//! transport lives in `mod.rs`.
use serde::Serialize;
use serde_json::Value;

/// The generation span's name (`telemetry::genai::generation_span`).
pub(super) const GENERATION: &str = "gen_ai.generation";

/// Finds `key` (dotted, like `coda.run_id`) anywhere in `value`: as a flat
/// key, nested by segments, or under any container Langfuse or Datadog puts
/// attributes in (`metadata.attributes`, `attributes.custom`, ...).
pub(super) fn find<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    if let Some(found) = dotted(value, key) {
        return Some(found);
    }
    match value {
        Value::Object(map) => map.values().find_map(|child| find(child, key)),
        Value::Array(items) => items.iter().find_map(|child| find(child, key)),
        _ => None,
    }
}

/// [`find`] for a string attribute.
pub(super) fn attribute<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    find(value, key).and_then(Value::as_str)
}

fn dotted<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    let map = value.as_object()?;
    if let Some(found) = map.get(key).filter(|found| !found.is_null()) {
        return Some(found);
    }
    key.match_indices('.').find_map(|(at, _)| {
        let child = map.get(&key[..at])?;
        dotted(child, &key[at + 1..])
    })
}

/// Whether `needle` occurs anywhere in `value`, keys included.
pub(super) fn mentions(value: &Value, needle: &str) -> bool {
    value.to_string().contains(needle)
}

/// What Langfuse holds for one rehearsal turn.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct LangfuseTurn {
    pub(super) trace_id: String,
    pub(super) environment: Option<String>,
    pub(super) session_id: Option<String>,
    pub(super) run_id: Option<String>,
    pub(super) scenario_id: Option<String>,
    pub(super) model: Option<String>,
    /// Whether the generation's input or output carries the canary.
    pub(super) canary_in_content: bool,
}

/// The newest trace id in a `GET /api/public/traces?sessionId=` answer.
pub(super) fn session_trace(body: &Value) -> Option<String> {
    body["data"]
        .as_array()?
        .iter()
        .max_by_key(|trace| trace["timestamp"].as_str().unwrap_or_default().to_owned())
        .and_then(|trace| trace["id"].as_str())
        .map(str::to_owned)
}

/// Reads a `GET /api/public/traces/{id}` answer for its generation.
pub(super) fn langfuse_turn(trace: &Value, canary: &str) -> Option<LangfuseTurn> {
    let generation = trace["observations"]
        .as_array()?
        .iter()
        .find(|observation| observation["name"] == GENERATION)?;
    let text = |value: &Value| value.as_str().map(str::to_owned);
    let content_has = |field: &str| mentions(&generation[field], canary);
    Some(LangfuseTurn {
        trace_id: text(&trace["id"])?,
        environment: text(&trace["environment"]).or_else(|| text(&generation["environment"])),
        session_id: text(&trace["sessionId"]),
        run_id: attribute(&generation["metadata"], "coda.run_id").map(str::to_owned),
        scenario_id: attribute(&generation["metadata"], "coda.scenario_id").map(str::to_owned),
        model: text(&generation["model"])
            .or_else(|| text(&generation["providedModelName"]))
            .or_else(|| {
                attribute(&generation["metadata"], "gen_ai.request.model").map(str::to_owned)
            }),
        canary_in_content: content_has("input") || content_has("output"),
    })
}

/// What `rehearse` requires of the Langfuse generation.
pub(super) fn rehearsal_gaps(turn: &LangfuseTurn, chat: &str, scenario: &str) -> Vec<String> {
    let mut gaps = Vec::new();
    let mut expect = |field: &str, actual: Option<&str>, wanted: &str| {
        if actual != Some(wanted) {
            gaps.push(format!("{field} is {actual:?}, not {wanted:?}"));
        }
    };
    expect("environment", turn.environment.as_deref(), "simulation");
    expect("session", turn.session_id.as_deref(), chat);
    expect("coda.scenario_id", turn.scenario_id.as_deref(), scenario);
    if turn.run_id.as_deref().is_none_or(str::is_empty) {
        gaps.push("coda.run_id is missing".to_owned());
    }
    if !is_trace_id(&turn.trace_id) {
        gaps.push(format!("trace id {:?} is not 32 hex digits", turn.trace_id));
    }
    if !turn.canary_in_content {
        gaps.push("the generation's content does not carry the canary".to_owned());
    }
    gaps
}

pub(super) fn is_trace_id(value: &str) -> bool {
    value.len() == 32 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

/// The lower 64 bits of a 128-bit trace id in decimal: Datadog's `trace_id`.
pub(super) fn datadog_trace_id(trace_id: &str) -> Option<String> {
    let low = trace_id.get(16..32)?;
    u64::from_str_radix(low, 16).ok().map(|id| id.to_string())
}

/// What an operations backend holds for the same trace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct OpsSpan {
    pub(super) backend: &'static str,
    pub(super) found: bool,
    pub(super) run_id: Option<String>,
    pub(super) model: Option<String>,
    pub(super) usage: bool,
    pub(super) canary: bool,
}

/// Reads a Datadog `POST /api/v2/spans/events/search` answer for the
/// generation span.
pub(super) fn datadog_span(body: &Value, canary: &str) -> OpsSpan {
    let spans = body["data"].as_array().cloned().unwrap_or_default();
    let generation = spans.iter().find(|span| {
        let attributes = &span["attributes"];
        attributes["resource_name"] == GENERATION
            || attributes["operation_name"] == GENERATION
            || attribute(attributes, "otel.name") == Some(GENERATION)
    });
    ops_span("datadog", generation, &spans, canary)
}

/// Flattens a Tempo `GET /api/traces/{id}` answer (OTLP JSON) into spans
/// whose attributes are a `{key: value}` object.
pub(super) fn tempo_spans(body: &Value) -> Vec<Value> {
    let batches = body["batches"]
        .as_array()
        .or_else(|| body["resourceSpans"].as_array())
        .cloned()
        .unwrap_or_default();
    let mut spans = Vec::new();
    for batch in &batches {
        let scopes = batch["scopeSpans"]
            .as_array()
            .or_else(|| batch["instrumentationLibrarySpans"].as_array());
        for scope in scopes.into_iter().flatten() {
            for span in scope["spans"].as_array().into_iter().flatten() {
                let mut attributes = serde_json::Map::new();
                for pair in span["attributes"].as_array().into_iter().flatten() {
                    let Some(key) = pair["key"].as_str() else {
                        continue;
                    };
                    let value = pair["value"]
                        .as_object()
                        .and_then(|value| value.values().next())
                        .cloned()
                        .unwrap_or(Value::Null);
                    let value = match value {
                        Value::Number(number) => Value::String(number.to_string()),
                        other => other,
                    };
                    attributes.insert(key.to_owned(), value);
                }
                spans.push(serde_json::json!({
                    "name": span["name"],
                    "attributes": attributes,
                    "raw": span,
                }));
            }
        }
    }
    spans
}

pub(super) fn tempo_span(body: &Value, canary: &str) -> OpsSpan {
    let spans = tempo_spans(body);
    let generation = spans.iter().find(|span| span["name"] == GENERATION);
    ops_span("tempo", generation, &spans, canary)
}

fn ops_span(
    backend: &'static str,
    generation: Option<&Value>,
    trace: &[Value],
    canary: &str,
) -> OpsSpan {
    let Some(generation) = generation else {
        return OpsSpan {
            backend,
            found: false,
            run_id: None,
            model: None,
            usage: false,
            canary: trace.iter().any(|span| mentions(span, canary)),
        };
    };
    let text = |key: &str| attribute(generation, key).map(str::to_owned);
    OpsSpan {
        backend,
        found: true,
        run_id: text("coda.run_id"),
        model: text("gen_ai.request.model"),
        usage: ["gen_ai.usage.input_tokens", "gen_ai.usage.output_tokens"]
            .iter()
            .any(|key| find(generation, key).is_some()),
        canary: trace.iter().any(|span| mentions(span, canary)),
    }
}

/// What `isolation` requires of an operations span: it is the same trace's
/// generation, it carries model and usage, and it carries no content.
pub(super) fn isolation_gaps(span: &OpsSpan, run_id: Option<&str>) -> Vec<String> {
    let backend = span.backend;
    if !span.found {
        return vec![format!(
            "{backend} has no generation span for the trace; an empty search is not proof"
        )];
    }
    let mut gaps = Vec::new();
    if span.canary {
        gaps.push(format!("{backend} carries the canary"));
    }
    if span.model.is_none() {
        gaps.push(format!("{backend} span has no gen_ai.request.model"));
    }
    if !span.usage {
        gaps.push(format!("{backend} span has no token usage"));
    }
    if run_id.is_some() && span.run_id.as_deref() != run_id {
        gaps.push(format!(
            "{backend} coda.run_id is {:?}, Langfuse has {run_id:?}",
            span.run_id
        ));
    }
    gaps
}

/// Number of log lines in a Loki `query_range` answer, or `None` when the
/// answer is not a successful streams result: a malformed answer is never
/// read as zero lines.
pub(super) fn loki_lines(body: &Value) -> Option<usize> {
    if body["status"] != "success" || body["data"]["resultType"] != "streams" {
        return None;
    }
    let mut lines = 0;
    for stream in body["data"]["result"].as_array()? {
        lines += stream["values"].as_array()?.len();
    }
    Some(lines)
}

/// A Datadog monitor group's state for the Langfuse exporter, from
/// `GET /api/v1/monitor?name=...&group_states=all`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct MonitorGroup {
    pub(super) monitor: String,
    pub(super) status: String,
    pub(super) last_triggered_ts: Option<i64>,
    pub(super) last_resolved_ts: Option<i64>,
}

pub(super) fn exporter_groups(monitors: &Value, exporter: &str) -> Vec<MonitorGroup> {
    let wanted = format!("exporter:{exporter}");
    let mut groups = Vec::new();
    for monitor in monitors.as_array().into_iter().flatten() {
        let name = monitor["name"].as_str().unwrap_or_default();
        let Some(states) = monitor["state"]["groups"].as_object() else {
            continue;
        };
        for (group, state) in states {
            if !group.split(',').any(|tag| tag == wanted) {
                continue;
            }
            groups.push(MonitorGroup {
                monitor: name.to_owned(),
                status: state["status"].as_str().unwrap_or_default().to_owned(),
                last_triggered_ts: state["last_triggered_ts"].as_i64(),
                last_resolved_ts: state["last_resolved_ts"].as_i64(),
            });
        }
    }
    groups
}

/// The total of a Datadog `GET /api/v1/query` answer's points.
pub(super) fn series_total(body: &Value) -> f64 {
    body["series"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|series| series["pointlist"].as_array().into_iter().flatten())
        .filter_map(|point| point.get(1).and_then(Value::as_f64))
        .sum()
}

/// What a NextAuth `POST /api/auth/callback/credentials` with `json=true`
/// did. Langfuse lists the credentials provider even with passwords off
/// (`AUTH_DISABLE_USERNAME_PASSWORD`), and refuses inside its callback, so
/// only an attempted sign-in and the session it leaves say whether passwords
/// work.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum PasswordSignIn {
    /// The callback answered without an error and the session holds a user.
    SignedIn,
    /// The callback answered with an error, or left no user in the session.
    Refused,
}

/// Judges the credentials callback's redirect `url` and the session read
/// right after it.
pub(super) fn password_sign_in(callback: &Value, session: &Value) -> PasswordSignIn {
    let refused = callback["url"]
        .as_str()
        .is_none_or(|url| url.contains("error="));
    let user = session["user"]["email"]
        .as_str()
        .is_some_and(|email| !email.is_empty());
    if !refused && user {
        PasswordSignIn::SignedIn
    } else {
        PasswordSignIn::Refused
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const TRACE: &str = "0af7651916cd43dd8448eb211c80319c";

    fn langfuse_trace() -> Value {
        json!({
            "id": TRACE,
            "environment": "simulation",
            "sessionId": "chat-1",
            "observations": [
                { "name": "physician.worker.attempt", "metadata": {} },
                {
                    "name": "gen_ai.generation",
                    "model": "gpt-6-sol",
                    "input": "canary-123 hello",
                    "output": "reply",
                    "metadata": {
                        "attributes": {
                            "coda.run_id": "run-1",
                            "coda.scenario_id": "langfuse-drill"
                        },
                        "resourceAttributes": { "service.name": "physician-worker" }
                    }
                }
            ]
        })
    }

    #[test]
    fn attributes_are_found_flat_nested_or_under_containers() {
        let flat = json!({ "attributes": { "coda.run_id": "a" } });
        let nested = json!({ "custom": { "coda": { "run_id": "b" } } });
        assert_eq!(attribute(&flat, "coda.run_id"), Some("a"));
        assert_eq!(attribute(&nested, "coda.run_id"), Some("b"));
        assert_eq!(attribute(&json!({ "coda": {} }), "coda.run_id"), None);
    }

    #[test]
    fn a_rehearsal_generation_meets_the_contract() {
        let turn = langfuse_turn(&langfuse_trace(), "canary-123").unwrap();
        assert_eq!(turn.run_id.as_deref(), Some("run-1"));
        assert_eq!(turn.model.as_deref(), Some("gpt-6-sol"));
        assert!(rehearsal_gaps(&turn, "chat-1", "langfuse-drill").is_empty());
    }

    #[test]
    fn a_real_or_mislabelled_generation_is_refused() {
        let mut trace = langfuse_trace();
        trace["environment"] = json!("real");
        trace["observations"][1]["input"] = json!("hello");
        let turn = langfuse_turn(&trace, "canary-123").unwrap();
        let gaps = rehearsal_gaps(&turn, "chat-2", "langfuse-drill");
        assert_eq!(gaps.len(), 3, "{gaps:?}");
    }

    #[test]
    fn a_trace_without_its_generation_is_not_a_turn() {
        let mut trace = langfuse_trace();
        trace["observations"] = json!([]);
        assert_eq!(langfuse_turn(&trace, "c"), None);
    }

    #[test]
    fn the_newest_session_trace_wins() {
        let body = json!({ "data": [
            { "id": "old", "timestamp": "2026-11-09T10:00:00.000Z" },
            { "id": "new", "timestamp": "2026-11-09T11:00:00.000Z" }
        ]});
        assert_eq!(session_trace(&body).as_deref(), Some("new"));
        assert_eq!(session_trace(&json!({ "data": [] })), None);
    }

    #[test]
    fn datadog_ids_are_the_low_64_bits() {
        assert_eq!(
            datadog_trace_id(TRACE).as_deref(),
            Some(
                u64::from_str_radix("8448eb211c80319c", 16)
                    .unwrap()
                    .to_string()
                    .as_str()
            )
        );
        assert_eq!(datadog_trace_id("short"), None);
    }

    fn datadog_body(extra: Value) -> Value {
        json!({ "data": [
            { "attributes": { "resource_name": "physician.worker.attempt" } },
            { "attributes": {
                "resource_name": "gen_ai.generation",
                "custom": {
                    "coda": { "run_id": "run-1" },
                    "gen_ai": { "request": { "model": "gpt-6-sol" }, "usage": { "input_tokens": 12 } },
                    "extra": extra
                }
            } }
        ]})
    }

    #[test]
    fn a_content_free_datadog_span_is_isolated() {
        let span = datadog_span(&datadog_body(json!(null)), "canary-123");
        assert!(span.found && span.usage, "{span:?}");
        assert!(isolation_gaps(&span, Some("run-1")).is_empty());
    }

    #[test]
    fn a_canary_anywhere_in_the_trace_fails_isolation() {
        let span = datadog_span(&datadog_body(json!("canary-123")), "canary-123");
        assert_eq!(
            isolation_gaps(&span, Some("run-1")),
            ["datadog carries the canary"]
        );
    }

    #[test]
    fn an_empty_search_is_never_isolation_proof() {
        let span = datadog_span(&json!({ "data": [] }), "canary-123");
        let gaps = isolation_gaps(&span, None);
        assert!(gaps[0].contains("not proof"), "{gaps:?}");
    }

    #[test]
    fn tempo_otlp_attributes_are_read() {
        let body = json!({ "batches": [{ "scopeSpans": [{ "spans": [{
            "name": "gen_ai.generation",
            "attributes": [
                { "key": "coda.run_id", "value": { "stringValue": "run-1" } },
                { "key": "gen_ai.request.model", "value": { "stringValue": "m" } },
                { "key": "gen_ai.usage.input_tokens", "value": { "intValue": "3" } }
            ]
        }]}]}]});
        let span = tempo_span(&body, "canary-123");
        assert!(isolation_gaps(&span, Some("run-1")).is_empty(), "{span:?}");
    }

    #[test]
    fn monitor_groups_are_picked_by_exporter() {
        let monitors = json!([{
            "name": "[staging] otel-collector send_failed_spans",
            "state": { "groups": {
                "exporter:otlphttp/langfuse": { "status": "Alert", "last_triggered_ts": 10 },
                "exporter:datadog": { "status": "OK" }
            }}
        }]);
        let groups = exporter_groups(&monitors, "otlphttp/langfuse");
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].status, "Alert");
        assert_eq!(groups[0].last_triggered_ts, Some(10));
    }

    #[test]
    fn series_and_loki_and_providers_are_counted() {
        let series = json!({ "series": [{ "pointlist": [[1, 2.0], [2, null], [3, 1.5]] }] });
        assert!((series_total(&series) - 3.5).abs() < f64::EPSILON);
        let loki = json!({ "status": "success", "data": {
            "resultType": "streams",
            "result": [{ "values": [["1", "a"], ["2", "b"]] }]
        }});
        assert_eq!(loki_lines(&loki), Some(2));
        let empty =
            json!({ "status": "success", "data": { "resultType": "streams", "result": [] } });
        assert_eq!(loki_lines(&empty), Some(0));
        assert_eq!(loki_lines(&json!({})), None);
        assert_eq!(loki_lines(&json!({ "status": "error" })), None);
    }

    #[test]
    fn only_a_session_with_a_user_counts_as_password_sign_in() {
        let ok = json!({ "url": "http://127.0.0.1/langfuse/" });
        let user = json!({ "user": { "email": "admin@codahealth.ai" } });
        assert_eq!(password_sign_in(&ok, &user), PasswordSignIn::SignedIn);
        let refused =
            json!({ "url": "http://127.0.0.1/langfuse/auth/sign-in?error=CredentialsSignin" });
        assert_eq!(password_sign_in(&refused, &user), PasswordSignIn::Refused);
        assert_eq!(password_sign_in(&ok, &json!({})), PasswordSignIn::Refused);
        assert_eq!(password_sign_in(&json!({}), &user), PasswordSignIn::Refused);
    }
}

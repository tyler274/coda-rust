//! Reading one rehearsal turn back from Langfuse and the operations
//! backends. Parsing is pure so the drill's judgments are unit-tested; the
//! transport lives in `mod.rs`.
use chrono::{DateTime, Utc};
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
    /// Langfuse's observation level: `ERROR` for a failed generation.
    pub(super) level: Option<String>,
    /// Whether the generation's input or output carries the canary.
    pub(super) canary_in_content: bool,
}

/// The generations in a `GET /api/public/v2/observations?sessionId=` answer,
/// newest first: a retried attempt's generation comes before the one it
/// replaced. `None` when the answer has no `data` list or a generation in it
/// has no `traceId`: a malformed answer, not an empty session.
pub(super) fn session_generations(body: &Value, canary: &str) -> Option<Vec<LangfuseTurn>> {
    let mut generations: Vec<&Value> = body["data"]
        .as_array()?
        .iter()
        .filter(|observation| observation["name"] == GENERATION)
        .collect();
    generations.sort_by_key(|observation| {
        std::cmp::Reverse(
            observation["startTime"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
        )
    });
    generations
        .into_iter()
        .map(|generation| langfuse_turn(generation, canary))
        .collect()
}

/// A span attribute of a v2 observation read with `expandMetadata=attributes`:
/// Langfuse flattens each one into `metadata` as `attributes.<key>`.
fn span_attribute(generation: &Value, key: &str) -> Option<String> {
    generation["metadata"][format!("attributes.{key}")]
        .as_str()
        .map(str::to_owned)
}

/// Reads one generation observation (v2 observations API) as a turn.
fn langfuse_turn(generation: &Value, canary: &str) -> Option<LangfuseTurn> {
    let text = |value: &Value| value.as_str().map(str::to_owned);
    let content_has = |field: &str| mentions(&generation[field], canary);
    Some(LangfuseTurn {
        trace_id: text(&generation["traceId"])?,
        environment: text(&generation["environment"]),
        session_id: text(&generation["sessionId"]),
        run_id: span_attribute(generation, "coda.run_id"),
        scenario_id: span_attribute(generation, "coda.scenario_id"),
        model: text(&generation["model"])
            .or_else(|| text(&generation["providedModelName"]))
            .or_else(|| span_attribute(generation, "gen_ai.request.model")),
        level: text(&generation["level"]),
        canary_in_content: content_has("input") || content_has("output"),
    })
}

/// What `chat-failure` requires of the failed attempt's Langfuse generation:
/// a simulation generation of that attempt, at level `ERROR`, holding the
/// canary it was asked.
pub(super) fn failed_generation_gaps(turn: &LangfuseTurn, run_id: &str) -> Vec<String> {
    let mut gaps = Vec::new();
    if turn.run_id.as_deref() != Some(run_id) {
        gaps.push(format!(
            "Langfuse coda.run_id is {:?}, not the failed attempt {run_id}",
            turn.run_id
        ));
    }
    if turn.level.as_deref() != Some("ERROR") {
        gaps.push(format!("Langfuse level is {:?}, not \"ERROR\"", turn.level));
    }
    if turn.environment.as_deref() != Some("simulation") {
        gaps.push(format!(
            "Langfuse environment is {:?}, not \"simulation\"",
            turn.environment
        ));
    }
    if !turn.canary_in_content {
        gaps.push("the failed generation's content does not carry the canary".to_owned());
    }
    gaps
}

/// One `physician.worker.attempt` span as Datadog holds it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct AttemptSpan {
    pub(super) run_id: String,
    pub(super) work_id: Option<String>,
    pub(super) predecessor_run_id: Option<String>,
    pub(super) error_type: Option<String>,
    /// `coda.trace_relation`: how the attempt's new trace relates to the
    /// request that made its work runnable.
    pub(super) trace_relation: Option<String>,
    pub(super) start: Option<DateTime<Utc>>,
    pub(super) duration_ms: Option<i64>,
    /// `coda.rehearsed_hold_ms`: the hold a `[hold:S]` marker made it take.
    pub(super) rehearsed_hold_ms: Option<i64>,
}

/// The attempt span for `run_id` in a Datadog span search answer.
pub(super) fn attempt_span(body: &Value, run_id: &str) -> Option<AttemptSpan> {
    let span = body["data"].as_array()?.iter().find(|span| {
        let attributes = &span["attributes"];
        attributes["resource_name"] == "physician.worker.attempt"
            && attribute(attributes, "coda.run_id") == Some(run_id)
    })?;
    let attributes = &span["attributes"];
    let text = |key: &str| attribute(attributes, key).map(str::to_owned);
    let at = |key: &str| {
        attributes[key]
            .as_str()
            .and_then(|at| DateTime::parse_from_rfc3339(at).ok())
            .map(|at| at.with_timezone(&Utc))
    };
    let start = at("start_timestamp");
    Some(AttemptSpan {
        run_id: run_id.to_owned(),
        work_id: text("coda.work_id"),
        predecessor_run_id: text("coda.predecessor_run_id"),
        error_type: text("error.type"),
        trace_relation: text("coda.trace_relation"),
        start,
        duration_ms: start
            .zip(at("end_timestamp"))
            .map(|(start, end)| (end - start).num_milliseconds()),
        rehearsed_hold_ms: find(attributes, "coda.rehearsed_hold_ms").and_then(|hold| {
            hold.as_i64()
                .or_else(|| hold.as_str().and_then(|hold| hold.parse().ok()))
        }),
    })
}

/// What the worker logs just before a rehearsed crash aborts it.
pub(super) const CRASH_EVENT: &str = "physician.worker.rehearsed_crash";

/// The rehearsed-crash receipt of attempt `run_id` in a Cloud Logging
/// `entries:list` answer. The worker aborts right after writing it, so only
/// its stdout line reaches a backend.
pub(super) fn crash_receipt(body: &Value, run_id: &str) -> Option<Value> {
    body["entries"]
        .as_array()?
        .iter()
        .find(|entry| {
            let payload = &entry["jsonPayload"];
            payload["event"] == CRASH_EVENT && payload["coda.run_id"] == run_id
        })
        .cloned()
}

/// Where an attempt misses its relation to the accepting request: the worker
/// starts each attempt's own trace and links it to the request's saved trace
/// context, recording `coda.trace_relation` `link`. Any other relation means
/// the work reached the worker without the request's context.
pub(super) fn link_gaps(attempt: &AttemptSpan) -> Vec<String> {
    if attempt.trace_relation.as_deref() == Some("link") {
        return Vec::new();
    }
    vec![format!(
        "attempt {} has coda.trace_relation {:?}, not \"link\"; it is not linked to the accepting request",
        attempt.run_id, attempt.trace_relation
    )]
}

/// What `chat-failure` requires of a marked turn's two attempts: the first
/// failed on purpose, and the one that answered succeeds it in the same work.
pub(super) fn retry_gaps(failed: &AttemptSpan, successor: &AttemptSpan) -> Vec<String> {
    let mut gaps = Vec::new();
    if failed.error_type.as_deref() != Some("rehearsed") {
        gaps.push(format!(
            "attempt {} has error.type {:?}, not \"rehearsed\"",
            failed.run_id, failed.error_type
        ));
    }
    if let Some(error) = &successor.error_type {
        gaps.push(format!(
            "the answering attempt {} failed with {error}",
            successor.run_id
        ));
    }
    if successor.predecessor_run_id.as_deref() != Some(failed.run_id.as_str()) {
        gaps.push(format!(
            "the answering attempt's predecessor is {:?}, not {}",
            successor.predecessor_run_id, failed.run_id
        ));
    }
    if failed.work_id.is_none() || failed.work_id != successor.work_id {
        gaps.push(format!(
            "work {:?} of the failed attempt is not work {:?} of its successor",
            failed.work_id, successor.work_id
        ));
    }
    gaps
}

/// A named Datadog monitor: its alerting threshold and its states, one per
/// group, or the overall state when it is not grouped.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(super) struct NamedMonitor {
    pub(super) threshold: Option<f64>,
    pub(super) groups: Vec<MonitorGroup>,
}

/// Reads the monitor called `name` from `GET /api/v1/monitor?group_states=all`.
/// An ungrouped monitor's transition time is its `overall_state_modified`.
pub(super) fn named_monitor(monitors: &Value, name: &str) -> Option<NamedMonitor> {
    let monitor = monitors
        .as_array()?
        .iter()
        .find(|monitor| monitor["name"] == name)?;
    let threshold = monitor["options"]["thresholds"]["critical"].as_f64();
    let mut groups: Vec<MonitorGroup> = monitor["state"]["groups"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(_, state)| MonitorGroup {
            monitor: name.to_owned(),
            status: state["status"].as_str().unwrap_or_default().to_owned(),
            last_triggered_ts: state["last_triggered_ts"].as_i64(),
            last_resolved_ts: state["last_resolved_ts"].as_i64(),
        })
        .collect();
    if groups.is_empty() {
        let status = monitor["overall_state"].as_str().unwrap_or_default();
        let modified = monitor["overall_state_modified"]
            .as_str()
            .and_then(|at| chrono::DateTime::parse_from_rfc3339(at).ok())
            .map(|at| at.timestamp());
        groups.push(MonitorGroup {
            monitor: name.to_owned(),
            status: status.to_owned(),
            last_triggered_ts: modified.filter(|_| status == "Alert"),
            last_resolved_ts: modified.filter(|_| status == "OK"),
        });
    }
    Some(NamedMonitor { threshold, groups })
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

/// Whether Datadog's trace id names the OpenTelemetry `trace_id`. Datadog
/// answers either the lower 64 bits in decimal or the full 128-bit hex id.
pub(super) fn same_trace(datadog: &str, trace_id: &str) -> bool {
    datadog_trace_id(trace_id).as_deref() == Some(datadog) || datadog.eq_ignore_ascii_case(trace_id)
}

/// What an operations backend holds for the same trace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the drill's saved report has these three flags"
)]
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

/// One log backend's answer for one turn: the physician log lines of its
/// trace (the positive control that its logs arrive at all) and the lines
/// that carry the canary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct LogIsolation {
    pub(super) backend: &'static str,
    pub(super) trace_lines: usize,
    pub(super) canary_lines: usize,
}

impl LogIsolation {
    pub(super) fn gaps(&self) -> Vec<String> {
        let backend = self.backend;
        let mut gaps = Vec::new();
        if self.trace_lines == 0 {
            gaps.push(format!(
                "{backend} holds no physician log for the turn's trace; an empty answer is not proof"
            ));
        }
        if self.canary_lines > 0 {
            gaps.push(format!(
                "{backend} holds {} log line(s) with the canary",
                self.canary_lines
            ));
        }
        gaps
    }
}

/// Reads a Datadog `POST /api/v2/logs/events/search` page: every event is
/// scanned whole, so the canary is found in its message, attributes or
/// error fields alike. `None` when the answer has no `data` list, or when it
/// may be incomplete: more pages follow, the search timed out, or Datadog
/// warns of partial results. Such a page cannot vouch for the window.
pub(super) fn datadog_logs(body: &Value, trace_id: &str, canary: &str) -> Option<LogIsolation> {
    let meta = &body["meta"];
    let complete = !meta["page"]["after"].is_string()
        && meta["status"] == "done"
        && meta["warnings"].as_array().is_none_or(Vec::is_empty);
    if !complete {
        return None;
    }
    let events = body["data"].as_array()?;
    Some(LogIsolation {
        backend: "datadog",
        trace_lines: events
            .iter()
            .filter(|event| mentions(event, trace_id))
            .count(),
        canary_lines: events
            .iter()
            .filter(|event| mentions(event, canary))
            .count(),
    })
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

/// A Datadog monitor group's state, from
/// `GET /api/v1/monitor?name=...&group_states=all`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct MonitorGroup {
    pub(super) monitor: String,
    pub(super) status: String,
    pub(super) last_triggered_ts: Option<i64>,
    pub(super) last_resolved_ts: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const TRACE: &str = "0af7651916cd43dd8448eb211c80319c";

    /// One generation as `GET /api/public/v2/observations` returns it.
    fn generation() -> Value {
        json!({
            "id": "obs-1",
            "traceId": TRACE,
            "startTime": "2026-11-09T10:00:00.000Z",
            "type": "GENERATION",
            "name": "gen_ai.generation",
            "environment": "simulation",
            "sessionId": "chat-1",
            "model": "gpt-6-sol",
            "input": "canary-123 hello",
            "output": "reply",
            "metadata": {
                "attributes.coda.run_id": "run-1",
                "attributes.coda.scenario_id": "langfuse-drill",
                "resourceAttributes.service.name": "physician-worker"
            }
        })
    }

    fn observations(items: &[Value]) -> Value {
        let attempt =
            json!({ "traceId": TRACE, "name": "physician.worker.attempt", "metadata": {} });
        let mut data = vec![attempt];
        data.extend_from_slice(items);
        json!({ "data": data, "meta": {} })
    }

    fn turn_of(item: Value) -> LangfuseTurn {
        session_generations(&observations(&[item]), "canary-123")
            .unwrap()
            .remove(0)
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
        let turn = turn_of(generation());
        assert_eq!(turn.trace_id, TRACE);
        assert_eq!(turn.run_id.as_deref(), Some("run-1"));
        assert_eq!(turn.model.as_deref(), Some("gpt-6-sol"));
        assert!(rehearsal_gaps(&turn, "chat-1", "langfuse-drill").is_empty());
    }

    #[test]
    fn a_real_or_mislabelled_generation_is_refused() {
        let mut item = generation();
        item["environment"] = json!("real");
        item["input"] = json!("hello");
        let gaps = rehearsal_gaps(&turn_of(item), "chat-2", "langfuse-drill");
        assert_eq!(gaps.len(), 3, "{gaps:?}");
    }

    #[test]
    fn a_session_without_its_generation_has_no_turn() {
        assert_eq!(session_generations(&observations(&[]), "c"), Some(vec![]));
    }

    #[test]
    fn a_malformed_answer_is_not_an_empty_session() {
        assert_eq!(session_generations(&json!({}), "c"), None);
        let mut item = generation();
        item["traceId"] = Value::Null;
        assert_eq!(session_generations(&observations(&[item]), "c"), None);
    }

    #[test]
    fn session_generations_come_newest_first() {
        let mut first = generation();
        first["traceId"] = json!("first");
        let mut retried = generation();
        retried["traceId"] = json!("retried");
        retried["startTime"] = json!("2026-11-09T10:00:12.000Z");
        let turns = session_generations(&observations(&[first, retried]), "canary-123").unwrap();
        let traces: Vec<&str> = turns.iter().map(|turn| turn.trace_id.as_str()).collect();
        assert_eq!(traces, ["retried", "first"]);
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

    #[test]
    fn a_datadog_trace_matches_in_decimal_or_full_hex() {
        let low = datadog_trace_id(TRACE).unwrap();
        assert!(same_trace(&low, TRACE));
        assert!(same_trace(TRACE, TRACE));
        assert!(same_trace(&TRACE.to_uppercase(), TRACE));
        assert!(!same_trace("1", TRACE));
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
    fn an_attempt_must_link_to_the_accepting_request() {
        let related = |relation: Value| {
            let mut body = attempt_body("run-1", json!({}));
            body["data"][0]["attributes"]["custom"]["coda"]["trace_relation"] = relation;
            attempt_span(&body, "run-1").unwrap()
        };
        let linked = related(json!("link"));
        assert_eq!(linked.trace_relation.as_deref(), Some("link"));
        assert!(link_gaps(&linked).is_empty());
        for unlinked in [related(json!("root")), related(Value::Null)] {
            let gaps = link_gaps(&unlinked);
            assert_eq!(gaps.len(), 1, "{gaps:?}");
            assert!(gaps[0].contains("run-1"), "{gaps:?}");
        }
    }

    #[test]
    fn datadog_logs_need_the_trace_and_no_canary() {
        let event = |extra: Value| {
            json!({ "attributes": { "service": "physician-sample",
                "attributes": { "otel": { "trace_id": TRACE } }, "extra": extra } })
        };
        let clean = json!({ "data": [event(json!(null)), event(json!(null))], "meta": { "status": "done" } });
        let logs = datadog_logs(&clean, TRACE, "canary-123").unwrap();
        assert_eq!((logs.trace_lines, logs.canary_lines), (2, 0));
        assert!(logs.gaps().is_empty());

        let leaked = json!({ "data": [event(json!({ "error": { "message": "canary-123" } }))],
            "meta": { "status": "done" } });
        let logs = datadog_logs(&leaked, TRACE, "canary-123").unwrap();
        assert_eq!(logs.gaps(), ["datadog holds 1 log line(s) with the canary"]);

        let empty = json!({ "data": [], "meta": { "status": "done" } });
        let empty = datadog_logs(&empty, TRACE, "canary-123").unwrap();
        assert!(empty.gaps()[0].contains("not proof"));

        let paged =
            json!({ "data": [], "meta": { "status": "done", "page": { "after": "next" } } });
        assert_eq!(datadog_logs(&paged, TRACE, "canary-123"), None);
        assert_eq!(datadog_logs(&json!({}), TRACE, "canary-123"), None);

        // A search that timed out or warns of partial results may have left
        // out the very event with the canary, so it vouches for nothing.
        let traced = [event(json!(null))];
        let timed_out = json!({ "data": traced, "meta": { "status": "timeout" } });
        assert_eq!(datadog_logs(&timed_out, TRACE, "canary-123"), None);
        let warned = json!({ "data": traced, "meta": { "status": "done",
            "warnings": [{ "code": "unknown_index", "title": "partial" }] } });
        assert_eq!(datadog_logs(&warned, TRACE, "canary-123"), None);
        let done = json!({ "data": traced, "meta": { "status": "done", "warnings": [] } });
        assert!(datadog_logs(&done, TRACE, "canary-123").is_some());
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
    fn loki_lines_are_counted() {
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
    fn a_failed_generation_is_an_error_of_that_attempt_with_the_canary() {
        let mut item = generation();
        item["level"] = json!("ERROR");
        item["output"] = Value::Null;
        let turn = turn_of(item.clone());
        assert!(
            failed_generation_gaps(&turn, "run-1").is_empty(),
            "{turn:?}"
        );
        let gaps = failed_generation_gaps(&turn, "run-2");
        assert_eq!(gaps.len(), 1, "{gaps:?}");
        item["level"] = json!("DEFAULT");
        item["input"] = json!("hello");
        assert_eq!(failed_generation_gaps(&turn_of(item), "run-1").len(), 2);
    }

    fn attempt_body(run: &str, extra: Value) -> Value {
        let mut attributes = json!({
            "resource_name": "physician.worker.attempt",
            "custom": { "coda": { "run_id": run, "work_id": "work-1" } }
        });
        if let (Some(map), Value::Object(more)) = (attributes["custom"].as_object_mut(), extra) {
            map.extend(more);
        }
        json!({ "data": [{ "attributes": attributes }] })
    }

    #[test]
    fn a_rehearsed_failure_and_its_successor_share_the_work() {
        let failed = attempt_span(
            &attempt_body("run-1", json!({ "error": { "type": "rehearsed" } })),
            "run-1",
        )
        .unwrap();
        let mut body = attempt_body("run-2", json!({}));
        body["data"][0]["attributes"]["custom"]["coda"]["predecessor_run_id"] = json!("run-1");
        let successor = attempt_span(&body, "run-2").unwrap();
        assert_eq!(failed.error_type.as_deref(), Some("rehearsed"));
        assert!(retry_gaps(&failed, &successor).is_empty());
        assert_eq!(attempt_span(&body, "run-9"), None);
    }

    #[test]
    fn a_wrong_failure_or_a_broken_chain_is_a_gap() {
        let failed = AttemptSpan {
            run_id: "run-1".into(),
            work_id: Some("work-1".into()),
            predecessor_run_id: None,
            error_type: Some("model".into()),
            trace_relation: None,
            start: None,
            duration_ms: None,
            rehearsed_hold_ms: None,
        };
        let successor = AttemptSpan {
            run_id: "run-2".into(),
            work_id: Some("work-2".into()),
            predecessor_run_id: None,
            error_type: Some("model".into()),
            trace_relation: None,
            start: None,
            duration_ms: None,
            rehearsed_hold_ms: None,
        };
        assert_eq!(retry_gaps(&failed, &successor).len(), 4);
    }

    #[test]
    fn the_chat_failure_monitor_is_read_by_name_grouped_or_not() {
        let monitors = json!([
            { "name": "other", "options": { "thresholds": { "critical": 1.0 } } },
            {
                "name": "[staging] physician chat generation errors",
                "options": { "thresholds": { "critical": 5.0 } },
                "overall_state": "Alert",
                "overall_state_modified": "2026-11-10T17:00:00+00:00",
                "state": { "groups": {} }
            }
        ]);
        let monitor =
            named_monitor(&monitors, "[staging] physician chat generation errors").unwrap();
        assert_eq!(monitor.threshold, Some(5.0));
        assert_eq!(monitor.groups.len(), 1);
        assert_eq!(monitor.groups[0].status, "Alert");
        assert_eq!(monitor.groups[0].last_triggered_ts, Some(1_794_330_000));
        assert_eq!(monitor.groups[0].last_resolved_ts, None);
        assert!(named_monitor(&monitors, "missing").is_none());

        let grouped = json!([{
            "name": "m",
            "options": { "thresholds": { "critical": 5.0 } },
            "overall_state": "OK",
            "state": { "groups": { "*": { "status": "OK", "last_resolved_ts": 7 } } }
        }]);
        let monitor = named_monitor(&grouped, "m").unwrap();
        assert_eq!(monitor.groups[0].last_resolved_ts, Some(7));
    }
}

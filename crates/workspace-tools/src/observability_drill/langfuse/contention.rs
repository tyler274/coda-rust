//! The judgments of `burst` and `hard-stop`: how long a wave of held turns
//! may take to drain, what each turn's samples say, and what a recovered
//! hard stop must leave behind. Parsing and judging are pure so they are
//! unit-tested; the transport lives in `mod.rs`.
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use super::readback::AttemptSpan;

/// Turns one worker instance runs at once (`max_instance_request_concurrency`
/// on the worker in `infra/gcp/rust-environments/workloads.tf`).
pub(super) const TURNS_PER_INSTANCE: u32 = 4;
/// What the drain bound allows beyond the held rounds: discovery, the model
/// call and the snapshot poll.
const DRAIN_SLACK: Duration = Duration::from_secs(10);

/// How long a wave of `turns` turns, each held `hold`, may take from its
/// first submit to its last reply on `instances` worker instances:
/// `ceil(turns / (4 x instances)) x hold + 10 s`.
pub(super) fn drain_bound(turns: u32, instances: u32, hold: Duration) -> Duration {
    let slots = TURNS_PER_INSTANCE * instances.max(1);
    hold * turns.div_ceil(slots) + DRAIN_SLACK
}

/// The worker instances writing the queue gauges, and the turns they count
/// due now, from a Cloud Monitoring `timeSeries.list` answer for the `due`
/// gauge. Each instance writes its own series; the newest point of each is
/// its current count. `None` when no instance wrote a point, which proves
/// nothing either way.
pub(super) fn queue_due(body: &Value) -> Option<QueueDue> {
    let newest: Vec<u64> = body["timeSeries"]
        .as_array()?
        .iter()
        .filter_map(|series| {
            // Cloud Monitoring lists a series' points newest first.
            let value = &series["points"].as_array()?.first()?["value"]["int64Value"];
            value.as_str()?.parse().ok()
        })
        .collect();
    let instances = u32::try_from(newest.len())
        .ok()
        .filter(|count| *count > 0)?;
    Some(QueueDue {
        instances,
        due: newest.iter().sum(),
    })
}

/// What the queue gauges say right now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(super) struct QueueDue {
    pub(super) instances: u32,
    pub(super) due: u64,
}

/// One burst turn's samples. A sample the drill could not read stays `None`
/// and is listed as missing, never counted as zero.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct TurnSample {
    pub(super) chat_id: String,
    pub(super) submitted: DateTime<Utc>,
    pub(super) completed: Option<DateTime<Utc>>,
    pub(super) attempt_id: Option<String>,
    pub(super) span_start: Option<DateTime<Utc>>,
    pub(super) span_duration_ms: Option<i64>,
    pub(super) hold_ms: Option<i64>,
}

impl TurnSample {
    pub(super) fn submitted(chat_id: String, submitted: DateTime<Utc>) -> Self {
        Self {
            chat_id,
            submitted,
            completed: None,
            attempt_id: None,
            span_start: None,
            span_duration_ms: None,
            hold_ms: None,
        }
    }

    /// Takes what the attempt span says about the turn.
    pub(super) fn spanned(&mut self, span: &AttemptSpan) {
        self.span_start = span.start;
        self.span_duration_ms = span.duration_ms;
        self.hold_ms = span.rehearsed_hold_ms;
    }

    /// From submit to the attempt's start: how long the turn waited for a
    /// worker slot.
    pub(super) fn queue_delay_ms(&self) -> Option<i64> {
        self.span_start
            .map(|start| (start - self.submitted).num_milliseconds())
    }

    /// The samples this turn lacks, by name. `held` says whether the turn
    /// carried a hold marker, so its span must record the hold.
    pub(super) fn missing(&self, held: bool) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.completed.is_none() {
            missing.push("completion");
        }
        if self.attempt_id.is_none() {
            missing.push("attempt id");
        }
        if self.span_start.is_none() {
            missing.push("attempt span start");
        }
        if self.span_duration_ms.is_none() {
            missing.push("attempt span duration");
        }
        if held && self.hold_ms.is_none() {
            missing.push("rehearsed hold");
        }
        missing
    }
}

/// One wave's outcome against its drain bound.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(super) struct WaveSummary {
    pub(super) turns: usize,
    pub(super) bound_seconds: f64,
    /// First submit to last reply; `None` until every turn replied.
    pub(super) drained_seconds: Option<f64>,
    pub(super) max_queue_delay_ms: Option<i64>,
    pub(super) median_queue_delay_ms: Option<i64>,
    /// `chat id: sample` for every sample the drill could not read.
    pub(super) missing: Vec<String>,
}

impl WaveSummary {
    pub(super) fn of(samples: &[TurnSample], bound: Duration, held: bool) -> Self {
        let first = samples.iter().map(|sample| sample.submitted).min();
        let last = samples
            .iter()
            .map(|sample| sample.completed)
            .collect::<Option<Vec<_>>>()
            .and_then(|done| done.into_iter().max());
        let mut delays: Vec<i64> = samples
            .iter()
            .filter_map(TurnSample::queue_delay_ms)
            .collect();
        delays.sort_unstable();
        Self {
            turns: samples.len(),
            bound_seconds: bound.as_secs_f64(),
            drained_seconds: first.zip(last).map(|(first, last)| seconds(last - first)),
            max_queue_delay_ms: delays.last().copied(),
            median_queue_delay_ms: delays.get(delays.len() / 2).copied(),
            missing: samples
                .iter()
                .flat_map(|sample| {
                    sample
                        .missing(held)
                        .into_iter()
                        .map(|what| format!("{}: {what}", sample.chat_id))
                })
                .collect(),
        }
    }

    /// Where the wave failed its bound: it did not drain, or drained late.
    pub(super) fn gaps(&self, wave: &str) -> Vec<String> {
        match self.drained_seconds {
            None => vec![format!("{wave} did not drain: a turn never replied")],
            Some(drained) if drained > self.bound_seconds => vec![format!(
                "{wave} drained in {drained:.1} s, past its {:.1} s bound",
                self.bound_seconds
            )],
            Some(_) => Vec::new(),
        }
    }
}

fn seconds(elapsed: chrono::Duration) -> f64 {
    // Millisecond precision is all a drain needs.
    #[expect(
        clippy::cast_precision_loss,
        reason = "a wave's milliseconds are far below f64's exact range"
    )]
    let millis = elapsed.num_milliseconds() as f64;
    millis / 1000.0
}

/// What a recovered hard stop must show: the answering attempt succeeded the
/// crashed one in the same work, the crashed one logged its receipt, and the
/// chat holds exactly one reply.
pub(super) fn hard_stop_gaps(
    successor: &AttemptSpan,
    receipt: Option<&Value>,
    replies: usize,
) -> Vec<String> {
    let mut gaps = Vec::new();
    let Some(crashed) = successor.predecessor_run_id.as_deref() else {
        gaps.push(format!(
            "the answering attempt {} names no predecessor; no attempt was interrupted",
            successor.run_id
        ));
        return gaps;
    };
    match receipt {
        None => gaps.push(format!(
            "no physician.worker.rehearsed_crash log line names attempt {crashed}"
        )),
        Some(receipt) => {
            let work = receipt["jsonPayload"]["coda.work_id"].as_str();
            if work.is_none() || work != successor.work_id.as_deref() {
                gaps.push(format!(
                    "the crash receipt names work {work:?}, the successor works on {:?}",
                    successor.work_id
                ));
            }
        }
    }
    if replies != 1 {
        gaps.push(format!(
            "the chat holds {replies} replies; a recovered turn answers exactly once"
        ));
    }
    gaps
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn the_drain_bound_counts_rounds_of_four_turns_per_instance() {
        let hold = Duration::from_secs(20);
        assert_eq!(drain_bound(12, 1, hold), Duration::from_secs(70));
        assert_eq!(drain_bound(12, 2, hold), Duration::from_secs(50));
        assert_eq!(drain_bound(12, 3, hold), Duration::from_secs(30));
        assert_eq!(drain_bound(12, 4, hold), Duration::from_secs(30));
        assert_eq!(
            drain_bound(12, 0, hold),
            drain_bound(12, 1, hold),
            "no gauge reading still counts the one instance that stays up"
        );
    }

    fn series(newest: &str, older: &str) -> Value {
        json!({ "points": [
            { "value": { "int64Value": newest } },
            { "value": { "int64Value": older } },
        ]})
    }

    #[test]
    fn the_queue_reads_each_instances_newest_point() {
        let body = json!({ "timeSeries": [series("0", "5"), series("2", "0")] });
        assert_eq!(
            queue_due(&body),
            Some(QueueDue {
                instances: 2,
                due: 2
            })
        );
        let idle = json!({ "timeSeries": [series("0", "3")] });
        assert_eq!(queue_due(&idle).map(|queue| queue.due), Some(0));
    }

    #[test]
    fn no_gauge_point_is_not_an_idle_queue() {
        assert_eq!(queue_due(&json!({})), None);
        assert_eq!(queue_due(&json!({ "timeSeries": [] })), None);
        assert_eq!(
            queue_due(&json!({ "timeSeries": [{ "points": [] }] })),
            None
        );
    }

    fn at(seconds: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(1_800_000_000 + seconds, 0).unwrap()
    }

    fn sample(chat: &str, submitted: i64, start: i64, completed: Option<i64>) -> TurnSample {
        TurnSample {
            chat_id: chat.to_owned(),
            submitted: at(submitted),
            completed: completed.map(at),
            attempt_id: Some(format!("run-{chat}")),
            span_start: Some(at(start)),
            span_duration_ms: Some(20_500),
            hold_ms: Some(20_000),
        }
    }

    #[test]
    fn a_wave_drains_from_its_first_submit_to_its_last_reply() {
        let samples = [
            sample("a", 0, 1, Some(22)),
            sample("b", 1, 22, Some(44)),
            sample("c", 2, 3, Some(24)),
        ];
        let wave = WaveSummary::of(&samples, Duration::from_secs(50), true);
        assert_eq!(wave.drained_seconds, Some(44.0));
        assert_eq!(wave.max_queue_delay_ms, Some(21_000));
        assert_eq!(wave.median_queue_delay_ms, Some(1_000));
        assert!(wave.missing.is_empty());
        assert!(wave.gaps("wave 1").is_empty());
        let late = WaveSummary::of(&samples, Duration::from_secs(40), true);
        assert_eq!(late.gaps("wave 1").len(), 1);
    }

    #[test]
    fn a_missing_sample_is_listed_never_zero() {
        let mut unread = TurnSample::submitted("b".into(), at(1));
        unread.completed = Some(at(30));
        let wave = WaveSummary::of(
            &[sample("a", 0, 1, Some(22)), unread],
            Duration::from_secs(50),
            true,
        );
        assert_eq!(
            wave.max_queue_delay_ms,
            Some(1_000),
            "only read delays count"
        );
        assert_eq!(
            wave.missing,
            [
                "b: attempt id",
                "b: attempt span start",
                "b: attempt span duration",
                "b: rehearsed hold"
            ]
        );
        let unheld = TurnSample {
            hold_ms: None,
            ..sample("c", 0, 1, Some(5))
        };
        assert!(unheld.missing(false).is_empty());
    }

    #[test]
    fn a_turn_that_never_replied_leaves_the_wave_undrained() {
        let wave = WaveSummary::of(
            &[sample("a", 0, 1, Some(22)), sample("b", 0, 1, None)],
            Duration::from_secs(50),
            true,
        );
        assert_eq!(wave.drained_seconds, None);
        assert_eq!(wave.missing, ["b: completion"]);
        assert_eq!(wave.gaps("wave 2").len(), 1);
    }

    fn successor(predecessor: Option<&str>) -> AttemptSpan {
        AttemptSpan {
            run_id: "run-2".into(),
            work_id: Some("work-1".into()),
            predecessor_run_id: predecessor.map(str::to_owned),
            error_type: None,
            trace_relation: Some("link".into()),
            start: None,
            duration_ms: None,
            rehearsed_hold_ms: Some(30_000),
        }
    }

    fn receipt(work: &str) -> Value {
        json!({ "jsonPayload": {
            "event": "physician.worker.rehearsed_crash",
            "coda.run_id": "run-1",
            "coda.work_id": work,
        }})
    }

    #[test]
    fn a_recovered_hard_stop_links_its_attempts_and_answers_once() {
        let linked = successor(Some("run-1"));
        assert!(hard_stop_gaps(&linked, Some(&receipt("work-1")), 1).is_empty());
        assert_eq!(
            hard_stop_gaps(&linked, Some(&receipt("work-9")), 1).len(),
            1
        );
        assert_eq!(hard_stop_gaps(&linked, None, 1).len(), 1);
        assert_eq!(
            hard_stop_gaps(&linked, Some(&receipt("work-1")), 2).len(),
            1
        );
        assert_eq!(
            hard_stop_gaps(&linked, Some(&receipt("work-1")), 0).len(),
            1
        );
    }

    #[test]
    fn an_answer_without_a_predecessor_never_crashed() {
        let gaps = hard_stop_gaps(&successor(None), Some(&receipt("work-1")), 1);
        assert_eq!(gaps.len(), 1);
        assert!(gaps[0].contains("no predecessor"), "{gaps:?}");
    }
}

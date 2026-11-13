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

/// The worker instances writing one queue gauge, and the count it reads,
/// from a Cloud Monitoring `timeSeries.list` answer. Each instance writes its
/// own series from the same rows, so the newest point of each is the same
/// count read at a different moment, and the largest is the reading. Only
/// points written at or after `since` count, so an old zero cannot vouch for
/// the queue now. `None` when no instance wrote such a point, which proves
/// nothing either way.
pub(super) fn queue_gauge(body: &Value, since: DateTime<Utc>) -> Option<QueueGauge> {
    let newest: Vec<u64> = body["timeSeries"]
        .as_array()?
        .iter()
        .filter_map(|series| {
            // Cloud Monitoring lists a series' points newest first.
            let point = series["points"].as_array()?.first()?;
            let written: DateTime<Utc> = point["interval"]["endTime"].as_str()?.parse().ok()?;
            if written < since {
                return None;
            }
            point["value"]["int64Value"].as_str()?.parse().ok()
        })
        .collect();
    let instances = u32::try_from(newest.len())
        .ok()
        .filter(|count| *count > 0)?;
    Some(QueueGauge {
        instances,
        count: newest.iter().copied().max().unwrap_or(0),
    })
}

/// What one queue gauge says right now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(super) struct QueueGauge {
    pub(super) instances: u32,
    pub(super) count: u64,
}

/// The queue before a hard stop: work runnable now and work a live lease
/// holds. `None` for a gauge no instance wrote.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(super) struct QueueNow {
    pub(super) due: Option<QueueGauge>,
    pub(super) running: Option<QueueGauge>,
}

impl QueueNow {
    /// Why a crash now would cost more than the drill's own turn: an abort
    /// drops every attempt on the instance, so the queue must hold nothing
    /// due and nothing running, and an unread gauge proves neither.
    pub(super) fn busy(&self) -> Vec<String> {
        let mut busy = Vec::new();
        for (name, gauge) in [("due", self.due), ("running", self.running)] {
            match gauge {
                None => busy.push(format!(
                    "no worker wrote the {name} gauge after the drill began"
                )),
                Some(gauge) if gauge.count > 0 => busy.push(format!(
                    "{} turns are {name}; a crash could drop theirs, so the drill needs an idle queue",
                    gauge.count
                )),
                Some(_) => {}
            }
        }
        busy
    }
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

    /// Where the wave failed: it did not drain, drained late, or left a
    /// sample unread, since a wave judged on partial samples proves nothing.
    pub(super) fn gaps(&self, wave: &str) -> Vec<String> {
        let mut gaps = match self.drained_seconds {
            None => vec![format!("{wave} did not drain: a turn never replied")],
            Some(drained) if drained > self.bound_seconds => vec![format!(
                "{wave} drained in {drained:.1} s, past its {:.1} s bound",
                self.bound_seconds
            )],
            Some(_) => Vec::new(),
        };
        gaps.extend(
            self.missing
                .iter()
                .map(|missing| format!("{wave} lacks {missing}")),
        );
        gaps
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

/// How long a hard-stopped turn may take from submit to its reply: the
/// 30-second held first attempt, the 60-second lease expiry, discovery and
/// the successor's own hold fit well inside it.
pub(super) const HARD_STOP_LIMIT: Duration = Duration::from_secs(300);

/// What a recovered hard stop must show: the reply came within
/// [`HARD_STOP_LIMIT`] of the submit, the answering attempt succeeded the
/// crashed one in the same work, the crashed one logged its receipt, and the
/// chat holds exactly one reply.
pub(super) fn hard_stop_gaps(
    recovered_in: chrono::Duration,
    successor: &AttemptSpan,
    receipt: Option<&Value>,
    replies: usize,
) -> Vec<String> {
    let mut gaps = Vec::new();
    let limit = chrono::Duration::from_std(HARD_STOP_LIMIT).unwrap_or(chrono::Duration::MAX);
    if recovered_in > limit {
        gaps.push(format!(
            "the turn answered {:.1} s after submit, past the {} s recovery limit",
            seconds(recovered_in),
            HARD_STOP_LIMIT.as_secs()
        ));
    }
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

    fn point(value: &str, written: i64) -> Value {
        json!({
            "interval": { "endTime": at(written).to_rfc3339() },
            "value": { "int64Value": value },
        })
    }

    fn series(newest: &str, older: &str) -> Value {
        json!({ "points": [point(newest, 60), point(older, 0)] })
    }

    #[test]
    fn the_queue_reads_each_instances_newest_point() {
        let body = json!({ "timeSeries": [series("0", "5"), series("2", "0")] });
        assert_eq!(
            queue_gauge(&body, at(30)),
            Some(QueueGauge {
                instances: 2,
                count: 2
            })
        );
        let same_rows = json!({ "timeSeries": [series("3", "0"), series("3", "0")] });
        assert_eq!(
            queue_gauge(&same_rows, at(30)).map(|gauge| gauge.count),
            Some(3),
            "instances count the same rows, so their readings are not added"
        );
        let idle = json!({ "timeSeries": [series("0", "3")] });
        assert_eq!(queue_gauge(&idle, at(30)).map(|gauge| gauge.count), Some(0));
    }

    #[test]
    fn a_point_written_before_the_drill_began_does_not_count() {
        let stale = json!({ "timeSeries": [series("0", "0")] });
        assert_eq!(
            queue_gauge(&stale, at(61)),
            None,
            "an old zero cannot vouch for the queue now"
        );
        let mixed = json!({ "timeSeries": [
            { "points": [point("3", 10)] },
            { "points": [point("0", 90)] },
        ]});
        assert_eq!(
            queue_gauge(&mixed, at(61)),
            Some(QueueGauge {
                instances: 1,
                count: 0
            }),
            "only the instance that wrote since counts"
        );
        let untimed = json!({ "timeSeries": [{ "points": [{ "value": { "int64Value": "0" } }] }] });
        assert_eq!(queue_gauge(&untimed, at(0)), None);
    }

    #[test]
    fn no_gauge_point_is_not_an_idle_queue() {
        assert_eq!(queue_gauge(&json!({}), at(0)), None);
        assert_eq!(queue_gauge(&json!({ "timeSeries": [] }), at(0)), None);
        assert_eq!(
            queue_gauge(&json!({ "timeSeries": [{ "points": [] }] }), at(0)),
            None
        );
    }

    #[test]
    fn a_hard_stop_needs_nothing_due_and_nothing_running() {
        let gauge = |count| {
            Some(QueueGauge {
                instances: 1,
                count,
            })
        };
        let idle = QueueNow {
            due: gauge(0),
            running: gauge(0),
        };
        assert!(idle.busy().is_empty());
        let running = QueueNow {
            running: gauge(2),
            ..idle
        };
        assert_eq!(
            running.busy().len(),
            1,
            "leased work is not due but a crash drops it"
        );
        assert!(running.busy()[0].contains("2 turns are running"));
        let unread = QueueNow {
            running: None,
            ..idle
        };
        assert_eq!(
            unread.busy().len(),
            1,
            "an unread gauge is not an idle queue"
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
            wave.gaps("wave 1").len(),
            4,
            "a wave that drained in bound still fails on each unread sample"
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
        assert_eq!(
            wave.gaps("wave 2"),
            [
                "wave 2 did not drain: a turn never replied",
                "wave 2 lacks b: completion"
            ]
        );
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

    fn within() -> chrono::Duration {
        chrono::Duration::seconds(120)
    }

    #[test]
    fn a_recovered_hard_stop_links_its_attempts_and_answers_once() {
        let linked = successor(Some("run-1"));
        let work = receipt("work-1");
        assert!(hard_stop_gaps(within(), &linked, Some(&work), 1).is_empty());
        assert_eq!(
            hard_stop_gaps(within(), &linked, Some(&receipt("work-9")), 1).len(),
            1
        );
        assert_eq!(hard_stop_gaps(within(), &linked, None, 1).len(), 1);
        assert_eq!(hard_stop_gaps(within(), &linked, Some(&work), 2).len(), 1);
        assert_eq!(hard_stop_gaps(within(), &linked, Some(&work), 0).len(), 1);
    }

    #[test]
    fn a_hard_stop_recovered_past_five_minutes_fails() {
        let linked = successor(Some("run-1"));
        let work = receipt("work-1");
        let limit = chrono::Duration::seconds(300);
        assert!(hard_stop_gaps(limit, &linked, Some(&work), 1).is_empty());
        let late = hard_stop_gaps(
            limit + chrono::Duration::seconds(1),
            &linked,
            Some(&work),
            1,
        );
        assert_eq!(late.len(), 1);
        assert!(late[0].contains("recovery limit"), "{late:?}");
    }

    #[test]
    fn an_answer_without_a_predecessor_never_crashed() {
        let gaps = hard_stop_gaps(within(), &successor(None), Some(&receipt("work-1")), 1);
        assert_eq!(gaps.len(), 1);
        assert!(gaps[0].contains("no predecessor"), "{gaps:?}");
    }
}

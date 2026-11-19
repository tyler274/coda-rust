//! Honest freshness reporting across the clinical library and each of its
//! upstream sources, derived strictly from recorded Postgres state.

use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, TimeDelta, Utc};
use sea_orm::{EntityTrait, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::alert::Alert;
use crate::corpus::{LatencySummary, source_wire_identity};
use crate::migration::{
    item, night, pubmed_status, slot as slot_table, source_status, unpublished,
};
use crate::night::{DeferralReason, NightOutcome};
use crate::slot::SlotName;
use crate::source::Source;
use crate::store::{Store, StoreError, outcome_from_row};

/// Maximum acceptable age for PubMed retraction/correction checks (1 day).
pub const PUBMED_MAX_AGE_DAYS: i64 = 1;

/// Maximum acceptable age for upstream content updates (3 days).
pub const CONTENT_UPDATE_MAX_AGE_DAYS: i64 = 3;

/// Maximum acceptable interval between full library re-confirmations (7 days).
pub const RECONFIRM_MAX_AGE_DAYS: i64 = 7;

/// Overall standing of the clinical library corpus.
///
/// The library never reports [`LibraryStanding::Current`] when any source has
/// never completed refresh, when the latest night failed or deferred, when any
/// items are waiting/pending person resolution, or when admitted content has
/// not yet been activated into a serving version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum LibraryStanding {
    /// Every source is confirmed without pending failures or unpublished lag.
    Current,
    /// At least one source or nightly invariant is stale, lagging, or failed.
    Stale {
        /// Human-readable summary naming why the library is not current.
        reason: String,
    },
}

/// Freshness state for one individual clinical library source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum SourceFreshnessState {
    /// No successful refresh check has ever completed for this source.
    NeverRefreshed,
    /// Source completed refresh with no pending retries, waiting items, or errors.
    Current {
        /// ISO-8601 UTC timestamp when this source was last confirmed current.
        last_confirmed_at: String,
    },
    /// Source is behind due to errors, retries, waiting items, or unpublished content.
    Lagging {
        /// Last confirmation timestamp if a prior refresh succeeded.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_confirmed_at: Option<String>,
        /// Items awaiting automatic retry.
        pending_retry: u64,
        /// Items waiting for an upstream or reader fix.
        waiting_change: u64,
        /// Items requiring operator intervention.
        needs_person: u64,
        /// Newly admitted items not yet activated in a serving version.
        unpublished: u64,
        /// Last recorded failure code for this source, if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_error_code: Option<String>,
    },
}

/// Per-source freshness breakdown inside [`LibraryFreshnessReport`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFreshness {
    /// Wire source identifier (`"pubmed"`, `"pmc"`, `"dailymed"`, `"cdc-mmwr"`, `"uspstf"`).
    pub source_code: String,
    /// Human-readable source name.
    pub source_display_name: String,
    /// Current freshness state for this source.
    pub status: SourceFreshnessState,
    /// Total admitted documents/items held for this source.
    pub admitted_count: u64,
}

/// Complete library freshness and serving health report returned by `GET /v1/freshness`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryFreshnessReport {
    /// Overall freshness standing (`current` vs `stale` with named reason).
    #[serde(flatten)]
    pub standing: LibraryStanding,
    /// Currently active serving version identifier, if one has been activated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_version: Option<String>,
    /// ISO-8601 UTC timestamp of the most recent recorded nightly run, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_night_recorded_at: Option<String>,
    /// Outcome label for the most recent nightly run (`"succeeded"`, `"deferred:..."`, `"failed:..."`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_night_outcome: Option<String>,
    /// Per-source freshness standing across every configured collection.
    pub sources: Vec<SourceFreshness>,
    /// Count of open operator alerts requiring human review.
    pub open_alerts: usize,
    /// Observed search endpoint latency summary.
    pub search_latency: LatencySummary,
}

#[derive(Default)]
struct SourceItemTallies {
    admitted: u64,
    pending_retry: u64,
    waiting_change: u64,
    needs_person: u64,
}

impl Store {
    /// Computes an honest [`LibraryFreshnessReport`] strictly from Postgres state at the current UTC clock.
    pub async fn freshness_report(&self) -> Result<LibraryFreshnessReport, StoreError> {
        self.freshness_report_at(DateTime::<Utc>::from(std::time::SystemTime::now()))
            .await
    }

    /// Computes an honest [`LibraryFreshnessReport`] against `now` so SLA age thresholds
    /// can be verified deterministically without mutating wall clocks.
    pub async fn freshness_report_at(
        &self,
        now: DateTime<Utc>,
    ) -> Result<LibraryFreshnessReport, StoreError> {
        let active_version = self.active_version().await?.map(|v| v.as_str().to_owned());

        let latest_night = night::Entity::find()
            .order_by_desc(night::Column::Id)
            .one(self.db())
            .await?;

        let (last_night_recorded_at, last_night_outcome, night_issue) = match latest_night {
            None => (
                None,
                None,
                Some("clinical library has not completed an initial nightly refresh".to_owned()),
            ),
            Some(row) => {
                let recorded_dt = row.recorded_at.with_timezone(&Utc);
                let recorded = Some(recorded_dt.to_rfc3339());
                let parsed = outcome_from_row(row)?;
                let mut issue = match &parsed {
                    NightOutcome::Succeeded(_) => None,
                    NightOutcome::Deferred(deferred) => {
                        let reason_label = format_deferral_label(deferred.reason());
                        Some(format!("latest nightly refresh deferred ({reason_label})"))
                    }
                    NightOutcome::Failed(failed) => {
                        let code = failed.code().as_str().to_owned();
                        Some(format!("latest nightly refresh failed ({code})"))
                    }
                };
                if issue.is_none()
                    && now.signed_duration_since(recorded_dt)
                        > TimeDelta::days(RECONFIRM_MAX_AGE_DAYS)
                {
                    issue = Some(format!(
                        "latest nightly refresh exceeds {RECONFIRM_MAX_AGE_DAYS}-day re-confirmation SLA"
                    ));
                }
                let outcome_label = match &parsed {
                    NightOutcome::Succeeded(_) => "succeeded".to_owned(),
                    NightOutcome::Deferred(deferred) => {
                        format!("deferred:{}", format_deferral_label(deferred.reason()))
                    }
                    NightOutcome::Failed(failed) => format!("failed:{}", failed.code()),
                };
                (recorded, Some(outcome_label), issue)
            }
        };

        let mut tallies: BTreeMap<Source, SourceItemTallies> = BTreeMap::new();
        for item_row in item::Entity::find().all(self.db()).await? {
            let Some(source) = Source::parse(&item_row.source) else {
                continue;
            };
            let entry = tallies.entry(source).or_default();
            match item_row.state.as_str() {
                "admitted" => entry.admitted += 1,
                "waiting_to_retry" => entry.pending_retry += 1,
                "waiting_for_change" => entry.waiting_change += 1,
                "needs_person" => entry.needs_person += 1,
                _ => {}
            }
        }

        let mut unpublished_counts: BTreeMap<Source, u64> = BTreeMap::new();
        for unpub_row in unpublished::Entity::find().all(self.db()).await? {
            if let Some(source) = Source::parse(&unpub_row.source) {
                *unpublished_counts.entry(source).or_insert(0) += 1;
            }
        }

        let mut status_rows = BTreeMap::new();
        for row in source_status::Entity::find().all(self.db()).await? {
            if let Some(source) = Source::parse(&row.source) {
                status_rows.insert(source, row);
            }
        }

        let pubmed_max_confirmed = pubmed_status::Entity::find()
            .order_by_desc(pubmed_status::Column::ConfirmedAt)
            .one(self.db())
            .await?
            .map(|row| row.confirmed_at.with_timezone(&Utc));

        let mut sources = Vec::with_capacity(Source::ALL.len());
        let mut stale_reasons: Vec<String> = Vec::new();
        if let Some(issue) = night_issue {
            stale_reasons.push(issue);
        }

        for source in Source::ALL {
            let (source_code, source_display_name) = source_wire_identity(source);
            let count = tallies.get(&source);
            let admitted_count = count.map_or(0, |t| t.admitted);
            let pending_retry = count.map_or(0, |t| t.pending_retry);
            let waiting_change = count.map_or(0, |t| t.waiting_change);
            let needs_person = count.map_or(0, |t| t.needs_person);
            let unpublished = unpublished_counts.get(&source).copied().unwrap_or(0);

            let status_row = status_rows.get(&source);
            let status_confirmed = status_row
                .and_then(|r| r.last_succeeded_at)
                .map(|ts| ts.with_timezone(&Utc));
            let confirmed_dt = match source {
                Source::PubMed => match (status_confirmed, pubmed_max_confirmed) {
                    (Some(a), Some(b)) => Some(a.max(b)),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                },
                _ => status_confirmed,
            };
            let last_confirmed_at = confirmed_dt.map(|dt| dt.to_rfc3339());
            let last_error_code = status_row.and_then(|r| r.last_error_code.clone());
            let max_age_days = match source {
                Source::PubMed => PUBMED_MAX_AGE_DAYS,
                Source::Pmc | Source::DailyMed | Source::Mmwr | Source::Uspstf => {
                    CONTENT_UPDATE_MAX_AGE_DAYS
                }
            };
            let sla_overdue = confirmed_dt
                .is_some_and(|dt| now.signed_duration_since(dt) > TimeDelta::days(max_age_days));

            let has_lag = pending_retry > 0
                || waiting_change > 0
                || needs_person > 0
                || unpublished > 0
                || last_error_code.is_some()
                || sla_overdue;

            let status = if has_lag {
                if sla_overdue {
                    stale_reasons.push(format!(
                        "{source_display_name} exceeded {max_age_days}-day freshness SLA"
                    ));
                } else {
                    stale_reasons
                        .push(format!("{source_display_name} has lagging or failed items"));
                }
                SourceFreshnessState::Lagging {
                    last_confirmed_at,
                    pending_retry,
                    waiting_change,
                    needs_person,
                    unpublished,
                    last_error_code,
                }
            } else if let Some(ts) = last_confirmed_at {
                SourceFreshnessState::Current {
                    last_confirmed_at: ts,
                }
            } else {
                stale_reasons.push(format!("{source_display_name} has not completed refresh"));
                SourceFreshnessState::NeverRefreshed
            };

            sources.push(SourceFreshness {
                source_code: source_code.to_owned(),
                source_display_name: source_display_name.to_owned(),
                status,
                admitted_count,
            });
        }

        if active_version.is_none()
            && tallies
                .iter()
                .any(|(src, t)| src.publishes_through_versions() && t.admitted > 0)
        {
            stale_reasons.push("admitted content has no active serving version".to_owned());
        }

        let mut all_slot_names = BTreeSet::new();
        for s in slot_table::Entity::find().all(self.db()).await? {
            all_slot_names.insert(s.slot);
        }
        for n in night::Entity::find().all(self.db()).await? {
            all_slot_names.insert(n.slot);
        }
        let mut slot_alert_count = 0_usize;
        let mut saw_item_alert = false;
        for slot_str in all_slot_names {
            let Ok(slot_name) = SlotName::new(slot_str) else {
                continue;
            };
            for alert in self.alerts(&slot_name, &Source::ALL).await? {
                match alert {
                    Alert::ItemsNeedPerson { .. } => {
                        if !saw_item_alert {
                            saw_item_alert = true;
                            slot_alert_count += 1;
                        }
                    }
                    Alert::Stopped(_) | Alert::OwnerUnreadable | Alert::NotUpdating { .. } => {
                        slot_alert_count += 1;
                    }
                }
            }
        }
        let open_alerts = slot_alert_count;
        if open_alerts > 0 {
            stale_reasons.push(format!("{open_alerts} open clinical library alert(s)"));
        }

        let search_latency = self.latency_summary("search").await?;
        let standing = if stale_reasons.is_empty() {
            LibraryStanding::Current
        } else {
            LibraryStanding::Stale {
                reason: stale_reasons.join("; "),
            }
        };

        Ok(LibraryFreshnessReport {
            standing,
            active_version,
            last_night_recorded_at,
            last_night_outcome,
            sources,
            open_alerts,
            search_latency,
        })
    }
}

fn format_deferral_label(reason: &DeferralReason) -> String {
    match reason {
        DeferralReason::Stopped(why) => format!("stopped:{}", why.as_str()),
        other => other.as_str().to_owned(),
    }
}

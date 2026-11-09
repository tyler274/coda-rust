//! One night's PubMed status refresh over a list of articles.
//!
//! The night asks NCBI for each article that is due, in batches, and records
//! what came back one article at a time. An article is due when it is new,
//! waiting to retry, or admitted (so its status is re-confirmed); one that
//! waits for a change or needs a person isn't tried.
//!
//! For each article:
//!
//! - **NLM sent its record:** a new or retrying article is admitted with it.
//!   For an admitted article, an unchanged record re-confirms the stored
//!   status, and a changed one is new content from the source, which
//!   replaces it.
//! - **NLM sent a record this reader can't read:** our own defect. The
//!   article waits for a change and tells a person at once, and the last
//!   good status stays.
//! - **NLM sent nothing for it, or the request failed for an outside
//!   reason:** a new or retrying article counts one outside failure. An
//!   admitted article keeps its stored status and state; its status simply
//!   isn't re-confirmed tonight, which the freshness report shows.
//!
//! Each of these sets aside only the article it touches. A request NCBI
//! refuses outright, such as for an invalid API key, fails the night.

use std::collections::BTreeSet;
use std::sync::Arc;

use super::client::{BATCH, FetchError, StatusSource};
use super::efetch::{self, Answer, RecordError, ResponseError};
use super::record::{Pmid, StatusRecord};
use crate::article::{ArticleEvent, ArticleState, WaitReason};
use crate::failure::FailureCode;
use crate::night::NightOutcome;
use crate::slot::Lease;
use crate::source::ItemKey;
use crate::store::{Store, StoreError};

/// What one night's check found for one article.
#[derive(Clone, Debug)]
pub enum StatusCheck {
    /// NLM's record, which names its article.
    Found(StatusRecord),
    /// NLM sent a record this reader couldn't read.
    Unreadable {
        /// The article.
        pmid: Pmid,
        /// The record's hash as sent.
        record_sha256: [u8; 32],
        /// What was wrong with it.
        cause: RecordError,
    },
    /// The article wasn't answered, for a reason that may not repeat.
    Outside {
        /// The article.
        pmid: Pmid,
        /// The failure's stable code.
        code: FailureCode,
        /// What happened.
        cause: Cause,
    },
}

impl StatusCheck {
    /// The code for an article NLM sent no record for.
    pub const MISSING: &str = "pubmed_record_missing";
    /// The code for a response that answered nothing usable.
    pub const UNREADABLE_RESPONSE: &str = "ncbi_unreadable_response";

    /// The article this check is about.
    pub fn pmid(&self) -> Pmid {
        match self {
            Self::Found(record) => record.pmid,
            Self::Unreadable { pmid, .. } | Self::Outside { pmid, .. } => *pmid,
        }
    }
}

/// Why an article went unanswered.
#[derive(Clone, Debug, thiserror::Error)]
pub enum Cause {
    /// NLM sent no record for it. That is never a sign it wasn't retracted.
    #[error("NLM sent no record for it")]
    Missing,
    /// NCBI gave no usable response to its batch.
    #[error("NCBI gave no usable response")]
    Fetch(#[source] Arc<FetchError>),
    /// NCBI's response to its batch answered nothing usable.
    #[error("NCBI's response answered nothing usable")]
    Response(#[source] Arc<ResponseError>),
}

/// How one article's check ended tonight.
#[derive(Clone, Debug)]
pub enum CheckResult {
    /// Its status is admitted or re-confirmed.
    Done,
    /// It was set aside, and why.
    SetAside(SetAside),
}

/// Why an article was set aside, with the failure that caused it.
#[derive(Clone, Debug, thiserror::Error)]
pub enum SetAside {
    /// An outside failure; it is tried again on a later night.
    #[error("outside failure {code}")]
    Outside {
        /// The failure's stable code.
        code: FailureCode,
        /// What happened.
        #[source]
        cause: Cause,
    },
    /// This reader couldn't read NLM's record; a person is told.
    #[error("this reader couldn't read NLM's record")]
    OwnDefect(#[source] RecordError),
}

impl SetAside {
    /// The outside failure's code, or `None` for our own defect.
    pub fn code(&self) -> Option<&FailureCode> {
        match self {
            Self::Outside { code, .. } => Some(code),
            Self::OwnDefect(_) => None,
        }
    }
}

/// What recording one check does: the article's moves, and what happens to
/// its stored status.
#[derive(Clone, Debug)]
pub(crate) struct Plan {
    pub(crate) events: Vec<ArticleEvent>,
    pub(crate) status: StatusWrite,
    pub(crate) checked: CheckResult,
}

/// What happens to an article's stored status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StatusWrite {
    /// The record replaces whatever is stored.
    Replace,
    /// The stored status is unchanged and now re-confirmed.
    Confirm,
    /// The stored status stays as it was.
    Keep,
}

/// Whether tonight checks an article in `state`: a new or retrying article
/// is tried, and an admitted one is re-confirmed.
fn is_checked(state: &ArticleState) -> bool {
    state.is_due() || *state == ArticleState::Admitted
}

/// Decides what recording `check` does to an article in `current`, whose
/// stored record hashes to `stored`. `None` when tonight doesn't check it.
pub(crate) fn plan(
    current: &ArticleState,
    stored: Option<[u8; 32]>,
    check: &StatusCheck,
) -> Option<Plan> {
    if !is_checked(current) {
        return None;
    }
    let admitted = *current == ArticleState::Admitted;
    let plan = match check {
        StatusCheck::Found(record) if admitted && stored == Some(record.record_sha256) => Plan {
            events: Vec::new(),
            status: StatusWrite::Confirm,
            checked: CheckResult::Done,
        },
        StatusCheck::Found(_) => Plan {
            events: changed(admitted, ArticleEvent::Admitted),
            status: StatusWrite::Replace,
            checked: CheckResult::Done,
        },
        StatusCheck::Unreadable { cause, .. } => Plan {
            events: changed(admitted, ArticleEvent::Refused(WaitReason::OwnDefect)),
            status: StatusWrite::Keep,
            checked: CheckResult::SetAside(SetAside::OwnDefect(cause.clone())),
        },
        StatusCheck::Outside { code, cause, .. } => Plan {
            events: if admitted {
                Vec::new()
            } else {
                vec![ArticleEvent::OutsideFailure(code.clone())]
            },
            status: StatusWrite::Keep,
            checked: CheckResult::SetAside(SetAside::Outside {
                code: code.clone(),
                cause: cause.clone(),
            }),
        },
    };
    Some(plan)
}

/// An admitted article's new record is new content from its source, so it
/// moves through being due before tonight's result.
fn changed(admitted: bool, result: ArticleEvent) -> Vec<ArticleEvent> {
    if admitted {
        vec![ArticleEvent::SourceChanged, result]
    } else {
        vec![result]
    }
}

/// What one night's refresh did.
#[derive(Clone, Debug, Default)]
pub struct Report {
    /// Articles checked and admitted or re-confirmed.
    pub done: Vec<Pmid>,
    /// Articles set aside, and why.
    pub set_aside: Vec<(Pmid, SetAside)>,
    /// Articles not tried because they wait for a change or need a person.
    pub not_due: Vec<Pmid>,
}

impl Report {
    /// The night's one outcome.
    pub fn outcome(&self) -> NightOutcome {
        NightOutcome::recorded(self.done.len(), self.set_aside.len())
    }
}

/// A refresh that stopped before finishing the list.
#[derive(Debug, thiserror::Error)]
pub enum RefreshError {
    /// NCBI refused the request itself, so every batch would fail the same
    /// way. The night fails with this code.
    #[error("NCBI refused the status request")]
    Refused(#[source] FetchError),
    /// The records couldn't be read or written, or the run lost its slot.
    #[error("the clinical library's records failed")]
    Store(#[from] StoreError),
}

impl RefreshError {
    /// The failure code the night records, if the night can still record
    /// one. A run that lost its slot records nothing: the night is the new
    /// holder's.
    pub fn code(&self) -> Option<FailureCode> {
        match self {
            Self::Refused(error) => Some(error.code()),
            Self::Store(StoreError::LeaseLost { .. }) => None,
            Self::Store(_) => Some(FailureCode::known("clinical_library_store_failed")),
        }
    }
}

/// Refreshes the status of every article in `pmids`, recording each result
/// under `lease`.
///
/// The list is what qualifies an article for the library: each PMID on it is
/// qualified before it is checked.
pub async fn refresh(
    store: &Store,
    lease: &Lease,
    ncbi: &dyn StatusSource,
    pmids: &[Pmid],
) -> Result<Report, RefreshError> {
    let mut report = Report::default();
    let mut due = Vec::new();
    let unique: BTreeSet<Pmid> = pmids.iter().copied().collect();
    for pmid in unique {
        let state = store.qualify(lease, &ItemKey::pubmed(pmid)).await?;
        if is_checked(&state) {
            due.push(pmid);
        } else {
            report.not_due.push(pmid);
        }
    }

    for batch in due.chunks(BATCH) {
        let checks: Vec<StatusCheck> = match ncbi.efetch(batch).await {
            Ok(body) => match efetch::read(&body, batch) {
                Ok(answers) => answers.into_iter().map(check_from).collect(),
                Err(error) => outside(
                    batch,
                    &FailureCode::known(StatusCheck::UNREADABLE_RESPONSE),
                    &Cause::Response(Arc::new(error)),
                ),
            },
            Err(error) if error.is_outside() => {
                let code = error.code();
                outside(batch, &code, &Cause::Fetch(Arc::new(error)))
            }
            Err(error) => return Err(RefreshError::Refused(error)),
        };
        for check in checks {
            let pmid = check.pmid();
            match store.record_status_check(lease, &check).await? {
                Some(CheckResult::Done) => report.done.push(pmid),
                Some(CheckResult::SetAside(why)) => report.set_aside.push((pmid, why)),
                // Its state changed since the night read it, such as a
                // person acting on it; it isn't tonight's to try.
                None => report.not_due.push(pmid),
            }
        }
    }
    Ok(report)
}

fn check_from((pmid, answer): (Pmid, Answer)) -> StatusCheck {
    match answer {
        Answer::Found(record) => StatusCheck::Found(record),
        Answer::Missing => StatusCheck::Outside {
            pmid,
            code: FailureCode::known(StatusCheck::MISSING),
            cause: Cause::Missing,
        },
        Answer::Unreadable {
            record_sha256,
            cause,
        } => StatusCheck::Unreadable {
            pmid,
            record_sha256,
            cause,
        },
    }
}

fn outside(batch: &[Pmid], code: &FailureCode, cause: &Cause) -> Vec<StatusCheck> {
    batch
        .iter()
        .map(|pmid| StatusCheck::Outside {
            pmid: *pmid,
            code: code.clone(),
            cause: cause.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article::RETRY_LIMIT;

    fn record(sha: u8) -> StatusCheck {
        StatusCheck::Found(StatusRecord {
            pmid: Pmid::new(1).unwrap(),
            publication_types: vec!["Journal Article".into()],
            relations: Vec::new(),
            revised: None,
            record_sha256: [sha; 32],
            record_xml: String::new(),
        })
    }

    /// Applies a plan's moves, as the store does.
    fn after(state: &ArticleState, plan: &Plan) -> ArticleState {
        plan.events
            .iter()
            .try_fold(state.clone(), |state, event| state.apply(event))
            .expect("every planned move is one the article's rules allow")
    }

    #[test]
    fn every_code_this_source_records_is_a_failure_code() {
        let codes = [
            StatusCheck::MISSING,
            StatusCheck::UNREADABLE_RESPONSE,
            "clinical_library_store_failed",
            "ncbi_throttled",
            "ncbi_server_error",
            "ncbi_unreachable",
            "ncbi_response_too_large",
            "ncbi_refused_request",
        ];
        for code in codes {
            assert!(FailureCode::new(code).is_ok(), "{code:?}");
        }
    }

    #[test]
    fn a_new_article_is_admitted_with_its_record() {
        let new = ArticleState::new_article();
        let plan = plan(&new, None, &record(1)).unwrap();
        assert_eq!(plan.status, StatusWrite::Replace);
        assert_eq!(after(&new, &plan), ArticleState::Admitted);
    }

    #[test]
    fn an_unchanged_record_re_confirms_and_a_changed_one_replaces() {
        let admitted = ArticleState::Admitted;
        let same = plan(&admitted, Some([1; 32]), &record(1)).unwrap();
        assert_eq!((same.status, same.events.len()), (StatusWrite::Confirm, 0));
        let changed = plan(&admitted, Some([1; 32]), &record(2)).unwrap();
        assert_eq!(changed.status, StatusWrite::Replace);
        assert_eq!(after(&admitted, &changed), ArticleState::Admitted);
    }

    #[test]
    fn an_unreadable_record_tells_a_person_and_keeps_the_last_good_status() {
        for state in [ArticleState::new_article(), ArticleState::Admitted] {
            let unreadable = StatusCheck::Unreadable {
                pmid: Pmid::new(1).unwrap(),
                record_sha256: [9; 32],
                cause: RecordError::NoPublicationTypes,
            };
            let plan = plan(&state, Some([1; 32]), &unreadable).unwrap();
            assert_eq!(plan.status, StatusWrite::Keep);
            assert!(after(&state, &plan).tells_person_now());
        }
    }

    #[test]
    fn an_outside_failure_counts_only_for_an_article_not_yet_admitted() {
        let missing = StatusCheck::Outside {
            pmid: Pmid::new(1).unwrap(),
            code: FailureCode::known(StatusCheck::MISSING),
            cause: Cause::Missing,
        };
        let mut state = ArticleState::new_article();
        for _ in 0..RETRY_LIMIT {
            let plan = plan(&state, None, &missing).unwrap();
            state = after(&state, &plan);
        }
        assert!(matches!(state, ArticleState::NeedsPerson { .. }));
        assert!(plan(&state, None, &record(1)).is_none());

        let admitted = plan(&ArticleState::Admitted, Some([1; 32]), &missing).unwrap();
        assert!(admitted.events.is_empty());
        assert_eq!(admitted.status, StatusWrite::Keep);
    }
}

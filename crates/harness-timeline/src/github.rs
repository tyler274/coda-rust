//! GitHub pull requests through the `gh` command, signed in as the developer.

use std::path::Path;
use std::process::Command;

use serde::Deserialize;

use crate::error::{CommandError, output};
use crate::event::{CheckRollup, PrState};
use crate::key::Revision;

const FIELDS: &str =
    "number,title,state,isDraft,headRefName,headRefOid,statusCheckRollup,reviewDecision,url";

/// A pull request as the timeline needs it.
#[derive(Clone, Debug, PartialEq)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub branch: String,
    pub state: PrState,
    pub checks: CheckRollup,
    pub review: Option<String>,
    pub head: Revision,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Listed {
    number: u64,
    title: String,
    state: String,
    is_draft: bool,
    head_ref_name: String,
    head_ref_oid: Revision,
    #[serde(default)]
    status_check_rollup: Vec<Check>,
    review_decision: Option<String>,
    url: String,
}

/// A check run (`status` and `conclusion`) or a commit status (`state`).
#[derive(Deserialize)]
struct Check {
    status: Option<String>,
    conclusion: Option<String>,
    state: Option<String>,
}

fn rollup(checks: &[Check]) -> CheckRollup {
    if checks.is_empty() {
        return CheckRollup::None;
    }
    let failed = checks.iter().any(|check| {
        matches!(
            check.conclusion.as_deref().or(check.state.as_deref()),
            Some(
                "FAILURE"
                    | "ERROR"
                    | "CANCELLED"
                    | "TIMED_OUT"
                    | "ACTION_REQUIRED"
                    | "STARTUP_FAILURE"
            )
        )
    });
    if failed {
        return CheckRollup::Failed;
    }
    let pending = checks
        .iter()
        .any(|check| match (&check.status, &check.state) {
            (Some(status), _) => status != "COMPLETED",
            (None, Some(state)) => matches!(state.as_str(), "PENDING" | "EXPECTED"),
            (None, None) => true,
        });
    if pending {
        CheckRollup::Pending
    } else {
        CheckRollup::Passed
    }
}

/// Reads `gh pr list --json` output.
pub fn parse_list(json: &str) -> Result<Vec<PullRequest>, serde_json::Error> {
    let listed: Vec<Listed> = serde_json::from_str(json)?;
    Ok(listed
        .into_iter()
        .map(|pr| PullRequest {
            state: match (pr.state.as_str(), pr.is_draft) {
                ("MERGED", _) => PrState::Merged,
                ("CLOSED", _) => PrState::Closed,
                (_, true) => PrState::Draft,
                _ => PrState::Open,
            },
            checks: rollup(&pr.status_check_rollup),
            number: pr.number,
            title: pr.title,
            branch: pr.head_ref_name,
            review: pr.review_decision.filter(|decision| !decision.is_empty()),
            head: pr.head_ref_oid,
            url: pr.url,
        })
        .collect())
}

/// The repository's most recent pull requests in every state.
pub fn list(repo: &Path, limit: u32) -> Result<Vec<PullRequest>, CommandError> {
    let json = output(Command::new("gh").current_dir(repo).args([
        "pr",
        "list",
        "--state",
        "all",
        "--limit",
        &limit.to_string(),
        "--json",
        FIELDS,
    ]))?;
    parse_list(&json).map_err(|source| CommandError::Unreadable {
        command: "gh pr list".to_owned(),
        output: json.chars().take(200).collect(),
        source: Box::new(source),
    })
}

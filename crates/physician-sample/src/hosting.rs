//! Hosting boundary for the physician runtime.
//!
//! Local runs are unrestricted. Hosted runs (Cloud Run sets `K_SERVICE`, or
//! `CODA_HOSTED` is set) start only under the explicit staging-preview profile:
//! `CODA_ENVIRONMENT=stg`, `CODA_HOSTED_PROFILE=staging-preview`,
//! `PHYSICIAN_AUTH=required` and `PHYSICIAN_DATABASE_URL`. Production and every
//! other combination refuse to start. The profile serves the synthetic practice
//! behind IAP; it is not a clinical-data qualification.

/// The only hosted profile this binary will start under.
pub const STAGING_PROFILE: &str = "staging-preview";

/// Whether this process is a hosted revision.
///
/// Cloud Run sets `K_SERVICE`. `CODA_HOSTED` is the explicit marker. Any
/// value, including an empty string or `false`, still counts as hosted.
pub fn is_hosted(get: impl Fn(&str) -> Option<String>) -> bool {
    get("K_SERVICE").is_some() || get("CODA_HOSTED").is_some()
}

/// Refuses hosted execution unless the staging-preview profile is complete.
pub fn validate_startup(get: impl Fn(&str) -> Option<String>) -> Result<(), String> {
    if !is_hosted(&get) {
        return Ok(());
    }
    if get("CODA_LOCAL_MODEL_EXECUTION").as_deref() == Some("subscription") {
        return Err("Developer ChatGPT plans are available only in local execution".into());
    }
    // Emulator credentials are intentionally absent. Presence alone is enough
    // to refuse: an empty value or "false" must not turn hosted authentication
    // into a local-development exception.
    if ["PUBSUB_EMULATOR_HOST", "PHYSICIAN_LOCAL_STACK"]
        .iter()
        .any(|name| get(name).is_some())
    {
        return Err(
            "Physician hosted execution forbids local stack and Pub/Sub emulator configuration"
                .to_owned(),
        );
    }
    let environment = get("CODA_ENVIRONMENT").unwrap_or_default();
    let profile = get("CODA_HOSTED_PROFILE").unwrap_or_default();
    let auth = get("PHYSICIAN_AUTH").unwrap_or_default();
    let database = get("PHYSICIAN_DATABASE_URL").unwrap_or_default();
    if environment == "stg"
        && profile == STAGING_PROFILE
        && auth == "required"
        && !database.trim().is_empty()
    {
        return Ok(());
    }
    Err(format!(
        "Physician hosted activation is not qualified: hosted runs require CODA_ENVIRONMENT=stg, CODA_HOSTED_PROFILE={STAGING_PROFILE}, PHYSICIAN_AUTH=required and PHYSICIAN_DATABASE_URL; production and every other profile refuse to start"
    ))
}

/// One preview account the staging migration job ensures exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagingAccount {
    /// The lower-cased sign-in email.
    pub email: String,
    /// The account's display name.
    pub display_name: String,
    /// Whether the account administers the practice.
    pub admin: bool,
}

/// Parses `PHYSICIAN_STAGING_ACCOUNTS`: comma-separated
/// `email|Display Name|admin` entries (the last field is `admin` or
/// `physician`). Returns no accounts when the list is unset or blank, and
/// refuses any list outside the staging-preview profile, so a production
/// migration can never create sign-in accounts.
pub fn staging_accounts(
    get: impl Fn(&str) -> Option<String>,
) -> Result<Vec<StagingAccount>, String> {
    let list = get("PHYSICIAN_STAGING_ACCOUNTS").unwrap_or_default();
    if list.trim().is_empty() {
        return Ok(Vec::new());
    }
    if get("CODA_HOSTED_PROFILE").as_deref() != Some(STAGING_PROFILE) {
        return Err(format!(
            "PHYSICIAN_STAGING_ACCOUNTS is only honoured under CODA_HOSTED_PROFILE={STAGING_PROFILE}"
        ));
    }
    list.split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let fields: Vec<&str> = entry.split('|').map(str::trim).collect();
            let [email, display_name, role] = fields[..] else {
                return Err(format!("staging account `{entry}` is not email|name|role"));
            };
            if !email.contains('@') || display_name.is_empty() {
                return Err(format!(
                    "staging account `{entry}` needs an email and a name"
                ));
            }
            let admin = match role {
                "admin" => true,
                "physician" => false,
                other => {
                    return Err(format!(
                        "staging account role `{other}` is not admin or physician"
                    ));
                }
            };
            Ok(StagingAccount {
                email: email.to_lowercase(),
                display_name: display_name.to_owned(),
                admin,
            })
        })
        .collect()
}

/// Practices whose chat is rehearsal traffic, from
/// `PHYSICIAN_SIMULATION_PRACTICES` (comma-separated practice ids).
///
/// A turn in one of these practices is `simulation` in telemetry and may carry
/// a scenario id; every other practice is `real` and may not. Only the
/// staging-preview profile accepts the list, so a production process can never
/// relabel real traffic. An unset or blank list is empty.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SimulationPractices(std::collections::BTreeSet<String>);

impl SimulationPractices {
    /// Reads `PHYSICIAN_SIMULATION_PRACTICES`, refusing it outside the
    /// staging-preview profile.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let list = get("PHYSICIAN_SIMULATION_PRACTICES").unwrap_or_default();
        if list.trim().is_empty() {
            return Ok(Self::default());
        }
        if get("CODA_HOSTED_PROFILE").as_deref() != Some(STAGING_PROFILE) {
            return Err(format!(
                "PHYSICIAN_SIMULATION_PRACTICES is only honoured under CODA_HOSTED_PROFILE={STAGING_PROFILE}"
            ));
        }
        Ok(Self(
            list.split(',')
                .map(str::trim)
                .filter(|practice| !practice.is_empty())
                .map(str::to_owned)
                .collect(),
        ))
    }

    /// The given practices, for tests and in-process composition.
    pub fn of<'a>(practices: impl IntoIterator<Item = &'a str>) -> Self {
        Self(practices.into_iter().map(str::to_owned).collect())
    }

    /// Whether `practice` is a rehearsal practice.
    pub fn contains(&self, practice: &str) -> bool {
        self.0.contains(practice)
    }

    /// Whether the attempt after `earlier_attempts` on a turn in `practice`
    /// fails on purpose: only a rehearsal practice honours a `[fail:K]`
    /// marker in its message (see [`marked_attempt_fails`]). A real practice
    /// never does, and a production process has no rehearsal practices.
    pub fn rehearsed_failure(&self, practice: &str, input: &str, earlier_attempts: u32) -> bool {
        self.contains(practice) && marked_attempt_fails(earlier_attempts, input)
    }

    /// Admits a scenario id sent by a session in `practice`: only a rehearsal
    /// practice may send one, and only a short slug (`[a-z0-9._-]`, at most 64
    /// characters), since it becomes a Langfuse tag.
    pub fn admit_scenario(&self, practice: &str, scenario: &str) -> Result<(), ScenarioRefused> {
        if !self.contains(practice) {
            return Err(ScenarioRefused::RealPractice);
        }
        let slug = !scenario.is_empty()
            && scenario.len() <= 64
            && scenario
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"._-".contains(&b));
        if slug {
            Ok(())
        } else {
            Err(ScenarioRefused::Malformed)
        }
    }

    /// The telemetry origin and scenario for a turn in `practice` that was
    /// accepted with `scenario`. A real practice never reports a scenario,
    /// even one saved before its practice left the list.
    pub fn origin(
        &self,
        practice: &str,
        scenario: Option<&str>,
    ) -> (telemetry::genai::Origin, Option<String>) {
        if self.contains(practice) {
            (
                telemetry::genai::Origin::Simulation,
                scenario.map(str::to_owned),
            )
        } else {
            (telemetry::genai::Origin::Real, None)
        }
    }
}

/// Whether the attempt after `earlier_attempts` fails for a message marked
/// `[fail:K]`: the first K attempts fail. The count is the conversation's
/// saved attempts on this input, so it survives a worker restart and every
/// worker instance agrees. Try again keeps the input, so `[fail:5]`
/// dead-letters on its fifth failure and its Try again then succeeds.
pub fn marked_attempt_fails(earlier_attempts: u32, input: &str) -> bool {
    marked_failures(input).is_some_and(|failures| earlier_attempts < failures)
}

/// K from the first `[fail:K]` in `input`, if K is a whole number.
fn marked_failures(input: &str) -> Option<u32> {
    let (_, rest) = input.split_once("[fail:")?;
    let (count, _) = rest.split_once(']')?;
    count.parse().ok()
}

/// Why a submitted scenario id was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScenarioRefused {
    /// The session's practice is not a rehearsal practice.
    RealPractice,
    /// The id is empty, too long, or not a lower-case slug.
    Malformed,
}

/// The rehearsal sign-in's settings: physician's own direct Cloud Run IAP
/// audience (`PHYSICIAN_IAP_AUDIENCE`) and the one service account IAP must
/// have admitted (`PHYSICIAN_REHEARSAL_PRINCIPAL`).
///
/// Both or neither: with neither, `POST /auth/rehearsal` does not exist. Only
/// the staging-preview profile accepts them, so a production process never
/// serves the route, and the principal must be a service account, never a
/// person.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RehearsalSignIn {
    /// `/projects/NUMBER/locations/REGION/services/physician`.
    pub audience: String,
    /// The lower-cased service-account email.
    pub principal: String,
}

impl RehearsalSignIn {
    /// Reads both settings, refusing a partial pair, a malformed audience, a
    /// principal that is not a service account, or any value outside the
    /// staging-preview profile.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Option<Self>, String> {
        let read = |name| get(name).map(|value| value.trim().to_owned());
        let audience = read("PHYSICIAN_IAP_AUDIENCE").filter(|value| !value.is_empty());
        let principal = read("PHYSICIAN_REHEARSAL_PRINCIPAL").filter(|value| !value.is_empty());
        if audience.is_none() && principal.is_none() {
            return Ok(None);
        }
        if get("CODA_HOSTED_PROFILE").as_deref() != Some(STAGING_PROFILE) {
            return Err(format!(
                "the rehearsal sign-in is only honoured under CODA_HOSTED_PROFILE={STAGING_PROFILE}"
            ));
        }
        let (Some(audience), Some(principal)) = (audience, principal) else {
            return Err(
                "the rehearsal sign-in needs both PHYSICIAN_IAP_AUDIENCE and PHYSICIAN_REHEARSAL_PRINCIPAL"
                    .to_owned(),
            );
        };
        http::iap::validate_cloud_run_audience(&audience)
            .map_err(|reason| format!("PHYSICIAN_IAP_AUDIENCE: {reason}"))?;
        let principal = principal.to_lowercase();
        let service_account = principal.split_once('@').is_some_and(|(name, domain)| {
            !name.is_empty() && domain.ends_with(".iam.gserviceaccount.com")
        });
        if !service_account {
            return Err(
                "PHYSICIAN_REHEARSAL_PRINCIPAL must be a service-account email (NAME@PROJECT.iam.gserviceaccount.com)"
                    .to_owned(),
            );
        }
        Ok(Some(Self {
            audience,
            principal,
        }))
    }
}

/// The folder of built synthetic charts `load-charts` reads. Any profile but
/// staging-preview refuses the load: synthetic patients never enter another
/// profile's database.
pub fn synthetic_charts(
    get: impl Fn(&str) -> Option<String>,
) -> Result<std::path::PathBuf, String> {
    if get("CODA_HOSTED_PROFILE").as_deref() != Some(STAGING_PROFILE) {
        return Err(format!(
            "load-charts only runs under CODA_HOSTED_PROFILE={STAGING_PROFILE}"
        ));
    }
    get("PHYSICIAN_SYNTHETIC_CHARTS")
        .filter(|path| !path.trim().is_empty())
        .map(Into::into)
        .ok_or_else(|| "PHYSICIAN_SYNTHETIC_CHARTS is required".to_owned())
}

/// Stable physician id for one staging email.
///
/// The readable slug is not unique (`a.b@c` and `a-b@c` share it), so a short
/// hash of the exact lower-cased email is appended. A rerun of account
/// creation leaves an existing physician id alone.
pub fn staging_physician_id(email: &str) -> String {
    use sha2::{Digest, Sha256};
    let email = email.trim().to_lowercase();
    let mut slug = String::new();
    let mut dashed = false;
    for character in email.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            dashed = false;
        } else if !dashed && !slug.is_empty() {
            slug.push('-');
            dashed = true;
        }
        if slug.len() >= 32 {
            break;
        }
    }
    let slug = slug.trim_end_matches('-');
    let suffix =
        Sha256::digest(email.as_bytes())
            .iter()
            .take(4)
            .fold(String::new(), |mut out, byte| {
                use std::fmt::Write as _;
                let _ = write!(out, "{byte:02x}");
                out
            });
    if slug.is_empty() {
        format!("physician-{suffix}")
    } else {
        format!("{slug}-{suffix}")
    }
}

/// Readiness for a hosted revision: the auth store and the front-door schema
/// both answer. Anything else, including an unconfigured local run, is 503.
pub async fn ready(
    auth: Option<&physician_auth::PostgresAuthStore>,
    front_door: Option<&front_door::PostgresFrontDoorStore>,
) -> axum::http::StatusCode {
    match (auth, front_door) {
        (Some(auth), Some(front_door))
            if auth.check_schema().await.is_ok() && front_door.check_schema().await.is_ok() =>
        {
            axum::http::StatusCode::OK
        }
        _ => axum::http::StatusCode::SERVICE_UNAVAILABLE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staging_physician_ids_are_stable_and_distinct_per_email() {
        let dotted = staging_physician_id("  A.B@Practice.test ");
        assert_eq!(dotted, staging_physician_id("a.b@practice.test"));
        let dashed = staging_physician_id("a-b@practice.test");
        assert_ne!(dotted, dashed);
        assert!(dotted.starts_with("a-b-practice-test-"));
        assert!(dashed.starts_with("a-b-practice-test-"));
        assert_ne!(
            staging_physician_id("admin@practice.test"),
            staging_physician_id("member@practice.test")
        );
    }

    fn hosted(extra: &[(&str, &str)]) -> Result<(), String> {
        validate_startup(|name| {
            if name == "K_SERVICE" {
                return Some("physician".to_owned());
            }
            extra
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        })
    }

    fn accounts(pairs: &[(&str, &str)]) -> Result<Vec<StagingAccount>, String> {
        staging_accounts(|name| {
            pairs
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        })
    }

    #[test]
    fn synthetic_charts_load_only_under_the_preview_profile() {
        let charts = |profile: Option<&str>| {
            synthetic_charts(|key| match key {
                "PHYSICIAN_SYNTHETIC_CHARTS" => Some("/charts".to_owned()),
                "CODA_HOSTED_PROFILE" => profile.map(str::to_owned),
                _ => None,
            })
        };
        for profile in [Some("production"), Some(""), None] {
            assert!(charts(profile).is_err(), "{profile:?}");
        }
        assert_eq!(charts(Some(STAGING_PROFILE)), Ok("/charts".into()));
    }

    #[test]
    fn staging_accounts_parse_under_the_preview_profile() {
        let parsed = accounts(&[
            ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
            (
                "PHYSICIAN_STAGING_ACCOUNTS",
                " Tyler@CodaHealth.ai|Tyler|admin , doc@practice.test|Doc Two|physician,",
            ),
        ])
        .expect("parsed");
        assert_eq!(
            parsed,
            vec![
                StagingAccount {
                    email: "tyler@codahealth.ai".to_owned(),
                    display_name: "Tyler".to_owned(),
                    admin: true,
                },
                StagingAccount {
                    email: "doc@practice.test".to_owned(),
                    display_name: "Doc Two".to_owned(),
                    admin: false,
                },
            ]
        );
        assert_eq!(accounts(&[]), Ok(Vec::new()));
        assert_eq!(
            accounts(&[("PHYSICIAN_STAGING_ACCOUNTS", "  ")]),
            Ok(Vec::new())
        );
    }

    #[test]
    fn staging_accounts_refuse_outside_the_preview_profile_and_bad_entries() {
        for profile in [None, Some("production"), Some("")] {
            let mut pairs = vec![("PHYSICIAN_STAGING_ACCOUNTS", "a@b.test|A|admin")];
            if let Some(profile) = profile {
                pairs.push(("CODA_HOSTED_PROFILE", profile));
            }
            assert!(accounts(&pairs).is_err(), "{profile:?} must refuse");
        }
        for bad in [
            "a@b.test|A",
            "nobody|A|admin",
            "a@b.test||admin",
            "a@b.test|A|owner",
        ] {
            assert!(
                accounts(&[
                    ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
                    ("PHYSICIAN_STAGING_ACCOUNTS", bad),
                ])
                .is_err(),
                "{bad} must refuse"
            );
        }
    }

    #[test]
    fn simulation_practices_parse_only_under_the_preview_profile() {
        let read = |pairs: &[(&str, &str)]| {
            SimulationPractices::from_env(|name| {
                pairs
                    .iter()
                    .find(|(key, _)| *key == name)
                    .map(|(_, value)| (*value).to_owned())
            })
        };
        let parsed = read(&[
            ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
            (
                "PHYSICIAN_SIMULATION_PRACTICES",
                " coda-rehearsal-practice , ,other",
            ),
        ])
        .unwrap();
        assert!(parsed.contains("coda-rehearsal-practice"));
        assert!(parsed.contains("other"));
        assert!(!parsed.contains("roger-synthetic-practice"));
        assert_eq!(read(&[]), Ok(SimulationPractices::default()));
        for profile in [None, Some("production"), Some("")] {
            let mut pairs = vec![("PHYSICIAN_SIMULATION_PRACTICES", "coda-rehearsal-practice")];
            if let Some(profile) = profile {
                pairs.push(("CODA_HOSTED_PROFILE", profile));
            }
            assert!(read(&pairs).is_err(), "{profile:?} must refuse");
        }
    }

    #[test]
    fn origin_follows_the_practice_and_scenarios_need_a_rehearsal_practice() {
        use telemetry::genai::Origin;
        let practices = SimulationPractices::of(["coda-rehearsal-practice"]);
        assert_eq!(
            practices.origin("coda-rehearsal-practice", Some("canary")),
            (Origin::Simulation, Some("canary".to_owned()))
        );
        assert_eq!(
            practices.origin("roger-synthetic-practice", Some("canary")),
            (Origin::Real, None)
        );
        assert_eq!(
            practices.admit_scenario("coda-rehearsal-practice", "drill.rehearse-1"),
            Ok(())
        );
        assert_eq!(
            practices.admit_scenario("roger-synthetic-practice", "canary"),
            Err(ScenarioRefused::RealPractice)
        );
        for bad in ["", "Upper", "has space", &"x".repeat(65)] {
            assert_eq!(
                practices.admit_scenario("coda-rehearsal-practice", bad),
                Err(ScenarioRefused::Malformed),
                "{bad}"
            );
        }
    }

    #[test]
    fn fail_marker_parses_whole_counts_only() {
        assert_eq!(marked_failures("Retry [fail:1]"), Some(1));
        assert_eq!(marked_failures("[fail:5] then text"), Some(5));
        assert_eq!(marked_failures("Plain turn"), None);
        assert_eq!(marked_failures("[fail:]"), None);
        assert_eq!(marked_failures("[fail:-1]"), None);
        assert_eq!(marked_failures("[fail:2"), None);
        assert_eq!(marked_failures("[fail:x]"), None);
    }

    #[test]
    fn fail_marker_fails_the_first_k_saved_attempts() {
        // `[fail:2]` fails its first two attempts, then succeeds for good.
        assert!(marked_attempt_fails(0, "Retry [fail:2]"));
        assert!(marked_attempt_fails(1, "Retry [fail:2]"));
        assert!(!marked_attempt_fails(2, "Retry [fail:2]"));
        assert!(!marked_attempt_fails(3, "Retry [fail:2]"));
        // An unmarked turn never fails, whatever its history.
        assert!(!marked_attempt_fails(0, "Plain turn"));
        assert!(!marked_attempt_fails(4, "Plain turn"));
        assert!(!marked_attempt_fails(0, "[fail:0]"));
    }

    #[test]
    fn only_a_rehearsal_practice_fails_a_marked_turn() {
        let practices = SimulationPractices::of(["coda-rehearsal-practice"]);
        assert!(practices.rehearsed_failure("coda-rehearsal-practice", "Drill [fail:1]", 0));
        assert!(!practices.rehearsed_failure("coda-rehearsal-practice", "Drill [fail:1]", 1));
        assert!(!practices.rehearsed_failure("coda-rehearsal-practice", "Plain turn", 0));
        // A real practice never fails, whatever its message says.
        assert!(!practices.rehearsed_failure("roger-synthetic-practice", "Drill [fail:1]", 0));
        // Outside the preview profile there are no rehearsal practices, so
        // nothing fails on purpose.
        let production = SimulationPractices::from_env(|name| {
            (name == "CODA_HOSTED_PROFILE").then(|| "production".to_owned())
        })
        .unwrap();
        assert!(!production.rehearsed_failure("coda-rehearsal-practice", "Drill [fail:1]", 0));
    }

    #[test]
    fn the_rehearsal_sign_in_needs_both_settings_and_the_preview_profile() {
        const AUDIENCE: &str = "/projects/123/locations/us-east1/services/physician";
        const PRINCIPAL: &str = "physician-rehearsal@coda-rust-staging.iam.gserviceaccount.com";
        let read = |pairs: &[(&str, &str)]| {
            RehearsalSignIn::from_env(|name| {
                pairs
                    .iter()
                    .find(|(key, _)| *key == name)
                    .map(|(_, value)| (*value).to_owned())
            })
        };
        assert_eq!(read(&[]), Ok(None));
        assert_eq!(
            read(&[("CODA_HOSTED_PROFILE", STAGING_PROFILE)]),
            Ok(None),
            "the profile alone serves no route"
        );
        assert_eq!(
            read(&[
                ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
                ("PHYSICIAN_IAP_AUDIENCE", AUDIENCE),
                (
                    "PHYSICIAN_REHEARSAL_PRINCIPAL",
                    " Physician-Rehearsal@coda-rust-staging.iam.gserviceaccount.com "
                ),
            ]),
            Ok(Some(RehearsalSignIn {
                audience: AUDIENCE.into(),
                principal: PRINCIPAL.into(),
            }))
        );
        for profile in [None, Some("production"), Some("")] {
            let mut pairs = vec![
                ("PHYSICIAN_IAP_AUDIENCE", AUDIENCE),
                ("PHYSICIAN_REHEARSAL_PRINCIPAL", PRINCIPAL),
            ];
            if let Some(profile) = profile {
                pairs.push(("CODA_HOSTED_PROFILE", profile));
            }
            assert!(read(&pairs).is_err(), "{profile:?} must refuse");
        }
        for (audience, principal) in [
            (AUDIENCE, ""),
            ("", PRINCIPAL),
            ("https://physician.example", PRINCIPAL),
            (AUDIENCE, "engineer@codahealth.ai"),
            (AUDIENCE, "@coda-rust-staging.iam.gserviceaccount.com"),
        ] {
            assert!(
                read(&[
                    ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
                    ("PHYSICIAN_IAP_AUDIENCE", audience),
                    ("PHYSICIAN_REHEARSAL_PRINCIPAL", principal),
                ])
                .is_err(),
                "{audience:?} / {principal:?} must refuse"
            );
        }
    }

    #[test]
    fn local_runs_start_without_any_profile() {
        assert!(validate_startup(|_| None).is_ok());
    }

    #[test]
    fn staging_preview_is_the_only_hosted_profile() {
        let complete = [
            ("CODA_ENVIRONMENT", "stg"),
            ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_DATABASE_URL", "postgres://db/physician"),
        ];
        assert!(hosted(&complete).is_ok());
        for index in 0..complete.len() {
            let dropped: Vec<_> = complete
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .map(|(_, pair)| *pair)
                .collect();
            assert!(hosted(&dropped).is_err(), "missing {}", complete[index].0);
        }
    }

    #[test]
    fn hosted_emulator_exceptions_refuse_even_with_complete_profile() {
        for marker in ["K_SERVICE", "CODA_HOSTED"] {
            for exception in ["PUBSUB_EMULATOR_HOST", "PHYSICIAN_LOCAL_STACK"] {
                for value in ["", "false", "127.0.0.1:8085"] {
                    assert!(
                        validate_startup(|name| match name {
                            "CODA_ENVIRONMENT" => Some("stg".into()),
                            "CODA_HOSTED_PROFILE" => Some(STAGING_PROFILE.into()),
                            "PHYSICIAN_AUTH" => Some("required".into()),
                            "PHYSICIAN_DATABASE_URL" => Some("postgres://db/physician".into()),
                            key if key == marker => Some("".into()),
                            key if key == exception => Some(value.into()),
                            _ => None,
                        })
                        .is_err(),
                        "{marker}/{exception}={value} must refuse"
                    );
                }
            }
        }
    }

    #[test]
    fn production_and_other_profiles_refuse_even_when_complete() {
        for (environment, profile) in [
            ("prd", STAGING_PROFILE),
            ("stg", "production"),
            ("dev", STAGING_PROFILE),
            ("", STAGING_PROFILE),
        ] {
            assert!(
                hosted(&[
                    ("CODA_ENVIRONMENT", environment),
                    ("CODA_HOSTED_PROFILE", profile),
                    ("PHYSICIAN_AUTH", "required"),
                    ("PHYSICIAN_DATABASE_URL", "postgres://db/physician"),
                ])
                .is_err(),
                "{environment}/{profile} must refuse"
            );
        }
    }

    #[test]
    fn a_false_cloud_marker_is_still_hosted() {
        assert!(
            validate_startup(|name| (name == "K_SERVICE").then(|| "false".to_owned())).is_err()
        );
        assert!(validate_startup(|name| (name == "CODA_HOSTED").then(|| "0".to_owned())).is_err());
    }

    #[tokio::test]
    async fn readiness_fails_closed_without_live_stores() {
        assert_eq!(
            ready(None, None).await,
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        );
    }
}

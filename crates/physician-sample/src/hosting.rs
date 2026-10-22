//! Hosting boundary for the physician runtime.
//!
//! Local runs are unrestricted. Hosted runs (Cloud Run sets `K_SERVICE`, or
//! `CODA_HOSTED` is set) start only under the explicit staging-preview profile:
//! `CODA_ENVIRONMENT=stg`, `CODA_HOSTED_PROFILE=staging-preview`,
//! `PHYSICIAN_AUTH=required` and `PHYSICIAN_DATABASE_URL`. Production and every
//! other combination refuse to start. The profile serves the recorded synthetic
//! practice behind IAP; it is not a clinical-data qualification.

/// The only hosted profile this binary will start under.
pub const STAGING_PROFILE: &str = "staging-preview";

/// Refuses hosted execution unless the staging-preview profile is complete.
pub fn validate_startup(get: impl Fn(&str) -> Option<String>) -> Result<(), String> {
    if get("K_SERVICE").is_none() && get("CODA_HOSTED").is_none() {
        return Ok(());
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

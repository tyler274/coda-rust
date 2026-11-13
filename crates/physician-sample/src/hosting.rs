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

/// Whether `CODA_HOSTED_PROFILE` selects the `staging-preview` profile.
pub fn is_staging_preview(get: impl Fn(&str) -> Option<String>) -> bool {
    get("CODA_HOSTED_PROFILE").as_deref() == Some(STAGING_PROFILE)
}

/// Refuses hosted execution unless the staging-preview profile is complete.
pub fn validate_startup(get: impl Fn(&str) -> Option<String>) -> Result<(), String> {
    if !is_hosted(&get) {
        return Ok(());
    }
    if get("CODA_LOCAL_MODEL_EXECUTION").as_deref() == Some("subscription") {
        return Err("Developer ChatGPT plans are available only in local execution".into());
    }
    // Presence alone is enough to refuse: an empty value or "false" must not
    // turn hosted authentication into a local-development exception.
    if get("PHYSICIAN_LOCAL_STACK").is_some() {
        return Err("Physician hosted execution forbids local stack configuration".to_owned());
    }
    let environment = get("CODA_ENVIRONMENT").unwrap_or_default();
    let auth = get("PHYSICIAN_AUTH").unwrap_or_default();
    let database = get("PHYSICIAN_DATABASE_URL").unwrap_or_default();
    if environment == "stg"
        && is_staging_preview(&get)
        && auth == "required"
        && !database.trim().is_empty()
    {
        ClinicalLibraryConfig::from_env(&get).map_err(|error| error.to_string())?;
        StagingSyntheticControlConfig::from_env(&get).map_err(|error| error.to_string())?;
        StaffSetupAuth::from_env(&get)?;
        return Ok(());
    }
    Err(format!(
        "Physician hosted activation is not qualified: hosted runs require CODA_ENVIRONMENT=stg, CODA_HOSTED_PROFILE={STAGING_PROFILE}, PHYSICIAN_AUTH=required and PHYSICIAN_DATABASE_URL; production and every other profile refuse to start"
    ))
}

/// Environment variable holding the bearer secret for the Coda staff practice and
/// physician account setup routes (`/auth/staff/*`).
pub const STAFF_SETUP_TOKEN_ENV: &str = "CODA_STAFF_SETUP_TOKEN";

/// Minimum length of `CODA_STAFF_SETUP_TOKEN` so short tokens fail closed at startup.
const MIN_STAFF_SETUP_TOKEN_BYTES: usize = 16;

/// Constant-time bearer gate for the Coda staff setup routes (`/auth/staff/*`),
/// independent of staging rehearsal (`PHYSICIAN_STAGING_ACCOUNTS` and `POST /auth/rehearsal`).
///
/// When `CODA_STAFF_SETUP_TOKEN` is unset or blank, `from_env` returns `Ok(None)`
/// and no `/auth/staff/*` route is mounted (`404 Not Found`).
#[derive(Clone)]
pub struct StaffSetupAuth {
    digest: [u8; 32],
}

impl std::fmt::Debug for StaffSetupAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaffSetupAuth")
            .field("digest", &"<redacted>")
            .finish()
    }
}

impl PartialEq for StaffSetupAuth {
    fn eq(&self, other: &Self) -> bool {
        use subtle::ConstantTimeEq;
        bool::from(self.digest.ct_eq(&other.digest))
    }
}

impl Eq for StaffSetupAuth {}

impl StaffSetupAuth {
    /// Parses `CODA_STAFF_SETUP_TOKEN` when present, rejecting short or newline-bearing values.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Option<Self>, String> {
        let Some(raw) = get(STAFF_SETUP_TOKEN_ENV) else {
            return Ok(None);
        };
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        Self::from_secret(trimmed).map(Some)
    }

    /// Builds a verifier from an explicit bearer secret (at least 16 bytes, single-line).
    pub fn from_secret(secret: &str) -> Result<Self, String> {
        let trimmed = secret.trim();
        if trimmed.len() < MIN_STAFF_SETUP_TOKEN_BYTES {
            return Err(format!(
                "{STAFF_SETUP_TOKEN_ENV} must be at least {MIN_STAFF_SETUP_TOKEN_BYTES} bytes"
            ));
        }
        if trimmed.bytes().any(|b| matches!(b, b'\r' | b'\n' | 0)) {
            return Err(format!(
                "{STAFF_SETUP_TOKEN_ENV} must not contain control characters"
            ));
        }
        use sha2::Digest;
        let digest: [u8; 32] = sha2::Sha256::digest(trimmed.as_bytes()).into();
        Ok(Self { digest })
    }

    /// Verifies a presented bearer token in constant time by comparing fixed-width SHA-256 digests.
    pub fn verify(&self, presented: &str) -> bool {
        use sha2::Digest;
        use subtle::ConstantTimeEq;
        let candidate: [u8; 32] = sha2::Sha256::digest(presented.as_bytes()).into();
        bool::from(self.digest.ct_eq(&candidate))
    }
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
/// ends the turn failed on its fifth failure and its Try again then succeeds.
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
        if !is_staging_preview(&get) {
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
        let principal = ServiceAccountEmail::parse(&principal)
            .map_err(|error| format!("PHYSICIAN_REHEARSAL_PRINCIPAL: {error}"))?;
        Ok(Some(Self {
            audience,
            principal: principal.into_string(),
        }))
    }
}

/// Environment variable that opts into the staging synthetic event control routes.
pub const STAGING_SYNTHETIC_CONTROL_ENV: &str = "CODA_STAGING_SYNTHETIC_CONTROL";

/// Optional environment variable naming the mutable working bank directory for
/// staging synthetic events.
pub const STAGING_SYNTHETIC_WORKDIR_ENV: &str = "CODA_STAGING_SYNTHETIC_WORKDIR";

/// Validated settings for the staging-only synthetic event control surface.
///
/// When unset, no `/staging-synthetic/*` route is mounted. Hosted revisions
/// accept this setting exclusively under `CODA_HOSTED_PROFILE=staging-preview`
/// (`CODA_ENVIRONMENT=stg`); production and non-staging environments refuse to
/// start if it is set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagingSyntheticControlConfig {
    /// Canonical read-only patient bank restored on startup and on reset.
    pub base_bank_dir: std::path::PathBuf,
    /// Mutable working directory read by the synthetic chart and calendar adapters.
    pub working_bank_dir: std::path::PathBuf,
}

/// Why [`StagingSyntheticControlConfig::from_env`] refused environment settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StagingSyntheticControlConfigError {
    /// `CODA_STAGING_SYNTHETIC_WORKDIR` was set without enabling `CODA_STAGING_SYNTHETIC_CONTROL`.
    WorkdirWithoutEnable,
    /// `CODA_STAGING_SYNTHETIC_CONTROL` held a value other than `"enabled"`.
    InvalidToggle {
        /// Raw trimmed value supplied in the environment.
        value: String,
    },
    /// A hosted process attempted to enable synthetic control outside `staging-preview`.
    NotStagingPreview,
    /// `CODA_ENVIRONMENT` named a non-staging, non-local environment such as `prd` or `production`.
    DisallowedEnvironment {
        /// Environment name supplied in `CODA_ENVIRONMENT`.
        environment: String,
    },
    /// `PHYSICIAN_SYNTHETIC_CHARTS` was missing or empty when synthetic control was enabled.
    MissingBaseBank,
    /// `working_bank_dir` pointed at the same path as `base_bank_dir`, which would mutate base charts.
    WorkingDirMatchesBaseBank {
        /// Colliding filesystem path.
        path: std::path::PathBuf,
    },
}

impl std::fmt::Display for StagingSyntheticControlConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WorkdirWithoutEnable => write!(
                f,
                "{STAGING_SYNTHETIC_WORKDIR_ENV} requires {STAGING_SYNTHETIC_CONTROL_ENV}=enabled"
            ),
            Self::InvalidToggle { value } => write!(
                f,
                "{STAGING_SYNTHETIC_CONTROL_ENV} must be `enabled` or unset, got `{value}`"
            ),
            Self::NotStagingPreview => write!(
                f,
                "{STAGING_SYNTHETIC_CONTROL_ENV} is only honoured under CODA_HOSTED_PROFILE={STAGING_PROFILE}"
            ),
            Self::DisallowedEnvironment { environment } => write!(
                f,
                "{STAGING_SYNTHETIC_CONTROL_ENV} is forbidden when CODA_ENVIRONMENT=`{environment}`"
            ),
            Self::MissingBaseBank => write!(
                f,
                "{STAGING_SYNTHETIC_CONTROL_ENV}=enabled requires PHYSICIAN_SYNTHETIC_CHARTS"
            ),
            Self::WorkingDirMatchesBaseBank { path } => write!(
                f,
                "{STAGING_SYNTHETIC_WORKDIR_ENV} (`{}`) must differ from PHYSICIAN_SYNTHETIC_CHARTS so base charts stay immutable",
                path.display()
            ),
        }
    }
}

impl std::error::Error for StagingSyntheticControlConfigError {}

impl StagingSyntheticControlConfig {
    /// Parses staging synthetic control settings, returning `Ok(None)` when disabled.
    pub fn from_env(
        get: impl Fn(&str) -> Option<String>,
    ) -> Result<Option<Self>, StagingSyntheticControlConfigError> {
        let raw_toggle = get(STAGING_SYNTHETIC_CONTROL_ENV);
        let raw_workdir = get(STAGING_SYNTHETIC_WORKDIR_ENV);
        if raw_toggle.is_some() || raw_workdir.is_some() {
            if is_hosted(&get) && !is_staging_preview(&get) {
                return Err(StagingSyntheticControlConfigError::NotStagingPreview);
            }
            if let Some(environment) = get("CODA_ENVIRONMENT")
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
                && !matches!(environment.as_str(), "stg" | "local" | "dev")
            {
                return Err(StagingSyntheticControlConfigError::DisallowedEnvironment {
                    environment,
                });
            }
        }

        let toggle = raw_toggle
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let workdir = raw_workdir
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());

        let Some(toggle) = toggle else {
            if workdir.is_some() {
                return Err(StagingSyntheticControlConfigError::WorkdirWithoutEnable);
            }
            return Ok(None);
        };

        if toggle != "enabled" {
            return Err(StagingSyntheticControlConfigError::InvalidToggle { value: toggle });
        }

        let base_bank = get("PHYSICIAN_SYNTHETIC_CHARTS")
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .ok_or(StagingSyntheticControlConfigError::MissingBaseBank)?;
        let base_bank_dir = std::path::PathBuf::from(base_bank);
        let working_bank_dir = match workdir {
            Some(explicit) => std::path::PathBuf::from(explicit),
            None => std::env::temp_dir().join(format!(
                "coda-staging-synthetic-work-{}",
                std::process::id()
            )),
        };
        if working_bank_dir == base_bank_dir {
            return Err(
                StagingSyntheticControlConfigError::WorkingDirMatchesBaseBank {
                    path: working_bank_dir,
                },
            );
        }
        Ok(Some(Self {
            base_bank_dir,
            working_bank_dir,
        }))
    }
}

/// Validated, lower-cased Google service-account email (`NAME@PROJECT.iam.gserviceaccount.com`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceAccountEmail(String);

impl ServiceAccountEmail {
    /// Parses and normalizes one Google service-account email, rejecting empty
    /// values, human accounts, groups, and malformed domains.
    pub fn parse(raw: &str) -> Result<Self, ServiceAccountError> {
        let normalized = raw.trim().to_lowercase();
        if normalized.is_empty() {
            return Err(ServiceAccountError::Empty);
        }
        let valid = normalized.split_once('@').is_some_and(|(name, domain)| {
            !name.is_empty()
                && !name.chars().any(char::is_whitespace)
                && domain
                    .strip_suffix(".iam.gserviceaccount.com")
                    .is_some_and(|project| {
                        !project.is_empty()
                            && !project.contains('@')
                            && !project.chars().any(char::is_whitespace)
                    })
        });
        if valid {
            Ok(Self(normalized))
        } else {
            Err(ServiceAccountError::NotServiceAccount { value: normalized })
        }
    }

    /// Borrowed lower-cased email string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Owned lower-cased email string.
    pub fn into_string(self) -> String {
        self.0
    }
}

/// Why a string could not be parsed as a [`ServiceAccountEmail`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServiceAccountError {
    /// The input was empty or whitespace.
    Empty,
    /// The input did not have the form `NAME@PROJECT.iam.gserviceaccount.com`.
    NotServiceAccount {
        /// Normalized candidate value.
        value: String,
    },
}

impl std::fmt::Display for ServiceAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(
                "must be a service-account email (NAME@PROJECT.iam.gserviceaccount.com), got empty value",
            ),
            Self::NotServiceAccount { value } => write!(
                f,
                "`{value}` is not a service-account email (NAME@PROJECT.iam.gserviceaccount.com)"
            ),
        }
    }
}

impl std::error::Error for ServiceAccountError {}

/// Default GCE metadata identity endpoint used by Cloud Run to mint OIDC ID tokens.
pub const DEFAULT_METADATA_IDENTITY_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity";

/// Maximum candidate identifiers accepted in a single `POST /v1/read` request.
pub const MAX_CLINICAL_READ_ITEMS: usize = 32;

const CREDENTIAL_RESPONSE_LIMIT: usize = 64 * 1024;
const LIBRARY_RESPONSE_LIMIT: usize = 2 * 1024 * 1024;
const DEFAULT_LIBRARY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;
    let mut hex = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

/// Validated connection settings for the read-only clinical library service.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClinicalLibraryConfig {
    /// Normalized service origin without a trailing slash.
    pub origin: String,
    /// Outbound authentication mode.
    pub auth: ClinicalLibraryAuth,
}

/// How outbound requests authenticate to the clinical library service.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClinicalLibraryAuth {
    /// Mint a Google OIDC ID token from `metadata_identity_url` for `audience`.
    GoogleIdToken {
        /// Target Cloud Run audience verified by IAM.
        audience: String,
        /// Metadata server identity endpoint.
        metadata_identity_url: String,
    },
    /// Unauthenticated local loopback endpoint, admitted only outside hosted runs.
    LocalLoopback,
}

/// Why `CLINICAL_LIBRARY_URL` or `CLINICAL_LIBRARY_AUDIENCE` was refused at startup.
#[derive(Debug)]
pub enum ClinicalLibraryConfigError {
    /// `CLINICAL_LIBRARY_AUDIENCE` was set without `CLINICAL_LIBRARY_URL`.
    AudienceWithoutUrl,
    /// A URL could not be parsed.
    InvalidUrl {
        /// Environment variable name.
        variable: &'static str,
        /// Underlying URL parse error.
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// A URL carried credentials, a path, query, fragment, or disallowed scheme/host.
    InvalidOrigin {
        /// Environment variable name.
        variable: &'static str,
        /// Specific rule violated.
        reason: &'static str,
    },
}

impl std::fmt::Display for ClinicalLibraryConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AudienceWithoutUrl => {
                f.write_str("CLINICAL_LIBRARY_AUDIENCE requires CLINICAL_LIBRARY_URL")
            }
            Self::InvalidUrl { variable, source } => {
                write!(f, "{variable} is not a valid URL: {source}")
            }
            Self::InvalidOrigin { variable, reason } => write!(f, "{variable} {reason}"),
        }
    }
}

impl std::error::Error for ClinicalLibraryConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidUrl { source, .. } => Some(source.as_ref()),
            Self::AudienceWithoutUrl | Self::InvalidOrigin { .. } => None,
        }
    }
}

fn parse_clean_origin(
    variable: &'static str,
    raw: &str,
) -> Result<(String, reqwest::Url), ClinicalLibraryConfigError> {
    let trimmed = raw.trim().trim_end_matches('/').to_owned();
    let parsed =
        reqwest::Url::parse(&trimmed).map_err(|error| ClinicalLibraryConfigError::InvalidUrl {
            variable,
            source: Box::new(error),
        })?;
    if parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.path() != "/"
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        return Err(ClinicalLibraryConfigError::InvalidOrigin {
            variable,
            reason: "must be a bare origin with no credentials, path, query, or fragment",
        });
    }
    Ok((trimmed, parsed))
}

fn is_loopback_url_host(url: &reqwest::Url) -> bool {
    url.host_str()
        .map(|host| host.trim_start_matches('[').trim_end_matches(']'))
        .and_then(|host| host.parse::<std::net::IpAddr>().ok())
        .is_some_and(|ip| ip.is_loopback())
}

impl ClinicalLibraryConfig {
    /// Reads `CLINICAL_LIBRARY_URL` and optional `CLINICAL_LIBRARY_AUDIENCE`.
    ///
    /// Hosted runs require an `https://` origin and always authenticate with
    /// Google OIDC ID tokens. Unauthenticated `http://` origins are admitted
    /// only on local numeric loopback addresses when not hosted.
    pub fn from_env(
        get: impl Fn(&str) -> Option<String>,
    ) -> Result<Option<Self>, ClinicalLibraryConfigError> {
        let url = get("CLINICAL_LIBRARY_URL")
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let audience = get("CLINICAL_LIBRARY_AUDIENCE")
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let Some(raw_url) = url else {
            if audience.is_some() {
                return Err(ClinicalLibraryConfigError::AudienceWithoutUrl);
            }
            return Ok(None);
        };
        let hosted = is_hosted(&get);
        let (origin, parsed_url) = parse_clean_origin("CLINICAL_LIBRARY_URL", &raw_url)?;
        match parsed_url.scheme() {
            "https" => {
                let resolved_audience = match audience {
                    Some(raw_audience) => {
                        let (clean, parsed_aud) =
                            parse_clean_origin("CLINICAL_LIBRARY_AUDIENCE", &raw_audience)?;
                        if parsed_aud.scheme() != "https" {
                            return Err(ClinicalLibraryConfigError::InvalidOrigin {
                                variable: "CLINICAL_LIBRARY_AUDIENCE",
                                reason: "must use https",
                            });
                        }
                        clean
                    }
                    None => origin.clone(),
                };
                Ok(Some(Self {
                    origin,
                    auth: ClinicalLibraryAuth::GoogleIdToken {
                        audience: resolved_audience,
                        metadata_identity_url: DEFAULT_METADATA_IDENTITY_URL.to_owned(),
                    },
                }))
            }
            "http" if !hosted && is_loopback_url_host(&parsed_url) => {
                if audience.is_some() {
                    return Err(ClinicalLibraryConfigError::InvalidOrigin {
                        variable: "CLINICAL_LIBRARY_AUDIENCE",
                        reason: "cannot be set for unauthenticated local loopback CLINICAL_LIBRARY_URL",
                    });
                }
                Ok(Some(Self {
                    origin,
                    auth: ClinicalLibraryAuth::LocalLoopback,
                }))
            }
            "http" if hosted => Err(ClinicalLibraryConfigError::InvalidOrigin {
                variable: "CLINICAL_LIBRARY_URL",
                reason: "must use https in hosted execution",
            }),
            "http" => Err(ClinicalLibraryConfigError::InvalidOrigin {
                variable: "CLINICAL_LIBRARY_URL",
                reason: "http is permitted only on numeric loopback addresses in local execution",
            }),
            _ => Err(ClinicalLibraryConfigError::InvalidOrigin {
                variable: "CLINICAL_LIBRARY_URL",
                reason: "must use https (or local loopback http)",
            }),
        }
    }
}

/// Publication and lifecycle status of an admitted clinical library passage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassageStatus {
    /// Current admitted evidence with no correction, retraction, or withdrawal.
    Current,
    /// Admitted evidence carrying a published correction notice.
    Corrected,
    /// Flagged retracted by publisher or bibliographic status check.
    Retracted,
    /// Replaced by a newer source version.
    Superseded,
    /// Withdrawn under rights or source enforcement.
    Withdrawn,
    /// Source did not declare a lifecycle state.
    Unknown,
}

/// Why an exact read item returned `outcome = "unavailable"` from the clinical library.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadUnavailableCause {
    /// Requested candidate identifier was not a valid UUID.
    InvalidCandidateId,
    /// Candidate did not exist or was excluded by live rights or withdrawal views.
    NotFoundOrNotAuthorized,
    /// Candidate's visual evidence asset could not be loaded.
    VisualUnavailable,
    /// Request exceeded the service's visual read limit.
    VisualReadLimit,
}

impl std::fmt::Display for ReadUnavailableCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCandidateId => f.write_str("invalid_candidate_id"),
            Self::NotFoundOrNotAuthorized => f.write_str("not_found_or_not_authorized"),
            Self::VisualUnavailable => f.write_str("visual_unavailable"),
            Self::VisualReadLimit => f.write_str("visual_read_limit"),
        }
    }
}

/// One passage returned by the clinical library search or exact-read endpoints.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ClinicalPassage {
    /// Stable candidate identifier used for exact readback on `POST /v1/read`.
    pub passage_id: String,
    /// Source collection or publisher display name.
    pub source: String,
    /// Source document title.
    pub title: String,
    /// Publication, version, or acquisition date carried with the source unit.
    pub date: String,
    /// Currentness and retraction status from live SQL views.
    pub status: PassageStatus,
    /// Required license, citation, or source attribution text.
    pub attribution: String,
    /// Exact passage text.
    pub text: String,
    /// SHA-256 hex digest of `text` reported by the clinical library.
    pub content_sha256: String,
}

enum WireUnitTranslation {
    Displayable(ClinicalPassage),
    WithheldByRights { passage_id: String },
}

impl ClinicalPassage {
    fn from_wire_unit(
        candidate_id: String,
        unit: WireUnit,
    ) -> Result<WireUnitTranslation, &'static str> {
        let passage_id = candidate_id.trim().to_owned();
        if passage_id.is_empty() {
            return Err("passage is missing passage_id");
        }
        if unit.unit_id.trim() != passage_id {
            return Err("returned unit_id did not match candidate_id");
        }
        if unit.content.trim().is_empty() {
            return Err("passage is missing text");
        }
        let content_sha256 = unit.content_sha256.trim().to_owned();
        if content_sha256.is_empty() {
            return Err("passage is missing content_sha256");
        }
        if sha256_hex(unit.content.as_bytes()) != content_sha256 {
            return Err("passage content_sha256 did not match text digest");
        }
        if !unit.may_display_excerpt {
            return Ok(WireUnitTranslation::WithheldByRights { passage_id });
        }
        let non_blank = |value: Option<String>| {
            value
                .map(|text| text.trim().to_owned())
                .filter(|text| !text.is_empty())
        };
        let source = unit.source_display_name.trim().to_owned();
        let title = unit.title.trim().to_owned();
        let destination = unit.native_destination_uri.trim().to_owned();
        let date = non_blank(unit.publication_date)
            .or_else(|| non_blank(unit.version_date))
            .or_else(|| non_blank(Some(unit.acquired_at)))
            .unwrap_or_default();
        let attribution = match (
            non_blank(unit.attribution_text),
            non_blank(unit.license_identifier),
        ) {
            (Some(text), _) => text,
            (None, Some(license)) if !source.is_empty() && !destination.is_empty() => {
                format!("{source} ({license}) — {destination}")
            }
            (None, None) if !source.is_empty() && !destination.is_empty() => {
                format!("{source} — {destination}")
            }
            (None, _) => String::new(),
        };
        let passage = Self {
            passage_id,
            source,
            title,
            date,
            status: unit.lifecycle_state,
            attribution,
            text: unit.content,
            content_sha256,
        };
        passage.validate()?;
        Ok(WireUnitTranslation::Displayable(passage))
    }

    fn validate(&self) -> Result<(), &'static str> {
        if self.passage_id.trim().is_empty() {
            return Err("passage is missing passage_id");
        }
        if self.source.trim().is_empty() {
            return Err("passage is missing source");
        }
        if self.title.trim().is_empty() {
            return Err("passage is missing title");
        }
        if self.date.trim().is_empty() {
            return Err("passage is missing date");
        }
        if self.attribution.trim().is_empty() {
            return Err("passage is missing attribution");
        }
        if self.text.trim().is_empty() {
            return Err("passage is missing text");
        }
        if self.content_sha256.trim().is_empty() {
            return Err("passage is missing content_sha256");
        }
        if sha256_hex(self.text.as_bytes()) != self.content_sha256.trim() {
            return Err("passage content_sha256 did not match text digest");
        }
        Ok(())
    }
}

/// Outcome of an authoritative clinical library search.
///
/// Only a 2xx search response with zero available passages produces [`Self::NothingFound`];
/// transport failures, 5xx errors, and IAM refusals return [`ClinicalLibraryError`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClinicalSearchOutcome {
    /// One or more validated passages matched the query.
    Found(Vec<ClinicalPassage>),
    /// The clinical library answered authoritatively with zero matching passages.
    NothingFound,
}

/// Operational standing of the remote clinical library service, kept separate
/// from container `/readyz` so a parked GPU service never takes down chat.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LibraryStanding {
    /// `/ready` answered 2xx with `status = "ready"` and an active build identifier.
    Ready {
        /// Active retrieval build identifier (`sha256:...`) reported by `/ready`.
        build_id: String,
    },
    /// Cloud Run IAM or an authentication redirect refused the request.
    Refused {
        /// HTTP status code returned by the gate (401, 403, or 3xx).
        status: u16,
    },
    /// The service is parked, unreachable, timed out, or failed its readiness check.
    Unavailable {
        /// HTTP status code when the server answered.
        status: Option<u16>,
        /// Bounded operational description.
        reason: String,
    },
}

/// Errors produced when calling the remote clinical library service.
#[derive(Debug)]
pub enum ClinicalLibraryError {
    /// Caller supplied a blank query, zero limit, or malformed passage identifier.
    InvalidRequest(&'static str),
    /// Outbound OIDC token acquisition failed before reaching the service.
    Authentication {
        /// Stage where token minting failed.
        context: &'static str,
        /// Underlying transport or parse error, if any.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    /// Cloud Run IAM or an authentication redirect refused the request.
    Refused {
        /// HTTP status code (401, 403, or 3xx).
        status: u16,
    },
    /// Exact read reported the requested candidate as unavailable.
    PassageUnavailable {
        /// Requested passage identifier.
        passage_id: String,
        /// Cause reported by `POST /v1/read`.
        cause: ReadUnavailableCause,
    },
    /// Service was unreachable, timed out, or answered with a non-serving status (e.g. 503).
    Unavailable {
        /// HTTP status code when the server answered.
        status: Option<u16>,
        /// Underlying transport error when no HTTP status was received.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    /// Service answered 2xx with malformed JSON or an incomplete passage record.
    InvalidResponse {
        /// What failed validation.
        reason: &'static str,
        /// Underlying decode error, if any.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl std::fmt::Display for ClinicalLibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest(reason) => write!(f, "invalid clinical library request: {reason}"),
            Self::Authentication {
                context,
                source: Some(source),
            } => write!(
                f,
                "clinical library authentication failed ({context}): {source}"
            ),
            Self::Authentication {
                context,
                source: None,
            } => write!(f, "clinical library authentication failed ({context})"),
            Self::Refused { status } => {
                write!(f, "clinical library refused the call with HTTP {status}")
            }
            Self::PassageUnavailable { passage_id, cause } => {
                write!(
                    f,
                    "clinical library passage `{passage_id}` is unavailable ({cause})"
                )
            }
            Self::Unavailable {
                status: Some(status),
                ..
            } => write!(f, "clinical library unavailable (HTTP {status})"),
            Self::Unavailable {
                status: None,
                source: Some(source),
            } => write!(f, "clinical library unavailable: {source}"),
            Self::Unavailable {
                status: None,
                source: None,
            } => f.write_str("clinical library unavailable"),
            Self::InvalidResponse {
                reason,
                source: Some(source),
            } => write!(f, "invalid clinical library response ({reason}): {source}"),
            Self::InvalidResponse {
                reason,
                source: None,
            } => write!(f, "invalid clinical library response ({reason})"),
        }
    }
}

impl std::error::Error for ClinicalLibraryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Authentication {
                source: Some(source),
                ..
            }
            | Self::Unavailable {
                source: Some(source),
                ..
            }
            | Self::InvalidResponse {
                source: Some(source),
                ..
            } => Some(source.as_ref()),
            Self::InvalidRequest(_)
            | Self::Authentication { source: None, .. }
            | Self::Refused { .. }
            | Self::PassageUnavailable { .. }
            | Self::Unavailable { source: None, .. }
            | Self::InvalidResponse { source: None, .. } => None,
        }
    }
}

#[derive(serde::Deserialize)]
struct ReadyWireResponse {
    status: String,
    build: ReadyWireBuild,
}

#[derive(serde::Deserialize)]
struct ReadyWireBuild {
    build_id: String,
}

/// Exact seven-key `z.strictObject` payload accepted by `POST /v1/search`.
#[derive(serde::Serialize)]
struct SearchWireRequest<'a> {
    mode: &'static str,
    query: &'a str,
    scheme: Option<&'a str>,
    value: Option<&'a str>,
    candidate_id: Option<&'a str>,
    relationship: Option<&'a str>,
    cursor: Option<&'a str>,
}

impl<'a> SearchWireRequest<'a> {
    fn query(query: &'a str) -> Self {
        Self {
            mode: "query",
            query,
            scheme: None,
            value: None,
            candidate_id: None,
            relationship: None,
            cursor: None,
        }
    }
}

#[derive(serde::Deserialize)]
struct SearchWireResponse {
    candidates: Vec<SearchCandidate>,
}

#[derive(serde::Deserialize)]
struct SearchCandidate {
    candidate_id: String,
}

/// Exact single-key `z.strictObject` payload accepted by `POST /v1/read`.
#[derive(serde::Serialize)]
struct ReadWireRequest<'a> {
    candidate_ids: &'a [&'a str],
}

#[derive(serde::Deserialize)]
struct ReadWireResponse {
    results: Vec<ReadWireItem>,
}

#[derive(serde::Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
enum ReadWireItem {
    Available {
        candidate_id: String,
        unit: Box<WireUnit>,
    },
    Unavailable {
        candidate_id: String,
        cause: ReadUnavailableCause,
    },
}

#[derive(serde::Deserialize)]
struct WireUnit {
    unit_id: String,
    source_display_name: String,
    title: String,
    publication_date: Option<String>,
    version_date: Option<String>,
    acquired_at: String,
    lifecycle_state: PassageStatus,
    content: String,
    content_sha256: String,
    native_destination_uri: String,
    license_identifier: Option<String>,
    attribution_text: Option<String>,
    may_display_excerpt: bool,
}

/// Authenticated HTTP client for the read-only clinical library service.
#[derive(Clone, Debug)]
pub struct ClinicalLibraryClient {
    config: ClinicalLibraryConfig,
    client: reqwest::Client,
}

impl ClinicalLibraryClient {
    /// Builds a client with the default 30-second request and connect timeout.
    pub fn new(config: ClinicalLibraryConfig) -> Result<Self, ClinicalLibraryError> {
        Self::with_timeout(config, DEFAULT_LIBRARY_TIMEOUT)
    }

    /// Builds a client with an explicit request and connect timeout.
    pub fn with_timeout(
        config: ClinicalLibraryConfig,
        timeout: std::time::Duration,
    ) -> Result<Self, ClinicalLibraryError> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(timeout)
            .connect_timeout(timeout)
            .build()
            .map_err(|error| ClinicalLibraryError::Unavailable {
                status: None,
                source: Some(Box::new(error)),
            })?;
        Ok(Self { config, client })
    }

    /// Validated connection configuration for this client.
    pub fn config(&self) -> &ClinicalLibraryConfig {
        &self.config
    }

    async fn bearer_header(
        &self,
    ) -> Result<Option<reqwest::header::HeaderValue>, ClinicalLibraryError> {
        let ClinicalLibraryAuth::GoogleIdToken {
            audience,
            metadata_identity_url,
        } = &self.config.auth
        else {
            return Ok(None);
        };
        let response = self
            .client
            .get(metadata_identity_url)
            .query(&[("audience", audience.as_str()), ("format", "full")])
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|error| ClinicalLibraryError::Authentication {
                context: "metadata identity request failed",
                source: Some(Box::new(error)),
            })?;
        if !response.status().is_success() {
            return Err(ClinicalLibraryError::Authentication {
                context: "metadata identity endpoint returned non-success status",
                source: None,
            });
        }
        let bytes = read_bounded_bytes(response, CREDENTIAL_RESPONSE_LIMIT)
            .await
            .map_err(|error| ClinicalLibraryError::Authentication {
                context: "metadata identity body read failed",
                source: Some(Box::new(error)),
            })?;
        let token = std::str::from_utf8(&bytes)
            .map_err(|error| ClinicalLibraryError::Authentication {
                context: "metadata identity token was not valid UTF-8",
                source: Some(Box::new(error)),
            })?
            .trim();
        if token.split('.').count() != 3 || token.chars().any(char::is_whitespace) {
            return Err(ClinicalLibraryError::Authentication {
                context: "metadata identity endpoint did not return a three-part JWT",
                source: None,
            });
        }
        let mut header = reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|error| ClinicalLibraryError::Authentication {
                context: "metadata identity token could not form an Authorization header",
                source: Some(Box::new(error)),
            })?;
        header.set_sensitive(true);
        Ok(Some(header))
    }

    fn apply_auth(
        request: reqwest::RequestBuilder,
        bearer: Option<reqwest::header::HeaderValue>,
    ) -> reqwest::RequestBuilder {
        match bearer {
            Some(bearer) => request.header(reqwest::header::AUTHORIZATION, bearer),
            None => request,
        }
    }

    async fn post_json<Req: serde::Serialize, Res: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
        bearer: Option<reqwest::header::HeaderValue>,
        decode_context: &'static str,
    ) -> Result<Res, ClinicalLibraryError> {
        let request = Self::apply_auth(
            self.client
                .post(format!("{}{path}", self.config.origin))
                .json(body),
            bearer,
        );
        let response = request
            .send()
            .await
            .map_err(|error| ClinicalLibraryError::Unavailable {
                status: None,
                source: Some(Box::new(error)),
            })?;
        let status = response.status();
        if status.is_redirection()
            || status == reqwest::StatusCode::UNAUTHORIZED
            || status == reqwest::StatusCode::FORBIDDEN
        {
            return Err(ClinicalLibraryError::Refused {
                status: status.as_u16(),
            });
        }
        if !status.is_success() {
            return Err(ClinicalLibraryError::Unavailable {
                status: Some(status.as_u16()),
                source: None,
            });
        }
        let bytes = read_bounded_bytes(response, LIBRARY_RESPONSE_LIMIT).await?;
        serde_json::from_slice(&bytes).map_err(|error| ClinicalLibraryError::InvalidResponse {
            reason: decode_context,
            source: Some(Box::new(error)),
        })
    }

    /// Probes `GET /ready` on the clinical library service without affecting
    /// container `/readyz`.
    pub async fn probe(&self) -> LibraryStanding {
        let bearer = match self.bearer_header().await {
            Ok(bearer) => bearer,
            Err(error) => {
                return LibraryStanding::Unavailable {
                    status: None,
                    reason: error.to_string(),
                };
            }
        };
        let request = Self::apply_auth(
            self.client.get(format!("{}/ready", self.config.origin)),
            bearer,
        );
        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                return LibraryStanding::Unavailable {
                    status: None,
                    reason: error.to_string(),
                };
            }
        };
        let status = response.status();
        if status.is_redirection()
            || status == reqwest::StatusCode::UNAUTHORIZED
            || status == reqwest::StatusCode::FORBIDDEN
        {
            return LibraryStanding::Refused {
                status: status.as_u16(),
            };
        }
        if !status.is_success() {
            return LibraryStanding::Unavailable {
                status: Some(status.as_u16()),
                reason: format!("readiness probe returned HTTP {}", status.as_u16()),
            };
        }
        let bytes = match read_bounded_bytes(response, CREDENTIAL_RESPONSE_LIMIT).await {
            Ok(bytes) => bytes,
            Err(error) => {
                return LibraryStanding::Unavailable {
                    status: Some(status.as_u16()),
                    reason: error.to_string(),
                };
            }
        };
        let parsed: ReadyWireResponse = match serde_json::from_slice(&bytes) {
            Ok(parsed) => parsed,
            Err(error) => {
                return LibraryStanding::Unavailable {
                    status: Some(status.as_u16()),
                    reason: format!("invalid readiness payload: {error}"),
                };
            }
        };
        let build_id = parsed.build.build_id.trim().to_owned();
        if parsed.status != "ready" || build_id.is_empty() {
            return LibraryStanding::Unavailable {
                status: Some(status.as_u16()),
                reason: "readiness payload did not report an active build".to_owned(),
            };
        }
        LibraryStanding::Ready { build_id }
    }

    /// Searches the read-only clinical library via `POST /v1/search` and hydrates
    /// matching candidates via `POST /v1/read`, distinguishing empty search
    /// results ([`ClinicalSearchOutcome::NothingFound`]) from down or refused calls.
    pub async fn search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<ClinicalSearchOutcome, ClinicalLibraryError> {
        let query = query.trim();
        if query.is_empty() {
            return Err(ClinicalLibraryError::InvalidRequest(
                "query must not be blank",
            ));
        }
        if limit == 0 {
            return Err(ClinicalLibraryError::InvalidRequest(
                "limit must be at least 1",
            ));
        }
        let bearer = self.bearer_header().await?;
        let discovered: SearchWireResponse = self
            .post_json(
                "/v1/search",
                &SearchWireRequest::query(query),
                bearer.clone(),
                "search body was not valid JSON",
            )
            .await?;
        if discovered.candidates.is_empty() {
            return Ok(ClinicalSearchOutcome::NothingFound);
        }
        let capped = limit.min(MAX_CLINICAL_READ_ITEMS);
        let mut requested_ids = Vec::with_capacity(discovered.candidates.len().min(capped));
        for candidate in discovered.candidates.iter().take(capped) {
            let id = candidate.candidate_id.trim();
            if id.is_empty() || id.chars().any(char::is_whitespace) {
                return Err(ClinicalLibraryError::InvalidResponse {
                    reason: "search candidate was missing candidate_id",
                    source: None,
                });
            }
            requested_ids.push(id);
        }
        let hydrated: ReadWireResponse = self
            .post_json(
                "/v1/read",
                &ReadWireRequest {
                    candidate_ids: &requested_ids,
                },
                bearer,
                "read body was not valid JSON",
            )
            .await?;
        let mut passages = Vec::with_capacity(hydrated.results.len());
        for item in hydrated.results {
            match item {
                ReadWireItem::Available { candidate_id, unit } => {
                    match ClinicalPassage::from_wire_unit(candidate_id, *unit).map_err(
                        |reason| ClinicalLibraryError::InvalidResponse {
                            reason,
                            source: None,
                        },
                    )? {
                        WireUnitTranslation::Displayable(passage) => passages.push(passage),
                        WireUnitTranslation::WithheldByRights { .. } => {}
                    }
                }
                ReadWireItem::Unavailable {
                    cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
                    ..
                } => {}
                ReadWireItem::Unavailable {
                    cause: ReadUnavailableCause::InvalidCandidateId,
                    ..
                } => {
                    return Err(ClinicalLibraryError::InvalidResponse {
                        reason: "search returned a candidate_id rejected by exact read",
                        source: None,
                    });
                }
                ReadWireItem::Unavailable {
                    candidate_id,
                    cause,
                } => {
                    return Err(ClinicalLibraryError::PassageUnavailable {
                        passage_id: candidate_id,
                        cause,
                    });
                }
            }
        }
        if passages.is_empty() {
            return Ok(ClinicalSearchOutcome::NothingFound);
        }
        Ok(ClinicalSearchOutcome::Found(passages))
    }

    /// Reads one exact passage by candidate identifier via `POST /v1/read`,
    /// verifying the returned passage matches `passage_id` and carries complete
    /// source, date, status, and attribution metadata.
    pub async fn read_passage(
        &self,
        passage_id: &str,
    ) -> Result<ClinicalPassage, ClinicalLibraryError> {
        let passage_id = passage_id.trim();
        if passage_id.is_empty() || passage_id.chars().any(char::is_whitespace) {
            return Err(ClinicalLibraryError::InvalidRequest(
                "passage_id must be a non-empty identifier without whitespace",
            ));
        }
        let bearer = self.bearer_header().await?;
        let hydrated: ReadWireResponse = self
            .post_json(
                "/v1/read",
                &ReadWireRequest {
                    candidate_ids: &[passage_id],
                },
                bearer,
                "read body was not valid JSON",
            )
            .await?;
        let mut items = hydrated.results.into_iter();
        let Some(item) = items.next() else {
            return Err(ClinicalLibraryError::InvalidResponse {
                reason: "read response did not return an entry for requested passage_id",
                source: None,
            });
        };
        if items.next().is_some() {
            return Err(ClinicalLibraryError::InvalidResponse {
                reason: "read response returned multiple entries for a single passage_id",
                source: None,
            });
        };
        match item {
            ReadWireItem::Available { candidate_id, unit } => {
                if candidate_id != passage_id {
                    return Err(ClinicalLibraryError::InvalidResponse {
                        reason: "returned candidate_id did not match requested passage_id",
                        source: None,
                    });
                }
                match ClinicalPassage::from_wire_unit(candidate_id, *unit).map_err(|reason| {
                    ClinicalLibraryError::InvalidResponse {
                        reason,
                        source: None,
                    }
                })? {
                    WireUnitTranslation::Displayable(passage) => Ok(passage),
                    WireUnitTranslation::WithheldByRights { passage_id } => {
                        Err(ClinicalLibraryError::PassageUnavailable {
                            passage_id,
                            cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
                        })
                    }
                }
            }
            ReadWireItem::Unavailable {
                candidate_id,
                cause,
            } => {
                if candidate_id != passage_id {
                    return Err(ClinicalLibraryError::InvalidResponse {
                        reason: "returned candidate_id did not match requested passage_id",
                        source: None,
                    });
                }
                Err(ClinicalLibraryError::PassageUnavailable {
                    passage_id: candidate_id,
                    cause,
                })
            }
        }
    }
}

async fn read_bounded_bytes(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<Vec<u8>, ClinicalLibraryError> {
    let mut bytes = Vec::new();
    while let Some(chunk) =
        response
            .chunk()
            .await
            .map_err(|error| ClinicalLibraryError::InvalidResponse {
                reason: "response body stream failed",
                source: Some(Box::new(error)),
            })?
    {
        if bytes.len().saturating_add(chunk.len()) > limit {
            return Err(ClinicalLibraryError::InvalidResponse {
                reason: "response body exceeded byte limit",
                source: None,
            });
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
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

/// The auth and front-door stores a site serves with, both reading through
/// the process's one runtime pool. Startup and `/readyz` check the same pair,
/// so readiness never opens a connection of its own.
#[derive(Clone)]
pub struct Readiness {
    pub(crate) auth: physician_auth::PostgresAuthStore,
    pub(crate) front_door: front_door::PostgresFrontDoorStore,
}

impl Readiness {
    /// The stores over `runtime`'s pool.
    pub fn shared(runtime: &local_product::runtime::Store) -> Self {
        Self {
            auth: physician_auth::PostgresAuthStore::from_connection(runtime.connection().clone()),
            front_door: front_door::PostgresFrontDoorStore::from_connection(
                runtime.connection().clone(),
            ),
        }
    }

    /// Whether both schemas answer.
    pub async fn check(&self) -> Result<(), SchemaError> {
        self.auth
            .check_schema()
            .await
            .map_err(|source| SchemaError::new(SchemaOwner::Auth, source))?;
        self.front_door
            .check_schema()
            .await
            .map_err(|source| SchemaError::new(SchemaOwner::FrontDoor, source))
    }
}

/// Readiness for a hosted revision: the auth and front-door schemas both
/// answer. Anything else, including an unconfigured local run, is 503.
pub async fn ready(readiness: Option<&Readiness>) -> axum::http::StatusCode {
    let Some(readiness) = readiness else {
        return axum::http::StatusCode::SERVICE_UNAVAILABLE;
    };
    match readiness.check().await {
        Ok(()) => axum::http::StatusCode::OK,
        Err(_) => axum::http::StatusCode::SERVICE_UNAVAILABLE,
    }
}

/// Which store's schema check failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaOwner {
    /// The physician auth tables.
    Auth,
    /// The patient front-door tables.
    FrontDoor,
}

impl SchemaOwner {
    /// The schema's name in an error message.
    fn schema(self) -> &'static str {
        match self {
            Self::Auth => "physician auth schema",
            Self::FrontDoor => "front-door schema",
        }
    }
}

/// A schema check that did not answer, keeping the database error and what
/// kind of failure it was. Only a missing table names the migration job; a
/// pool timeout or lost connection says so instead.
#[derive(Debug, thiserror::Error)]
#[error("{} {}", .owner.schema(), schema_failure_reason(.failure))]
pub struct SchemaError {
    owner: SchemaOwner,
    failure: local_product::runtime::DbFailure,
    #[source]
    source: sea_orm::DbErr,
}

impl SchemaError {
    fn new(owner: SchemaOwner, source: sea_orm::DbErr) -> Self {
        Self {
            owner,
            failure: local_product::runtime::DbFailure::of(&source),
            source,
        }
    }

    /// What kind of database failure stopped the check.
    pub fn failure(&self) -> local_product::runtime::DbFailure {
        self.failure
    }

    /// The database error the check returned.
    pub fn cause(&self) -> &sea_orm::DbErr {
        &self.source
    }
}

/// Why a physician process, site or worker, could not start.
#[derive(Debug, thiserror::Error)]
pub enum StartupError {
    /// The process's one Postgres pool could not be opened.
    #[error("physician runtime store unavailable")]
    RuntimeStore(#[source] sea_orm::DbErr),
    /// The environment asks for something the process refuses to run.
    #[error("{0}")]
    Refused(String),
}

impl StartupError {
    /// The database error that stopped startup, when one did.
    pub fn database(&self) -> Option<&sea_orm::DbErr> {
        match self {
            Self::RuntimeStore(source) => Some(source),
            Self::Refused(_) => None,
        }
    }
}

impl From<String> for StartupError {
    fn from(reason: String) -> Self {
        Self::Refused(reason)
    }
}

impl From<&str> for StartupError {
    fn from(reason: &str) -> Self {
        Self::Refused(reason.to_owned())
    }
}

/// What a failed schema check says about the schema: only a missing table
/// names the migration job.
fn schema_failure_reason(failure: &local_product::runtime::DbFailure) -> &'static str {
    use local_product::runtime::DbFailure;
    match failure {
        DbFailure::SchemaMissing => "missing; run the migration job",
        DbFailure::PoolTimeout => "unchecked: no pooled connection came free",
        DbFailure::Connection => "unchecked: the database connection failed",
        DbFailure::Query => "check failed",
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
    fn hosted_local_stack_refuses_even_with_complete_profile() {
        for marker in ["K_SERVICE", "CODA_HOSTED"] {
            for value in ["", "false", "1"] {
                assert!(
                    validate_startup(|name| match name {
                        "CODA_ENVIRONMENT" => Some("stg".into()),
                        "CODA_HOSTED_PROFILE" => Some(STAGING_PROFILE.into()),
                        "PHYSICIAN_AUTH" => Some("required".into()),
                        "PHYSICIAN_DATABASE_URL" => Some("postgres://db/physician".into()),
                        key if key == marker => Some("".into()),
                        "PHYSICIAN_LOCAL_STACK" => Some(value.into()),
                        _ => None,
                    })
                    .is_err(),
                    "{marker}/PHYSICIAN_LOCAL_STACK={value} must refuse"
                );
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
            ready(None).await,
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[test]
    fn a_pool_timeout_does_not_suggest_the_migration_job() {
        let error = SchemaError::new(
            SchemaOwner::Auth,
            sea_orm::DbErr::ConnectionAcquire(sea_orm::ConnAcquireErr::Timeout),
        );
        assert_eq!(
            error.failure(),
            local_product::runtime::DbFailure::PoolTimeout
        );
        let message = error.to_string();
        assert_eq!(
            message,
            "physician auth schema unchecked: no pooled connection came free"
        );
        assert!(!message.contains("migration"), "{message}");
        let source = std::error::Error::source(&error).expect("the database error is the source");
        assert_eq!(source.to_string(), error.cause().to_string());
    }

    #[test]
    fn service_account_email_normalizes_and_rejects_non_service_principals() {
        let parsed = ServiceAccountEmail::parse(
            " Physician-Scheduler@coda-rust-staging.iam.gserviceaccount.com ",
        )
        .expect("valid service account");
        assert_eq!(
            parsed.as_str(),
            "physician-scheduler@coda-rust-staging.iam.gserviceaccount.com"
        );
        assert_eq!(
            ServiceAccountEmail::parse("   "),
            Err(ServiceAccountError::Empty)
        );
        for bad in [
            "doctor@codahealth.test",
            "group:ops@codahealth.test",
            "@coda-rust-staging.iam.gserviceaccount.com",
            "scheduler@.iam.gserviceaccount.com",
            "scheduler@a@b.iam.gserviceaccount.com",
            "scheduler name@coda-rust-staging.iam.gserviceaccount.com",
        ] {
            assert!(
                matches!(
                    ServiceAccountEmail::parse(bad),
                    Err(ServiceAccountError::NotServiceAccount { .. })
                ),
                "{bad} must be rejected"
            );
        }
    }

    #[test]
    fn clinical_library_config_enforces_https_in_hosted_runs_and_loopback_locally() {
        const PROD_URL: &str = "https://coda-clinical-evidence-720954567292.us-east4.run.app";
        let from_pairs = |pairs: &[(&str, &str)]| {
            ClinicalLibraryConfig::from_env(|name| {
                pairs
                    .iter()
                    .find(|(key, _)| *key == name)
                    .map(|(_, value)| (*value).to_owned())
            })
        };
        assert_eq!(from_pairs(&[]).expect("unset"), None);
        let prod = from_pairs(&[("CLINICAL_LIBRARY_URL", &format!(" {PROD_URL}/ "))])
            .expect("valid https url")
            .expect("present");
        assert_eq!(prod.origin, PROD_URL);
        assert_eq!(
            prod.auth,
            ClinicalLibraryAuth::GoogleIdToken {
                audience: PROD_URL.to_owned(),
                metadata_identity_url: DEFAULT_METADATA_IDENTITY_URL.to_owned(),
            }
        );
        assert!(
            hosted(&[
                ("CODA_ENVIRONMENT", "stg"),
                ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
                ("PHYSICIAN_AUTH", "required"),
                ("PHYSICIAN_DATABASE_URL", "postgres://db/physician"),
                ("CLINICAL_LIBRARY_URL", PROD_URL),
            ])
            .is_ok()
        );
        for local_url in ["http://127.0.0.1:18080", "http://[::1]:18080"] {
            let local = from_pairs(&[("CLINICAL_LIBRARY_URL", local_url)])
                .expect("loopback http allowed locally")
                .expect("present");
            assert_eq!(local.auth, ClinicalLibraryAuth::LocalLoopback);
            assert!(
                hosted(&[
                    ("CODA_ENVIRONMENT", "stg"),
                    ("CODA_HOSTED_PROFILE", STAGING_PROFILE),
                    ("PHYSICIAN_AUTH", "required"),
                    ("PHYSICIAN_DATABASE_URL", "postgres://db/physician"),
                    ("CLINICAL_LIBRARY_URL", local_url),
                ])
                .is_err(),
                "hosted startup must refuse {local_url}"
            );
        }
        for invalid in [
            &[("CLINICAL_LIBRARY_AUDIENCE", PROD_URL)][..],
            &[("CLINICAL_LIBRARY_URL", "http://clinical.example.com")],
            &[(
                "CLINICAL_LIBRARY_URL",
                "https://coda-clinical-evidence-720954567292.us-east4.run.app/v1/search",
            )],
            &[(
                "CLINICAL_LIBRARY_URL",
                "https://coda-clinical-evidence-720954567292.us-east4.run.app?x=1",
            )],
            &[(
                "CLINICAL_LIBRARY_URL",
                "ftp://coda-clinical-evidence-720954567292.us-east4.run.app",
            )],
            &[
                ("CLINICAL_LIBRARY_URL", PROD_URL),
                ("CLINICAL_LIBRARY_AUDIENCE", "http://insecure.example.com"),
            ],
        ] {
            assert!(from_pairs(invalid).is_err(), "{invalid:?} must fail");
        }
    }

    #[tokio::test]
    async fn clinical_library_search_and_exact_read_authenticate_and_preserve_passage_metadata() {
        use axum::extract::Query;
        use axum::http::{HeaderMap, StatusCode};
        use axum::routing::{get, post};
        use axum::{Json, Router};
        use std::collections::BTreeMap;

        const AUDIENCE: &str = "https://coda-clinical-evidence-720954567292.us-east4.run.app";
        const SYNTHETIC_ID_TOKEN: &str = "header.payload.signature";
        const BUILD_ID: &str =
            "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        const FIRST_ID: &str = "11111111-1111-4111-8111-111111111111";
        const SECOND_ID: &str = "22222222-2222-4222-8222-222222222222";
        const WITHDRAWN_ID: &str = "33333333-3333-4333-8333-333333333333";

        let meta_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let meta_addr = meta_listener.local_addr().unwrap();
        let meta_server = tokio::spawn(async move {
            axum::serve(
                meta_listener,
                Router::new().route(
                    "/computeMetadata/v1/instance/service-accounts/default/identity",
                    get(
                        |headers: HeaderMap, Query(params): Query<BTreeMap<String, String>>| async move {
                            if headers.get("Metadata-Flavor").and_then(|v| v.to_str().ok())
                                != Some("Google")
                            {
                                return (StatusCode::FORBIDDEN, "missing flavor");
                            }
                            if params.get("audience").map(String::as_str) != Some(AUDIENCE)
                                || params.get("format").map(String::as_str) != Some("full")
                            {
                                return (StatusCode::BAD_REQUEST, "bad query");
                            }
                            (StatusCode::OK, SYNTHETIC_ID_TOKEN)
                        },
                    ),
                ),
            )
            .await
            .unwrap();
        });

        let first_text = "Confirm out-of-office blood pressure before intensifying dual therapy.";
        let first_sha256 = sha256_hex(first_text.as_bytes());
        let second_text = "Ambulatory blood pressure monitoring is the reference standard.";
        let second_sha256 = sha256_hex(second_text.as_bytes());

        let expected_first = ClinicalPassage {
            passage_id: FIRST_ID.to_owned(),
            source: "PubMed Central".to_owned(),
            title: "Out-of-Office Blood Pressure Confirmation".to_owned(),
            date: "2025-09-14".to_owned(),
            status: PassageStatus::Current,
            attribution: "CC BY 4.0 — Smith et al., J Clin Hypertens 2025; PMCID: PMC9001001"
                .to_owned(),
            text: first_text.to_owned(),
            content_sha256: first_sha256.clone(),
        };
        let expected_second = ClinicalPassage {
            passage_id: SECOND_ID.to_owned(),
            source: "USPSTF".to_owned(),
            title: "Screening for Hypertension in Adults".to_owned(),
            date: "2024-11-01".to_owned(),
            status: PassageStatus::Corrected,
            attribution:
                "USPSTF (US-Public-Domain) — https://www.uspreventiveservicestaskforce.org/uspstf/recommendation/hypertension-in-adults-screening"
                    .to_owned(),
            text: second_text.to_owned(),
            content_sha256: second_sha256.clone(),
        };
        let expected_bearer = format!("Bearer {SYNTHETIC_ID_TOKEN}");

        let lib_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let lib_addr = lib_listener.local_addr().unwrap();
        let lib_server = tokio::spawn(async move {
            let check_bearer = move |headers: &HeaderMap| {
                headers
                    .get(axum::http::header::AUTHORIZATION)
                    .and_then(|v| v.to_str().ok())
                    == Some(expected_bearer.as_str())
            };
            let check_ready = check_bearer.clone();
            let check_search = check_bearer.clone();
            let check_read = check_bearer;
            axum::serve(
                lib_listener,
                Router::new()
                    .route(
                        "/ready",
                        get(move |headers: HeaderMap| {
                            let ok = check_ready(&headers);
                            async move {
                                if !ok {
                                    return (
                                        StatusCode::FORBIDDEN,
                                        Json(serde_json::json!({"error": "forbidden"})),
                                    );
                                }
                                (
                                    StatusCode::OK,
                                    Json(serde_json::json!({
                                        "status": "ready",
                                        "build": {
                                            "build_id": BUILD_ID,
                                            "projection_sha256": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
                                            "input_set_sha256": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
                                            "unit_count": 2
                                        },
                                        "worker_count": 1
                                    })),
                                )
                            }
                        }),
                    )
                    .route(
                        "/v1/search",
                        post(
                            move |headers: HeaderMap, Json(body): Json<serde_json::Value>| {
                                let ok = check_search(&headers);
                                async move {
                                    if !ok {
                                        return (
                                            StatusCode::FORBIDDEN,
                                            Json(serde_json::json!({"error": "forbidden"})),
                                        );
                                    }
                                    // Enforce ClinicalEvidenceServer's exact 7-key z.strictObject contract.
                                    let Some(object) = body.as_object() else {
                                        return (
                                            StatusCode::BAD_REQUEST,
                                            Json(serde_json::json!({"error": "not_object"})),
                                        );
                                    };
                                    let keys: Vec<&str> = object.keys().map(String::as_str).collect();
                                    let expected_keys = [
                                        "candidate_id",
                                        "cursor",
                                        "mode",
                                        "query",
                                        "relationship",
                                        "scheme",
                                        "value",
                                    ];
                                    if keys != expected_keys
                                        || object.get("mode").and_then(|v| v.as_str())
                                            != Some("query")
                                        || !object["scheme"].is_null()
                                        || !object["value"].is_null()
                                        || !object["candidate_id"].is_null()
                                        || !object["relationship"].is_null()
                                        || !object["cursor"].is_null()
                                    {
                                        return (
                                            StatusCode::BAD_REQUEST,
                                            Json(serde_json::json!({
                                                "error": {
                                                    "code": "search_unavailable",
                                                    "message": "Clinical evidence request is invalid",
                                                    "retryable": false
                                                }
                                            })),
                                        );
                                    }
                                    let query =
                                        object.get("query").and_then(|v| v.as_str()).unwrap_or("");
                                    if query == "unmatched rare term" {
                                        return (
                                            StatusCode::OK,
                                            Json(serde_json::json!({
                                                "candidates": [],
                                                "page": {
                                                    "examined_candidate_count": 0,
                                                    "returned_candidate_count": 0,
                                                    "unavailable_candidate_count": 0,
                                                    "truncated": false
                                                }
                                            })),
                                        );
                                    }
                                    (
                                        StatusCode::OK,
                                        Json(serde_json::json!({
                                            "candidates": [
                                                { "candidate_id": FIRST_ID },
                                                { "candidate_id": WITHDRAWN_ID },
                                                { "candidate_id": SECOND_ID }
                                            ],
                                            "page": {
                                                "examined_candidate_count": 3,
                                                "returned_candidate_count": 3,
                                                "unavailable_candidate_count": 0,
                                                "truncated": false
                                            }
                                        })),
                                    )
                                }
                            },
                        ),
                    )
                    .route(
                        "/v1/read",
                        post(
                            move |headers: HeaderMap, Json(body): Json<serde_json::Value>| {
                                let ok = check_read(&headers);
                                let first_sha256 = first_sha256.clone();
                                let second_sha256 = second_sha256.clone();
                                async move {
                                    if !ok {
                                        return (
                                            StatusCode::FORBIDDEN,
                                            Json(serde_json::json!({"error": "forbidden"})),
                                        );
                                    }
                                    // Enforce ClinicalEvidenceServer's exact single-key z.strictObject contract.
                                    let Some(object) = body.as_object() else {
                                        return (
                                            StatusCode::BAD_REQUEST,
                                            Json(serde_json::json!({"error": "not_object"})),
                                        );
                                    };
                                    if object.len() != 1 || !object.contains_key("candidate_ids") {
                                        return (
                                            StatusCode::BAD_REQUEST,
                                            Json(serde_json::json!({
                                                "error": {
                                                    "code": "read_unavailable",
                                                    "message": "Clinical evidence request is invalid",
                                                    "retryable": false
                                                }
                                            })),
                                        );
                                    }
                                    let Some(ids) =
                                        object.get("candidate_ids").and_then(|v| v.as_array())
                                    else {
                                        return (
                                            StatusCode::BAD_REQUEST,
                                            Json(serde_json::json!({"error": "bad_ids"})),
                                        );
                                    };
                                    if ids.is_empty() || ids.len() > MAX_CLINICAL_READ_ITEMS {
                                        return (
                                            StatusCode::BAD_REQUEST,
                                            Json(serde_json::json!({"error": "out_of_bounds"})),
                                        );
                                    }
                                    let mut results = Vec::with_capacity(ids.len());
                                    for raw_id in ids {
                                        let id = raw_id.as_str().unwrap_or("");
                                        let item = match id {
                                            FIRST_ID => serde_json::json!({
                                                "candidate_id": FIRST_ID,
                                                "outcome": "available",
                                                "unit": {
                                                    "unit_id": FIRST_ID,
                                                    "version_id": "44444444-4444-4444-8444-444444444444",
                                                    "source_code": "pmc",
                                                    "source_display_name": "PubMed Central",
                                                    "native_document_id": "PMC9001001",
                                                    "title": "Out-of-Office Blood Pressure Confirmation",
                                                    "document_type": "research-article",
                                                    "publication_date": "2025-09-14",
                                                    "acquired_at": "2026-10-01T03:00:00Z",
                                                    "lifecycle_state": "current",
                                                    "native_unit_id": "p3",
                                                    "native_locator": "Methods > Paragraph 3",
                                                    "unit_kind": "paragraph",
                                                    "ordinal": 3,
                                                    "content": first_text,
                                                    "content_sha256": first_sha256,
                                                    "native_destination_uri": "https://pmc.ncbi.nlm.nih.gov/articles/PMC9001001/",
                                                    "version_destination_uri": "https://pmc.ncbi.nlm.nih.gov/articles/PMC9001001/",
                                                    "rights_decision_id": "55555555-5555-4555-8555-555555555555",
                                                    "license_identifier": "CC-BY-4.0",
                                                    "attribution_text": "CC BY 4.0 — Smith et al., J Clin Hypertens 2025; PMCID: PMC9001001",
                                                    "may_display_excerpt": true,
                                                    "identifiers": [],
                                                    "relationships": []
                                                }
                                            }),
                                            SECOND_ID => serde_json::json!({
                                                "candidate_id": SECOND_ID,
                                                "outcome": "available",
                                                "unit": {
                                                    "unit_id": SECOND_ID,
                                                    "version_id": "66666666-6666-4666-8666-666666666666",
                                                    "source_code": "uspstf",
                                                    "source_display_name": "USPSTF",
                                                    "native_document_id": "hypertension-screening",
                                                    "title": "Screening for Hypertension in Adults",
                                                    "document_type": "recommendation",
                                                    "version_date": "2024-11-01",
                                                    "acquired_at": "2026-10-01T03:00:00Z",
                                                    "lifecycle_state": "corrected",
                                                    "native_unit_id": "p1",
                                                    "native_locator": "Summary",
                                                    "unit_kind": "paragraph",
                                                    "ordinal": 1,
                                                    "content": second_text,
                                                    "content_sha256": second_sha256,
                                                    "native_destination_uri": "https://www.uspreventiveservicestaskforce.org/uspstf/recommendation/hypertension-in-adults-screening",
                                                    "version_destination_uri": "https://www.uspreventiveservicestaskforce.org/uspstf/recommendation/hypertension-in-adults-screening",
                                                    "rights_decision_id": "77777777-7777-4777-8777-777777777777",
                                                    "license_identifier": "US-Public-Domain",
                                                    "may_display_excerpt": true,
                                                    "identifiers": [],
                                                    "relationships": []
                                                }
                                            }),
                                            other => serde_json::json!({
                                                "candidate_id": other,
                                                "outcome": "unavailable",
                                                "cause": "not_found_or_not_authorized"
                                            }),
                                        };
                                        results.push(item);
                                    }
                                    (
                                        StatusCode::OK,
                                        Json(serde_json::json!({ "results": results })),
                                    )
                                }
                            },
                        ),
                    ),
            )
            .await
            .unwrap();
        });

        let client = ClinicalLibraryClient::new(ClinicalLibraryConfig {
            origin: format!("http://{lib_addr}"),
            auth: ClinicalLibraryAuth::GoogleIdToken {
                audience: AUDIENCE.to_owned(),
                metadata_identity_url: format!(
                    "http://{meta_addr}/computeMetadata/v1/instance/service-accounts/default/identity"
                ),
            },
        })
        .expect("client");

        assert_eq!(
            client.probe().await,
            LibraryStanding::Ready {
                build_id: BUILD_ID.to_owned(),
            }
        );

        let outcome = client
            .search("stage 2 hypertension", 5)
            .await
            .expect("authenticated search");
        let ClinicalSearchOutcome::Found(found) = outcome else {
            panic!("expected passages from clinical library search");
        };
        assert_eq!(found.len(), 2);
        for passage in &found {
            assert!(!passage.source.is_empty());
            assert!(!passage.title.is_empty());
            assert!(!passage.date.is_empty());
            assert!(!passage.attribution.is_empty());
            assert!(!passage.text.is_empty());
            assert!(!passage.content_sha256.is_empty());
        }
        assert_eq!(found[0], expected_first);
        assert_eq!(found[1], expected_second);

        let exact_first = client
            .read_passage(&found[0].passage_id)
            .await
            .expect("exact passage readback");
        assert_eq!(exact_first, found[0]);

        let exact_second = client
            .read_passage(&found[1].passage_id)
            .await
            .expect("exact passage readback for public-domain source");
        assert_eq!(exact_second, found[1]);

        assert!(matches!(
            client.read_passage(WITHDRAWN_ID).await,
            Err(ClinicalLibraryError::PassageUnavailable {
                cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
                ..
            })
        ));
        assert_eq!(
            client.search("unmatched rare term", 5).await.unwrap(),
            ClinicalSearchOutcome::NothingFound
        );

        meta_server.abort();
        lib_server.abort();
    }

    #[tokio::test]
    async fn clinical_library_down_or_refused_never_masquerades_as_nothing_found() {
        use axum::http::{HeaderValue, StatusCode};
        use axum::response::IntoResponse;
        use axum::routing::{get, post};
        use axum::{Json, Router};
        use std::error::Error as _;
        use std::time::Duration;

        const CANDIDATE_ID: &str = "11111111-1111-4111-8111-111111111111";

        // 1. Closed local port: connection refused / down keeps its transport cause.
        let closed_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let closed_addr = closed_listener.local_addr().unwrap();
        drop(closed_listener);
        let down_client = ClinicalLibraryClient::with_timeout(
            ClinicalLibraryConfig {
                origin: format!("http://{closed_addr}"),
                auth: ClinicalLibraryAuth::LocalLoopback,
            },
            Duration::from_secs(2),
        )
        .unwrap();
        let down_err = down_client
            .search("anticoagulation", 3)
            .await
            .expect_err("unreachable service must not look like nothing found");
        assert!(matches!(
            down_err,
            ClinicalLibraryError::Unavailable { status: None, .. }
        ));
        assert!(
            down_err.source().is_some(),
            "transport error must keep its underlying cause"
        );
        assert!(matches!(
            down_client.probe().await,
            LibraryStanding::Unavailable { status: None, .. }
        ));

        // 2. HTTP 503 (parked GPU / stopped SQL on search or mid-hydration read),
        //    401/403 (IAM refusal), 302 redirect, and malformed data.
        let mode = std::sync::Arc::new(std::sync::atomic::AtomicU16::new(503));
        let mode_ready = mode.clone();
        let mode_search = mode.clone();
        let mode_read = mode.clone();
        let fault_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let fault_addr = fault_listener.local_addr().unwrap();
        let fault_server = tokio::spawn(async move {
            let reply = |code: u16| match code {
                302 => (
                    StatusCode::FOUND,
                    [(
                        axum::http::header::LOCATION,
                        HeaderValue::from_static("https://accounts.google.com/"),
                    )],
                    Json(serde_json::json!({})),
                )
                    .into_response(),
                other => (
                    StatusCode::from_u16(other).unwrap(),
                    Json(serde_json::json!({"error": "forced"})),
                )
                    .into_response(),
            };
            axum::serve(
                fault_listener,
                Router::new()
                    .route(
                        "/ready",
                        get(move || {
                            let code = mode_ready.load(std::sync::atomic::Ordering::SeqCst);
                            async move { reply(code) }
                        }),
                    )
                    .route(
                        "/v1/search",
                        post(move || {
                            let code = mode_search.load(std::sync::atomic::Ordering::SeqCst);
                            async move {
                                if matches!(code, 997 | 999) {
                                    return (
                                        StatusCode::OK,
                                        Json(serde_json::json!({
                                            "candidates": [{ "candidate_id": CANDIDATE_ID }]
                                        })),
                                    )
                                        .into_response();
                                }
                                reply(code)
                            }
                        }),
                    )
                    .route(
                        "/v1/read",
                        post(move || {
                            let code = mode_read.load(std::sync::atomic::Ordering::SeqCst);
                            async move {
                                match code {
                                    // Mid-hydration read outage after /v1/search returned candidates.
                                    997 => (
                                        StatusCode::SERVICE_UNAVAILABLE,
                                        Json(serde_json::json!({
                                            "error": {
                                                "code": "read_unavailable",
                                                "message": "Clinical evidence service is unavailable",
                                                "retryable": true
                                            }
                                        })),
                                    )
                                        .into_response(),
                                    // Returned candidate_id mismatch on exact read.
                                    998 => (
                                        StatusCode::OK,
                                        Json(serde_json::json!({
                                            "results": [{
                                                "candidate_id": "99999999-9999-4999-8999-999999999999",
                                                "outcome": "available",
                                                "unit": {
                                                    "unit_id": "99999999-9999-4999-8999-999999999999",
                                                    "source_display_name": "PubMed Central",
                                                    "title": "Title",
                                                    "publication_date": "2025-01-01",
                                                    "acquired_at": "2025-01-01T00:00:00Z",
                                                    "lifecycle_state": "current",
                                                    "content": "Body",
                                                    "content_sha256": sha256_hex(b"Body"),
                                                    "native_destination_uri": "https://pmc.ncbi.nlm.nih.gov/articles/PMC1/",
                                                    "attribution_text": "CC BY 4.0",
                                                    "may_display_excerpt": true
                                                }
                                            }]
                                        })),
                                    )
                                        .into_response(),
                                    // Empty content + missing attribution/destination must fail validation.
                                    999 => (
                                        StatusCode::OK,
                                        Json(serde_json::json!({
                                            "results": [{
                                                "candidate_id": CANDIDATE_ID,
                                                "outcome": "available",
                                                "unit": {
                                                    "unit_id": CANDIDATE_ID,
                                                    "source_display_name": "PubMed Central",
                                                    "title": "Title",
                                                    "publication_date": "2025-01-01",
                                                    "acquired_at": "2025-01-01T00:00:00Z",
                                                    "lifecycle_state": "current",
                                                    "content": "   ",
                                                    "content_sha256": sha256_hex(b"   "),
                                                    "native_destination_uri": "",
                                                    "may_display_excerpt": true
                                                }
                                            }]
                                        })),
                                    )
                                        .into_response(),
                                    other => reply(other),
                                }
                            }
                        }),
                    ),
            )
            .await
            .unwrap();
        });

        let fault_client = ClinicalLibraryClient::new(ClinicalLibraryConfig {
            origin: format!("http://{fault_addr}"),
            auth: ClinicalLibraryAuth::LocalLoopback,
        })
        .unwrap();

        mode.store(503, std::sync::atomic::Ordering::SeqCst);
        assert!(matches!(
            fault_client.search("sepsis", 3).await,
            Err(ClinicalLibraryError::Unavailable {
                status: Some(503),
                ..
            })
        ));
        assert!(matches!(
            fault_client.probe().await,
            LibraryStanding::Unavailable {
                status: Some(503),
                ..
            }
        ));

        // Search succeeds, then /v1/read fails 503: must surface Unavailable, never NothingFound.
        mode.store(997, std::sync::atomic::Ordering::SeqCst);
        assert!(matches!(
            fault_client.search("sepsis", 3).await,
            Err(ClinicalLibraryError::Unavailable {
                status: Some(503),
                ..
            })
        ));

        for refused in [401_u16, 403, 302] {
            mode.store(refused, std::sync::atomic::Ordering::SeqCst);
            assert!(
                matches!(
                    fault_client.search("sepsis", 3).await,
                    Err(ClinicalLibraryError::Refused { status }) if status == refused
                ),
                "HTTP {refused} must report Refused, never NothingFound"
            );
            assert!(
                matches!(
                    fault_client.read_passage(CANDIDATE_ID).await,
                    Err(ClinicalLibraryError::Refused { status }) if status == refused
                ),
                "HTTP {refused} on exact read must report Refused"
            );
            assert_eq!(
                fault_client.probe().await,
                LibraryStanding::Refused { status: refused }
            );
        }

        mode.store(999, std::sync::atomic::Ordering::SeqCst);
        assert!(matches!(
            fault_client.search("sepsis", 3).await,
            Err(ClinicalLibraryError::InvalidResponse { .. })
        ));

        mode.store(998, std::sync::atomic::Ordering::SeqCst);
        assert!(matches!(
            fault_client.read_passage(CANDIDATE_ID).await,
            Err(ClinicalLibraryError::InvalidResponse { .. })
        ));

        assert!(matches!(
            fault_client.search("   ", 3).await,
            Err(ClinicalLibraryError::InvalidRequest(_))
        ));
        assert!(matches!(
            fault_client.search("sepsis", 0).await,
            Err(ClinicalLibraryError::InvalidRequest(_))
        ));
        assert!(matches!(
            fault_client.read_passage("has space").await,
            Err(ClinicalLibraryError::InvalidRequest(_))
        ));

        fault_server.abort();
    }

    #[tokio::test]
    async fn clinical_library_enforces_excerpt_rights_unit_id_and_sha256_integrity() {
        use axum::http::StatusCode;
        use axum::routing::post;
        use axum::{Json, Router};

        const RESTRICTED_ID: &str = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
        const DISPLAYABLE_ID: &str = "eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee";
        const MISMATCH_UNIT_ID: &str = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
        const BAD_HASH_ID: &str = "cccccccc-cccc-4ccc-8ccc-cccccccccccc";

        let restricted_text = "Publisher full-text excerpt display is prohibited by license.";
        let restricted_sha = sha256_hex(restricted_text.as_bytes());
        let displayable_text = "Open-access passage permitted for verbatim excerpt display.";
        let displayable_sha = sha256_hex(displayable_text.as_bytes());
        let mismatch_text = "Unit identifier does not match candidate identifier.";
        let mismatch_sha = sha256_hex(mismatch_text.as_bytes());

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(
                listener,
                Router::new()
                    .route(
                        "/v1/search",
                        post(|Json(body): Json<serde_json::Value>| async move {
                            let query = body.get("query").and_then(|v| v.as_str()).unwrap_or("");
                            let candidates = match query {
                                "mixed" => serde_json::json!([
                                    { "candidate_id": RESTRICTED_ID },
                                    { "candidate_id": DISPLAYABLE_ID }
                                ]),
                                "restricted" => {
                                    serde_json::json!([{ "candidate_id": RESTRICTED_ID }])
                                }
                                "unit-mismatch" => {
                                    serde_json::json!([{ "candidate_id": MISMATCH_UNIT_ID }])
                                }
                                "bad-sha256" => {
                                    serde_json::json!([{ "candidate_id": BAD_HASH_ID }])
                                }
                                _ => serde_json::json!([{ "candidate_id": RESTRICTED_ID }]),
                            };
                            let count = candidates.as_array().map(Vec::len).unwrap_or(0);
                            (
                                StatusCode::OK,
                                Json(serde_json::json!({
                                    "candidates": candidates,
                                    "page": {
                                        "examined_candidate_count": count,
                                        "returned_candidate_count": count,
                                        "unavailable_candidate_count": 0,
                                        "truncated": false
                                    }
                                })),
                            )
                        }),
                    )
                    .route(
                        "/v1/read",
                        post(move |Json(body): Json<serde_json::Value>| {
                            let restricted_sha = restricted_sha.clone();
                            let displayable_sha = displayable_sha.clone();
                            let mismatch_sha = mismatch_sha.clone();
                            async move {
                                let ids = body
                                    .get("candidate_ids")
                                    .and_then(|v| v.as_array())
                                    .cloned()
                                    .unwrap_or_default();
                                let mut results = Vec::with_capacity(ids.len());
                                for raw_id in ids {
                                    let id = raw_id.as_str().unwrap_or("");
                                    let item = match id {
                                        RESTRICTED_ID => serde_json::json!({
                                            "candidate_id": RESTRICTED_ID,
                                            "outcome": "available",
                                            "unit": {
                                                "unit_id": RESTRICTED_ID,
                                                "source_display_name": "Publisher Restricted",
                                                "title": "Restricted Excerpt Article",
                                                "publication_date": "2025-05-01",
                                                "acquired_at": "2026-10-01T00:00:00Z",
                                                "lifecycle_state": "current",
                                                "content": restricted_text,
                                                "content_sha256": restricted_sha,
                                                "native_destination_uri": "https://publisher.example.org/article/1",
                                                "attribution_text": "All rights reserved",
                                                "may_display_excerpt": false
                                            }
                                        }),
                                        DISPLAYABLE_ID => serde_json::json!({
                                            "candidate_id": DISPLAYABLE_ID,
                                            "outcome": "available",
                                            "unit": {
                                                "unit_id": DISPLAYABLE_ID,
                                                "source_display_name": "PubMed Central",
                                                "title": "Open-Access Excerpt Article",
                                                "publication_date": "2025-05-02",
                                                "acquired_at": "2026-10-01T00:00:00Z",
                                                "lifecycle_state": "current",
                                                "content": displayable_text,
                                                "content_sha256": displayable_sha,
                                                "native_destination_uri": "https://pmc.ncbi.nlm.nih.gov/articles/PMC99/",
                                                "attribution_text": "CC BY 4.0",
                                                "may_display_excerpt": true
                                            }
                                        }),
                                        MISMATCH_UNIT_ID => serde_json::json!({
                                            "candidate_id": MISMATCH_UNIT_ID,
                                            "outcome": "available",
                                            "unit": {
                                                "unit_id": "dddddddd-dddd-4ddd-8ddd-dddddddddddd",
                                                "source_display_name": "PubMed Central",
                                                "title": "Mismatched Unit ID",
                                                "publication_date": "2025-05-01",
                                                "acquired_at": "2026-10-01T00:00:00Z",
                                                "lifecycle_state": "current",
                                                "content": mismatch_text,
                                                "content_sha256": mismatch_sha,
                                                "native_destination_uri": "https://pmc.ncbi.nlm.nih.gov/articles/PMC2/",
                                                "attribution_text": "CC BY 4.0",
                                                "may_display_excerpt": true
                                            }
                                        }),
                                        _ => serde_json::json!({
                                            "candidate_id": BAD_HASH_ID,
                                            "outcome": "available",
                                            "unit": {
                                                "unit_id": BAD_HASH_ID,
                                                "source_display_name": "PubMed Central",
                                                "title": "Corrupted Digest",
                                                "publication_date": "2025-05-01",
                                                "acquired_at": "2026-10-01T00:00:00Z",
                                                "lifecycle_state": "current",
                                                "content": "Tampered passage body that does not match its digest.",
                                                "content_sha256": "0000000000000000000000000000000000000000000000000000000000000000",
                                                "native_destination_uri": "https://pmc.ncbi.nlm.nih.gov/articles/PMC3/",
                                                "attribution_text": "CC BY 4.0",
                                                "may_display_excerpt": true
                                            }
                                        }),
                                    };
                                    results.push(item);
                                }
                                (
                                    StatusCode::OK,
                                    Json(serde_json::json!({ "results": results })),
                                )
                            }
                        }),
                    ),
            )
            .await
            .unwrap();
        });

        let client = ClinicalLibraryClient::new(ClinicalLibraryConfig {
            origin: format!("http://{addr}"),
            auth: ClinicalLibraryAuth::LocalLoopback,
        })
        .unwrap();

        let ClinicalSearchOutcome::Found(mixed) = client.search("mixed", 5).await.unwrap() else {
            panic!(
                "mixed search must keep displayable unit while withholding non-excerptable unit"
            );
        };
        assert_eq!(mixed.len(), 1);
        assert_eq!(mixed[0].passage_id, DISPLAYABLE_ID);
        assert_eq!(mixed[0].text, displayable_text);

        assert_eq!(
            client.search("restricted", 5).await.unwrap(),
            ClinicalSearchOutcome::NothingFound,
            "may_display_excerpt=false unit must be withheld from search results"
        );
        assert!(
            matches!(
                client.read_passage(RESTRICTED_ID).await,
                Err(ClinicalLibraryError::PassageUnavailable {
                    ref passage_id,
                    cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
                }) if passage_id == RESTRICTED_ID
            ),
            "may_display_excerpt=false unit must be withheld on exact read"
        );

        assert!(
            matches!(
                client.search("unit-mismatch", 5).await,
                Err(ClinicalLibraryError::InvalidResponse {
                    reason: "returned unit_id did not match candidate_id",
                    ..
                })
            ),
            "unit_id mismatch must fail InvalidResponse on search"
        );
        assert!(
            matches!(
                client.read_passage(MISMATCH_UNIT_ID).await,
                Err(ClinicalLibraryError::InvalidResponse {
                    reason: "returned unit_id did not match candidate_id",
                    ..
                })
            ),
            "unit_id mismatch must fail InvalidResponse on exact read"
        );

        assert!(
            matches!(
                client.search("bad-sha256", 5).await,
                Err(ClinicalLibraryError::InvalidResponse {
                    reason: "passage content_sha256 did not match text digest",
                    ..
                })
            ),
            "SHA-256 mismatch must fail InvalidResponse on search"
        );
        assert!(
            matches!(
                client.read_passage(BAD_HASH_ID).await,
                Err(ClinicalLibraryError::InvalidResponse {
                    reason: "passage content_sha256 did not match text digest",
                    ..
                })
            ),
            "SHA-256 mismatch must fail InvalidResponse on exact read"
        );

        server.abort();
    }
}

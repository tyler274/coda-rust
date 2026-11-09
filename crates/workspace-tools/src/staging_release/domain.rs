//! Closed release identities and validated receipt values.
use super::{Error, Migration, PROJECT, Phase, REGION, Result, ServiceRecord};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum Product {
    Physician,
    Duet,
}
impl Product {
    pub const ALL: [Self; 2] = [Self::Physician, Self::Duet];
    pub fn name(self) -> &'static str {
        match self {
            Self::Physician => "physician",
            Self::Duet => "duet",
        }
    }
    pub fn job(self) -> &'static str {
        match self {
            Self::Physician => "physician-migrate",
            Self::Duet => "duet-migrate",
        }
    }
    pub fn service(self) -> ReleaseService {
        match self {
            Self::Physician => ReleaseService::Physician,
            Self::Duet => ReleaseService::Duet,
        }
    }
    pub fn migration_image(self) -> ImageRole {
        match self {
            Self::Physician => ImageRole::PhysicianMigration,
            Self::Duet => ImageRole::DuetMigration,
        }
    }
    pub fn phase(self, step: MigrationStep) -> Phase {
        match (self, step) {
            (Self::Physician, MigrationStep::Update) => Phase::PhysicianMigrationUpdate,
            (Self::Physician, MigrationStep::Execute) => Phase::PhysicianMigrationExecute,
            (Self::Physician, MigrationStep::Wait) => Phase::PhysicianMigrationWait,
            (Self::Duet, MigrationStep::Update) => Phase::DuetMigrationUpdate,
            (Self::Duet, MigrationStep::Execute) => Phase::DuetMigrationExecute,
            (Self::Duet, MigrationStep::Wait) => Phase::DuetMigrationWait,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub(super) enum MigrationStep {
    Update,
    Execute,
    Wait,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum ReleaseService {
    Physician,
    Duet,
    PhysicianWorker,
    ObservabilityRelay,
}
impl ReleaseService {
    pub const ALL: [Self; 4] = [
        Self::Physician,
        Self::Duet,
        Self::PhysicianWorker,
        Self::ObservabilityRelay,
    ];
    /// Services a release rolls when present and only publishes for while
    /// infrastructure has not created them yet.
    pub const OPTIONAL: [Self; 2] = [Self::PhysicianWorker, Self::ObservabilityRelay];
    pub fn name(self) -> &'static str {
        match self {
            Self::Physician => "physician",
            Self::Duet => "duet",
            Self::PhysicianWorker => "physician-worker",
            Self::ObservabilityRelay => "observability-relay",
        }
    }
    pub fn image(self) -> ImageRole {
        match self {
            Self::Physician => ImageRole::PhysicianRuntime,
            Self::Duet => ImageRole::DuetRuntime,
            Self::PhysicianWorker => ImageRole::PhysicianWorker,
            Self::ObservabilityRelay => ImageRole::ObservabilityRelay,
        }
    }
    pub fn phase(self) -> Phase {
        match self {
            Self::Physician => Phase::PhysicianRollout,
            Self::Duet => Phase::DuetRollout,
            Self::PhysicianWorker => Phase::WorkerRollout,
            Self::ObservabilityRelay => Phase::RelayRollout,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum ImageRole {
    PhysicianRuntime,
    PhysicianWorker,
    PhysicianMigration,
    DuetRuntime,
    DuetMigration,
    ObservabilityRelay,
}
impl ImageRole {
    pub const ALL: [Self; 6] = [
        Self::PhysicianRuntime,
        Self::PhysicianWorker,
        Self::PhysicianMigration,
        Self::DuetRuntime,
        Self::DuetMigration,
        Self::ObservabilityRelay,
    ];
    pub fn key(self) -> &'static str {
        match self {
            Self::PhysicianRuntime => "physician-runtime",
            Self::PhysicianWorker => "physician-worker",
            Self::PhysicianMigration => "physician-migration",
            Self::DuetRuntime => "duet-runtime",
            Self::DuetMigration => "duet-migration",
            Self::ObservabilityRelay => "observability-relay",
        }
    }
    pub fn repository(self) -> &'static str {
        match self {
            Self::PhysicianRuntime => "coda-physician-sample",
            Self::PhysicianWorker => "coda-physician-worker",
            Self::PhysicianMigration => "coda-physician-migrate",
            Self::DuetRuntime => "coda-duet-api",
            Self::DuetMigration => "coda-duet-migrate",
            Self::ObservabilityRelay => "coda-observability-relay",
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Services {
    physician: ServiceRecord,
    duet: ServiceRecord,
    #[serde(rename = "physician-worker")]
    worker: ServiceRecord,
    #[serde(rename = "observability-relay")]
    relay: ServiceRecord,
}
impl Services {
    pub fn new(record: ServiceRecord) -> Self {
        Self {
            physician: record.clone(),
            duet: record.clone(),
            worker: record.clone(),
            relay: record,
        }
    }
    pub fn get(&self, service: ReleaseService) -> &ServiceRecord {
        match service {
            ReleaseService::Physician => &self.physician,
            ReleaseService::Duet => &self.duet,
            ReleaseService::PhysicianWorker => &self.worker,
            ReleaseService::ObservabilityRelay => &self.relay,
        }
    }
    pub fn get_mut(&mut self, service: ReleaseService) -> &mut ServiceRecord {
        match service {
            ReleaseService::Physician => &mut self.physician,
            ReleaseService::Duet => &mut self.duet,
            ReleaseService::PhysicianWorker => &mut self.worker,
            ReleaseService::ObservabilityRelay => &mut self.relay,
        }
    }
    #[cfg(test)]
    pub fn values(&self) -> impl Iterator<Item = &ServiceRecord> {
        [&self.physician, &self.duet, &self.worker, &self.relay].into_iter()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Migrations {
    physician: Migration,
    duet: Migration,
}
impl Migrations {
    pub fn new(record: Migration) -> Self {
        Self {
            physician: record.clone(),
            duet: record,
        }
    }
    pub fn get(&self, product: Product) -> &Migration {
        match product {
            Product::Physician => &self.physician,
            Product::Duet => &self.duet,
        }
    }
    pub fn get_mut(&mut self, product: Product) -> &mut Migration {
        match product {
            Product::Physician => &mut self.physician,
            Product::Duet => &mut self.duet,
        }
    }
    #[cfg(test)]
    pub fn values(&self) -> impl Iterator<Item = &Migration> {
        [&self.physician, &self.duet].into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub(super) struct PublishedImage {
    reference: String,
    #[serde(skip)]
    role: ImageRole,
}
impl PublishedImage {
    pub fn parse(role: ImageRole, reference: &str) -> Result<Self> {
        let prefix = format!(
            "{REGION}-docker.pkg.dev/{PROJECT}/apps/{}@sha256:",
            role.repository()
        );
        let digest = reference
            .strip_prefix(&prefix)
            .ok_or(Error::Input("unexpected image repository"))?;
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err(Error::Input("immutable digest required"));
        }
        Ok(Self {
            reference: reference.to_owned(),
            role,
        })
    }
    pub fn as_str(&self) -> &str {
        &self.reference
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Images {
    #[serde(rename = "physician-runtime", deserialize_with = "physician_runtime")]
    physician_runtime: Option<PublishedImage>,
    #[serde(rename = "physician-worker", deserialize_with = "physician_worker")]
    physician_worker: Option<PublishedImage>,
    #[serde(
        rename = "physician-migration",
        deserialize_with = "physician_migration"
    )]
    physician_migration: Option<PublishedImage>,
    #[serde(rename = "duet-runtime", deserialize_with = "duet_runtime")]
    duet_runtime: Option<PublishedImage>,
    #[serde(rename = "duet-migration", deserialize_with = "duet_migration")]
    duet_migration: Option<PublishedImage>,
    #[serde(
        rename = "observability-relay",
        deserialize_with = "observability_relay"
    )]
    observability_relay: Option<PublishedImage>,
}
macro_rules! image_parser {
    ($function:ident, $role:ident) => {
        fn $function<'de, D: serde::Deserializer<'de>>(
            de: D,
        ) -> std::result::Result<Option<PublishedImage>, D::Error> {
            Option::<String>::deserialize(de)?
                .map(|s| {
                    PublishedImage::parse(ImageRole::$role, &s).map_err(serde::de::Error::custom)
                })
                .transpose()
        }
    };
}
image_parser!(physician_runtime, PhysicianRuntime);
image_parser!(physician_worker, PhysicianWorker);
image_parser!(physician_migration, PhysicianMigration);
image_parser!(duet_runtime, DuetRuntime);
image_parser!(duet_migration, DuetMigration);
image_parser!(observability_relay, ObservabilityRelay);
impl Images {
    pub fn get(&self, role: ImageRole) -> Option<&PublishedImage> {
        match role {
            ImageRole::PhysicianRuntime => self.physician_runtime.as_ref(),
            ImageRole::PhysicianWorker => self.physician_worker.as_ref(),
            ImageRole::PhysicianMigration => self.physician_migration.as_ref(),
            ImageRole::DuetRuntime => self.duet_runtime.as_ref(),
            ImageRole::DuetMigration => self.duet_migration.as_ref(),
            ImageRole::ObservabilityRelay => self.observability_relay.as_ref(),
        }
    }
    pub fn insert(&mut self, image: PublishedImage) {
        let slot = match image.role {
            ImageRole::PhysicianRuntime => &mut self.physician_runtime,
            ImageRole::PhysicianWorker => &mut self.physician_worker,
            ImageRole::PhysicianMigration => &mut self.physician_migration,
            ImageRole::DuetRuntime => &mut self.duet_runtime,
            ImageRole::DuetMigration => &mut self.duet_migration,
            ImageRole::ObservabilityRelay => &mut self.observability_relay,
        };
        *slot = Some(image);
    }
    pub fn complete(&self) -> bool {
        ImageRole::ALL
            .into_iter()
            .all(|role| self.get(role).is_some())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub(super) struct ObservedImageRef(String);
impl TryFrom<String> for ObservedImageRef {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        // Preserve the cloud projection's permissive image-reference contract.
        if value.len() > 512
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-._/:@".contains(&b))
        {
            return Err(Error::Cloud(LocalCode::InvalidCloudImage));
        }
        Ok(Self(value))
    }
}
impl From<ObservedImageRef> for String {
    fn from(value: ObservedImageRef) -> Self {
        value.0
    }
}
impl ObservedImageRef {
    pub fn matches(&self, image: &PublishedImage) -> bool {
        self.0 == image.as_str()
    }
}
macro_rules! identifier_type {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub(super) struct $name(String);
        impl TryFrom<String> for $name {
            type Error = Error;
            fn try_from(value: String) -> Result<Self> {
                if !super::identifier(&value) {
                    return Err(Error::Cloud(LocalCode::InvalidCloudIdentifier));
                }
                Ok(Self(value))
            }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}
identifier_type!(ExecutionId);
identifier_type!(RevisionId);

macro_rules! codes {
    ($name:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub(super) enum $name { $(#[serde(rename = $text)] $variant),+ }
        impl $name { pub fn as_str(self) -> &'static str { match self { $(Self::$variant => $text),+ } } }
        impl fmt::Display for $name { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) } }
    };
}
codes!(LocalCode {
    InvalidInput => "invalid-input",
    IoFailed => "io-failed",
    JsonInvalid => "json-invalid",
    ObservationsFailed => "observations-failed",
    MigrationUpdateNotConfirmed => "migration-update-not-confirmed",
    JobUpdateResponseReconciled => "job-update-response-reconciled",
    MigrationExecutionUnknown => "migration-execution-unknown",
    ExecuteResponseReconciled => "execute-response-reconciled",
    MigrationFailed => "migration-failed",
    MigrationTimeout => "migration-timeout",
    UpdateFailedSpecUnchanged => "update-failed-spec-unchanged",
    UpdateResponseFailedReconciled => "update-response-failed-reconciled",
    RolloutNotConfirmed => "rollout-not-confirmed",
    BeforeStateUnknown => "before-state-unknown",
    RequiredServiceAbsent => "required-service-absent",
    ReleaseStopped => "release-stopped",
    FinalStateUnknown => "final-state-unknown",
    MissingCloudField => "missing-cloud-field",
    InvalidCloudIdentifier => "invalid-cloud-identifier",
    MissingCloudImage => "missing-cloud-image",
    InvalidCloudImage => "invalid-cloud-image",
    MissingTraffic => "missing-traffic",
    InvalidExecutions => "invalid-executions",
    ExecutionImageMismatch => "execution-image-mismatch",
    MissingExecutionStatus => "missing-execution-status",
    InvalidPublicationEvidence => "invalid-publication-evidence",
    FinalizedIncompleteRelease => "finalized-incomplete-release",
    ReleaseIncomplete => "release-incomplete",
    InitialStateUnknown => "initial-state-unknown",
    ReleaseCommandBudgetExhausted => "release-command-budget-exhausted",
    MissingStdout => "missing-stdout",
    MissingStderr => "missing-stderr",
    CommandPipeDrainTimeout => "command-pipe-drain-timeout",
    StdoutReaderPanicked => "stdout-reader-panicked",
    StderrReaderPanicked => "stderr-reader-panicked",
    GcloudTimeout => "gcloud-timeout",
    GcloudCommandFailed => "gcloud-command-failed",
    ReceiptUnwritable => "receipt-unwritable",
    Unused => "unused",
    LostResponse => "lost-response",
    WorkerUpdateFailed => "worker-update-failed",
    ProductContainerNotFound => "product-container-not-found",
    UnexpectedContainerSet => "unexpected-container-set",
    TelemetryContractIncomplete => "telemetry-contract-incomplete",
    ContainerSetChanged => "container-set-changed",
    InfrastructureFieldChanged => "infrastructure-field-changed"
});
impl LocalCode {
    /// The service's template is not one infrastructure produces, or a
    /// rollout moved a field infrastructure owns.
    pub fn is_shape_refusal(self) -> bool {
        matches!(
            self,
            Self::ProductContainerNotFound
                | Self::UnexpectedContainerSet
                | Self::TelemetryContractIncomplete
                | Self::ContainerSetChanged
                | Self::InfrastructureFieldChanged
        )
    }
}
codes!(ApiStatus {
    PermissionDenied => "PERMISSION_DENIED",
    Unauthenticated => "UNAUTHENTICATED",
    NotFound => "NOT_FOUND",
    AlreadyExists => "ALREADY_EXISTS",
    InvalidArgument => "INVALID_ARGUMENT",
    FailedPrecondition => "FAILED_PRECONDITION",
    ResourceExhausted => "RESOURCE_EXHAUSTED",
    DeadlineExceeded => "DEADLINE_EXCEEDED",
    Unavailable => "UNAVAILABLE",
    Aborted => "ABORTED",
    Internal => "INTERNAL"
});
impl ApiStatus {
    pub fn from_stderr(stderr: &[u8]) -> Option<Self> {
        match crate::failure::api_status(stderr)? {
            "PERMISSION_DENIED" => Some(Self::PermissionDenied),
            "UNAUTHENTICATED" => Some(Self::Unauthenticated),
            "NOT_FOUND" => Some(Self::NotFound),
            "ALREADY_EXISTS" => Some(Self::AlreadyExists),
            "INVALID_ARGUMENT" => Some(Self::InvalidArgument),
            "FAILED_PRECONDITION" => Some(Self::FailedPrecondition),
            "RESOURCE_EXHAUSTED" => Some(Self::ResourceExhausted),
            "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
            "UNAVAILABLE" => Some(Self::Unavailable),
            "ABORTED" => Some(Self::Aborted),
            "INTERNAL" => Some(Self::Internal),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum DiagnosticCode {
    Local(LocalCode),
    Api(ApiStatus),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closed_identities_reject_unknown_spellings() {
        for invalid in ["production", "Physician", "", "physician-runtime"] {
            let value = serde_json::Value::String(invalid.into());
            assert!(serde_json::from_value::<Product>(value.clone()).is_err());
            assert!(serde_json::from_value::<ReleaseService>(value).is_err());
        }
        assert!(serde_json::from_str::<ImageRole>("\"physician\"").is_err());
        for role in ImageRole::ALL {
            assert_eq!(serde_json::to_value(role).unwrap(), role.key());
        }
    }
    #[test]
    fn published_images_validate_repository_and_lowercase_digest() {
        for role in ImageRole::ALL {
            let prefix = format!(
                "{REGION}-docker.pkg.dev/{PROJECT}/apps/{}@sha256:",
                role.repository()
            );
            for digest in [
                "a".repeat(63),
                "a".repeat(65),
                "A".repeat(64),
                "z".repeat(64),
                String::new(),
            ] {
                assert!(PublishedImage::parse(role, &format!("{prefix}{digest}")).is_err());
            }
            let reference = format!("{prefix}{}", "a".repeat(64));
            assert!(PublishedImage::parse(role, &reference).is_ok());
            assert!(
                PublishedImage::parse(role, &reference.replace(PROJECT, "production")).is_err()
            );
        }
    }
    #[test]
    fn observed_images_keep_the_existing_permissive_contract() {
        for reference in ["", "image:latest", "image@sha256:abc", "old-image"] {
            let image =
                ObservedImageRef::try_from(reference.to_owned()).expect("observed reference");
            assert_eq!(serde_json::to_value(image).unwrap(), reference);
        }
        for reference in [
            "image secret".to_owned(),
            "image\nsecret".to_owned(),
            "x".repeat(513),
        ] {
            assert!(ObservedImageRef::try_from(reference).is_err());
        }
    }
    #[test]
    fn execution_and_revision_ids_validate_on_deserialization() {
        for value in [
            "".to_owned(),
            "bad/id".to_owned(),
            "secret text".to_owned(),
            "x".repeat(256),
        ] {
            let value = serde_json::Value::String(value);
            assert!(serde_json::from_value::<ExecutionId>(value.clone()).is_err());
            assert!(serde_json::from_value::<RevisionId>(value).is_err());
        }
        let id = ExecutionId::try_from("job-001".to_owned()).unwrap();
        assert_eq!(serde_json::to_value(id).unwrap(), "job-001");
    }
}

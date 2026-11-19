//! Real PostgreSQL integration tests for message and follow-up producers in the
//! Today and Updates read model: routing of every message and follow-up kind,
//! panel-gated match suggestions, `Done` clearance, partial-failure isolation,
//! and the joint reply + concern lifecycle on Today.

#![expect(
    dead_code,
    reason = "Today/Updates message and follow-up test scenarios land in companion slice"
)]
#![expect(
    unused_imports,
    reason = "test imports are exercised in companion suite slice"
)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, Weak};

use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::concerns::{
    CommandKey, ConcernId, ConcernUrgency, ExactRecordReference, RaiseConcern, SourceWork,
    WorkspaceAccess,
};
use local_product::runtime::follow_ups::{
    FollowUpChangeNote, FollowUpCommandKey, FollowUpId, FollowUpReviewOutcome, FollowUpSummary,
    PhysicianChangeFollowUp, PhysicianCreateFollowUp, WithdrawWrongPatientRequest,
};
use local_product::runtime::reply::proposals::{
    AnsweredMessageClosure, ChannelConnection, CompleteReplyReview, ConversationLink,
    MessageSender, ObservePracticeMessage, PracticeMessageKind, RecordPatientMessage,
    ReplyProposalBasisInput, ReplyProposalInput, ReplyReviewDecision,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    ChannelStopCause, IncomingSpruceItem, MatchAssessment, MatchingDecision, MessageStanding,
    MessageUrgencyChoice, NonPatientCategory, OutsideParticipant, ReadableSpruceItem,
    SenderAddress, UnreadableSpruceItem,
};
use local_product::runtime::today_updates::{
    ClearItemDoneRequest, OpenItemRequest, PRACTICE_MESSAGING_WORKSPACE, ProducerKind,
    ReadCompleteness, SeenStanding, TodayDoneResult, TodayRowPayload, TodayUpdatesProbe,
    UnhandledWorkKind, UpdateEntryKind,
};
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, async_trait};
use local_product::sha256_hex;
use local_product::spruce_matching_recipe::{GUIDANCE_VERSION, RECIPE_VERSION};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::json;
use uuid::Uuid;

fn current_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_secs()
}

struct LocalCluster {
    cluster_dir: PathBuf,
    data_dir: PathBuf,
    admin_url: String,
}

impl Drop for LocalCluster {
    fn drop(&mut self) {
        let _ = Command::new("pg_ctl")
            .args([
                "-D",
                self.data_dir.to_str().unwrap_or_default(),
                "-m",
                "immediate",
                "stop",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = std::fs::remove_dir_all(&self.cluster_dir);
    }
}

static LOCAL_CLUSTER: StdMutex<Weak<LocalCluster>> = StdMutex::new(Weak::new());
static MIGRATE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn acquire_database() -> Option<(String, Option<Arc<LocalCluster>>)> {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
    ] {
        if let Ok(url) = std::env::var(key) {
            let trimmed = url.trim().to_owned();
            if !trimmed.is_empty() {
                return Some((trimmed, None));
            }
        }
    }

    let mut slot = LOCAL_CLUSTER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = slot.upgrade() {
        return Some((existing.admin_url.clone(), Some(existing)));
    }

    let port = {
        use std::net::TcpListener;
        (0..500)
            .find_map(|_| {
                let candidate = 32_000 + (Uuid::new_v4().as_u128() % 9_000) as u16;
                TcpListener::bind(("127.0.0.1", candidate))
                    .ok()
                    .map(|_| candidate)
            })
            .expect("free loopback port for local postgres")
    };
    let cluster_dir =
        std::env::temp_dir().join(format!("coda-tumf-pg-{}-{}", std::process::id(), port));
    let _ = std::fs::remove_dir_all(&cluster_dir);
    let data_dir = cluster_dir.join("data");
    let sock_dir = cluster_dir.join("sock");
    std::fs::create_dir_all(&data_dir).ok()?;
    std::fs::create_dir_all(&sock_dir).ok()?;

    let init = Command::new("initdb")
        .args([
            "-D",
            data_dir.to_str()?,
            "-U",
            "postgres",
            "-A",
            "trust",
            "--no-locale",
            "-E",
            "UTF8",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !init.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let log_file = cluster_dir.join("postgres.log");
    let opts = format!("-F -p {port} -h 127.0.0.1 -k {}", sock_dir.to_str()?);
    let start = Command::new("pg_ctl")
        .args([
            "-D",
            data_dir.to_str()?,
            "-l",
            log_file.to_str()?,
            "-o",
            &opts,
            "-w",
            "start",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !start.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let cluster = Arc::new(LocalCluster {
        cluster_dir,
        data_dir,
        admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
    });
    *slot = Arc::downgrade(&cluster);
    Some((cluster.admin_url.clone(), Some(cluster)))
}

async fn connect_stores() -> Option<(Store, Arc<PostgresAuthStore>, Option<Arc<LocalCluster>>)> {
    let Some((url, cluster)) = acquire_database() else {
        assert!(
            std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
            "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
        );
        return None;
    };
    let _guard = MIGRATE_LOCK.lock().await;
    let store = Store::connect(&url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    drop(_guard);
    Some((store, auth, cluster))
}

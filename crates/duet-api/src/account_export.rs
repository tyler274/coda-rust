//! Rights-never-wait full personal data export (`intent/duet.md` D5.1).
//!
//! Exports every kind of patient-owned record with its source and date inside one
//! `AccountDatabase::begin` RLS transaction using `AccessLevel::RightsOnly`, so
//! export never waits on a source refresh or on newer Terms. Internal credential
//! hashes, OAuth tokens, and internal execution machinery are never included.

use std::collections::HashMap;

use axum::Json;
use axum::http::header;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QueryTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    ApiError,
    database::{AccountDatabase, orm},
    identity::{AccessLevel, AccountContext, check_current},
};

mod export_entity {
    pub mod user {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "users")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub email: Option<String>,
            pub display_name: Option<String>,
            pub appearance: String,
            pub sharing_label: String,
            pub terms_version: Option<String>,
            pub terms_accepted_at: Option<DateTimeUtc>,
            pub intelligence_enabled: bool,
            pub luna_consent_version: Option<String>,
            pub luna_consent_decision: Option<String>,
            pub luna_consent_at: Option<DateTimeUtc>,
            pub apple_health_generation: i32,
            pub apple_health_disconnected_at: Option<DateTimeUtc>,
            pub health_records_generation: i32,
            pub health_records_disconnected_at: Option<DateTimeUtc>,
            pub health_connect_generation: i32,
            pub health_connect_disconnected_at: Option<DateTimeUtc>,
            pub doctor_note: Option<String>,
            pub doctor_note_generation: i64,
            pub doctor_page: Json,
            pub doctor_page_updated_at: DateTimeUtc,
            pub created_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod health_evidence {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "health_evidence")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub user_id: Uuid,
            pub concept_id: String,
            pub source_kind: String,
            pub source_connection_id: Option<String>,
            pub source_record_id: Option<String>,
            pub measured_at: Option<DateTimeUtc>,
            pub received_at: DateTimeUtc,
            pub payload: Json,
            pub created_at: DateTimeUtc,
            pub updated_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod clinical_note {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "clinical_notes")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub user_id: Uuid,
            pub source_id: Option<String>,
            pub title: String,
            pub preview: Option<String>,
            pub body: Option<String>,
            pub recorded_at: Option<DateTimeUtc>,
            pub source_label: String,
            pub created_at: DateTimeUtc,
            pub duet_source_payload: Option<Json>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod revision {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "canonical_evidence_revisions")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub revision: i64,
            pub user_id: Uuid,
            pub evidence_id: Option<Uuid>,
            pub note_id: Option<Uuid>,
            pub payload: Json,
            pub retained_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod device_snapshot {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "duet_device_snapshots")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub user_id: Uuid,
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub generated_at: DateTimeUtc,
            pub payload: Json,
            pub received_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod doctor_item {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "doctor_collection_items")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub user_id: Uuid,
            pub kind: String,
            pub source_id: String,
            pub title: String,
            pub detail: String,
            pub evidence: Json,
            pub range_start: Option<DateTimeUtc>,
            pub range_end: Option<DateTimeUtc>,
            pub saved_at: DateTimeUtc,
            pub position: i32,
            pub page_entry: Option<Json>,
            pub medication_confirmed: bool,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod portal_connection {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "duet_portal_connections")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub user_id: Uuid,
            pub organization_name: String,
            pub vendor: String,
            pub brand: String,
            pub state: String,
            pub last_synced_at: Option<DateTimeUtc>,
            pub failing_since: Option<DateTimeUtc>,
            pub failure_reason: Option<String>,
            pub disconnected_at: Option<DateTimeUtc>,
            pub created_at: DateTimeUtc,
            pub updated_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod conversation {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "duet_conversations")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub user_id: Uuid,
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub created_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod command {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "duet_commands")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub user_id: Uuid,
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub conversation_id: Uuid,
            pub input: String,
            pub state: String,
            pub reply: Option<Json>,
            pub created_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod private_memory {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "duet_private_memory")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub user_id: Uuid,
            pub revision: i64,
            pub content: Json,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod privacy_receipt {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(schema_name = "public", table_name = "privacy_recovery_events")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: Uuid,
            pub account_id: Uuid,
            pub operation: String,
            pub scope_id: Uuid,
            pub occurred_at: DateTimeUtc,
            pub journaled_at: Option<DateTimeUtc>,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }
}

/// Complete portable personal data export for one Duet account.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountExport {
    /// Export schema version.
    pub schema_version: u32,
    /// UTC timestamp when Duet produced this export.
    pub exported_at: DateTime<Utc>,
    /// Account identity profile, preferences, and consent standing (never secret hashes).
    pub account: AccountProfileExport,
    /// Imported and user-entered canonical health records and retained source revisions.
    pub health_evidence: Vec<ExportedEvidenceRecord>,
    /// Clinical notes with source label, date, and retained revisions.
    pub clinical_notes: Vec<ExportedClinicalNote>,
    /// Raw device snapshots retained on the account.
    pub device_snapshots: Vec<ExportedDeviceSnapshot>,
    /// Current saved `For my doctor` page and selected items.
    pub doctor_page: ExportedDoctorPage,
    /// Connected or disconnected portal accounts (metadata only, never OAuth tokens).
    pub portal_connections: Vec<ExportedPortalConnection>,
    /// Assistant conversations and turn records.
    pub conversations: Vec<ExportedConversation>,
    /// Duet's current understanding/memory of the person (stays exportable even if AI paused).
    pub private_memory: Option<ExportedPrivateMemory>,
    /// Committed privacy restriction receipts (audit trail without secrets).
    pub privacy_receipts: Vec<ExportedPrivacyReceipt>,
}

/// Exported account profile and consent standing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountProfileExport {
    /// Internal account ID.
    pub account_id: Uuid,
    /// Verified email address.
    pub email: Option<String>,
    /// Chosen display name.
    pub display_name: Option<String>,
    /// Chosen appearance (`system`, `light`, `dark`).
    pub appearance: String,
    /// Sharing label.
    pub sharing_label: String,
    /// Accepted Terms version, if any.
    pub terms_version: Option<String>,
    /// When Terms were accepted.
    pub terms_accepted_at: Option<DateTime<Utc>>,
    /// Current AI permission flag.
    pub ai_enabled: bool,
    /// Accepted AI consent version, if any.
    pub ai_consent_version: Option<String>,
    /// AI consent decision, if recorded.
    pub ai_consent_decision: Option<String>,
    /// When AI consent was recorded.
    pub ai_consent_at: Option<DateTime<Utc>>,
    /// Apple Health source generation and last disconnect date.
    pub apple_health_generation: i32,
    /// Apple Health disconnect timestamp, if disconnected.
    pub apple_health_disconnected_at: Option<DateTime<Utc>>,
    /// Health Records source generation and last disconnect date.
    pub health_records_generation: i32,
    /// Health Records disconnect timestamp, if disconnected.
    pub health_records_disconnected_at: Option<DateTime<Utc>>,
    /// Health Connect source generation and last disconnect date.
    pub health_connect_generation: i32,
    /// Health Connect disconnect timestamp, if disconnected.
    pub health_connect_disconnected_at: Option<DateTime<Utc>>,
    /// Account creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Exported immutable canonical revision.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedEvidenceRevision {
    /// Monotonic account revision number.
    pub revision: i64,
    /// Date Duet retained this revision.
    pub retained_at: DateTime<Utc>,
    /// Original literal payload for this revision.
    pub payload: Value,
}

/// Exported health evidence record with provenance, dates, and retained revisions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedEvidenceRecord {
    /// Canonical record ID.
    pub id: Uuid,
    /// Concept identifier.
    pub concept_id: String,
    /// Source kind (`appleHealth`, `healthRecords`, `healthConnect`, `epic`, `oracle`, `manual`, etc.).
    pub source_kind: String,
    /// Connection identifier, when imported from a portal/source connection.
    pub source_connection_id: Option<String>,
    /// Source-native record ID.
    pub source_record_id: Option<String>,
    /// Measurement date declared by the source, if known.
    pub measured_at: Option<DateTime<Utc>>,
    /// Date Duet received the latest revision.
    pub received_at: DateTime<Utc>,
    /// Date the record was first saved.
    pub created_at: DateTime<Utc>,
    /// Date the record was last updated.
    pub updated_at: DateTime<Utc>,
    /// Latest literal JSON payload.
    pub payload: Value,
    /// Complete revision history retained for this record.
    pub revisions: Vec<ExportedEvidenceRevision>,
}

/// Exported clinical note with source and dates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedClinicalNote {
    /// Canonical note ID.
    pub id: Uuid,
    /// Human-readable source label.
    pub source_label: String,
    /// Source-native note identifier.
    pub source_id: Option<String>,
    /// Note title.
    pub title: String,
    /// Note preview, if any.
    pub preview: Option<String>,
    /// Full note body, if any.
    pub body: Option<String>,
    /// Date declared by the source, if known.
    pub recorded_at: Option<DateTime<Utc>>,
    /// Date Duet stored the note.
    pub created_at: DateTime<Utc>,
    /// Original source payload, if retained.
    pub source_payload: Option<Value>,
    /// Complete revision history retained for this note.
    pub revisions: Vec<ExportedEvidenceRevision>,
}

/// Exported device snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedDeviceSnapshot {
    /// Snapshot ID.
    pub id: Uuid,
    /// Date the snapshot was generated on the device.
    pub generated_at: DateTime<Utc>,
    /// Date Duet received the snapshot.
    pub received_at: DateTime<Utc>,
    /// Snapshot JSON payload.
    pub payload: Value,
}

/// Exported `For my doctor` workspace and saved items.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedDoctorPage {
    /// Saved patient note.
    pub note: Option<String>,
    /// Current note/page generation.
    pub note_generation: i64,
    /// Saved page structure.
    pub page: Value,
    /// Date the page was last updated.
    pub page_updated_at: DateTime<Utc>,
    /// Saved items on `For my doctor`.
    pub items: Vec<ExportedDoctorItem>,
}

/// One exported item from `For my doctor`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedDoctorItem {
    /// Item ID.
    pub id: Uuid,
    /// Item kind.
    pub kind: String,
    /// Source identifier.
    pub source_id: String,
    /// Display title.
    pub title: String,
    /// Detail text.
    pub detail: String,
    /// Linked evidence entries.
    pub evidence: Value,
    /// Range start, if any.
    pub range_start: Option<DateTime<Utc>>,
    /// Range end, if any.
    pub range_end: Option<DateTime<Utc>>,
    /// Date saved onto the page.
    pub saved_at: DateTime<Utc>,
    /// Order index.
    pub position: i32,
    /// Rendered entry structure, if saved.
    pub page_entry: Option<Value>,
    /// Whether the patient confirmed this medication.
    pub medication_confirmed: bool,
}

/// Exported connected portal organization metadata (zero OAuth secrets).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedPortalConnection {
    /// Connection ID.
    pub id: Uuid,
    /// Portal vendor (`epic`, `oracle`).
    pub vendor: String,
    /// Organization display name.
    pub organization_name: String,
    /// Brand name.
    pub brand: String,
    /// Connection state (`connected`, `partial`, `broken`, `disconnected`).
    pub state: String,
    /// Last successful sync date.
    pub last_synced_at: Option<DateTime<Utc>>,
    /// First failing date if broken.
    pub failing_since: Option<DateTime<Utc>>,
    /// Non-secret failure reason.
    pub failure_reason: Option<String>,
    /// Disconnected date, if disconnected.
    pub disconnected_at: Option<DateTime<Utc>>,
    /// Connection created date.
    pub created_at: DateTime<Utc>,
    /// Connection updated date.
    pub updated_at: DateTime<Utc>,
}

/// Exported conversation turn.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedConversationTurn {
    /// Turn ID.
    pub id: Uuid,
    /// Patient message.
    pub input: String,
    /// Turn completion state.
    pub state: String,
    /// Assistant reply payload, if completed.
    pub reply: Option<Value>,
    /// Turn creation date.
    pub created_at: DateTime<Utc>,
}

/// Exported assistant conversation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedConversation {
    /// Conversation ID.
    pub id: Uuid,
    /// Conversation created date.
    pub created_at: DateTime<Utc>,
    /// Turns belonging to this conversation in chronological order.
    pub turns: Vec<ExportedConversationTurn>,
}

/// Exported Duet private memory state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedPrivateMemory {
    /// Current memory revision.
    pub revision: i64,
    /// Structured memory content.
    pub content: Value,
}

/// Exported privacy restriction receipt metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedPrivacyReceipt {
    /// Idempotent event identity.
    pub event_id: Uuid,
    /// Privacy operation name.
    pub operation: String,
    /// Target resource or account scope ID.
    pub scope_id: Uuid,
    /// Date committed.
    pub occurred_at: DateTime<Utc>,
    /// Date confirmed in independent privacy journal.
    pub journaled_at: Option<DateTime<Utc>>,
}

/// Export every patient-owned category inside one RLS-scoped read transaction.
/// Uses `AccessLevel::RightsOnly` so export never waits on source refreshes or newer Terms.
pub async fn build_export(
    database: &AccountDatabase,
    context: &AccountContext,
) -> Result<AccountExport, ApiError> {
    let mut tx = database.begin(context.account_id).await?;
    check_current(&mut tx, context, AccessLevel::RightsOnly).await?;

    let user = orm::model::<export_entity::user::Model>(
        &mut tx,
        &export_entity::user::Entity::find()
            .filter(export_entity::user::Column::Id.eq(context.account_id))
            .into_query(),
    )
    .await?
    .ok_or(ApiError::AbsentAccount)?;

    let evidence_rows = orm::models::<export_entity::health_evidence::Model>(
        &mut tx,
        &export_entity::health_evidence::Entity::find()
            .filter(export_entity::health_evidence::Column::UserId.eq(context.account_id))
            .order_by_asc(export_entity::health_evidence::Column::ReceivedAt)
            .order_by_asc(export_entity::health_evidence::Column::Id)
            .into_query(),
    )
    .await?;

    let note_rows = orm::models::<export_entity::clinical_note::Model>(
        &mut tx,
        &export_entity::clinical_note::Entity::find()
            .filter(export_entity::clinical_note::Column::UserId.eq(context.account_id))
            .order_by_asc(export_entity::clinical_note::Column::CreatedAt)
            .order_by_asc(export_entity::clinical_note::Column::Id)
            .into_query(),
    )
    .await?;

    let revision_rows = orm::models::<export_entity::revision::Model>(
        &mut tx,
        &export_entity::revision::Entity::find()
            .filter(export_entity::revision::Column::UserId.eq(context.account_id))
            .order_by_asc(export_entity::revision::Column::Revision)
            .into_query(),
    )
    .await?;

    let mut evidence_revs: HashMap<Uuid, Vec<ExportedEvidenceRevision>> = HashMap::new();
    let mut note_revs: HashMap<Uuid, Vec<ExportedEvidenceRevision>> = HashMap::new();
    for rev in revision_rows {
        let entry = ExportedEvidenceRevision {
            revision: rev.revision,
            retained_at: rev.retained_at,
            payload: rev.payload,
        };
        if let Some(eid) = rev.evidence_id {
            evidence_revs.entry(eid).or_default().push(entry);
        } else if let Some(nid) = rev.note_id {
            note_revs.entry(nid).or_default().push(entry);
        }
    }

    let snapshot_rows = orm::models::<export_entity::device_snapshot::Model>(
        &mut tx,
        &export_entity::device_snapshot::Entity::find()
            .filter(export_entity::device_snapshot::Column::UserId.eq(context.account_id))
            .order_by_asc(export_entity::device_snapshot::Column::ReceivedAt)
            .order_by_asc(export_entity::device_snapshot::Column::Id)
            .into_query(),
    )
    .await?;

    let doctor_items = orm::models::<export_entity::doctor_item::Model>(
        &mut tx,
        &export_entity::doctor_item::Entity::find()
            .filter(export_entity::doctor_item::Column::UserId.eq(context.account_id))
            .order_by_asc(export_entity::doctor_item::Column::Position)
            .order_by_asc(export_entity::doctor_item::Column::Id)
            .into_query(),
    )
    .await?;

    let portal_rows = orm::models::<export_entity::portal_connection::Model>(
        &mut tx,
        &export_entity::portal_connection::Entity::find()
            .filter(export_entity::portal_connection::Column::UserId.eq(context.account_id))
            .order_by_asc(export_entity::portal_connection::Column::CreatedAt)
            .order_by_asc(export_entity::portal_connection::Column::Id)
            .into_query(),
    )
    .await?;

    let conv_rows = orm::models::<export_entity::conversation::Model>(
        &mut tx,
        &export_entity::conversation::Entity::find()
            .filter(export_entity::conversation::Column::UserId.eq(context.account_id))
            .order_by_asc(export_entity::conversation::Column::CreatedAt)
            .order_by_asc(export_entity::conversation::Column::Id)
            .into_query(),
    )
    .await?;

    let cmd_rows = orm::models::<export_entity::command::Model>(
        &mut tx,
        &export_entity::command::Entity::find()
            .filter(export_entity::command::Column::UserId.eq(context.account_id))
            .order_by_asc(export_entity::command::Column::CreatedAt)
            .order_by_asc(export_entity::command::Column::Id)
            .into_query(),
    )
    .await?;

    let mut turns_by_conv: HashMap<Uuid, Vec<ExportedConversationTurn>> = HashMap::new();
    for cmd in cmd_rows {
        turns_by_conv
            .entry(cmd.conversation_id)
            .or_default()
            .push(ExportedConversationTurn {
                id: cmd.id,
                input: cmd.input,
                state: cmd.state,
                reply: cmd.reply,
                created_at: cmd.created_at,
            });
    }

    let memory_row = orm::model::<export_entity::private_memory::Model>(
        &mut tx,
        &export_entity::private_memory::Entity::find()
            .filter(export_entity::private_memory::Column::UserId.eq(context.account_id))
            .into_query(),
    )
    .await?;

    let receipt_rows = orm::models::<export_entity::privacy_receipt::Model>(
        &mut tx,
        &export_entity::privacy_receipt::Entity::find()
            .filter(export_entity::privacy_receipt::Column::AccountId.eq(context.account_id))
            .order_by_asc(export_entity::privacy_receipt::Column::OccurredAt)
            .order_by_asc(export_entity::privacy_receipt::Column::Id)
            .into_query(),
    )
    .await?;

    tx.commit().await?;

    Ok(AccountExport {
        schema_version: 1,
        exported_at: Utc::now(),
        account: AccountProfileExport {
            account_id: user.id,
            email: user.email,
            display_name: user.display_name,
            appearance: user.appearance,
            sharing_label: user.sharing_label,
            terms_version: user.terms_version,
            terms_accepted_at: user.terms_accepted_at,
            ai_enabled: user.intelligence_enabled,
            ai_consent_version: user.luna_consent_version,
            ai_consent_decision: user.luna_consent_decision,
            ai_consent_at: user.luna_consent_at,
            apple_health_generation: user.apple_health_generation,
            apple_health_disconnected_at: user.apple_health_disconnected_at,
            health_records_generation: user.health_records_generation,
            health_records_disconnected_at: user.health_records_disconnected_at,
            health_connect_generation: user.health_connect_generation,
            health_connect_disconnected_at: user.health_connect_disconnected_at,
            created_at: user.created_at,
        },
        health_evidence: evidence_rows
            .into_iter()
            .map(|row| ExportedEvidenceRecord {
                revisions: evidence_revs.remove(&row.id).unwrap_or_default(),
                id: row.id,
                concept_id: row.concept_id,
                source_kind: row.source_kind,
                source_connection_id: row.source_connection_id,
                source_record_id: row.source_record_id,
                measured_at: row.measured_at,
                received_at: row.received_at,
                created_at: row.created_at,
                updated_at: row.updated_at,
                payload: row.payload,
            })
            .collect(),
        clinical_notes: note_rows
            .into_iter()
            .map(|row| ExportedClinicalNote {
                revisions: note_revs.remove(&row.id).unwrap_or_default(),
                id: row.id,
                source_label: row.source_label,
                source_id: row.source_id,
                title: row.title,
                preview: row.preview,
                body: row.body,
                recorded_at: row.recorded_at,
                created_at: row.created_at,
                source_payload: row.duet_source_payload,
            })
            .collect(),
        device_snapshots: snapshot_rows
            .into_iter()
            .map(|row| ExportedDeviceSnapshot {
                id: row.id,
                generated_at: row.generated_at,
                received_at: row.received_at,
                payload: row.payload,
            })
            .collect(),
        doctor_page: ExportedDoctorPage {
            note: user.doctor_note,
            note_generation: user.doctor_note_generation,
            page: user.doctor_page,
            page_updated_at: user.doctor_page_updated_at,
            items: doctor_items
                .into_iter()
                .map(|row| ExportedDoctorItem {
                    id: row.id,
                    kind: row.kind,
                    source_id: row.source_id,
                    title: row.title,
                    detail: row.detail,
                    evidence: row.evidence,
                    range_start: row.range_start,
                    range_end: row.range_end,
                    saved_at: row.saved_at,
                    position: row.position,
                    page_entry: row.page_entry,
                    medication_confirmed: row.medication_confirmed,
                })
                .collect(),
        },
        portal_connections: portal_rows
            .into_iter()
            .map(|row| ExportedPortalConnection {
                id: row.id,
                vendor: row.vendor,
                organization_name: row.organization_name,
                brand: row.brand,
                state: row.state,
                last_synced_at: row.last_synced_at,
                failing_since: row.failing_since,
                failure_reason: row.failure_reason,
                disconnected_at: row.disconnected_at,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect(),
        conversations: conv_rows
            .into_iter()
            .map(|row| ExportedConversation {
                turns: turns_by_conv.remove(&row.id).unwrap_or_default(),
                id: row.id,
                created_at: row.created_at,
            })
            .collect(),
        private_memory: memory_row.map(|row| ExportedPrivateMemory {
            revision: row.revision,
            content: row.content,
        }),
        privacy_receipts: receipt_rows
            .into_iter()
            .map(|row| ExportedPrivacyReceipt {
                event_id: row.id,
                operation: row.operation,
                scope_id: row.scope_id,
                occurred_at: row.occurred_at,
                journaled_at: row.journaled_at,
            })
            .collect(),
    })
}

/// Axum response with explicit `Cache-Control: no-store` for portable export downloads.
pub(crate) fn export_response(export: AccountExport) -> axum::response::Response {
    ([(header::CACHE_CONTROL, "no-store")], Json(export)).into_response()
}

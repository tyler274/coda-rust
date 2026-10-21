//! Canonical measurement producer, separate from doctor-device snapshots and model reads.
//!
//! A dedicated non-owner, RLS-bound producer connection owns writes. Its pool is
//! private; ordinary conversation/source-reader credentials remain SELECT-only.
//! Arithmetic consumes immutable evidence through EvidenceStore, never uploaded
//! measurement facts. Unsupported shapes remain visible as calculation limits.
use crate::{
    ApiError,
    database::AccountDatabase,
    evidence::{AddressedRecordRevision, EvidenceSnapshot, EvidenceStore},
    identity::{AccountContext, check_current},
};
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use duet_wire::models::{HealthFactInput, HealthFactKind, HealthWindow, InsightCycleKind};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::{Postgres, Row, Transaction};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

/// A native presentation window; only narrow native supplemental namespaces are accepted.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Projection {
    /// Daily presentation or the latest completed weekly presentation.
    pub kind: InsightCycleKind,
    /// Inclusive local-calendar boundary, expressed as an instant.
    pub window_start: DateTime<Utc>,
    /// Exclusive local-calendar boundary, expressed as an instant.
    pub window_end: DateTime<Utc>,
    /// IANA zone validated against PostgreSQL's timezone database.
    pub timezone: String,
    /// Foundation weekday numbering: 1=Sunday through 7=Saturday.
    pub first_weekday: u8,
    /// Uploads contain native supplements only; responses also contain derived facts.
    #[serde(default)]
    pub facts: Vec<HealthFactInput>,
}
/// Upload identity and capture metadata. Measurement facts are always server-derived.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    /// Caller capture identity; reused exactly for retries.
    pub id: Uuid,
    /// Native capture boundary on input, canonical provenance boundary on output.
    pub source_boundary_id: Uuid,
    /// Currently supported schema is 1.
    pub schema_version: i32,
    /// Capture time; canonical refreshes use database time.
    pub generated_at: DateTime<Utc>,
    /// IANA presentation timezone.
    pub timezone: String,
    /// Native withdrawal generations; output also includes canonical_evidence_revision.
    #[serde(default)]
    pub source_generations: BTreeMap<String, i64>,
    /// One or two unique daily/weekly projections.
    pub projections: Vec<Projection>,
}
/// Optimistic context write. The capture UUID is also the durable retry identity.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Upload {
    /// None requires no existing context; otherwise an exact optimistic revision.
    pub expected_revision: Option<i64>,
    /// Typed field of the canonical health-context contract.
    pub snapshot: Snapshot,
}
/// Exact revision provenance, not a claim that a model read the record body.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    /// Immutable canonical address retaining original source identity and revision.
    pub address: AddressedRecordRevision,
    /// Literal source family used for withdrawal provenance.
    pub source_kind: String,
    /// Digest of the exact immutable payload, not a mutable summary.
    pub payload_sha256: String,
}
/// A non-calculable source remains identifiable and available through the exact reader.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CalculationLimit {
    /// Immutable canonical address retaining original source identity and revision.
    pub address: AddressedRecordRevision,
    /// Typed field of the canonical health-context contract.
    pub reason: String,
}
/// Typed published context with an explicit completeness report.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StoredContext {
    /// Typed field of the canonical health-context contract.
    pub revision: i64,
    /// Typed field of the canonical health-context contract.
    pub snapshot: Snapshot,
    /// Typed field of the canonical health-context contract.
    pub dependencies: Vec<Dependency>,
    /// Typed field of the canonical health-context contract.
    pub uncalculated: Vec<CalculationLimit>,
}
/// Prepared work cannot be constructed by callers or replaced with trusted JSON.
pub struct PreparedContext {
    account: Uuid,
    incarnation: Uuid,
    expected_revision: Option<i64>,
    evidence_revision: i64,
    apple_generation: i32,
    records_generation: i32,
    input: Snapshot,
    snapshot: Snapshot,
    dependencies: Vec<Dependency>,
    limits: Vec<CalculationLimit>,
    operation: Option<(Uuid, String)>,
}
/// Private write capability over a separately provisioned non-owner connection.
#[derive(Clone)]
pub struct HealthContextProducer {
    database: AccountDatabase,
}
fn digest(value: &impl Serialize) -> String {
    Sha256::digest(serde_json::to_vec(value).expect("serializable producer value"))
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn stable_uuid(value: &impl Serialize) -> Uuid {
    let hash = Sha256::digest(serde_json::to_vec(value).expect("serializable identity"));
    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hash[..16]);
    bytes[6] = (bytes[6] & 15) | 0x80;
    bytes[8] = (bytes[8] & 63) | 0x80;
    Uuid::from_bytes(bytes)
}
fn invalid(message: &'static str) -> ApiError {
    ApiError::Unprocessable(message)
}
fn changed() -> ApiError {
    ApiError::Conflict("health context sources changed; prepare again")
}

impl HealthContextProducer {
    /// Connect only with separately scoped producer credentials, never migration credentials.
    /// Typed field of the canonical health-context contract.
    pub async fn connect(url: &str) -> Result<Self, ApiError> {
        let database = AccountDatabase::connect(url).await?;
        let mut tx = database.begin(Uuid::nil()).await?;
        let allowed: bool = sqlx::query_scalar("select has_table_privilege(current_user,'public.health_contexts','INSERT') and has_table_privilege(current_user,'public.health_contexts','UPDATE') and has_table_privilege(current_user,'public.duet_health_context_operations','INSERT') and not has_table_privilege(current_user,'public.insight_cycles','INSERT') and not has_table_privilege(current_user,'public.follow_ups','INSERT')")
            .fetch_one(&mut *tx).await?;
        tx.rollback().await?;
        if !allowed {
            return Err(ApiError::Unavailable(
                "dedicated health context producer grants required",
            ));
        }
        Ok(Self { database })
    }

    /// Derive outside the publication lock, preserving an exact source/current-context fence.
    /// Typed field of the canonical health-context contract.
    pub async fn prepare_upload(
        &self,
        context: &AccountContext,
        upload: &Upload,
    ) -> Result<PreparedContext, ApiError> {
        validate_snapshot(&upload.snapshot)?;
        let operation = Some((upload.snapshot.id, digest(upload)));
        self.prepare(
            context,
            upload.snapshot.clone(),
            upload.expected_revision,
            operation,
            false,
        )
        .await
    }

    /// Retry reconciles the original operation; it never reapplies an older capture over a newer one.
    /// Typed field of the canonical health-context contract.
    pub async fn upload(
        &self,
        context: &AccountContext,
        upload: &Upload,
    ) -> Result<StoredContext, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        if let Some(row) = sqlx::query("select request_sha256,snapshot_id,revision from duet_health_context_operations where user_id=$1 and operation_id=$2")
            .bind(context.account_id).bind(upload.snapshot.id).fetch_optional(&mut *tx).await? {
            if row.get::<String,_>("request_sha256") != digest(upload) { return Err(ApiError::Conflict("health context identity reused")); }
            let stored = load(&mut tx, context.account_id).await?.ok_or_else(changed)?;
            if stored.revision != row.get::<i64,_>("revision") || stored.snapshot.id != row.get::<Uuid,_>("snapshot_id") { return Err(changed()); }
            current(&mut tx, context.account_id, &stored).await?;
            return Ok(stored);
        }
        tx.rollback().await?;
        let prepared = self.prepare_upload(context, upload).await?;
        self.publish(context, prepared).await
    }

    /// Rebuild/roll the retained capture using current canonical evidence. Does not require AI consent.
    /// Typed field of the canonical health-context contract.
    pub async fn refresh(
        &self,
        context: &AccountContext,
    ) -> Result<Option<StoredContext>, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let row =
            sqlx::query("select revision,producer_input from health_contexts where user_id=$1")
                .bind(context.account_id)
                .fetch_optional(&mut *tx)
                .await?;
        let Some(row) = row else {
            return Ok(None);
        };
        let input: Option<Value> = row.get("producer_input");
        let Some(input) = input else {
            return Err(ApiError::Conflict(
                "retained context requires fresh producer upload",
            ));
        };
        let snapshot = serde_json::from_value(input)
            .map_err(|_| ApiError::Storage("invalid producer input".into()))?;
        let revision = row.get("revision");
        tx.rollback().await?;
        let prepared = self
            .prepare(context, snapshot, Some(revision), None, true)
            .await?;
        Ok(Some(self.publish(context, prepared).await?))
    }

    /// Read is side-effect free and refuses stale evidence/withdrawal generations.
    /// Typed field of the canonical health-context contract.
    pub async fn read(&self, context: &AccountContext) -> Result<Option<StoredContext>, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let stored = load(&mut tx, context.account_id).await?;
        if let Some(ref stored) = stored {
            current(&mut tx, context.account_id, stored).await?;
        }
        tx.commit().await?;
        Ok(stored)
    }

    async fn prepare(
        &self,
        context: &AccountContext,
        mut input: Snapshot,
        expected_revision: Option<i64>,
        operation: Option<(Uuid, String)>,
        roll: bool,
    ) -> Result<PreparedContext, ApiError> {
        // This is the existing evidence owner: exact current immutable bodies, not health_evidence summaries.
        let evidence = EvidenceStore::new(self.database.clone())
            .snapshot(context, 0)
            .await?;
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let (cursor, apple, records): (i64,i32,i32) = sqlx::query_as("select evidence_revision,apple_health_generation,health_records_generation from users where id=$1")
            .bind(context.account_id).fetch_one(&mut *tx).await?;
        if cursor != evidence.cursor || records != evidence.health_records_generation {
            return Err(changed());
        }
        valid_zone(&mut tx, &input.timezone).await?;
        for projection in &input.projections {
            let start_day = local_day(&mut tx, projection.window_start, &input.timezone).await?;
            let span = if projection.kind == InsightCycleKind::Daily {
                1
            } else {
                7
            };
            if projection.window_start != midnight(&mut tx, start_day, &input.timezone).await?
                || projection.window_end
                    != midnight(&mut tx, start_day + Duration::days(span), &input.timezone).await?
                || (projection.kind == InsightCycleKind::Weekly
                    && week_start(start_day, projection.first_weekday) != start_day)
            {
                return Err(invalid(
                    "projection must name a complete local day or aligned week",
                ));
            }
        }
        let now: DateTime<Utc> = sqlx::query_scalar("select clock_timestamp()")
            .fetch_one(&mut *tx)
            .await?;
        let stale_native = input
            .source_generations
            .get("apple_health_generation")
            .copied()
            .unwrap_or(0)
            != i64::from(apple);
        let stale_records = input
            .source_generations
            .get("health_records_generation")
            .copied()
            .unwrap_or(0)
            != i64::from(records);
        if !roll && (stale_native || stale_records) {
            return Err(changed());
        }
        if roll && stale_native {
            // A withdrawal is not a disconnect. Never relabel old native facts with a new generation.
            for projection in &mut input.projections {
                projection.facts.retain(|fact| {
                    !fact.evidence.iter().any(|e| {
                        e.source_kind.as_ref().and_then(|v| v.as_deref()) == Some("appleHealth")
                    })
                });
            }
        }
        if roll && stale_records {
            for projection in &mut input.projections {
                projection.facts.retain(|fact| {
                    !fact.evidence.iter().any(|e| {
                        e.source_kind.as_ref().and_then(|v| v.as_deref()) == Some("healthRecords")
                    })
                });
            }
        }
        if roll {
            for projection in &mut input.projections {
                let day = local_day(&mut tx, now, &input.timezone).await?;
                let (start, end) = if projection.kind == InsightCycleKind::Daily {
                    (day, day + Duration::days(1))
                } else {
                    let end = week_start(day, projection.first_weekday);
                    (end - Duration::days(7), end)
                };
                let changed_window = projection.window_start
                    != midnight(&mut tx, start, &input.timezone).await?
                    || projection.window_end != midnight(&mut tx, end, &input.timezone).await?;
                projection.window_start = midnight(&mut tx, start, &input.timezone).await?;
                projection.window_end = midnight(&mut tx, end, &input.timezone).await?;
                // Native facts are captured for a window. Do not move stale workout/profile summaries into a new one.
                if changed_window {
                    projection.facts.clear();
                }
            }
        }
        let deleted: Vec<Uuid> = sqlx::query_scalar("select scope_id from privacy_recovery_events where account_id=$1 and operation='evidence_delete'").bind(context.account_id).fetch_all(&mut *tx).await?;
        for projection in &mut input.projections {
            if !roll
                && projection
                    .facts
                    .iter()
                    .any(|f| references_deleted(&json!(f), &deleted))
            {
                return Err(changed());
            }
            projection
                .facts
                .retain(|f| !references_deleted(&json!(f), &deleted));
        }
        input
            .source_generations
            .insert("apple_health_generation".into(), i64::from(apple));
        input
            .source_generations
            .insert("health_records_generation".into(), i64::from(records));
        let (series, dependencies, limits) =
            prepare_series(&mut tx, &evidence, &input.timezone, now).await?;
        let mut snapshot = input.clone();
        for projection in &mut snapshot.projections {
            let reference = if projection.kind == InsightCycleKind::Weekly {
                projection.window_end
            } else {
                now
            };
            for item in &series {
                projection
                    .facts
                    .extend(derive(&mut tx, item, projection, reference).await?);
            }
            projection.facts.sort_by(|a, b| a.id.cmp(&b.id));
        }
        let count: usize = snapshot.projections.iter().map(|p| p.facts.len()).sum();
        if count > 2048 {
            return Err(ApiError::Unavailable(
                "health context exceeds bounded fact envelope",
            ));
        }
        snapshot
            .source_generations
            .insert("canonical_evidence_revision".into(), cursor);
        snapshot.source_boundary_id = stable_uuid(&json!([
            "duet-canonical-context-v1",
            context.account_id,
            context.incarnation,
            input.source_boundary_id,
            cursor,
            apple,
            records
        ]));
        if roll {
            snapshot.generated_at = now;
            snapshot.id = stable_uuid(&json!([
                "duet-context-refresh-v1",
                snapshot.source_boundary_id,
                snapshot.projections
            ]));
        }
        tx.commit().await?;
        Ok(PreparedContext {
            account: context.account_id,
            incarnation: context.incarnation,
            expected_revision,
            evidence_revision: cursor,
            apple_generation: apple,
            records_generation: records,
            input,
            snapshot,
            dependencies,
            limits,
            operation,
        })
    }

    /// Atomically publish only over the exact evidence and context revisions prepared above.
    /// Typed field of the canonical health-context contract.
    pub async fn publish(
        &self,
        context: &AccountContext,
        prepared: PreparedContext,
    ) -> Result<StoredContext, ApiError> {
        if prepared.account != context.account_id || prepared.incarnation != context.incarnation {
            return Err(ApiError::Unauthorized);
        }
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let (cursor, apple, records): (i64,i32,i32) = sqlx::query_as("select evidence_revision,apple_health_generation,health_records_generation from users where id=$1")
            .bind(context.account_id).fetch_one(&mut *tx).await?;
        if (cursor, apple, records)
            != (
                prepared.evidence_revision,
                prepared.apple_generation,
                prepared.records_generation,
            )
        {
            return Err(changed());
        }
        let prior = load(&mut tx, context.account_id).await?;
        if let Some((operation, request)) = &prepared.operation
            && let Some(row) = sqlx::query("select request_sha256,snapshot_id,revision from duet_health_context_operations where user_id=$1 and operation_id=$2")
                .bind(context.account_id).bind(operation).fetch_optional(&mut *tx).await? {
                if row.get::<String,_>("request_sha256") != *request { return Err(ApiError::Conflict("health context identity reused")); }
                let stored = prior.ok_or_else(changed)?;
                if stored.snapshot.id != row.get::<Uuid,_>("snapshot_id") || stored.revision != row.get::<i64,_>("revision") { return Err(changed()); }
                current(&mut tx,context.account_id,&stored).await?;
                return Ok(stored);
        }
        if prior.as_ref().map(|v| v.revision) != prepared.expected_revision {
            return Err(changed());
        }
        let fingerprint = digest(&json!([
            prepared.snapshot.source_boundary_id,
            prepared.snapshot.schema_version,
            prepared.snapshot.timezone,
            prepared.snapshot.source_generations,
            prepared.snapshot.projections,
            prepared.dependencies,
            prepared.limits
        ]));
        let old_fingerprint: Option<String> =
            sqlx::query_scalar("select producer_fingerprint from health_contexts where user_id=$1")
                .bind(context.account_id)
                .fetch_optional(&mut *tx)
                .await?
                .flatten();
        let stored = if old_fingerprint.as_deref() == Some(&fingerprint) {
            prior.ok_or_else(changed)?
        } else {
            if prior
                .as_ref()
                .is_some_and(|p| p.snapshot.id == prepared.snapshot.id)
            {
                return Err(ApiError::Conflict(
                    "health context identity reused for changed content",
                ));
            }
            let revision = prior.as_ref().map_or(1, |p| p.revision + 1);
            let snapshot = prepared.snapshot;
            let mut facts = BTreeMap::new();
            let projections: Vec<Value> = snapshot.projections.iter().map(|p| {
                for fact in &p.facts { facts.insert(fact.id.clone(),fact.clone()); }
                json!({"kind":p.kind,"window_start":p.window_start,"window_end":p.window_end,"timezone":p.timezone,"first_weekday":p.first_weekday,"fact_ids":p.facts.iter().map(|f|&f.id).collect::<Vec<_>>()})
            }).collect();
            sqlx::query("insert into health_contexts(user_id,snapshot_id,source_boundary_id,schema_version,revision,generated_at,timezone,source_generations,facts,projections,producer_input,producer_dependencies,producer_limits,producer_fingerprint) values($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14) on conflict(user_id) do update set snapshot_id=excluded.snapshot_id,source_boundary_id=excluded.source_boundary_id,schema_version=excluded.schema_version,revision=excluded.revision,generated_at=excluded.generated_at,timezone=excluded.timezone,source_generations=excluded.source_generations,facts=excluded.facts,projections=excluded.projections,producer_input=excluded.producer_input,producer_dependencies=excluded.producer_dependencies,producer_limits=excluded.producer_limits,producer_fingerprint=excluded.producer_fingerprint,updated_at=now()")
                .bind(context.account_id).bind(snapshot.id).bind(snapshot.source_boundary_id).bind(snapshot.schema_version).bind(revision).bind(snapshot.generated_at).bind(&snapshot.timezone).bind(json!(snapshot.source_generations)).bind(json!(facts.values().collect::<Vec<_>>())).bind(json!(projections)).bind(json!(prepared.input)).bind(json!(prepared.dependencies)).bind(json!(prepared.limits)).bind(fingerprint).execute(&mut *tx).await?;
            StoredContext {
                revision,
                snapshot,
                dependencies: prepared.dependencies,
                uncalculated: prepared.limits,
            }
        };
        if let Some((operation, request)) = prepared.operation {
            sqlx::query("insert into duet_health_context_operations(user_id,operation_id,request_sha256,snapshot_id,revision) values($1,$2,$3,$4,$5)")
                .bind(context.account_id).bind(operation).bind(request).bind(stored.snapshot.id).bind(stored.revision).execute(&mut *tx).await?;
        }
        // Return the persisted representation (PostgreSQL timestamps have microsecond
        // precision and JSON decoding must match every retry/restart readback).
        let stored = load(&mut tx, context.account_id)
            .await?
            .ok_or_else(changed)?;
        tx.commit().await?;
        Ok(stored)
    }
}

fn validate_snapshot(s: &Snapshot) -> Result<(), ApiError> {
    if s.schema_version != 1
        || s.projections.is_empty()
        || s.projections.len() > 2
        || s.timezone.is_empty()
        || s.timezone.len() > 80
        || s.source_generations.values().any(|v| *v < 0)
        || serde_json::to_vec(s)
            .map_err(|_| invalid("invalid snapshot"))?
            .len()
            > 8 * 1024 * 1024
    {
        return Err(invalid("invalid health context envelope"));
    }
    let mut kinds = BTreeSet::new();
    let mut ids = BTreeMap::new();
    for p in &s.projections {
        if p.timezone != s.timezone
            || !(1..=7).contains(&p.first_weekday)
            || p.window_end <= p.window_start
            || !kinds.insert(format!("{:?}", p.kind))
            || p.facts.len() > 1024
        {
            return Err(invalid("invalid projection window"));
        }
        let mut projection_ids = BTreeSet::new();
        for f in &p.facts {
            if !projection_ids.insert(&f.id) || ids.insert(&f.id, f).is_some_and(|old| old != f) {
                return Err(invalid("conflicting native fact identity"));
            }
            if f.id.len() < 8
                || f.id.len() > 200
                || f.title.is_empty()
                || f.title.len() > 640
                || f.range_end < f.range_start
                || f.concept_ids.is_empty()
                || f.concept_ids.len() > 8
                || f.evidence.is_empty()
                || f.evidence.len() > 16
            {
                return Err(invalid("invalid native supplemental fact"));
            }
            // No client measurement JSON, exact-record claims, or canonical owner spoofing.
            let body = json!(f);
            if !body["evidence"].as_array().unwrap().iter().all(|e| {
                let kind = e["kind"].as_str().unwrap_or("");
                let prefix = e["id"]
                    .as_str()
                    .unwrap_or("")
                    .split(':')
                    .next()
                    .unwrap_or("");
                let permitted = matches!(kind, "workout" | "profile")
                    || (e["source_kind"] == "appleHealth"
                        && matches!(prefix, "ecg" | "audiogram" | "specialized"));
                permitted
                    && !e["source_fields"].as_array().is_some_and(|fields| {
                        fields.iter().any(|v| v["name"] == "projection_owner")
                    })
            }) {
                return Err(invalid(
                    "only workout, profile and Apple specialized native supplements are accepted",
                ));
            }
        }
    }
    Ok(())
}
async fn valid_zone(tx: &mut Transaction<'_, Postgres>, zone: &str) -> Result<(), ApiError> {
    let exists: bool =
        sqlx::query_scalar("select exists(select 1 from pg_timezone_names where name=$1)")
            .bind(zone)
            .fetch_one(&mut **tx)
            .await?;
    if !exists {
        return Err(invalid("invalid health context timezone"));
    }
    Ok(())
}
async fn local_day(
    tx: &mut Transaction<'_, Postgres>,
    at: DateTime<Utc>,
    zone: &str,
) -> Result<NaiveDate, ApiError> {
    Ok(
        sqlx::query_scalar("select ($1::timestamptz at time zone $2)::date")
            .bind(at)
            .bind(zone)
            .fetch_one(&mut **tx)
            .await?,
    )
}
async fn midnight(
    tx: &mut Transaction<'_, Postgres>,
    day: NaiveDate,
    zone: &str,
) -> Result<DateTime<Utc>, ApiError> {
    Ok(
        sqlx::query_scalar("select $1::date::timestamp at time zone $2")
            .bind(day)
            .bind(zone)
            .fetch_one(&mut **tx)
            .await?,
    )
}
fn week_start(day: NaiveDate, weekday: u8) -> NaiveDate {
    day - Duration::days(
        (i64::from(day.weekday().num_days_from_sunday()) - i64::from(weekday - 1)).rem_euclid(7),
    )
}
/// Same-transaction freshness check for canonical-context consumers. Call only
/// after the account lock; never substitute a doctor-device generation.
pub async fn validate_current_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
) -> Result<(), ApiError> {
    let stored = load(tx, account).await?.ok_or(ApiError::NotFound)?;
    current(tx, account, &stored).await
}

fn references_deleted(value: &Value, ids: &[Uuid]) -> bool {
    match value {
        Value::Object(map) => {
            ["id", "record_id", "canonical_record_id"]
                .iter()
                .any(|key| {
                    map.get(*key)
                        .and_then(Value::as_str)
                        .is_some_and(|s| ids.iter().any(|id| s == id.to_string()))
                })
                || map.values().any(|v| references_deleted(v, ids))
        }
        Value::Array(items) => items.iter().any(|v| references_deleted(v, ids)),
        _ => false,
    }
}

async fn current(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    stored: &StoredContext,
) -> Result<(), ApiError> {
    let (cursor,apple,records): (i64,i32,i32) = sqlx::query_as("select evidence_revision,apple_health_generation,health_records_generation from users where id=$1").bind(account).fetch_one(&mut **tx).await?;
    let deleted: Vec<Uuid> = sqlx::query_scalar("select scope_id from privacy_recovery_events where account_id=$1 and operation='evidence_delete'").bind(account).fetch_all(&mut **tx).await?;
    if references_deleted(&json!(stored.snapshot.projections), &deleted) {
        return Err(changed());
    }
    let g = &stored.snapshot.source_generations;
    if g.get("canonical_evidence_revision") != Some(&cursor)
        || g.get("apple_health_generation") != Some(&i64::from(apple))
        || g.get("health_records_generation") != Some(&i64::from(records))
    {
        return Err(changed());
    }
    // Exact reads enforce deletion visibility even after restore. Receipt replay must complete before serving.
    for dependency in &stored.dependencies {
        let exact =
            crate::evidence::exact_in_transaction(tx, account, &dependency.address, true).await?;
        if digest(&exact.payload) != dependency.payload_sha256 {
            return Err(changed());
        }
    }
    Ok(())
}
async fn load(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
) -> Result<Option<StoredContext>, ApiError> {
    let Some(row) = sqlx::query("select * from health_contexts where user_id=$1")
        .bind(account)
        .fetch_optional(&mut **tx)
        .await?
    else {
        return Ok(None);
    };
    let facts: Vec<HealthFactInput> = serde_json::from_value(row.get("facts"))
        .map_err(|_| ApiError::Storage("invalid retained health facts".into()))?;
    let values: Vec<Value> = serde_json::from_value(row.get("projections"))
        .map_err(|_| ApiError::Storage("invalid retained projections".into()))?;
    let mut projections = Vec::new();
    for mut value in values {
        let ids = value
            .as_object_mut()
            .and_then(|v| v.remove("fact_ids"))
            .ok_or_else(|| ApiError::Storage("invalid retained projection membership".into()))?;
        let ids: Vec<String> = serde_json::from_value(ids)
            .map_err(|_| ApiError::Storage("invalid fact membership".into()))?;
        let selected: Result<Vec<_>, _> = ids
            .iter()
            .map(|id| {
                facts
                    .iter()
                    .find(|f| &f.id == id)
                    .cloned()
                    .ok_or_else(|| ApiError::Storage("missing retained fact".into()))
            })
            .collect();
        value["facts"] = json!(selected?);
        projections.push(
            serde_json::from_value(value)
                .map_err(|_| ApiError::Storage("invalid retained projection".into()))?,
        );
    }
    Ok(Some(StoredContext {
        revision: row.get("revision"),
        snapshot: Snapshot {
            id: row.get("snapshot_id"),
            source_boundary_id: row.get("source_boundary_id"),
            schema_version: row.get("schema_version"),
            generated_at: row.get("generated_at"),
            timezone: row.get("timezone"),
            source_generations: serde_json::from_value(row.get("source_generations"))
                .map_err(|_| ApiError::Storage("invalid source generations".into()))?,
            projections,
        },
        dependencies: serde_json::from_value(row.get("producer_dependencies"))
            .map_err(|_| ApiError::Storage("invalid dependencies".into()))?,
        uncalculated: serde_json::from_value(row.get("producer_limits"))
            .map_err(|_| ApiError::Storage("invalid calculation limits".into()))?,
    }))
}

#[derive(Clone)]
struct Reading {
    day: NaiveDate,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    readings: Vec<(DateTime<Utc>, f64)>,
    available: DateTime<Utc>,
    value: f64,
    partial: bool,
    aggregate: bool,
    address: AddressedRecordRevision,
}
struct Series {
    key: String,
    concept: String,
    title: String,
    unit: String,
    kind: String,
    connection: Option<String>,
    name: String,
    aggregation: String,
    sensitivity: String,
    points: Vec<Reading>,
}
fn str_at<'a>(v: &'a Value, path: &str) -> &'a str {
    v.pointer(path).and_then(Value::as_str).unwrap_or("")
}
fn timestamp(s: &str) -> Option<DateTime<Utc>> {
    s.parse().ok()
}
fn limit(p: &Value) -> Option<&'static str> {
    if p["verification"] == "staged" {
        return Some("Unconfirmed staged evidence");
    }
    if matches!(
        str_at(p, "/source/statuses/status"),
        "registered" | "cancelled" | "entered-in-error"
    ) {
        return Some("Source declares no current result");
    }
    if p["source_record"]["format"] == "fhir-json" {
        let content = &p["source_record"]["content"];
        let decoded = content
            .as_str()
            .and_then(|s| serde_json::from_str::<Value>(s).ok());
        let resource = decoded.as_ref().unwrap_or(content);
        if resource["resourceType"] == "Observation"
            && matches!(
                resource["status"].as_str(),
                Some("registered" | "cancelled" | "entered-in-error")
            )
        {
            return Some("Source declares no current result");
        }
    }
    if p["catalog"]["resolution"] == "conflict" {
        return Some("Unresolved measurement meaning");
    }
    if p["source"]["provenance"] == "copied" && str_at(p, "/source/issuer").is_empty() {
        return Some("Copied measurement has unknown original source");
    }
    if p["value"]["type"] != "quantity" {
        return Some(
            "Only exact scalar quantities are implemented; components/ranges/ratios/text require their exact reader",
        );
    }
    if !p["value"]["quantity"]["comparator"].is_null() {
        return Some("Bound is not an exact value");
    }
    if p["value"]["quantity"]["canonical_value"]
        .as_f64()
        .is_none_or(|v| !v.is_finite())
        || str_at(p, "/value/quantity/canonical_unit").is_empty()
    {
        return Some("Exact canonical quantity and established unit required");
    }
    let aggregation = str_at(p, "/context/aggregation");
    if (p["method"] == "daily_aggregate" || !aggregation.is_empty())
        && !matches!(
            aggregation,
            "daily_sum"
                | "daily_event_count"
                | "latest_reading_in_day"
                | "mean_of_readings_in_day"
                | "asleep_minutes_ending_in_day"
        )
    {
        return Some("Unestablished daily aggregation");
    }
    if str_at(p, "/concept_id").is_empty() || str_at(p, "/display_name").is_empty() {
        return Some("Measurement identity and display name required");
    }
    None
}
async fn prepare_series(
    tx: &mut Transaction<'_, Postgres>,
    evidence: &EvidenceSnapshot,
    fallback: &str,
    now: DateTime<Utc>,
) -> Result<(Vec<Series>, Vec<Dependency>, Vec<CalculationLimit>), ApiError> {
    let corrections: BTreeSet<String> = evidence
        .records
        .iter()
        .filter(|r| r.payload["verification"] != "staged")
        .filter_map(|r| {
            r.payload["corrects_evidence_id"]
                .as_str()
                .map(|s| s.to_ascii_lowercase())
        })
        .collect();
    let mut grouped: BTreeMap<String, Series> = BTreeMap::new();
    let mut dependencies = Vec::new();
    let mut limits = Vec::new();
    for record in &evidence.records {
        if record.address.entity
            != duet_wire::models::addressed_record_revision::Entity::HealthEvidence
        {
            continue;
        }
        let p = &record.payload;
        if corrections.contains(&record.address.canonical_record_id.to_string())
            || p["id"]
                .as_str()
                .is_some_and(|id| corrections.contains(&id.to_ascii_lowercase()))
        {
            continue;
        }
        if let Some(reason) = limit(p) {
            limits.push(CalculationLimit {
                address: record.address.clone(),
                reason: reason.into(),
            });
            continue;
        }
        let Some(at) = timestamp(str_at(p, "/measured_interval/start")) else {
            limits.push(CalculationLimit {
                address: record.address.clone(),
                reason: "Explicit measured_interval.start required".into(),
            });
            continue;
        };
        if at < now - Duration::days(402) || at >= now + Duration::days(1) {
            continue;
        }
        let source_kind = str_at(p, "/source/kind");
        let mut issuer = str_at(p, "/source/issuer").to_owned();
        if source_kind == "healthRecords" {
            let resource = str_at(p, "/source/resource_type");
            let upstream = str_at(p, "/source/upstream_record_id");
            if !resource.is_empty()
                && !upstream.is_empty()
                && !resource.contains('/')
                && !upstream.contains('/')
            {
                let suffix = format!("/{resource}/{upstream}");
                if issuer.ends_with(&suffix) {
                    issuer.truncate(issuer.len() - suffix.len());
                }
            }
        }
        let connection = p["source"]["connection_id"].as_str().map(str::to_owned);
        let concept = str_at(p, "/concept_id");
        let unit = str_at(p, "/value/quantity/canonical_unit");
        let identity = if p["catalog"]["resolution"] == "resolved"
            && !str_at(p, "/catalog/series_id").is_empty()
        {
            format!(
                "catalog|{}|{}",
                str_at(p, "/catalog/release_id"),
                str_at(p, "/catalog/series_id")
            )
        } else {
            format!("concept|{concept}")
        };
        let key = format!(
            "{source_kind}|{}|{issuer}|{identity}|{unit}",
            connection.as_deref().unwrap_or("")
        );
        let zone = str_at(p, "/context/day_timezone");
        let valid: bool =
            sqlx::query_scalar("select exists(select 1 from pg_timezone_names where name=$1)")
                .bind(zone)
                .fetch_one(&mut **tx)
                .await?;
        let zone = if valid { zone } else { fallback };
        let role = str_at(p, "/source/projected_date_role");
        let declared = if let Some(base) = role.strip_suffix(".start") {
            p["source"]["declared_dates"][base]["start"].as_str()
        } else {
            p["source"]["declared_dates"][role].as_str()
        };
        let day = match declared
            .filter(|s| s.len() == 10)
            .and_then(|s| s.parse::<NaiveDate>().ok())
        {
            Some(d) => d,
            None => local_day(tx, at, zone).await?,
        };
        let start = midnight(tx, day, zone).await?;
        let end = midnight(tx, day + Duration::days(1), zone).await?;
        let reading = Reading {
            day,
            start,
            end,
            readings: if p["method"] == "daily_aggregate" {
                vec![]
            } else {
                vec![(
                    at,
                    p["value"]["quantity"]["canonical_value"].as_f64().unwrap(),
                )]
            },
            available: timestamp(str_at(p, "/source_synced_at"))
                .or_else(|| timestamp(str_at(p, "/received_at")))
                .unwrap_or(at),
            value: p["value"]["quantity"]["canonical_value"].as_f64().unwrap(),
            partial: p["context"]["day_partial"] == true
                || str_at(p, "/context/day_partial").eq_ignore_ascii_case("true"),
            aggregate: p["method"] == "daily_aggregate",
            address: record.address.clone(),
        };
        let sensitivity = str_at(p, "/context/sensitivity");
        let sensitivity = if matches!(
            sensitivity,
            "standard" | "reproductive" | "symptom" | "mental_health" | "medication"
        ) {
            sensitivity
        } else if matches!(
            concept,
            "menstrual_flow"
                | "cervical_mucus"
                | "ovulation_tests"
                | "pregnancy_tests"
                | "progesterone_tests"
                | "contraceptive_use"
                | "intermenstrual_bleeding"
                | "persistent_intermenstrual_bleeding"
                | "irregular_cycles"
                | "infrequent_cycles"
                | "prolonged_periods"
                | "pregnancy"
                | "lactation"
                | "bleeding_during_pregnancy"
                | "bleeding_after_pregnancy"
                | "sexual_activity"
        ) {
            "reproductive"
        } else if concept.starts_with("symptom_") {
            "symptom"
        } else if concept == "mindful" {
            "mental_health"
        } else {
            "standard"
        };
        let series = grouped
            .entry(format!("{concept}|{key}"))
            .or_insert_with(|| Series {
                key,
                concept: concept.into(),
                title: str_at(p, "/display_name").into(),
                unit: unit.into(),
                kind: source_kind.into(),
                connection,
                name: p["context"]["source_name"]
                    .as_str()
                    .or_else(|| p["source"]["label"].as_str())
                    .unwrap_or(source_kind)
                    .into(),
                aggregation: str_at(p, "/context/aggregation").into(),
                sensitivity: sensitivity.into(),
                points: vec![],
            });
        series.points.push(reading);
        dependencies.push(Dependency {
            address: record.address.clone(),
            source_kind: source_kind.into(),
            payload_sha256: digest(p),
        });
    }
    for series in grouped.values_mut() {
        let mut days: BTreeMap<NaiveDate, Vec<Reading>> = BTreeMap::new();
        for point in series.points.drain(..) {
            days.entry(point.day).or_default().push(point);
        }
        for (_, mut readings) in days {
            readings.sort_by_key(|p| (p.available, p.address.canonical_record_id));
            let raw: Vec<_> = readings.iter().filter(|p| !p.aggregate).collect();
            let mut daily = readings.last().unwrap().clone();
            if !raw.is_empty() {
                daily.value = raw.iter().map(|p| p.value).sum::<f64>() / raw.len() as f64;
                daily.aggregate = false;
                daily.readings = raw
                    .iter()
                    .flat_map(|p| p.readings.iter().copied())
                    .collect();
                daily.readings.sort_by_key(|p| p.0);
            }
            if !daily.value.is_finite() {
                return Err(ApiError::Unavailable(
                    "daily arithmetic exceeds finite numeric range",
                ));
            }
            daily.partial = readings.iter().any(|p| p.partial);
            series.points.push(daily);
        }
    }
    dependencies.sort_by_key(|d| d.address.canonical_record_id);
    limits.sort_by_key(|d| d.address.canonical_record_id);
    Ok((grouped.into_values().collect(), dependencies, limits))
}

// Each argument is an explicit source-owned or arithmetic field of the fact.
#[allow(clippy::too_many_arguments)]
fn fact(
    s: &Series,
    kind: HealthFactKind,
    window: HealthWindow,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    value: f64,
    comparison: Option<f64>,
    points: &[&Reading],
    method: &str,
    extras: Vec<(&str, String)>,
) -> HealthFactInput {
    let mut fields = vec![("projection_owner","canonical_daily_evidence".to_owned()),("source_id",s.key.clone()),("source_name",s.name.clone()),("calculation",method.into()),("input_granularity","canonical daily source values; not raw samples".into()),("observed_day_count",points.len().to_string()),("coverage","Available complete recorded days only for longitudinal calculations; missing days are not zero.".into())];
    if let Some(first) = points.iter().map(|p| p.day).min() {
        fields.push(("first_available_day", first.to_string()));
    }
    if let Some(last) = points.iter().map(|p| p.day).max() {
        fields.push(("last_available_day", last.to_string()));
    }
    if points.len() == 1 && !points[0].readings.is_empty() {
        fields.push((
            "readings",
            points[0]
                .readings
                .iter()
                .map(|(at, value)| format!("{}: {} {}", at.to_rfc3339(), value, s.unit))
                .collect::<Vec<_>>()
                .join("; "),
        ));
        fields.push((
            "day_value",
            "average of every reading taken that day".into(),
        ));
    }
    fields.extend(extras);
    let mut f: HealthFactInput = serde_json::from_value(json!({"id":"pending-id","kind":kind,"concept_ids":[s.concept],"title":s.title,"window":window,"range_start":start,"range_end":end,"value":value,"unit":s.unit,"comparison_value":comparison,"support_count":points.len(),"sensitivity":s.sensitivity,"evidence":[{"id":format!("stat:{}:{}",s.concept,&digest(&s.key)[..12]),"kind":"stat","title":s.title,"concept_id":s.concept,"window":window,"range_start":start,"range_end":end,"source_kind":s.kind,"source_connection_ids":s.connection.iter().collect::<Vec<_>>(),"source_fields":fields.into_iter().map(|(name,value)|json!({"name":name,"value":value})).collect::<Vec<_>>()}]})).expect("typed producer fact");
    f.id = format!("fact:{}", digest(&f));
    f
}
fn delta(f: &mut HealthFactInput, value: f64, baseline: f64, unit: &str) {
    f.absolute_delta = Some(Some(value - baseline));
    let percent = (value - baseline) / baseline.abs() * 100.;
    if unit != "Cel" && baseline != 0. && percent.is_finite() {
        f.percent_delta = Some(Some(percent));
    }
}
async fn derive(
    tx: &mut Transaction<'_, Postgres>,
    s: &Series,
    projection: &Projection,
    reference: DateTime<Utc>,
) -> Result<Vec<HealthFactInput>, ApiError> {
    let points: Vec<_> = s.points.iter().filter(|p| p.start <= reference).collect();
    let Some(latest) = points.last() else {
        return Ok(vec![]);
    };
    let mut output = vec![fact(
        s,
        HealthFactKind::CurrentValue,
        HealthWindow::Day,
        latest.start,
        latest.end,
        latest.value,
        None,
        &[latest],
        "latest_available_daily_value",
        vec![],
    )];
    let complete: Vec<_> = points
        .into_iter()
        .filter(|p| !p.partial && p.end <= reference)
        .collect();
    let Some(last) = complete.last() else {
        return Ok(output);
    };
    let observed = last.day + Duration::days(1);
    for window in [
        HealthWindow::Week,
        HealthWindow::Month,
        HealthWindow::ThreeMonths,
    ] {
        let (start, prior_start) = match window {
            HealthWindow::Week => {
                let start = week_start(observed, projection.first_weekday);
                (start, start - Duration::days(7))
            }
            HealthWindow::Month => {
                let start = observed.with_day(1).unwrap();
                (
                    start,
                    start.checked_sub_months(chrono::Months::new(1)).unwrap(),
                )
            }
            _ => {
                let start = observed.checked_sub_months(chrono::Months::new(3)).unwrap();
                (
                    start,
                    start.checked_sub_months(chrono::Months::new(3)).unwrap(),
                )
            }
        };
        let prior_end = if window == HealthWindow::ThreeMonths {
            start
        } else {
            std::cmp::min(start, prior_start + (observed - start))
        };
        let cur: Vec<_> = complete
            .iter()
            .copied()
            .filter(|p| p.day >= start && p.day < observed)
            .collect();
        let prior: Vec<_> = complete
            .iter()
            .copied()
            .filter(|p| p.day >= prior_start && p.day < prior_end)
            .collect();
        if cur.is_empty() || prior.is_empty() {
            continue;
        }
        let mean = cur.iter().map(|p| p.value).sum::<f64>() / cur.len() as f64;
        let baseline = prior.iter().map(|p| p.value).sum::<f64>() / prior.len() as f64;
        let observed_points: Vec<_> = cur.iter().chain(prior.iter()).copied().collect();
        let mut f = fact(
            s,
            HealthFactKind::PeriodComparison,
            window,
            midnight(tx, start, &projection.timezone).await?,
            midnight(tx, observed, &projection.timezone).await?,
            mean,
            Some(baseline),
            &observed_points,
            "arithmetic_mean_of_available_complete_days",
            vec![
                (
                    "comparison_range_start",
                    midnight(tx, prior_start, &projection.timezone)
                        .await?
                        .to_rfc3339(),
                ),
                (
                    "comparison_range_end_exclusive",
                    midnight(tx, prior_end, &projection.timezone)
                        .await?
                        .to_rfc3339(),
                ),
            ],
        );
        delta(&mut f, mean, baseline, &s.unit);
        f.current_support_count = Some(Some(cur.len() as i32));
        f.comparison_support_count = Some(Some(prior.len() as i32));
        output.push(f);
    }
    let first = complete[0];
    let n = complete.len() as f64;
    let mean = complete.iter().map(|p| p.value).sum::<f64>() / n;
    let variance = complete
        .iter()
        .map(|p| (p.value - mean).powi(2))
        .sum::<f64>()
        / n;
    let low = complete
        .iter()
        .min_by(|a, b| a.value.total_cmp(&b.value))
        .unwrap();
    let high = complete
        .iter()
        .max_by(|a, b| a.value.total_cmp(&b.value))
        .unwrap();
    let mut extrema = fact(
        s,
        HealthFactKind::Extrema,
        HealthWindow::Year,
        first.start,
        last.end,
        high.value,
        Some(low.value),
        &complete,
        "extrema_of_available_complete_days",
        vec![
            ("one_maximum_day", high.day.to_string()),
            ("one_minimum_day", low.day.to_string()),
        ],
    );
    extrema.absolute_delta = Some(Some(high.value - low.value));
    output.push(extrema);
    let mut variability = fact(
        s,
        HealthFactKind::Variability,
        HealthWindow::Year,
        first.start,
        last.end,
        mean,
        Some(variance.sqrt()),
        &complete,
        "arithmetic_mean_and_population_standard_deviation",
        vec![],
    );
    variability.absolute_delta = Some(Some(variance.sqrt()));
    if mean != 0. && s.unit != "Cel" {
        variability.percent_delta = Some(Some(variance.sqrt() / mean.abs() * 100.));
    }
    output.push(variability);
    if complete.len() >= 2 && first.day != last.day {
        let xs: Vec<f64> = complete
            .iter()
            .map(|p| (p.day - first.day).num_days() as f64)
            .collect();
        let mx = xs.iter().sum::<f64>() / n;
        let denominator = xs.iter().map(|x| (x - mx).powi(2)).sum::<f64>();
        if denominator > 0. {
            let slope = xs
                .iter()
                .zip(&complete)
                .map(|(x, p)| (x - mx) * (p.value - mean))
                .sum::<f64>()
                / denominator;
            let intercept = mean - slope * mx;
            let end = intercept + slope * xs.last().unwrap();
            let residual = xs
                .iter()
                .zip(&complete)
                .map(|(x, p)| (p.value - (intercept + slope * x)).powi(2))
                .sum::<f64>();
            let mut trend = fact(
                s,
                HealthFactKind::Trend,
                HealthWindow::Year,
                first.start,
                last.end,
                end,
                Some(intercept),
                &complete,
                "ordinary_least_squares_by_recorded_day",
                vec![],
            );
            delta(&mut trend, end, intercept, &s.unit);
            if variance > 0. {
                trend.fit_strength = Some(Some((1. - residual / (variance * n)).clamp(0., 1.)));
            }
            output.push(trend);
        }
    }
    if s.aggregation == "daily_event_count" {
        output.push(fact(
            s,
            HealthFactKind::EventRecurrence,
            HealthWindow::Year,
            first.start,
            last.end,
            complete.iter().map(|p| p.value.max(0.)).sum(),
            None,
            &complete,
            "sum_of_available_complete_day_event_counts",
            vec![],
        ));
    }
    if projection.kind == InsightCycleKind::Weekly {
        let cur: Vec<_> = s
            .points
            .iter()
            .filter(|p| {
                !p.partial && p.start >= projection.window_start && p.end <= projection.window_end
            })
            .collect();
        let prior_start =
            local_day(tx, projection.window_start, &projection.timezone).await? - Duration::days(7);
        let prior_start = midnight(tx, prior_start, &projection.timezone).await?;
        let prior: Vec<_> = s
            .points
            .iter()
            .filter(|p| !p.partial && p.start >= prior_start && p.end <= projection.window_start)
            .collect();
        if !cur.is_empty() {
            output.retain(|f| {
                !(f.window == HealthWindow::Week
                    && matches!(
                        f.kind,
                        HealthFactKind::CurrentValue | HealthFactKind::PeriodComparison
                    ))
            });
            let mean = cur.iter().map(|p| p.value).sum::<f64>() / cur.len() as f64;
            let baseline = if prior.is_empty() {
                None
            } else {
                Some(prior.iter().map(|p| p.value).sum::<f64>() / prior.len() as f64)
            };
            let all: Vec<_> = cur.iter().chain(&prior).copied().collect();
            let mut f = fact(
                s,
                if baseline.is_some() {
                    HealthFactKind::PeriodComparison
                } else {
                    HealthFactKind::CurrentValue
                },
                HealthWindow::Week,
                projection.window_start,
                projection.window_end,
                mean,
                baseline,
                &all,
                "arithmetic_mean_of_available_complete_days",
                vec![
                    ("comparison_range_start", prior_start.to_rfc3339()),
                    (
                        "comparison_range_end_exclusive",
                        projection.window_start.to_rfc3339(),
                    ),
                ],
            );
            f.current_support_count = Some(Some(cur.len() as i32));
            if let Some(b) = baseline {
                f.comparison_support_count = Some(Some(prior.len() as i32));
                delta(&mut f, mean, b, &s.unit);
            }
            output.push(f);
        }
    }
    // Final identities include literal arithmetic and support metadata, not mutable labels alone.
    for fact in &mut output {
        if [
            &fact.value,
            &fact.comparison_value,
            &fact.absolute_delta,
            &fact.percent_delta,
            &fact.fit_strength,
        ]
        .iter()
        .any(|v| v.as_ref().and_then(|v| *v).is_some_and(|v| !v.is_finite()))
        {
            return Err(ApiError::Unavailable(
                "fact arithmetic exceeds finite numeric range",
            ));
        }
        fact.id = "canonical-fact".into();
        fact.id = format!("fact:{}", digest(fact));
    }
    Ok(output)
}

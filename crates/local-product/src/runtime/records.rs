//! Appended execution facts, separate from the product's mutable control rows.
use super::{Scope, conversion_error, engine, entities::execution_event, error};
use sea_orm::sea_query::StringLen;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use std::collections::HashMap;

/// Runtime owners have disjoint event namespaces, even for equal IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Owner {
    /// A physician conversation.
    #[sea_orm(string_value = "chat")]
    Chat,
    /// A physician reply draft revision.
    #[sea_orm(string_value = "revision")]
    Revision,
    /// A practice-owned background job.
    #[sea_orm(string_value = "practice_job")]
    PracticeJob,
}

#[derive(Clone, Copy)]
pub(super) struct OwnerKey<'a> {
    pub practice: &'a str,
    pub actor: &'a str,
    pub workspace: &'a str,
    pub target: &'a str,
    pub owner: Owner,
}

impl<'a> OwnerKey<'a> {
    fn physician(scope: &'a Scope, owner: Owner) -> Self {
        Self {
            practice: &scope.practice,
            actor: &scope.physician,
            workspace: &scope.workspace,
            target: &scope.chat,
            owner,
        }
    }
}

fn owner_filter(key: &OwnerKey<'_>) -> Condition {
    use execution_event::Column as C;
    Condition::all()
        .add(C::Practice.eq(key.practice))
        .add(C::Physician.eq(key.actor))
        .add(C::Workspace.eq(key.workspace))
        .add(C::Target.eq(key.target))
        .add(C::Owner.eq(key.owner))
}

fn count(value: u64) -> Result<i64, DbErr> {
    i64::try_from(value).map_err(conversion_error("execution identity", "i64"))
}

fn model(
    key: &OwnerKey<'_>,
    event: &engine::Event,
    admissible: bool,
) -> Result<execution_event::ActiveModel, DbErr> {
    Ok(execution_event::ActiveModel {
        practice: Set(key.practice.to_owned()),
        physician: Set(key.actor.to_owned()),
        workspace: Set(key.workspace.to_owned()),
        target: Set(key.target.to_owned()),
        owner: Set(key.owner),
        attempt: Set(event.attempt_id.clone()),
        sequence: Set(count(event.sequence)?),
        ordinal: Set(count(event.ordinal)?),
        admissible: Set(admissible),
        record: Set(serde_json::to_value(&event.kind)
            .map_err(conversion_error("EventKind", "execution event record"))?),
    })
}

pub(super) async fn append_for(
    tx: &impl ConnectionTrait,
    key: &OwnerKey<'_>,
    event: &engine::Event,
    admissible: bool,
) -> Result<(), DbErr> {
    if event.sequence == 0 {
        return Err(error("invalid event sequence"));
    }
    let row = model(key, event, admissible)?;
    let records = execution_event::Entity::find()
        .filter(owner_filter(key))
        .filter(execution_event::Column::Attempt.eq(event.attempt_id.as_str()));
    if let Some(saved) = records
        .clone()
        .filter(execution_event::Column::Sequence.eq(count(event.sequence)?))
        .one(tx)
        .await?
    {
        if saved.ordinal != count(event.ordinal)? || saved.record != *row.record.as_ref() {
            return Err(error("event collision"));
        }
        return Ok(());
    }
    let last = records
        .select_only()
        .column(execution_event::Column::Sequence)
        .order_by_desc(execution_event::Column::Sequence)
        .into_tuple::<i64>()
        .one(tx)
        .await?;
    if count(event.sequence)? != last.map_or(1, |sequence| sequence + 1) {
        return Err(error("event sequence gap"));
    }
    execution_event::Entity::insert(row)
        .exec_without_returning(tx)
        .await?;
    Ok(())
}

/// The caller holds the conversation or draft lock. An ambiguous acknowledgment
/// reuses the same identity; a different fact under that identity is an error.
pub(super) async fn append(
    tx: &impl ConnectionTrait,
    scope: &Scope,
    owner: Owner,
    event: &engine::Event,
    admissible: bool,
) -> Result<(), DbErr> {
    append_for(tx, &OwnerKey::physician(scope, owner), event, admissible).await
}

pub(super) async fn load_for<T>(
    db: &impl ConnectionTrait,
    key: &OwnerKey<'_>,
    attempts: &[String],
    project: impl Fn(engine::Event, bool) -> T,
) -> Result<HashMap<String, Vec<T>>, DbErr> {
    if attempts.is_empty() {
        return Ok(HashMap::new());
    }
    let rows = execution_event::Entity::find()
        .filter(owner_filter(key))
        .filter(execution_event::Column::Attempt.is_in(attempts.iter().cloned()))
        .order_by_asc(execution_event::Column::Sequence)
        .all(db)
        .await?;
    let mut events: HashMap<String, Vec<T>> = HashMap::new();
    for row in rows {
        events.entry(row.attempt.clone()).or_default().push(project(
            engine::Event {
                attempt_id: row.attempt,
                sequence: u64::try_from(row.sequence)
                    .map_err(conversion_error("saved event sequence", "u64"))?,
                ordinal: u64::try_from(row.ordinal)
                    .map_err(conversion_error("saved provider ordinal", "u64"))?,
                kind: serde_json::from_value(row.record)
                    .map_err(conversion_error("execution event record", "EventKind"))?,
            },
            row.admissible,
        ));
    }
    Ok(events)
}

/// Reconstruct the public attempt view only when a caller needs its evidence.
pub(super) async fn load<T>(
    db: &impl ConnectionTrait,
    scope: &Scope,
    owner: Owner,
    attempts: &[String],
    project: impl Fn(engine::Event, bool) -> T,
) -> Result<HashMap<String, Vec<T>>, DbErr> {
    load_for(db, &OwnerKey::physician(scope, owner), attempts, project).await
}

pub(super) async fn ordered_for(
    db: &impl ConnectionTrait,
    key: &OwnerKey<'_>,
    attempts: &[String],
) -> Result<Vec<(engine::Event, bool)>, DbErr> {
    let mut records = load_for(db, key, attempts, |event, admitted| (event, admitted)).await?;
    Ok(attempts
        .iter()
        .flat_map(|attempt| records.remove(attempt).unwrap_or_default())
        .collect())
}

/// Ordered records preserve attempt order and the authority recorded at acknowledgment.
pub(super) async fn ordered(
    db: &impl ConnectionTrait,
    scope: &Scope,
    owner: Owner,
    attempts: &[String],
) -> Result<Vec<(engine::Event, bool)>, DbErr> {
    ordered_for(db, &OwnerKey::physician(scope, owner), attempts).await
}

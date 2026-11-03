//! Accepted follow-through. All mutation methods borrow the caller's transaction:
//! the conversation owner must fence and record its existing tool effect in that
//! SAME transaction. No client endpoint may deserialize an agent authority.
//! Person restrictions must be committed with the existing privacy outbox and
//! independently confirmed by its journal; these methods never claim a receipt.
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]
// Field names mirror the reference follow-up wire vocabulary; operation methods
// below document the authority and transaction requirements.
#![allow(missing_docs)]
use crate::{
    ApiError,
    identity::{AccountContext, check_current},
};
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{Postgres, Row, Transaction};
use uuid::Uuid;

type Tx<'a> = Transaction<'a, Postgres>;
fn conflict() -> ApiError {
    ApiError::Conflict("follow-up or accepted authority changed")
}
fn invalid() -> ApiError {
    ApiError::Unprocessable("invalid follow-up definition or schedule")
}

/// Calendar schedules supported by the original follow-up owner. Timezone is
/// retained IANA identity, never the worker or phone's current timezone.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Schedule {
    Once {
        timezone: String,
        local_date: NaiveDate,
        local_time: NaiveTime,
    },
    Recurring {
        timezone: String,
        start_date: NaiveDate,
        local_time: NaiveTime,
        frequency: Frequency,
        #[serde(default = "one")]
        interval: u16,
        weekdays: Option<Vec<Weekday>>,
        day_of_month: Option<u8>,
        ends_on: Option<NaiveDate>,
    },
}
fn one() -> u16 {
    1
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl Schedule {
    fn zone_time(&self) -> (&str, NaiveTime) {
        match self {
            Self::Once {
                timezone,
                local_time,
                ..
            }
            | Self::Recurring {
                timezone,
                local_time,
                ..
            } => (timezone, *local_time),
        }
    }
    fn validate(&self) -> Result<(), ApiError> {
        let (zone, _) = self.zone_time();
        if zone.is_empty() || zone.len() > 100 {
            return Err(invalid());
        }
        if let Self::Recurring {
            start_date,
            frequency,
            interval,
            weekdays,
            day_of_month,
            ends_on,
            ..
        } = self
        {
            if !(1..=365).contains(interval) || ends_on.is_some_and(|end| end < *start_date) {
                return Err(invalid());
            }
            match frequency {
                Frequency::Daily if weekdays.is_some() || day_of_month.is_some() => {
                    return Err(invalid());
                }
                Frequency::Weekly => {
                    let days = weekdays.as_ref().ok_or_else(invalid)?;
                    if days.is_empty()
                        || day_of_month.is_some()
                        || days.iter().enumerate().any(|(i, d)| days[..i].contains(d))
                    {
                        return Err(invalid());
                    }
                }
                Frequency::Monthly
                    if weekdays.is_some()
                        || !day_of_month.is_some_and(|d| (1..=31).contains(&d)) =>
                {
                    return Err(invalid());
                }
                _ => {}
            }
        }
        Ok(())
    }
    fn matches(&self, day: NaiveDate) -> bool {
        match self {
            Self::Once { local_date, .. } => day == *local_date,
            Self::Recurring {
                start_date,
                frequency,
                interval,
                weekdays,
                day_of_month,
                ends_on,
                ..
            } => {
                if day < *start_date || ends_on.is_some_and(|end| day > end) {
                    return false;
                }
                let distance = day.signed_duration_since(*start_date).num_days();
                match frequency {
                    Frequency::Daily => distance % i64::from(*interval) == 0,
                    Frequency::Weekly => {
                        distance / 7 % i64::from(*interval) == 0
                            && weekdays.as_ref().is_some_and(|days| {
                                days.iter()
                                    .any(|d| *d as u32 == day.weekday().num_days_from_monday())
                            })
                    }
                    Frequency::Monthly => {
                        ((day.year() - start_date.year()) * 12 + day.month() as i32
                            - start_date.month() as i32)
                            % i32::from(*interval)
                            == 0
                            && Some(day.day() as u8) == *day_of_month
                    }
                }
            }
        }
    }
    /// Stable presentation only; the typed schedule, not this label, is authority.
    pub fn summary(&self) -> String {
        let (zone, time) = self.zone_time();
        match self {
            Self::Once { local_date, .. } => format!("{local_date} at {time} ({zone})"),
            Self::Recurring {
                frequency,
                interval,
                weekdays,
                day_of_month,
                ends_on,
                ..
            } => {
                let cadence = match frequency {
                    Frequency::Daily => {
                        if *interval == 1 {
                            "Every day".to_owned()
                        } else {
                            format!("Every {interval} days")
                        }
                    }
                    Frequency::Weekly => {
                        let days = weekdays
                            .as_ref()
                            .map(|days| {
                                days.iter()
                                    .map(|day| format!("{day:?}"))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            })
                            .unwrap_or_default();
                        if *interval == 1 {
                            format!("Every {days}")
                        } else {
                            format!("Every {interval} weeks on {days}")
                        }
                    }
                    Frequency::Monthly => format!(
                        "Every {interval} month(s) on day {}",
                        day_of_month.unwrap_or_default()
                    ),
                };
                let ending = ends_on
                    .map(|end| format!(" through {end}"))
                    .unwrap_or_default();
                format!("{cadence} at {time} ({zone}){ending}")
            }
        }
    }
}
/// PostgreSQL's installed tzdb supplies IANA rules without a new Rust dependency.
/// Gaps use PostgreSQL's forward-by-gap resolution. Folds explicitly select the
/// first instant (PostgreSQL's ordinary AT TIME ZONE would select the second).
async fn wall_time(
    tx: &mut Tx<'_>,
    zone: &str,
    day: NaiveDate,
    time: NaiveTime,
) -> Result<DateTime<Utc>, ApiError> {
    Ok(sqlx::query_scalar("with wall as (select $1::timestamp as w), offsets as (select distinct (s at time zone $2)-(s at time zone 'UTC') as off from wall, lateral generate_series((w at time zone 'UTC')-interval '2 days',(w at time zone 'UTC')+interval '2 days',interval '1 hour') s), candidates as (select (w-off) at time zone 'UTC' as instant,w from wall,offsets) select coalesce(min(instant) filter(where instant at time zone $2=w),(select w at time zone $2 from wall)) from candidates")
        .bind(day.and_time(time)).bind(zone).fetch_one(&mut **tx).await?)
}
/// Exact reference daily/weekly/monthly recurrence, bounded at fifty years;
/// nonexistent month days are skipped and overdue work never catches up in bursts.
pub async fn next_occurrence(
    tx: &mut Tx<'_>,
    schedule: &Schedule,
    after: DateTime<Utc>,
    inclusive: bool,
) -> Result<Option<DateTime<Utc>>, ApiError> {
    schedule.validate()?;
    let (zone, time) = schedule.zone_time();
    let known: bool =
        sqlx::query_scalar("select exists(select 1 from pg_timezone_names where name=$1)")
            .bind(zone)
            .fetch_one(&mut **tx)
            .await?;
    if !known {
        return Err(invalid());
    }
    let local: NaiveDate = sqlx::query_scalar("select ($1::timestamptz at time zone $2)::date")
        .bind(after)
        .bind(zone)
        .fetch_one(&mut **tx)
        .await?;
    let mut day = match schedule {
        Schedule::Once { local_date, .. } => *local_date,
        Schedule::Recurring { start_date, .. } => local.max(*start_date),
    };
    let end = day
        .checked_add_signed(Duration::days(366 * 50))
        .ok_or_else(invalid)?;
    while day <= end {
        if let Schedule::Recurring {
            ends_on: Some(last),
            ..
        } = schedule
            && day > *last
        {
            break;
        }
        if schedule.matches(day) {
            let instant = wall_time(tx, zone, day, time).await?;
            if instant > after || (inclusive && instant == after) {
                return Ok(Some(instant));
            }
        }
        if matches!(schedule, Schedule::Once { .. }) {
            break;
        }
        day = day.succ_opt().ok_or_else(invalid)?;
    }
    Ok(None)
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    CheckIn,
    Reminder,
}
impl Operation {
    fn name(self) -> &'static str {
        match self {
            Self::CheckIn => "check_in",
            Self::Reminder => "reminder",
        }
    }
}
/// Only tool arguments, NOT an authenticated import contract. The current
/// accepted command and delivered exact source messages supply authority.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    pub id: Uuid,
    pub operation: Operation,
    pub accepted_instruction: String,
    pub schedule: Schedule,
    pub source_message_ids: Vec<Uuid>,
    pub display_title: Option<String>,
}
impl Definition {
    fn validate(&self) -> Result<(), ApiError> {
        self.schedule.validate()?;
        if self.accepted_instruction.trim().is_empty()
            || self.accepted_instruction.len() > 32_000
            || self.source_message_ids.is_empty()
            || self.source_message_ids.len() > 16
            || self
                .source_message_ids
                .iter()
                .enumerate()
                .any(|(i, id)| self.source_message_ids[..i].contains(id))
            || self
                .display_title
                .as_ref()
                .is_some_and(|t| t.trim().is_empty() || t.chars().count() > 96)
        {
            return Err(invalid());
        }
        Ok(())
    }
    fn identity(&self) -> Value {
        json!({"operation":self.operation,"accepted_instruction":self.accepted_instruction.trim(),"schedule":self.schedule,"source_message_ids":self.source_message_ids})
    }
}
/// Unserializable lease authority, constructed only by the execution owner.
/// Never accept a model/client supplied `read_untrusted` flag.
pub struct AgentAuthority {
    context: AccountContext,
    command: Uuid,
    lease: Uuid,
    claim: i64,
}
impl AgentAuthority {
    /// Caller obtains these fields from its existing owner, not tool arguments.
    pub fn from_owner(context: AccountContext, command: Uuid, lease: Uuid, claim: i64) -> Self {
        Self {
            context,
            command,
            lease,
            claim,
        }
    }
    async fn validate(&self, tx: &mut Tx<'_>) -> Result<bool, ApiError> {
        check_current(tx, &self.context, true).await?;
        let row=sqlx::query("select d.input,d.request,d.admission,d.context from duet_commands d join duet_conversations c on c.user_id=d.user_id and c.id=d.conversation_id where d.user_id=$1 and d.id=$2 and d.state='running' and d.lease_id=$3 and d.claim=$4 and d.lease_until>clock_timestamp() and d.base_revision=c.revision and not exists(select 1 from duet_follow_up_occurrences o where o.user_id=d.user_id and o.command_id=d.id)")
            .bind(self.context.account_id).bind(self.command).bind(self.lease).bind(self.claim).fetch_optional(&mut **tx).await?.ok_or_else(conflict)?;
        let accepted: AccountContext =
            serde_json::from_value(row.get("context")).map_err(|_| conflict())?;
        check_current(tx, &accepted, true).await?;
        if accepted.account_id != self.context.account_id {
            return Err(conflict());
        }
        let input: String = row.get("input");
        let request: Value = row.get("request");
        // A current patient message, not a scheduled assignment or imported text.
        if input.trim().is_empty()
            || !request["items"].as_array().is_some_and(|items| {
                items
                    .iter()
                    .any(|v| v["role"] == "user" && v["text"] == input)
            })
        {
            return Err(conflict());
        }
        let admission: Value = row.get("admission");
        let reads: bool=sqlx::query_scalar("select exists(select 1 from duet_tool_effects where user_id=$1 and command_id=$2 and call->>'name' not in ('read_message','read_follow_up','list_follow_ups','create_follow_up','edit_follow_up','pause_follow_up','resume_follow_up','cancel_follow_up','stage_memory'))")
            .bind(self.context.account_id).bind(self.command).fetch_one(&mut **tx).await?;
        Ok(reads || admission.get("source").is_some())
    }
    async fn sources(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<(), ApiError> {
        if !ids.contains(&self.command) {
            return Err(ApiError::Conflict(
                "follow-up requires the current patient's request or acceptance",
            ));
        }
        for id in ids {
            if *id == self.command {
                continue;
            }
            // ENG-200: a steer already dispatched to this command is current
            // patient input to it, like the command's own message.
            let steer: bool = sqlx::query_scalar("select exists(select 1 from duet_command_steers where user_id=$1 and command_id=$2 and id=$3 and applied_sequence is not null and btrim(input)<>'')")
                .bind(self.context.account_id).bind(self.command).bind(id).fetch_one(&mut **tx).await?;
            if steer {
                continue;
            }
            // Message IDs match the existing phone projection. Prior exact reads
            // are required even if an unaddressed transcript was in initial context.
            let command = Uuid::from_u128(id.as_u128() ^ (1u128 << 127));
            let exact: bool=sqlx::query_scalar("select exists(select 1 from duet_commands d join duet_tool_effects e on e.user_id=d.user_id and e.command_id=$2 where d.user_id=$1 and (d.id=$3 or d.id=$4) and d.state='complete' and e.call->>'name'='read_message' and e.result->>'command_id'=d.id::text and e.result->>'input'=d.input and e.result->'reply'=d.reply and ((d.id=$3 and btrim(d.input)<>'') or (d.id=$4 and coalesce(d.reply->>'text','')<>'')))")
                .bind(self.context.account_id).bind(self.command).bind(id).bind(command).fetch_one(&mut **tx).await?;
            if !exact {
                return Err(ApiError::Conflict(
                    "follow-up requires exact delivered account-owned source messages",
                ));
            }
        }
        Ok(())
    }
}
async fn lock_account(tx: &mut Tx<'_>, account: Uuid) -> Result<(), ApiError> {
    sqlx::query("select id from users where id=$1 for update")
        .bind(account)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(())
}
/// Read the actual definition and occurrence owner; never manufacture evidence cards.
pub async fn read(tx: &mut Tx<'_>, account: Uuid, id: Uuid) -> Result<Value, ApiError> {
    let mut value: Value =
        sqlx::query_scalar("select to_jsonb(f) from follow_ups f where user_id=$1 and id=$2")
            .bind(account)
            .bind(id)
            .fetch_optional(&mut **tx)
            .await?
            .ok_or(ApiError::NotFound)?;
    let current:Option<Value>=sqlx::query_scalar("select to_jsonb(o) from duet_follow_up_occurrences o where user_id=$1 and follow_up_id=$2 and status in ('queued','running')").bind(account).bind(id).fetch_optional(&mut **tx).await?;
    value["current_occurrence"] = json!(current);
    let last:Option<Value>=sqlx::query_scalar("select jsonb_build_object('status',status,'scheduled_for',scheduled_for,'completed_at',completed_at,'conversation_message_id',conversation_message_id) from duet_follow_up_occurrences where user_id=$1 and follow_up_id=$2 and completed_at is not null order by completed_at desc,id desc limit 1").bind(account).bind(id).fetch_optional(&mut **tx).await?;
    value["last_outcome"] = json!(last);
    let ids: Vec<Uuid> =
        serde_json::from_value(value["source_message_ids"].clone()).map_err(|_| conflict())?;
    let mut basis = Vec::new();
    for message in ids {
        let assistant_command = Uuid::from_u128(message.as_u128() ^ (1u128 << 127));
        if let Some(row)=sqlx::query("select id,created_at from duet_commands where user_id=$1 and (id=$2 or id=$3) order by (id=$2) desc limit 1").bind(account).bind(message).bind(assistant_command).fetch_optional(&mut **tx).await? {
            basis.push(json!({"message_id":message,"role":if row.get::<Uuid,_>("id")==message {"user"}else{"assistant"},"created_at":row.get::<DateTime<Utc>,_>("created_at")}));
        }
    }
    value["source_basis"] = json!(basis);
    value["pause_reason"] = if value["state"] == "paused" {
        json!(if value["confirmation_required"] == true {
            "confirmation_required"
        } else {
            "user"
        })
    } else {
        Value::Null
    };
    Ok(value)
}
async fn open_limit(tx: &mut Tx<'_>, account: Uuid) -> Result<(), ApiError> {
    let count: i64 = sqlx::query_scalar(
        "select count(*) from follow_ups where user_id=$1 and state in ('active','paused')",
    )
    .bind(account)
    .fetch_one(&mut **tx)
    .await?;
    if count >= 20 {
        return Err(ApiError::Conflict("account has 20 open follow-ups"));
    }
    Ok(())
}
/// Atomic with the caller's duet_tool_effects insertion. Title is cosmetic and
/// excluded from duplicate-create identity, matching the reference owner.
pub async fn create(
    tx: &mut Tx<'_>,
    actor: &AgentAuthority,
    definition: &Definition,
) -> Result<Value, ApiError> {
    definition.validate()?;
    let account = actor.context.account_id;
    lock_account(tx, account).await?;
    let untrusted = actor.validate(tx).await?;
    actor.sources(tx, &definition.source_message_ids).await?;
    if let Some(row)=sqlx::query("select command_id,creation_identity from duet_follow_up_origins where user_id=$1 and follow_up_id=$2").bind(account).bind(definition.id).fetch_optional(&mut **tx).await? {
        if row.get::<Uuid,_>("command_id")!=actor.command || row.get::<Value,_>("creation_identity")!=definition.identity() {return Err(conflict());}
        return read(tx,account,definition.id).await;
    }
    open_limit(tx, account).await?;
    let now: DateTime<Utc> = sqlx::query_scalar("select clock_timestamp()")
        .fetch_one(&mut **tx)
        .await?;
    let next = next_occurrence(tx, &definition.schedule, now, true)
        .await?
        .ok_or_else(invalid)?;
    sqlx::query("insert into follow_ups(id,user_id,operation,accepted_instruction,schedule,schedule_summary,state,next_occurrence_at,source_message_ids,display_title,confirmation_required) values($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)")
        .bind(definition.id).bind(account).bind(definition.operation.name()).bind(definition.accepted_instruction.trim()).bind(json!(definition.schedule)).bind(definition.schedule.summary()).bind(if untrusted {"paused"}else{"active"}).bind(if untrusted {None}else{Some(next)}).bind(&definition.source_message_ids).bind(definition.display_title.as_ref().map(|s|s.trim())).bind(untrusted).execute(&mut **tx).await?;
    sqlx::query("insert into duet_follow_up_origins(user_id,follow_up_id,command_id,creation_identity) values($1,$2,$3,$4)").bind(account).bind(definition.id).bind(actor.command).bind(definition.identity()).execute(&mut **tx).await?;
    read(tx, account, definition.id).await
}
async fn exact_definition(
    tx: &mut Tx<'_>,
    actor: &AgentAuthority,
    id: Uuid,
    revision: i64,
) -> Result<(), ApiError> {
    let seen:bool=sqlx::query_scalar("select exists(select 1 from duet_tool_effects where user_id=$1 and command_id=$2 and call->>'name'='read_follow_up' and result->>'id'=$3::uuid::text and result->>'revision'=$4::bigint::text)").bind(actor.context.account_id).bind(actor.command).bind(id).bind(revision).fetch_one(&mut **tx).await?;
    if !seen {
        return Err(ApiError::Conflict(
            "mutation requires exact delivered follow-up revision",
        ));
    }
    Ok(())
}
/// Revision edit cannot clear a pending person-confirmation gate.
pub async fn edit(
    tx: &mut Tx<'_>,
    actor: &AgentAuthority,
    revision: i64,
    definition: &Definition,
) -> Result<Value, ApiError> {
    definition.validate()?;
    let account = actor.context.account_id;
    lock_account(tx, account).await?;
    let untrusted = actor.validate(tx).await?;
    actor.sources(tx, &definition.source_message_ids).await?;
    exact_definition(tx, actor, definition.id, revision).await?;
    let current = read(tx, account, definition.id).await?;
    if current["revision"] != revision || current["state"] == "cancelled" {
        return Err(conflict());
    }
    if current["state"] == "completed" {
        open_limit(tx, account).await?;
    }
    if current["operation"] == definition.operation.name()
        && current["accepted_instruction"] == definition.accepted_instruction.trim()
        && current["schedule"] == json!(definition.schedule)
        && definition
            .display_title
            .as_ref()
            .is_some_and(|t| current["display_title"] != t.trim())
    {
        return Err(ApiError::Conflict(
            "title-only revision is not accepted work",
        ));
    }
    let waiting = untrusted || current["confirmation_required"] == true;
    let paused = waiting || current["state"] == "paused";
    let now = sqlx::query_scalar("select clock_timestamp()")
        .fetch_one(&mut **tx)
        .await?;
    let next = next_occurrence(tx, &definition.schedule, now, true)
        .await?
        .ok_or_else(invalid)?;
    sqlx::query("update follow_ups set operation=$3,accepted_instruction=$4,schedule=$5,schedule_summary=$6,revision=revision+1,state=$7,next_occurrence_at=$8,source_message_ids=$9,display_title=coalesce($10,display_title),confirmation_required=$11,updated_at=now() where user_id=$1 and id=$2")
        .bind(account).bind(definition.id).bind(definition.operation.name()).bind(definition.accepted_instruction.trim()).bind(json!(definition.schedule)).bind(definition.schedule.summary()).bind(if paused {"paused"}else{"active"}).bind(if paused {None}else{Some(next)}).bind(&definition.source_message_ids).bind(definition.display_title.as_ref().map(|t|t.trim())).bind(waiting).execute(&mut **tx).await?;
    interrupt(tx, account, definition.id).await?;
    read(tx, account, definition.id).await
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Control {
    Pause,
    Resume,
    Cancel,
}
/// Person controls work without AI permission. Pause/cancel callers MUST append
/// the privacy receipt in this transaction and confirm it after commit.
pub async fn person_control(
    tx: &mut Tx<'_>,
    context: &AccountContext,
    id: Uuid,
    revision: i64,
    control: Control,
) -> Result<Value, ApiError> {
    check_current(tx, context, false).await?;
    control_inner(tx, context.account_id, id, revision, control, false, false).await
}
pub async fn agent_control(
    tx: &mut Tx<'_>,
    actor: &AgentAuthority,
    id: Uuid,
    revision: i64,
    control: Control,
) -> Result<Value, ApiError> {
    lock_account(tx, actor.context.account_id).await?;
    let untrusted = actor.validate(tx).await?;
    exact_definition(tx, actor, id, revision).await?;
    control_inner(
        tx,
        actor.context.account_id,
        id,
        revision,
        control,
        true,
        untrusted,
    )
    .await
}
async fn control_inner(
    tx: &mut Tx<'_>,
    account: Uuid,
    id: Uuid,
    revision: i64,
    control: Control,
    agent: bool,
    untrusted: bool,
) -> Result<Value, ApiError> {
    lock_account(tx, account).await?;
    let current = read(tx, account, id).await?;
    if current["revision"] != revision {
        return Err(conflict());
    }
    let prior = current["state"].as_str().ok_or_else(conflict)?;
    let mut waiting = current["confirmation_required"] == true;
    let mut next = None;
    let target = match control {
        Control::Cancel => "cancelled",
        Control::Pause => {
            if prior == "cancelled" || prior == "completed" {
                return Err(conflict());
            }
            "paused"
        }
        Control::Resume => {
            if prior == "cancelled" || (agent && (waiting || untrusted)) {
                return Err(conflict());
            }
            let schedule: Schedule =
                serde_json::from_value(current["schedule"].clone()).map_err(|_| invalid())?;
            let now = sqlx::query_scalar("select clock_timestamp()")
                .fetch_one(&mut **tx)
                .await?;
            next = next_occurrence(tx, &schedule, now, true).await?;
            if !agent {
                waiting = false;
            }
            if next.is_some() {
                if prior == "completed" {
                    open_limit(tx, account).await?;
                }
                "active"
            } else {
                "completed"
            }
        }
    };
    if target != prior || (current["confirmation_required"] == true && !waiting) {
        sqlx::query("update follow_ups set state=$3,revision=revision+1,next_occurrence_at=$4,confirmation_required=$5,updated_at=now() where user_id=$1 and id=$2")
            .bind(account).bind(id).bind(target).bind(next).bind(waiting && target=="paused").execute(&mut **tx).await?;
    }
    if control != Control::Resume {
        interrupt(tx, account, id).await?;
    } else if target != prior {
        notify(tx, account, id).await?;
    }
    read(tx, account, id).await
}
async fn notify(tx: &mut Tx<'_>, account: Uuid, id: Uuid) -> Result<(), ApiError> {
    sqlx::query("insert into duet_conversation_events(user_id,command_id,event) select distinct e.user_id,e.command_id,jsonb_build_object('type','follow_up_changed','follow_up_id',$2::uuid) from duet_tool_effects e where e.user_id=$1 and e.call->>'name' in ('create_follow_up','edit_follow_up','pause_follow_up','resume_follow_up','cancel_follow_up') and e.result->>'id'=$2::uuid::text").bind(account).bind(id).execute(&mut **tx).await?;
    Ok(())
}
async fn interrupt(tx: &mut Tx<'_>, account: Uuid, id: Uuid) -> Result<(), ApiError> {
    // Wake clients of original action messages without rewriting historical text.
    notify(tx, account, id).await?;
    sqlx::query("update duet_commands d set state='cancelled',lease_id=null,lease_until=null,staged_memory=null where user_id=$1 and state in ('accepted','running') and exists(select 1 from duet_follow_up_occurrences o where o.user_id=d.user_id and o.command_id=d.id and o.follow_up_id=$2)").bind(account).bind(id).execute(&mut **tx).await?;
    sqlx::query("update duet_follow_up_occurrences set status='interrupted',completed_at=now(),updated_at=now() where user_id=$1 and follow_up_id=$2 and status in ('queued','running')").bind(account).bind(id).execute(&mut **tx).await?;
    Ok(())
}
/// Called ONLY by privacy replay after receipt byte verification and incarnation
/// validation. It never grants authority, resumes work, or reopens ended work.
pub async fn replay_restriction(
    tx: &mut Tx<'_>,
    account: Uuid,
    incarnation: Uuid,
    id: Uuid,
    cancel: bool,
) -> Result<(), ApiError> {
    lock_account(tx, account).await?;
    let same: bool = sqlx::query_scalar("select duet_incarnation=$2 from users where id=$1")
        .bind(account)
        .bind(incarnation)
        .fetch_one(&mut **tx)
        .await?;
    if !same {
        return Ok(());
    }
    sqlx::query("update follow_ups set state=case when $3 then 'cancelled' else 'paused' end,revision=revision+1,next_occurrence_at=null,confirmation_required=case when $3 then false else confirmation_required end,updated_at=now() where user_id=$1 and id=$2 and (($3 and state<>'cancelled') or (not $3 and state='active'))").bind(account).bind(id).bind(cancel).execute(&mut **tx).await?;
    interrupt(tx, account, id).await
}
/// Versioned patient setting. Disabled settings fence publication without
/// rewriting accepted work. A withdrawal must share the caller's privacy outbox.
pub async fn set_enabled(
    tx: &mut Tx<'_>,
    context: &AccountContext,
    revision: i64,
    enabled: bool,
) -> Result<Value, ApiError> {
    check_current(tx, context, false).await?;
    let account = context.account_id;
    sqlx::query("insert into duet_follow_up_settings(user_id) values($1) on conflict do nothing")
        .bind(account)
        .execute(&mut **tx)
        .await?;
    let row = sqlx::query("select revision,enabled from duet_follow_up_settings where user_id=$1")
        .bind(account)
        .fetch_one(&mut **tx)
        .await?;
    if row.get::<i64, _>("revision") != revision {
        return Err(conflict());
    }
    if row.get::<bool, _>("enabled") != enabled {
        sqlx::query(
            "update duet_follow_up_settings set enabled=$2,revision=revision+1 where user_id=$1",
        )
        .bind(account)
        .bind(enabled)
        .execute(&mut **tx)
        .await?;
    }
    // Publication is immediately fenced. Scheduler integration suspends the same
    // queued/running command, rather than cancelling it or creating a replacement.
    let value: Value =
        sqlx::query_scalar("select to_jsonb(s) from duet_follow_up_settings s where user_id=$1")
            .bind(account)
            .fetch_one(&mut **tx)
            .await?;
    Ok(value)
}
/// Privacy-only monotonic settings replay after independent receipt validation.
pub async fn replay_settings_withdrawal(
    tx: &mut Tx<'_>,
    account: Uuid,
    incarnation: Uuid,
) -> Result<(), ApiError> {
    lock_account(tx, account).await?;
    sqlx::query("insert into duet_follow_up_settings(user_id,enabled) select id,false from users where id=$1 and duet_incarnation=$2 on conflict(user_id) do update set enabled=false,revision=duet_follow_up_settings.revision+(duet_follow_up_settings.enabled::int)").bind(account).bind(incarnation).execute(&mut **tx).await?;
    Ok(())
}
/// Reserve one due occurrence with the SAME command identity the conversation
/// owner will insert before committing. No synthetic patient message is created.
/// Deferred FK makes an orphan reservation impossible. Admission refuses a busy
/// account so newer interactive work retains precedence.
pub async fn reserve_due(
    tx: &mut Tx<'_>,
    context: &AccountContext,
    command: Uuid,
) -> Result<Option<Value>, ApiError> {
    check_current(tx, context, true).await?;
    let account = context.account_id;
    lock_account(tx, account).await?;
    if let Some(prior) = sqlx::query_scalar::<_, Value>(
        "select to_jsonb(o) from duet_follow_up_occurrences o where user_id=$1 and command_id=$2",
    )
    .bind(account)
    .bind(command)
    .fetch_optional(&mut **tx)
    .await?
    {
        return Ok(Some(prior));
    }
    let allowed:bool=sqlx::query_scalar("select coalesce((select enabled from duet_follow_up_settings where user_id=$1),true) and not exists(select 1 from duet_commands where user_id=$1 and state in ('accepted','running'))").bind(account).fetch_one(&mut **tx).await?;
    if !allowed {
        return Ok(None);
    }
    let row=sqlx::query("select f.id,f.revision,f.next_occurrence_at,f.schedule,f.operation,f.accepted_instruction from follow_ups f join duet_follow_up_origins p on p.user_id=f.user_id and p.follow_up_id=f.id where f.user_id=$1 and f.state='active' and not f.confirmation_required and f.next_occurrence_at<=clock_timestamp() and not exists(select 1 from duet_follow_up_occurrences o where o.user_id=f.user_id and o.follow_up_id=f.id and o.status in ('queued','running')) order by f.next_occurrence_at,f.id limit 1 for update of f").bind(account).fetch_optional(&mut **tx).await?;
    let Some(row) = row else { return Ok(None) };
    let id: Uuid = row.get("id");
    let revision: i64 = row.get("revision");
    let due: DateTime<Utc> = row.get("next_occurrence_at");
    let schedule: Schedule = serde_json::from_value(row.get("schedule")).map_err(|_| invalid())?;
    let now = sqlx::query_scalar("select clock_timestamp()")
        .fetch_one(&mut **tx)
        .await?;
    let next = next_occurrence(tx, &schedule, now, false).await?;
    let occurrence = Uuid::new_v4();
    sqlx::query("insert into duet_follow_up_occurrences(user_id,id,follow_up_id,follow_up_revision,scheduled_for,command_id,operation,accepted_instruction) values($1,$2,$3,$4,$5,$6,$7,$8)").bind(account).bind(occurrence).bind(id).bind(revision).bind(due).bind(command).bind(row.get::<String,_>("operation")).bind(row.get::<String,_>("accepted_instruction")).execute(&mut **tx).await?;
    sqlx::query("update follow_ups set next_occurrence_at=$3,state=case when $3::timestamptz is null then 'completed' else 'active' end,updated_at=now() where user_id=$1 and id=$2").bind(account).bind(id).bind(next).execute(&mut **tx).await?;
    Ok(Some(
        json!({"id":occurrence,"follow_up_id":id,"follow_up_revision":revision,"scheduled_for":due,"command_id":command,"operation":row.get::<String,_>("operation"),"accepted_instruction":row.get::<String,_>("accepted_instruction")}),
    ))
}
/// Include this in ConversationOwner::fence for every scheduled effect and
/// completion. One-time schedule exhaustion does not revoke its last occurrence.
pub async fn fence_scheduled(
    tx: &mut Tx<'_>,
    context: &AccountContext,
    command: Uuid,
) -> Result<Option<Value>, ApiError> {
    check_current(tx, context, true).await?;
    let row: Option<Value> = sqlx::query_scalar(
        "select to_jsonb(o) from duet_follow_up_occurrences o where user_id=$1 and command_id=$2",
    )
    .bind(context.account_id)
    .bind(command)
    .fetch_optional(&mut **tx)
    .await?;
    let Some(row) = row else { return Ok(None) };
    let valid:bool=sqlx::query_scalar("select exists(select 1 from duet_follow_up_occurrences o join follow_ups f on f.user_id=o.user_id and f.id=o.follow_up_id where o.user_id=$1 and o.command_id=$2 and o.status in ('queued','running') and f.state in ('active','completed') and not f.confirmation_required and f.revision=o.follow_up_revision) and coalesce((select enabled from duet_follow_up_settings where user_id=$1),true)").bind(context.account_id).bind(command).fetch_one(&mut **tx).await?;
    if !valid {
        return Err(conflict());
    }
    Ok(Some(row))
}
/// Mark running alongside claiming the existing command; recovery reclaims the
/// same command/occurrence, not another scheduler execution ledger.
pub async fn mark_running(
    tx: &mut Tx<'_>,
    context: &AccountContext,
    command: Uuid,
) -> Result<(), ApiError> {
    fence_scheduled(tx, context, command).await?;
    sqlx::query("update duet_follow_up_occurrences set status='running',updated_at=now() where user_id=$1 and command_id=$2 and status='queued'").bind(context.account_id).bind(command).execute(&mut **tx).await?;
    Ok(())
}
/// Finish atomically with the existing reply/effect/event commit; no push or
/// evidence chip is fabricated. The caller has already fenced its lease.
pub async fn complete_scheduled(
    tx: &mut Tx<'_>,
    context: &AccountContext,
    command: Uuid,
    message: Uuid,
) -> Result<(), ApiError> {
    if fence_scheduled(tx, context, command).await?.is_none() {
        return Ok(());
    }
    let expected = Uuid::from_u128(command.as_u128() ^ (1u128 << 127));
    if message != expected {
        return Err(conflict());
    }
    let changed=sqlx::query("update duet_follow_up_occurrences set status='completed',conversation_message_id=$3,completed_at=now(),updated_at=now() where user_id=$1 and command_id=$2 and status='running'").bind(context.account_id).bind(command).bind(message).execute(&mut **tx).await?;
    if changed.rows_affected() != 1 {
        return Err(conflict());
    }
    Ok(())
}
/// Temporary newer-message/settings hold releases the SAME unfinished assignment.
/// Owner must suspend the linked command in this transaction, preserving identity.
pub async fn hold_scheduled(tx: &mut Tx<'_>, account: Uuid, command: Uuid) -> Result<(), ApiError> {
    lock_account(tx, account).await?;
    sqlx::query("update duet_follow_up_occurrences set status='queued',updated_at=now() where user_id=$1 and command_id=$2 and status='running'").bind(account).bind(command).execute(&mut **tx).await?;
    Ok(())
}

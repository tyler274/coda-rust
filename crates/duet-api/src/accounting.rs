//! Content-free per-attempt usage, separate from diagnostic execution records.
//! Prices follow Duet cost_controls.MODEL_RATES, not an invented billing policy.
use crate::{
    ApiError,
    database::AccountDatabase,
    identity::{AccountContext, check_current},
};
use harness::execution::{Record, Usage};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, Postgres, Row, Transaction};
use uuid::Uuid;

// Versioned reference rate: gpt-6-luna, effective 2026-09-01 UTC. Cache
// writes are not separately reported and are treated as ordinary input;
// cached input retains $0.01/M in the >272K-input long-context tier.
const PRICED: &str = "select *, case when input_tokens is null or output_tokens is null then null when input_tokens=0 and output_tokens=0 then 0::numeric when model='gpt-6-luna' and started_at >= '2026-09-01T00:00:00Z'::timestamptz and coalesce(cached_input_tokens,0)<=input_tokens then ((input_tokens-coalesce(cached_input_tokens,0))::numeric*(case when input_tokens>272000 then 0.20 else 0.10 end)+coalesce(cached_input_tokens,0)::numeric*0.01+output_tokens::numeric*(case when input_tokens>272000 then 0.75 else 0.50 end))/1000000 else null end cost from duet_model_calls where user_id=$1 and command_id=$2";

/// Record under the same transaction as the execution evidence. Late receipts
/// may update only their original claim's attempt, even after permission loss.
pub async fn record(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    command: Uuid,
    claim: i64,
    record: &Record,
) -> Result<(), ApiError> {
    if let Record::Dispatch { sequence, request } = record {
        sqlx::query("insert into duet_model_calls(user_id,command_id,sequence,claim,model,purpose) select user_id,id,$3,$4,$5,work_kind from duet_commands where user_id=$1 and id=$2")
            .bind(account).bind(command).bind(*sequence as i64).bind(claim).bind(&request.model).execute(&mut **tx).await?;
        return Ok(());
    }
    let (sequence, usage, provenance, outcome) = match record {
        Record::Exchange { sequence, response } => (
            *sequence,
            response.usage.clone(),
            response.provenance.clone(),
            "completed",
        ),
        Record::Observation {
            sequence,
            usage,
            provenance,
        } => (*sequence, usage.clone(), provenance.clone(), "observed"),
        Record::Failure { sequence, .. } => (*sequence, Usage::default(), Value::Null, "failed"),
        _ => unreachable!(),
    };
    let token = |value: Option<u64>| value.and_then(|v| i64::try_from(v).ok());
    // Null observations cannot erase a previously reported count (including 0).
    sqlx::query("update duet_model_calls set input_tokens=coalesce($5,input_tokens),output_tokens=coalesce($6,output_tokens),cached_input_tokens=coalesce($7,cached_input_tokens),response_id=coalesce($8,response_id),outcome=case when outcome='completed' then outcome else $9 end where user_id=$1 and command_id=$2 and sequence=$3 and claim=$4")
        .bind(account).bind(command).bind(sequence as i64).bind(claim)
        .bind(token(usage.input_tokens)).bind(token(usage.output_tokens))
        .bind(token(provenance["cached_input_tokens"].as_u64()))
        .bind(provenance["response_id"].as_str()).bind(outcome).execute(&mut **tx).await?;
    Ok(())
}

/// Account-authorized command usage view. Incomplete totals remain null. The
/// exact decimal USD estimate is query-time only; invoice remains authoritative.
pub async fn run_usage(
    database: &AccountDatabase,
    context: &AccountContext,
    command: Uuid,
) -> Result<Value, ApiError> {
    let mut tx = database.begin(context.account_id).await?;
    check_current(&mut tx, context, false).await?;
    let exists: bool =
        sqlx::query_scalar("select exists(select 1 from duet_commands where user_id=$1 and id=$2)")
            .bind(context.account_id)
            .bind(command)
            .fetch_one(&mut *tx)
            .await?;
    if !exists {
        return Err(ApiError::NotFound);
    }
    // `PRICED` is a compile-time constant; every value is a bind parameter.
    let rows = sqlx::query(AssertSqlSafe(format!(
        "select *,cost::text as estimated_cost from ({PRICED}) priced order by sequence"
    )))
    .bind(context.account_id)
    .bind(command)
    .fetch_all(&mut *tx)
    .await?;
    // PostgreSQL computes exact decimal totals, never binary floating-point prices.
    let total: Option<String> = sqlx::query_scalar(AssertSqlSafe(format!("select case when count(*)=count(cost) then coalesce(sum(cost),0)::text end from ({PRICED}) priced")))
        .bind(context.account_id).bind(command).fetch_one(&mut *tx).await?;
    let mut input = Some(0i64);
    let mut output = Some(0i64);
    let mut unknown = 0;
    let mut attempts = Vec::new();
    for row in rows {
        let i: Option<i64> = row.get("input_tokens");
        let o: Option<i64> = row.get("output_tokens");
        input = input.zip(i).and_then(|(a, b)| a.checked_add(b));
        output = output.zip(o).and_then(|(a, b)| a.checked_add(b));
        if i.is_none() || o.is_none() {
            unknown += 1;
        }
        attempts.push(json!({"sequence":row.get::<i64,_>("sequence"),"claim":row.get::<i64,_>("claim"),"model":row.get::<String,_>("model"),"purpose":row.get::<String,_>("purpose"),"started_at":row.get::<Option<chrono::DateTime<chrono::Utc>>,_>("started_at"),"outcome":row.get::<String,_>("outcome"),"input_tokens":i,"output_tokens":o,"cached_input_tokens":row.get::<Option<i64>,_>("cached_input_tokens"),"response_id":row.get::<Option<String>,_>("response_id"),"estimated_cost_usd":row.get::<Option<String>,_>("estimated_cost")}));
    }
    Ok(
        json!({"command_id":command,"model_calls":attempts.len(),"unknown_usage_model_calls":unknown,"input_tokens":input,"output_tokens":output,"estimated_cost_usd":total,"cost_status":if total.is_some(){"estimated"}else{"unknown"},"rate_version":"duet-2026-09-01","invoice_authoritative":true,"attempts":attempts}),
    )
}

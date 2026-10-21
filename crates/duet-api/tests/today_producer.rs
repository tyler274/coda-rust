//! Deterministic producer proof. Requires the coordinator's ConversationOwner
//! hooks and migration registration; no provider keys or network model calls.
//! The path module also permits compile/unit validation before shared wiring.
pub use duet_api::{ApiError, database, evidence, identity, today};
use duet_api::{
    conversation::ConversationStore, database::AccountDatabase, identity::AccountContext,
};
use harness::execution::{
    Call, Cancellation, Limits, Provider, Request, Response, StreamEvent, Usage,
};
use serde_json::{Value, json};
use sqlx::AssertSqlSafe;
use sqlx::{ConnectOptions, Connection, Row};
use today::{SlotRequest, TodayStore};
use uuid::Uuid;

struct LocalProvider {
    cycle: Uuid,
    phase: u8,
    quiet: bool,
}
impl Provider for LocalProvider {
    async fn exchange(
        &mut self,
        request: &Request,
        _: &Cancellation,
        _: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        assert!(request.tools.iter().all(|t| matches!(
            t["name"].as_str(),
            Some("read_assessment_fact" | "stage_today" | "skill_view")
        )));
        assert!(
            request
                .items
                .iter()
                .all(|i| !matches!(i,harness::execution::Item::Message{role,..} if role=="user"))
        );
        let calls = match self.phase {
            0 if !self.quiet => vec![Call {
                id: "fact-read".into(),
                name: "read_assessment_fact".into(),
                arguments: json!({"cycle_id":self.cycle,"fact_id":"steps:completed"}),
            }],
            0 | 1 => vec![Call {
                id: "decision".into(),
                name: "stage_today".into(),
                arguments: json!({"cycle_id":self.cycle,"outcome":if self.quiet{"quiet"}else{"observations"},"insights":if self.quiet{json!([])}else{json!([{"title":"Recorded steps","detail":"The completed day recorded 7654 steps.","read_ids":["fact-read"]}])}}),
            }],
            _ => vec![],
        };
        self.phase += if self.quiet { 2 } else { 1 };
        Ok(Response {
            text: if calls.is_empty() {
                "Internal completion; never a patient reply".into()
            } else {
                String::new()
            },
            calls,
            usage: Usage {
                input_tokens: Some(12),
                output_tokens: Some(6),
            },
            provenance: json!({"mode":"local-deterministic"}),
        })
    }
}

#[test]
fn request_has_no_patient_message_or_mutating_conversation_capabilities() {
    let r = today::assessment_request(
        "local-test",
        "Health safety instructions",
        &json!({"slots":[]}),
    );
    assert_eq!(r.tools.len(), 2);
    assert!(
        r.items
            .iter()
            .all(|i| matches!(i,harness::execution::Item::Message{role,..} if role=="system"))
    );
    assert!(
        serde_json::from_value::<today::Decision>(
            json!({"cycle_id":Uuid::new_v4(),"outcome":"quiet","insights":[],"supporting_facts":[]})
        )
        .is_err()
    );
}

#[tokio::test]
async fn published_and_quiet_cycles_use_existing_command_owner() {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("isolated local PostgreSQL required, never silently skip producer proof");
    let opts: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&opts.get_host()));
    let mut root = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("duet_today_{suffix}");
    let role = format!("today_producer_{suffix}");
    sqlx::query(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut root)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut root)
        .await
        .unwrap();
    let owner_url = opts.clone().database(&name).to_url_lossy().to_string();
    let producer_url = opts
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    database::migrate(&owner_url, &role).await.unwrap();
    let mut admin = sqlx::PgConnection::connect(&owner_url).await.unwrap();
    // Parent owns registration. Do not quietly install a missing migration here.
    let registered: bool = sqlx::query_scalar(
        "select exists(select 1 from duet_schema_migrations where version='0012_today_producer')",
    )
    .fetch_one(&mut admin)
    .await
    .unwrap();
    assert!(
        registered,
        "register 0012_today_producer before running integrated producer proof"
    );
    // Test-only explicit producer grant. The ordinary runtime keeps SELECT only.
    sqlx::query(AssertSqlSafe(format!(
        "grant insert,update(corrected_insight_ids,updated_at) on insight_cycles to {role}"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
    let db = AccountDatabase::connect(&producer_url).await.unwrap();
    let today = TodayStore::new(db.clone());
    let conversations = ConversationStore::new(db.clone());
    for quiet in [false, true] {
        let account = Uuid::new_v4();
        sqlx::query("insert into users(id,auth_user_id,sharing_label,intelligence_enabled,luna_consent_decision,luna_consent_version,terms_version) values($1,$2,'Today proof',true,'accepted','2026-09-24','2026-09-23')").bind(account).bind(account.to_string()).execute(&mut admin).await.unwrap();
        let (incarnation, permission_revision): (Uuid, i64) = sqlx::query_as(
            "select duet_incarnation,intelligence_control_revision from users where id=$1",
        )
        .bind(account)
        .fetch_one(&mut admin)
        .await
        .unwrap();
        let c = AccountContext {
            account_id: account,
            incarnation,
            permission_revision,
            authenticated_at: chrono::Utc::now(),
        };
        let (start,end):(chrono::DateTime<chrono::Utc>,chrono::DateTime<chrono::Utc>)=sqlx::query_as("select date_trunc('day',clock_timestamp() at time zone 'UTC') at time zone 'UTC',(date_trunc('day',clock_timestamp() at time zone 'UTC')+interval '1 day') at time zone 'UTC'").fetch_one(&mut admin).await.unwrap();
        let snapshot = Uuid::new_v4();
        let slot = SlotRequest {
            kind: "daily".into(),
            window_start: start,
            window_end: end,
            timezone: "UTC".into(),
            first_weekday: 7,
            snapshot_id: snapshot,
        };
        let fact = json!({"id":"steps:completed","kind":"current_value","concept_ids":["steps"],"title":"Steps recorded yesterday","window":"day","range_start":start-chrono::Duration::days(1),"range_end":start,"value":7654.0,"unit":"count","evidence":[{"id":"stat:steps","kind":"stat","title":"Steps","source_kind":"appleHealth","source_connection_ids":["test-phone"],"source_portal":null}]});
        let mut projection = json!(slot);
        projection.as_object_mut().unwrap().remove("snapshot_id");
        projection["fact_ids"] = json!(["steps:completed"]);
        sqlx::query("insert into health_contexts(user_id,snapshot_id,source_boundary_id,generated_at,timezone,facts,projections,source_generations) select $1,$2,$3,now(),'UTC',$4,$5,jsonb_build_object('canonical_evidence_revision',evidence_revision,'apple_health_generation',apple_health_generation,'health_records_generation',health_records_generation) from users where id=$1").bind(account).bind(snapshot).bind(Uuid::new_v4()).bind(json!([fact])).bind(json!([projection])).execute(&mut admin).await.unwrap();
        let admitted = today
            .admit(&c, &slot, "local-test", "Health safety instructions")
            .await
            .unwrap();
        let command: Uuid = serde_json::from_value(admitted["run_id"].clone()).unwrap();
        assert_eq!(
            today
                .admit(&c, &slot, "local-test", "Health safety instructions")
                .await
                .unwrap()["run_id"],
            json!(command)
        );
        let row = sqlx::query(
            "select input,today_assignment from duet_commands where user_id=$1 and id=$2",
        )
        .bind(account)
        .bind(command)
        .fetch_one(&mut admin)
        .await
        .unwrap();
        assert_eq!(row.get::<String, _>("input"), "");
        let assignment: Value = row.get("today_assignment");
        let cycle: Uuid =
            serde_json::from_value(assignment["slots"][0]["cycle_id"].clone()).unwrap();
        // An executed read receipt is not proof its full bytes reached a model.
        // Roll back this adversarial attempt; the real owner below repeats it.
        let mut tx = db.begin(account).await.unwrap();
        identity::check_current(&mut tx, &c, true).await.unwrap();
        let read = Call {
            id: "undelivered".into(),
            name: "read_assessment_fact".into(),
            arguments: json!({"cycle_id":cycle,"fact_id":"steps:completed"}),
        };
        let result = today::effect_in_transaction(&mut tx, account, command, &read)
            .await
            .unwrap()
            .unwrap();
        sqlx::query("insert into duet_tool_effects(user_id,command_id,invocation_id,call,result) values($1,$2,$3,$4,$5)").bind(account).bind(command).bind(&read.id).bind(json!(read)).bind(result).execute(&mut *tx).await.unwrap();
        let stage = Call {
            id: "premature".into(),
            name: "stage_today".into(),
            arguments: json!({"cycle_id":cycle,"outcome":"observations","insights":[{"title":"Not delivered","detail":"Must not publish","read_ids":["undelivered"]}]}),
        };
        assert!(
            today::effect_in_transaction(&mut tx, account, command, &stage)
                .await
                .is_err()
        );
        let forbidden = Call {
            id: "forbidden".into(),
            name: "stage_memory".into(),
            arguments: json!({"content":"unauthorized"}),
        };
        assert!(
            today::effect_in_transaction(&mut tx, account, command, &forbidden)
                .await
                .is_err()
        );
        tx.rollback().await.unwrap();
        let mut provider = LocalProvider {
            cycle,
            phase: 0,
            quiet,
        };
        let mut owner = conversations.claim(&c, command).await.unwrap();
        owner
            .execute(&mut provider, Limits::default(), &Cancellation::default())
            .await
            .unwrap();
        let restored = TodayStore::new(db.clone());
        let ready = restored.status(&c, &slot).await.unwrap();
        assert_eq!(ready["state"], "ready");
        assert_eq!(ready["cycle"]["id"], json!(cycle));
        assert_eq!(
            ready["cycle"]["insights"].as_array().unwrap().len(),
            if quiet { 0 } else { 1 }
        );
        assert_eq!(
            ready["cycle"]["supporting_facts"],
            if quiet { json!([]) } else { json!([fact]) }
        );
        assert!(
            conversations.phone_page(&c, true).await.unwrap()["messages"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        assert!(
            restored
                .admit(&c, &slot, "local-test", "Health safety instructions")
                .await
                .unwrap()["run_id"]
                .is_null()
        );
        // Exercise a real mutation in both cases; [] -> [] is an allowed no-op.
        assert!(
            sqlx::query("update insight_cycles set insights=$3 where user_id=$1 and id=$2")
                .bind(account)
                .bind(cycle)
                .bind(if quiet { json!([{}]) } else { json!([]) })
                .execute(&mut admin)
                .await
                .is_err()
        );
        if !quiet {
            let insight: Uuid =
                serde_json::from_value(ready["cycle"]["insights"][0]["id"].clone()).unwrap();
            restored.correct(&c, cycle, insight).await.unwrap();
            assert_eq!(
                restored.status(&c, &slot).await.unwrap()["cycle"]["corrected_insight_ids"],
                json!([insight])
            );
        }
    }
    drop(conversations);
    drop(today);
    drop(db);
    drop(admin);
    sqlx::query(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut root)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut root)
        .await
        .unwrap();
}

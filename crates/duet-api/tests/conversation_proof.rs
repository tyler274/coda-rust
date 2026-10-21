//! Controlled provider proof through real PostgreSQL accepted-work ownership.
use duet_api::{
    conversation::ConversationStore, database::AccountDatabase, identity::AccountContext,
};
use harness::execution::{
    Call, Cancellation, Limits, Owner, Provider, Request, Response, StreamEvent, Usage,
};
use serde_json::{Value, json};
use sqlx::Connection;
use uuid::Uuid;
async fn retained_context_families(
    store: &ConversationStore,
    db: &AccountDatabase,
    c: &AccountContext,
    admin: &mut sqlx::PgConnection,
) {
    use duet_api::{ApiError, conversation::Submission};
    let boundary = Uuid::new_v4();
    let snapshot = Uuid::new_v4();
    let follow = Uuid::new_v4();
    let cycle = Uuid::new_v4();
    let insight = Uuid::new_v4();
    // Explicit synthetic retained-owner fixtures, not invented runtime producers.
    let fact = json!({"id":"steps:day","kind":"current_value","window":"day","concept_ids":["steps"],"title":"Frozen selected steps","range_start":"2026-10-19T00:00:00Z","range_end":"2026-10-20T00:00:00Z","value":7654.0,"unit":"count","evidence":[{"id":"stat:steps","kind":"stat","title":"Steps","source_kind":"appleHealth","source_connection_ids":["phone-1"]}]});
    sqlx::query("insert into health_contexts(user_id,snapshot_id,source_boundary_id,generated_at,timezone,projections) values($1,$2,$3,now(),'UTC','[{\"kind\":\"daily\"}]')").bind(c.account_id).bind(snapshot).bind(boundary).execute(&mut *admin).await.unwrap();
    sqlx::query("insert into follow_ups(id,user_id,accepted_instruction,schedule,schedule_summary,state,source_message_ids) values($1,$2,'Ask about my recorded walk','{}','Once','paused','{}')").bind(follow).bind(c.account_id).execute(&mut *admin).await.unwrap();
    let observations = json!([{"id":insight,"title":"Retained Today observation","detail":"Original published words","concept_ids":["steps"],"fact_ids":["steps:day"],"evidence":[],"sources":[]}]);
    sqlx::query("insert into insight_cycles(id,user_id,context_snapshot_id,kind,window_start,window_end,timezone,first_weekday,status,insights,supporting_facts,observed_through,generated_at) values($1,$2,$3,'daily','2026-10-19','2026-10-20','UTC',1,'ready',$4,$5,now(),now())").bind(cycle).bind(c.account_id).bind(snapshot).bind(observations).bind(json!([fact])).execute(&mut *admin).await.unwrap();
    let selections = [
        json!({"kind":"measurement","source_boundary_id":boundary,"health_context_revision":1,"source":{"kind":"appleHealth","connection_id":"phone-1"},"measurement_id":"steps","period_start":"2026-10-19T00:00:00Z","period_end":"2026-10-20T00:00:00Z","fact":fact}),
        json!({"kind":"follow_up","id":follow,"revision":1}),
        json!({"kind":"published_insight","cycle_id":cycle,"insight_id":insight}),
    ];
    // Published content cannot be silently rewritten behind a retained citation.
    assert!(
        sqlx::query("update insight_cycles set supporting_facts='[]' where id=$1")
            .bind(cycle)
            .execute(&mut *admin)
            .await
            .is_err()
    );
    let foreign_id = Uuid::new_v4();
    sqlx::query("insert into users(id,auth_user_id,sharing_label,intelligence_enabled,luna_consent_decision,luna_consent_version,terms_version) values($1,$2,'Foreign',true,'accepted','2026-09-24','2026-09-23')").bind(foreign_id).bind(foreign_id.to_string()).execute(&mut *admin).await.unwrap();
    let (incarnation, permission_revision): (Uuid, i64) = sqlx::query_as(
        "select duet_incarnation,intelligence_control_revision from users where id=$1",
    )
    .bind(foreign_id)
    .fetch_one(&mut *admin)
    .await
    .unwrap();
    let foreign = AccountContext {
        account_id: foreign_id,
        incarnation,
        permission_revision,
        authenticated_at: chrono::Utc::now(),
    };
    for selected in &selections {
        let submission = Submission {
            timezone: "UTC".into(),
            selected_context: Some(serde_json::from_value(selected.clone()).unwrap()),
            reply_to_message_id: None,
        };
        assert!(matches!(
            store
                .phone_accept_submission(
                    &foreign,
                    Uuid::new_v4(),
                    "Foreign selection",
                    "controlled",
                    &submission
                )
                .await,
            Err(ApiError::Conflict(_))
        ));
    }
    let mut malformed = selections[0].clone();
    malformed["source"]["connection_id"] = json!("foreign-connection");
    let malformed = Submission {
        timezone: "UTC".into(),
        selected_context: Some(serde_json::from_value(malformed).unwrap()),
        reply_to_message_id: None,
    };
    assert!(matches!(
        store
            .phone_accept_submission(
                c,
                Uuid::new_v4(),
                "Malformed selection",
                "controlled",
                &malformed
            )
            .await,
        Err(ApiError::Unprocessable(_))
    ));
    let mut retained = vec![];
    let mut admitted_ids = vec![];
    for (index, selected) in selections.iter().enumerate() {
        let submission = Submission {
            timezone: "UTC".into(),
            selected_context: Some(serde_json::from_value(selected.clone()).unwrap()),
            reply_to_message_id: None,
        };
        let id = Uuid::new_v4();
        admitted_ids.push(id);
        let accepted = store
            .phone_accept_submission(c, id, "Explain selected context", "controlled", &submission)
            .await
            .unwrap();
        let restored = ConversationStore::new(db.clone());
        assert_eq!(
            restored
                .phone_accept_submission(
                    c,
                    id,
                    "Explain selected context",
                    "controlled",
                    &submission
                )
                .await
                .unwrap()["message"],
            accepted["message"]
        );
        let changed = Submission {
            selected_context: None,
            ..submission.clone()
        };
        assert!(matches!(
            restored
                .phone_accept_submission(c, id, "Explain selected context", "controlled", &changed)
                .await,
            Err(ApiError::Conflict("command identity reused"))
        ));
        let mut owner = restored.claim(c, id).await.unwrap();
        assert!(
            owner
                .dispatch(
                    &call("unauthorized-producer", "stage_today", json!({})),
                    &Cancellation::default()
                )
                .await
                .is_err(),
            "conversation cannot invoke an unadvertised assessment producer"
        );
        if index != 1 {
            let source = accepted["message"]["sources"][0]["id"].as_str().unwrap();
            let detail = restored.message_source(c, id, source).await.unwrap();
            assert!(detail["text"].as_str().unwrap().contains("7654"));
            owner
                .dispatch(
                    &call("selected-read", "read_selected_source", json!({})),
                    &Cancellation::default(),
                )
                .await
                .unwrap();
            owner
                .dispatch(
                    &call(
                        "selected-citation",
                        "cite_health_sources",
                        json!({"read_ids":["selected-read"]}),
                    ),
                    &Cancellation::default(),
                )
                .await
                .unwrap();
            retained.push((id, source.to_string(), detail));
        } else {
            assert!(
                accepted["message"]["sources"]
                    .as_array()
                    .unwrap()
                    .is_empty()
            );
        }
        owner
            .complete(&response("Selected context explained", vec![]))
            .await
            .unwrap();
        if index != 1 {
            assert!(
                restored
                    .message_source(
                        c,
                        Uuid::from_u128(id.as_u128() ^ (1u128 << 127)),
                        "selected-read"
                    )
                    .await
                    .is_ok()
            );
        }
    }
    sqlx::query(
        "update health_contexts set revision=2,source_boundary_id=$2,facts='[]' where user_id=$1",
    )
    .bind(c.account_id)
    .bind(Uuid::new_v4())
    .execute(&mut *admin)
    .await
    .unwrap();
    sqlx::query("update follow_ups set revision=2,state='cancelled' where id=$1")
        .bind(follow)
        .execute(&mut *admin)
        .await
        .unwrap();
    sqlx::query("update insight_cycles set corrected_insight_ids=array[$2::uuid] where id=$1")
        .bind(cycle)
        .bind(insight)
        .execute(&mut *admin)
        .await
        .unwrap();
    assert_eq!(
        store
            .message_source(c, retained[0].0, &retained[0].1)
            .await
            .unwrap(),
        retained[0].2
    );
    assert!(
        store
            .message_source(c, retained[1].0, &retained[1].1)
            .await
            .unwrap()["text"]
            .as_str()
            .unwrap()
            .contains(&insight.to_string())
    );
    for selected in &selections {
        let submission = Submission {
            timezone: "UTC".into(),
            selected_context: Some(serde_json::from_value(selected.clone()).unwrap()),
            reply_to_message_id: None,
        };
        assert!(matches!(
            store
                .phone_accept_submission(
                    c,
                    Uuid::new_v4(),
                    "Stale selection",
                    "controlled",
                    &submission
                )
                .await,
            Err(ApiError::Conflict(_))
        ));
    }
    // Restart/retry preserves command identity even when the original context is stale.
    let restarted = ConversationStore::new(db.clone());
    for (id, selected) in admitted_ids.iter().zip(&selections) {
        let submission = Submission {
            timezone: "UTC".into(),
            selected_context: Some(serde_json::from_value(selected.clone()).unwrap()),
            reply_to_message_id: None,
        };
        restarted
            .phone_accept_submission(
                c,
                *id,
                "Explain selected context",
                "controlled",
                &submission,
            )
            .await
            .unwrap();
    }
    // Restriction survives owner restoration and prevents frozen data resurrection.
    sqlx::query("insert into privacy_recovery_events(id,account_id,operation,scope_id,scope_version,journaled_at) values($1,$2,'device_withdraw',$2,'0',now())").bind(Uuid::new_v4()).bind(c.account_id).execute(&mut *admin).await.unwrap();
    for (id, source, _) in &retained {
        assert!(matches!(
            store.message_source(c, *id, source).await,
            Err(ApiError::NotFound)
        ));
    }
    let page = store.phone_page(c, true).await.unwrap();
    assert!(
        page["messages"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|m| retained.iter().any(|(id, _, _)| m["id"] == json!(id)))
            .all(|m| m["selected_context"].is_null())
    );
}
async fn admission_and_sources(
    store: &ConversationStore,
    db: &AccountDatabase,
    c: &AccountContext,
    address: &duet_api::evidence::AddressedRecordRevision,
    admin: &mut sqlx::PgConnection,
) {
    use duet_api::{ApiError, conversation::Submission};
    let target = Uuid::new_v4();
    store
        .phone_accept(c, target, "Earlier question", "controlled")
        .await
        .unwrap();
    store
        .claim(c, target)
        .await
        .unwrap()
        .complete(&response("語".repeat(321).as_str(), vec![]))
        .await
        .unwrap();
    let id = Uuid::new_v4();
    let submission = Submission {
        timezone: "America/New_York".into(),
        selected_context: Some(serde_json::from_value(json!({"kind":"record","record_id":address.canonical_record_id,"revision":address.revision})).unwrap()),
        reply_to_message_id: Some(Uuid::from_u128(target.as_u128() ^ (1u128 << 127))),
    };
    let accepted = store
        .phone_accept_submission(c, id, "Explain selected record", "controlled", &submission)
        .await
        .unwrap();
    assert_eq!(
        accepted["message"]["selected_context"],
        json!(submission.selected_context)
    );
    assert_eq!(
        accepted["message"]["reply_to"]["text_preview"],
        "語".repeat(320)
    );
    assert_eq!(accepted["message"]["reply_to"]["is_truncated"], true);
    let saved: Value =
        sqlx::query_scalar("select request from duet_commands where user_id=$1 and id=$2")
            .bind(c.account_id)
            .bind(id)
            .fetch_one(&mut *admin)
            .await
            .unwrap();
    assert!(saved.to_string().contains("literal/source/id"));
    let restored = ConversationStore::new(db.clone());
    restored
        .phone_accept_submission(c, id, "Explain selected record", "controlled", &submission)
        .await
        .unwrap();
    for changed in [
        Submission {
            timezone: "UTC".into(),
            ..submission.clone()
        },
        Submission {
            reply_to_message_id: None,
            ..submission.clone()
        },
        Submission {
            selected_context: None,
            ..submission.clone()
        },
    ] {
        assert!(matches!(
            restored
                .phone_accept_submission(c, id, "Explain selected record", "controlled", &changed)
                .await,
            Err(ApiError::Conflict("command identity reused"))
        ));
    }
    let selected_id = accepted["message"]["sources"][0]["id"].as_str().unwrap();
    let detail = restored.message_source(c, id, selected_id).await.unwrap();
    assert_eq!(detail["superseded"], false);
    assert!(detail["text"].as_str().unwrap().contains("<0.03"));
    let mut owner = restored.claim(c, id).await.unwrap();
    owner
        .dispatch(
            &call(
                "retained-read",
                "read_health_source",
                json!({"address":address}),
            ),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    owner
        .dispatch(
            &call(
                "retained-citation",
                "cite_health_sources",
                json!({"read_ids":["retained-read"]}),
            ),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    owner
        .dispatch(
            &call(
                "uncited-read",
                "read_health_source",
                json!({"address":address}),
            ),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    owner
        .complete(&response("Exact retained source", vec![]))
        .await
        .unwrap();
    let assistant = Uuid::from_u128(id.as_u128() ^ (1u128 << 127));
    assert!(
        restored
            .message_source(c, assistant, "uncited-read")
            .await
            .is_err()
    );
    assert_eq!(
        restored
            .message_source(c, assistant, "retained-read")
            .await
            .unwrap()["text"],
        detail["text"]
    );
    assert!(
        restored
            .message_source(c, target, "retained-read")
            .await
            .is_err()
    );
    // A forged or cross-conversation reply cannot be attached, even when canonical elsewhere.
    for reply in [Uuid::new_v4(), id] {
        let mut bad = submission.clone();
        bad.reply_to_message_id = Some(reply);
        let result = restored
            .accept_submission(
                c,
                Uuid::new_v4(),
                Uuid::new_v4(),
                0,
                "Wrong history",
                "controlled",
                &bad,
            )
            .await;
        assert!(matches!(
            result,
            Err(ApiError::Conflict("reply_target_unavailable"))
        ));
    }
    let mut bad = submission.clone();
    bad.selected_context = Some(
        serde_json::from_value(
            json!({"kind":"record","record_id":Uuid::new_v4(),"revision":address.revision}),
        )
        .unwrap(),
    );
    assert!(matches!(
        restored
            .phone_accept_submission(c, Uuid::new_v4(), "Mismatched record", "controlled", &bad)
            .await,
        Err(ApiError::Conflict("selected_context_unavailable"))
    ));
    assert!(matches!(
        restored
            .phone_accept(c, assistant, "Collision", "controlled")
            .await,
        Err(ApiError::Conflict("message identity reused"))
    ));
    let foreign = Uuid::new_v4();
    sqlx::query("insert into users(id,auth_user_id,sharing_label,intelligence_enabled,luna_consent_decision,luna_consent_version,terms_version) values($1,$2,'Foreign',true,'accepted','2026-09-24','2026-09-23')")
        .bind(foreign).bind(foreign.to_string()).execute(&mut *admin).await.unwrap();
    let (incarnation, permission_revision) = sqlx::query_as::<_, (Uuid, i64)>(
        "select duet_incarnation,intelligence_control_revision from users where id=$1",
    )
    .bind(foreign)
    .fetch_one(&mut *admin)
    .await
    .unwrap();
    let foreign_context = AccountContext {
        account_id: foreign,
        incarnation,
        permission_revision,
        authenticated_at: chrono::Utc::now(),
    };
    assert!(matches!(
        restored
            .message_source(&foreign_context, assistant, "retained-read")
            .await,
        Err(ApiError::NotFound)
    ));
    let only_record = Submission {
        reply_to_message_id: None,
        ..submission.clone()
    };
    assert!(matches!(
        restored
            .phone_accept_submission(
                &foreign_context,
                Uuid::new_v4(),
                "Foreign record",
                "controlled",
                &only_record
            )
            .await,
        Err(ApiError::Conflict("selected_context_unavailable"))
    ));
    let only_reply = Submission {
        selected_context: None,
        ..submission.clone()
    };
    assert!(matches!(
        restored
            .phone_accept_submission(
                &foreign_context,
                Uuid::new_v4(),
                "Foreign reply",
                "controlled",
                &only_reply
            )
            .await,
        Err(ApiError::Conflict("reply_target_unavailable"))
    ));
    // Same-command replay is an identity check, not a new admission, even after a target disappears.
    sqlx::query("delete from duet_commands where user_id=$1 and id=$2")
        .bind(c.account_id)
        .bind(target)
        .execute(&mut *admin)
        .await
        .unwrap();
    restored
        .phone_accept_submission(c, id, "Explain selected record", "controlled", &submission)
        .await
        .unwrap();
    let page = restored.phone_page(c, true).await.unwrap();
    let retained = page["messages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["id"] == json!(id))
        .unwrap();
    assert_eq!(
        retained["reply_to_message_id"],
        json!(submission.reply_to_message_id)
    );
    assert!(retained["reply_to"].is_null());
    serde_json::from_value::<duet_wire::models::ConversationPage>(page).unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, Value>(
            "select request from duet_commands where user_id=$1 and id=$2"
        )
        .bind(c.account_id)
        .bind(id)
        .fetch_one(&mut *admin)
        .await
        .unwrap(),
        saved
    );
}

struct Controlled {
    replies: std::collections::VecDeque<Result<Response, String>>,
}
impl Provider for Controlled {
    async fn exchange(
        &mut self,
        _: &Request,
        _: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        emit(StreamEvent::Usage(Usage {
            input_tokens: Some(7),
            output_tokens: Some(3),
        }))?;
        self.replies
            .pop_front()
            .expect("unexpected paid/provider dispatch")
    }
}
fn response(text: &str, calls: Vec<Call>) -> Response {
    Response {
        text: text.into(),
        calls,
        usage: Usage {
            input_tokens: Some(7),
            output_tokens: Some(3),
        },
        provenance: json!({"mode":"controlled","provider":"test"}),
    }
}
fn call(id: &str, name: &str, arguments: Value) -> Call {
    Call {
        id: id.into(),
        name: name.into(),
        arguments,
    }
}
#[tokio::test]
async fn durable_tool_recovery_memory_and_permission_fences() {
    let url = std::env::var("DUET_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
    let opts: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&opts.get_host()));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("duet_execution_{suffix}");
    let role = format!("duet_exec_{suffix}");
    sqlx::query(&format!("create role {role} login"))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(&format!("create database {name}"))
        .execute(&mut admin)
        .await
        .unwrap();
    use sqlx::ConnectOptions;
    let owner_url = opts.clone().database(&name).to_url_lossy().to_string();
    let runtime_url = opts
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    duet_api::database::migrate(&owner_url, &role)
        .await
        .unwrap();
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let mut admin_db = sqlx::PgConnection::connect(&owner_url).await.unwrap();
    let account = Uuid::new_v4();
    sqlx::query("insert into users(id,auth_user_id,sharing_label,intelligence_enabled,luna_consent_decision,luna_consent_version,terms_version) values($1,'controlled-person','Duet',true,'accepted','2026-09-24',$2)").bind(account).bind("2026-09-23").execute(&mut admin_db).await.unwrap();
    let (incarnation, revision): (Uuid, i64) = sqlx::query_as(
        "select duet_incarnation,intelligence_control_revision from users where id=$1",
    )
    .bind(account)
    .fetch_one(&mut admin_db)
    .await
    .unwrap();
    let c = AccountContext {
        account_id: account,
        incarnation,
        permission_revision: revision,
        authenticated_at: chrono::Utc::now(),
    };
    let store = ConversationStore::new(db.clone());
    let id = Uuid::new_v4();
    let conversation = Uuid::new_v4();
    store
        .accept(
            &c,
            id,
            conversation,
            0,
            "Help prepare my own question",
            "controlled",
        )
        .await
        .unwrap();
    store
        .accept(
            &c,
            id,
            conversation,
            0,
            "Help prepare my own question",
            "controlled",
        )
        .await
        .unwrap();
    assert!(
        store
            .accept(&c, id, conversation, 0, "changed", "controlled")
            .await
            .is_err()
    );
    let mut owner = store.claim(&c, id).await.unwrap();
    assert!(store.claim(&c, id).await.is_err());
    let proposal = call(
        "proposal-1",
        "stage_proposal",
        json!({"base_generation":0,"items":[],"page":{"concern":"My own words","questions":["Is this normal?"]},"note":null}),
    );
    let memory = call(
        "memory-1",
        "stage_memory",
        json!({"content":{"preference":"Use my own words"}}),
    );
    let mut provider = Controlled {
        replies: vec![
            Ok(response(
                "",
                vec![
                    call("workspace-read", "read_workspace", json!({})),
                    memory.clone(),
                    proposal.clone(),
                ],
            )),
            Err("connection interrupted".into()),
        ]
        .into(),
    };
    assert!(
        owner
            .execute(&mut provider, Limits::default(), &Cancellation::default())
            .await
            .is_err()
    );
    let proposals: i64 = sqlx::query_scalar("select count(*) from duet_doctor_proposals")
        .fetch_one(&mut admin_db)
        .await
        .unwrap();
    assert_eq!(proposals, 1);
    let memories: i64 = sqlx::query_scalar("select count(*) from duet_private_memory")
        .fetch_one(&mut admin_db)
        .await
        .unwrap();
    assert_eq!(memories, 0);
    // Observed cumulative usage replaces snapshots, never double counts retries.
    let records: Vec<Value> = sqlx::query_scalar(
        "select record from duet_execution_records where command_id=$1 order by ordinal",
    )
    .bind(id)
    .fetch_all(&mut admin_db)
    .await
    .unwrap();
    let records: Vec<harness::execution::Record> = records
        .into_iter()
        .map(|v| serde_json::from_value(v).unwrap())
        .collect();
    let progress = harness::execution::Progress::from_records(&records);
    assert_eq!(progress.calls, 2);
    assert_eq!(progress.known_tokens, 20);
    let mut recovered =
        ConversationStore::new(AccountDatabase::connect(&runtime_url).await.unwrap())
            .claim(&c, id)
            .await
            .unwrap();
    let first = recovered
        .dispatch(&proposal, &Cancellation::default())
        .await
        .unwrap();
    let second = recovered
        .dispatch(&proposal, &Cancellation::default())
        .await
        .unwrap();
    assert_eq!(first, second);
    serde_json::from_value::<duet_wire::models::DoctorPageProposal>(first).unwrap();
    let mut changed = proposal.clone();
    changed.arguments["note"] = json!("changed");
    assert!(
        recovered
            .dispatch(&changed, &Cancellation::default())
            .await
            .is_err()
    );
    let mut provider = Controlled {
        replies: vec![Ok(response("Ready to review; nothing shared.", vec![]))].into(),
    };
    recovered
        .execute(&mut provider, Limits::default(), &Cancellation::default())
        .await
        .unwrap();
    let memories: i64 = sqlx::query_scalar("select count(*) from duet_private_memory")
        .fetch_one(&mut admin_db)
        .await
        .unwrap();
    assert_eq!(memories, 1);
    assert_eq!(
        store.read(&c, conversation).await.unwrap()["commands"][0]["state"],
        "complete"
    );
    let events = store.events(&c, id, 0).await.unwrap();
    assert!(!events.is_empty());
    assert!(
        store
            .events(&c, id, events.last().unwrap()["id"].as_i64().unwrap())
            .await
            .unwrap()
            .is_empty()
    );
    // Crash after an effect but before the next dispatch resumes the retained tool batch.
    let interrupted_id = Uuid::new_v4();
    let interrupted_conversation = Uuid::new_v4();
    store
        .accept(
            &c,
            interrupted_id,
            interrupted_conversation,
            0,
            "Remember a preference",
            "controlled",
        )
        .await
        .unwrap();
    let mut interrupted = store.claim(&c, interrupted_id).await.unwrap();
    let request: Value = sqlx::query_scalar("select request from duet_commands where id=$1")
        .bind(interrupted_id)
        .fetch_one(&mut admin_db)
        .await
        .unwrap();
    interrupted
        .record(harness::execution::Record::Dispatch {
            sequence: 1,
            request: serde_json::from_value(request).unwrap(),
        })
        .await
        .unwrap();
    let remembered = call(
        "stable-memory-effect",
        "stage_memory",
        json!({"content":{"preference":"Brief answers"}}),
    );
    let batch = response("", vec![remembered.clone()]);
    interrupted
        .record(harness::execution::Record::Exchange {
            sequence: 1,
            response: batch,
        })
        .await
        .unwrap();
    interrupted
        .dispatch(&remembered, &Cancellation::default())
        .await
        .unwrap();
    sqlx::query("update duet_commands set lease_until=now()-interval '1 second' where id=$1")
        .bind(interrupted_id)
        .execute(&mut admin_db)
        .await
        .unwrap();
    let mut resumed = store.claim(&c, interrupted_id).await.unwrap();
    assert!(
        interrupted
            .complete(&response("late old lease", vec![]))
            .await
            .is_err()
    );
    let mut controlled = Controlled {
        replies: vec![Ok(response("Remembered", vec![]))].into(),
    };
    resumed
        .execute(&mut controlled, Limits::default(), &Cancellation::default())
        .await
        .unwrap();
    let effects: i64 =
        sqlx::query_scalar("select count(*) from duet_tool_effects where command_id=$1")
            .bind(interrupted_id)
            .fetch_one(&mut admin_db)
            .await
            .unwrap();
    assert_eq!(effects, 1);
    // Existing consumer DTOs deserialize the same accepted owner, not a simulation.
    let phone_id = Uuid::new_v4();
    let acceptance = store
        .phone_accept(&c, phone_id, "Phone message", "controlled")
        .await
        .unwrap();
    serde_json::from_value::<duet_wire::models::ConversationAcceptance>(acceptance).unwrap();
    serde_json::from_value::<duet_wire::models::ConversationPage>(
        store.phone_page(&c, true).await.unwrap(),
    )
    .unwrap();
    // Persisted budget prevents recovery from contacting the provider again.
    let mut phone_owner = store.claim(&c, phone_id).await.unwrap();
    let mut no_calls = Controlled {
        replies: Default::default(),
    };
    let limits = Limits {
        max_calls: 0,
        ..Limits::default()
    };
    assert!(
        phone_owner
            .execute(&mut no_calls, limits, &Cancellation::default())
            .await
            .is_err()
    );
    store
        .claim(&c, phone_id)
        .await
        .unwrap()
        .complete(&response("Phone completed", vec![]))
        .await
        .unwrap();
    // Historical recall is a full completed message, while citations require delivered exact reads.
    let next = Uuid::new_v4();
    store
        .accept(
            &c,
            next,
            conversation,
            1,
            "Recall my earlier question",
            "controlled",
        )
        .await
        .unwrap();
    let mut next_owner = store.claim(&c, next).await.unwrap();
    let history = next_owner
        .dispatch(
            &call("recall", "read_message", json!({"command_id":id})),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(history["input"], "Help prepare my own question");
    assert!(
        next_owner
            .dispatch(
                &call(
                    "bad-cite",
                    "cite_health_sources",
                    json!({"read_ids":["never-read"]})
                ),
                &Cancellation::default()
            )
            .await
            .is_err()
    );
    // Exact citations cannot be forged from search metadata, and source correction fences completion.
    let evidence = duet_api::evidence::EvidenceStore::new(db.clone());
    let mut mutation = duet_api::evidence::EvidenceMutation {
        operation_id: Uuid::new_v4(),
        canonical_record_id: Uuid::new_v4(),
        record_id: "literal/source/id".into(),
        entity: duet_wire::models::addressed_record_revision::Entity::HealthEvidence,
        expected_revision: None,
        source_generation: None,
        payload: json!({"concept_id":"test","measured_at":"2026-10-20T00:00:00Z","source":{"kind":"manual"},"value":{"literal":"<0.03","unit":"mg/dL"}}),
    };
    let address = evidence.ingest(&c, &mutation).await.unwrap();
    let search = next_owner
        .dispatch(
            &call("search-1", "search_health_sources", json!({"text":"test"})),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(search["complete_read"], false);
    assert_eq!(
        search["results"][0]["address"]["record_id"],
        "literal/source/id"
    );
    assert!(
        next_owner
            .dispatch(
                &call(
                    "search-not-citation",
                    "cite_health_sources",
                    json!({"read_ids":["search-1"]})
                ),
                &Cancellation::default()
            )
            .await
            .is_err()
    );
    next_owner
        .dispatch(
            &call("exact-1", "read_health_source", json!({"address":address})),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let cited = next_owner
        .dispatch(
            &call(
                "cite-1",
                "cite_health_sources",
                json!({"read_ids":["exact-1"]}),
            ),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(
        cited["sources"][0]["source"]["payload"]["value"]["literal"],
        "<0.03"
    );
    retained_context_families(&store, &db, &c, &mut admin_db).await;
    admission_and_sources(&store, &db, &c, &address, &mut admin_db).await;
    mutation.operation_id = Uuid::new_v4();
    mutation.expected_revision = Some(i64::from(address.revision));
    mutation.payload["value"]["literal"] = json!("0.04");
    evidence.ingest(&c, &mutation).await.unwrap();
    assert!(
        next_owner
            .complete(&response("stale cited reply", vec![]))
            .await
            .is_err()
    );
    // Shared rate budget is persisted across independent pools.
    let db2 = AccountDatabase::connect(&runtime_url).await.unwrap();
    assert!(db.admit_public_retrieval().await.unwrap());
    sqlx::query("update duet_public_budget set used=600,window_start=now()")
        .execute(&mut admin_db)
        .await
        .unwrap();
    assert!(!db2.admit_public_retrieval().await.unwrap());
    // Withdrawal fences all effects/completion, but late observations remain billable evidence.
    sqlx::query("update users set intelligence_enabled=false,intelligence_control_revision=intelligence_control_revision+1 where id=$1").bind(account).execute(&mut admin_db).await.unwrap();
    assert!(
        next_owner
            .complete(&response("late reply", vec![]))
            .await
            .is_err()
    );
    next_owner
        .record(harness::execution::Record::Observation {
            sequence: 1,
            usage: Usage {
                input_tokens: Some(90),
                output_tokens: Some(10),
            },
            provenance: json!({"late":true}),
        })
        .await
        .unwrap();
    let late: i64 =
        sqlx::query_scalar("select count(*) from duet_execution_records where command_id=$1")
            .bind(next)
            .fetch_one(&mut admin_db)
            .await
            .unwrap();
    assert_eq!(late, 1);
    // Original v1 receipts preserve raw journal bytes while replaying supported restrictions.
    use duet_api::privacy::ReceiptJournal;
    let root = std::env::temp_dir().join(format!("duet-legacy-{suffix}"));
    let journal = std::sync::Arc::new(duet_api::privacy::FileJournal::open(root).await.unwrap());
    let legacy_id = Uuid::new_v4();
    let legacy = json!({"schema_version":1,"event_id":legacy_id,"account_id":account,"operation":"luna_withdraw","scope_id":account,"scope_version":"","occurred_at":chrono::Utc::now(),"state":"restriction_committed"});
    let raw = serde_json::to_vec(&legacy).unwrap();
    journal.put_if_absent(legacy_id, &raw).await.unwrap();
    let privacy = duet_api::privacy::PrivacyStore::new(db.clone(), journal.clone());
    // This account was created after cohort migration. It cannot be enrolled by
    // presenting a legacy receipt; the preexisting-account replay proof lives in evidence_cohort.
    assert!(matches!(
        privacy.replay(legacy_id).await,
        Err(duet_api::ApiError::Unavailable(
            "legacy receipt account incarnation is not admitted by migration"
        ))
    ));
    assert_eq!(journal.read(legacy_id).await.unwrap().unwrap(), raw);
    drop(next_owner);
    drop(recovered);
    drop(owner);
    drop(store);
    drop(db);
    drop(admin_db);
    sqlx::query(&format!("drop database {name} with (force)"))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(&format!("drop role {role}"))
        .execute(&mut admin)
        .await
        .unwrap();
}

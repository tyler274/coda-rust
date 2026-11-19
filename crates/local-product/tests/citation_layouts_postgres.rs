//! Real PostgreSQL integration tests for per-`(source_id, record_type)` citation
//! field layouts (`Citations` Layout 1–5, Code 12–14).
//!
//! Exercise against a disposable Postgres instance with
//! `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` set.

use std::collections::BTreeSet;

use local_product::physician_sources::{
    READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::citation_layouts::CitationLayoutError;
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store, async_trait, engine,
};
use local_product::sha256_hex;
use serde_json::{Value, json};
use uuid::Uuid;

struct AllowTestAuthority;

#[async_trait]
impl Authority for AllowTestAuthority {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn lock_publication(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

struct ScriptedChatProvider {
    turns: Vec<engine::Turn>,
}

impl engine::recipe::Provider for ScriptedChatProvider {
    async fn exchange(
        &mut self,
        _: &engine::recipe::Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn engine::recipe::Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        Ok(self.turns.remove(0))
    }
}

fn database_url() -> String {
    std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .expect("PHYSICIAN_RUNTIME_TEST_DATABASE_URL must be set")
}

fn structured_record(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    record_type: &str,
    payload: &Value,
) -> SourceRevision {
    let content = serde_json::to_string(payload).unwrap();
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: "rev-1".to_owned(),
        record_type: record_type.to_owned(),
        dates: vec![RecordDate {
            name: "recorded".to_owned(),
            value: "2026-11-18".to_owned(),
        }],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts: Vec::new(),
        provenance: json!({
            "title": format!("{record_type} {candidate_id}"),
            "source_label": source_id,
        }),
        envelope: None,
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn citation_layouts_persist_validate_and_drive_readback_without_dropping_or_synthesizing() {
    let store = Store::connect(&database_url()).await.unwrap();
    store.migrate().await.unwrap();

    let practice = format!("prac-{}", Uuid::new_v4().simple());
    let workspace = format!("ws-{}", Uuid::new_v4().simple());
    let physician = "dr-layout".to_owned();
    let source_id = format!("ehr-{}", Uuid::new_v4().simple());
    let other_source = format!("other-{}", Uuid::new_v4().simple());

    let lab_payload = json!({
        "test": "Basic metabolic panel",
        "potassium_mmol_l": 5.4,
        "sodium_mmol_l": 139,
        "notes": null,
        "chloride": { "dataAbsentReason": { "coding": [{ "code": "specimen-hemolyzed" }] } },
        "specimen": { "source": "venous", "hemolyzed": false },
        "fasting": { "absent": true }
    });
    let lab_rev = structured_record(
        &workspace,
        &source_id,
        "obs-k-layout",
        "Observation",
        &lab_payload,
    );

    let other_lab_payload = json!({
        "test": "Lipid panel",
        "potassium_mmol_l": 4.1,
        "ldl_mg_dl": 92
    });
    let other_source_rev = structured_record(
        &workspace,
        &other_source,
        "obs-other-source",
        "Observation",
        &other_lab_payload,
    );

    let narrative_payload = json!({
        "test": "Narrative note record",
        "potassium_mmol_l": 4.2
    });
    let mut narrative_rev = structured_record(
        &workspace,
        &source_id,
        "obs-narrative",
        "Observation",
        &narrative_payload,
    );
    narrative_rev.readable_parts = vec![ReadablePart {
        label: "Clinical interpretation".to_owned(),
        text: "Source supplied its own narrative text: potassium 4.2 mmol/L.".to_owned(),
    }];

    // File sources into the workspace.
    for (src_id, records) in [
        (
            source_id.as_str(),
            vec![lab_rev.clone(), narrative_rev.clone()],
        ),
        (other_source.as_str(), vec![other_source_rev.clone()]),
    ] {
        let pull = SourcePull {
            source_id: src_id.to_owned(),
            source_label: src_id.to_owned(),
            patient: SourcePatient {
                source_patient_id: format!("sp-{src_id}"),
                display_name: "Layout Test Patient".to_owned(),
                birth_date: Some("1970-01-01".to_owned()),
                sex_or_gender: None,
                medical_record_numbers: Vec::new(),
            },
            pulled_at: Some(1_760_000_000),
        };
        store
            .record_chart(&workspace, &records, &pull)
            .await
            .unwrap();
    }

    // Exercise a real PhysicianHost turn citing all three records.
    let scope = Scope {
        practice: practice.clone(),
        physician: physician.clone(),
        workspace: workspace.clone(),
        chat: "chat-layout-proof".to_owned(),
    };
    let binding = local_product::ModelBinding {
        model: "controlled".into(),
        effort: "none".into(),
        adapter: "fixed".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 272_000,
    };
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    let recipe = local_product::physician_chat::recipe(
        &binding,
        local_product::physician_chat::INSTRUCTIONS,
        &skills,
    );
    let input = Input {
        key: "cmd-ask-layout".into(),
        text: "Review the labs and note.".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    store
        .accept_authorized(&scope, &input, vec![0; 32], &skills, &recipe, None)
        .await
        .unwrap();
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: AllowTestAuthority,
        shown_steps: local_product::runtime::ShownSteps::new(()),
    };

    let mut provider = ScriptedChatProvider {
        turns: vec![
            engine::Turn {
                items: vec![engine::Item::Call(engine::Invocation {
                    id: "call-read".into(),
                    name: READ_TOOL.into(),
                    arguments: json!({
                        "references": [
                            { "candidate_id": "obs-k-layout", "revision": "rev-1", "cursor": null },
                            { "candidate_id": "obs-other-source", "revision": "rev-1", "cursor": null },
                            { "candidate_id": "obs-narrative", "revision": "rev-1", "cursor": null },
                        ],
                        "max_result_bytes": 32_768
                    })
                    .to_string(),
                })],
                receipt: json!({"step": 1}),
                usage: Default::default(),
            },
            engine::Turn {
                items: vec![engine::Item::Call(engine::Invocation {
                    id: "call-finish".into(),
                    name: "finish_answer".into(),
                    arguments: json!({
                        "answer": "Potassium is 5.4 mmol/L [S1] while lipid check [S2] and narrative [S3] are on file.",
                        "quotes": [],
                        "descriptions": [],
                        "cites_none": false,
                        "basis": "Reviewed current chart records for layout verification."
                    })
                    .to_string(),
                })],
                receipt: json!({"step": 2}),
                usage: Default::default(),
            },
        ],
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // 1. Layout 3 (general view): no saved layout yet -> general view, plain null omitted, explicit absent shown.
    let readback_before = store.reopen_conversation(&scope).await.unwrap().unwrap();
    let lab_before = readback_before
        .sources
        .values()
        .find(|s| s.candidate_id == "obs-k-layout")
        .expect("lab citation present");
    assert!(lab_before.body.contains("Potassium mmol l: 5.4"));
    assert!(lab_before.body.contains("Chloride: not supplied"));
    assert!(lab_before.body.contains("Fasting: not supplied"));
    assert!(!lab_before.body.contains("Notes"));

    // 2. Layout 5 + Code 12: invalid layouts (extra value key, numeric literal label, unmatched paths)
    // must be rejected without saving anything to Postgres.
    let err_literal = store
        .save_validated_citation_layout(
            &source_id,
            "Observation",
            json!({
                "sections": [{
                    "fields": [{ "path": "potassium_mmol_l", "value": "5.4" }]
                }]
            }),
            Some(&lab_payload),
        )
        .await
        .unwrap_err();
    assert!(matches!(err_literal, CitationLayoutError::InvalidSchema(_)));

    let invalid_candidate = store
        .ensure_citation_layout_for_record(
            &source_id,
            "Observation",
            &lab_payload,
            Some(json!({
                "sections": [{
                    "fields": [{ "path": "potassium_mmol_l", "label": "Potassium 9.9" }]
                }]
            })),
        )
        .await
        .unwrap();
    assert!(invalid_candidate.is_none());
    assert!(
        store
            .citation_layout(&source_id, "Observation")
            .await
            .unwrap()
            .is_none(),
        "failed layout validation must save nothing"
    );

    // 3. Layout 4 + Layout 2 + Code 13 + Code 14: save valid layout for (source_id, "Observation").
    let saved = store
        .save_validated_citation_layout(
            &source_id,
            "Observation",
            json!({
                "sections": [{
                    "heading": "Electrolytes",
                    "fields": [
                        { "path": "potassium_mmol_l", "label": "Potassium (mmol/L)" },
                        { "path": "omitted_comment", "label": "Specimen comment" },
                        { "path": "notes", "label": "Plain null note" },
                        { "path": "chloride", "label": "Chloride" }
                    ]
                }]
            }),
            Some(&lab_payload),
        )
        .await
        .unwrap();
    assert_eq!(saved.key.source_id, source_id);
    assert_eq!(saved.key.record_type, "Observation");

    let readback_after = store.reopen_conversation(&scope).await.unwrap().unwrap();

    // Structured record without readable_parts uses the saved layout (Layout 2).
    let lab_after = readback_after
        .sources
        .values()
        .find(|s| s.candidate_id == "obs-k-layout")
        .unwrap();
    assert!(
        lab_after
            .body
            .starts_with("Electrolytes\nPotassium (mmol/L): 5.4\nChloride: not supplied"),
        "unexpected layout body:\n{}",
        lab_after.body
    );
    // Code 14: omitted keys and plain null never synthesize "not supplied".
    assert!(!lab_after.body.contains("Specimen comment"));
    assert!(!lab_after.body.contains("Plain null note"));
    // Code 13: unplaced fields (sodium, specimen.*, fasting, test) are never dropped.
    assert!(lab_after.body.contains("Sodium mmol l: 139"));
    assert!(lab_after.body.contains("Specimen · Source: venous"));
    assert!(lab_after.body.contains("Specimen · Hemolyzed: No"));
    assert!(lab_after.body.contains("Fasting: not supplied"));
    assert!(lab_after.body.contains("Test: Basic metabolic panel"));

    // Layout 1: narrative readable_parts still take precedence over any saved layout.
    let narrative_after = readback_after
        .sources
        .values()
        .find(|s| s.candidate_id == "obs-narrative")
        .unwrap();
    assert!(
        narrative_after
            .body
            .contains("Source supplied its own narrative text: potassium 4.2 mmol/L.")
    );
    assert!(!narrative_after.body.contains("Electrolytes"));

    // Different source_id stays on general view (Layout 3).
    let other_after = readback_after
        .sources
        .values()
        .find(|s| s.candidate_id == "obs-other-source")
        .unwrap();
    assert!(other_after.body.contains("Potassium mmol l: 4.1"));
    assert!(!other_after.body.contains("Electrolytes"));

    // Auto-generation saves deterministic layout when valid (Layout 4).
    let generated = store
        .ensure_citation_layout_for_record(
            &source_id,
            "MedicationRequest",
            &json!({ "name": "Lisinopril 10 mg", "status": "active" }),
            None,
        )
        .await
        .unwrap()
        .expect("deterministic layout should be generated and saved");
    let placed: BTreeSet<String> = generated
        .spec
        .sections
        .iter()
        .flat_map(|s| s.fields.iter().map(|f| f.path.as_str()))
        .collect();
    assert_eq!(
        placed,
        BTreeSet::from(["name".to_owned(), "status".to_owned()])
    );
}

//! Verification of shared clinical communication guidance ownership, audience isolation,
//! and run-record skill version provenance across Chords and Duet jobs.

#[path = "common/disposable_postgres.rs"]
mod disposable_postgres;
#[path = "../../../verification/model_text.rs"]
mod model_text;

use std::fs;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use harness::engine::history::Role;
use harness::engine::recipe::{Finish, Observer, Provider, Request, RequestBinding, ToolFuture};
use harness::engine::{
    Cancellation, Cause, Event, Invocation, Item, LimitBackstop, Product, ProductMessage,
    RunLimits, Tool, ToolOutcome, Turn, WindowSeed, offer_working_notes,
};
use harness::skills::{
    Package, SKILL_VIEW, SkillLoadOrigin, SkillLoadRecord, SkillTarget, SkillView,
    load_agent_skills, trace_run, version_id,
};
use local_product::ModelBinding;
use local_product::physician_chat::{
    BRIEFING_INSTRUCTIONS, INSTRUCTIONS as CHAT_INSTRUCTIONS, PhysicianJob, PhysicianSkills,
    REVIEW_INSTRUCTIONS, job_recipe, tools as chat_tools,
};
use local_product::runtime::reply::{
    FINISH_TOOL, INSTRUCTIONS as REPLY_INSTRUCTIONS, RevisionContext,
    request as reply_revision_request, requests_finish, revised_text, tools as reply_tools,
};
use model_text::{assert_clean_model_text, assert_skill_tools};
use serde_json::{Value, json};

fn skills_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills")
}

fn fixture_binding(role: &str) -> ModelBinding {
    ModelBinding {
        model: format!("fixture-{role}"),
        context_window_tokens: 272_000,
        max_input_tokens: 250_000,
        effort: "high".into(),
        adapter: "fixture".into(),
    }
}

fn load_duet_package(root: &Path) -> Package {
    Package::load_visible("duet", root, |skill| {
        skill.visibility.admits_explicit("duet")
    })
    .expect("duet skill package loads")
}

fn file_digest(relative: &str) -> String {
    let bytes = fs::read(skills_root().join(relative)).expect("skill file exists");
    version_id(&bytes)
}

async fn execute_run(
    request: Request,
    attempt_id: &str,
    provider: &mut ScriptedProvider,
    host: &mut HarnessJobHost,
) -> Turn {
    host.seed_items = request.items.clone();
    request
        .freeze()
        .expect("request freezes")
        .recover(
            attempt_id,
            harness::engine::recovery::Records {
                events: Vec::new(),
                allowance_after: 0,
            },
            harness::engine::recovery::Messages {
                sources: &[],
                partial: None,
            },
            harness::engine::recovery::Inputs::default(),
        )
        .expect("fresh attempt initializes")
        .run(
            provider,
            host,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("scripted run completes")
}

/// Deterministic scripted provider capturing every materialized `Request` dispatched by `harness`.
struct ScriptedProvider {
    turns: std::collections::VecDeque<Turn>,
    dispatched: Vec<Request>,
}

impl ScriptedProvider {
    fn new(turns: Vec<Turn>) -> Self {
        Self {
            turns: turns.into(),
            dispatched: Vec::new(),
        }
    }
}

impl Provider for ScriptedProvider {
    async fn exchange(
        &mut self,
        request: &Request,
        _cancel: &Cancellation,
        _limits: &RunLimits,
        _observations: &mut dyn Observer,
    ) -> Result<Turn, Cause> {
        self.dispatched.push(request.clone());
        Ok(self
            .turns
            .pop_front()
            .expect("scripted provider has a turn for each request"))
    }
}

enum FinishMode {
    AssistantText,
    ReplyRevision,
}

/// Deterministic in-memory `harness::engine::Product` host recording exact run events.
struct HarnessJobHost {
    package: Package,
    finish_mode: FinishMode,
    seed_items: Vec<Item>,
    events: Vec<Event>,
    committed_reply: Option<String>,
}

impl HarnessJobHost {
    fn assistant(package: Package) -> Self {
        Self {
            package,
            finish_mode: FinishMode::AssistantText,
            seed_items: Vec::new(),
            events: Vec::new(),
            committed_reply: None,
        }
    }

    fn reply_revision(package: Package) -> Self {
        Self {
            package,
            finish_mode: FinishMode::ReplyRevision,
            seed_items: Vec::new(),
            events: Vec::new(),
            committed_reply: None,
        }
    }
}

#[async_trait]
impl Product for HarnessJobHost {
    fn read_result(&mut self, _call_id: &str) -> ToolFuture<Option<String>> {
        Box::pin(async { Ok(None) })
    }

    async fn finish_partial(
        &mut self,
        _turn: &Turn,
        _backstops: &[LimitBackstop],
    ) -> Result<(), Cause> {
        Err("unexpected limit backstop in test fixture".into())
    }

    async fn window_seed(&mut self) -> Result<WindowSeed, Cause> {
        Ok(WindowSeed {
            items: self.seed_items.clone(),
            note_references: Vec::new(),
        })
    }

    async fn message_sources(&mut self) -> Result<Option<Vec<ProductMessage>>, Cause> {
        Ok(None)
    }

    async fn admit_binding(&mut self, _binding: &RequestBinding) -> Result<(), Cause> {
        Ok(())
    }

    async fn assert_current(&mut self) -> Result<(), Cause> {
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), Cause> {
        self.events.push(event);
        Ok(())
    }

    fn terminal(&self, turn: &Turn) -> bool {
        match self.finish_mode {
            FinishMode::AssistantText => false,
            FinishMode::ReplyRevision => requests_finish(turn),
        }
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> ToolFuture {
        let outcome = if call.name == SKILL_VIEW {
            match self.package.execute_skill_view(&arguments) {
                Ok(view) => Ok(ToolOutcome::Returned(view.to_json())),
                Err(reason) => Ok(ToolOutcome::refused(reason.message())),
            }
        } else {
            Ok(ToolOutcome::refused(format!(
                "tool {} is not wired in this guidance fixture",
                call.name
            )))
        };
        Box::pin(std::future::ready(outcome))
    }

    async fn finish(&mut self, candidate: &Turn) -> Result<Finish, Cause> {
        match self.finish_mode {
            FinishMode::AssistantText => Ok(Finish::Committed),
            FinishMode::ReplyRevision => match revised_text(candidate) {
                Ok(text) => {
                    self.committed_reply = Some(text);
                    Ok(Finish::Committed)
                }
                Err(reason) => Ok(local_product::runtime::reply::revision_feedback(reason)),
            },
        }
    }
}

#[test]
fn ac1_shared_rules_have_single_guidance_owner_without_conflicting_copies() {
    let root = skills_root();
    let all_skills = load_agent_skills(&root).expect("all skills load");
    let chords = PhysicianSkills::load(&root).expect("chords skills load");
    let duet = load_duet_package(&root);

    // Audience-specific skills remain distinct rather than collapsed into a single mega-skill.
    assert_eq!(
        all_skills
            .iter()
            .map(|skill| skill.name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "clinical-communication",
            "duet-health-communication",
            "patient-communication",
            "patient-memory-maintenance",
            "patient-record-retrieval",
            "physician-clinical-communication",
        ]
    );

    // Every prompt and packaged skill across both products passes model-text and tool-name checks.
    for prompt in [
        CHAT_INSTRUCTIONS,
        REVIEW_INSTRUCTIONS,
        BRIEFING_INSTRUCTIONS,
        REPLY_INSTRUCTIONS,
    ] {
        assert_clean_model_text(prompt);
    }
    let mut chords_offered = chat_tools().iter().map(Tool::function).collect::<Vec<_>>();
    chords_offered.extend(reply_tools());
    assert_skill_tools(
        &serde_json::to_value(&chords).unwrap(),
        &chords_offered,
        &[],
    );
    let duet_offered = vec![harness::skills::tool().function()];
    assert_skill_tools(&serde_json::to_value(&duet).unwrap(), &duet_offered, &[]);

    // Collect every guidance surface (4 system prompts + every skill body and supporting resource)
    // and verify each shared clinical communication rule has exactly ONE owner: clinical-communication.
    let mut surfaces: Vec<(String, String)> = vec![
        ("prompt:physician_chat".into(), CHAT_INSTRUCTIONS.into()),
        ("prompt:patient_review".into(), REVIEW_INSTRUCTIONS.into()),
        ("prompt:visit_briefing".into(), BRIEFING_INSTRUCTIONS.into()),
        ("prompt:reply_revision".into(), REPLY_INSTRUCTIONS.into()),
    ];
    for skill in &all_skills {
        surfaces.push((format!("skill:{}", skill.name), skill.content.clone()));
        for resource in &skill.resources {
            surfaces.push((
                format!("resource:{}:{}", skill.name, resource.file_path),
                resource.content.clone(),
            ));
        }
    }

    let owners_matching = |needle: &str| -> Vec<String> {
        let lower_needle = needle.to_ascii_lowercase();
        surfaces
            .iter()
            .filter(|(_, text)| text.to_ascii_lowercase().contains(&lower_needle))
            .map(|(owner, _)| owner.clone())
            .collect()
    };

    // 1. Non-device CDS safety framing -> owned solely by clinical-communication.
    assert_eq!(
        owners_matching("not a diagnostic, monitoring, or clinical alarm medical device"),
        vec!["skill:clinical-communication"]
    );
    assert_eq!(
        owners_matching("medical signals or images as a device"),
        vec!["skill:clinical-communication"]
    );

    // 2. Proposed vs accepted care distinction -> owned solely by clinical-communication.
    assert_eq!(
        owners_matching("proposed versus accepted care"),
        vec!["skill:clinical-communication"]
    );
    assert_eq!(
        owners_matching("never turns proposed care into accepted care"),
        vec!["skill:clinical-communication"]
    );

    // 3. Uncertainty, provenance, Duet origin labeling, and four-digit year dates beside conclusions
    //    -> owned solely by clinical-communication (not duplicated in system prompts).
    assert_eq!(
        owners_matching("uncertainty, provenance, and limits beside conclusions"),
        vec!["skill:clinical-communication"]
    );
    assert_eq!(
        owners_matching("via the patient's duet"),
        vec!["skill:clinical-communication"]
    );
    assert_eq!(
        owners_matching("four-digit year"),
        vec!["skill:clinical-communication"]
    );

    // 4. Citation and source presentation alignment (including unavailable vs empty sources)
    //    -> owned solely by clinical-communication (not duplicated in reply or chat prompts).
    assert_eq!(
        owners_matching("citations and source presentation"),
        vec!["skill:clinical-communication"]
    );
    assert_eq!(
        owners_matching("unavailable sources never establish the absence of clinical information"),
        vec!["skill:clinical-communication"]
    );

    // Audience-specific rules remain owned once by their respective audience guidance.
    assert_eq!(
        owners_matching("under 80 words"),
        vec!["skill:physician-clinical-communication"]
    );
    assert_eq!(
        owners_matching("quiet reviews end in updates"),
        vec!["resource:physician-clinical-communication:references/reviews-and-briefings.md"]
    );
    assert_eq!(
        owners_matching("in the physician's voice"),
        vec!["skill:patient-communication"]
    );
    assert_eq!(
        owners_matching("your doctor's plan"),
        vec!["skill:duet-health-communication"]
    );
    // Connected-care boundary: Chords practice messages require physician approval, whereas
    // Duet's `Your doctor's plan` reflects documented care decisions final in the record without
    // physician approval of Duet's wording or a separate patient plan.
    assert_eq!(
        owners_matching("physician-approved"),
        vec!["skill:patient-communication"]
    );
    assert_eq!(
        owners_matching("never treating unsigned drafts as part of the plan"),
        vec!["skill:duet-health-communication"]
    );
}

#[tokio::test]
async fn ac1_representative_physician_and_patient_jobs_record_exact_skill_versions_in_run_records()
{
    let root = skills_root();
    let chords = PhysicianSkills::load(&root).expect("chords skills load");
    let duet = load_duet_package(&root);
    let binding = fixture_binding("ac1");

    let clinical_comm_v = file_digest("clinical-communication/SKILL.md");
    let physician_comm_v = file_digest("physician-clinical-communication/SKILL.md");
    let patient_comm_v = file_digest("patient-communication/SKILL.md");
    let duet_comm_v = file_digest("duet-health-communication/SKILL.md");

    // 1. Representative Physician Conversation Job: starts small, opens clinical + physician skills,
    //    and records exact package and skill SHA-256 versions in its run record.
    let chat_recipe = job_recipe(&binding, PhysicianJob::Conversation, &chords);
    let chat_request = chat_recipe
        .bind(None)
        .request(vec![Item::ProductMessage(ProductMessage {
            id: "msg-1".into(),
            role: Role::User,
            text: "Summarize why her potassium rose and whether to hold lisinopril.".into(),
        })]);
    let mut chat_host = HarnessJobHost::assistant(chords.package().clone());
    let mut chat_provider = ScriptedProvider::new(vec![
        Turn {
            items: vec![
                Item::Call(Invocation {
                    id: "call-shared".into(),
                    name: SKILL_VIEW.into(),
                    arguments: json!({"name": "clinical-communication"}).to_string(),
                }),
                Item::Call(Invocation {
                    id: "call-phys".into(),
                    name: SKILL_VIEW.into(),
                    arguments: json!({"name": "physician-clinical-communication"}).to_string(),
                }),
            ],
            receipt: json!({"id": "resp-chat-1"}),
            usage: Default::default(),
        },
        Turn {
            items: vec![Item::assistant(
                "Potassium rose from 4.4 to 5.3 mmol/L on 2026-11-08 after starting spironolactone.\n\nBasis: 2026-11-08 metabolic panel and active medication list; outside urgent-care labs remain unavailable.",
            )],
            receipt: json!({"id": "resp-chat-2"}),
            usage: Default::default(),
        },
    ]);
    let chat_outcome = execute_run(
        chat_request,
        "attempt-chat-1",
        &mut chat_provider,
        &mut chat_host,
    )
    .await;
    assert!(chat_outcome.text().contains("Basis:"));

    let chat_trace = chords
        .trace_run(&chat_recipe, &chat_host.events)
        .expect("trace physician conversation run");
    assert_eq!(chat_trace.product, "chords");
    assert_eq!(chat_trace.package_version, chords.package().version);
    assert_eq!(
        chat_trace.loads,
        vec![
            SkillLoadRecord {
                target: SkillTarget::body("clinical-communication"),
                origin: SkillLoadOrigin::Discovered {
                    call_id: "call-shared".into(),
                },
                version_id: clinical_comm_v.clone(),
            },
            SkillLoadRecord {
                target: SkillTarget::body("physician-clinical-communication"),
                origin: SkillLoadOrigin::Discovered {
                    call_id: "call-phys".into(),
                },
                version_id: physician_comm_v.clone(),
            },
        ]
    );

    // 2. Representative Chords Patient Reply Revision Job: preloads shared + patient-communication
    //    at start, finishes via finish_reply_revision, and records exact preloaded versions.
    let revision_context = RevisionContext {
        saved_draft: "We received your lab results and are reviewing them.".into(),
        instruction: ProductMessage {
            id: "rev-1:instruction".into(),
            role: Role::User,
            text: "Keep it warm and note that we are still reviewing whether to adjust her dose."
                .into(),
        },
        expected_draft_revision: 1,
        physician_time_zone: "America/Los_Angeles".into(),
    };
    let reply_req = reply_revision_request(&binding, &revision_context, &[], &chords);
    let reply_recipe = reply_req.recipe();
    let mut reply_host = HarnessJobHost::reply_revision(chords.package().clone());
    let mut reply_provider = ScriptedProvider::new(vec![Turn {
        items: vec![Item::Call(Invocation {
            id: "call-finish-reply".into(),
            name: FINISH_TOOL.into(),
            arguments: json!({
                "text": "We received your potassium result from November 8, 2026. Dr. Patel is reviewing whether any change to your medication is needed, and our office will follow up with next steps.",
                "review_basis": "States only that the 2026-11-08 result was received and dose adjustment remains under physician review; makes no unapproved medication change or timeline promise."
            })
            .to_string(),
        })],
        receipt: json!({"id": "resp-reply-1"}),
        usage: Default::default(),
    }]);
    execute_run(
        reply_req,
        "attempt-reply-1",
        &mut reply_provider,
        &mut reply_host,
    )
    .await;
    assert!(reply_host.committed_reply.is_some());

    let reply_trace = chords
        .trace_run(&reply_recipe, &reply_host.events)
        .expect("trace reply revision run");
    assert_eq!(reply_trace.package_version, chords.package().version);
    assert_eq!(
        reply_trace.loads,
        vec![
            SkillLoadRecord {
                target: SkillTarget::body("clinical-communication"),
                origin: SkillLoadOrigin::Preloaded,
                version_id: clinical_comm_v.clone(),
            },
            SkillLoadRecord {
                target: SkillTarget::body("patient-communication"),
                origin: SkillLoadOrigin::Preloaded,
                version_id: patient_comm_v,
            },
        ]
    );

    // 3. Representative Direct Patient Job (Duet): preloads clinical-communication, discovers
    //    duet-health-communication, refuses physician skills as UnknownSkill, and records versions.
    let duet_targets = vec![SkillTarget::body("clinical-communication")];
    let duet_preload = duet
        .preload(&duet_targets)
        .expect("duet preloads shared clinical communication");
    let mut duet_instructions = format!(
        "Explain the person's health records clearly.\n\n{}\n\n{}",
        duet_preload.instructions,
        duet.catalog_instructions()
    );
    let mut duet_tools = vec![harness::skills::tool().function()];
    offer_working_notes(&mut duet_instructions, &mut duet_tools);
    let duet_recipe = harness::engine::recipe::RequestRecipe {
        model: binding.model.clone(),
        context_window_tokens: binding.context_window_tokens,
        max_input_tokens: binding.max_input_tokens,
        options: json!({"effort": binding.effort}),
        instructions: duet_instructions,
        tools: duet_tools,
        preloaded_skills: duet_targets,
    };
    let duet_request = duet_recipe
        .bind(None)
        .request(vec![Item::ProductMessage(ProductMessage {
            id: "duet-msg-1".into(),
            role: Role::User,
            text: "What does my doctor's plan say about my lipid panel?".into(),
        })]);
    let mut duet_host = HarnessJobHost::assistant(duet.clone());
    let mut duet_provider = ScriptedProvider::new(vec![
        Turn {
            items: vec![
                Item::Call(Invocation {
                    id: "duet-open-own".into(),
                    name: SKILL_VIEW.into(),
                    arguments: json!({"name": "duet-health-communication"}).to_string(),
                }),
                Item::Call(Invocation {
                    id: "duet-try-physician".into(),
                    name: SKILL_VIEW.into(),
                    arguments: json!({"name": "physician-clinical-communication"}).to_string(),
                }),
            ],
            receipt: json!({"id": "resp-duet-1"}),
            usage: Default::default(),
        },
        Turn {
            items: vec![Item::assistant(
                "Your doctor's plan from Dr. Patel's signed visit note on October 18, 2026 lists repeating your lipid panel in 8 weeks (status: open).",
            )],
            receipt: json!({"id": "resp-duet-2"}),
            usage: Default::default(),
        },
    ]);
    execute_run(
        duet_request,
        "attempt-duet-1",
        &mut duet_provider,
        &mut duet_host,
    )
    .await;

    // Verify the attempted read of physician-clinical-communication returned unavailable with only Duet's catalog.
    assert_eq!(
        duet.execute_skill_view(&json!({"name": "physician-clinical-communication"}))
            .unwrap(),
        SkillView::UnknownSkill {
            name: "physician-clinical-communication".into(),
            file_path: None,
            available_skills: vec![
                "clinical-communication".into(),
                "duet-health-communication".into(),
            ],
        }
    );

    let duet_trace = trace_run(&duet, &duet_recipe, &duet_host.events).expect("trace duet run");
    assert_eq!(duet_trace.product, "duet");
    assert_eq!(duet_trace.package_version, duet.version);
    assert_eq!(
        duet_trace.loads,
        vec![
            SkillLoadRecord {
                target: SkillTarget::body("clinical-communication"),
                origin: SkillLoadOrigin::Preloaded,
                version_id: clinical_comm_v,
            },
            SkillLoadRecord {
                target: SkillTarget::body("duet-health-communication"),
                origin: SkillLoadOrigin::Discovered {
                    call_id: "duet-open-own".into(),
                },
                version_id: duet_comm_v,
            },
        ]
    );
}

#[tokio::test]
async fn ac2_initial_contexts_and_discovered_traces_isolate_audience_guidance_across_workflows() {
    let root = skills_root();
    let chords = PhysicianSkills::load(&root).expect("chords skills load");
    let duet = load_duet_package(&root);
    let binding = fixture_binding("ac2");

    let conversation = job_recipe(&binding, PhysicianJob::Conversation, &chords);
    let review = job_recipe(&binding, PhysicianJob::PatientReview, &chords);
    let reply = job_recipe(&binding, PhysicianJob::ReplyRevision, &chords);
    let briefing = job_recipe(&binding, PhysicianJob::VisitBriefing, &chords);

    // 1. Initial context inspection across conversation, reviews, replies, briefings, and Duet:
    // Conversation starts small: metadata-only catalog, zero preloaded bodies, omitted JSON key when empty.
    assert!(conversation.preloaded_skills.is_empty());
    assert!(
        serde_json::to_value(&conversation)
            .unwrap()
            .get("preloaded_skills")
            .is_none()
    );
    assert!(!conversation.instructions.contains("--- Preloaded skill:"));
    assert!(
        !conversation
            .instructions
            .contains("# Clinical Communication")
    );
    assert!(
        !conversation
            .instructions
            .contains("# Patient Communication")
    );
    assert!(
        !conversation
            .instructions
            .contains("# Physician Clinical Communication")
    );
    assert!(
        !conversation
            .instructions
            .contains("duet-health-communication")
    );

    // Reviews and Briefings preload shared + physician clinical communication + reviews-and-briefings
    // reference, WITHOUT forcing patient-communication or duet-health-communication into initial context.
    assert_eq!(
        review.preloaded_skills,
        PhysicianJob::PatientReview.preloaded_targets()
    );
    assert_eq!(
        briefing.preloaded_skills,
        PhysicianJob::VisitBriefing.preloaded_targets()
    );
    assert_eq!(
        reply.preloaded_skills,
        PhysicianJob::ReplyRevision.preloaded_targets()
    );
    // Mismatched package and recipe preloads fail explicitly rather than reading as empty.
    assert_eq!(
        trace_run(&duet, &reply, &[]),
        Err(harness::skills::PreloadError::UnknownSkill {
            product: "duet".into(),
            name: "patient-communication".into(),
        })
    );
    for physician_bg in [&review, &briefing] {
        assert!(
            physician_bg
                .instructions
                .contains("--- Preloaded skill: clinical-communication ---")
        );
        assert!(
            physician_bg
                .instructions
                .contains("--- Preloaded skill: physician-clinical-communication ---")
        );
        assert!(physician_bg.instructions.contains(
            "--- Preloaded skill resource: physician-clinical-communication (references/reviews-and-briefings.md) ---"
        ));
        assert!(
            !physician_bg
                .instructions
                .contains("--- Preloaded skill: patient-communication ---")
        );
        assert!(
            !physician_bg
                .instructions
                .contains("# Patient Communication")
        );
        assert!(
            !physician_bg
                .instructions
                .contains("duet-health-communication")
        );
    }

    // Patient Reply Revision preloads shared + patient-communication, WITHOUT forcing physician
    // synthesis or review/briefing guidance into initial context.
    assert!(
        reply
            .instructions
            .contains("--- Preloaded skill: clinical-communication ---")
    );
    assert!(
        reply
            .instructions
            .contains("--- Preloaded skill: patient-communication ---")
    );
    assert!(
        !reply
            .instructions
            .contains("--- Preloaded skill: physician-clinical-communication ---")
    );
    assert!(
        !reply
            .instructions
            .contains("# Physician Clinical Communication")
    );
    assert!(
        !reply
            .instructions
            .contains("# Patient Reviews and Pre-Visit Briefings")
    );
    assert!(!reply.instructions.contains("duet-health-communication"));

    // Duet catalog excludes Chords-only skills completely.
    let duet_catalog = duet.catalog_instructions();
    assert!(duet_catalog.contains("clinical-communication"));
    assert!(duet_catalog.contains("duet-health-communication"));
    assert!(!duet_catalog.contains("physician-clinical-communication"));
    assert!(!duet_catalog.contains("patient-communication"));
    assert!(!duet_catalog.contains("patient-record-retrieval"));

    // 2. Discovered-skill traces:
    // (a) A VisitBriefing run completes using its preloaded physician guidance without ever loading patient-communication.
    let briefing_req = briefing
        .bind(None)
        .request(vec![Item::ProductMessage(ProductMessage {
            id: "briefing-input-1".into(),
            role: Role::User,
            text: "Prepare tomorrow's visit briefing for Elena.".into(),
        })]);
    let mut briefing_host = HarnessJobHost::assistant(chords.package().clone());
    let mut briefing_provider = ScriptedProvider::new(vec![Turn {
        items: vec![Item::assistant(
            "Orientation: Follow-up visit focused on blood-pressure trajectory and statin tolerance since October 2026.\n\nRecords as of 2026-11-11.",
        )],
        receipt: json!({"id": "resp-briefing-1"}),
        usage: Default::default(),
    }]);
    execute_run(
        briefing_req,
        "attempt-briefing-1",
        &mut briefing_provider,
        &mut briefing_host,
    )
    .await;
    assert_eq!(briefing_provider.dispatched.len(), 1);
    assert!(
        !briefing_provider.dispatched[0]
            .instructions
            .contains("# Patient Communication")
    );
    let briefing_trace = chords
        .trace_run(&briefing, &briefing_host.events)
        .expect("trace briefing run");
    assert_eq!(
        briefing_trace
            .loads
            .iter()
            .map(|load| (
                load.target.skill_name(),
                load.target.resource_path(),
                &load.origin
            ))
            .collect::<Vec<_>>(),
        vec![
            ("clinical-communication", None, &SkillLoadOrigin::Preloaded),
            (
                "physician-clinical-communication",
                None,
                &SkillLoadOrigin::Preloaded
            ),
            (
                "physician-clinical-communication",
                Some("references/reviews-and-briefings.md"),
                &SkillLoadOrigin::Preloaded
            ),
        ]
    );

    // (b) A PatientReview run starts without patient-communication in its initial window, then
    //     discovers patient-communication via skill_view when a message requires a reply draft.
    let review_req = review
        .bind(None)
        .request(vec![Item::ProductMessage(ProductMessage {
            id: "review-input-1".into(),
            role: Role::User,
            text: "New patient message asking whether to stop atorvastatin for mild muscle ache."
                .into(),
        })]);
    let mut review_host = HarnessJobHost::assistant(chords.package().clone());
    let mut review_provider = ScriptedProvider::new(vec![
        Turn {
            items: vec![Item::Call(Invocation {
                id: "discover-patient-comm".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({"name": "patient-communication"}).to_string(),
            })],
            receipt: json!({"id": "resp-review-1"}),
            usage: Default::default(),
        },
        Turn {
            items: vec![Item::assistant(
                "Today concern (today): Patient reports mild myalgia on atorvastatin 20 mg; review CK order and approve reply draft.",
            )],
            receipt: json!({"id": "resp-review-2"}),
            usage: Default::default(),
        },
    ]);
    execute_run(
        review_req,
        "attempt-review-1",
        &mut review_provider,
        &mut review_host,
    )
    .await;

    // Turn 1 initial context had zero patient-communication prose; Turn 2 received it via MaterializedResult.
    assert_eq!(review_provider.dispatched.len(), 2);
    assert!(
        !review_provider.dispatched[0]
            .instructions
            .contains("# Patient Communication")
    );
    assert!(review_provider.dispatched[1].items.iter().any(|item| {
        matches!(
            item,
            Item::MaterializedResult { call_id, output }
                if call_id == "discover-patient-comm" && output.contains("# Patient Communication")
        )
    }));

    let review_trace = chords
        .trace_run(&review, &review_host.events)
        .expect("trace review run");
    assert_eq!(
        review_trace.loads,
        vec![
            SkillLoadRecord {
                target: SkillTarget::body("clinical-communication"),
                origin: SkillLoadOrigin::Preloaded,
                version_id: file_digest("clinical-communication/SKILL.md"),
            },
            SkillLoadRecord {
                target: SkillTarget::body("physician-clinical-communication"),
                origin: SkillLoadOrigin::Preloaded,
                version_id: file_digest("physician-clinical-communication/SKILL.md"),
            },
            SkillLoadRecord {
                target: SkillTarget::resource(
                    "physician-clinical-communication",
                    "references/reviews-and-briefings.md"
                ),
                origin: SkillLoadOrigin::Preloaded,
                version_id: file_digest(
                    "physician-clinical-communication/references/reviews-and-briefings.md"
                ),
            },
            SkillLoadRecord {
                target: SkillTarget::body("patient-memory-maintenance"),
                origin: SkillLoadOrigin::Preloaded,
                version_id: file_digest("patient-memory-maintenance/SKILL.md"),
            },
            SkillLoadRecord {
                target: SkillTarget::body("patient-communication"),
                origin: SkillLoadOrigin::Discovered {
                    call_id: "discover-patient-comm".into(),
                },
                version_id: file_digest("patient-communication/SKILL.md"),
            },
        ]
    );
}

#[tokio::test]
async fn durable_postgres_run_records_preserve_frozen_skill_versions_for_chat_and_reply_revision() {
    use local_product::runtime::{
        Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store,
        reply::{RevisionAccepted, RevisionInput},
    };

    struct OpenAuthority;
    #[async_trait]
    impl Authority for OpenAuthority {
        async fn check(
            &mut self,
            _tx: &PublicationTransaction,
            _scope: &Scope,
            _session: Option<&[u8]>,
        ) -> Result<(), String> {
            Ok(())
        }
        async fn lock_publication(
            &mut self,
            _tx: &PublicationTransaction,
            _scope: &Scope,
            _session: Option<&[u8]>,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    let Some(pg) = disposable_postgres::DisposablePostgres::acquire("clinical-comm-pg").await
    else {
        return;
    };
    let store = pg.store().await;

    let chords = PhysicianSkills::load(&skills_root()).expect("chords skills load");
    let binding = fixture_binding("pg");

    // 1. Durable physician conversation run persisted in Postgres.
    let chat_scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "dr-patel".into(),
        workspace: "patient-elena".into(),
        chat: "chat-guidance".into(),
    };
    let chat_recipe = job_recipe(&binding, PhysicianJob::Conversation, &chords);
    store
        .accept_authorized(
            &chat_scope,
            &Input {
                key: "chat-cmd-1".into(),
                text: "Review potassium trend and options.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            vec![9; 32],
            &chords,
            &chat_recipe,
        )
        .await
        .expect("accept chat turn");
    let claim = store
        .claim(&chat_scope, 60)
        .await
        .expect("claim chat")
        .expect("chat claim present");
    let mut provider = ScriptedProvider::new(vec![
        Turn {
            items: vec![
                Item::Call(Invocation {
                    id: "pg-shared".into(),
                    name: SKILL_VIEW.into(),
                    arguments: json!({"name": "clinical-communication"}).to_string(),
                }),
                Item::Call(Invocation {
                    id: "pg-physician".into(),
                    name: SKILL_VIEW.into(),
                    arguments: json!({"name": "physician-clinical-communication"}).to_string(),
                }),
            ],
            receipt: json!({"id": "pg-resp-1"}),
            usage: Default::default(),
        },
        Turn {
            items: vec![Item::assistant(
                "Potassium is 5.3 mmol/L on 2026-11-08.\n\nBasis: 2026-11-08 panel; outside urgent-care labs remain unavailable.",
            )],
            receipt: json!({"id": "pg-resp-2"}),
            usage: Default::default(),
        },
    ]);
    PhysicianHost {
        store: store.clone(),
        claim,
        authority: OpenAuthority,
    }
    .run(
        &mut provider,
        RunLimits::default(),
        &Cancellation::default(),
    )
    .await
    .expect("durable physician chat completes");

    let reopened = Store::connect(pg.url()).await.expect("reopen store");
    let saved_chat = reopened
        .read(&chat_scope)
        .await
        .expect("read chat")
        .expect("saved conversation");
    let frozen_chat_skills = reopened
        .frozen_skills(&chat_scope, &saved_chat.skills)
        .await
        .expect("load frozen chat skills");
    let frozen_chat_binding = reopened
        .frozen_binding(
            &chat_scope,
            saved_chat.bindings.get(&saved_chat.work).unwrap(),
        )
        .await
        .expect("load frozen chat binding");
    assert!(frozen_chat_binding.recipe.preloaded_skills.is_empty());
    let durable_chat_trace = frozen_chat_skills
        .trace_run(&frozen_chat_binding.recipe, &saved_chat.attempts[0].events)
        .expect("trace durable chat run");
    assert_eq!(durable_chat_trace.package_version, chords.package().version);
    assert_eq!(
        durable_chat_trace.loads,
        vec![
            SkillLoadRecord {
                target: SkillTarget::body("clinical-communication"),
                origin: SkillLoadOrigin::Discovered {
                    call_id: "pg-shared".into(),
                },
                version_id: file_digest("clinical-communication/SKILL.md"),
            },
            SkillLoadRecord {
                target: SkillTarget::body("physician-clinical-communication"),
                origin: SkillLoadOrigin::Discovered {
                    call_id: "pg-physician".into(),
                },
                version_id: file_digest("physician-clinical-communication/SKILL.md"),
            },
        ]
    );

    // 2. Durable reply revision accepted in Postgres freezes preloaded patient + shared skill versions.
    let reply_scope = Scope {
        practice: chat_scope.practice.clone(),
        physician: "dr-patel".into(),
        workspace: "patient-elena".into(),
        chat: "reply-guidance".into(),
    };
    reopened
        .open_draft_authorized(
            &reply_scope,
            "We received your lab results.",
            &[7; 32],
            &mut OpenAuthority,
        )
        .await
        .expect("open draft")
        .expect("authorized draft");
    let reply_recipe = job_recipe(&binding, PhysicianJob::ReplyRevision, &chords);
    assert_eq!(
        reopened
            .accept_revision(
                &reply_scope,
                &RevisionInput {
                    key: "rev-pg-1".into(),
                    expected_revision: 1,
                    instruction: "Say we are reviewing her medication dose.".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                vec![7; 32],
                &chords,
                &reply_recipe,
            )
            .await
            .expect("accept revision"),
        RevisionAccepted::New
    );
    let saved_draft = reopened
        .draft(&reply_scope)
        .await
        .expect("read draft")
        .expect("draft exists");
    let accepted = saved_draft.accepted.as_ref().expect("accepted revision");
    let frozen_reply_skills = reopened
        .frozen_skills(&reply_scope, &accepted.skills)
        .await
        .expect("load frozen reply skills");
    let frozen_reply_binding = reopened
        .frozen_binding(&reply_scope, &accepted.binding)
        .await
        .expect("load frozen reply binding");
    assert_eq!(
        frozen_reply_binding.recipe.preloaded_skills,
        PhysicianJob::ReplyRevision.preloaded_targets()
    );
    let durable_reply_trace = frozen_reply_skills
        .trace_run(&frozen_reply_binding.recipe, &[])
        .expect("trace durable reply revision");
    assert_eq!(
        durable_reply_trace.package_version,
        chords.package().version
    );
    assert_eq!(
        durable_reply_trace.loads,
        vec![
            SkillLoadRecord {
                target: SkillTarget::body("clinical-communication"),
                origin: SkillLoadOrigin::Preloaded,
                version_id: file_digest("clinical-communication/SKILL.md"),
            },
            SkillLoadRecord {
                target: SkillTarget::body("patient-communication"),
                origin: SkillLoadOrigin::Preloaded,
                version_id: file_digest("patient-communication/SKILL.md"),
            },
        ]
    );
}

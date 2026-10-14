//! Local practice simulation engine, connected-walk invariant guards, and investor metric.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Jake Smith's workspace ID in the synthetic practice.
pub const JAKE_WORKSPACE_ID: &str = "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d";
/// Dan Cohen's workspace ID in the synthetic practice.
pub const DAN_WORKSPACE_ID: &str = "pw-f5404d89-acde-4bb5-8c42-2fa191766811";
/// Tanvir Chowdhury's workspace ID in the synthetic practice.
pub const TANVIR_WORKSPACE_ID: &str = "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee";
/// Teresa Hayes's workspace ID in the synthetic practice.
pub const TERESA_WORKSPACE_ID: &str = "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971";
/// Mary Louis's workspace ID in the synthetic practice.
pub const MARY_WORKSPACE_ID: &str = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";

/// Number of patients on Roger's synthetic practice panel.
pub const ROGER_PANEL_SIZE: u32 = 5;
/// Default scenario loaded when starting the local simulation.
pub const DEFAULT_SIMULATION_SCENARIO: &str = "connected-walk";

/// Whether a medication recommendation has been decided by the physician.
///
/// Undecided recommendations must never be promoted into the active patient plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanDecisionState {
    /// Recommendation has not yet been decided by the physician.
    Undecided,
    /// Physician approved the recommendation into the active plan.
    Decided,
    /// Physician declined the recommendation.
    Declined,
}

/// A medication recommendation surfaced from a clinical note or model review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MedicationRecommendation {
    /// Stable recommendation identifier.
    pub recommendation_id: String,
    /// Patient workspace this recommendation belongs to.
    pub patient_workspace_id: String,
    /// Medication name and dose.
    pub medication: String,
    /// Clinical rationale or context.
    pub rationale: String,
    /// Source note or review label.
    pub source_label: String,
    /// Current physician decision state.
    pub decision_state: PlanDecisionState,
}

/// An item in the patient's active care plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedPlanItem {
    /// Stable plan item identifier.
    pub item_id: String,
    /// Patient workspace this plan item belongs to.
    pub patient_workspace_id: String,
    /// Short plan item title.
    pub title: String,
    /// Clinical detail for the plan item.
    pub detail: String,
    /// ISO-8601 timestamp when the item was decided.
    pub decided_at: String,
    /// Recommendation identifier if promoted from a recommendation.
    #[serde(default)]
    pub source_recommendation_id: Option<String>,
}

/// Status of a laboratory observation in the source feed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LabObservationStatus {
    /// Final verified result.
    Final,
    /// Amended/corrected result replacing an earlier value.
    Corrected,
    /// Erroneous result withdrawn (`entered-in-error`).
    EnteredInError,
    /// Tombstoned/deleted observation.
    Tombstoned,
}

/// One observation inside a laboratory report event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabObservationEvent {
    /// Source observation identifier.
    pub observation_id: String,
    /// Display name of the analyte.
    pub display_name: String,
    /// Formatted value and units.
    pub value: String,
    /// Observation status.
    pub status: LabObservationStatus,
}

/// A laboratory report event admitted into a patient workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabReportEvent {
    /// Simulation input key.
    pub input_key: String,
    /// Diagnostic report identifier.
    pub report_id: String,
    /// Patient workspace receiving the report.
    pub patient_workspace_id: String,
    /// Report version number.
    pub report_version: u32,
    /// Report status (`final` or `corrected`).
    pub report_status: String,
    /// Effective timestamp of the report.
    pub effective_at: String,
    /// Observations contained in the report.
    pub observations: Vec<LabObservationEvent>,
}

/// Lifecycle state of a drafted patient reply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplyLifecycleState {
    /// Active reply ready for the physician.
    Ready,
    /// Automatically superseded and cleared after corrected labs invalidated its basis.
    SupersededByCorrection,
    /// Closed or sent by the physician.
    ClosedByPhysician,
}

/// A patient reply drafted during a simulation run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedReply {
    /// Reply work identifier.
    pub reply_work_id: String,
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Patient display name.
    pub patient_display_name: String,
    /// Outbound channel (`spruce` or `email`).
    pub channel: String,
    /// Summary headline shown on Today and in the workspace.
    pub headline: String,
    /// Drafted message body.
    pub draft_text: String,
    /// Urgency tier (`urgent_ed` or `routine`).
    pub urgency: String,
    /// Timestamp when the reply was drafted.
    pub created_at: String,
    /// Diagnostic report ID this reply was based on.
    #[serde(default)]
    pub based_on_report_id: Option<String>,
    /// Specific observation ID this reply was based on.
    #[serde(default)]
    pub based_on_observation_id: Option<String>,
    /// Value that triggered the reply.
    #[serde(default)]
    pub based_on_value: Option<String>,
    /// Current lifecycle state of the reply.
    pub state: ReplyLifecycleState,
    /// Input key that superseded this reply, if any.
    #[serde(default)]
    pub superseded_by_input: Option<String>,
    /// Explanation of why this reply was superseded.
    #[serde(default)]
    pub superseded_reason: Option<String>,
}

/// Investor proof metric: items the physician touched per 100 patients per week.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestorMetric {
    /// Count of items directly touched by the physician during the run.
    pub physician_touched_items: u32,
    /// Count of clinical episodes/reviews handled ambiently by Coda without physician touch.
    pub ambient_handled_items: u32,
    /// Number of patients on the simulated panel.
    pub panel_size: u32,
    /// Simulated duration in weeks.
    pub simulated_weeks: f64,
    /// Normalized rate: `(physician_touched_items / (panel_size * simulated_weeks)) * 100.0`.
    pub items_per_100_patients_per_week: f64,
}

/// Aggregate investor metric across all simulation runs in the session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationAggregateMetric {
    /// Total simulation runs executed in this session.
    pub runs_count: u32,
    /// Total physician-touched items across all runs.
    pub total_physician_touched_items: u32,
    /// Total ambiently handled items across all runs.
    pub total_ambient_handled_items: u32,
    /// Total patient-weeks simulated across all runs.
    pub total_patient_weeks: f64,
    /// Aggregate items the physician touched per 100 patients per week across runs.
    pub items_per_100_patients_per_week: f64,
}

/// Summary of one synthetic patient on Roger's panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedPanelPatient {
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Patient display name.
    pub display_name: String,
    /// Date of birth (`YYYY-MM-DD`).
    pub birth_date: String,
    /// Medical record number.
    pub mrn: String,
    /// Clinical role in the simulation scenarios.
    pub role_summary: String,
    /// Whether the simulated patient takes no unprompted action.
    pub passive_patient: bool,
}

/// Metadata describing one rerunnable simulation scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationScenarioInfo {
    /// Scenario key (`connected-walk`, `reviewability`, etc.).
    pub key: String,
    /// Short human-readable scenario title for UI buttons.
    pub label: String,
    /// Detailed description of what the scenario exercises.
    pub description: String,
    /// Simulated horizon in days.
    pub simulated_days: u32,
}

/// One graded invariant check evaluated at the end of a simulation run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationCheckResult {
    /// Check identifier.
    pub key: String,
    /// Description of the invariant checked.
    pub description: String,
    /// Whether the check passed.
    pub passed: bool,
    /// Evidence detail for the check result.
    pub detail: String,
}

/// Replayable record of a completed simulation run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationRunRecord {
    /// Unique run identifier.
    pub run_id: String,
    /// Scenario key executed.
    pub scenario_key: String,
    /// Human-readable scenario label.
    pub scenario_label: String,
    /// Initial simulation clock (`ISO-8601`).
    pub initial_simulation_time: String,
    /// Final simulation clock (`ISO-8601`).
    pub final_simulation_time: String,
    /// Ordered step log for replay and inspection.
    pub steps_executed: Vec<String>,
    /// Graded invariant checks for the run.
    pub checks: Vec<SimulationCheckResult>,
    /// Investor metric computed for this run.
    pub investor_metric: InvestorMetric,
    /// Whether all invariant checks passed.
    pub passed: bool,
}

/// Per-workspace simulated clinical state layered over the recorded fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedWorkspaceOverlay {
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Current simulated clock (`ISO-8601`).
    pub current_time: String,
    /// Clinical synopsis update appended from simulated episodes.
    pub synopsis_append: Option<String>,
    /// Decided active plan items.
    pub active_plan_items: Vec<SimulatedPlanItem>,
    /// Undecided medication recommendations held out of the active plan.
    pub undecided_recommendations: Vec<MedicationRecommendation>,
    /// Active ready replies.
    pub active_replies: Vec<SimulatedReply>,
    /// Replies superseded by corrected labs.
    pub superseded_replies: Vec<SimulatedReply>,
    /// Active follow-up descriptions.
    pub active_follow_up_descriptions: Vec<String>,
    /// Observation IDs invalidated by lab corrections or tombstones.
    pub invalidated_observations: BTreeSet<String>,
}

/// Projects the active plan and undecided medication recommendations.
///
/// Connected-walk invariant (a): an undecided medication recommendation must
/// never become an active plan item. Only items explicitly decided by the
/// physician are admitted into `active_plan_items`.
pub fn project_active_plan(
    decided_items: &[SimulatedPlanItem],
    recommendations: &[MedicationRecommendation],
) -> (Vec<SimulatedPlanItem>, Vec<MedicationRecommendation>) {
    let mut active_plan = decided_items.to_vec();
    let mut undecided = Vec::new();

    for rec in recommendations {
        match rec.decision_state {
            PlanDecisionState::Decided => {
                let already_present = active_plan.iter().any(|item| {
                    item.source_recommendation_id.as_deref() == Some(rec.recommendation_id.as_str())
                        || item.title == rec.medication
                });
                if !already_present {
                    active_plan.push(SimulatedPlanItem {
                        item_id: format!("plan-{}", rec.recommendation_id),
                        patient_workspace_id: rec.patient_workspace_id.clone(),
                        title: rec.medication.clone(),
                        detail: format!("Decided by physician ({})", rec.rationale),
                        decided_at: "2026-08-27T15:00:00.000Z".to_owned(),
                        source_recommendation_id: Some(rec.recommendation_id.clone()),
                    });
                }
            }
            PlanDecisionState::Undecided => {
                undecided.push(rec.clone());
            }
            PlanDecisionState::Declined => {}
        }
    }

    // Defensive filter: strip any plan item that references a still-undecided recommendation.
    let undecided_ids: BTreeSet<&str> = undecided
        .iter()
        .map(|rec| rec.recommendation_id.as_str())
        .collect();
    let undecided_meds: BTreeSet<&str> = undecided
        .iter()
        .map(|rec| rec.medication.as_str())
        .collect();
    active_plan.retain(|item| {
        let by_id = item
            .source_recommendation_id
            .as_deref()
            .is_some_and(|id| undecided_ids.contains(id));
        let by_title = undecided_meds.contains(item.title.as_str());
        !by_id && !by_title
    });

    (active_plan, undecided)
}

/// Applies a laboratory report event and reconciles any replies drafted from prior observations.
///
/// Connected-walk invariant (b): when a corrected lab report marks a prior critical observation
/// `entered-in-error` / `tombstoned` or corrects the abnormal value to normal, any stale ED
/// reply drafted from the prior result is automatically superseded and cleared from active replies.
pub fn apply_lab_report_and_reconcile_replies(
    event: &LabReportEvent,
    replies: &mut Vec<SimulatedReply>,
    invalidated_observations: &mut BTreeSet<String>,
) {
    let is_correction = event.report_status == "corrected" || event.report_version > 1;

    for obs in &event.observations {
        if matches!(
            obs.status,
            LabObservationStatus::EnteredInError
                | LabObservationStatus::Tombstoned
                | LabObservationStatus::Corrected
        ) {
            invalidated_observations.insert(obs.observation_id.clone());
        }
    }

    if !is_correction {
        // Check if this initial report contains critical potassium >= 6.0 mmol/L.
        for obs in &event.observations {
            if obs.status == LabObservationStatus::Final
                && obs.display_name.to_ascii_lowercase().contains("potassium")
                && obs.value.starts_with("6.")
            {
                let reply_id = format!("reply-{}-ed", event.report_id);
                if !replies.iter().any(|r| r.reply_work_id == reply_id) {
                    replies.push(SimulatedReply {
                        reply_work_id: reply_id,
                        patient_workspace_id: event.patient_workspace_id.clone(),
                        patient_display_name: "Jake Smith".to_owned(),
                        channel: "spruce".to_owned(),
                        headline: format!(
                            "Urgent potassium {} — advise ED evaluation",
                            obs.value
                        ),
                        draft_text: format!(
                            "Hi Jake, your Basic Metabolic Panel reported a potassium of {}. Please go to the nearest emergency department immediately for a repeat potassium and ECG.",
                            obs.value
                        ),
                        urgency: "urgent_ed".to_owned(),
                        created_at: event.effective_at.clone(),
                        based_on_report_id: Some(event.report_id.clone()),
                        based_on_observation_id: Some(obs.observation_id.clone()),
                        based_on_value: Some(obs.value.clone()),
                        state: ReplyLifecycleState::Ready,
                        superseded_by_input: None,
                        superseded_reason: None,
                    });
                }
            }
        }
        return;
    }

    // Build a human-readable summary of the corrected values.
    let corrected_summary = event
        .observations
        .iter()
        .filter(|obs| {
            matches!(
                obs.status,
                LabObservationStatus::Final | LabObservationStatus::Corrected
            )
        })
        .map(|obs| format!("{} {}", obs.display_name, obs.value))
        .collect::<Vec<_>>()
        .join(", ");

    let reason = if corrected_summary.is_empty() {
        format!(
            "Erroneous lab observation removed by {}; stale ED reply superseded.",
            event.input_key
        )
    } else {
        format!(
            "Corrected BMP ({corrected_summary}; prior K+ 6.3 mmol/L entered in error) superseded the urgent ED reply."
        )
    };

    for reply in replies.iter_mut() {
        if reply.patient_workspace_id != event.patient_workspace_id {
            continue;
        }
        let matches_report = reply.based_on_report_id.as_deref() == Some(event.report_id.as_str());
        let matches_invalidated_obs = reply
            .based_on_observation_id
            .as_deref()
            .is_some_and(|id| invalidated_observations.contains(id));

        if reply.state == ReplyLifecycleState::Ready && (matches_report || matches_invalidated_obs)
        {
            reply.state = ReplyLifecycleState::SupersededByCorrection;
            reply.superseded_by_input = Some(event.input_key.clone());
            reply.superseded_reason = Some(reason.clone());
        }
    }
}

/// Splits workspace replies into `(active_replies, superseded_replies)`, guaranteeing
/// that no reply based on an invalidated observation remains active.
pub fn project_active_replies(
    replies: &[SimulatedReply],
    invalidated_observations: &BTreeSet<String>,
) -> (Vec<SimulatedReply>, Vec<SimulatedReply>) {
    let mut active = Vec::new();
    let mut superseded = Vec::new();

    for reply in replies {
        let obs_invalidated = reply
            .based_on_observation_id
            .as_deref()
            .is_some_and(|id| invalidated_observations.contains(id));
        if reply.state == ReplyLifecycleState::Ready && !obs_invalidated {
            active.push(reply.clone());
        } else if reply.state == ReplyLifecycleState::SupersededByCorrection || obs_invalidated {
            let mut copy = reply.clone();
            copy.state = ReplyLifecycleState::SupersededByCorrection;
            if copy.superseded_reason.is_none() {
                copy.superseded_reason = Some(
                    "Underlying lab observation was corrected or entered in error.".to_owned(),
                );
            }
            superseded.push(copy.clone());
        }
    }

    (active, superseded)
}

/// Computes the investor metric: items the physician touched per 100 patients per week.
pub fn compute_investor_metric(
    physician_touched_items: u32,
    ambient_handled_items: u32,
    panel_size: u32,
    simulated_weeks: f64,
) -> InvestorMetric {
    let items_per_100 = if panel_size == 0 || simulated_weeks <= 0.0 {
        0.0
    } else {
        let raw = (f64::from(physician_touched_items) / (f64::from(panel_size) * simulated_weeks))
            * 100.0;
        (raw * 10.0).round() / 10.0
    };
    InvestorMetric {
        physician_touched_items,
        ambient_handled_items,
        panel_size,
        simulated_weeks,
        items_per_100_patients_per_week: items_per_100,
    }
}

/// Computes the aggregate investor metric across a sequence of simulation runs.
pub fn compute_aggregate_investor_metric(
    runs: &[SimulationRunRecord],
) -> SimulationAggregateMetric {
    let runs_count = u32::try_from(runs.len()).unwrap_or(u32::MAX);
    let mut total_physician_touched_items = 0_u32;
    let mut total_ambient_handled_items = 0_u32;
    let mut total_patient_weeks = 0.0_f64;

    for run in runs {
        total_physician_touched_items = total_physician_touched_items
            .saturating_add(run.investor_metric.physician_touched_items);
        total_ambient_handled_items =
            total_ambient_handled_items.saturating_add(run.investor_metric.ambient_handled_items);
        total_patient_weeks +=
            f64::from(run.investor_metric.panel_size) * run.investor_metric.simulated_weeks;
    }

    let items_per_100 = if total_patient_weeks <= 0.0 {
        0.0
    } else {
        let raw = (f64::from(total_physician_touched_items) / total_patient_weeks) * 100.0;
        (raw * 10.0).round() / 10.0
    };

    SimulationAggregateMetric {
        runs_count,
        total_physician_touched_items,
        total_ambient_handled_items,
        total_patient_weeks: (total_patient_weeks * 10.0).round() / 10.0,
        items_per_100_patients_per_week: items_per_100,
    }
}

/// Returns the 5 synthetic patients on Roger's practice panel.
pub fn roger_panel_patients() -> Vec<SimulatedPanelPatient> {
    vec![
        SimulatedPanelPatient {
            patient_workspace_id: JAKE_WORKSPACE_ID.to_owned(),
            display_name: "Jake Smith".to_owned(),
            birth_date: "1950-03-12".to_owned(),
            mrn: "00000001".to_owned(),
            role_summary: "Stopped lisinopril (dry cough); BMP K+ 6.3 -> corrected 4.3 mmol/L"
                .to_owned(),
            passive_patient: true,
        },
        SimulatedPanelPatient {
            patient_workspace_id: TANVIR_WORKSPACE_ID.to_owned(),
            display_name: "Tanvir Chowdhury".to_owned(),
            birth_date: "1989-11-08".to_owned(),
            mrn: "00000003".to_owned(),
            role_summary:
                "Elevated Lp(a) 136 nmol/L; prevention consult defers statin pending 6-wk lipids"
                    .to_owned(),
            passive_patient: true,
        },
        SimulatedPanelPatient {
            patient_workspace_id: DAN_WORKSPACE_ID.to_owned(),
            display_name: "Dan Cohen".to_owned(),
            birth_date: "1989-03-15".to_owned(),
            mrn: "00000002".to_owned(),
            role_summary:
                "Post-LAD DES & cardiac anxiety; passive patient on ambient weekly review"
                    .to_owned(),
            passive_patient: true,
        },
        SimulatedPanelPatient {
            patient_workspace_id: TERESA_WORKSPACE_ID.to_owned(),
            display_name: "Teresa Hayes".to_owned(),
            birth_date: "1960-08-22".to_owned(),
            mrn: "00000004".to_owned(),
            role_summary: "HTN & knee OA with prior NSAID creatinine drift; passive between visits"
                .to_owned(),
            passive_patient: true,
        },
        SimulatedPanelPatient {
            patient_workspace_id: MARY_WORKSPACE_ID.to_owned(),
            display_name: "Mary Louis".to_owned(),
            birth_date: "1972-03-18".to_owned(),
            mrn: "00000005".to_owned(),
            role_summary: "Nonischemic DCM (LVEF 35%); 3:30 PM follow-up visit".to_owned(),
            passive_patient: true,
        },
    ]
}

/// Returns the catalog of rerunnable simulation scenarios for Roger's local practice.
pub fn local_simulation_scenarios() -> Vec<SimulationScenarioInfo> {
    vec![
        SimulationScenarioInfo {
            key: "connected-walk".to_owned(),
            label: "Connected care walk".to_owned(),
            description: "Full 1-week synthetic walk across all 5 patients: reconciles Jake's stopped lisinopril without promoting an undecided ARB to the plan, supersedes the stale K+ 6.3 ED reply when corrected labs (K+ 4.3) arrive, processes Tanvir's prevention consult, and completes ambient rounds.".to_owned(),
            simulated_days: 7,
        },
        SimulationScenarioInfo {
            key: "lab-arrives-ed-reply".to_owned(),
            label: "Critical lab arrival (ED reply)".to_owned(),
            description: "Stops immediately after Jake Smith's initial BMP (K+ 6.3 mmol/L, Cr 1.9 mg/dL) arrives so Roger can inspect the active urgent ED reply before the lab correction supersedes it.".to_owned(),
            simulated_days: 1,
        },
        SimulationScenarioInfo {
            key: "reviewability".to_owned(),
            label: "Lab correction supersedes ED reply".to_owned(),
            description: "Admits Jake Smith's initial BMP (K+ 6.3 mmol/L) and then the corrected BMP (K+ 4.3 mmol/L, Cr 1.08 mg/dL; 6.3 entered in error), proving the stale ED reply is automatically superseded and cleared.".to_owned(),
            simulated_days: 7,
        },
        SimulationScenarioInfo {
            key: "physician-directed-follow-up".to_owned(),
            label: "Undecided medication guard".to_owned(),
            description: "Physician reconciles Jake Smith's stopped lisinopril and elevated home BP (152-160/84-90) without deciding on a replacement medication yet; guards the undecided ARB recommendation from becoming an active plan item.".to_owned(),
            simulated_days: 7,
        },
        SimulationScenarioInfo {
            key: "rounds-and-census".to_owned(),
            label: "Passive panel ambient week".to_owned(),
            description: "Advances the 5-patient panel across 7 days where simulated patients do nothing; Coda completes ambient whole-patient reviews with 0 physician touches.".to_owned(),
            simulated_days: 7,
        },
        SimulationScenarioInfo {
            key: "follow-through-evidence".to_owned(),
            label: "Specialist consult arrival".to_owned(),
            description: "Admits Tanvir Chowdhury's cardiovascular prevention clinic note, tracking the 6-week repeat fasting lipid order while holding undecided statin therapy out of the active plan.".to_owned(),
            simulated_days: 7,
        },
    ]
}

/// Builds Jake Smith's initial Basic Metabolic Panel event (`K+ 6.3 mmol/L`, `Cr 1.9 mg/dL`).
pub fn jake_initial_bmp_event() -> LabReportEvent {
    LabReportEvent {
        input_key: "jake-basic-metabolic-panel".to_owned(),
        report_id: "jake-bmp-follow-up".to_owned(),
        patient_workspace_id: JAKE_WORKSPACE_ID.to_owned(),
        report_version: 1,
        report_status: "final".to_owned(),
        effective_at: "2026-08-27T12:00:00.000Z".to_owned(),
        observations: vec![
            LabObservationEvent {
                observation_id: "jake-potassium-follow-up".to_owned(),
                display_name: "Potassium".to_owned(),
                value: "6.3 mmol/L".to_owned(),
                status: LabObservationStatus::Final,
            },
            LabObservationEvent {
                observation_id: "jake-creatinine-follow-up".to_owned(),
                display_name: "Creatinine".to_owned(),
                value: "1.9 mg/dL".to_owned(),
                status: LabObservationStatus::Final,
            },
        ],
    }
}

/// Builds Jake Smith's corrected Basic Metabolic Panel event (`K+ 6.3` entered-in-error, repeat `K+ 4.3 mmol/L`, `Cr 1.08 mg/dL`).
pub fn jake_corrected_bmp_event() -> LabReportEvent {
    LabReportEvent {
        input_key: "jake-basic-metabolic-panel-correction".to_owned(),
        report_id: "jake-bmp-follow-up".to_owned(),
        patient_workspace_id: JAKE_WORKSPACE_ID.to_owned(),
        report_version: 2,
        report_status: "corrected".to_owned(),
        effective_at: "2026-08-27T14:00:00.000Z".to_owned(),
        observations: vec![
            LabObservationEvent {
                observation_id: "jake-potassium-follow-up".to_owned(),
                display_name: "Potassium".to_owned(),
                value: "6.3 mmol/L".to_owned(),
                status: LabObservationStatus::EnteredInError,
            },
            LabObservationEvent {
                observation_id: "jake-potassium-repeat".to_owned(),
                display_name: "Potassium".to_owned(),
                value: "4.3 mmol/L".to_owned(),
                status: LabObservationStatus::Final,
            },
            LabObservationEvent {
                observation_id: "jake-creatinine-follow-up".to_owned(),
                display_name: "Creatinine".to_owned(),
                value: "1.08 mg/dL".to_owned(),
                status: LabObservationStatus::Corrected,
            },
        ],
    }
}

/// Stateful local simulation engine that backs `physician-sample` in simulation mode.
#[derive(Debug, Clone)]
pub struct SimulationSession {
    /// Currently active scenario key.
    pub active_scenario_key: String,
    /// Human-readable label for the active scenario.
    pub active_scenario_label: String,
    /// Current simulated clock (`ISO-8601`).
    pub current_time: String,
    /// Investor metric for the current run.
    pub investor_metric: InvestorMetric,
    /// Aggregate investor metric across all runs in this session.
    pub aggregate_metric: SimulationAggregateMetric,
    /// Per-workspace simulated clinical overlays.
    pub overlays: BTreeMap<String, SimulatedWorkspaceOverlay>,
    /// Decided plan items per workspace.
    pub decided_plans: BTreeMap<String, Vec<SimulatedPlanItem>>,
    /// Medication recommendations per workspace.
    pub recommendations: BTreeMap<String, Vec<MedicationRecommendation>>,
    /// All replies (active, superseded, and closed) per workspace.
    pub replies: BTreeMap<String, Vec<SimulatedReply>>,
    /// Invalidated observation IDs per workspace.
    pub invalidated_observations: BTreeMap<String, BTreeSet<String>>,
    /// Replayable history of all simulation runs executed in this session.
    pub run_history: Vec<SimulationRunRecord>,
    next_run_seq: u32,
}

impl Default for SimulationSession {
    fn default() -> Self {
        Self::new(DEFAULT_SIMULATION_SCENARIO)
    }
}

impl SimulationSession {
    /// Creates a new simulation session and runs `initial_scenario` (falling back to `connected-walk`).
    pub fn new(initial_scenario: &str) -> Self {
        let mut session = Self {
            active_scenario_key: String::new(),
            active_scenario_label: String::new(),
            current_time: "2026-08-27T12:00:00.000Z".to_owned(),
            investor_metric: compute_investor_metric(0, 0, ROGER_PANEL_SIZE, 1.0),
            aggregate_metric: SimulationAggregateMetric {
                runs_count: 0,
                total_physician_touched_items: 0,
                total_ambient_handled_items: 0,
                total_patient_weeks: 0.0,
                items_per_100_patients_per_week: 0.0,
            },
            overlays: BTreeMap::new(),
            decided_plans: BTreeMap::new(),
            recommendations: BTreeMap::new(),
            replies: BTreeMap::new(),
            invalidated_observations: BTreeMap::new(),
            run_history: Vec::new(),
            next_run_seq: 1,
        };
        let key = if local_simulation_scenarios()
            .iter()
            .any(|s| s.key == initial_scenario)
        {
            initial_scenario
        } else {
            DEFAULT_SIMULATION_SCENARIO
        };
        let _ = session.run_scenario(key);
        session
    }

    /// Runs or reruns a named simulation scenario from a clean panel baseline.
    pub fn run_scenario(&mut self, scenario_key: &str) -> Result<SimulationRunRecord, String> {
        let scenario = local_simulation_scenarios()
            .into_iter()
            .find(|item| item.key == scenario_key)
            .ok_or_else(|| format!("unknown simulation scenario: {scenario_key}"))?;

        self.reset_panel_state();
        self.active_scenario_key = scenario.key.clone();
        self.active_scenario_label = scenario.label.clone();

        let initial_time = "2026-08-27T12:00:00.000Z".to_owned();
        let mut steps = Vec::new();
        let mut physician_touches = 0_u32;
        let mut ambient_items = 0_u32;

        match scenario_key {
            "connected-walk" => {
                self.seed_jake_reconciliation();
                physician_touches += 1;
                steps.push(
                    "2026-08-27T12:00:00Z · Physician reconciled Jake Smith's stopped lisinopril (dry cough) and home BP 152-160/84-90; Losartan 50 mg daily held as undecided recommendation.".to_owned(),
                );

                self.apply_lab_event(&jake_initial_bmp_event());
                ambient_items += 1;
                steps.push(
                    "2026-08-27T12:05:00Z · Admitted Jake Smith BMP v1 (K+ 6.3 mmol/L, Cr 1.9 mg/dL); Coda drafted urgent ED reply.".to_owned(),
                );

                self.apply_lab_event(&jake_corrected_bmp_event());
                ambient_items += 1;
                steps.push(
                    "2026-08-27T14:05:00Z · Admitted corrected BMP v2 (K+ 4.3 mmol/L, Cr 1.08 mg/dL; prior K+ 6.3 entered in error); stale ED reply automatically superseded and cleared.".to_owned(),
                );

                self.seed_tanvir_specialist_consultation();
                ambient_items += 1;
                steps.push(
                    "2026-08-28T12:05:00Z · Admitted Tanvir Chowdhury cardiovascular prevention consult; 6-week repeat fasting lipids tracked while Rosuvastatin 10 mg remains undecided.".to_owned(),
                );

                ambient_items += 1;
                self.current_time = "2026-09-03T12:00:00.000Z".to_owned();
                steps.push(
                    "2026-09-03T12:00:00Z · Completed 7-day ambient census review across all 5 synthetic patients without additional physician touches.".to_owned(),
                );
            }
            "lab-arrives-ed-reply" => {
                self.seed_jake_reconciliation();
                physician_touches += 1;
                steps.push(
                    "2026-08-27T12:00:00Z · Physician reconciled Jake Smith's stopped lisinopril."
                        .to_owned(),
                );
                self.apply_lab_event(&jake_initial_bmp_event());
                ambient_items += 1;
                self.current_time = "2026-08-27T12:05:00.000Z".to_owned();
                steps.push(
                    "2026-08-27T12:05:00Z · Initial BMP v1 (K+ 6.3 mmol/L, Cr 1.9 mg/dL) arrived; urgent ED reply is active awaiting review or lab correction.".to_owned(),
                );
            }
            "reviewability" => {
                self.seed_jake_reconciliation();
                physician_touches += 1;
                self.apply_lab_event(&jake_initial_bmp_event());
                ambient_items += 1;
                self.apply_lab_event(&jake_corrected_bmp_event());
                ambient_items += 1;
                self.current_time = "2026-09-03T12:00:00.000Z".to_owned();
                steps.push(
                    "2026-08-27T12:05:00Z · Initial BMP v1 (K+ 6.3 mmol/L) drafted urgent ED reply.".to_owned(),
                );
                steps.push(
                    "2026-08-27T14:05:00Z · Corrected BMP v2 (K+ 4.3 mmol/L, Cr 1.08 mg/dL) superseded and cleared the stale ED reply.".to_owned(),
                );
            }
            "physician-directed-follow-up" => {
                self.seed_jake_reconciliation();
                physician_touches += 1;
                ambient_items += 1;
                self.current_time = "2026-09-03T12:00:00.000Z".to_owned();
                steps.push(
                    "2026-08-27T12:00:00Z · Physician recorded stopped lisinopril and home BP 152-160/84-90 without deciding on a replacement medication; undecided Losartan 50 mg held out of active plan.".to_owned(),
                );
            }
            "rounds-and-census" => {
                ambient_items += 5;
                self.current_time = "2026-09-03T12:00:00.000Z".to_owned();
                steps.push(
                    "2026-09-03T12:00:00Z · Completed 7-day ambient rounds across all 5 passive patients with 0 physician touches.".to_owned(),
                );
            }
            "follow-through-evidence" => {
                self.seed_tanvir_specialist_consultation();
                ambient_items += 2;
                self.current_time = "2026-09-04T12:00:00.000Z".to_owned();
                steps.push(
                    "2026-08-28T12:05:00Z · Admitted Tanvir Chowdhury cardiovascular prevention consultation; repeat fasting lipids tracked and undecided statin kept out of active plan.".to_owned(),
                );
            }
            _ => unreachable!(),
        }

        let weeks = (f64::from(scenario.simulated_days) / 7.0).max(1.0);
        self.investor_metric =
            compute_investor_metric(physician_touches, ambient_items, ROGER_PANEL_SIZE, weeks);
        self.rebuild_overlays();

        let checks = self.evaluate_checks(scenario_key);
        let passed = checks.iter().all(|check| check.passed);

        let run_id = format!("sim-run-{:03}-{}", self.next_run_seq, scenario.key);
        self.next_run_seq += 1;

        let record = SimulationRunRecord {
            run_id,
            scenario_key: scenario.key,
            scenario_label: scenario.label,
            initial_simulation_time: initial_time,
            final_simulation_time: self.current_time.clone(),
            steps_executed: steps,
            checks,
            investor_metric: self.investor_metric.clone(),
            passed,
        };
        self.run_history.push(record.clone());
        self.aggregate_metric = compute_aggregate_investor_metric(&self.run_history);
        Ok(record)
    }

    /// Physician explicitly approves an undecided medication recommendation into the active plan.
    pub fn decide_recommendation(
        &mut self,
        patient_workspace_id: &str,
        recommendation_id: &str,
    ) -> Result<SimulatedPlanItem, String> {
        let (rec_id, medication, rationale, newly_decided) = {
            let recs = self
                .recommendations
                .get_mut(patient_workspace_id)
                .ok_or_else(|| {
                    format!("no recommendations for workspace {patient_workspace_id}")
                })?;
            let rec = recs
                .iter_mut()
                .find(|item| item.recommendation_id == recommendation_id)
                .ok_or_else(|| format!("unknown recommendation {recommendation_id}"))?;

            let newly_decided = rec.decision_state != PlanDecisionState::Decided;
            rec.decision_state = PlanDecisionState::Decided;
            (
                rec.recommendation_id.clone(),
                rec.medication.clone(),
                rec.rationale.clone(),
                newly_decided,
            )
        };

        if newly_decided {
            self.record_physician_touch();
        }

        let plan_item = SimulatedPlanItem {
            item_id: format!("plan-{rec_id}"),
            patient_workspace_id: patient_workspace_id.to_owned(),
            title: medication,
            detail: format!("Approved by physician ({rationale})"),
            decided_at: self.current_time.clone(),
            source_recommendation_id: Some(rec_id),
        };

        let plans = self
            .decided_plans
            .entry(patient_workspace_id.to_owned())
            .or_default();
        if !plans.iter().any(|item| item.item_id == plan_item.item_id) {
            plans.push(plan_item.clone());
        }

        self.rebuild_overlays();
        Ok(plan_item)
    }

    /// Admits Jake Smith's corrected BMP (`K+ 4.3 mmol/L`, `Cr 1.08 mg/dL`) and supersedes any stale ED reply.
    pub fn admit_corrected_labs(&mut self, patient_workspace_id: &str) -> Result<(), String> {
        if patient_workspace_id != JAKE_WORKSPACE_ID {
            return Err(format!(
                "no pending lab correction for workspace {patient_workspace_id}"
            ));
        }
        self.apply_lab_event(&jake_corrected_bmp_event());
        self.investor_metric = compute_investor_metric(
            self.investor_metric.physician_touched_items,
            self.investor_metric.ambient_handled_items.saturating_add(1),
            self.investor_metric.panel_size,
            self.investor_metric.simulated_weeks,
        );
        if let Some(last) = self.run_history.last_mut() {
            last.investor_metric = self.investor_metric.clone();
        }
        self.aggregate_metric = compute_aggregate_investor_metric(&self.run_history);
        self.rebuild_overlays();
        Ok(())
    }

    /// Closes an active reply by physician action, counting as a physician touch.
    pub fn close_reply(
        &mut self,
        patient_workspace_id: &str,
        reply_work_id: &str,
    ) -> Result<(), String> {
        let replies = self
            .replies
            .get_mut(patient_workspace_id)
            .ok_or_else(|| format!("no replies for workspace {patient_workspace_id}"))?;
        let reply = replies
            .iter_mut()
            .find(|item| item.reply_work_id == reply_work_id)
            .ok_or_else(|| format!("unknown reply {reply_work_id}"))?;

        if reply.state == ReplyLifecycleState::Ready {
            reply.state = ReplyLifecycleState::ClosedByPhysician;
            self.record_physician_touch();
        }
        self.rebuild_overlays();
        Ok(())
    }

    /// Records a direct physician touch (e.g., approving a recommendation, sending a chat turn, or closing a reply).
    pub fn record_physician_touch(&mut self) {
        let next_touches = self
            .investor_metric
            .physician_touched_items
            .saturating_add(1);
        self.investor_metric = compute_investor_metric(
            next_touches,
            self.investor_metric.ambient_handled_items,
            self.investor_metric.panel_size,
            self.investor_metric.simulated_weeks,
        );
        if let Some(last) = self.run_history.last_mut() {
            last.investor_metric = self.investor_metric.clone();
        }
        self.aggregate_metric = compute_aggregate_investor_metric(&self.run_history);
    }

    fn reset_panel_state(&mut self) {
        self.current_time = "2026-08-27T12:00:00.000Z".to_owned();
        self.overlays.clear();
        self.decided_plans.clear();
        self.recommendations.clear();
        self.replies.clear();
        self.invalidated_observations.clear();

        for patient in roger_panel_patients() {
            self.decided_plans
                .insert(patient.patient_workspace_id.clone(), Vec::new());
            self.recommendations
                .insert(patient.patient_workspace_id.clone(), Vec::new());
            self.replies
                .insert(patient.patient_workspace_id.clone(), Vec::new());
            self.invalidated_observations
                .insert(patient.patient_workspace_id, BTreeSet::new());
        }
    }

    fn seed_jake_reconciliation(&mut self) {
        self.decided_plans.insert(
            JAKE_WORKSPACE_ID.to_owned(),
            vec![
                SimulatedPlanItem {
                    item_id: "plan-jake-amlodipine-continue".to_owned(),
                    patient_workspace_id: JAKE_WORKSPACE_ID.to_owned(),
                    title: "Continue amlodipine 5 mg daily".to_owned(),
                    detail: "Lisinopril 20 mg stopped two weeks ago due to persistent dry cough; continue amlodipine 5 mg daily while collecting home BP readings.".to_owned(),
                    decided_at: "2026-08-27T12:00:00.000Z".to_owned(),
                    source_recommendation_id: None,
                },
                SimulatedPlanItem {
                    item_id: "plan-jake-home-bp-log".to_owned(),
                    patient_workspace_id: JAKE_WORKSPACE_ID.to_owned(),
                    title: "Record home blood-pressure log for 1 week".to_owned(),
                    detail: "Track twice-daily home blood pressures (currently 152-160/84-90 mmHg) before deciding on replacement antihypertensive therapy.".to_owned(),
                    decided_at: "2026-08-27T12:00:00.000Z".to_owned(),
                    source_recommendation_id: None,
                },
            ],
        );

        self.recommendations.insert(
            JAKE_WORKSPACE_ID.to_owned(),
            vec![MedicationRecommendation {
                recommendation_id: "rec-jake-losartan-50".to_owned(),
                patient_workspace_id: JAKE_WORKSPACE_ID.to_owned(),
                medication: "Losartan 50 mg daily".to_owned(),
                rationale: "Candidate ARB to replace stopped lisinopril if 1-week home blood pressures remain above goal; physician has not decided on a medication change yet.".to_owned(),
                source_label: "Medication reconciliation review".to_owned(),
                decision_state: PlanDecisionState::Undecided,
            }],
        );
    }

    fn seed_tanvir_specialist_consultation(&mut self) {
        self.decided_plans.insert(
            TANVIR_WORKSPACE_ID.to_owned(),
            vec![SimulatedPlanItem {
                item_id: "plan-tanvir-repeat-lipids-6wk".to_owned(),
                patient_workspace_id: TANVIR_WORKSPACE_ID.to_owned(),
                title: "Repeat fasting lipid panel in 6 weeks".to_owned(),
                detail: "Cardiovascular prevention clinic attended 2026-08-28; repeat fasting lipids ordered in 6 weeks with continued aerobic exercise and dietary counseling.".to_owned(),
                decided_at: "2026-08-28T12:05:00.000Z".to_owned(),
                source_recommendation_id: None,
            }],
        );

        self.recommendations.insert(
            TANVIR_WORKSPACE_ID.to_owned(),
            vec![MedicationRecommendation {
                recommendation_id: "rec-tanvir-rosuvastatin-10".to_owned(),
                patient_workspace_id: TANVIR_WORKSPACE_ID.to_owned(),
                medication: "Rosuvastatin 10 mg nightly".to_owned(),
                rationale: "Prevention specialist deferred lipid-lowering therapy until 6-week repeat fasting lipids are reviewed; medication recommendation remains undecided.".to_owned(),
                source_label: "Cardiovascular Prevention Consultation (Dr. Elena Rostova)".to_owned(),
                decision_state: PlanDecisionState::Undecided,
            }],
        );
    }

    fn apply_lab_event(&mut self, event: &LabReportEvent) {
        let replies = self
            .replies
            .entry(event.patient_workspace_id.clone())
            .or_default();
        let invalidated = self
            .invalidated_observations
            .entry(event.patient_workspace_id.clone())
            .or_default();
        apply_lab_report_and_reconcile_replies(event, replies, invalidated);
    }

    fn rebuild_overlays(&mut self) {
        for patient in roger_panel_patients() {
            let ws_id = &patient.patient_workspace_id;
            let decided = self.decided_plans.get(ws_id).cloned().unwrap_or_default();
            let recs = self.recommendations.get(ws_id).cloned().unwrap_or_default();
            let (active_plan_items, undecided_recommendations) =
                project_active_plan(&decided, &recs);

            let all_replies = self.replies.get(ws_id).cloned().unwrap_or_default();
            let invalidated = self
                .invalidated_observations
                .get(ws_id)
                .cloned()
                .unwrap_or_default();
            let (active_replies, superseded_replies) =
                project_active_replies(&all_replies, &invalidated);

            let synopsis_append = match ws_id.as_str() {
                JAKE_WORKSPACE_ID => {
                    if !superseded_replies.is_empty() {
                        Some("On August 27, 2026, an initial Basic Metabolic Panel potassium of 6.3 mmol/L (with creatinine 1.9 mg/dL) was marked entered-in-error on a corrected report two hours later showing repeat potassium 4.3 mmol/L and corrected creatinine 1.08 mg/dL; the draft ED reply was superseded. Lisinopril 20 mg remains stopped for dry cough while he logs home blood pressures (152-160/84-90) on amlodipine 5 mg; replacement ARB therapy (Losartan 50 mg daily) remains undecided.".to_owned())
                    } else if !active_replies.is_empty() {
                        Some("On August 27, 2026 at 12:00Z, Basic Metabolic Panel v1 reported potassium 6.3 mmol/L and creatinine 1.9 mg/dL; an urgent ED reply draft is active.".to_owned())
                    } else if !active_plan_items.is_empty() {
                        Some("Lisinopril 20 mg was stopped two weeks ago due to dry cough; home blood pressures are 152-160/84-90 on amlodipine 5 mg daily, and replacement ARB therapy (Losartan 50 mg daily) remains undecided pending a 1-week home BP log.".to_owned())
                    } else {
                        None
                    }
                }
                TANVIR_WORKSPACE_ID if !active_plan_items.is_empty() => Some(
                    "On August 28, 2026, he attended the cardiovascular prevention consultation with Dr. Elena Rostova; no lipid-lowering medication was started, and repeat fasting lipids were requested in 6 weeks before deciding on statin therapy.".to_owned(),
                ),
                _ => None,
            };

            let active_follow_up_descriptions = active_plan_items
                .iter()
                .map(|item| format!("{}: {}", item.title, item.detail))
                .collect();

            self.overlays.insert(
                ws_id.clone(),
                SimulatedWorkspaceOverlay {
                    patient_workspace_id: ws_id.clone(),
                    current_time: self.current_time.clone(),
                    synopsis_append,
                    active_plan_items,
                    undecided_recommendations,
                    active_replies,
                    superseded_replies,
                    active_follow_up_descriptions,
                    invalidated_observations: invalidated,
                },
            );
        }
    }

    fn evaluate_checks(&self, scenario_key: &str) -> Vec<SimulationCheckResult> {
        let jake = self.overlays.get(JAKE_WORKSPACE_ID);
        let tanvir = self.overlays.get(TANVIR_WORKSPACE_ID);

        let jake_no_undecided_in_plan = jake.is_none_or(|ov| {
            !ov.active_plan_items
                .iter()
                .any(|item| item.title.contains("Losartan"))
        });
        let tanvir_no_undecided_in_plan = tanvir.is_none_or(|ov| {
            !ov.active_plan_items
                .iter()
                .any(|item| item.title.contains("Rosuvastatin"))
        });
        let plan_guard_passed = jake_no_undecided_in_plan && tanvir_no_undecided_in_plan;

        let reply_check = if scenario_key == "lab-arrives-ed-reply" {
            let active_ed = jake.is_some_and(|ov| !ov.active_replies.is_empty());
            SimulationCheckResult {
                key: "initial-critical-bmp-drafts-ed-reply".to_owned(),
                description:
                    "Initial critical BMP (K+ 6.3 mmol/L) drafts an active urgent ED reply before correction arrives."
                        .to_owned(),
                passed: active_ed,
                detail: if active_ed {
                    "Urgent ED reply is active on Today and in Jake Smith's workspace.".to_owned()
                } else {
                    "Expected urgent ED reply to be active before correction.".to_owned()
                },
            }
        } else {
            let no_stale_ed = jake.is_none_or(|ov| ov.active_replies.is_empty());
            SimulationCheckResult {
                key: "corrected-labs-supersede-ed-reply".to_owned(),
                description:
                    "Corrected labs (K+ 4.3 mmol/L, Cr 1.08 mg/dL; 6.3 entered-in-error) supersede and clear the stale ED reply."
                        .to_owned(),
                passed: no_stale_ed,
                detail: if no_stale_ed {
                    "No stale ED reply remains active after the corrected BMP.".to_owned()
                } else {
                    "Stale ED reply remained active after lab correction.".to_owned()
                },
            }
        };

        vec![
            SimulationCheckResult {
                key: "undecided-medication-not-in-plan".to_owned(),
                description:
                    "Undecided medication recommendations are held out of active patient plans."
                        .to_owned(),
                passed: plan_guard_passed,
                detail: if plan_guard_passed {
                    "All undecided medication recommendations (Losartan 50 mg, Rosuvastatin 10 mg) were kept out of active_plan_items.".to_owned()
                } else {
                    "An undecided medication recommendation leaked into active_plan_items."
                        .to_owned()
                },
            },
            reply_check,
            SimulationCheckResult {
                key: "investor-metric-computed".to_owned(),
                description: "Run computes items the physician touched per 100 patients per week."
                    .to_owned(),
                passed: self.investor_metric.panel_size == ROGER_PANEL_SIZE
                    && self.investor_metric.simulated_weeks > 0.0,
                detail: format!(
                    "{:.1} items touched per 100 patients per week ({} physician touches across {} patients over {:.1} weeks).",
                    self.investor_metric.items_per_100_patients_per_week,
                    self.investor_metric.physician_touched_items,
                    self.investor_metric.panel_size,
                    self.investor_metric.simulated_weeks,
                ),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undecided_medication_recommendation_never_enters_active_plan() {
        let decided = vec![SimulatedPlanItem {
            item_id: "plan-1".to_owned(),
            patient_workspace_id: JAKE_WORKSPACE_ID.to_owned(),
            title: "Continue amlodipine 5 mg daily".to_owned(),
            detail: "Current baseline medication".to_owned(),
            decided_at: "2026-08-27T12:00:00.000Z".to_owned(),
            source_recommendation_id: None,
        }];
        let recommendations = vec![MedicationRecommendation {
            recommendation_id: "rec-jake-losartan-50".to_owned(),
            patient_workspace_id: JAKE_WORKSPACE_ID.to_owned(),
            medication: "Losartan 50 mg daily".to_owned(),
            rationale: "Undecided ARB option after stopping lisinopril".to_owned(),
            source_label: "Reconciliation".to_owned(),
            decision_state: PlanDecisionState::Undecided,
        }];

        let (active_plan, undecided) = project_active_plan(&decided, &recommendations);
        assert_eq!(active_plan.len(), 1);
        assert_eq!(active_plan[0].title, "Continue amlodipine 5 mg daily");
        assert!(
            !active_plan
                .iter()
                .any(|item| item.title.contains("Losartan")),
            "undecided medication must not become an active plan item"
        );
        assert_eq!(undecided.len(), 1);
        assert_eq!(undecided[0].medication, "Losartan 50 mg daily");
    }

    #[test]
    fn corrected_labs_supersede_and_clear_stale_ed_reply() {
        let mut replies = Vec::new();
        let mut invalidated = BTreeSet::new();

        apply_lab_report_and_reconcile_replies(
            &jake_initial_bmp_event(),
            &mut replies,
            &mut invalidated,
        );
        let (active_before, superseded_before) = project_active_replies(&replies, &invalidated);
        assert_eq!(active_before.len(), 1);
        assert!(active_before[0].headline.contains("6.3 mmol/L"));
        assert!(superseded_before.is_empty());

        apply_lab_report_and_reconcile_replies(
            &jake_corrected_bmp_event(),
            &mut replies,
            &mut invalidated,
        );
        let (active_after, superseded_after) = project_active_replies(&replies, &invalidated);
        assert!(
            active_after.is_empty(),
            "stale ED reply must be cleared after corrected BMP arrives"
        );
        assert_eq!(superseded_after.len(), 1);
        assert_eq!(
            superseded_after[0].state,
            ReplyLifecycleState::SupersededByCorrection
        );
        assert!(
            superseded_after[0]
                .superseded_reason
                .as_deref()
                .unwrap_or_default()
                .contains("4.3 mmol/L")
        );
    }

    #[test]
    fn connected_walk_computes_investor_metric_and_passes_all_checks() {
        let mut session = SimulationSession::new("connected-walk");
        assert_eq!(session.run_history.len(), 1);
        let run = &session.run_history[0];
        assert!(run.passed);
        assert_eq!(run.investor_metric.physician_touched_items, 1);
        assert_eq!(run.investor_metric.panel_size, 5);
        assert!((run.investor_metric.simulated_weeks - 1.0).abs() < f64::EPSILON);
        assert!((run.investor_metric.items_per_100_patients_per_week - 20.0).abs() < f64::EPSILON);

        let jake = session.overlays.get(JAKE_WORKSPACE_ID).expect("jake");
        assert!(jake.active_replies.is_empty());
        assert_eq!(jake.superseded_replies.len(), 1);
        assert_eq!(jake.undecided_recommendations.len(), 1);
        assert!(
            !jake
                .active_plan_items
                .iter()
                .any(|item| item.title.contains("Losartan"))
        );

        // Deciding the recommendation promotes it to active plan and increments the investor metric.
        session
            .decide_recommendation(JAKE_WORKSPACE_ID, "rec-jake-losartan-50")
            .expect("decide recommendation");
        let jake_after = session.overlays.get(JAKE_WORKSPACE_ID).expect("jake after");
        assert!(jake_after.undecided_recommendations.is_empty());
        assert!(
            jake_after
                .active_plan_items
                .iter()
                .any(|item| item.title == "Losartan 50 mg daily")
        );
        assert!(
            (session.investor_metric.items_per_100_patients_per_week - 40.0).abs() < f64::EPSILON
        );

        // Running a passive week updates the aggregate metric across runs.
        let passive_run = session
            .run_scenario("rounds-and-census")
            .expect("rounds-and-census");
        assert!(
            (passive_run.investor_metric.items_per_100_patients_per_week - 0.0).abs()
                < f64::EPSILON
        );
        assert_eq!(session.aggregate_metric.runs_count, 2);
        assert_eq!(session.aggregate_metric.total_physician_touched_items, 2);
        assert!((session.aggregate_metric.total_patient_weeks - 10.0).abs() < f64::EPSILON);
        assert!(
            (session.aggregate_metric.items_per_100_patients_per_week - 20.0).abs() < f64::EPSILON
        );
    }
}

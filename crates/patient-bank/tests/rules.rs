//! The bank's chronology and safety rules, each on a small hand-written chart.

use patient_bank::catalog::Catalog;
use patient_bank::fhir::Chart;
use patient_bank::ledger::{Ledger, Part, Patient, Stamp, Terms};
use patient_bank::render::Renderer;
use patient_bank::seeds::{Seed, Seeds, Unit};
use patient_bank::timeline::{OrderStatus, ProblemStatus, Timeline};
use patient_bank::{bank, check, paths, stories};
use serde_json::json;

fn at(s: &str) -> Stamp {
    Stamp::parse(s).expect("stamp")
}

fn seed() -> Seed {
    serde_json::from_value(json!({
        "mrn": "MRN99999999", "sex": "female", "age_at_as_of": 50, "birth_year": 1976,
        "birth_date": "1976-03-02", "given": "Ada", "family": "Example", "town": "Northampton",
        "postal_code": "01060", "race": "White", "ethnicity": "Not Hispanic or Latino",
        "preferred_language": "English", "insurance": "commercial employer plan",
        "occupation_hint": "teacher", "complexity": "light", "years_with_practice": 6,
        "target_documents": 40, "target_observations": 200, "anchors": [], "episodes": [],
        "chart_texture": [], "phone": "+1-413-555-0100"
    }))
    .expect("seed")
}

fn patient(events: serde_json::Value) -> Patient {
    serde_json::from_value(json!({
        "mrn": "MRN99999999", "given": "Ada", "family": "Example", "birth_date": "1976-03-02",
        "events": events
    }))
    .expect("patient")
}

fn render(p: &Patient, when: &str) -> Chart {
    let catalog = Catalog::load(&paths::catalog()).expect("catalog");
    let terms = Terms::default();
    Renderer {
        catalog: &catalog,
        terms: &terms,
    }
    .render(&seed(), p, at(when))
    .0
}

fn visit(id: &str, start: &str, extra: serde_json::Value) -> serde_json::Value {
    let mut ev = json!({"id": id, "kind": "office_visit", "start": start});
    for (k, v) in extra.as_object().expect("object") {
        ev[k] = v.clone();
    }
    ev
}

const SERTRALINE: &str = "sertraline 50 MG Oral Tablet";

#[test]
fn a_restarted_drug_stops_again_at_its_own_stop() {
    let med = |action: &str| json!({"medications": [{"drug": SERTRALINE, "action": action}]});
    let p = patient(json!([
        visit("e1", "2016-01-05T09:10", med("start")),
        visit("e2", "2017-02-07T09:10", med("stop")),
        visit("e3", "2019-03-12T09:10", med("start")),
        visit("e4", "2020-04-14T09:10", med("stop")),
    ]));
    let tl = Timeline::new(&p);
    assert_eq!(tl.orders.len(), 2);
    assert_eq!(
        tl.orders[0].status(at("2018-01-01T00:00")),
        OrderStatus::Stopped(at("2017-02-07T09:10"))
    );
    assert_eq!(
        tl.orders[1].status(at("2019-06-01T00:00")),
        OrderStatus::Active
    );
    assert_eq!(
        tl.orders[1].status(at("2021-01-01T00:00")),
        OrderStatus::Stopped(at("2020-04-14T09:10"))
    );

    let chart = render(&p, "2026-08-27T12:00");
    let status: Vec<&str> = chart.medications.iter().map(|m| m.status).collect();
    assert_eq!(status, ["stopped", "stopped"]);
}

#[test]
fn a_resolved_problem_can_recur() {
    let cond = |action: &str| json!({"conditions": [{"label": "Iron deficiency anemia", "action": action}]});
    let p = patient(json!([
        visit("e1", "2018-09-17T10:00", cond("diagnosed")),
        visit("e2", "2019-09-27T10:00", cond("resolved")),
        visit("e3", "2026-07-21T10:00", cond("diagnosed")),
    ]));
    let tl = Timeline::new(&p);
    assert_eq!(tl.problems.len(), 1, "one problem-list row per concept");
    let prob = &tl.problems[0];
    assert_eq!(prob.status(at("2019-01-01T00:00")), ProblemStatus::Active);
    assert_eq!(
        prob.status(at("2020-01-01T00:00")),
        ProblemStatus::Resolved(at("2019-09-27T10:00"))
    );
    assert_eq!(
        prob.status(at("2026-08-01T00:00")),
        ProblemStatus::Recurrence
    );

    let then = render(&p, "2020-01-01T00:00");
    assert_eq!(
        then.conditions[0].clinical_status.coding[0].code,
        "resolved"
    );
    assert!(then.conditions[0].abatement_date_time.is_some());
    let now = render(&p, "2026-08-27T12:00");
    assert_eq!(
        now.conditions[0].clinical_status.coding[0].code,
        "recurrence"
    );
    assert!(now.conditions[0].abatement_date_time.is_none());
}

#[test]
fn each_record_appears_only_after_its_own_time() {
    // An order written two days after the visit, and rooming vitals a few minutes in.
    let p = patient(json!([visit(
        "e1",
        "2024-01-01T09:00",
        json!({
            "medications": [{"drug": SERTRALINE, "action": "start", "date": "2024-01-03T11:00"}],
            "vitals": {"hr": 72},
            "documents": [{"type": "Progress Notes", "date": "2024-01-01T09:40", "body": "Seen for follow-up of mood; plan reviewed with the patient at length."}]
        })
    )]));
    let day_two = render(&p, "2024-01-02T12:00");
    assert!(
        day_two.medications.is_empty(),
        "an order written on Jan 3 is not in a Jan 2 chart"
    );
    assert_eq!(day_two.documents.len(), 1);
    let one_minute_in = render(&p, "2024-01-01T09:01");
    assert!(
        one_minute_in.observations.is_empty(),
        "vitals taken after the cutoff are absent"
    );
    assert!(one_minute_in.documents.is_empty());
    assert_eq!(render(&p, "2024-01-04T00:00").medications.len(), 1);
}

#[test]
fn a_result_with_its_own_release_time_stays_pending_until_then() {
    let p = patient(json!([visit(
        "e1",
        "2021-11-09T09:55",
        json!({"labs": [{"panel": null, "collected": "2021-11-09T09:55", "results": [
            {"test": "Hemoglobin A1c", "value": 5.8},
            {"test": "High-risk HPV DNA", "text": "Negative", "resulted": "2021-11-15T14:00"}
        ]}]})
    )]));
    let status = |chart: &Chart, test: &str| {
        let o = chart
            .observations
            .iter()
            .find(|o| o.code.text.as_deref() == Some(test))
            .expect("observation");
        (o.status, o.issued.clone())
    };
    let next_day = render(&p, "2021-11-10T09:00");
    assert_eq!(status(&next_day, "Hemoglobin A1c").0, "final");
    assert_eq!(status(&next_day, "High-risk HPV DNA"), ("registered", None));
    let later = render(&p, "2021-11-16T09:00");
    assert_eq!(
        status(&later, "High-risk HPV DNA"),
        ("final", Some("2021-11-15T19:00:00+00:00".into()))
    );
}

#[test]
fn a_computed_result_waits_for_its_inputs() {
    let p = patient(json!([visit(
        "e1",
        "2024-01-01T09:00",
        json!({"labs": [{"panel": "Basic Metabolic Panel (BMP)", "collected": "2024-01-01T09:00", "resulted": "2024-01-01T15:00", "results": [
            {"test": "Creatinine", "value": 1.4, "resulted": "2024-01-03T10:00"}
        ]}]})
    )]));
    let status = |when: &str, test: &str| {
        let chart = render(&p, when);
        let o = chart
            .observations
            .iter()
            .find(|o| o.code.text.as_deref() == Some(test))
            .expect("observation");
        o.status
    };
    assert_eq!(status("2024-01-02T09:00", "Sodium"), "final");
    assert_eq!(status("2024-01-02T09:00", "Creatinine"), "registered");
    assert_eq!(
        status(
            "2024-01-02T09:00",
            "Estimated Glomerular Filtration Rate (Creatinine)"
        ),
        "registered"
    );
    assert_eq!(
        status(
            "2024-01-04T09:00",
            "Estimated Glomerular Filtration Rate (Creatinine)"
        ),
        "final"
    );
}

#[test]
fn a_test_stated_twice_in_one_draw_is_rejected() {
    let p = patient(json!([visit(
        "e1",
        "2024-01-01T09:00",
        json!({"labs": [{"panel": null, "collected": "2024-01-01T09:00", "results": [
            {"test": "Hemoglobin", "value": 6.0}, {"test": "Hemoglobin", "value": 14.0}
        ]}]})
    )]));
    let unit = Unit {
        unit: "unit-9999".into(),
        kind: "individual".into(),
        members: vec![seed()],
    };
    let catalog = Catalog::load(&paths::catalog()).expect("catalog");
    let (errors, _) = check::check(&unit, &[p], &catalog);
    assert!(
        errors.iter().any(|e| e.contains("stated twice")),
        "{errors:?}"
    );
}

fn errors_for(seed: Seed, p: Patient) -> Vec<String> {
    let unit = Unit {
        unit: "unit-9999".into(),
        kind: "individual".into(),
        members: vec![seed],
    };
    let catalog = Catalog::load(&paths::catalog()).expect("catalog");
    check::check(&unit, &[p], &catalog).0
}

fn note(body: &str) -> serde_json::Value {
    json!({"documents": [{"type": "Progress Notes", "body": body}]})
}

#[test]
fn notes_refer_to_clinicians_by_role() {
    let filler =
        "Follow-up visit; blood pressure reviewed and the plan was discussed with the patient.";
    for named in [
        "Seen with Dr. Okafor today.",
        "Signed: M. Lindqvist, MD.",
        "Colleen Fitzgerald, NP covered.",
        "Roomed by J. Ortiz, MA.",
        "Faxed back.\n-- B. Kowal, front desk",
    ] {
        let p = patient(json!([visit(
            "e1",
            "2024-01-01T09:00",
            note(&format!("{filler} {named}"))
        )]));
        let errors = errors_for(seed(), p);
        assert!(
            errors.iter().any(|e| e.contains("names a clinician")),
            "{named}: {errors:?}"
        );
    }
    let places = format!(
        "{filler} Moved here from West Springfield, MA; parents live in State College, PA. Seen by the PCP."
    );
    let p = patient(json!([visit("e1", "2024-01-01T09:00", note(&places))]));
    let errors = errors_for(seed(), p);
    assert!(
        !errors.iter().any(|e| e.contains("names a clinician")),
        "{errors:?}"
    );
}

#[test]
fn a_coded_seed_fixes_the_problem_list() {
    let mut coded = seed();
    coded.coded_anchors = true;
    coded.anchors = vec!["Essential hypertension".into()];
    let cond = |labels: &[(&str, &str)]| json!({"conditions": labels.iter().map(|(l, a)| json!({"label": l, "action": a})).collect::<Vec<_>>()});
    let has = |errors: &[String], what: &str| errors.iter().any(|e| e.contains(what));

    let missing = patient(json!([visit(
        "e1",
        "2024-01-01T09:00",
        cond(&[("Gout", "diagnosed")])
    )]));
    assert!(has(&errors_for(coded.clone(), missing), "never recorded"));

    let three_extra = cond(&[
        ("Essential hypertension", "diagnosed"),
        ("Gout", "diagnosed"),
        ("Insomnia", "diagnosed"),
        ("Rosacea", "diagnosed"),
    ]);
    let p = patient(json!([visit("e1", "2024-01-01T09:00", three_extra)]));
    assert!(has(&errors_for(coded.clone(), p), "did not plan"));

    let p = patient(json!([
        visit(
            "e1",
            "2024-01-01T09:00",
            cond(&[
                ("Essential hypertension", "diagnosed"),
                ("Ankle sprain", "diagnosed"),
                ("COVID-19", "diagnosed"),
                ("Gout", "diagnosed")
            ])
        ),
        visit(
            "e2",
            "2024-02-01T09:00",
            cond(&[("Ankle sprain", "resolved"), ("COVID-19", "resolved")])
        ),
    ]));
    let errors = errors_for(coded, p);
    assert!(
        !has(&errors, "did not plan") && !has(&errors, "never recorded"),
        "{errors:?}"
    );
}

#[test]
fn stories_redraw_only_unwritten_units_and_repeat_exactly() {
    let dir = std::env::temp_dir().join(format!("patient-bank-stories-{}", std::process::id()));
    let ledgers = dir.join("ledgers");
    std::fs::create_dir_all(&ledgers).expect("dirs");
    std::fs::write(ledgers.join("unit-0001.json"), "{}").expect("ledger marker");
    let path = dir.join("seeds.json");
    std::fs::copy(paths::seeds(), &path).expect("copy seeds");
    let before = Seeds::read(&path).expect("seeds");

    stories::run(&path, &ledgers).expect("first draw");
    let first = std::fs::read(&path).expect("read");
    std::fs::copy(paths::seeds(), &path).expect("copy seeds");
    stories::run(&path, &ledgers).expect("second draw");
    assert_eq!(
        first,
        std::fs::read(&path).expect("read"),
        "the same seeds and ledgers give the same draw"
    );

    let after = Seeds::read(&path).expect("seeds");
    let kept = |s: &Seeds| serde_json::to_string(s.unit("unit-0001").expect("unit")).expect("json");
    assert_eq!(
        kept(&before),
        kept(&after),
        "a unit with an accepted ledger keeps its seed"
    );
    let catalog = Catalog::load(&paths::catalog()).expect("catalog");
    let redrawn = &after.unit("unit-0002").expect("unit").members[0];
    assert!(redrawn.coded_anchors);
    assert!(
        redrawn
            .anchors
            .iter()
            .all(|a| catalog.conditions.contains_key(a)),
        "{:?}",
        redrawn.anchors
    );
    assert_eq!(
        redrawn.mrn,
        before.unit("unit-0002").expect("unit").members[0].mrn,
        "identity stays"
    );
    std::fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn malformed_drafts_fail_to_parse_instead_of_losing_content() {
    let part = |events: serde_json::Value| {
        serde_json::from_value::<Part>(
            json!({"patients": [{"mrn": "MRN99999999", "events": events}]}),
        )
    };
    assert!(
        part(json!({"id": "e1"})).is_err(),
        "an object where the event list belongs"
    );
    let good = json!({"id": "e1", "kind": "telephone", "start": "2024-01-01T09:00"});
    assert!(part(json!([good.clone()])).is_ok());
    let mut misspelled = good.clone();
    misspelled["documnets"] = json!([]);
    assert!(part(json!([misspelled])).is_err(), "an unknown field");
    let mut bad_time = good.clone();
    bad_time["start"] = json!("2024-01-01 09:00");
    assert!(
        part(json!([bad_time])).is_err(),
        "a time not written YYYY-MM-DDTHH:MM"
    );
    let mut bad_action = good;
    bad_action["medications"] = json!([{"drug": SERTRALINE, "action": "stoped"}]);
    assert!(part(json!([bad_action])).is_err(), "a misspelled action");
}

#[test]
fn an_accepted_ledger_round_trips() {
    let p = patient(json!([visit(
        "e1",
        "2024-01-01T09:00",
        json!({"vitals": {"bp": [128, 82], "temp_c": 36.8}})
    )]));
    let ledger = Ledger {
        unit: "unit-9999".into(),
        model: "test".into(),
        warnings: Vec::new(),
        patients: vec![p],
        terms: Terms::default(),
    };
    let text = serde_json::to_string(&ledger).expect("serialize");
    let back: Ledger = serde_json::from_str(&text).expect("parse");
    assert_eq!(serde_json::to_string(&back).expect("serialize"), text);
    assert!(
        text.contains("[128,82]") && text.contains("36.8"),
        "numbers keep their written form"
    );
}

#[test]
fn the_build_refuses_a_missing_ledger_directory() {
    let dir = std::env::temp_dir().join(format!("patient-bank-missing-{}", std::process::id()));
    let err = bank::run(
        &paths::seeds(),
        &dir.join("ledgres"),
        &dir.join("bank"),
        None,
    )
    .expect_err("missing ledgers");
    assert!(err.to_string().contains("not a directory of ledgers"));
    assert!(!dir.join("bank").exists(), "nothing written");
}

#[test]
fn the_build_refuses_an_output_that_overlaps_its_inputs() {
    let dir = std::env::temp_dir().join(format!("patient-bank-overlap-{}", std::process::id()));
    let ledgers = dir.join("bank/ledgers");
    std::fs::create_dir_all(&ledgers).expect("dirs");
    assert!(bank::refuse_overlap(&dir.join("bank"), &[&ledgers]).is_err());
    assert!(bank::refuse_overlap(&ledgers, &[&dir.join("bank")]).is_err());
    assert!(bank::refuse_overlap(&dir.join("out"), &[&ledgers]).is_ok());
    std::fs::remove_dir_all(&dir).expect("cleanup");
}

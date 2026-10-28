//! The bank's chronology rules, each on a small hand-written chart.

use patient_bank::catalog::Catalog;
use patient_bank::fhir::Chart;
use patient_bank::ledger::{Ledger, Part, Patient, Stamp, Terms};
use patient_bank::render::Renderer;
use patient_bank::seeds::{Seed, Seeds, Unit};
use patient_bank::timeline::{OrderStatus, ProblemStatus, Timeline};
use patient_bank::{bank, calendar, check, measure, paths, repeats};
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
        "chart_texture": [], "last_seen": "2026-07-20", "booked": false,
        "phone": "+1-413-555-0100"
    }))
    .expect("seed")
}

/// A two-unit seeds file in `dir`, so no test reads the real patient set.
/// A patient set of two units, each folder with its seed.json and no ledger.
fn patient_set(dir: &std::path::Path) -> std::path::PathBuf {
    let member = |mrn: &str| Seed {
        mrn: mrn.into(),
        ..seed()
    };
    for (unit, mrn) in ["unit-0001", "unit-0002"]
        .iter()
        .zip(["MRN99999998", "MRN99999999"])
    {
        let unit = Unit {
            unit: (*unit).into(),
            kind: "individual".into(),
            members: vec![member(mrn)],
        };
        let folder = dir.join("units").join(&unit.unit);
        std::fs::create_dir_all(&folder).expect("unit dir");
        std::fs::write(folder.join("seed.json"), unit.to_text().expect("text"))
            .expect("write seed");
    }
    dir.to_path_buf()
}

fn patient(events: serde_json::Value) -> Patient {
    serde_json::from_value(json!({
        "mrn": "MRN99999999", "given": "Ada", "family": "Example", "birth_date": "1976-03-02",
        "events": events
    }))
    .expect("patient")
}

/// A ledger rendered the way the bank renders it: measured first.
fn render(p: &Patient, when: &str) -> Chart {
    let catalog = Catalog::load(&paths::catalog()).expect("catalog");
    let terms = Terms::default();
    let (p, _) = measure::apply(&seed(), p, &catalog);
    Renderer {
        catalog: &catalog,
        terms: &terms,
    }
    .render(&seed(), &p, at(when))
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
fn a_problem_diagnosed_again_after_it_resolved_is_a_new_episode() {
    let cond = |action: &str| json!({"conditions": [{"label": "Iron deficiency anemia", "action": action}]});
    let p = patient(json!([
        visit("e1", "2018-09-17T10:00", cond("diagnosed")),
        visit("e2", "2019-09-27T10:00", cond("resolved")),
        visit("e3", "2026-07-21T10:00", cond("diagnosed")),
    ]));
    let tl = Timeline::new(&p);
    assert_eq!(tl.problems.len(), 2, "one problem-list row per episode");
    let (first, second) = (&tl.problems[0], &tl.problems[1]);
    assert_eq!(first.status(at("2019-01-01T00:00")), ProblemStatus::Active);
    assert_eq!(
        first.status(at("2026-08-01T00:00")),
        ProblemStatus::Resolved(at("2019-09-27T10:00"))
    );
    assert_eq!(second.status(at("2026-08-01T00:00")), ProblemStatus::Active);

    let then = render(&p, "2020-01-01T00:00");
    assert_eq!(then.conditions.len(), 1, "the second episode has not begun");
    assert_eq!(
        then.conditions[0].clinical_status.coding[0].code,
        "resolved"
    );
    let now = render(&p, "2026-08-27T12:00");
    let codes: Vec<&str> = now
        .conditions
        .iter()
        .map(|c| c.clinical_status.coding[0].code.as_str())
        .collect();
    assert_eq!(codes, ["resolved", "active"]);
    assert!(now.conditions[1].abatement_date_time.is_none());
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
    json!({"documents": [{"type": "Progress Notes", "author": "PCP", "body": body}]})
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
fn the_build_refuses_a_patient_set_with_no_seeds() {
    let dir = std::env::temp_dir().join(format!("patient-bank-missing-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("patinets/units")).expect("dir");
    let err = bank::run(&dir.join("patinets"), &dir.join("bank")).expect_err("no seeds");
    assert!(err.to_string().contains("no unit with a seed.json"));
    assert!(!dir.join("bank").exists(), "nothing written");
    std::fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn a_patient_set_is_the_seed_of_every_unit_folder() {
    let dir = std::env::temp_dir().join(format!("patient-bank-set-{}", std::process::id()));
    let seeds = Seeds::read(&patient_set(&dir)).expect("seeds");
    let units: Vec<&str> = seeds.units.iter().map(|u| u.unit.as_str()).collect();
    assert_eq!(units, ["unit-0001", "unit-0002"]);
    assert_eq!(
        seeds.member("MRN99999999").expect("member").0.unit,
        "unit-0002"
    );
    std::fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn the_build_refuses_an_output_that_overlaps_its_inputs() {
    let dir = std::env::temp_dir().join(format!("patient-bank-overlap-{}", std::process::id()));
    let patients = dir.join("bank/patients");
    std::fs::create_dir_all(&patients).expect("dirs");
    assert!(bank::refuse_overlap(&dir.join("bank"), &[&patients]).is_err());
    assert!(bank::refuse_overlap(&patients, &[&dir.join("bank")]).is_err());
    assert!(bank::refuse_overlap(&dir.join("out"), &[&patients]).is_ok());
    std::fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn a_visit_is_an_encounter_its_records_link_to() {
    let p = patient(json!([visit(
        "e1",
        "2025-10-14T10:00",
        json!({
            "end": "2025-10-14T10:30",
            "vitals": {"bp": [131, 83], "hr": 71},
            "immunizations": [{"vaccine": "Influenza, quadrivalent, injectable"}],
            "documents": [{"type": "Progress Notes", "author": "PCP",
                "body": "Flu shot today. BP a little high, recheck at the next visit."}]
        })
    )]));
    let during = render(&p, "2025-10-14T10:15");
    assert_eq!(during.encounters[0].status, "in-progress");
    let chart = render(&p, "2025-11-01T09:00");
    let enc = &chart.encounters[0];
    assert_eq!((enc.status, enc.class.code.as_str()), ("finished", "AMB"));
    let link = format!("Encounter/{}", enc.id);
    let imm = &chart.immunizations[0];
    assert_eq!(imm.vaccine_code.coding[0].code, "150");
    assert_eq!(
        imm.encounter.as_ref().map(|r| r.reference.as_str()),
        Some(link.as_str())
    );
    assert!(
        chart
            .observations
            .iter()
            .all(|o| o.encounter.as_ref().map(|r| &r.reference) == Some(&link))
    );
    let doc = &chart.documents[0];
    assert_eq!(doc.author[0].display, "PCP");
    assert_eq!(
        doc.context
            .as_ref()
            .map(|c| c.encounter[0].reference.as_str()),
        Some(link.as_str())
    );
}

#[test]
fn an_order_is_coded_by_what_it_orders_and_stays_open_until_closed() {
    let p = patient(json!([
        visit("e1", "2026-03-02T09:00", json!({"orders": [
            {"id": "cards", "action": "placed", "order": "Cardiology referral",
             "category": "referral", "to": "Cardiology", "reason": "palpitations"},
            {"id": "echo", "action": "placed", "order": "Transthoracic echocardiogram",
             "category": "imaging"},
            {"id": "fit", "action": "placed", "order": "Fecal Immunochemical Test",
             "category": "lab"}
        ]})),
        {"id": "e2", "kind": "telephone", "start": "2026-04-10T15:00", "orders": [
            {"id": "echo", "action": "cancelled"}
        ]},
        {"id": "e3", "kind": "outside_records", "start": "2026-05-20T11:00", "orders": [
            {"id": "cards", "action": "done"}
        ]}
    ]));
    let status = |chart: &Chart, what: &str| {
        chart
            .service_requests
            .iter()
            .find(|o| o.code.text.as_deref() == Some(what))
            .map(|o| o.status)
    };
    let open = render(&p, "2026-04-01T09:00");
    assert_eq!(status(&open, "Cardiology referral"), Some("active"));
    assert_eq!(
        status(&open, "Transthoracic echocardiogram"),
        Some("active")
    );
    let later = render(&p, "2026-06-01T09:00");
    assert_eq!(status(&later, "Cardiology referral"), Some("completed"));
    assert_eq!(
        status(&later, "Transthoracic echocardiogram"),
        Some("revoked")
    );
    assert!(render(&p, "2026-03-01T09:00").service_requests.is_empty());
    // each order carries its catalog code; a referral's specialty is its performer
    let catalog = catalog();
    let order = |what: &str| {
        open.service_requests
            .iter()
            .find(|o| o.code.text.as_deref() == Some(what))
            .expect(what)
    };
    let fit = &catalog.labs["Fecal Immunochemical Test"].code;
    assert_eq!(order("Fecal Immunochemical Test").code.coding[0].code, *fit);
    assert_eq!(
        order("Cardiology referral").performer_type[0].coding[0].code,
        catalog.specialties["Cardiology"].code
    );
}

#[test]
fn a_chart_ends_at_today_with_pending_results_and_booked_visits() {
    let p = patient(json!([
        visit(
            "e1",
            "2026-08-25T09:30",
            json!({
                "labs": [{"panel": null, "collected": "2026-08-25T09:30", "pending": true,
                    "results": [{"test": "High-risk HPV DNA"}]}],
                "documents": [{"type": "Progress Notes", "author": "PCP",
                    "body": "Pap and HPV co-test sent today. Will call with results."}]
            })
        ),
        visit(
            "e2",
            "2026-09-15T08:40",
            json!({"appointment": "booked", "reason": "BP recheck"})
        )
    ]));
    let now = render(&p, "2026-08-27T12:00");
    let hpv = now
        .observations
        .iter()
        .find(|o| o.code.text.as_deref() == Some("High-risk HPV DNA"))
        .expect("hpv");
    assert_eq!((hpv.status, hpv.issued.as_deref()), ("registered", None));
    // still pending however late the chart is rendered: what comes back is decided later
    let later = render(&p, "2027-01-01T09:00");
    assert!(later.observations.iter().all(|o| o.status == "registered"));
    assert_eq!(
        later.encounters.len(),
        1,
        "a booked visit is not an encounter"
    );

    let cal = calendar::visits(&seed(), &p);
    let slots: Vec<(&str, &str)> = cal.iter().map(|a| (&a.start[..10], a.status)).collect();
    assert_eq!(slots, [("2026-09-15", "booked")]);
    let seen = Seed {
        last_seen: day("2026-08-25"),
        booked: true,
        ..seed()
    };
    assert!(errors_for(seen, p).is_empty());
}

fn day(s: &str) -> patient_bank::ledger::Day {
    serde_json::from_value(json!(s)).expect("day")
}

#[test]
fn the_practice_last_saw_the_patient_on_the_seeds_day() {
    let body = "Doing well. BP fine today. Continue current plan.";
    let p = || {
        patient(json!([
            visit("e1", "2026-05-12T09:00", note(body)),
            visit("e2", "2026-07-20T09:00", note(body))
        ]))
    };
    assert!(errors_for(seed(), p()).is_empty());
    let earlier = Seed {
        last_seen: day("2026-05-12"),
        ..seed()
    };
    assert!(
        errors_for(earlier, p())
            .iter()
            .any(|e| e.contains("last_seen 2026-05-12"))
    );
    let wants = Seed {
        booked: true,
        ..seed()
    };
    assert!(
        errors_for(wants, p())
            .iter()
            .any(|e| e.contains("booked after today"))
    );
}

#[test]
fn care_after_the_last_visit_leaves_the_practice_lists_alone() {
    let body = "Doing well. BP fine today. Continue current plan.";
    let ed = |extra: serde_json::Value| {
        let mut ev = json!({"id": "e2", "kind": "ed_visit", "start": "2026-08-05T14:00",
            "documents": [{"type": "ED Notes", "author": "emergency medicine",
            "body": "Seen for wheezing, treated and sent home."}]});
        for (k, v) in extra.as_object().expect("object") {
            ev[k] = v.clone();
        }
        ev
    };
    let lists = |extra| {
        let p = patient(json!([
            visit("e1", "2026-07-20T09:00", note(body)),
            ed(extra)
        ]));
        errors_for(seed(), p)
            .iter()
            .any(|e| e.contains("lists do not change"))
    };
    assert!(!lists(json!({})));
    assert!(lists(
        json!({"conditions": [{"label": "Iron deficiency anemia", "action": "diagnosed"}]})
    ));
}

#[test]
fn check_keeps_everything_before_today_but_booked_visits() {
    let body = "Doing well. BP fine today. Continue current plan, labs before next visit.";
    let has = |events: serde_json::Value, what: &str| {
        let e = errors_for(seed(), patient(events));
        assert!(e.iter().any(|x| x.contains(what)), "{what}: {e:?}");
    };
    has(
        json!([visit("e1", "2026-09-02T09:00", note(body))]),
        "is not before today",
    );
    has(
        json!([visit(
            "e1",
            "2026-09-02T09:00",
            json!({"appointment": "booked", "documents": [
            {"type": "Progress Notes", "author": "PCP", "body": body}]})
        )]),
        "holds nothing yet",
    );
    has(
        json!([visit(
            "e1",
            "2026-08-02T09:00",
            json!({"appointment": "booked"})
        )]),
        "booked before today",
    );
    has(
        json!([visit(
            "e1",
            "2026-08-27T10:40",
            json!({
            "labs": [{"panel": null, "collected": "2026-08-27T10:40",
                "results": [{"test": "Hemoglobin A1c", "value": 6.1}]}],
            "documents": [{"type": "Progress Notes", "author": "PCP", "body": body}]})
        )]),
        "is not back by today",
    );
    has(
        json!([visit(
            "e1",
            "2026-08-25T09:00",
            json!({
            "labs": [{"panel": null, "collected": "2026-08-25T09:00", "pending": true,
                "results": [{"test": "Hemoglobin A1c", "value": 6.1}]}],
            "documents": [{"type": "Progress Notes", "author": "PCP", "body": body}]})
        )]),
        "has no value until it comes back",
    );
}

#[test]
fn check_holds_orders_and_missed_visits() {
    let body = "Doing well. BP fine today. Continue current plan, labs before next visit.";
    let errs = |events: serde_json::Value| errors_for(seed(), patient(events));
    let has = |errors: &[String], what: &str| errors.iter().any(|e| e.contains(what));

    let e = errs(json!([visit(
        "e1",
        "2024-01-01T09:00",
        json!({
            "orders": [{"id": "x", "action": "done"}],
            "documents": [{"type": "Progress Notes", "author": "PCP", "body": body}]
        })
    )]));
    assert!(has(&e, "closed before it was placed"), "{e:?}");
    let e = errs(json!([visit(
        "e1",
        "2024-01-01T09:00",
        json!({"appointment": "no-show", "vitals": {"bp": [121, 77]}})
    )]));
    assert!(has(&e, "records nothing clinical"), "{e:?}");
}

#[test]
fn repeats_find_what_charts_share_but_not_their_own_or_the_practices() {
    let ledger = |unit: &str, tsh: f64, body: &str| Ledger {
        unit: unit.into(),
        model: "test".into(),
        warnings: Vec::new(),
        patients: vec![patient(json!([visit(
            "e1",
            "2024-01-01T09:00",
            json!({
                "labs": [{"panel": null, "collected": "2024-01-01T09:00",
                    "results": [{"test": "Thyroid Stimulating Hormone", "value": tsh}]}],
                "documents": [{"type": "Progress Notes", "author": "PCP", "body": body}]
            })
        )]))],
        terms: Terms::default(),
    };
    let template = "Exam: lungs clear heart regular rate and rhythm.";
    // a relative's name and a phrase in two charts, a default TSH in three, and the
    // practice's exam template in all ten
    let units: Vec<Ledger> = (0..10)
        .map(|i| {
            let extra = if i < 2 {
                "Her brother Walter drove her in and asked whether she would need surgery soon."
            } else {
                "Doing well, no changes today."
            };
            let tsh = if i < 3 {
                2.14
            } else {
                1.0 + f64::from(i) / 7.0
            };
            ledger(
                &format!("unit-{i:04}"),
                tsh,
                &format!("{template} Ada says hi. {extra}"),
            )
        })
        .collect();
    let own = (0..10)
        .map(|i| (format!("unit-{i:04}"), ["Ada".to_string()].into()))
        .collect();
    let r = repeats::find(&units, &own, 2);
    let has = |items: &[(String, usize)], what: &str| items.iter().any(|(k, _)| k.contains(what));
    assert!(has(&r.names, "Walter"), "{:?}", r.names);
    assert!(!has(&r.names, "Ada"), "a patient's own name is theirs");
    assert!(
        has(
            &r.phrases,
            "her brother walter drove her in and asked whether she would need surgery soon"
        ),
        "{:?}",
        r.phrases
    );
    assert!(
        !has(&r.phrases, "lungs clear heart regular"),
        "the practice template repeats"
    );
    assert!(
        has(&r.values, "Thyroid Stimulating Hormone 2.14"),
        "{:?}",
        r.values
    );
}

#[test]
fn code_measures_from_the_state_the_author_records() {
    let visits: Vec<serde_json::Value> = (0..12)
        .map(|i| {
            let mut extra = json!({
                "measure": ["bp", "hr", "weight_kg"],
                "labs": [{"panel": null, "collected": format!("2024-{:02}-03T09:00", i + 1),
                    "results": [{"test": "Hemoglobin A1c"}, {"test": "Glucose"}]}]
            });
            if i == 0 {
                extra["state"] =
                    json!({"bp": [148, 94], "weight_kg": 91.0, "labs": {"Hemoglobin A1c": 7.9}});
            }
            visit(
                &format!("e{i:02}"),
                &format!("2024-{:02}-03T09:00", i + 1),
                extra,
            )
        })
        .collect();
    let p = patient(json!(visits));
    let (once, errors) = measure::apply(&seed(), &p, &catalog());
    assert!(errors.is_empty(), "{errors:?}");
    let (again, _) = measure::apply(&seed(), &p, &catalog());
    assert_eq!(
        serde_json::to_string(&once).expect("json"),
        serde_json::to_string(&again).expect("json"),
        "the same ledger measures the same values"
    );
    let systolic: Vec<f64> = once
        .events
        .iter()
        .map(|e| patient_bank::ledger::num(&e.vitals.sets()[0].bp.as_ref().expect("bp")[0]))
        .collect();
    let a1c: Vec<f64> = once
        .events
        .iter()
        .map(|e| patient_bank::ledger::num(e.labs[0].results[0].value.as_ref().expect("a1c")))
        .collect();
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
    assert!((mean(&systolic) - 148.0).abs() < 8.0, "{systolic:?}");
    assert!((mean(&a1c) - 7.9).abs() < 0.5, "{a1c:?}");
    let distinct: std::collections::HashSet<i64> = systolic.iter().map(|x| *x as i64).collect();
    assert!(
        distinct.len() > 6,
        "readings vary like a cuff's: {systolic:?}"
    );
    // a weight with no level set is an error, not a made-up number
    let unset = patient(json!([visit(
        "e1",
        "2024-01-01T09:00",
        json!({"measure": ["weight_kg"]})
    )]));
    assert!(!measure::apply(&seed(), &unset, &catalog()).1.is_empty());
}

#[test]
fn a_draw_keeps_its_derived_members_and_set_levels_consistent() {
    let draw = |at: &str| {
        json!({"state": {"labs": {"Hemoglobin": 9.1, "Ferritin": 5, "Total Cholesterol": 240}},
            "labs": [{"panel": "Lipid Panel", "collected": at,
                      "results": [{"test": "Total Cholesterol"}, {"test": "Non-HDL Cholesterol"}]},
                     {"panel": "Complete Blood Count (CBC)", "collected": at, "results": []},
                     {"panel": null, "collected": at, "results": [{"test": "Ferritin"}]}]})
    };
    let visits: Vec<serde_json::Value> = (0..20)
        .map(|i| {
            let at = format!("{}-{:02}-03T09:00", 2010 + i / 12, i % 12 + 1);
            visit(&format!("e{i:02}"), &at, draw(&at))
        })
        .collect();
    let (p, errors) = measure::apply(&seed(), &patient(json!(visits)), &catalog());
    assert!(errors.is_empty(), "{errors:?}");
    for e in &p.events {
        let value = |k: usize, t: &str| {
            e.labs[k]
                .results
                .iter()
                .find(|r| r.test == t)
                .map(|r| r.value.clone())
        };
        // the draw's calculated members agree with the values they come from
        let lipid = |t: &str| patient_bank::ledger::num(value(0, t).flatten().as_ref().expect(t));
        let total = lipid("Low Density Lipoprotein")
            + lipid("High Density Lipoprotein")
            + lipid("Triglycerides") / 5.0;
        assert!(
            (lipid("Total Cholesterol") - total).abs() <= 0.5,
            "total {total}"
        );
        assert_eq!(
            lipid("Non-HDL Cholesterol"),
            lipid("Total Cholesterol") - lipid("High Density Lipoprotein")
        );
        // a set total cholesterol moves the members it is calculated from
        assert!((total - 240.0).abs() < 25.0, "total cholesterol {total}");
        // a set level reaches the panel member the author did not name
        let hgb =
            patient_bank::ledger::num(value(1, "Hemoglobin").flatten().as_ref().expect("hgb"));
        assert!((hgb - 9.1).abs() < 1.5, "hemoglobin {hgb}");
        // a low level strays in proportion to itself, not to the normal range
        let ferritin =
            patient_bank::ledger::num(value(2, "Ferritin").flatten().as_ref().expect("ferritin"));
        assert!((2.0..=8.0).contains(&ferritin), "ferritin {ferritin}");
    }
}

#[test]
fn a_stated_calculated_test_fixes_its_open_input_and_every_lab_formula_holds() {
    let lipids = |results: serde_json::Value| {
        patient(json!([visit(
            "e1",
            "2024-01-03T09:00",
            json!({"labs": [{"panel": "Lipid Panel", "collected": "2024-01-03T09:00",
                "results": results}],
                "documents": [{"type": "Progress Notes", "author": "PCP", "body": "Lipids drawn."}]})
        )]))
    };
    // the source gives total cholesterol, LDL and HDL: triglycerides follow from them
    let stated = lipids(json!([{"test": "Total Cholesterol", "value": 190},
        {"test": "Low Density Lipoprotein", "value": 100},
        {"test": "High Density Lipoprotein", "value": 45}]));
    let (p, errors) = measure::apply(&seed(), &stated, &catalog());
    assert!(errors.is_empty(), "{errors:?}");
    let tg = p.events[0].labs[0]
        .results
        .iter()
        .find(|r| r.test == "Triglycerides")
        .and_then(|r| r.value.as_ref())
        .map(patient_bank::ledger::num);
    assert_eq!(tg, Some(225.0), "TC 190 = LDL 100 + HDL 45 + TG/5");
    assert!(
        errors_for(seed(), p)
            .iter()
            .all(|e| !e.contains("disagrees"))
    );

    // a Martin-Hopkins LDL is a lab's real calculation; one no formula gives is not
    let all = |ldl: i64| {
        lipids(json!([{"test": "Total Cholesterol", "value": 210},
            {"test": "Low Density Lipoprotein", "value": ldl},
            {"test": "High Density Lipoprotein", "value": 45},
            {"test": "Triglycerides", "value": 200}]))
    };
    let disagrees = |ldl| {
        let (p, _) = measure::apply(&seed(), &all(ldl), &catalog());
        errors_for(seed(), p)
            .iter()
            .any(|e| e.contains("Total Cholesterol 210 disagrees"))
    };
    assert!(!disagrees(140), "TG/f with f = 8 is Martin-Hopkins");
    assert!(disagrees(60), "no lab's formula puts LDL 60 here");

    // a result answered in text stays text: no number is solved into it, and the
    // formula it blocks holds back none of the other members
    let withheld = lipids(json!([{"test": "Total Cholesterol", "value": 260},
        {"test": "Triglycerides", "value": 640},
        {"test": "Low Density Lipoprotein", "text": "Not calculated (triglycerides >400 mg/dL)"}]));
    let (p, errors) = measure::apply(&seed(), &withheld, &catalog());
    assert!(errors.is_empty(), "{errors:?}");
    let result = |test: &str| {
        p.events[0].labs[0]
            .results
            .iter()
            .find(|r| r.test == test)
            .unwrap_or_else(|| panic!("{test} left out"))
    };
    let ldl = result("Low Density Lipoprotein");
    assert!(ldl.value.is_none() && ldl.text.is_some(), "{ldl:?}");
    assert!(result("High Density Lipoprotein").value.is_some());
    assert!(result("Non-HDL Cholesterol").value.is_some());
}

#[test]
fn no_known_drug_allergies_ends_when_a_drug_allergy_is_recorded() {
    let allergy = |substance: &str, category: Option<&str>| {
        let mut a = json!({"substance": substance});
        if let Some(c) = category {
            a["category"] = json!(c);
        }
        a
    };
    let chart = |later: serde_json::Value| {
        patient(json!([
            visit(
                "e1",
                "2024-01-03T09:00",
                json!({"allergies": [allergy("No known drug allergies", None)]})
            ),
            visit("e2", "2024-06-03T09:00", json!({"allergies": [later]})),
        ]))
    };
    let nkda_status = |p: &Patient, when: &str| {
        render(p, when)
            .allergies
            .iter()
            .find(|a| a.code.text.as_deref() == Some("No known drug allergies"))
            .map(|a| a.clinical_status.coding[0].code.clone())
    };
    let drug = chart(allergy("Penicillin", None));
    assert_eq!(
        nkda_status(&drug, "2024-03-01T00:00").as_deref(),
        Some("active")
    );
    assert_eq!(
        nkda_status(&drug, "2024-07-01T00:00").as_deref(),
        Some("inactive")
    );
    let food = chart(allergy("Peanut", None));
    assert_eq!(
        nkda_status(&food, "2024-07-01T00:00").as_deref(),
        Some("active"),
        "a food allergy leaves no known drug allergies true"
    );

    // saying there are none while one is listed contradicts the list
    let again = patient(json!([
        visit(
            "e1",
            "2024-01-03T09:00",
            json!({"allergies": [allergy("Penicillin", None)]})
        ),
        visit(
            "e2",
            "2024-06-03T09:00",
            json!({"allergies": [allergy("No known drug allergies", None)]})
        ),
    ]));
    let errors = errors_for(seed(), again);
    assert!(
        errors
            .iter()
            .any(|e| e.contains("while \"Penicillin\" is on the allergy list")),
        "{errors:?}"
    );
    // the same contradiction inside one visit leaves no known drug allergies inactive
    let same = patient(json!([visit(
        "e1",
        "2024-01-03T09:00",
        json!({"allergies": [allergy("Penicillin", None), allergy("No known drug allergies", None)]})
    )]));
    assert_eq!(
        nkda_status(&same, "2024-03-01T00:00").as_deref(),
        Some("inactive")
    );
    assert!(
        errors_for(seed(), same)
            .iter()
            .any(|e| e.contains("while \"Penicillin\" is on the allergy list"))
    );
    // an allergen the catalog lacks says what kind it is
    let errors = errors_for(seed(), chart(allergy("Dust mites", None)));
    assert!(
        errors.iter().any(|e| e.contains("\"category\"")),
        "{errors:?}"
    );
}

fn catalog() -> Catalog {
    Catalog::load(&paths::catalog()).expect("catalog")
}

#[test]
fn a_patients_own_normal_stays_in_range() {
    let visits: Vec<serde_json::Value> = (0..40)
        .map(|i| {
            visit(
                &format!("e{i:02}"),
                &format!("20{:02}-03-03T09:00", 10 + i / 2),
                json!({"labs": [{"panel": null, "collected": format!("20{:02}-03-03T09:00", 10 + i / 2),
                    "results": [{"test": "Sodium"}, {"test": "Chloride"}, {"test": "Glucose"}]}]}),
            )
        })
        .collect();
    let (p, errors) = measure::apply(&seed(), &patient(json!(visits)), &catalog());
    assert!(errors.is_empty(), "{errors:?}");
    for e in &p.events {
        for r in &e.labs[0].results {
            let ((lo, hi), _) = patient_bank::labs::normal(&r.test, false).expect("range");
            let v = patient_bank::ledger::num(r.value.as_ref().expect("value"));
            assert!((lo..=hi).contains(&v), "{} {v} outside {lo}-{hi}", r.test);
        }
    }
}

#[test]
fn a_concept_the_catalog_lacks_is_coded_by_its_verified_code() {
    let draw_with = |loinc: serde_json::Value, range: Option<serde_json::Value>| {
        let mut result =
            json!({"test": "Calcitonin", "loinc": loinc, "unit": "pg/mL", "value": 500});
        if let Some(range) = range {
            result["range"] = range;
        }
        patient(json!([visit(
            "e1",
            "2024-01-01T09:00",
            json!({"labs": [{"panel": null, "collected": "2024-01-01T09:00", "results": [result]}],
            "allergies": [{"substance": "Dust mites", "snomed": "260147004", "category": "environment"}],
            "procedures": [{"procedure": "Cesarean section", "snomed": "11466000",
                "performed": "2024-01-01T09:30"}],
            "orders": [{"id": "mri", "action": "placed", "category": "imaging",
                "order": "MRI brain with and without contrast"},
                // the catalog's procedure of this label is unverified; its study is not
                {"id": "pft", "action": "placed", "category": "procedure", "order": "Spirometry"}],
            "documents": [{"type": "Progress Notes", "author": "PCP", "body": "Calcitonin sent."}]})
        )]))
    };
    let draw = |loinc: serde_json::Value| draw_with(loinc, Some(json!({"high": 10})));
    let unknown = |e: &Vec<String>| e.iter().any(|x| x.contains("unknown lab test"));
    assert!(unknown(&errors_for(seed(), draw(json!(null)))));
    let p = draw(json!("1992-7"));
    assert!(!unknown(&errors_for(seed(), p.clone())));
    // a measured test the catalog lacks states its reference range, or that none applies
    let unranged = |e: &Vec<String>| e.iter().any(|x| x.contains("give its reference range"));
    assert!(unranged(&errors_for(
        seed(),
        draw_with(json!("1992-7"), None)
    )));
    assert!(!unranged(&errors_for(seed(), p.clone())));
    assert!(!unranged(&errors_for(
        seed(),
        draw_with(json!("1992-7"), Some(json!({})))
    )));

    let catalog = Catalog::load(&paths::catalog()).expect("catalog");
    let mut terms = Terms::default();
    terms.labs.insert(
        "Calcitonin".into(),
        patient_bank::ledger::VerifiedCode {
            code: "1992-7".into(),
            display: "Calcitonin [Mass/volume] in Serum or Plasma".into(),
            unit: Some("pg/mL".into()),
            range: p.events[0].labs[0].results[0].range.clone(),
            receipt: "test".into(),
            ..Default::default()
        },
    );
    let snomed = |code: &str, display: &str| patient_bank::ledger::VerifiedCode {
        code: code.into(),
        display: display.into(),
        receipt: "test".into(),
        ..Default::default()
    };
    terms
        .allergens
        .insert("Dust mites".into(), snomed("260147004", "House dust mite"));
    terms.procedures.insert(
        "Cesarean section".into(),
        snomed("11466000", "Cesarean section"),
    );
    // an order for a study the unit verified is coded, and check does not call it uncoded
    let uncoded = |terms: &Terms| {
        check::uncoded_orders(std::slice::from_ref(&p), &catalog, terms)
            .iter()
            .any(|w| w.contains("MRI brain with and without contrast"))
    };
    assert!(uncoded(&terms));
    assert!(
        !check::uncoded_orders(std::slice::from_ref(&p), &catalog, &terms)
            .iter()
            .any(|w| w.contains("Spirometry")),
        "a verified entry of the label codes the order"
    );
    terms.reports.insert(
        "MRI brain with and without contrast".into(),
        snomed("24587-8", "MR Brain WO and W contrast IV"),
    );
    assert!(!uncoded(&terms));
    let chart = Renderer {
        catalog: &catalog.with_terms(&terms),
        terms: &terms,
    }
    .render(&seed(), &p, at("2024-02-01T00:00"))
    .0;
    assert!(
        chart
            .observations
            .iter()
            .any(|o| o.code.coding.iter().any(|c| c.code == "1992-7")),
        "the verified test renders with its LOINC code"
    );
    // the author's one-sided range renders as given and flags the value above it
    let calcitonin = (chart.observations.iter())
        .find(|o| o.code.coding.iter().any(|c| c.code == "1992-7"))
        .expect("calcitonin");
    let range = &calcitonin.reference_range[0];
    assert!(range.low.is_none() && range.high.is_some());
    assert!((calcitonin.interpretation.iter()).any(|i| i.coding.iter().any(|c| c.code == "H")));
    // each result keeps the range stated for it, not the first one its name was given
    let other = draw_with(json!("1992-7"), Some(json!({"high": 600})));
    let chart2 = Renderer {
        catalog: &catalog.with_terms(&terms),
        terms: &terms,
    }
    .render(&seed(), &other, at("2024-02-01T00:00"))
    .0;
    let calcitonin = (chart2.observations.iter())
        .find(|o| o.code.coding.iter().any(|c| c.code == "1992-7"))
        .expect("calcitonin");
    let high = calcitonin.reference_range[0].high.as_ref().expect("high");
    assert_eq!(
        serde_json::to_value(high).expect("json")["value"].as_f64(),
        Some(600.0)
    );
    assert!(calcitonin.interpretation.is_empty());
    // the catalog gives its own labs' ranges
    let stated = patient(json!([visit(
        "e1",
        "2024-01-01T09:00",
        json!({"labs": [{"panel": null, "collected": "2024-01-01T09:00", "results": [
            {"test": "Hemoglobin A1c", "value": 6.1, "range": {"high": 7}}
        ]}]})
    )]));
    assert!(
        errors_for(seed(), stated)
            .iter()
            .any(|e| e.contains("in the catalog, which gives its range"))
    );
    let coded = |codes: Vec<&patient_bank::fhir::CodeableConcept>, code: &str| {
        codes
            .iter()
            .any(|c| c.coding.iter().any(|x| x.code == code))
    };
    assert!(coded(
        chart.allergies.iter().map(|a| &a.code).collect(),
        "260147004"
    ));
    assert_eq!(chart.allergies[0].category, vec!["environment".to_string()]);
    assert!(coded(
        chart.procedures.iter().map(|x| &x.code).collect(),
        "11466000"
    ));
    assert!(coded(
        chart.service_requests.iter().map(|x| &x.code).collect(),
        "24587-8"
    ));
}

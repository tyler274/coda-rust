//! Captured source envelopes feed the actual Hint filer and Postgres delivery owner.
//! Chrome acquisition is qualified separately; these tests start at its output.
use local_product::physician_sources::hint::ChartCapture;
use local_product::physician_sources::{
    Bucket, ChartRequest, ChartScope, READ_TOOL, SEARCH_TOOL, SourcePullOutput,
};
use local_product::runtime::Store;
use local_product::source_status::PartialDelivery;
use serde_json::{Value, json};
use std::{fs, path::PathBuf};

fn proof_output(name: &str) -> PathBuf {
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/verification/record-adapters");
    fs::create_dir_all(&root).unwrap();
    root.join(name)
}

fn tool(
    store_records: &[local_product::physician_sources::SourceRevision],
    workspace: &str,
    version: u64,
    name: &str,
    arguments: Value,
) -> Value {
    let request = ChartRequest::parse(name, &arguments, ChartScope::Chat).unwrap();
    let output = request.run(workspace, version, store_records).unwrap();
    serde_json::from_str(output.text().unwrap()).unwrap()
}

#[tokio::test]
#[ignore = "requires explicit disposable Postgres"]
async fn captured_hint_files_usable_items_and_other_with_honest_partial_delivery() {
    let evidence =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/verification/record-adapters");
    fs::create_dir_all(&evidence).unwrap();
    let capture: Value = serde_json::from_str(include_str!("support/hint-capture.json")).unwrap();
    let patient_id = "sbx-pat-QItKCySDsdET";
    let workspace = format!("hint-mapping-{}", uuid::Uuid::new_v4());
    let source_id = format!("hint-{}", uuid::Uuid::new_v4());
    let store = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    store.migrate().await.unwrap();
    let filed = serde_json::from_value::<ChartCapture>(capture.clone())
        .unwrap()
        .file(&workspace, &source_id, patient_id);
    let SourcePullOutput::Partial {
        patient,
        records,
        succeeded_kinds,
        failed_parts,
    } = filed
    else {
        panic!("bounded acquisition must remain partial");
    };
    assert_eq!(
        records.len(),
        13,
        "all identifiable browser-shaped records survive"
    );
    assert!(
        failed_parts
            .iter()
            .any(|part| part.reason.contains("ident"))
    );
    assert!(
        failed_parts
            .iter()
            .any(|part| part.reason.contains("patient binding mismatch"))
    );
    assert!(
        failed_parts
            .iter()
            .any(|part| part.reason.contains("no progress"))
    );
    assert!(
        failed_parts
            .iter()
            .any(|part| part.part.contains("full note detail"))
    );
    assert!(
        !records
            .iter()
            .any(|record| record.candidate_id.ends_with("wrong-patient"))
    );
    assert_eq!(
        records
            .iter()
            .find(|record| record.candidate_id.ends_with("/note-1"))
            .unwrap()
            .envelope
            .as_ref()
            .unwrap()
            .bucket,
        Bucket::Notes
    );
    assert_eq!(
        records
            .iter()
            .find(|record| record.candidate_id.ends_with("/vital-1"))
            .unwrap()
            .envelope
            .as_ref()
            .unwrap()
            .bucket,
        Bucket::Vitals
    );
    for (id, bucket, title) in [
        ("allergy-1", Bucket::Allergies, "Penicillin"),
        ("social-1", Bucket::History, "Tobacco use"),
        ("family-1", Bucket::History, "Diabetes"),
        ("vaccine-1", Bucket::Immunizations, "Influenza"),
        ("surgery-1", Bucket::Other, "Source surgery entry"),
        ("task-1", Bucket::Other, "Review retained records"),
    ] {
        let envelope = records
            .iter()
            .find_map(|record| {
                record
                    .envelope
                    .as_ref()
                    .filter(|envelope| envelope.native_id == id)
            })
            .unwrap();
        assert_eq!(envelope.bucket, bucket);
        assert_eq!(envelope.title.as_deref(), Some(title));
    }
    for attempt in 0..2 {
        let (receipt, status) = store
            .record_partial_pull(PartialDelivery {
                practice_id: "mapping-fixture",
                workspace_id: &workspace,
                source_id: &source_id,
                source_label: "Hint capture",
                patient: patient.as_ref(),
                succeeded_kinds: &succeeded_kinds,
                records: &records,
                failed_parts: &failed_parts,
                attempted_at: 100 + attempt,
                ticket: None,
            })
            .await
            .unwrap();
        assert_eq!(receipt.changed, if attempt == 0 { 13 } else { 0 });
        assert!(status.last_good_read_at.is_none());
    }
    let observed = store.sources(&workspace).await.unwrap();
    assert_eq!(observed.sources, {
        let mut values = records.clone();
        values.sort_by(|left, right| left.candidate_id.cmp(&right.candidate_id));
        values
    });
    let other = tool(
        &observed.sources,
        &workspace,
        observed.version,
        SEARCH_TOOL,
        json!({"bucket":"other","query":""}),
    );
    assert_eq!(other["results"].as_array().unwrap().len(), 4);
    let read = tool(
        &observed.sources,
        &workspace,
        observed.version,
        READ_TOOL,
        json!({"references":other["results"].as_array().unwrap().iter().map(|record| record["reference"].clone()).collect::<Vec<_>>(),"max_result_bytes":16384}),
    );
    let rendered = read.to_string();
    assert!(rendered.contains("keep me"));
    assert!(rendered.contains("New native subtype"));
    assert!(rendered.contains("unavailable_parts"));
    // Wrong capture scope cannot turn into a successful Other record.
    assert!(matches!(
        serde_json::from_value::<ChartCapture>(capture.clone())
            .unwrap()
            .file(&workspace, &source_id, "wrong-request"),
        SourcePullOutput::Failed { .. }
    ));

    let mut changed = capture;
    let notes = changed["sections"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|section| section["section"] == "notes")
        .unwrap();
    notes["result"]["body"]["items"][0]["body"]["noteType"] = json!("UNFAMILIAR");
    let SourcePullOutput::Partial { records: later, .. } =
        serde_json::from_value::<ChartCapture>(changed)
            .unwrap()
            .file(&workspace, &source_id, patient_id)
    else {
        panic!("partial capture");
    };
    let old_note = observed
        .sources
        .iter()
        .find(|record| record.candidate_id.ends_with("/note-1"))
        .unwrap();
    let new_note = later
        .iter()
        .find(|record| record.candidate_id.ends_with("/note-1"))
        .unwrap();
    assert_eq!(
        old_note.candidate_id, new_note.candidate_id,
        "classification is not identity"
    );
    store.record_source(new_note).await.unwrap();
    assert_eq!(store.sources(&workspace).await.unwrap().sources.len(), 13);
    let earlier = store
        .sources_at(&workspace, observed.version)
        .await
        .unwrap();
    assert_eq!(
        earlier
            .sources
            .iter()
            .find(|record| record.candidate_id == old_note.candidate_id)
            .unwrap(),
        old_note
    );
    fs::write(evidence.join("hint-persisted-read.json"), serde_json::to_vec_pretty(&json!({"search_other":other,"read_other":read,"failed_parts":failed_parts,"old_revision":old_note.revision,"new_revision":new_note.revision})).unwrap()).unwrap();
}

#[tokio::test]
#[ignore = "requires explicit disposable Postgres"]
async fn mixed_fhir_batch_persists_usable_records_and_readable_attachment_siblings() {
    use local_product::physician_sources::PatientSource;
    use physician_sample::synthetic_chart::{SOURCE_ID, SyntheticCharts};
    let root = std::env::temp_dir().join(format!("mapping-mixed-fhir-{}", uuid::Uuid::new_v4()));
    let chart = root.join("patients/patient-1");
    fs::create_dir_all(&chart).unwrap();
    fs::write(chart.join("Patient.ndjson"), r#"{"resourceType":"Patient","id":"patient-1","name":[{"given":["Fixture"],"family":"Patient"}]}"#).unwrap();
    fs::write(chart.join("Observation.ndjson"), concat!(
        "{\"resourceType\":\"Observation\",\"id\":\"bp\",\"subject\":{\"reference\":\"Patient/patient-1\"},\"code\":{\"coding\":[{\"system\":\"http://loinc.org\",\"code\":\"85354-9\"}]},\"component\":[{\"code\":{\"text\":\"Systolic\"},\"valueQuantity\":{\"value\":120,\"unit\":\"mmHg\"}}]}\n",
        "{invalid-json}\n",
        "{\"resourceType\":\"Observation\",\"id\":\"foreign\",\"subject\":{\"reference\":\"https://example.test/fhir/Patient/other-patient\"}}\n",
        "{\"resourceType\":\"Basic\",\"id\":\"unknown\",\"code\":{\"text\":\"Retained unsupported source kind\"}}\n"
    )).unwrap();
    fs::write(chart.join("DocumentReference.ndjson"), r#"{"resourceType":"DocumentReference","id":"doc","content":[{"attachment":{"contentType":"application/pdf","url":"https://example.test/scan.pdf"}},{"attachment":{"contentType":"text/plain","data":"SGVsbG8gd29ybGQ="}},{"attachment":{"contentType":"text/plain","data":"invalid***"}}]}"#).unwrap();
    let workspace = format!("mixed-fhir-{}", uuid::Uuid::new_v4());
    let source = SyntheticCharts::open(&root);
    let SourcePullOutput::Partial {
        records,
        failed_parts,
        patient,
        succeeded_kinds,
    } = source
        .pull_patient("mapping-fixture", "patient-1", &workspace)
        .await
    else {
        panic!("mixed FHIR batch must report partial");
    };
    assert_eq!(records.len(), 4);
    assert_eq!(failed_parts.len(), 2);
    assert!(
        failed_parts
            .iter()
            .any(|part| part.reason.contains("patient binding mismatch"))
    );
    let store = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    store.migrate().await.unwrap();
    store
        .record_partial_pull(PartialDelivery {
            practice_id: "mapping-fixture",
            workspace_id: &workspace,
            source_id: SOURCE_ID,
            source_label: "FHIR fixture",
            patient: patient.as_ref(),
            succeeded_kinds: &succeeded_kinds,
            records: &records,
            failed_parts: &failed_parts,
            attempted_at: 100,
            ticket: None,
        })
        .await
        .unwrap();
    let observed = store.sources(&workspace).await.unwrap();
    let other = observed
        .sources
        .iter()
        .find(|record| record.record_type == "Basic")
        .unwrap();
    assert_eq!(other.envelope.as_ref().unwrap().bucket, Bucket::Other);
    let doc = observed
        .sources
        .iter()
        .find(|record| record.record_type == "DocumentReference")
        .unwrap();
    assert_eq!(doc.readable_parts[0].text, "Hello world");
    let opened = tool(
        &observed.sources,
        &workspace,
        observed.version,
        READ_TOOL,
        json!({"references":[{"candidate_id":doc.candidate_id,"revision":doc.revision,"cursor":null}],"max_result_bytes":16384}),
    );
    assert_eq!(
        opened["results"][0]["unavailable_parts"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        opened["results"][0]["readable_parts"][0]["text"],
        "Hello world"
    );
    let evidence =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/verification/record-adapters");
    fs::create_dir_all(&evidence).unwrap();
    fs::write(
        evidence.join("fhir-persisted-read.json"),
        serde_json::to_vec_pretty(&json!({"read":opened,"failed_parts":failed_parts})).unwrap(),
    )
    .unwrap();
    // Completing the acquisition refreshes the source even though the same
    // retained document still has unreadable content.
    let usable_lines = records
        .iter()
        .filter(|record| !matches!(record.record_type.as_str(), "Patient" | "DocumentReference"))
        .map(|record| record.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(chart.join("Observation.ndjson"), &usable_lines).unwrap();
    let SourcePullOutput::Complete { patient, records } = source
        .pull_patient("mapping-fixture", "patient-1", &workspace)
        .await
    else {
        panic!("unread content must not block a completed acquisition");
    };
    let (_, refreshed) = store
        .record_complete_pull(local_product::source_status::CompleteDelivery {
            practice_id: "mapping-fixture",
            workspace_id: &workspace,
            source_id: SOURCE_ID,
            source_label: "FHIR fixture",
            patient: &patient,
            records: &records,
            pulled_at: 101,
            ticket: None,
        })
        .await
        .unwrap();
    assert_eq!(refreshed.last_good_read_at, Some(101));
    assert_eq!(
        store
            .sources(&workspace)
            .await
            .unwrap()
            .sources
            .iter()
            .find(|record| record.candidate_id == doc.candidate_id)
            .unwrap()
            .envelope
            .as_ref()
            .unwrap()
            .unavailable_parts
            .len(),
        2
    );

    // A later empty successful document collection detects a missing document
    // even when an unrelated malformed observation makes the whole pull partial.
    fs::write(chart.join("DocumentReference.ndjson"), "").unwrap();
    fs::write(
        chart.join("Observation.ndjson"),
        format!("{usable_lines}\ninvalid-json"),
    )
    .unwrap();
    let SourcePullOutput::Partial {
        patient,
        records,
        succeeded_kinds,
        failed_parts,
    } = source
        .pull_patient("mapping-fixture", "patient-1", &workspace)
        .await
    else {
        panic!("failed item keeps the acquisition partial");
    };
    let (_, partial) = store
        .record_partial_pull(PartialDelivery {
            practice_id: "mapping-fixture",
            workspace_id: &workspace,
            source_id: SOURCE_ID,
            source_label: "FHIR fixture",
            patient: patient.as_ref(),
            records: &records,
            succeeded_kinds: &succeeded_kinds,
            failed_parts: &failed_parts,
            attempted_at: 102,
            ticket: None,
        })
        .await
        .unwrap();
    assert_eq!(partial.last_good_read_at, Some(101));
    let local_product::source_status::PullOutcome::Partial {
        missing_candidates, ..
    } = partial.latest_attempt.unwrap().outcome
    else {
        panic!("partial persisted outcome");
    };
    assert_eq!(missing_candidates, vec![doc.candidate_id.clone()]);
    fs::remove_dir_all(root).unwrap();
}

/// External recorded inputs keep private captures out of the repository. The
/// upstream bodies, expected dispositions and provenance are supplied separately
/// from the production adapter; no test mapper replaces the ingestion path.
#[tokio::test]
#[ignore = "requires SOURCE_REPLAY_ROOT and disposable Postgres"]
async fn recorded_hint_and_mimic_survive_the_actual_adapters_and_persisted_reads() {
    use local_product::physician_sources::PatientSource;
    use local_product::source_status::CompleteDelivery;
    use physician_sample::synthetic_chart::{SOURCE_ID, SyntheticCharts};
    let inputs = PathBuf::from(std::env::var("SOURCE_REPLAY_ROOT").unwrap());
    let capture: Value =
        serde_json::from_slice(&fs::read(inputs.join("hint-replayed.json")).unwrap()).unwrap();
    let bodies: Value =
        serde_json::from_slice(&fs::read(inputs.join("hint-bodies.json")).unwrap()).unwrap();
    let workspace = format!("recorded-hint-{}", uuid::Uuid::new_v4());
    let patient_id = bodies["patient"]["id"].as_str().unwrap();
    let SourcePullOutput::Partial {
        patient,
        records,
        succeeded_kinds,
        failed_parts,
    } = serde_json::from_value::<ChartCapture>(capture)
        .unwrap()
        .file(&workspace, "hint", patient_id)
    else {
        panic!("bounded Hint acquisition remains partial")
    };
    assert_eq!(
        records.len(),
        13,
        "all thirteen captured records must survive"
    );
    assert!(
        failed_parts
            .iter()
            .any(|part| part.part.contains("full note detail")),
        "archived list-only inputs cannot qualify note details"
    );
    assert!(
        failed_parts
            .iter()
            .any(|part| part.part.starts_with("appointments")),
        "archived inputs contain no appointment response"
    );
    assert!(
        failed_parts
            .iter()
            .any(|part| part.part.starts_with("files")),
        "archived inputs contain no file response"
    );
    let store = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    store.migrate().await.unwrap();
    for attempt in 0..2 {
        let (receipt, _) = store
            .record_partial_pull(PartialDelivery {
                practice_id: "recorded-proof",
                workspace_id: &workspace,
                source_id: "hint",
                source_label: "Hint capture",
                patient: patient.as_ref(),
                succeeded_kinds: &succeeded_kinds,
                records: &records,
                failed_parts: &failed_parts,
                attempted_at: 100 + attempt,
                ticket: None,
            })
            .await
            .unwrap();
        assert_eq!(receipt.changed, if attempt == 0 { 13 } else { 0 });
    }
    let observed = store.sources(&workspace).await.unwrap();
    let mut evidence = Vec::new();
    for (section, bucket) in [
        ("patient", "patient"),
        ("diagnoses", "problems"),
        ("allergies", "allergies"),
        ("medications", "medications"),
        ("surgeries", "other"),
        ("social-history", "history"),
        ("family-history", "history"),
        ("vaccines", "immunizations"),
        ("general", "other"),
        ("notes", "notes"),
        ("tasks", "other"),
    ] {
        let items = if section == "patient" {
            vec![&bodies[section]]
        } else {
            bodies[section][if section == "notes" {
                "data"
            } else {
                "entities"
            }]
            .as_array()
            .unwrap()
            .iter()
            .collect()
        };
        for item in items {
            let id = item[if section == "patient" {
                "id"
            } else if section == "notes" {
                "noteId"
            } else {
                "ident"
            }]
            .as_str()
            .unwrap();
            let bucket = if section == "notes" {
                match item["noteType"].as_str() {
                    Some("NOTE") => "notes",
                    Some("VITALS") => "vitals",
                    _ => "other",
                }
            } else {
                bucket
            };
            let candidate = format!("hint/{section}/{id}");
            let record = observed
                .sources
                .iter()
                .find(|record| record.candidate_id == candidate)
                .unwrap();
            // Independently enumerate the observed payment fields covered by
            // the agreed patient billing exclusion. Clinical siblings survive.
            let mut expected_body = item.clone();
            if section == "patient" {
                for field in [
                    "billing_address",
                    "autopay",
                    "credit_expires_soon",
                    "credit_in_cents",
                    "exempt_from_credit_card_fees",
                    "has_payment_method",
                    "overdue_in_cents",
                    "owed_in_cents",
                    "request_pay_info_sent_at",
                    "should_charge_credit_card_fees",
                    "stripe_link",
                    "uses_billing_address",
                    "payer",
                    "payees",
                ] {
                    expected_body.as_object_mut().unwrap().remove(field);
                }
            }
            let envelope = record.envelope.as_ref().unwrap();
            let expected_title = match section {
                "medications" => &item["productName"],
                "social-history" => &item["question"],
                "family-history" => &item["diagnosis"],
                "notes" => &item["data"]["title"],
                "tasks" => &item["text"],
                _ => &item["name"],
            };
            assert_eq!(
                envelope.title.as_deref(),
                expected_title
                    .as_str()
                    .filter(|value| !value.trim().is_empty())
            );
            assert_eq!(
                envelope.status.as_deref(),
                item["status"]
                    .as_str()
                    .filter(|value| !value.trim().is_empty())
            );
            let expected_dates = match section {
                "patient" => vec![("birth_date", "dob")],
                "notes" => vec![
                    ("occurred", "eventTimestamp"),
                    ("recorded", "recordCreationTimestamp"),
                ],
                "medications" => vec![("start", "start"), ("end", "end")],
                "allergies" | "social-history" | "general" => {
                    vec![("start", "startDate"), ("end", "endDate")]
                }
                _ => vec![],
            }
            .into_iter()
            .filter_map(|(role, field)| {
                item[field]
                    .as_str()
                    .filter(|value| !value.trim().is_empty())
                    .map(|value| json!({"role":role,"field":field,"value":value}))
            })
            .collect::<Vec<_>>();
            assert_eq!(
                serde_json::to_value(&envelope.dates).unwrap(),
                json!(expected_dates),
                "indexed dates: {section}"
            );
            let code_field = match section {
                "diagnoses" | "allergies" => Some("icd10CodeId"),
                "family-history" => Some("icd10Code"),
                _ => None,
            };
            if let Some(field) = code_field {
                assert_eq!(envelope.codes.len(), 1);
                assert_eq!(envelope.codes[0].field, format!("{field}.code"));
                assert_eq!(
                    envelope.codes[0].code.as_deref(),
                    item[field]["code"].as_str()
                );
                assert_eq!(
                    envelope.codes[0].display.as_deref(),
                    item[field]["description"]
                        .as_str()
                        .filter(|value| !value.trim().is_empty())
                );
                assert!(
                    envelope.codes[0].system.is_none(),
                    "no guessed code namespace"
                );
            } else {
                assert!(envelope.codes.is_empty());
            }
            assert_eq!(
                serde_json::from_str::<Value>(&record.content).unwrap(),
                expected_body,
                "retained Hint body: {section}"
            );
            evidence.push(assert_persisted_read(
                &observed.sources,
                &workspace,
                observed.version,
                record,
                bucket,
            ));
        }
    }
    let expected: Vec<Value> =
        serde_json::from_slice(&fs::read(inputs.join("mimic-expected.json")).unwrap()).unwrap();
    let patients = expected
        .iter()
        .map(|row| row["patient"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let source = SyntheticCharts::open(&inputs.join("mimic"));
    for patient_id in patients {
        let workspace = format!("recorded-mimic-{}", uuid::Uuid::new_v4());
        let SourcePullOutput::Complete { patient, records } = source
            .pull_patient("recorded-proof", patient_id, &workspace)
            .await
        else {
            panic!("MIMIC acquired subset must file without item losses for {patient_id}")
        };
        let wanted = expected
            .iter()
            .filter(|row| row["patient"] == patient_id)
            .collect::<Vec<_>>();
        assert_eq!(records.len(), wanted.len());
        for attempt in 0..2 {
            let (receipt, _) = store
                .record_complete_pull(CompleteDelivery {
                    practice_id: "recorded-proof",
                    workspace_id: &workspace,
                    source_id: SOURCE_ID,
                    source_label: "MIMIC replay",
                    patient: &patient,
                    records: &records,
                    pulled_at: 100 + attempt,
                    ticket: None,
                })
                .await
                .unwrap();
            assert_eq!(receipt.changed, if attempt == 0 { wanted.len() } else { 0 });
        }
        let observed = store.sources(&workspace).await.unwrap();
        for row in wanted {
            let candidate = format!("{SOURCE_ID}/{}", row["native"].as_str().unwrap());
            let record = observed
                .sources
                .iter()
                .find(|record| record.candidate_id == candidate)
                .unwrap();
            assert_eq!(
                record.content,
                row["raw"].as_str().unwrap(),
                "source bytes must survive: {candidate}"
            );
            let read = assert_persisted_read(
                &observed.sources,
                &workspace,
                observed.version,
                record,
                row["bucket"].as_str().unwrap(),
            );
            evidence.push(json!({"origin":row["origin"], "case":row["case"], "read":read}));
            // A medication reference is usable evidence only when its actual
            // supporting definition can also be found and opened.
            let native: Value = serde_json::from_str(&record.content).unwrap();
            for field in ["medicationReference", "specimen"] {
                if let Some(target) = native[field]["reference"].as_str() {
                    let target = format!("{SOURCE_ID}/{target}");
                    assert!(
                        observed
                            .sources
                            .iter()
                            .any(|record| record.candidate_id == target),
                        "missing acquired support: {target}"
                    );
                }
            }
        }
    }
    fs::write(
        proof_output("persisted-reads.json"),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();
    println!(
        "Recorded source proof: 13 Hint records and {} MIMIC records retained, discovered, read and delivered twice without duplication",
        expected.len()
    );
}

fn assert_persisted_read(
    records: &[local_product::physician_sources::SourceRevision],
    workspace: &str,
    version: u64,
    record: &local_product::physician_sources::SourceRevision,
    bucket: &str,
) -> Value {
    let hits = tool(
        records,
        workspace,
        version,
        SEARCH_TOOL,
        json!({"bucket":bucket,"query":record.envelope.as_ref().unwrap().native_id}),
    );
    assert!(
        hits["results"]
            .as_array()
            .unwrap()
            .iter()
            .any(|hit| hit["reference"]["candidate_id"] == record.candidate_id),
        "expected bucket search hit: {} in {bucket}",
        record.candidate_id
    );
    let read = tool(
        records,
        workspace,
        version,
        READ_TOOL,
        json!({"references":[{"candidate_id":record.candidate_id,"revision":record.revision,"cursor":null}],"max_result_bytes":65536}),
    );
    assert_eq!(read["results"][0]["content"], record.content);
    assert_eq!(read["results"][0]["bucket"], bucket);
    read
}

/// Previously acquired sandbox responses pass through filing, Postgres and exact reads.
/// The supplied capture pairs qualify the acquisition boundary without running Chrome.
#[tokio::test]
#[ignore = "requires HINT_LIVE_PROOF and disposable Postgres"]
async fn captured_hint_pages_details_and_links_survive_delivery() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let input = root.join(std::env::var("HINT_LIVE_PROOF").unwrap());
    for (input, expected_count, repeated) in [
        (input.clone(), 30, 0),
        (input.with_file_name("live-proof-multipage.json"), 27, 3),
    ] {
        let proof: Value = serde_json::from_slice(&fs::read(&input).unwrap()).unwrap();
        let evidence = input.parent().unwrap();
        let store = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
        store.migrate().await.unwrap();
        let mut readbacks = Vec::new();
        let captures = proof["captures"].as_array().unwrap();
        assert_eq!(captures.len(), 4);
        for (index, live) in captures.iter().enumerate() {
            let patient_id = live["request"]["patient_id"].as_str().unwrap();
            let output_path = evidence.join(format!("worker-{repeated}-{index}.json"));
            let capture: Value = serde_json::from_slice(&fs::read(output_path).unwrap()).unwrap();
            for section in capture["sections"].as_array().unwrap() {
                let native = live["sections"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|native| native["section"] == section["section"])
                    .unwrap();
                assert_eq!(
                    section["result"]["body"]["items"], native["result"]["body"]["items"],
                    "capture and acquisition receipt must retain the same native items"
                );
            }
            let workspace = format!("live-hint-{}", uuid::Uuid::new_v4());
            let SourcePullOutput::Partial {
                patient,
                records,
                succeeded_kinds,
                failed_parts,
            } = serde_json::from_value::<ChartCapture>(capture)
                .unwrap()
                .file(&workspace, "hint", patient_id)
            else {
                panic!("appointment date-window limit must remain explicit")
            };
            assert_eq!(
                records.len(),
                if index == 0 { expected_count - 3 } else { 1 }
            );
            assert_eq!(
                failed_parts.len(),
                if index == 0 { repeated + 1 } else { 1 }
            );
            assert_eq!(
                failed_parts
                    .iter()
                    .filter(|part| part.reason.contains("repeated native identity"))
                    .count(),
                if index == 0 { repeated } else { 0 }
            );
            assert!(
                failed_parts
                    .iter()
                    .any(|part| part.part.contains("appointment date range"))
            );
            assert!(
                failed_parts
                    .iter()
                    .any(|part| part.reason.contains("outside it were not acquired"))
            );
            let (receipt, status) = store
                .record_partial_pull(PartialDelivery {
                    practice_id: "live-hint-proof",
                    workspace_id: &workspace,
                    source_id: "hint",
                    source_label: "Hint sandbox",
                    patient: patient.as_ref(),
                    records: &records,
                    succeeded_kinds: &succeeded_kinds,
                    failed_parts: &failed_parts,
                    attempted_at: 100,
                    ticket: None,
                })
                .await
                .unwrap();
            assert_eq!(receipt.changed, records.len());
            assert!(
                status.last_good_read_at.is_none(),
                "bounded scope is not whole-chart freshness"
            );
            let observed = store.sources(&workspace).await.unwrap();
            assert_eq!(observed.sources.len(), records.len());
            for record in &observed.sources {
                let envelope = record.envelope.as_ref().unwrap();
                let expected_bucket = match envelope.source_kind.kind.as_str() {
                    "patient" => "patient",
                    "diagnoses" => "problems",
                    "allergies" => "allergies",
                    "medications" => "medications",
                    "social-history" | "family-history" => "history",
                    "vaccines" => "immunizations",
                    "notes" => match envelope.source_kind.category.as_deref() {
                        Some("NOTE") => "notes",
                        Some("VITALS") => "vitals",
                        _ => "other",
                    },
                    "surgeries" | "general" | "tasks" | "appointments" => "other",
                    kind => panic!("unqualified live sample kind {kind}"),
                };
                let retained: Value = serde_json::from_str(&record.content).unwrap();
                assert!(envelope.unavailable_parts.is_empty());
                let link = envelope.native_link.as_ref().unwrap();
                match envelope.source_kind.kind.as_str() {
                    "notes" => {
                        assert!(link.ends_with(&format!("/notes/{}", envelope.native_id)));
                        assert_eq!(retained["data"]["costToDoctor"], Value::Null);
                        assert_eq!(retained["data"]["priceForPatient"], Value::Null);
                        if envelope.source_kind.category.as_deref() == Some("NOTE") {
                            assert!(
                                retained["data"]["formattedText"]
                                    .as_str()
                                    .unwrap()
                                    .contains("SYNTHETIC")
                            );
                        }
                    }
                    "tasks" => assert!(link.ends_with(&format!("taskId={}", envelope.native_id))),
                    "appointments" => {
                        assert!(link.ends_with(&format!("appointmentId={}", envelope.native_id)))
                    }
                    _ => assert!(link.ends_with(&format!("/patients/{patient_id}/chart"))),
                }
                readbacks.push(assert_persisted_read(
                    &observed.sources,
                    &workspace,
                    observed.version,
                    record,
                    expected_bucket,
                ));
            }
        }
        assert_eq!(readbacks.len(), expected_count);
        fs::write(
            proof_output(&format!("captured-persisted-reads-{repeated}.json")),
            serde_json::to_vec_pretty(&readbacks).unwrap(),
        )
        .unwrap();
        println!(
            "Four captured sandbox patients: {expected_count} distinct records; {repeated} duplicate page items reported; persisted reads and native links verified"
        );
    }
}

/// Hint's page-code lab reader consumes data.results as either one object or an
/// array. This is a source-shape fixture, not a populated live sandbox lab.
#[test]
fn hint_page_code_lab_shape_retains_results_and_attachment_limits() {
    for results in [
        json!({"id":"result-1","code":[{"code":"2823-3","system":"http://loinc.org"}],"valueQuantity":{"value":4.1,"unit":"mmol/L"},"ranges":[{"text":"native range"}],"interpretation":[{"text":"native interpretation"}]}),
        json!([{"id":"result-1","valueString":"Source result text","issued":"2026-11-15","performer":["Source laboratory"]}]),
    ] {
        let native = json!({"noteId":"lab-1","patientId":"patient-1","noteType":"LAB",
            "data":{"results":results,"orderedFrom":"Source laboratory","labReferenceId":"native-reference"},
            "files":[{"url":"https://example.test/retained.pdf"}]});
        let capture = json!({"request":{"patient_id":"patient-1"},"sections":[
            {"section":"notes","state":"done","result":{"body":{"items":[{"body":native,
                "unavailable_parts":[{"part":"attachments","reason":"native file references retained; attachment bodies were not acquired"}]}],"failed_parts":[]}}}]});
        let SourcePullOutput::Partial {
            records,
            succeeded_kinds,
            failed_parts,
            ..
        } = serde_json::from_value::<ChartCapture>(capture)
            .unwrap()
            .file("lab-shape", "hint", "patient-1")
        else {
            panic!("other chart sections were not acquired")
        };
        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(
            serde_json::from_str::<Value>(&record.content).unwrap(),
            native
        );
        let envelope = record.envelope.as_ref().unwrap();
        assert_eq!(
            envelope.bucket,
            Bucket::Other,
            "lab subtype is deliberately unqualified"
        );
        assert_eq!(envelope.source_kind.category.as_deref(), Some("LAB"));
        assert_eq!(envelope.unavailable_parts.len(), 1);
        assert!(succeeded_kinds.iter().any(|kind| kind.kind == "notes"));
        assert!(
            !failed_parts
                .iter()
                .any(|part| part.part.starts_with("notes")),
            "unread attachment content is a record limit, not incomplete acquisition"
        );
        let read = assert_persisted_read(&records, "lab-shape", 1, record, "other");
        assert_eq!(read["results"][0]["content"], record.content);
    }
}

#[test]
fn hint_secondary_patient_bindings_reject_foreign_items_and_keep_siblings() {
    let capture = json!({"request":{"patient_id":"patient-1"},"sections":[
        {"section":"appointments","state":"done","result":{"body":{"items":[
            {"body":{"ident":"foreign-appointment","attendees":[{"patient_id":"patient-2"}]},"unavailable_parts":[]},
            {"body":{"ident":"own-appointment","attendees":[{"patient_id":"patient-1"}]},"unavailable_parts":[]}
        ],"failed_parts":[]}}},
        {"section":"files","state":"done","result":{"body":{"items":[
            {"body":{"Key":"public/patient-2/file.pdf"},"unavailable_parts":[]},
            {"body":{"Key":"public/patient-1/file.pdf"},"unavailable_parts":[]}
        ],"failed_parts":[]}}}
    ]});
    let SourcePullOutput::Partial {
        records,
        failed_parts,
        ..
    } = serde_json::from_value::<ChartCapture>(capture)
        .unwrap()
        .file("bindings", "hint", "patient-1")
    else {
        panic!("foreign items and missing sections leave partial delivery")
    };
    assert_eq!(records.len(), 2);
    assert!(
        records
            .iter()
            .all(|record| !record.content.contains("patient-2"))
    );
    let mismatches = failed_parts
        .iter()
        .filter(|part| part.reason.contains("patient binding mismatch"))
        .collect::<Vec<_>>();
    assert_eq!(mismatches.len(), 2);
    assert!(
        mismatches
            .iter()
            .all(|part| part.reason.contains("patient-2"))
    );
}

#[test]
fn empty_patient_capture_reports_missing_identity() {
    let capture = json!({"request":{"patient_id":"patient-1"},"sections":[{"section":"patient","state":"done","result":{"body":{"items":[],"failed_parts":[]}}}]});
    let SourcePullOutput::Partial {
        patient,
        failed_parts,
        ..
    } = serde_json::from_value::<ChartCapture>(capture)
        .unwrap()
        .file("workspace", "hint", "patient-1")
    else {
        panic!("missing identity must not complete");
    };
    assert!(patient.is_none());
    assert!(
        failed_parts.iter().any(
            |part| part.part == "patient" && part.reason == "patient identity was not acquired"
        )
    );
}

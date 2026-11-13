//! Qualification tests for multimodal Spruce photo/text attachments and
//! page-by-page reading of clinical PDFs, inbound Spruce faxes, and chart scans:
//! - Reading unique clinical details from saved photo (`.png`, `.jpg`) and text
//!   (`.txt`, `.md`, `.csv`) attachments through the harness read contract while
//!   preserving original file bytes and SHA-256 digests.
//! - Isolating corrupted/unreadable attachments with named failures while sibling
//!   attachments remain readable.
//! - Reproducible local PDF/fax/scan reader benchmark saved to
//!   `target/verification/pdf-scan-reader-benchmark.json`.
//! - Page-by-page (`1..=page_count`) reading across multi-page PDFs, Spruce faxes,
//!   and noisy chart scans, per-page corruption isolation, and versioned document
//!   retention so corrected documents (`v2`) preserve exact `v1` page citations.

use std::path::PathBuf;

use local_product::{
    ModelBinding, ModelRoles,
    attachments::{
        AttachmentMediaKind, AttachmentPart, AttachmentReadFailure, PagedDocumentInput,
        READ_DOCUMENT_PAGE_TOOL, SavedAttachmentInput, VersionedDocumentArchive,
        build_corrupted_page_test_pdf, build_test_pdf, document_page_tool,
        execute_document_page_read_from_revisions, ingest_attachment_batch, ingest_paged_document,
        is_document_page_tool, render_synthetic_clinical_jpeg, render_synthetic_clinical_png,
        render_synthetic_clinical_rgb_png, run_document_reader_benchmark,
    },
    patient_reviews::{
        COMPLETE_PATIENT_REVIEW_TOOL, CompleteReviewArgs, CompleteReviewDisposition,
    },
    physician_chat::{ChatAssignment, PhysicianSkills},
    physician_sources::{
        ChartRequest, ChartScope, MedicalRecordNumber, READ_TOOL, SourcePatient, SourceRevision,
        read_tool,
    },
    runtime::{
        Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store,
        WorkStatus, async_trait,
        chat_operations::{RecordLimitKind, inspect_record_readability},
        concerns::ExactRecordReference,
        dispatch::StoredObservationChecker,
        engine::{
            self, AcceptedInput, Event, Invocation, Item, Product, ProductMessage, ToolOutcome,
            Turn, WindowSeed,
            recipe::{Finish, Observer, Provider, Request, RequestBinding, RequestRecipe},
        },
        jobs::PracticeJobHost,
        reviews::PatientReviewReceiver,
        sources::SourcePull,
    },
    sha256_hex,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

struct AttachmentHarnessHost {
    workspace: String,
    current_sources: Vec<SourceRevision>,
    retained_revisions: Vec<SourceRevision>,
    events: Vec<Event>,
}

#[async_trait]
impl Product for AttachmentHarnessHost {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<ProductMessage>>, engine::ProductError> {
        Ok(Some(vec![]))
    }

    async fn admit_binding(&mut self, _: &RequestBinding) -> Result<(), engine::ProductError> {
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        name == READ_TOOL || is_document_page_tool(name)
    }

    fn read_result(&mut self, _: &str) -> engine::recipe::ToolFuture<Option<String>> {
        Box::pin(async { Ok(None) })
    }

    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), engine::ProductError> {
        self.events.push(event);
        Ok(())
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> engine::recipe::ToolFuture {
        let workspace = self.workspace.clone();
        let current_sources = self.current_sources.clone();
        let retained_revisions = self.retained_revisions.clone();
        let name = call.name.clone();
        Box::pin(async move {
            if name == READ_TOOL {
                let request = match ChartRequest::parse(&name, &arguments, ChartScope::Chat) {
                    Ok(request) => request,
                    Err(reason) => return Ok(ToolOutcome::refused(reason.to_string())),
                };
                return request
                    .run(&workspace, 1, &current_sources)
                    .map_err(Into::into);
            }
            if is_document_page_tool(&name) {
                return Ok(execute_document_page_read_from_revisions(
                    &arguments,
                    &retained_revisions,
                ));
            }
            Ok(ToolOutcome::refused(format!("unknown tool `{name}`")))
        })
    }

    async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::ProductError> {
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        _: &Turn,
        _: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<WindowSeed, engine::ProductError> {
        Ok(WindowSeed {
            items: vec![],
            note_references: vec![],
        })
    }
}

async fn run_attachment_harness<P: Provider>(
    workspace: &str,
    current_sources: Vec<SourceRevision>,
    retained_revisions: Vec<SourceRevision>,
    prompt: &str,
    provider: &mut P,
) -> Vec<Event> {
    let recipe = RequestRecipe {
        model: "controlled-attachment-reader".into(),
        context_window_tokens: 200_000,
        max_input_tokens: 150_000,
        options: json!({"effort": "high"}),
        instructions: "Read patient chart attachments and paged documents accurately.".into(),
        tools: vec![
            read_tool(ChartScope::Chat).function(),
            document_page_tool().function(),
        ],
    };
    let mut request = recipe.bind(None).request(Vec::new());
    request.items.clear();
    let work = request
        .freeze()
        .unwrap()
        .recover(
            "attempt-att-1",
            engine::recovery::Records {
                events: vec![],
                allowance_after: 0,
            },
            engine::recovery::Messages {
                sources: &[],
                partial: None,
            },
            engine::recovery::Inputs {
                accepted: vec![AcceptedInput {
                    id: "work-att-1".into(),
                    items: vec![Item::ProductMessage(ProductMessage {
                        id: "work-att-1".into(),
                        role: engine::history::Role::User,
                        text: prompt.to_owned(),
                    })],
                }],
                source_feedback: None,
            },
        )
        .unwrap();
    let mut host = AttachmentHarnessHost {
        workspace: workspace.to_owned(),
        current_sources,
        retained_revisions,
        events: Vec::new(),
    };
    work.run(
        provider,
        &mut host,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    host.events
}

#[tokio::test]
async fn spruce_photo_and_text_attachments_read_unique_details_through_harness_read_tool_and_preserve_original_sha256()
 {
    // Unique details embedded ONLY inside the pixel bitmap and text attachment body:
    // - PNG photo pixel-only cuff code: `CUFF-HM-7741` and `SYS 168 DIA 104`
    // - JPEG photo pixel-only lot number: `LOT-K9482`
    // - Plain-text diary unique code: `SYMPTOM-LOG-8829`
    let png_bytes = render_synthetic_clinical_png(
        &["HOME BP MONITOR", "SYS 168 DIA 104", "CUFF-HM-7741"],
        false,
    );
    let png_sha = sha256_hex(&png_bytes);
    assert!(!png_bytes.windows(15).any(|w| w == b"CODA_RASTER_PNG"));

    let rgb_png_bytes = render_synthetic_clinical_rgb_png(&["RGB DERM PHOTO", "LESION 6MM"], false);
    let rgb_png_sha = sha256_hex(&rgb_png_bytes);
    let rgb_batch = ingest_attachment_batch(&[SavedAttachmentInput {
        attachment_id: "spruce-derm-rgb".into(),
        revision: "v1".into(),
        workspace: "pw-spruce-1".into(),
        source_id: "spruce".into(),
        authored_date: "2026-11-11".into(),
        title: "Inbound 24-bit RGB dermatology photograph".into(),
        media_kind: AttachmentMediaKind::PngPhoto,
        bytes: rgb_png_bytes,
        expected_sha256: Some(rgb_png_sha),
    }]);
    let Ok(AttachmentPart::Image {
        extracted_text: rgb_extracted,
        ..
    }) = &rgb_batch.records[0].outcome
    else {
        panic!("expected decoded 24-bit RGB PNG AttachmentPart::Image");
    };
    assert!(
        rgb_extracted.contains("LESION 6MM")
            && rgb_batch.records[0]
                .source_revision
                .readable_parts
                .iter()
                .any(|p| p.text.contains("LESION 6MM")),
        "24-bit RGB PNG must decode cleanly: {rgb_extracted}"
    );

    let jpg_bytes = render_synthetic_clinical_jpeg(&["AMOXICILLIN 500MG", "LOT-K9482"], false);
    let jpg_sha = sha256_hex(&jpg_bytes);
    assert!(
        !jpg_bytes.windows(15).any(|w| w == b"CODA_RASTER_PNG"),
        "JPEG fixture must use standard DCT/Huffman pixels with zero sidecar APP2 markers"
    );

    let txt_body = "Home ankle swelling log:\nMorning standing weight 194.6 lb.\nDiary code: SYMPTOM-LOG-8829\n";
    let txt_bytes = txt_body.as_bytes().to_vec();
    let txt_sha = sha256_hex(&txt_bytes);

    let md_body = "# Urgent Care Note\n- Afebrile, clear lungs, supportive care advised.\n";
    let csv_body = "date,systolic,diastolic\n2026-11-11,168,104\n";

    let batch = ingest_attachment_batch(&[
        SavedAttachmentInput {
            attachment_id: "spruce-bp-photo".into(),
            revision: "v1".into(),
            workspace: "pw-spruce-1".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Inbound patient photograph".into(),
            media_kind: AttachmentMediaKind::PngPhoto,
            bytes: png_bytes.clone(),
            expected_sha256: Some(png_sha.clone()),
        },
        SavedAttachmentInput {
            attachment_id: "spruce-rx-jpeg".into(),
            revision: "v1".into(),
            workspace: "pw-spruce-1".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Inbound bottle photograph".into(),
            media_kind: AttachmentMediaKind::JpegPhoto,
            bytes: jpg_bytes.clone(),
            expected_sha256: Some(jpg_sha.clone()),
        },
        SavedAttachmentInput {
            attachment_id: "spruce-symptom-diary".into(),
            revision: "v1".into(),
            workspace: "pw-spruce-1".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Inbound patient note".into(),
            media_kind: AttachmentMediaKind::PlainText,
            bytes: txt_bytes.clone(),
            expected_sha256: Some(txt_sha.clone()),
        },
        SavedAttachmentInput {
            attachment_id: "spruce-uc-md".into(),
            revision: "v1".into(),
            workspace: "pw-spruce-1".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-09".into(),
            title: "Inbound markdown summary".into(),
            media_kind: AttachmentMediaKind::MarkdownText,
            bytes: md_body.as_bytes().to_vec(),
            expected_sha256: None,
        },
        SavedAttachmentInput {
            attachment_id: "spruce-bp-csv".into(),
            revision: "v1".into(),
            workspace: "pw-spruce-1".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Inbound CSV log".into(),
            media_kind: AttachmentMediaKind::CsvTable,
            bytes: csv_body.as_bytes().to_vec(),
            expected_sha256: None,
        },
    ]);

    assert_eq!(batch.records.len(), 5);
    assert_eq!(batch.records[0].original_bytes, png_bytes);
    assert_eq!(batch.records[0].original_sha256, png_sha);
    assert_eq!(batch.records[1].original_bytes, jpg_bytes);
    assert_eq!(batch.records[1].original_sha256, jpg_sha);
    assert_eq!(batch.records[2].original_bytes, txt_bytes);
    assert_eq!(batch.records[2].original_sha256, txt_sha);

    let Ok(AttachmentPart::Image {
        data_uri: png_data_uri,
        original_sha256: decoded_png_sha,
        ..
    }) = &batch.records[0].outcome
    else {
        panic!("expected decoded PNG AttachmentPart::Image");
    };
    assert!(png_data_uri.starts_with("data:image/png;base64,"));
    assert_eq!(decoded_png_sha, &png_sha);
    assert_eq!(
        batch.records[0].source_revision.provenance["multimodal_part"]["type"],
        json!("input_image")
    );
    assert_eq!(
        batch.records[1].source_revision.provenance["multimodal_part"]["media_type"],
        json!("image/jpeg")
    );

    for rec in &batch.records {
        assert!(
            inspect_record_readability(&rec.source_revision).is_none(),
            "valid ingested attachment `{}` must have zero readability limits",
            rec.attachment_id
        );
    }

    // Prove via a harness run where the prompt contains NEITHER detail and the run reads
    // BOTH `CUFF-HM-7741` (from the PNG pixels) and `SYMPTOM-LOG-8829` (from the `.txt` file)
    // through `read_patient_sources`.
    struct ReadBothDetailsProvider {
        step: usize,
        synthesized_answer: String,
    }
    impl Provider for ReadBothDetailsProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => {
                    let prompt_blob = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        !prompt_blob.contains("CUFF-HM-7741")
                            && !prompt_blob.contains("SYMPTOM-LOG-8829"),
                        "unique attachment details must not exist anywhere prior to calling READ_TOOL"
                    );
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-spruce-attachments".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [
                                    {"candidate_id": "spruce-bp-photo", "revision": "v1", "cursor": null},
                                    {"candidate_id": "spruce-rx-jpeg", "revision": "v1", "cursor": null},
                                    {"candidate_id": "spruce-symptom-diary", "revision": "v1", "cursor": null}
                                ],
                                "max_result_bytes": 32_768
                            })
                            .to_string(),
                        })],
                        receipt: json!({"step": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let read_output = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            Item::MaterializedResult { call_id, output }
                                if call_id == "read-spruce-attachments" =>
                            {
                                Some(output.as_str())
                            }
                            _ => None,
                        })
                        .expect("READ_TOOL result delivered");
                    assert!(
                        read_output.contains("CUFF-HM-7741"),
                        "PNG photo unique detail must be readable through READ_TOOL: {read_output}"
                    );
                    assert!(
                        read_output.contains("LOT-K9482"),
                        "JPEG photo unique detail must be readable through READ_TOOL: {read_output}"
                    );
                    assert!(
                        read_output.contains("SYMPTOM-LOG-8829"),
                        "text attachment unique detail must be readable through READ_TOOL: {read_output}"
                    );
                    self.synthesized_answer =
                        "Monitor cuff reference CUFF-HM-7741 (168/104 mmHg), bottle LOT-K9482, and symptom diary SYMPTOM-LOG-8829."
                            .to_owned();
                    Ok(Turn {
                        items: vec![Item::assistant(self.synthesized_answer.clone())],
                        receipt: json!({"step": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected step {other}"),
            }
        }
    }

    let sources = batch.source_revisions();
    let mut provider = ReadBothDetailsProvider {
        step: 0,
        synthesized_answer: String::new(),
    };
    run_attachment_harness(
        "pw-spruce-1",
        sources.clone(),
        sources,
        "Report the blood pressure cuff reference from the photo and the diary code from the note.",
        &mut provider,
    )
    .await;
    assert!(provider.synthesized_answer.contains("CUFF-HM-7741"));
    assert!(provider.synthesized_answer.contains("SYMPTOM-LOG-8829"));

    // Optional live multimodal model check when `CODA_ALLOW_MODEL_CALLS=1` and `OPENAI_API_KEY` are provided.
    if std::env::var("CODA_ALLOW_MODEL_CALLS").as_deref() == Ok("1")
        && let Ok(api_key) = std::env::var("OPENAI_API_KEY")
        && !api_key.trim().is_empty()
    {
        let client = reqwest::Client::new();
        let resp: Value = client
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(api_key.trim())
            .json(&json!({
                "model": "gpt-4.1-mini",
                "input": [{
                    "role": "user",
                    "content": [
                        {"type": "input_text", "text": format!("Read the cuff reference in the attached image and the diary code in this note:\n{txt_body}")},
                        {"type": "input_image", "image_url": png_data_uri}
                    ]
                }]
            }))
            .send()
            .await
            .expect("live multimodal request")
            .json()
            .await
            .expect("live multimodal JSON");
        let resp_str = resp.to_string();
        assert!(resp_str.contains("HM-7741") || resp_str.contains("7741"));
        assert!(resp_str.contains("SYMPTOM-LOG-8829"));
    }
}

#[test]
fn corrupted_attachment_returns_named_failure_while_sibling_attachments_read_cleanly() {
    let valid_png = render_synthetic_clinical_png(&["CLEAN PHOTO", "PULSE 78 BPM"], false);
    let mut corrupted_png = valid_png.clone();
    // Truncate and corrupt PNG header/IDAT bytes.
    corrupted_png.truncate(18);

    let invalid_utf8_bytes = vec![0xFF, 0xFE, 0x00, 0x81, 0x99];
    let valid_md = b"# Clean Sibling Summary\nAll vitals stable.\n".to_vec();

    let batch = ingest_attachment_batch(&[
        SavedAttachmentInput {
            attachment_id: "bad-png".into(),
            revision: "v1".into(),
            workspace: "pw-fault".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Corrupted PNG photo".into(),
            media_kind: AttachmentMediaKind::PngPhoto,
            bytes: corrupted_png,
            expected_sha256: None,
        },
        SavedAttachmentInput {
            attachment_id: "bad-utf8-txt".into(),
            revision: "v1".into(),
            workspace: "pw-fault".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Corrupted text note".into(),
            media_kind: AttachmentMediaKind::PlainText,
            bytes: invalid_utf8_bytes,
            expected_sha256: None,
        },
        SavedAttachmentInput {
            attachment_id: "bad-hash-md".into(),
            revision: "v1".into(),
            workspace: "pw-fault".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Tampered digest note".into(),
            media_kind: AttachmentMediaKind::MarkdownText,
            bytes: valid_md.clone(),
            expected_sha256: Some(
                "0000000000000000000000000000000000000000000000000000000000000000".into(),
            ),
        },
        SavedAttachmentInput {
            attachment_id: "good-sibling-png".into(),
            revision: "v1".into(),
            workspace: "pw-fault".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Readable sibling photo".into(),
            media_kind: AttachmentMediaKind::PngPhoto,
            bytes: valid_png,
            expected_sha256: None,
        },
        SavedAttachmentInput {
            attachment_id: "good-sibling-md".into(),
            revision: "v1".into(),
            workspace: "pw-fault".into(),
            source_id: "spruce".into(),
            authored_date: "2026-11-11".into(),
            title: "Readable sibling markdown".into(),
            media_kind: AttachmentMediaKind::MarkdownText,
            bytes: valid_md,
            expected_sha256: None,
        },
    ]);

    assert!(matches!(
        batch.records[0].outcome,
        Err(AttachmentReadFailure::CorruptedImage { .. })
    ));
    assert!(matches!(
        batch.records[1].outcome,
        Err(AttachmentReadFailure::InvalidUtf8 { .. })
    ));
    assert!(matches!(
        batch.records[2].outcome,
        Err(AttachmentReadFailure::HashMismatch { .. })
    ));
    assert!(batch.records[3].outcome.is_ok());
    assert!(batch.records[4].outcome.is_ok());

    for bad_idx in [0, 1, 2] {
        let limit = inspect_record_readability(&batch.records[bad_idx].source_revision)
            .expect("corrupted attachment surfaces RecordLimit::Unreadable");
        assert_eq!(limit.kind, RecordLimitKind::Unreadable);
    }

    let sources = batch.source_revisions();
    let req = ChartRequest::parse(
        READ_TOOL,
        &json!({
            "references": [
                {"candidate_id": "good-sibling-png", "revision": "v1", "cursor": null},
                {"candidate_id": "good-sibling-md", "revision": "v1", "cursor": null}
            ],
            "max_result_bytes": 16_384
        }),
        ChartScope::Chat,
    )
    .unwrap();
    let ToolOutcome::Returned(out) = req.run("pw-fault", 1, &sources).unwrap() else {
        panic!("sibling attachments must read cleanly");
    };
    assert!(out.contains("PULSE 78 BPM"));
    assert!(out.contains("Clean Sibling Summary"));
}

#[test]
fn document_reader_benchmark_reproducibly_evaluates_candidates_and_writes_artifact() {
    let tool = document_page_tool();
    assert_eq!(tool.name, READ_DOCUMENT_PAGE_TOOL);
    model_text::assert_clean_model_text(&serde_json::to_string(&tool.function()).unwrap());

    let report = run_document_reader_benchmark();
    assert_eq!(report.selected_candidate, "hybrid_local_pdf_and_raster_ocr");
    assert_eq!(report.candidates.len(), 3);

    let selected = report
        .candidates
        .iter()
        .find(|c| c.candidate == "hybrid_local_pdf_and_raster_ocr")
        .unwrap();
    assert_eq!(selected.evaluated_pages, 6);
    assert_eq!(selected.exact_matched_pages, 6);
    assert_eq!(selected.page_accuracy_bps, 10_000);
    assert!(selected.isolated_corrupted_page);
    assert!(selected.zero_external_phi_vendor);
    assert_eq!(selected.marginal_cost_cents_per_1000_pages, 0);

    let native_only = report
        .candidates
        .iter()
        .find(|c| c.candidate == "native_pdf_text_only")
        .unwrap();
    assert_eq!(native_only.exact_matched_pages, 3);
    assert!(native_only.page_accuracy_bps < selected.page_accuracy_bps);

    // Persist reproducible comparison artifact under `target/verification/`.
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let verification_dir = repo_root.join("target/verification");
    std::fs::create_dir_all(&verification_dir).unwrap();
    let artifact_path = verification_dir.join("pdf-scan-reader-benchmark.json");
    std::fs::write(
        &artifact_path,
        serde_json::to_string_pretty(&report).unwrap(),
    )
    .unwrap();
    assert!(artifact_path.exists());
}

#[tokio::test]
async fn chart_scans_spruce_faxes_and_multipage_pdfs_read_page_by_page_through_read_tools() {
    let clinical_pdf = build_test_pdf(&[
        (
            vec![
                "CARDIOLOGY CONSULT P1".into(),
                "REASON EXERTIONAL DYSPNEA".into(),
            ],
            false,
            false,
        ),
        (
            vec!["ECHO MEASUREMENTS P2".into(), "LVEF 61 PERCENT".into()],
            false,
            false,
        ),
        (
            vec!["CARDIOLOGY PLAN P3".into(), "REPEAT BMP IN 14 DAYS".into()],
            false,
            false,
        ),
    ]);
    let spruce_fax = build_test_pdf(&[
        (
            vec!["SPRUCE FAX COVER P1".into(), "FAX-NEPH-4410".into()],
            false,
            false,
        ),
        (
            vec![
                "OUTSIDE BMP P2".into(),
                "POTASSIUM 5.4 MMOL/L".into(),
                "CREATININE 1.48 MG/DL".into(),
            ],
            true,
            false,
        ),
        (
            vec![
                "NEPHROLOGY SIGN-OFF P3".into(),
                "RECHECK IN 72 HOURS".into(),
            ],
            false,
            false,
        ),
    ]);
    let chart_scan = build_test_pdf(&[
        (
            vec!["MERCY DISCHARGE SCAN P1".into(), "WEIGHT 86.2 KG".into()],
            true,
            false,
        ),
        (
            vec![
                "MERCY DISCHARGE SCAN P2".into(),
                "FUROSEMIDE 40MG DAILY".into(),
            ],
            true,
            true,
        ),
        (
            vec!["MERCY DISCHARGE SCAN P3".into(), "PACKET HOSP-9921".into()],
            true,
            false,
        ),
    ]);

    let mut archive = VersionedDocumentArchive::new();
    for (doc_id, source_id, kind, bytes) in [
        (
            "doc-cardiology-pdf",
            "clinic-ehr",
            AttachmentMediaKind::PdfDocument,
            clinical_pdf,
        ),
        (
            "fax-nephrology",
            "spruce",
            AttachmentMediaKind::SpruceFaxPdf,
            spruce_fax,
        ),
        (
            "scan-mercy-packet",
            "clinic-ehr",
            AttachmentMediaKind::ChartScanPdf,
            chart_scan,
        ),
    ] {
        let ingested = archive
            .ingest(&PagedDocumentInput {
                document_id: doc_id.into(),
                version: 1,
                workspace: "pw-paged-1".into(),
                source_id: source_id.into(),
                authored_date: "2026-11-11".into(),
                title: format!("Document {doc_id}"),
                media_kind: kind,
                pdf_bytes: bytes,
                expected_sha256: None,
            })
            .unwrap();
        assert_eq!(ingested.page_count, 3);
    }

    // Also verify a standard `/Subtype /Image /Filter /DCTDecode` JPEG-scan PDF XObject!
    let dct_jpeg =
        render_synthetic_clinical_jpeg(&["DCT JPEG SCAN P1", "TROPONIN 0.01 NG/ML"], false);
    let dct_img = image::load_from_memory_with_format(&dct_jpeg, image::ImageFormat::Jpeg).unwrap();
    let (jw, jh) = (dct_img.width(), dct_img.height());
    let mut dct_pdf = b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /XObject << /Im0 5 0 R >> >> /Contents 4 0 R >>\nendobj\n4 0 obj\n<< /Length 31 >>\nstream\nq\n612 0 0 792 0 0 cm\n/Im0 Do\nQ\n\nendstream\nendobj\n".to_vec();
    dct_pdf.extend_from_slice(
        format!(
            "5 0 obj\n<< /Type /XObject /Subtype /Image /Width {jw} /Height {jh} /ColorSpace /DeviceGray /BitsPerComponent 8 /Filter /DCTDecode /Length {} >>\nstream\n",
            dct_jpeg.len()
        )
        .as_bytes(),
    );
    dct_pdf.extend_from_slice(&dct_jpeg);
    dct_pdf.extend_from_slice(b"\nendstream\nendobj\ntrailer\n<< /Root 1 0 R >>\n%%EOF\n");
    let dct_ingested = ingest_paged_document(&PagedDocumentInput {
        document_id: "scan-dct-jpeg".into(),
        version: 1,
        workspace: "pw-paged-1".into(),
        source_id: "clinic-ehr".into(),
        authored_date: "2026-11-11".into(),
        title: "DCT JPEG Scan".into(),
        media_kind: AttachmentMediaKind::ChartScanPdf,
        pdf_bytes: dct_pdf,
        expected_sha256: None,
    })
    .unwrap();
    assert!(
        dct_ingested
            .read_page(1)
            .unwrap()
            .text
            .contains("TROPONIN 0.01 NG/ML"),
        "DCTDecode JPEG scan XObject must OCR cleanly"
    );

    struct PageByPageProvider {
        step: usize,
    }
    impl Provider for PageByPageProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => {
                    let mut calls = Vec::new();
                    for doc_id in ["doc-cardiology-pdf", "fax-nephrology", "scan-mercy-packet"] {
                        for page in 1..=3 {
                            calls.push(Item::Call(Invocation {
                                id: format!("read-{doc_id}-p{page}"),
                                name: READ_DOCUMENT_PAGE_TOOL.into(),
                                arguments: json!({
                                    "document_id": doc_id,
                                    "version": null,
                                    "page_number": page
                                })
                                .to_string(),
                            }));
                        }
                    }
                    calls.push(Item::Call(Invocation {
                        id: "read-chart-sources".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [
                                {"candidate_id": "doc-cardiology-pdf", "revision": "v1", "cursor": null},
                                {"candidate_id": "fax-nephrology", "revision": "v1", "cursor": null},
                                {"candidate_id": "scan-mercy-packet", "revision": "v1", "cursor": null}
                            ],
                            "max_result_bytes": 65_536
                        })
                        .to_string(),
                    }));
                    Ok(Turn {
                        items: calls,
                        receipt: json!({"step": 1}),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let find_out = |id: &str| -> Value {
                        let raw = request
                            .items
                            .iter()
                            .find_map(|item| match item {
                                Item::MaterializedResult { call_id, output } if call_id == id => {
                                    Some(output.as_str())
                                }
                                _ => None,
                            })
                            .unwrap_or_else(|| panic!("missing tool result for {id}"));
                        serde_json::from_str(raw).unwrap()
                    };

                    let pdf_p2 = find_out("read-doc-cardiology-pdf-p2");
                    assert_eq!(pdf_p2["outcome"], "available");
                    assert_eq!(pdf_p2["document_id"], "doc-cardiology-pdf");
                    assert_eq!(pdf_p2["version"], 1);
                    assert_eq!(pdf_p2["page_number"], 2);
                    assert!(pdf_p2["text"].as_str().unwrap().contains("LVEF 61 PERCENT"));

                    let fax_p2 = find_out("read-fax-nephrology-p2");
                    assert_eq!(fax_p2["outcome"], "available");
                    assert_eq!(fax_p2["page_number"], 2);
                    assert!(
                        fax_p2["text"]
                            .as_str()
                            .unwrap()
                            .contains("POTASSIUM 5.4 MMOL/L")
                    );

                    let scan_p2 = find_out("read-scan-mercy-packet-p2");
                    assert_eq!(scan_p2["outcome"], "available");
                    assert_eq!(scan_p2["page_number"], 2);
                    assert!(
                        scan_p2["text"]
                            .as_str()
                            .unwrap()
                            .contains("FUROSEMIDE 40MG DAILY")
                    );

                    let chart_read = find_out("read-chart-sources").to_string();
                    assert!(chart_read.contains("page 1 of 3"));
                    assert!(chart_read.contains("page 2 of 3"));
                    assert!(chart_read.contains("page 3 of 3"));
                    assert!(chart_read.contains("PACKET HOSP-9921"));

                    Ok(Turn {
                        items: vec![Item::assistant("All nine pages verified.")],
                        receipt: json!({"step": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected step {other}"),
            }
        }
    }

    run_attachment_harness(
        "pw-paged-1",
        archive.current_source_revisions(),
        archive.all_version_revisions(),
        "Read all pages of the cardiology PDF, nephrology fax, and hospital discharge scan.",
        &mut PageByPageProvider { step: 0 },
    )
    .await;
}

#[test]
fn corrupted_page_in_multipage_fax_returns_named_failure_while_other_pages_remain_readable() {
    let broken_fax = build_corrupted_page_test_pdf(
        &[
            vec!["FAX PAGE 1 COVER INTACT".into()],
            vec!["FAX PAGE 2 DAMAGED".into()],
            vec!["FAX PAGE 3 ADDENDUM INTACT".into()],
        ],
        2,
    );

    let ingested = ingest_paged_document(&PagedDocumentInput {
        document_id: "fax-partial-corruption".into(),
        version: 1,
        workspace: "pw-corrupt-fax".into(),
        source_id: "spruce".into(),
        authored_date: "2026-11-11".into(),
        title: "Partially corrupted 3-page Spruce fax".into(),
        media_kind: AttachmentMediaKind::SpruceFaxPdf,
        pdf_bytes: broken_fax,
        expected_sha256: None,
    })
    .expect("document container remains valid despite page 2 stream corruption");

    assert_eq!(ingested.page_count, 3);
    assert!(
        ingested
            .read_page(1)
            .unwrap()
            .text
            .contains("FAX PAGE 1 COVER INTACT")
    );
    assert!(matches!(
        ingested.read_page(2),
        Err(AttachmentReadFailure::CorruptedPage {
            page_number: 2,
            version: 1,
            ..
        })
    ));
    assert!(
        ingested
            .read_page(3)
            .unwrap()
            .text
            .contains("FAX PAGE 3 ADDENDUM INTACT")
    );

    let retained = vec![ingested.source_revision];
    let ToolOutcome::Returned(p1_raw) = execute_document_page_read_from_revisions(
        &json!({"document_id": "fax-partial-corruption", "version": 1, "page_number": 1}),
        &retained,
    ) else {
        panic!("expected Returned for page 1");
    };
    let p1: Value = serde_json::from_str(&p1_raw).unwrap();
    assert_eq!(p1["outcome"], "available");
    assert!(
        p1["text"]
            .as_str()
            .unwrap()
            .contains("FAX PAGE 1 COVER INTACT")
    );

    let ToolOutcome::Returned(p2_raw) = execute_document_page_read_from_revisions(
        &json!({"document_id": "fax-partial-corruption", "version": 1, "page_number": 2}),
        &retained,
    ) else {
        panic!("expected structured failed_page outcome for page 2");
    };
    let p2: Value = serde_json::from_str(&p2_raw).unwrap();
    assert_eq!(p2["outcome"], "failed_page");
    assert_eq!(p2["page_number"], 2);
    assert_eq!(p2["failure_code"], "corrupted_page");

    let ToolOutcome::Returned(p3_raw) = execute_document_page_read_from_revisions(
        &json!({"document_id": "fax-partial-corruption", "version": 1, "page_number": 3}),
        &retained,
    ) else {
        panic!("expected Returned for page 3");
    };
    let p3: Value = serde_json::from_str(&p3_raw).unwrap();
    assert_eq!(p3["outcome"], "available");
    assert!(
        p3["text"]
            .as_str()
            .unwrap()
            .contains("FAX PAGE 3 ADDENDUM INTACT")
    );
}

#[test]
fn corrected_document_creates_new_version_and_reopens_exact_original_page_in_archive() {
    let v1_pdf = build_test_pdf(&[
        (vec!["ECHO COVER P1".into()], false, false),
        (
            vec!["ECHO LVEF 56 PERCENT (PRELIMINARY)".into()],
            false,
            false,
        ),
    ]);
    let v2_pdf = build_test_pdf(&[
        (vec!["ECHO COVER P1".into()], false, false),
        (
            vec!["ECHO LVEF 61 PERCENT (CORRECTED FINAL)".into()],
            false,
            false,
        ),
    ]);

    let mut archive = VersionedDocumentArchive::new();
    archive
        .ingest(&PagedDocumentInput {
            document_id: "doc-echo".into(),
            version: 1,
            workspace: "pw-ver-1".into(),
            source_id: "clinic-ehr".into(),
            authored_date: "2026-11-10".into(),
            title: "Preliminary Echo".into(),
            media_kind: AttachmentMediaKind::PdfDocument,
            pdf_bytes: v1_pdf,
            expected_sha256: None,
        })
        .unwrap();
    archive
        .ingest(&PagedDocumentInput {
            document_id: "doc-echo".into(),
            version: 2,
            workspace: "pw-ver-1".into(),
            source_id: "clinic-ehr".into(),
            authored_date: "2026-11-11".into(),
            title: "Corrected Final Echo".into(),
            media_kind: AttachmentMediaKind::PdfDocument,
            pdf_bytes: v2_pdf,
            expected_sha256: None,
        })
        .unwrap();

    assert_eq!(archive.latest_version_of("doc-echo"), Some(2));
    let latest_p2 = archive.read_page("doc-echo", None, 2).unwrap();
    assert_eq!(latest_p2.version, 2);
    assert!(latest_p2.text.contains("61 PERCENT (CORRECTED FINAL)"));

    let cited_v1_p2 = archive.read_page("doc-echo", Some(1), 2).unwrap();
    assert_eq!(cited_v1_p2.version, 1);
    assert_eq!(cited_v1_p2.page_number, 2);
    assert!(cited_v1_p2.text.contains("56 PERCENT (PRELIMINARY)"));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn corrected_document_in_postgres_creates_new_version_while_existing_citations_reopen_exact_v1_page()
 {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();

    let practice = "practice-docver";
    let workspace = format!("pw-docver-{}", uuid::Uuid::new_v4().simple());
    let v1_pdf = build_test_pdf(&[
        (vec!["ECHO REPORT V1 PAGE 1".into()], false, false),
        (vec!["LVEF 56 PERCENT PRELIMINARY".into()], false, false),
    ]);
    let v2_pdf = build_test_pdf(&[
        (vec!["ECHO REPORT V2 PAGE 1".into()], false, false),
        (vec!["LVEF 61 PERCENT FINAL ADDENDUM".into()], false, false),
    ]);

    let doc_v1 = ingest_paged_document(&PagedDocumentInput {
        document_id: "doc-echo-pg".into(),
        version: 1,
        workspace: workspace.clone(),
        source_id: "clinic-ehr".into(),
        authored_date: "2026-11-10".into(),
        title: "Preliminary Echo".into(),
        media_kind: AttachmentMediaKind::PdfDocument,
        pdf_bytes: v1_pdf,
        expected_sha256: None,
    })
    .unwrap();
    store.record_source(&doc_v1.source_revision).await.unwrap();

    let doc_v2 = ingest_paged_document(&PagedDocumentInput {
        document_id: "doc-echo-pg".into(),
        version: 2,
        workspace: workspace.clone(),
        source_id: "clinic-ehr".into(),
        authored_date: "2026-11-11".into(),
        title: "Corrected Echo".into(),
        media_kind: AttachmentMediaKind::PdfDocument,
        pdf_bytes: v2_pdf,
        expected_sha256: None,
    })
    .unwrap();
    store.record_source(&doc_v2.source_revision).await.unwrap();

    // Latest version read (`version: null`) returns `v2`.
    let ToolOutcome::Returned(latest_raw) = store
        .execute_document_page_tool(
            practice,
            &workspace,
            &json!({
                "document_id": "doc-echo-pg",
                "version": null,
                "page_number": 2
            }),
        )
        .await
        .unwrap()
    else {
        panic!("expected Returned for latest version");
    };
    let latest: Value = serde_json::from_str(&latest_raw).unwrap();
    assert_eq!(latest["outcome"], "available");
    assert_eq!(latest["version"], 2);
    assert!(
        latest["text"]
            .as_str()
            .unwrap()
            .contains("LVEF 61 PERCENT FINAL ADDENDUM")
    );

    // Existing citation to `(document_id: "doc-echo-pg", version: 1, page_number: 2)` reopens the exact retained `v1` page!
    let ToolOutcome::Returned(v1_raw) = store
        .execute_document_page_tool(
            practice,
            &workspace,
            &json!({
                "document_id": "doc-echo-pg",
                "version": 1,
                "page_number": 2
            }),
        )
        .await
        .unwrap()
    else {
        panic!("expected Returned for retained v1 citation");
    };
    let reopened_v1: Value = serde_json::from_str(&v1_raw).unwrap();
    assert_eq!(reopened_v1["outcome"], "available");
    assert_eq!(reopened_v1["version"], 1);
    assert_eq!(reopened_v1["page_number"], 2);
    assert!(
        reopened_v1["text"]
            .as_str()
            .unwrap()
            .contains("LVEF 56 PERCENT PRELIMINARY")
    );

    // End-to-end production host verification: BOTH `PracticeJobHost` (`PatientReview`) and
    // `PhysicianHost` (`physician_chat::recipe`) advertise and execute `READ_DOCUMENT_PAGE_TOOL`!
    struct PostgresDocAuthority;
    #[async_trait]
    impl Authority for PostgresDocAuthority {
        async fn check(
            &mut self,
            tx: &PublicationTransaction,
            scope: &Scope,
            session_hash: Option<&[u8]>,
        ) -> Result<(), String> {
            PostgresAuthStore::check_runtime_authority(
                tx,
                session_hash.ok_or("missing session reference")?,
                &PhysicianIdentity {
                    practice_id: scope.practice.clone(),
                    physician_id: scope.physician.clone(),
                },
                &scope.workspace,
            )
            .await
            .map_err(|cause| cause.to_string())
        }

        async fn lock_publication(
            &mut self,
            tx: &PublicationTransaction,
            scope: &Scope,
            session_hash: Option<&[u8]>,
        ) -> Result<(), String> {
            PostgresAuthStore::lock_runtime_authority(
                tx,
                session_hash.ok_or("missing session reference")?,
                &PhysicianIdentity {
                    practice_id: scope.practice.clone(),
                    physician_id: scope.physician.clone(),
                },
                &scope.workspace,
            )
            .await
            .map_err(|cause| cause.to_string())
        }
    }

    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let host_practice = format!("practice-doc-host-{}", uuid::Uuid::new_v4().simple());
    let host_workspace = format!("pw-doc-host-{}", uuid::Uuid::new_v4().simple());
    auth.ensure_practice(&host_practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let email = format!("dr-echo-{}@codahealth.test", uuid::Uuid::new_v4().simple());
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let invite = auth
        .invite(&host_practice, &email, "dr-echo", false, now_secs)
        .await
        .unwrap();
    let (physician, token) = auth
        .redeem_invitation(&email, &invite, now_secs)
        .await
        .unwrap();
    let session_hash = PostgresAuthStore::session_reference(&token);
    auth.register_patient(&host_practice, &host_workspace)
        .await
        .unwrap();
    auth.set_membership(&physician, &host_workspace, Membership::Allowed)
        .await
        .unwrap();

    let skills =
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .unwrap();
    let roles = ModelRoles {
        chat: ModelBinding {
            model: "chat-lane-doc".into(),
            effort: "medium".into(),
            context_window_tokens: 192_000,
            max_input_tokens: 160_000,
            adapter: "fixture".into(),
        },
        background: ModelBinding {
            model: "bg-lane-doc".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        },
    };

    let mut host_doc_v1 = doc_v1.source_revision.clone();
    host_doc_v1.workspace = host_workspace.clone();
    let mut host_doc_v2 = doc_v2.source_revision.clone();
    host_doc_v2.workspace = host_workspace.clone();

    store.record_source(&host_doc_v1).await.unwrap();
    store
        .register_patient_dispatch(&auth, &host_practice, &host_workspace, 1_790_000_000)
        .await
        .unwrap();
    store
        .record_chart(
            &host_workspace,
            std::slice::from_ref(&host_doc_v2),
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: SourcePatient {
                    source_patient_id: "pt-echo".into(),
                    display_name: "Elena Rostova".into(),
                    birth_date: Some("1974-02-19".into()),
                    sex_or_gender: Some("female".into()),
                    medical_record_numbers: vec![MedicalRecordNumber {
                        system: "clinic-mrn".into(),
                        value: "MRN-5510".into(),
                    }],
                },
                pulled_at: Some(1_790_003_600),
            },
        )
        .await
        .unwrap();

    // 1. Run `PracticeJobHost` (`PatientReview`) calling `READ_DOCUMENT_PAGE_TOOL` for `version: null` (`v2`) and `version: 1` (`v1`).
    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, &auth);
    let wake = store
        .wake_practice_dispatch(&auth, &host_practice, 1_790_003_600, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake.started_reviews.len(), 1);
    let reviews = store
        .patient_reviews(&host_practice, &host_workspace, &auth)
        .await
        .unwrap();
    let review_claim = store
        .claim_practice_job(&reviews[0].practice_scope(), 60, &auth)
        .await
        .unwrap()
        .unwrap();

    struct ReviewDocPageProvider {
        turn: usize,
    }
    impl Provider for ReviewDocPageProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            let tool_names: Vec<&str> = request
                .tools
                .iter()
                .filter_map(|t| t["name"].as_str())
                .collect();
            assert!(
                tool_names.contains(&READ_DOCUMENT_PAGE_TOOL),
                "PatientReview recipe must advertise READ_DOCUMENT_PAGE_TOOL: {tool_names:?}"
            );
            match self.turn {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "rev-read-echo-source".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "doc-echo-pg",
                                    "revision": "v2",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "rev-doc-p2-latest".into(),
                            name: READ_DOCUMENT_PAGE_TOOL.into(),
                            arguments: json!({
                                "document_id": "doc-echo-pg",
                                "version": null,
                                "page_number": 2
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "rev-doc-p2-v1".into(),
                            name: READ_DOCUMENT_PAGE_TOOL.into(),
                            arguments: json!({
                                "document_id": "doc-echo-pg",
                                "version": 1,
                                "page_number": 2
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let out_for = |cid: &str| -> Value {
                        let raw = request
                            .items
                            .iter()
                            .find_map(|item| match item {
                                Item::MaterializedResult { call_id, output } if call_id == cid => {
                                    Some(output.as_str())
                                }
                                _ => None,
                            })
                            .unwrap();
                        serde_json::from_str(raw).unwrap()
                    };
                    let latest_val = out_for("rev-doc-p2-latest");
                    assert_eq!(latest_val["version"], 2);
                    assert!(
                        latest_val["text"]
                            .as_str()
                            .unwrap()
                            .contains("LVEF 61 PERCENT FINAL ADDENDUM")
                    );
                    let v1_val = out_for("rev-doc-p2-v1");
                    assert_eq!(v1_val["version"], 1);
                    assert!(
                        v1_val["text"]
                            .as_str()
                            .unwrap()
                            .contains("LVEF 56 PERCENT PRELIMINARY")
                    );

                    let complete = CompleteReviewArgs {
                        disposition: CompleteReviewDisposition::Assessed {
                            summary: "Corrected echo confirms LVEF 61%.".into(),
                        },
                        citations: vec![ExactRecordReference {
                            candidate_id: "doc-echo-pg".into(),
                            revision: "v2".into(),
                        }],
                        basis_and_limitations: "Clinic EHR corrected echocardiogram v2.".into(),
                        unfinished_work: vec![],
                    };
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "rev-complete-echo".into(),
                            name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
                            arguments: serde_json::to_string(&complete).unwrap(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => Ok(Turn {
                    items: vec![Item::assistant("Completed echo review.")],
                    receipt: json!({"turn": 3}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected review turn {other}"),
            }
        }
    }

    let mut review_host = PracticeJobHost {
        store: store.clone(),
        claim: review_claim,
        authority: &auth,
    };
    review_host
        .run(
            &mut ReviewDocPageProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();

    // 2. Run `PhysicianHost` (`physician_chat::recipe`) calling `READ_DOCUMENT_PAGE_TOOL` for `version: null` (`v2`) and `version: 1` (`v1`).
    let chat_scope = Scope {
        practice: host_practice.clone(),
        physician: physician.physician_id.clone(),
        workspace: host_workspace.clone(),
        chat: "chat-doc-page-1".into(),
    };
    let starting = store.patient_starting_context(&chat_scope).await.unwrap();
    let recipe = ChatAssignment::for_patient(chat_scope.clone(), starting).recipe(&roles, &skills);
    assert_eq!(
        store
            .start_chat_authorized(
                &chat_scope,
                &Input {
                    key: "cmd-chat-doc-1".into(),
                    text: "Compare page 2 of the preliminary and corrected echo reports.".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session_hash,
                &skills,
                &recipe,
                &mut PostgresDocAuthority,
            )
            .await
            .unwrap(),
        Accepted::New
    );

    struct ChatDocPageProvider {
        turn: usize,
    }
    impl Provider for ChatDocPageProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            let tool_names: Vec<&str> = request
                .tools
                .iter()
                .filter_map(|t| t["name"].as_str())
                .collect();
            assert!(
                tool_names.contains(&READ_DOCUMENT_PAGE_TOOL),
                "physician_chat recipe must advertise READ_DOCUMENT_PAGE_TOOL: {tool_names:?}"
            );
            match self.turn {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "chat-read-echo-source".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "doc-echo-pg",
                                    "revision": "v2",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "chat-doc-p2-latest".into(),
                            name: READ_DOCUMENT_PAGE_TOOL.into(),
                            arguments: json!({
                                "document_id": "doc-echo-pg",
                                "version": null,
                                "page_number": 2
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "chat-doc-p2-v1".into(),
                            name: READ_DOCUMENT_PAGE_TOOL.into(),
                            arguments: json!({
                                "document_id": "doc-echo-pg",
                                "version": 1,
                                "page_number": 2
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let out_for = |cid: &str| -> Value {
                        let raw = request
                            .items
                            .iter()
                            .find_map(|item| match item {
                                Item::MaterializedResult { call_id, output } if call_id == cid => {
                                    Some(output.as_str())
                                }
                                _ => None,
                            })
                            .unwrap();
                        serde_json::from_str(raw).unwrap()
                    };
                    let latest_val = out_for("chat-doc-p2-latest");
                    assert_eq!(latest_val["version"], 2);
                    assert!(
                        latest_val["text"]
                            .as_str()
                            .unwrap()
                            .contains("LVEF 61 PERCENT FINAL ADDENDUM")
                    );
                    let v1_val = out_for("chat-doc-p2-v1");
                    assert_eq!(v1_val["version"], 1);
                    assert!(
                        v1_val["text"]
                            .as_str()
                            .unwrap()
                            .contains("LVEF 56 PERCENT PRELIMINARY")
                    );

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Page 2 changed from preliminary LVEF 56% to final LVEF 61% [1].\n\nBasis and limitations: Clinic EHR echocardiogram reports v1 and v2.",
                        )],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected chat turn {other}"),
            }
        }
    }

    let chat_claim = store.claim(&chat_scope, 60).await.unwrap().unwrap();
    let mut chat_host = PhysicianHost {
        store: store.clone(),
        claim: chat_claim,
        authority: PostgresDocAuthority,
        shown_steps: local_product::runtime::ShownSteps::new(()),
    };
    chat_host
        .run(
            &mut ChatDocPageProvider { turn: 0 },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(
        store.conversation(&chat_scope).await.unwrap().status,
        WorkStatus::Completed
    );
}

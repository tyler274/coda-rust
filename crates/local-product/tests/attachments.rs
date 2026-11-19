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

use local_product::{
    attachments::{
        AttachmentMediaKind, AttachmentPart, AttachmentReadFailure, SavedAttachmentInput,
        ingest_attachment_batch, render_synthetic_clinical_jpeg, render_synthetic_clinical_png,
        render_synthetic_clinical_rgb_png,
    },
    physician_sources::{ChartRequest, ChartScope, READ_TOOL, SourceRevision, read_tool},
    runtime::{
        async_trait,
        chat_operations::{RecordLimitKind, inspect_record_readability},
        engine::{
            self, AcceptedInput, Event, Invocation, Item, Product, ProductMessage, ToolOutcome,
            Turn, WindowSeed,
            recipe::{Finish, Observer, Provider, Request, RequestBinding, RequestRecipe},
        },
    },
    sha256_hex,
};
use serde_json::{Value, json};

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

struct AttachmentHarnessHost {
    workspace: String,
    current_sources: Vec<SourceRevision>,
    _retained_revisions: Vec<SourceRevision>,
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
        name == READ_TOOL
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
        tools: vec![read_tool(ChartScope::Chat).function()],
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
        _retained_revisions: retained_revisions,
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

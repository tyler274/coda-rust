use patient_bank::attachments::{
    PdfPageEncoding, PdfPageSpec, ScanQuality, SyntheticFixtureKind, corrupt_single_pdf_page,
    render_clinical_jpeg, render_clinical_png, render_multipage_pdf, synthetic_attachment_corpus,
};

#[test]
fn corpus_builds_deterministic_spruce_attachments_faxes_pdfs_and_scans() {
    let first = synthetic_attachment_corpus();
    let second = synthetic_attachment_corpus();
    assert_eq!(
        first, second,
        "synthetic attachment corpus must be deterministic"
    );
    assert_eq!(first.len(), 9);

    for fixture in &first {
        assert!(!fixture.bytes.is_empty());
        assert_eq!(fixture.sha256.len(), 64);
        let doc_ref = fixture.to_fhir_document_reference();
        assert_eq!(doc_ref["resourceType"], "DocumentReference");
        assert_eq!(doc_ref["content"][0]["attachment"]["hash"], fixture.sha256);
    }

    let png = first
        .iter()
        .find(|f| f.kind == SyntheticFixtureKind::SprucePhotoPng)
        .expect("PNG fixture");
    assert!(png.bytes.starts_with(b"\x89PNG\r\n\x1a\n"));

    let jpeg = first
        .iter()
        .find(|f| f.kind == SyntheticFixtureKind::SprucePhotoJpeg)
        .expect("JPEG fixture");
    assert!(jpeg.bytes.starts_with(&[0xFF, 0xD8]));
    assert!(jpeg.bytes.ends_with(&[0xFF, 0xD9]));

    let scan = first
        .iter()
        .find(|f| f.kind == SyntheticFixtureKind::ChartScanPdf)
        .expect("chart scan fixture");
    assert!(scan.bytes.starts_with(b"%PDF-1.4\n"));
    assert_eq!(scan.pages.len(), 3);
}

#[test]
fn single_page_corruption_helper_preserves_pdf_header_and_sibling_pages() {
    let pages = vec![
        PdfPageSpec {
            lines: vec!["PAGE ONE INTACT".into()],
            encoding: PdfPageEncoding::NativeText,
        },
        PdfPageSpec {
            lines: vec!["PAGE TWO TARGET".into()],
            encoding: PdfPageEncoding::NativeText,
        },
        PdfPageSpec {
            lines: vec!["PAGE THREE INTACT".into()],
            encoding: PdfPageEncoding::NativeText,
        },
    ];
    let clean = render_multipage_pdf(&pages);
    let damaged = corrupt_single_pdf_page(&pages, 2);
    assert_ne!(clean, damaged);
    assert!(damaged.starts_with(b"%PDF-1.4\n"));
    let damaged_lossy = String::from_utf8_lossy(&damaged);
    assert!(damaged_lossy.contains("PAGE ONE INTACT"));
    assert!(damaged_lossy.contains("PAGE THREE INTACT"));
    assert!(!damaged_lossy.contains("PAGE TWO TARGET"));

    let noisy_png = render_clinical_png(&["SCAN LINE 1"], ScanQuality::NoisyScan);
    let noisy_jpeg = render_clinical_jpeg(&["SCAN LINE 1"], ScanQuality::NoisyScan);
    assert!(noisy_png.starts_with(b"\x89PNG\r\n\x1a\n"));
    assert!(noisy_jpeg.starts_with(&[0xFF, 0xD8]));
}

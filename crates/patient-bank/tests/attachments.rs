use std::io::Read as _;

use flate2::read::ZlibDecoder;
use patient_bank::attachments::{
    PdfPageEncoding, PdfPageSpec, PngScanlineFilter, ScanQuality, SyntheticFixtureKind,
    corrupt_single_pdf_page, encode_grayscale_png_filtered, render_clinical_grayscale_pixels,
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
        assert!(
            !fixture
                .bytes
                .windows(b"CODA_RASTER_PNG".len())
                .any(|w| w == b"CODA_RASTER_PNG"),
            "fixture `{}` must not carry custom JPEG raster markers",
            fixture.document_id
        );
        assert!(
            !fixture
                .bytes
                .windows(b"ASCIIBase64PngScan".len())
                .any(|w| w == b"ASCIIBase64PngScan"),
            "fixture `{}` must not carry custom PDF inline filters",
            fixture.document_id
        );
        let doc_ref = fixture.to_fhir_document_reference();
        assert_eq!(doc_ref["resourceType"], "DocumentReference");
        assert_eq!(doc_ref["content"][0]["attachment"]["hash"], fixture.sha256);
    }

    let png = first
        .iter()
        .find(|f| f.kind == SyntheticFixtureKind::SprucePhotoPng)
        .expect("PNG fixture");
    assert!(png.bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let decoded_png =
        image::load_from_memory_with_format(&png.bytes, image::ImageFormat::Png).expect("real PNG");
    assert!(decoded_png.width() > 0 && decoded_png.height() > 0);

    let jpeg = first
        .iter()
        .find(|f| f.kind == SyntheticFixtureKind::SprucePhotoJpeg)
        .expect("JPEG fixture");
    assert!(jpeg.bytes.starts_with(&[0xFF, 0xD8]));
    assert!(jpeg.bytes.ends_with(&[0xFF, 0xD9]));
    let decoded_jpeg = image::load_from_memory_with_format(&jpeg.bytes, image::ImageFormat::Jpeg)
        .expect("real baseline DCT/Huffman JPEG");
    assert!(decoded_jpeg.width() > 0 && decoded_jpeg.height() > 0);

    let scan = first
        .iter()
        .find(|f| f.kind == SyntheticFixtureKind::ChartScanPdf)
        .expect("chart scan fixture");
    assert!(scan.bytes.starts_with(b"%PDF-1.4\n"));
    let scan_header = String::from_utf8_lossy(&scan.bytes);
    assert!(scan_header.contains("/Subtype /Image"));
    assert!(scan_header.contains("/Filter /FlateDecode"));
    assert!(scan_header.contains("/Filter /DCTDecode"));
    assert_eq!(scan.pages.len(), 3);
}

#[test]
fn single_page_corruption_helper_preserves_flate_sibling_pages_and_png_filters_roundtrip() {
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
    assert!(damaged_lossy.contains("/Filter /FlateDecode"));

    // Extract all stream bodies in `damaged` and verify intact pages inflate while page 2 fails.
    let mut inflated_texts = Vec::new();
    let mut failed_streams = 0_usize;
    let mut cursor = 0_usize;
    while let Some(rel) = damaged[cursor..]
        .windows(b"stream\n".len())
        .position(|w| w == b"stream\n")
    {
        let start = cursor + rel + b"stream\n".len();
        let end_rel = damaged[start..]
            .windows(b"\nendstream".len())
            .position(|w| w == b"\nendstream")
            .expect("endstream");
        let compressed = &damaged[start..start + end_rel];
        let mut out = String::new();
        if ZlibDecoder::new(compressed)
            .read_to_string(&mut out)
            .is_ok()
        {
            inflated_texts.push(out);
        } else {
            failed_streams += 1;
        }
        cursor = start + end_rel + b"\nendstream".len();
    }
    assert_eq!(failed_streams, 1);
    assert!(inflated_texts.iter().any(|t| t.contains("PAGE ONE INTACT")));
    assert!(
        inflated_texts
            .iter()
            .any(|t| t.contains("PAGE THREE INTACT"))
    );
    assert!(!inflated_texts.iter().any(|t| t.contains("PAGE TWO TARGET")));

    let (w, h, raw_pixels) =
        render_clinical_grayscale_pixels(&["SCAN LINE 1"], ScanQuality::NoisyScan);
    for filter in [
        PngScanlineFilter::None,
        PngScanlineFilter::Sub,
        PngScanlineFilter::Up,
        PngScanlineFilter::Avg,
        PngScanlineFilter::Paeth,
    ] {
        let filtered_png = encode_grayscale_png_filtered(w, h, &raw_pixels, filter);
        let decoded = image::load_from_memory_with_format(&filtered_png, image::ImageFormat::Png)
            .expect("filtered PNG decodes")
            .to_luma8();
        assert_eq!(decoded.into_raw(), raw_pixels);
    }

    let noisy_png = render_clinical_png(&["SCAN LINE 1"], ScanQuality::NoisyScan);
    let noisy_jpeg = render_clinical_jpeg(&["SCAN LINE 1"], ScanQuality::NoisyScan);
    assert!(noisy_png.starts_with(b"\x89PNG\r\n\x1a\n"));
    assert!(noisy_jpeg.starts_with(&[0xFF, 0xD8]));
}

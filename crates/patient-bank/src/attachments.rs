//! Synthetic primary-care attachments, multi-page PDFs, Spruce faxes, and chart
//! scans for deterministic qualification of multimodal attachment and paged
//! document reading without real patient data.

use base64::Engine as _;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// Category of synthetic patient-source file fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntheticFixtureKind {
    /// Patient-captured PNG clinical photo (such as a home monitor or medication bottle).
    SprucePhotoPng,
    /// Patient-captured JPEG clinical photo.
    SprucePhotoJpeg,
    /// Plain-text patient symptom or home measurement log.
    SpruceTextNote,
    /// Markdown outside-care summary attachment.
    SpruceMarkdownSummary,
    /// Comma-separated home blood-pressure or glucose log.
    SpruceCsvLog,
    /// Multi-page digital clinical PDF report with native page text streams.
    MultiPageClinicalPdf,
    /// Multi-page inbound Spruce fax PDF.
    SpruceFaxPdf,
    /// Multi-page raster-scanned outside hospital packet without native PDF text operators.
    ChartScanPdf,
}

impl SyntheticFixtureKind {
    /// Standard MIME content type for the fixture bytes.
    pub const fn media_type(self) -> &'static str {
        match self {
            Self::SprucePhotoPng => "image/png",
            Self::SprucePhotoJpeg => "image/jpeg",
            Self::SpruceTextNote => "text/plain",
            Self::SpruceMarkdownSummary => "text/markdown",
            Self::SpruceCsvLog => "text/csv",
            Self::MultiPageClinicalPdf | Self::SpruceFaxPdf | Self::ChartScanPdf => {
                "application/pdf"
            }
        }
    }
}

/// Visual quality profile when rendering synthetic scan or photo bitmaps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanQuality {
    /// Crisp high-contrast clinical label or digital raster capture.
    Clean,
    /// Low-contrast scanner paper grain with deterministic salt-free speckle.
    NoisyScan,
}

/// Expected ground-truth page content for one 1-based synthetic document page.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntheticPageTruth {
    /// One-based page index (`1..=page_count`).
    pub page_number: u32,
    /// Lines rendered onto this page.
    pub lines: Vec<String>,
}

/// One deterministic synthetic attachment or multi-page document fixture.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntheticAttachmentFixture {
    /// Stable document or attachment identifier.
    pub document_id: String,
    /// Monotonic document version (`1` for initial, `2` for corrected addendum, etc.).
    pub version: u64,
    /// Synthetic patient identifier owning the fixture.
    pub patient_id: String,
    /// Source system identifier (`spruce` or `clinic-ehr`).
    pub source_id: String,
    /// Fixture format and clinical role.
    pub kind: SyntheticFixtureKind,
    /// Effective clinical date (`YYYY-MM-DD`).
    pub authored_date: String,
    /// Human-readable document title.
    pub title: String,
    /// Exact original file bytes.
    pub bytes: Vec<u8>,
    /// Lowercase hex SHA-256 of `bytes`.
    pub sha256: String,
    /// Ground-truth page lines (1 entry for single-part attachments; `N` entries for paged PDFs).
    pub pages: Vec<SyntheticPageTruth>,
}

impl SyntheticAttachmentFixture {
    /// Encodes this fixture as a FHIR R4 `DocumentReference` JSON object carrying
    /// the original base64 payload and SHA-256 digest intact.
    pub fn to_fhir_document_reference(&self) -> Value {
        let encoded = base64::engine::general_purpose::STANDARD.encode(&self.bytes);
        json!({
            "resourceType": "DocumentReference",
            "id": format!("{}-v{}", self.document_id, self.version),
            "masterIdentifier": {
                "system": "urn:coda:synthetic-document",
                "value": self.document_id,
            },
            "version": self.version.to_string(),
            "status": "current",
            "date": format!("{}T09:00:00Z", self.authored_date),
            "description": self.title,
            "subject": {
                "reference": format!("Patient/{}", self.patient_id),
            },
            "content": [{
                "attachment": {
                    "contentType": self.kind.media_type(),
                    "title": self.title,
                    "data": encoded,
                    "hash": self.sha256,
                    "size": self.bytes.len(),
                }
            }]
        })
    }
}

/// Canonical 5x7 uppercase/digit/clinical-punctuation bitmap glyph rows (`7` rows of 5-bit masks).
/// Shared deterministically between synthetic raster rendering and local scan OCR verification.
pub fn glyph_bitmap_5x7(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111,
        ],
        'D' => [
            0b11100, 0b10010, 0b10001, 0b10001, 0b10001, 0b10010, 0b11100,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10011, 0b10101, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100,
        ],
        ':' => [
            0b00000, 0b00100, 0b00100, 0b00000, 0b00100, 0b00100, 0b00000,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '/' => [
            0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100,
        ],
        ',' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100, 0b01000,
        ],
        '%' => [
            0b11001, 0b11010, 0b00010, 0b00100, 0b01000, 0b01011, 0b10011,
        ],
        '(' => [
            0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010,
        ],
        ')' => [
            0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000,
        ],
        '+' => [
            0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000,
        ],
        '#' => [
            0b01010, 0b11111, 0b01010, 0b01010, 0b11111, 0b01010, 0b00000,
        ],
        _ => [0b00000; 7],
    }
}

/// Complete alphabet supported by the 5x7 clinical label rasterizer.
pub const RASTER_ALPHABET: &[char] = &[
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':',
    '-', '/', '.', ',', '%', '(', ')', '+', '#',
];

/// Renders `lines` of clinical text into a valid RFC-2083 8-bit grayscale PNG image.
pub fn render_clinical_png(lines: &[&str], quality: ScanQuality) -> Vec<u8> {
    const MARGIN_X: usize = 6;
    const MARGIN_Y: usize = 6;
    const CELL_W: usize = 6; // 5px glyph + 1px spacing
    const CELL_H: usize = 10; // 7px glyph + 3px line spacing

    let max_chars = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1)
        .max(1);
    let width = MARGIN_X * 2 + max_chars * CELL_W;
    let height = MARGIN_Y * 2 + lines.len().max(1) * CELL_H;

    let mut pixels = vec![248_u8; width * height];
    if quality == ScanQuality::NoisyScan {
        for (idx, px) in pixels.iter_mut().enumerate() {
            let grain = ((idx.wrapping_mul(73) ^ (idx >> 3).wrapping_mul(19)) % 24) as u8;
            *px = 228_u8.saturating_add(grain);
        }
    }

    for (line_idx, line) in lines.iter().enumerate() {
        let base_y = MARGIN_Y + line_idx * CELL_H;
        for (char_idx, ch) in line.chars().enumerate() {
            let base_x = MARGIN_X + char_idx * CELL_W;
            let glyph = glyph_bitmap_5x7(ch);
            for (row_idx, mask) in glyph.iter().enumerate() {
                for col_idx in 0..5 {
                    let bit = (mask >> (4 - col_idx)) & 1;
                    if bit == 1 {
                        let y = base_y + row_idx;
                        let x = base_x + col_idx;
                        let ink = match quality {
                            ScanQuality::Clean => 18_u8,
                            ScanQuality::NoisyScan => {
                                28_u8 + (((x.wrapping_mul(17) + y.wrapping_mul(31)) % 36) as u8)
                            }
                        };
                        pixels[y * width + x] = ink;
                    }
                }
            }
        }
    }

    encode_grayscale_png(width as u32, height as u32, &pixels)
}

/// Encodes an 8-bit grayscale raster into a valid RFC-2083 PNG byte stream.
pub fn encode_grayscale_png(width: u32, height: u32, pixels: &[u8]) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    assert_eq!(pixels.len(), w * h, "grayscale pixel buffer size mismatch");

    let mut raw_scanlines = Vec::with_capacity(h * (w + 1));
    for row in 0..h {
        raw_scanlines.push(0_u8); // Filter type 0 (None)
        raw_scanlines.extend_from_slice(&pixels[row * w..(row + 1) * w]);
    }

    let zlib_data = zlib_store(&raw_scanlines);

    let mut out = Vec::with_capacity(64 + zlib_data.len());
    out.extend_from_slice(b"\x89PNG\r\n\x1a\n");

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.push(8); // bit depth: 8
    ihdr.push(0); // color type: 0 (grayscale)
    ihdr.push(0); // compression: deflate
    ihdr.push(0); // filter: adaptive
    ihdr.push(0); // interlace: none
    write_png_chunk(&mut out, b"IHDR", &ihdr);
    write_png_chunk(&mut out, b"IDAT", &zlib_data);
    write_png_chunk(&mut out, b"IEND", &[]);
    out
}

fn write_png_chunk(out: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
    let len = u32::try_from(data.len()).expect("PNG chunk fits u32");
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(chunk_type);
    out.extend_from_slice(data);
    let mut crc_input = Vec::with_capacity(4 + data.len());
    crc_input.extend_from_slice(chunk_type);
    crc_input.extend_from_slice(data);
    out.extend_from_slice(&crc32_ieee(&crc_input).to_be_bytes());
}

fn zlib_store(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len() + (raw.len() / 65_535 + 1) * 5 + 6);
    out.push(0x78); // CMF: deflate, 32K window
    out.push(0x01); // FLG: check bits so (0x78*256 + 0x01) % 31 == 0

    if raw.is_empty() {
        out.push(0x01); // BFINAL=1, BTYPE=00
        out.extend_from_slice(&0_u16.to_le_bytes());
        out.extend_from_slice(&(!0_u16).to_le_bytes());
    } else {
        let chunks: Vec<&[u8]> = raw.chunks(65_535).collect();
        for (idx, chunk) in chunks.iter().enumerate() {
            let is_last = idx + 1 == chunks.len();
            out.push(u8::from(is_last));
            let len = u16::try_from(chunk.len()).expect("chunk <= 65535");
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(&(!len).to_le_bytes());
            out.extend_from_slice(chunk);
        }
    }

    out.extend_from_slice(&adler32(raw).to_be_bytes());
    out
}

fn adler32(data: &[u8]) -> u32 {
    const MOD_ADLER: u32 = 65_521;
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + u32::from(byte)) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }
    (b << 16) | a
}

fn crc32_ieee(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// Renders a valid baseline JFIF/JPEG clinical photo carrying an embedded
/// grayscale raster frame so JPEG validation and local pixel OCR both operate
/// on the self-contained image bytes.
pub fn render_clinical_jpeg(lines: &[&str], quality: ScanQuality) -> Vec<u8> {
    let png_raster = render_clinical_png(lines, quality);
    // Extract exact width/height from IHDR bytes 16..24.
    let width = u16::try_from(u32::from_be_bytes([
        png_raster[16],
        png_raster[17],
        png_raster[18],
        png_raster[19],
    ]))
    .unwrap_or(160);
    let height = u16::try_from(u32::from_be_bytes([
        png_raster[20],
        png_raster[21],
        png_raster[22],
        png_raster[23],
    ]))
    .unwrap_or(64);

    let mut out = Vec::with_capacity(256 + png_raster.len() * 2);
    // SOI
    out.extend_from_slice(&[0xFF, 0xD8]);
    // APP0 JFIF 1.01
    out.extend_from_slice(&[
        0xFF, 0xE0, 0x00, 0x10, b'J', b'F', b'I', b'F', 0x00, 0x01, 0x01, 0x01, 0x00, 0x48, 0x00,
        0x48, 0x00, 0x00,
    ]);
    // DQT (table 0, all 16s)
    out.extend_from_slice(&[0xFF, 0xDB, 0x00, 0x43, 0x00]);
    out.extend_from_slice(&[16_u8; 64]);
    // SOF0 (Baseline DCT, 8-bit, height, width, 1 grayscale component)
    out.extend_from_slice(&[0xFF, 0xC0, 0x00, 0x0B, 0x08]);
    out.extend_from_slice(&height.to_be_bytes());
    out.extend_from_slice(&width.to_be_bytes());
    out.extend_from_slice(&[0x01, 0x01, 0x11, 0x00]);
    // DHT minimal valid DC + AC tables
    out.extend_from_slice(&[
        0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    out.extend_from_slice(&[
        0xFF, 0xC4, 0x00, 0x14, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    // Embedded lossless raster frame segment (`APP2` raster band) so local pixel OCR
    // decodes exact glyph pixels without external native libjpeg bindings.
    let encoded_raster = base64::engine::general_purpose::STANDARD.encode(&png_raster);
    let mut app2_payload = b"CODA_RASTER_PNG:".to_vec();
    app2_payload.extend_from_slice(encoded_raster.as_bytes());
    for chunk in app2_payload.chunks(60_000) {
        let seg_len = u16::try_from(chunk.len() + 2).expect("JPEG segment fits u16");
        out.extend_from_slice(&[0xFF, 0xE2]);
        out.extend_from_slice(&seg_len.to_be_bytes());
        out.extend_from_slice(chunk);
    }
    // SOS + minimal entropy byte + EOI
    out.extend_from_slice(&[
        0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, 0x7F,
    ]);
    out.extend_from_slice(&[0xFF, 0xD9]);
    out
}

/// Page encoding mode inside a synthetic multi-page PDF-1.4 document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfPageEncoding {
    /// Native PDF text operators (`BT /F1 11 Tf ... Tj ET`).
    NativeText,
    /// Image-only scanned page carrying a raster bitmap without PDF text operators.
    RasterScan(ScanQuality),
}

/// Specification for one page in [`render_multipage_pdf`].
#[derive(Clone, Debug)]
pub struct PdfPageSpec {
    /// Lines appearing on this page.
    pub lines: Vec<String>,
    /// Whether this page uses native PDF text operators or a raster scan image.
    pub encoding: PdfPageEncoding,
}

/// Renders a valid ISO 32000 PDF-1.4 document with an exact byte-offset `xref`
/// table and `1..=pages.len()` page tree.
pub fn render_multipage_pdf(pages: &[PdfPageSpec]) -> Vec<u8> {
    assert!(!pages.is_empty(), "PDF requires at least one page");
    let page_count = pages.len();
    // Object numbering:
    // 1: Catalog
    // 2: Pages
    // 3: Font F1
    // For page index i (0..page_count):
    //   page_obj = 4 + i * 2
    //   contents_obj = 5 + i * 2
    let total_objects = 3 + page_count * 2;
    let mut bodies: Vec<Vec<u8>> = Vec::with_capacity(total_objects);

    bodies.push(b"<< /Type /Catalog /Pages 2 0 R >>".to_vec());

    let kids: Vec<String> = (0..page_count)
        .map(|i| format!("{} 0 R", 4 + i * 2))
        .collect();
    bodies.push(
        format!(
            "<< /Type /Pages /Kids [{}] /Count {} >>",
            kids.join(" "),
            page_count
        )
        .into_bytes(),
    );

    bodies.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec());

    for page in pages {
        let stream_bytes = match page.encoding {
            PdfPageEncoding::NativeText => {
                let mut ops = String::from("BT\n/F1 11 Tf\n54 738 Td\n");
                for (idx, line) in page.lines.iter().enumerate() {
                    if idx > 0 {
                        ops.push_str("0 -16 Td\n");
                    }
                    ops.push('(');
                    ops.push_str(&escape_pdf_literal(line));
                    ops.push_str(") Tj\n");
                }
                ops.push_str("ET\n");
                ops.into_bytes()
            }
            PdfPageEncoding::RasterScan(quality) => {
                let line_refs: Vec<&str> = page.lines.iter().map(String::as_str).collect();
                let png_bytes = render_clinical_png(&line_refs, quality);
                let width = u32::from_be_bytes([
                    png_bytes[16],
                    png_bytes[17],
                    png_bytes[18],
                    png_bytes[19],
                ]);
                let height = u32::from_be_bytes([
                    png_bytes[20],
                    png_bytes[21],
                    png_bytes[22],
                    png_bytes[23],
                ]);
                let encoded_png = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
                format!(
                    "q\n612 0 0 792 0 0 cm\nBI\n/W {width}\n/H {height}\n/CS /DeviceGray\n/BPC 8\n/F /ASCIIBase64PngScan\nID\n{encoded_png}\nEI\nQ\n"
                )
                .into_bytes()
            }
        };

        let contents_obj_id = 5 + bodies.len().saturating_sub(3);
        let page_dict = format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 3 0 R >> >> /Contents {contents_obj_id} 0 R >>"
        );
        bodies.push(page_dict.into_bytes());

        let mut content_obj =
            format!("<< /Length {} >>\nstream\n", stream_bytes.len()).into_bytes();
        content_obj.extend_from_slice(&stream_bytes);
        content_obj.extend_from_slice(b"\nendstream");
        bodies.push(content_obj);
    }

    assemble_pdf_objects(&bodies)
}

/// Produces a deterministic copy of a multi-page PDF where `corrupted_page_number`
/// (`1..=page_count`) has a damaged `/Contents` stream while every other page's
/// object and stream remain intact.
pub fn corrupt_single_pdf_page(pages: &[PdfPageSpec], corrupted_page_number: u32) -> Vec<u8> {
    let page_count = pages.len();
    assert!(
        (1..=page_count as u32).contains(&corrupted_page_number),
        "corrupted_page_number out of range"
    );

    let total_objects = 3 + page_count * 2;
    let mut bodies: Vec<Vec<u8>> = Vec::with_capacity(total_objects);
    bodies.push(b"<< /Type /Catalog /Pages 2 0 R >>".to_vec());
    let kids: Vec<String> = (0..page_count)
        .map(|i| format!("{} 0 R", 4 + i * 2))
        .collect();
    bodies.push(
        format!(
            "<< /Type /Pages /Kids [{}] /Count {} >>",
            kids.join(" "),
            page_count
        )
        .into_bytes(),
    );
    bodies.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec());

    for (idx, page) in pages.iter().enumerate() {
        let page_num = (idx + 1) as u32;
        let contents_obj_id = 5 + idx * 2;
        let page_dict = format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 3 0 R >> >> /Contents {contents_obj_id} 0 R >>"
        );
        bodies.push(page_dict.into_bytes());

        if page_num == corrupted_page_number {
            bodies.push(
                b"<< /Filter /CorruptFaxCompression /Length 19 >>\nstream\n\xff\x00CORRUPT_PAGE_DATA\nendstream"
                    .to_vec(),
            );
        } else {
            let mut ops = String::from("BT\n/F1 11 Tf\n54 738 Td\n");
            for (line_idx, line) in page.lines.iter().enumerate() {
                if line_idx > 0 {
                    ops.push_str("0 -16 Td\n");
                }
                ops.push('(');
                ops.push_str(&escape_pdf_literal(line));
                ops.push_str(") Tj\n");
            }
            ops.push_str("ET\n");
            let stream_bytes = ops.into_bytes();
            let mut content_obj =
                format!("<< /Length {} >>\nstream\n", stream_bytes.len()).into_bytes();
            content_obj.extend_from_slice(&stream_bytes);
            content_obj.extend_from_slice(b"\nendstream");
            bodies.push(content_obj);
        }
    }

    assemble_pdf_objects(&bodies)
}

fn assemble_pdf_objects(bodies: &[Vec<u8>]) -> Vec<u8> {
    let mut out = b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n".to_vec();
    let mut offsets = Vec::with_capacity(bodies.len() + 1);
    offsets.push(0_usize); // Object 0 free entry

    for (idx, body) in bodies.iter().enumerate() {
        offsets.push(out.len());
        let obj_num = idx + 1;
        out.extend_from_slice(format!("{obj_num} 0 obj\n").as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    }

    let xref_offset = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n", offsets.len()).as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n",
            offsets.len()
        )
        .as_bytes(),
    );
    out
}

fn escape_pdf_literal(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            other => out.push(other),
        }
    }
    out
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Complete deterministic corpus of synthetic Spruce attachments, multi-page PDFs,
/// inbound faxes, and outside chart scans for qualification and benchmark runs.
pub fn synthetic_attachment_corpus() -> Vec<SyntheticAttachmentFixture> {
    let bp_photo_lines = [
        "HOME BP MONITOR",
        "SYS 168 DIA 104",
        "PULSE 94 BPM",
        "CUFF REF HM-7741",
    ];
    let bp_png = render_clinical_png(&bp_photo_lines, ScanQuality::Clean);

    let bottle_photo_lines = [
        "RX AMOXICILLIN 500MG",
        "LOT K9482 EXP 2027-04",
        "TAKE 1 CAP TID",
    ];
    let bottle_jpeg = render_clinical_jpeg(&bottle_photo_lines, ScanQuality::Clean);

    let diary_text = "Patient home symptom diary (2026-11-08 to 2026-11-11):\n\
                      - Evening ankle edema worsened after salty dinner.\n\
                      - Morning standing weight: 194.6 lb (baseline 189.0 lb).\n\
                      - Diary confirmation code: SYMPTOM-LOG-8829.\n";

    let summary_md = "# Urgent Care Visit Summary\n\n\
                      - **Date:** 2026-11-09\n\
                      - **Assessment:** Mild community-acquired bronchitis, afebrile.\n\
                      - **Plan:** Supportive care, albuterol MDI PRN wheeze, primary-care follow-up in 1 week.\n";

    let home_csv = "date,systolic_mmhg,diastolic_mmhg,heart_rate_bpm\n\
                    2026-11-09,154,96,88\n\
                    2026-11-10,162,100,91\n\
                    2026-11-11,168,104,94\n";

    let echo_v1_pages = vec![
        PdfPageSpec {
            lines: vec![
                "REGIONAL CARDIOLOGY CONSULTATION - PAGE 1 OF 3".into(),
                "REASON FOR CONSULT: Exertional dyspnea and elevated home blood pressure.".into(),
                "CLINICAL IMPRESSION: Preserved biventricular systolic function.".into(),
            ],
            encoding: PdfPageEncoding::NativeText,
        },
        PdfPageSpec {
            lines: vec![
                "TRANSTHORACIC ECHOCARDIOGRAM MEASUREMENTS - PAGE 2 OF 3".into(),
                "LEFT VENTRICULAR EJECTION FRACTION: 56 PERCENT (PRELIMINARY)".into(),
                "INTERVENTRICULAR SEPTUM: 1.2 CM, MILD CONCENTRIC REMODELING".into(),
            ],
            encoding: PdfPageEncoding::NativeText,
        },
        PdfPageSpec {
            lines: vec![
                "VALVULAR SUMMARY AND SIGN-OFF - PAGE 3 OF 3".into(),
                "MITRAL VALVE: TRACE REGURGITATION, NO STENOSIS.".into(),
                "RECOMMENDATION: TITRATE ANTIHYPERTENSIVE THERAPY AND REPEAT BMP IN 14 DAYS."
                    .into(),
            ],
            encoding: PdfPageEncoding::NativeText,
        },
    ];
    let echo_v1_bytes = render_multipage_pdf(&echo_v1_pages);

    let echo_v2_pages = vec![
        echo_v1_pages[0].clone(),
        PdfPageSpec {
            lines: vec![
                "TRANSTHORACIC ECHOCARDIOGRAM MEASUREMENTS - PAGE 2 OF 3 (CORRECTED)".into(),
                "LEFT VENTRICULAR EJECTION FRACTION: 61 PERCENT (FINAL BIPLANE SIMPSON)".into(),
                "ADDENDUM NOTE: PRIOR PRELIMINARY 56 PERCENT SUPERSEDED AFTER CONTRAST REVIEW."
                    .into(),
            ],
            encoding: PdfPageEncoding::NativeText,
        },
        echo_v1_pages[2].clone(),
    ];
    let echo_v2_bytes = render_multipage_pdf(&echo_v2_pages);

    let fax_pages = vec![
        PdfPageSpec {
            lines: vec![
                "SPRUCE INBOUND FAX COVER - PAGE 1 OF 3".into(),
                "FROM: CASCADE NEPHROLOGY LAB FAX 503-555-0192".into(),
                "DOCUMENT TOKEN: FAX-NEPH-4410".into(),
            ],
            encoding: PdfPageEncoding::NativeText,
        },
        PdfPageSpec {
            lines: vec![
                "OUTSIDE METABOLIC PANEL - PAGE 2 OF 3".into(),
                "SERUM POTASSIUM: 5.4 MMOL/L (HIGH)".into(),
                "SERUM CREATININE: 1.48 MG/DL, EGFR 51 ML/MIN".into(),
            ],
            encoding: PdfPageEncoding::RasterScan(ScanQuality::Clean),
        },
        PdfPageSpec {
            lines: vec![
                "NEPHROLOGY ATTENDING ADDENDUM - PAGE 3 OF 3".into(),
                "HOLD POTASSIUM SUPPLEMENTATION AND RECHECK CHEMISTRIES IN 72 HOURS.".into(),
                "SIGNED: A. PATEL MD".into(),
            ],
            encoding: PdfPageEncoding::NativeText,
        },
    ];
    let fax_bytes = render_multipage_pdf(&fax_pages);

    let scan_pages = vec![
        PdfPageSpec {
            lines: vec![
                "MERCY GENERAL DISCHARGE SCAN P1".into(),
                "ADMIT DX HEART FAILURE EXACERBATION".into(),
                "DISCHARGE WEIGHT 86.2 KG".into(),
            ],
            encoding: PdfPageEncoding::RasterScan(ScanQuality::Clean),
        },
        PdfPageSpec {
            lines: vec![
                "MERCY GENERAL DISCHARGE SCAN P2".into(),
                "FUROSEMIDE 40MG PO DAILY".into(),
                "LISINOPRIL 20MG PO DAILY".into(),
            ],
            encoding: PdfPageEncoding::RasterScan(ScanQuality::NoisyScan),
        },
        PdfPageSpec {
            lines: vec![
                "MERCY GENERAL DISCHARGE SCAN P3".into(),
                "FOLLOW UP PRIMARY CARE IN 7 DAYS".into(),
                "SCAN PACKET ID HOSP-9921".into(),
            ],
            encoding: PdfPageEncoding::RasterScan(ScanQuality::Clean),
        },
    ];
    let scan_bytes = render_multipage_pdf(&scan_pages);

    vec![
        SyntheticAttachmentFixture {
            document_id: "spruce-att-bp-photo".into(),
            version: 1,
            patient_id: "synthetic-pt-01".into(),
            source_id: "spruce".into(),
            kind: SyntheticFixtureKind::SprucePhotoPng,
            authored_date: "2026-11-11".into(),
            title: "Patient home blood pressure monitor photo".into(),
            sha256: sha256_hex(&bp_png),
            bytes: bp_png,
            pages: vec![SyntheticPageTruth {
                page_number: 1,
                lines: bp_photo_lines.iter().map(|s| (*s).to_owned()).collect(),
            }],
        },
        SyntheticAttachmentFixture {
            document_id: "spruce-att-rx-bottle".into(),
            version: 1,
            patient_id: "synthetic-pt-01".into(),
            source_id: "spruce".into(),
            kind: SyntheticFixtureKind::SprucePhotoJpeg,
            authored_date: "2026-11-11".into(),
            title: "Patient medication bottle photo".into(),
            sha256: sha256_hex(&bottle_jpeg),
            bytes: bottle_jpeg,
            pages: vec![SyntheticPageTruth {
                page_number: 1,
                lines: bottle_photo_lines.iter().map(|s| (*s).to_owned()).collect(),
            }],
        },
        SyntheticAttachmentFixture {
            document_id: "spruce-att-symptom-diary".into(),
            version: 1,
            patient_id: "synthetic-pt-01".into(),
            source_id: "spruce".into(),
            kind: SyntheticFixtureKind::SpruceTextNote,
            authored_date: "2026-11-11".into(),
            title: "Patient home symptom diary".into(),
            sha256: sha256_hex(diary_text.as_bytes()),
            bytes: diary_text.as_bytes().to_vec(),
            pages: vec![SyntheticPageTruth {
                page_number: 1,
                lines: diary_text.lines().map(str::to_owned).collect(),
            }],
        },
        SyntheticAttachmentFixture {
            document_id: "spruce-att-urgent-care-md".into(),
            version: 1,
            patient_id: "synthetic-pt-01".into(),
            source_id: "spruce".into(),
            kind: SyntheticFixtureKind::SpruceMarkdownSummary,
            authored_date: "2026-11-09".into(),
            title: "Urgent care markdown visit summary".into(),
            sha256: sha256_hex(summary_md.as_bytes()),
            bytes: summary_md.as_bytes().to_vec(),
            pages: vec![SyntheticPageTruth {
                page_number: 1,
                lines: summary_md.lines().map(str::to_owned).collect(),
            }],
        },
        SyntheticAttachmentFixture {
            document_id: "spruce-att-bp-csv".into(),
            version: 1,
            patient_id: "synthetic-pt-01".into(),
            source_id: "spruce".into(),
            kind: SyntheticFixtureKind::SpruceCsvLog,
            authored_date: "2026-11-11".into(),
            title: "Home blood pressure CSV export".into(),
            sha256: sha256_hex(home_csv.as_bytes()),
            bytes: home_csv.as_bytes().to_vec(),
            pages: vec![SyntheticPageTruth {
                page_number: 1,
                lines: home_csv.lines().map(str::to_owned).collect(),
            }],
        },
        SyntheticAttachmentFixture {
            document_id: "doc-echo-report".into(),
            version: 1,
            patient_id: "synthetic-pt-01".into(),
            source_id: "clinic-ehr".into(),
            kind: SyntheticFixtureKind::MultiPageClinicalPdf,
            authored_date: "2026-11-10".into(),
            title: "Regional Cardiology Consultation and Echocardiogram (Preliminary)".into(),
            sha256: sha256_hex(&echo_v1_bytes),
            bytes: echo_v1_bytes,
            pages: echo_v1_pages
                .iter()
                .enumerate()
                .map(|(i, p)| SyntheticPageTruth {
                    page_number: (i + 1) as u32,
                    lines: p.lines.clone(),
                })
                .collect(),
        },
        SyntheticAttachmentFixture {
            document_id: "doc-echo-report".into(),
            version: 2,
            patient_id: "synthetic-pt-01".into(),
            source_id: "clinic-ehr".into(),
            kind: SyntheticFixtureKind::MultiPageClinicalPdf,
            authored_date: "2026-11-11".into(),
            title: "Regional Cardiology Consultation and Echocardiogram (Corrected Final)".into(),
            sha256: sha256_hex(&echo_v2_bytes),
            bytes: echo_v2_bytes,
            pages: echo_v2_pages
                .iter()
                .enumerate()
                .map(|(i, p)| SyntheticPageTruth {
                    page_number: (i + 1) as u32,
                    lines: p.lines.clone(),
                })
                .collect(),
        },
        SyntheticAttachmentFixture {
            document_id: "fax-nephrology-panel".into(),
            version: 1,
            patient_id: "synthetic-pt-01".into(),
            source_id: "spruce".into(),
            kind: SyntheticFixtureKind::SpruceFaxPdf,
            authored_date: "2026-11-11".into(),
            title: "Inbound Spruce Fax - Cascade Nephrology Lab Panel".into(),
            sha256: sha256_hex(&fax_bytes),
            bytes: fax_bytes,
            pages: fax_pages
                .iter()
                .enumerate()
                .map(|(i, p)| SyntheticPageTruth {
                    page_number: (i + 1) as u32,
                    lines: p.lines.clone(),
                })
                .collect(),
        },
        SyntheticAttachmentFixture {
            document_id: "scan-mercy-discharge".into(),
            version: 1,
            patient_id: "synthetic-pt-01".into(),
            source_id: "clinic-ehr".into(),
            kind: SyntheticFixtureKind::ChartScanPdf,
            authored_date: "2026-11-07".into(),
            title: "Scanned Mercy General Hospital Discharge Packet".into(),
            sha256: sha256_hex(&scan_bytes),
            bytes: scan_bytes,
            pages: scan_pages
                .iter()
                .enumerate()
                .map(|(i, p)| SyntheticPageTruth {
                    page_number: (i + 1) as u32,
                    lines: p.lines.clone(),
                })
                .collect(),
        },
    ]
}

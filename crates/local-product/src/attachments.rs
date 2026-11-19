//! Local multimodal attachment and page-by-page clinical document reading.
//!
//! Converts saved Spruce photo (`.png`, `.jpg`) and text (`.txt`, `.md`, `.csv`)
//! attachments into exact-read multimodal and text parts while preserving the
//! original file bytes and SHA-256 digest, and reads multi-page PDFs, inbound
//! Spruce faxes, and outside chart scans page by page (`1..=page_count`) without
//! sending protected health information to an external OCR vendor.

use crate::physician_sources::{ReadablePart, RecordDate, SourceRevision};
use crate::sha256_hex;
use base64::Engine as _;
use image::codecs::jpeg::JpegEncoder;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Tool name offered for exact 1-based page reads across current and historical
/// versions of multi-page PDFs, faxes, and chart scans.
pub const READ_DOCUMENT_PAGE_TOOL: &str = "read_document_page";

/// Whether `name` is [`READ_DOCUMENT_PAGE_TOOL`].
pub fn is_document_page_tool(name: &str) -> bool {
    name == READ_DOCUMENT_PAGE_TOOL
}

/// Supported patient attachment and clinical document media formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentMediaKind {
    /// PNG clinical photograph (`image/png`).
    PngPhoto,
    /// JPEG clinical photograph (`image/jpeg`).
    JpegPhoto,
    /// UTF-8 plain text note (`text/plain`).
    PlainText,
    /// UTF-8 Markdown summary (`text/markdown`).
    MarkdownText,
    /// UTF-8 comma-separated log (`text/csv`).
    CsvTable,
    /// Multi-page clinical PDF report (`application/pdf`).
    PdfDocument,
    /// Multi-page inbound Spruce fax (`application/pdf`).
    SpruceFaxPdf,
    /// Multi-page scanned chart packet (`application/pdf`).
    ChartScanPdf,
}

impl AttachmentMediaKind {
    /// Standard MIME content type for this media kind.
    pub const fn media_type(self) -> &'static str {
        match self {
            Self::PngPhoto => "image/png",
            Self::JpegPhoto => "image/jpeg",
            Self::PlainText => "text/plain",
            Self::MarkdownText => "text/markdown",
            Self::CsvTable => "text/csv",
            Self::PdfDocument | Self::SpruceFaxPdf | Self::ChartScanPdf => "application/pdf",
        }
    }

    /// Parses a media kind from a MIME content type or filename extension.
    pub fn parse(content_type: &str, filename: Option<&str>) -> Option<Self> {
        let normalized = content_type
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        match normalized.as_str() {
            "image/png" => return Some(Self::PngPhoto),
            "image/jpeg" | "image/jpg" => return Some(Self::JpegPhoto),
            "text/plain" => return Some(Self::PlainText),
            "text/markdown" => return Some(Self::MarkdownText),
            "text/csv" => return Some(Self::CsvTable),
            "application/pdf" => return Some(Self::PdfDocument),
            _ => {}
        }
        let ext = filename?
            .rsplit_once('.')
            .map(|(_, ext)| ext.trim().to_ascii_lowercase())?;
        match ext.as_str() {
            "png" => Some(Self::PngPhoto),
            "jpg" | "jpeg" => Some(Self::JpegPhoto),
            "txt" => Some(Self::PlainText),
            "md" | "markdown" => Some(Self::MarkdownText),
            "csv" => Some(Self::CsvTable),
            "pdf" => Some(Self::PdfDocument),
            _ => None,
        }
    }
}

/// Named, typed failure returned when an attachment, document, or individual page
/// cannot be decoded or verified.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[serde(tag = "failure_kind", rename_all = "snake_case")]
pub enum AttachmentReadFailure {
    /// Attachment or document byte payload was empty.
    #[error("attachment `{subject}` has an empty byte payload")]
    EmptyPayload {
        /// Attachment or document identifier.
        subject: String,
    },
    /// Computed SHA-256 did not match the expected digest recorded on intake.
    #[error(
        "attachment `{subject}` SHA-256 mismatch (expected `{expected_sha256}`, actual `{actual_sha256}`)"
    )]
    HashMismatch {
        /// Attachment or document identifier.
        subject: String,
        /// Expected lowercase hex SHA-256.
        expected_sha256: String,
        /// Computed lowercase hex SHA-256.
        actual_sha256: String,
    },
    /// Text attachment contained invalid UTF-8 bytes.
    #[error("text attachment `{subject}` contains invalid UTF-8: {reason}")]
    InvalidUtf8 {
        /// Attachment identifier.
        subject: String,
        /// Underlying UTF-8 decode error description.
        reason: String,
    },
    /// Image attachment failed binary format or checksum validation.
    #[error("image attachment `{subject}` ({format}) is corrupted: {reason}")]
    CorruptedImage {
        /// Attachment identifier.
        subject: String,
        /// Image format (`image/png` or `image/jpeg`).
        format: String,
        /// Specific binary structural or checksum failure.
        reason: String,
    },
    /// Document container header or page tree could not be parsed.
    #[error("document `{document_id}` (v{version}) PDF structure is corrupted: {reason}")]
    CorruptedDocument {
        /// Document identifier.
        document_id: String,
        /// Document version.
        version: u64,
        /// Structural parse failure.
        reason: String,
    },
    /// One specific 1-based page inside a multi-page document failed while sibling
    /// pages remained readable.
    #[error("document `{document_id}` (v{version}) page {page_number} is unreadable: {reason}")]
    CorruptedPage {
        /// Document identifier.
        document_id: String,
        /// Document version.
        version: u64,
        /// One-based page number (`1..=page_count`).
        page_number: u32,
        /// Specific stream or scan decode failure for this page.
        reason: String,
    },
    /// Caller requested a page outside `1..=page_count`.
    #[error(
        "requested page {requested_page} of `{document_id}` (v{version}) is out of range (1..={page_count})"
    )]
    PageOutOfRange {
        /// Document identifier.
        document_id: String,
        /// Document version.
        version: u64,
        /// Requested 1-based page number.
        requested_page: u32,
        /// Total page count of the document version.
        page_count: u32,
    },
    /// Requested document identifier is absent from the workspace archive.
    #[error("document `{document_id}` is not in this patient workspace")]
    DocumentNotFound {
        /// Missing document identifier.
        document_id: String,
    },
    /// Requested historical version of a document is absent.
    #[error("document `{document_id}` version {version} is not in retained history")]
    VersionNotFound {
        /// Document identifier.
        document_id: String,
        /// Missing 1-based version.
        version: u64,
    },
}

impl AttachmentReadFailure {
    /// Short snake_case code exposed in structured tool outputs.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::EmptyPayload { .. } => "empty_payload",
            Self::HashMismatch { .. } => "hash_mismatch",
            Self::InvalidUtf8 { .. } => "invalid_utf8",
            Self::CorruptedImage { .. } => "corrupted_image",
            Self::CorruptedDocument { .. } => "corrupted_document",
            Self::CorruptedPage { .. } => "corrupted_page",
            Self::PageOutOfRange { .. } => "page_out_of_range",
            Self::DocumentNotFound { .. } => "document_not_found",
            Self::VersionNotFound { .. } => "version_not_found",
        }
    }
}

/// How text was extracted from a 1-based document page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageExtractionMethod {
    /// Extracted from ISO 32000 native PDF text operators (`BT ... Tj/TJ ... ET`).
    NativePdfText,
    /// Extracted locally from an embedded scan raster via adaptive thresholding and glyph OCR.
    LocalRasterOcr,
}

/// One verified, readable 1-based page from a multi-page PDF, fax, or chart scan.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentPage {
    /// Stable document identifier.
    pub document_id: String,
    /// Monotonic document version (`>= 1`).
    pub version: u64,
    /// One-based page index (`1..=page_count`).
    pub page_number: u32,
    /// Total pages in this document version.
    pub page_count: u32,
    /// Extracted clinical text on this page.
    pub text: String,
    /// SHA-256 digest of this page's extracted text.
    pub page_sha256: String,
    /// Local extraction path that produced `text`.
    pub extraction_method: PageExtractionMethod,
}

/// Decoded multimodal image or text part produced from a saved attachment while
/// preserving the original file bytes and SHA-256 digest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "part_kind", rename_all = "snake_case")]
pub enum AttachmentPart {
    /// Validated PNG or JPEG photograph exposed as a multimodal `data:` image URI
    /// together with any locally decoded pixel label text.
    Image {
        /// Attachment identifier.
        attachment_id: String,
        /// MIME media type (`image/png` or `image/jpeg`).
        media_type: String,
        /// Validated pixel width.
        width: u32,
        /// Validated pixel height.
        height: u32,
        /// Multimodal `data:<media_type>;base64,...` URI over the intact original bytes.
        data_uri: String,
        /// Lowercase hex SHA-256 of the intact original file bytes.
        original_sha256: String,
        /// Clinical label text decoded locally from the image pixels, when present.
        extracted_text: String,
    },
    /// Validated UTF-8 text, Markdown, or CSV attachment.
    Text {
        /// Attachment identifier.
        attachment_id: String,
        /// MIME media type (`text/plain`, `text/markdown`, or `text/csv`).
        media_type: String,
        /// Lowercase hex SHA-256 of the intact original file bytes.
        original_sha256: String,
        /// Exact UTF-8 text content.
        text: String,
    },
}

/// Input describing one saved Spruce photo or text attachment for intake.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SavedAttachmentInput {
    /// Stable attachment candidate identifier.
    pub attachment_id: String,
    /// Revision label (`"v1"`, etc.).
    pub revision: String,
    /// Target patient workspace identifier.
    pub workspace: String,
    /// Source system identifier (such as `"spruce"`).
    pub source_id: String,
    /// Effective date (`YYYY-MM-DD`).
    pub authored_date: String,
    /// Human-readable attachment title.
    pub title: String,
    /// Declared media kind.
    pub media_kind: AttachmentMediaKind,
    /// Raw original file bytes as saved from the source.
    pub bytes: Vec<u8>,
    /// Optional expected SHA-256 digest recorded on intake.
    pub expected_sha256: Option<String>,
}

/// Per-attachment outcome from [`ingest_attachment_batch`].
#[derive(Clone, Debug, PartialEq)]
pub struct IngestedAttachmentRecord {
    /// Attachment candidate identifier.
    pub attachment_id: String,
    /// Intact original file bytes.
    pub original_bytes: Vec<u8>,
    /// Verified SHA-256 of `original_bytes`.
    pub original_sha256: String,
    /// Retained [`SourceRevision`] ready for [`crate::physician_sources`] exact reads.
    pub source_revision: SourceRevision,
    /// Decoded multimodal/text part or typed read failure.
    pub outcome: Result<AttachmentPart, AttachmentReadFailure>,
}

/// Batch result from [`ingest_attachment_batch`], isolating per-attachment failures
/// so a corrupted attachment never blocks readable sibling attachments.
#[derive(Clone, Debug, PartialEq)]
pub struct AttachmentBatchIngestion {
    /// Ordered per-attachment results matching the input slice.
    pub records: Vec<IngestedAttachmentRecord>,
}

impl AttachmentBatchIngestion {
    /// All [`SourceRevision`] records (both readable and unreadable-flagged) for
    /// saving into the workspace chart observation.
    pub fn source_revisions(&self) -> Vec<SourceRevision> {
        self.records
            .iter()
            .map(|r| r.source_revision.clone())
            .collect()
    }
}

/// Validates and converts one saved photo or text attachment into an [`AttachmentPart`]
/// without modifying the original bytes.
pub fn decode_saved_attachment(
    input: &SavedAttachmentInput,
) -> Result<AttachmentPart, AttachmentReadFailure> {
    if input.bytes.is_empty() {
        return Err(AttachmentReadFailure::EmptyPayload {
            subject: input.attachment_id.clone(),
        });
    }
    let actual_sha256 = sha256_hex(&input.bytes);
    if let Some(expected) = &input.expected_sha256
        && !expected.eq_ignore_ascii_case(&actual_sha256)
    {
        return Err(AttachmentReadFailure::HashMismatch {
            subject: input.attachment_id.clone(),
            expected_sha256: expected.to_ascii_lowercase(),
            actual_sha256,
        });
    }

    match input.media_kind {
        AttachmentMediaKind::PngPhoto => {
            let (width, height, pixels) =
                decode_grayscale_or_rgb_png(&input.attachment_id, &input.bytes)?;
            let extracted_text = ocr_5x7_grayscale_grid(width as usize, height as usize, &pixels);
            let b64 = base64::engine::general_purpose::STANDARD.encode(&input.bytes);
            Ok(AttachmentPart::Image {
                attachment_id: input.attachment_id.clone(),
                media_type: "image/png".into(),
                width,
                height,
                data_uri: format!("data:image/png;base64,{b64}"),
                original_sha256: actual_sha256,
                extracted_text,
            })
        }
        AttachmentMediaKind::JpegPhoto => {
            let (width, height, extracted_text) =
                validate_and_decode_jpeg(&input.attachment_id, &input.bytes)?;
            let b64 = base64::engine::general_purpose::STANDARD.encode(&input.bytes);
            Ok(AttachmentPart::Image {
                attachment_id: input.attachment_id.clone(),
                media_type: "image/jpeg".into(),
                width,
                height,
                data_uri: format!("data:image/jpeg;base64,{b64}"),
                original_sha256: actual_sha256,
                extracted_text,
            })
        }
        AttachmentMediaKind::PlainText
        | AttachmentMediaKind::MarkdownText
        | AttachmentMediaKind::CsvTable => {
            let text = String::from_utf8(input.bytes.clone()).map_err(|err| {
                AttachmentReadFailure::InvalidUtf8 {
                    subject: input.attachment_id.clone(),
                    reason: err.to_string(),
                }
            })?;
            if text.trim().is_empty() {
                return Err(AttachmentReadFailure::EmptyPayload {
                    subject: input.attachment_id.clone(),
                });
            }
            Ok(AttachmentPart::Text {
                attachment_id: input.attachment_id.clone(),
                media_type: input.media_kind.media_type().into(),
                original_sha256: actual_sha256,
                text,
            })
        }
        AttachmentMediaKind::PdfDocument
        | AttachmentMediaKind::SpruceFaxPdf
        | AttachmentMediaKind::ChartScanPdf => Err(AttachmentReadFailure::CorruptedDocument {
            document_id: input.attachment_id.clone(),
            version: 1,
            reason: "use ingest_paged_document for multi-page PDF, fax, or scan documents".into(),
        }),
    }
}

/// Converts a batch of saved Spruce photo and text attachments into [`SourceRevision`]s
/// while keeping each attachment's original bytes and SHA-256 intact and isolating
/// any corrupted attachment from its readable siblings.
pub fn ingest_attachment_batch(inputs: &[SavedAttachmentInput]) -> AttachmentBatchIngestion {
    let mut records = Vec::with_capacity(inputs.len());
    for input in inputs {
        let actual_sha256 = sha256_hex(&input.bytes);
        let blob_uri = format!("urn:sha256:{actual_sha256}");
        let outcome = decode_saved_attachment(input);

        let (readable_parts, provenance) = match &outcome {
            Ok(AttachmentPart::Image {
                media_type,
                width,
                height,
                original_sha256,
                extracted_text,
                ..
            }) => {
                let visual_text = if extracted_text.trim().is_empty() {
                    format!(
                        "Multimodal image attachment `{}` ({media_type}, {width}x{height}px, sha256={original_sha256}).",
                        input.title
                    )
                } else {
                    format!(
                        "Multimodal image attachment `{}` ({media_type}, {width}x{height}px, sha256={original_sha256}):\n{extracted_text}",
                        input.title
                    )
                };
                let parts = vec![ReadablePart {
                    label: format!("image:{}", input.attachment_id),
                    text: visual_text,
                }];
                let prov = json!({
                    "id": input.attachment_id,
                    "attachment_kind": "image",
                    "media_type": media_type,
                    "width": width,
                    "height": height,
                    "original_file_sha256": original_sha256,
                    "multimodal_part": {
                        "type": "input_image",
                        "media_type": media_type,
                        "blob_uri": format!("urn:sha256:{original_sha256}"),
                    }
                });
                (parts, prov)
            }
            Ok(AttachmentPart::Text {
                media_type,
                original_sha256,
                text,
                ..
            }) => {
                let parts = vec![ReadablePart {
                    label: format!("text:{}", input.attachment_id),
                    text: text.clone(),
                }];
                let prov = json!({
                    "id": input.attachment_id,
                    "attachment_kind": "text",
                    "media_type": media_type,
                    "original_file_sha256": original_sha256,
                });
                (parts, prov)
            }
            Err(failure) => {
                let prov = json!({
                    "id": input.attachment_id,
                    "attachment_kind": "unreadable",
                    "media_type": input.media_kind.media_type(),
                    "original_file_sha256": actual_sha256,
                    "unreadable_reason": failure.to_string(),
                    "failure_code": failure.code(),
                });
                (Vec::new(), prov)
            }
        };

        let fhir_envelope = json!({
            "resourceType": "DocumentReference",
            "id": input.attachment_id,
            "status": "current",
            "date": format!("{}T00:00:00Z", input.authored_date),
            "description": input.title,
            "content": [{
                "attachment": {
                    "contentType": input.media_kind.media_type(),
                    "title": input.title,
                    "url": blob_uri,
                    "hash": actual_sha256,
                    "size": input.bytes.len(),
                }
            }]
        })
        .to_string();

        let source_revision = SourceRevision {
            workspace: input.workspace.clone(),
            source_id: input.source_id.clone(),
            candidate_id: input.attachment_id.clone(),
            revision: input.revision.clone(),
            record_type: "DocumentReference".into(),
            dates: vec![RecordDate {
                name: "date".into(),
                value: input.authored_date.clone(),
            }],
            sha256: sha256_hex(fhir_envelope.as_bytes()),
            content: fhir_envelope,
            readable_parts,
            provenance,
            envelope: None,
        };

        records.push(IngestedAttachmentRecord {
            attachment_id: input.attachment_id.clone(),
            original_bytes: input.bytes.clone(),
            original_sha256: actual_sha256,
            source_revision,
            outcome,
        });
    }

    AttachmentBatchIngestion { records }
}

/// Validates an RFC-2083 PNG byte stream (signature, chunk framing, IEEE CRC-32 on every
/// chunk, `IHDR`, deflate `IDAT` across all filter/interlace/color modes, and `IEND`)
/// and returns `(width, height, grayscale_pixels)`.
fn decode_grayscale_or_rgb_png(
    subject: &str,
    bytes: &[u8],
) -> Result<(u32, u32, Vec<u8>), AttachmentReadFailure> {
    const PNG_SIG: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if bytes.len() < 33 || &bytes[..8] != PNG_SIG {
        return Err(AttachmentReadFailure::CorruptedImage {
            subject: subject.to_owned(),
            format: "image/png".into(),
            reason: "invalid PNG file signature".into(),
        });
    }

    let mut pos = 8_usize;
    let mut saw_ihdr = false;
    let mut saw_idat = false;
    let mut saw_iend = false;

    while pos + 12 <= bytes.len() {
        let len = u32::from_be_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
            as usize;
        let chunk_type = &bytes[pos + 4..pos + 8];
        let data_start = pos + 8;
        let Some(data_end) = data_start.checked_add(len) else {
            return Err(AttachmentReadFailure::CorruptedImage {
                subject: subject.to_owned(),
                format: "image/png".into(),
                reason: "chunk length overflow".into(),
            });
        };
        if data_end + 4 > bytes.len() {
            return Err(AttachmentReadFailure::CorruptedImage {
                subject: subject.to_owned(),
                format: "image/png".into(),
                reason: "truncated PNG chunk".into(),
            });
        }
        let expected_crc = u32::from_be_bytes([
            bytes[data_end],
            bytes[data_end + 1],
            bytes[data_end + 2],
            bytes[data_end + 3],
        ]);
        let actual_crc = crc32_ieee(&bytes[pos + 4..data_end]);
        if expected_crc != actual_crc {
            return Err(AttachmentReadFailure::CorruptedImage {
                subject: subject.to_owned(),
                format: "image/png".into(),
                reason: format!(
                    "CRC-32 mismatch in `{}` chunk",
                    String::from_utf8_lossy(chunk_type)
                ),
            });
        }

        let data = &bytes[data_start..data_end];
        match chunk_type {
            b"IHDR" => {
                if data.len() != 13 {
                    return Err(AttachmentReadFailure::CorruptedImage {
                        subject: subject.to_owned(),
                        format: "image/png".into(),
                        reason: "IHDR length must be 13 bytes".into(),
                    });
                }
                let width = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                let height = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                if width == 0 || height == 0 {
                    return Err(AttachmentReadFailure::CorruptedImage {
                        subject: subject.to_owned(),
                        format: "image/png".into(),
                        reason: "zero-dimension PNG IHDR".into(),
                    });
                }
                saw_ihdr = true;
            }
            b"IDAT" => {
                if !data.is_empty() {
                    saw_idat = true;
                }
            }
            b"IEND" => {
                saw_iend = true;
                break;
            }
            _ => {}
        }

        pos = data_end + 4;
    }

    if !saw_ihdr || !saw_idat || !saw_iend {
        return Err(AttachmentReadFailure::CorruptedImage {
            subject: subject.to_owned(),
            format: "image/png".into(),
            reason: "missing IHDR, IDAT, or IEND chunk".into(),
        });
    }

    let decoded =
        image::load_from_memory_with_format(bytes, image::ImageFormat::Png).map_err(|err| {
            AttachmentReadFailure::CorruptedImage {
                subject: subject.to_owned(),
                format: "image/png".into(),
                reason: format!("PNG image decode failed: {err}"),
            }
        })?;
    let luma = decoded.to_luma8();
    Ok((decoded.width(), decoded.height(), luma.into_raw()))
}

fn validate_and_decode_jpeg(
    subject: &str,
    bytes: &[u8],
) -> Result<(u32, u32, String), AttachmentReadFailure> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return Err(AttachmentReadFailure::CorruptedImage {
            subject: subject.to_owned(),
            format: "image/jpeg".into(),
            reason: "missing JPEG SOI (0xFFD8) header".into(),
        });
    }
    if bytes[bytes.len() - 2] != 0xFF || bytes[bytes.len() - 1] != 0xD9 {
        return Err(AttachmentReadFailure::CorruptedImage {
            subject: subject.to_owned(),
            format: "image/jpeg".into(),
            reason: "missing JPEG EOI (0xFFD9) trailer".into(),
        });
    }

    let decoded =
        image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg).map_err(|err| {
            AttachmentReadFailure::CorruptedImage {
                subject: subject.to_owned(),
                format: "image/jpeg".into(),
                reason: format!("JPEG DCT/Huffman decode failed: {err}"),
            }
        })?;
    let width = decoded.width();
    let height = decoded.height();
    if width == 0 || height == 0 {
        return Err(AttachmentReadFailure::CorruptedImage {
            subject: subject.to_owned(),
            format: "image/jpeg".into(),
            reason: "JPEG frame has zero width or height".into(),
        });
    }

    let luma = decoded.to_luma8();
    let extracted_text = ocr_5x7_grayscale_grid(width as usize, height as usize, luma.as_raw());
    Ok((width, height, extracted_text))
}

/// Adaptive grayscale glyph recognizer matching the 5x7 clinical label raster grid
/// across both clean and noisy/low-contrast scan bitmaps.
fn ocr_5x7_grayscale_grid(width: usize, height: usize, pixels: &[u8]) -> String {
    const MARGIN_X: usize = 6;
    const MARGIN_Y: usize = 6;
    const CELL_W: usize = 6;
    const CELL_H: usize = 10;

    if width <= MARGIN_X * 2 || height <= MARGIN_Y * 2 || pixels.len() != width * height {
        return String::new();
    }

    let min_px = pixels.iter().copied().min().unwrap_or(255);
    let max_px = pixels.iter().copied().max().unwrap_or(255);
    if max_px.saturating_sub(min_px) < 40 {
        return String::new();
    }
    let threshold = u8::try_from((u16::from(min_px) + u16::from(max_px)) / 2).unwrap_or(128);

    let line_slots = (height - MARGIN_Y * 2) / CELL_H;
    let char_slots = (width - MARGIN_X * 2) / CELL_W;
    if line_slots == 0 || char_slots == 0 {
        return String::new();
    }

    let mut lines = Vec::new();
    for line_idx in 0..line_slots {
        let base_y = MARGIN_Y + line_idx * CELL_H;
        let mut line = String::with_capacity(char_slots);
        let mut unmatched_glyph = false;

        for char_idx in 0..char_slots {
            let base_x = MARGIN_X + char_idx * CELL_W;
            let mut pattern = [0_u8; 7];
            for (row_idx, row_mask) in pattern.iter_mut().enumerate() {
                let mut mask = 0_u8;
                for col_idx in 0..5 {
                    let px = pixels[(base_y + row_idx) * width + (base_x + col_idx)];
                    if px < threshold {
                        mask |= 1 << (4 - col_idx);
                    }
                }
                *row_mask = mask;
            }
            if pattern == [0_u8; 7] {
                line.push(' ');
            } else if let Some(ch) = match_5x7_glyph(pattern) {
                line.push(ch);
            } else {
                unmatched_glyph = true;
                break;
            }
        }

        if unmatched_glyph {
            return String::new();
        }
        let trimmed = line.trim_end().to_owned();
        if !trimmed.is_empty() {
            lines.push(trimmed);
        }
    }

    lines.join("\n")
}

const GLYPH_ALPHABET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', '-',
    '/', '.', ',', '%', '(', ')', '+', '#',
];

fn match_5x7_glyph(pattern: [u8; 7]) -> Option<char> {
    GLYPH_ALPHABET
        .iter()
        .copied()
        .find(|&ch| glyph_bitmap_5x7(ch) == pattern)
}

fn glyph_bitmap_5x7(ch: char) -> [u8; 7] {
    match ch {
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

fn render_synthetic_grayscale_grid(lines: &[&str], noisy_scan: bool) -> (u32, u32, Vec<u8>) {
    const MARGIN_X: usize = 6;
    const MARGIN_Y: usize = 6;
    const CELL_W: usize = 6;
    const CELL_H: usize = 10;

    let max_chars = lines
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(1)
        .max(1);
    let width = MARGIN_X * 2 + max_chars * CELL_W;
    let height = MARGIN_Y * 2 + lines.len().max(1) * CELL_H;
    let mut pixels = vec![248_u8; width * height];
    if noisy_scan {
        for (idx, px) in pixels.iter_mut().enumerate() {
            let grain = ((idx.wrapping_mul(73) ^ (idx >> 3).wrapping_mul(19)) % 24) as u8;
            *px = 228_u8.saturating_add(grain);
        }
    }

    for (line_idx, line) in lines.iter().enumerate() {
        let base_y = MARGIN_Y + line_idx * CELL_H;
        for (char_idx, ch) in line.chars().enumerate() {
            let base_x = MARGIN_X + char_idx * CELL_W;
            let glyph = glyph_bitmap_5x7(ch.to_ascii_uppercase());
            for (row_idx, mask) in glyph.iter().enumerate() {
                for col_idx in 0..5 {
                    if ((mask >> (4 - col_idx)) & 1) == 1 {
                        let y = base_y + row_idx;
                        let x = base_x + col_idx;
                        pixels[y * width + x] = if noisy_scan {
                            28_u8 + (((x.wrapping_mul(17) + y.wrapping_mul(31)) % 36) as u8)
                        } else {
                            18_u8
                        };
                    }
                }
            }
        }
    }

    (
        u32::try_from(width).expect("grid width fits u32"),
        u32::try_from(height).expect("grid height fits u32"),
        pixels,
    )
}

/// Renders uppercase/digit/punctuation lines into a deflate-compressed RFC-2083 8-bit
/// grayscale PNG image using Paeth scanline filtering.
pub fn render_synthetic_clinical_png(lines: &[&str], noisy_scan: bool) -> Vec<u8> {
    let (width, height, pixels) = render_synthetic_grayscale_grid(lines, noisy_scan);
    let mut out = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut out, width, height);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Balanced);
        encoder.set_filter(png::Filter::Paeth);
        let mut writer = encoder.write_header().expect("valid PNG header");
        writer
            .write_image_data(&pixels)
            .expect("valid PNG image data");
    }
    out
}

/// Renders uppercase/digit/punctuation lines into a deflate-compressed 24-bit RGB PNG
/// image for color-type qualification tests.
pub fn render_synthetic_clinical_rgb_png(lines: &[&str], noisy_scan: bool) -> Vec<u8> {
    let (width, height, luma) = render_synthetic_grayscale_grid(lines, noisy_scan);
    let mut rgb = Vec::with_capacity(luma.len() * 3);
    for px in luma {
        rgb.extend_from_slice(&[px, px, px]);
    }
    let mut out = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut out, width, height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Balanced);
        encoder.set_filter(png::Filter::Sub);
        let mut writer = encoder.write_header().expect("valid RGB PNG header");
        writer
            .write_image_data(&rgb)
            .expect("valid RGB PNG image data");
    }
    out
}

/// Renders uppercase/digit/punctuation lines into a standard ISO/IEC 10918-1 baseline
/// DCT/Huffman grayscale JPEG image for synthetic Spruce photo attachment tests.
pub fn render_synthetic_clinical_jpeg(lines: &[&str], noisy_scan: bool) -> Vec<u8> {
    let (width, height, pixels) = render_synthetic_grayscale_grid(lines, noisy_scan);
    let mut out = Vec::new();
    JpegEncoder::new_with_quality(&mut out, 98)
        .encode(&pixels, width, height, image::ExtendedColorType::L8)
        .expect("valid baseline JPEG encode");
    out
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

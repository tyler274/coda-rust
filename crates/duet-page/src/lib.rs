//! Faithful, measured US Letter doctor pages. Approval/currentness belongs to the
//! caller; this crate only renders the supplied snapshot, without clinical inference.
//!
//! Use [`layout_approved`] for new callers. It retains the complete text and an
//! extended preview even on overflow, while [`PageLayout::pdf`] refuses export.
//! The cover note is deliberately absent from [`ApprovedPage`]. Noto Sans Regular
//! and Bold are embedded by `oxifont-bundled` under SIL OFL 1.1. Additional
//! mathematical/script fallback fonts come from `typst-assets` (see its bundled
//! NOTICE for the OFL, GUST, and DejaVu font licenses). No host fonts or network
//! access are used at runtime. Unsupported text blocks export explicitly; this is
//! not a promise of coverage for every Unicode script or emoji sequence.
//!
//! ```
//! use duet_page::{ApprovedPage, layout_approved};
//! let snapshot = ApprovedPage {
//!     name: Some("Alex Rivera".into()),
//!     date_of_birth: Some("1984-06-18".into()),
//!     prepared_date: Some("Sep 21, 2026".into()),
//!     concern: Some("I want to discuss my fatigue.".into()),
//!     ..Default::default()
//! };
//! let layout = layout_approved(&snapshot);
//! let preview = layout.preview_svg();
//! let pdf = layout.pdf().expect("the full snapshot fits and all glyphs exist");
//! ```
#![deny(missing_docs)]

use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Weight, Wrap, fontdb};
use krilla::{
    Document,
    geom::Point,
    page::PageSettings,
    text::{Font, GlyphId, KrillaGlyph},
};
use std::{collections::HashMap, fmt::Write, sync::Arc};

/// US Letter width in PDF points.
pub const PAGE_WIDTH: f32 = 612.0;
/// US Letter height in PDF points.
pub const PAGE_HEIGHT: f32 = 792.0;
/// Page margin, matching the original doctor page.
pub const MARGIN: f32 = 36.0;
const SIZES: [f32; 3] = [10.5, 10.0, 9.5];
const FOOTER_GAP: f32 = 14.0;
/// Patient-review attribution printed on every page.
pub const ATTRIBUTION: &str = "Prepared by me with help from Duet, an AI app, and reviewed by me";
/// Fixed patient-prepared disclaimer and page count.
pub const DISCLAIMER: &str = "Patient-prepared, not a medical record · Page 1 of 1";

/// The complete approved snapshot. Strings are literal display values: the caller
/// owns date formatting and exact evidence projection. No trimming, normalization,
/// rounding, selection, or summarization is performed here.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApprovedPage {
    /// Patient-entered name; missing/empty names get an explicit placeholder.
    pub name: Option<String>,
    /// Calendar date of birth, never interpreted as a timestamp.
    pub date_of_birth: Option<String>,
    /// Display date of the approved save (not the export time); `None` means unsaved.
    pub prepared_date: Option<String>,
    /// Patient's approved concern / what they want to discuss.
    pub concern: Option<String>,
    /// Ordered questions. More than five blocks export, never drops questions.
    pub questions: Vec<String>,
    /// Selected exact evidence entries; input order is preserved within each group.
    pub entries: Vec<PageEntry>,
}

/// A source-separated section, in the reference renderer's canonical order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceGroup {
    /// Chosen record evidence.
    Records,
    /// Dated device evidence, kept separate from records.
    Devices,
    /// Medications explicitly confirmed by the patient.
    ConfirmedMedications,
    /// Other medication statements in records, not asserted to be current use.
    RecordedMedications,
}

impl EvidenceGroup {
    /// Literal heading used in both preview and PDF.
    pub fn heading(self) -> &'static str {
        match self {
            Self::Records => "From my records",
            Self::Devices => "From my devices",
            Self::ConfirmedMedications => "What I take (confirmed by me)",
            Self::RecordedMedications => "Other medications in my records",
        }
    }
}

/// An already-selected exact evidence projection. Include every literal field,
/// date role, status, and source label in `lines`; the renderer prints them all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageEntry {
    /// Source/confirmation group.
    pub group: EvidenceGroup,
    /// Literal source-owned title.
    pub title: String,
    /// Ordered literal fields, including source labels. Unknown fields belong here too.
    pub lines: Vec<String>,
}

/// A reason that the complete snapshot cannot be exported faithfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportBlocker {
    /// The full text exceeds the one-page area at the smallest permitted size.
    Overflow,
    /// All questions remain in the preview, but at most five may be exported.
    TooManyQuestions,
    /// A cluster is unsupported by the bundled fonts, or contains a non-layout
    /// control character. The literal cluster is retained for recovery/UI feedback.
    UnsupportedText(String),
    /// The PDF library could not serialize the document. No partial PDF is returned.
    PdfEncoding,
}

/// One shaped glyph used identically by the SVG preview and the PDF.
#[derive(Debug, Clone)]
pub struct PreviewGlyph {
    /// Index into [`PageLayout::font_data`].
    pub font_index: usize,
    /// Glyph index in the embedded font.
    pub glyph_id: u16,
    /// Absolute x position in points (including shaping offset).
    pub x: f32,
    /// Absolute baseline y, measured from the top (including shaping offset).
    pub y: f32,
    /// Font size in points.
    pub size: f32,
    glyph: KrillaGlyph,
    source: Arc<str>,
}

/// A measured visual line; coordinates use a top-left origin in points.
#[derive(Debug, Clone, PartialEq)]
pub struct PreviewLine {
    /// Literal text covered by the visual line, in logical reading order.
    pub text: String,
    /// Left edge of the content area.
    pub x: f32,
    /// Baseline measured from the top.
    pub y: f32,
    /// Measured advance width, not a character-count estimate.
    pub width: f32,
    /// Font size in points.
    pub size: f32,
    /// Whether this is a bold heading/identity/title.
    pub bold: bool,
}

#[derive(Clone)]
struct EmbeddedFont {
    bytes: Arc<Vec<u8>>,
    index: u32,
    pdf: Font,
}

/// An immutable layout of the whole approved snapshot. Layout is not permission
/// to publish: the caller must still enforce approval, ownership, and currentness.
pub struct PageLayout {
    text: String,
    size: f32,
    height: f32,
    lines: Vec<PreviewLine>,
    glyphs: Vec<PreviewGlyph>,
    fonts: Vec<EmbeddedFont>,
    blockers: Vec<ExportBlocker>,
}

impl PageLayout {
    /// Complete logical document text, including line breaks, whitespace, and footer.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// Chosen 10.5 / 10 / 9.5 point body size.
    pub fn font_size(&self) -> f32 {
        self.size
    }
    /// Preview height. Overflow extends it instead of clipping any content.
    pub fn preview_height(&self) -> f32 {
        self.height
    }
    /// Measured preview lines in document order, including the footer.
    pub fn lines(&self) -> &[PreviewLine] {
        &self.lines
    }
    /// The exact positioned glyphs drawn by preview and PDF.
    pub fn glyphs(&self) -> &[PreviewGlyph] {
        &self.glyphs
    }
    /// Font bytes and face index for a glyph's `font_index`.
    pub fn font_data(&self, index: usize) -> Option<(&[u8], u32)> {
        self.fonts.get(index).map(|f| (f.bytes.as_slice(), f.index))
    }
    /// All blockers; the caller may save the snapshot regardless of these values.
    pub fn blockers(&self) -> &[ExportBlocker] {
        &self.blockers
    }
    /// Whether a complete, one-page export is possible.
    pub fn fits(&self) -> bool {
        self.blockers.is_empty()
    }

    /// Emit one US Letter page from the exact preview glyphs. Never clips, elides,
    /// truncates, or emits a partially supported document.
    pub fn pdf(&self) -> Result<Vec<u8>, Vec<ExportBlocker>> {
        if !self.fits() {
            return Err(self.blockers.clone());
        }
        let mut document = Document::new();
        let mut page =
            document.start_page_with(PageSettings::from_wh(PAGE_WIDTH, PAGE_HEIGHT).unwrap());
        let mut surface = page.surface();
        let mut start = 0;
        while start < self.glyphs.len() {
            let first = &self.glyphs[start];
            let mut end = start + 1;
            while end < self.glyphs.len() {
                let next = &self.glyphs[end];
                if next.font_index != first.font_index
                    || next.size != first.size
                    || !Arc::ptr_eq(&next.source, &first.source)
                {
                    break;
                }
                end += 1;
            }
            // Keep clusters together for ToUnicode/ActualText (ligatures and
            // combining marks must not duplicate their source text on extraction).
            let mut pen = first.x;
            let glyphs: Vec<_> = self.glyphs[start..end]
                .iter()
                .map(|positioned| {
                    let mut glyph = positioned.glyph.clone();
                    glyph.x_offset = (positioned.x - pen) / first.size;
                    glyph.y_offset = (first.y - positioned.y) / first.size;
                    pen += glyph.x_advance * first.size;
                    glyph
                })
                .collect();
            surface.draw_glyphs(
                Point::from_xy(first.x, first.y),
                &glyphs,
                self.fonts[first.font_index].pdf.clone(),
                &first.source,
                first.size,
                false,
            );
            start = end;
        }
        surface.finish();
        page.finish();
        document
            .finish()
            .map_err(|_| vec![ExportBlocker::PdfEncoding])
    }

    /// Resolution-independent, white-background preview using the same positioned
    /// glyph outlines as the PDF. It extends vertically on overflow. Missing glyphs
    /// are explicitly marked in red; consult [`Self::blockers`] before offering export.
    /// The escaped complete logical text is included for accessibility.
    pub fn preview_svg(&self) -> String {
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{PAGE_WIDTH}\" height=\"{}\" viewBox=\"0 0 {PAGE_WIDTH} {}\" role=\"img\"><title>For my doctor</title><desc>{}</desc><rect width=\"100%\" height=\"100%\" fill=\"white\"/>",
            self.height,
            self.height,
            escape_xml(&self.text)
        );
        let faces: Vec<_> = self
            .fonts
            .iter()
            .map(|font| {
                ttf_parser::Face::parse(font.bytes.as_slice(), font.index)
                    .expect("validated bundled font")
            })
            .collect();
        for glyph in &self.glyphs {
            let face = &faces[glyph.font_index];
            let mut path = SvgPath(String::new());
            if glyph.glyph_id == 0 {
                let _ = write!(
                    svg,
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"red\"><title>Unsupported: {}</title></rect>",
                    glyph.x,
                    glyph.y - glyph.size,
                    glyph.size,
                    glyph.size,
                    escape_xml(&glyph.source[glyph.glyph.text_range.clone()])
                );
            } else if face
                .outline_glyph(ttf_parser::GlyphId(glyph.glyph_id), &mut path)
                .is_some()
            {
                let scale = glyph.size / f32::from(face.units_per_em());
                let _ = write!(
                    svg,
                    "<path fill=\"black\" transform=\"translate({} {}) scale({} {})\" d=\"{}\"/>",
                    glyph.x, glyph.y, scale, -scale, path.0
                );
            }
        }
        svg.push_str("</svg>");
        svg
    }
}

struct SvgPath(String);
impl ttf_parser::OutlineBuilder for SvgPath {
    fn move_to(&mut self, x: f32, y: f32) {
        let _ = write!(self.0, "M{x} {y}");
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let _ = write!(self.0, "L{x} {y}");
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let _ = write!(self.0, "Q{x1} {y1} {x} {y}");
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let _ = write!(self.0, "C{x1} {y1} {x2} {y2} {x} {y}");
    }
    fn close(&mut self) {
        self.0.push('Z');
    }
}

fn escape_xml(text: &str) -> String {
    let mut escaped = String::new();
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            '\t'
            | '\n'
            | '\r'
            | '\u{20}'..='\u{d7ff}'
            | '\u{e000}'..='\u{fffd}'
            | '\u{10000}'..='\u{10ffff}' => escaped.push(character),
            _ => {
                let _ = write!(escaped, "[U+{:04X}]", character as u32);
            }
        }
    }
    escaped
}

struct Block {
    text: String,
    size: f32,
    bold: bool,
    gap: f32,
}
fn block(text: impl Into<String>, size: f32, bold: bool, gap: f32) -> Block {
    Block {
        text: text.into(),
        size,
        bold,
        gap,
    }
}

fn document(page: &ApprovedPage, size: f32) -> (Vec<Block>, Vec<Block>) {
    let name = page
        .name
        .as_deref()
        .filter(|v| !v.is_empty())
        .unwrap_or("Name not provided");
    let dob = page.date_of_birth.as_deref().unwrap_or("Not provided");
    let identity = format!("{name} · DOB: {dob}");
    let prepared = page
        .prepared_date
        .as_deref()
        .map(|v| format!("Prepared {v}"))
        .unwrap_or_else(|| "Unsaved page".into());
    let mut body = vec![
        block("For my doctor", 18.0, true, 4.0),
        block(&identity, size, true, 3.0),
        block(&prepared, size, false, 9.0),
    ];
    if let Some(concern) = &page.concern
        && !concern.is_empty()
    {
        body.push(block("What I want to discuss", size, true, 3.0));
        body.push(block(concern, size, false, 7.0));
    }
    if !page.questions.is_empty() {
        body.push(block("My questions", size, true, 3.0));
        for (index, question) in page.questions.iter().enumerate() {
            body.push(block(
                format!("{}. {question}", index + 1),
                size,
                false,
                3.0,
            ));
        }
    }
    for group in [
        EvidenceGroup::Records,
        EvidenceGroup::Devices,
        EvidenceGroup::ConfirmedMedications,
        EvidenceGroup::RecordedMedications,
    ] {
        let entries: Vec<_> = page
            .entries
            .iter()
            .filter(|entry| entry.group == group)
            .collect();
        if entries.is_empty() {
            continue;
        }
        body.push(block(group.heading(), size, true, 4.0));
        for entry in entries {
            body.push(block(&entry.title, size, true, 1.0));
            body.push(block(entry.lines.join("\n"), size, false, 6.0));
        }
    }
    let footer = vec![
        block(format!("{identity} · {prepared}"), size, true, 2.0),
        block(ATTRIBUTION, size, false, 2.0),
        block(DISCLAIMER, size, false, 0.0),
    ];
    (body, footer)
}

struct Engine {
    system: FontSystem,
    font_indices: HashMap<fontdb::ID, usize>,
    fonts: Vec<EmbeddedFont>,
}
impl Engine {
    fn new() -> Self {
        let mut db = fontdb::Database::new();
        // Deterministic font coverage; never depend on the server or user's installed fonts.
        db.load_font_data(oxifont_bundled::NOTO_SANS_REGULAR.to_vec());
        db.load_font_data(oxifont_bundled::NOTO_SANS_BOLD.to_vec());
        // Typst's redistributable bundled fonts supply mathematical symbols and
        // additional scripts (licenses are distributed with that asset crate).
        for font in typst_assets::fonts() {
            db.load_font_data(font.to_vec());
        }
        db.set_sans_serif_family("Noto Sans");
        Self {
            system: FontSystem::new_with_locale_and_db("en-US".into(), db),
            font_indices: HashMap::new(),
            fonts: Vec::new(),
        }
    }

    fn shape(&mut self, blocks: &[Block], y_start: f32, result: &mut PageLayout) -> f32 {
        let mut y = y_start;
        for block in blocks {
            result.text.push_str(&block.text);
            result.text.push('\n');
            for character in block
                .text
                .chars()
                .filter(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
            {
                add_blocker(
                    &mut result.blockers,
                    ExportBlocker::UnsupportedText(character.to_string()),
                );
            }
            let mut buffer = Buffer::new(
                &mut self.system,
                Metrics::new(block.size, block.size * 1.35),
            );
            buffer.set_size(Some(PAGE_WIDTH - 2.0 * MARGIN), None);
            buffer.set_wrap(Wrap::WordOrGlyph);
            let attrs = Attrs::new()
                .family(Family::Name("Noto Sans"))
                .weight(if block.bold {
                    Weight::BOLD
                } else {
                    Weight::NORMAL
                });
            buffer.set_text(&block.text, &attrs, Shaping::Advanced, None);
            buffer.shape_until_scroll(&mut self.system, false);
            let mut height: f32 = 0.0;
            for run in buffer.layout_runs() {
                height = height.max(run.line_top + run.line_height);
                if run.line_w > PAGE_WIDTH - 2.0 * MARGIN + 0.01 {
                    add_blocker(&mut result.blockers, ExportBlocker::Overflow);
                }
                let start = run.glyphs.iter().map(|g| g.start).min().unwrap_or(0);
                let end = run.glyphs.iter().map(|g| g.end).max().unwrap_or(0);
                result.lines.push(PreviewLine {
                    text: run.text[start..end].into(),
                    x: MARGIN,
                    y: y + run.line_y,
                    width: run.line_w,
                    size: block.size,
                    bold: block.bold,
                });
                let source: Arc<str> = run.text.into();
                for glyph in run.glyphs {
                    if glyph.glyph_id == 0 {
                        add_blocker(
                            &mut result.blockers,
                            ExportBlocker::UnsupportedText(run.text[glyph.start..glyph.end].into()),
                        );
                    }
                    let font_index = *self.font_indices.entry(glyph.font_id).or_insert_with(|| {
                        let (bytes, index) = self
                            .system
                            .db()
                            .with_face_data(glyph.font_id, |data, index| {
                                (Arc::new(data.to_vec()), index)
                            })
                            .expect("shaped font exists");
                        let pdf = Font::new(bytes.clone().into(), index)
                            .expect("bundled font supported by PDF encoder");
                        let font_index = self.fonts.len();
                        self.fonts.push(EmbeddedFont { bytes, index, pdf });
                        font_index
                    });
                    result.glyphs.push(PreviewGlyph {
                        font_index,
                        glyph_id: glyph.glyph_id,
                        x: MARGIN + glyph.x + glyph.x_offset * glyph.font_size,
                        y: y + run.line_y + glyph.y - glyph.y_offset * glyph.font_size,
                        size: glyph.font_size,
                        glyph: KrillaGlyph::new(
                            GlyphId::new(u32::from(glyph.glyph_id)),
                            glyph.w / glyph.font_size,
                            0.0,
                            0.0,
                            0.0,
                            glyph.start..glyph.end,
                            None,
                        ),
                        source: source.clone(),
                    });
                }
            }
            y += height + block.gap;
        }
        y
    }
}
fn add_blocker(blockers: &mut Vec<ExportBlocker>, blocker: ExportBlocker) {
    if !blockers.contains(&blocker) {
        blockers.push(blocker);
    }
}

/// Shape all approved content at the reference size steps. Never changes the
/// input; overflow remains fully available for saving and extended preview.
pub fn layout_approved(page: &ApprovedPage) -> PageLayout {
    let mut engine = Engine::new();
    for size in SIZES {
        let (body, footer) = document(page, size);
        let mut result = PageLayout {
            text: String::new(),
            size,
            height: PAGE_HEIGHT,
            lines: Vec::new(),
            glyphs: Vec::new(),
            fonts: Vec::new(),
            blockers: Vec::new(),
        };
        if page.questions.len() > 5 {
            result.blockers.push(ExportBlocker::TooManyQuestions);
        }
        let body_end = engine.shape(&body, MARGIN, &mut result);
        let footer_start_line = result.lines.len();
        let footer_start_glyph = result.glyphs.len();
        let footer_height = engine.shape(&footer, 0.0, &mut result);
        let required = body_end + FOOTER_GAP + footer_height + MARGIN;
        if required > PAGE_HEIGHT {
            add_blocker(&mut result.blockers, ExportBlocker::Overflow);
        }
        result.height = PAGE_HEIGHT.max(required);
        let footer_y = result.height - MARGIN - footer_height;
        for line in &mut result.lines[footer_start_line..] {
            line.y += footer_y;
        }
        for glyph in &mut result.glyphs[footer_start_glyph..] {
            glyph.y += footer_y;
        }
        // Advances alone do not bound ink (e.g. italic overhangs or stacked
        // combining accents). Refuse export if an outline leaves the page area.
        for glyph in &result.glyphs {
            let font = &engine.fonts[glyph.font_index];
            let face = ttf_parser::Face::parse(font.bytes.as_slice(), font.index)
                .expect("validated bundled font");
            if let Some(bounds) = face.glyph_bounding_box(ttf_parser::GlyphId(glyph.glyph_id)) {
                let scale = glyph.size / f32::from(face.units_per_em());
                if glyph.x + f32::from(bounds.x_min) * scale < MARGIN - 0.01
                    || glyph.x + f32::from(bounds.x_max) * scale > PAGE_WIDTH - MARGIN + 0.01
                    || glyph.y - f32::from(bounds.y_max) * scale < MARGIN - 0.01
                    || glyph.y - f32::from(bounds.y_min) * scale > result.height - MARGIN + 0.01
                {
                    add_blocker(&mut result.blockers, ExportBlocker::Overflow);
                }
            }
        }
        if !result.blockers.contains(&ExportBlocker::Overflow) || size == SIZES[2] {
            result.fonts = engine.fonts;
            return result;
        }
    }
    unreachable!("size sequence is nonempty")
}

/// Legacy, questions-only input. New integrations should use [`ApprovedPage`].
#[derive(Debug, Clone, PartialEq)]
pub struct DoctorPage {
    /// Patient name.
    pub name: String,
    /// Patient's calendar date of birth.
    pub date_of_birth: String,
    /// Ordered questions.
    pub questions: Vec<String>,
    /// Accompanies the document separately and is never included in PDF/preview.
    pub cover_note: Option<String>,
}
/// Legacy preview line; `y` retains the old bottom-left PDF coordinate convention.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedLine {
    /// Logical text.
    pub text: String,
    /// Font size in points.
    pub size: f32,
    /// Baseline measured from the bottom.
    pub y: f32,
}
/// Legacy result. Use [`PageLayout`] for detailed blockers and overflow preview.
#[derive(Debug, Clone, PartialEq)]
pub enum LaidOut {
    /// Complete, exportable page.
    Page {
        /// Preview text lines; for exact shaped preview use [`PageLayout::preview_svg`].
        lines: Vec<PlacedLine>,
        /// Complete PDF bytes.
        pdf: Vec<u8>,
    },
    /// Export is blocked (overflow, unsupported glyphs, or invalid question count).
    Overflow,
}
/// Compatibility adapter. Excludes the cover note, including for legacy callers.
pub fn lay_out(page: &DoctorPage) -> LaidOut {
    let layout = layout_approved(&ApprovedPage {
        name: Some(page.name.clone()),
        date_of_birth: Some(page.date_of_birth.clone()),
        questions: page.questions.clone(),
        ..Default::default()
    });
    match layout.pdf() {
        Ok(pdf) => LaidOut::Page {
            lines: layout
                .lines
                .iter()
                .map(|line| PlacedLine {
                    text: line.text.clone(),
                    size: line.size,
                    y: PAGE_HEIGHT - line.y,
                })
                .collect(),
            pdf,
        },
        Err(_) => LaidOut::Overflow,
    }
}

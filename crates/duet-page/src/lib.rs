//! One US Letter layout for the doctor page. Preview lines and the PDF are the
//! same placement. Overflow blocks sharing.
//! Book: `book/src/north-star.md`.
#![deny(missing_docs)]

/// The `PAGE_WIDTH` constant.
pub const PAGE_WIDTH: f32 = 612.0;
/// The `PAGE_HEIGHT` constant.
pub const PAGE_HEIGHT: f32 = 792.0;
const MARGIN: f32 = 54.0;
const SIZES: [f32; 3] = [10.5, 10.0, 9.5];

#[derive(Debug, Clone, PartialEq)]
/// The `DoctorPage` value exchanged by this crate.
pub struct DoctorPage {
    /// The `name` value.
    pub name: String,
    /// The `date_of_birth` value.
    pub date_of_birth: String,
    /// The `questions` value.
    pub questions: Vec<String>,
    /// The `cover_note` value.
    pub cover_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
/// The `PlacedLine` value exchanged by this crate.
pub struct PlacedLine {
    /// The `text` value.
    pub text: String,
    /// The `size` value.
    pub size: f32,
    /// The `y` value.
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
/// The `LaidOut` cases.
pub enum LaidOut {
    /// The `Page` case.
    Page {
        /// The `lines` value.
        lines: Vec<PlacedLine>,
        /// The `pdf` value.
        pdf: Vec<u8>,
    },
    /// The `Overflow` case.
    Overflow,
}

/// Places the doctor page, or blocks it when the page overflows.
pub fn lay_out(page: &DoctorPage) -> LaidOut {
    if page.questions.len() > 5 {
        return LaidOut::Overflow;
    }
    for size in SIZES {
        if let Some(lines) = place(page, size) {
            let pdf = render_pdf(&lines);
            return LaidOut::Page { lines, pdf };
        }
    }
    LaidOut::Overflow
}

fn place(page: &DoctorPage, size: f32) -> Option<Vec<PlacedLine>> {
    let mut blocks = vec![page.name.clone(), format!("Born {}", page.date_of_birth)];
    for (index, question) in page.questions.iter().enumerate() {
        blocks.push(format!("{}. {question}", index + 1));
    }
    if let Some(note) = &page.cover_note {
        blocks.push(note.clone());
    }
    let width = PAGE_WIDTH - 2.0 * MARGIN;
    let leading = size * 1.35;
    let mut y = PAGE_HEIGHT - MARGIN;
    let mut lines = Vec::new();
    for block in blocks {
        for line in wrap(&block, size, width) {
            y -= leading;
            if y < MARGIN {
                return None;
            }
            lines.push(PlacedLine {
                text: line,
                size,
                y,
            });
        }
        y -= leading * 0.35;
    }
    Some(lines)
}

fn wrap(text: &str, size: f32, width: f32) -> Vec<String> {
    let max_chars = (width / (size * 0.5)).floor().max(1.0) as usize;
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let next = if current.is_empty() {
            word.to_owned()
        } else {
            format!("{current} {word}")
        };
        if next.chars().count() > max_chars && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
            current = word.to_owned();
        } else {
            current = next;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}

fn render_pdf(lines: &[PlacedLine]) -> Vec<u8> {
    let mut stream = String::from("BT\n");
    for line in lines {
        let text = pdf_escape(&line.text);
        stream.push_str(&format!(
            "/F1 {} Tf\n1 0 0 1 {} {} Tm\n({}) Tj\n",
            line.size, MARGIN, line.y, text
        ));
    }
    stream.push_str("ET\n");
    let stream_bytes = stream.into_bytes();
    let objects = [
        b"1 0 obj<< /Type /Catalog /Pages 2 0 R >>endobj\n".to_vec(),
        b"2 0 obj<< /Type /Pages /Count 1 /Kids [3 0 R] >>endobj\n".to_vec(),
        format!("3 0 obj<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {PAGE_WIDTH} {PAGE_HEIGHT}] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>endobj\n").into_bytes(),
        format!("4 0 obj<< /Length {} >>stream\n", stream_bytes.len()).into_bytes().into_iter().chain(stream_bytes).chain(b"\nendstream\nendobj\n".iter().copied()).collect(),
        b"5 0 obj<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>endobj\n".to_vec(),
    ];
    let mut pdf = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::new();
    for object in objects {
        offsets.push(pdf.len());
        pdf.extend(object);
    }
    let xref = pdf.len();
    pdf.extend(format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1).into_bytes());
    for offset in &offsets {
        pdf.extend(format!("{offset:010} 00000 n \n").into_bytes());
    }
    pdf.extend(
        format!(
            "trailer<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            offsets.len() + 1
        )
        .into_bytes(),
    );
    pdf
}

fn pdf_escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_short_page_fits_and_exports_pdf() {
        let laid = lay_out(&DoctorPage {
            name: "Ada Lovelace".into(),
            date_of_birth: "1815-12-10".into(),
            questions: vec!["What changed?".into()],
            cover_note: Some("Please review.".into()),
        });
        match laid {
            LaidOut::Page { lines, pdf } => {
                assert!(lines.iter().any(|line| line.text.contains("Ada Lovelace")));
                assert!(pdf.starts_with(b"%PDF-1.4"));
                assert!(pdf.windows(4).any(|window| window == b"Ada "));
            }
            LaidOut::Overflow => panic!("short page overflowed"),
        }
    }

    #[test]
    fn more_than_five_questions_block_sharing() {
        let laid = lay_out(&DoctorPage {
            name: "Ada".into(),
            date_of_birth: "1815-12-10".into(),
            questions: (0..6).map(|n| format!("Question {n}")).collect(),
            cover_note: None,
        });
        assert_eq!(laid, LaidOut::Overflow);
    }
}

use duet_page::{ApprovedPage, EvidenceGroup, ExportBlocker, PageEntry, layout_approved};
use std::{fs, process::Command};

fn fixture() -> ApprovedPage {
    ApprovedPage {
        name: Some("Alex Rivera".into()),
        date_of_birth: Some("1984-06-18".into()),
        prepared_date: Some("Sep 21, 2026".into()),
        concern: Some("I want to discuss how I feel in the afternoons.".into()),
        questions: vec!["Could we discuss my fatigue?".into()],
        entries: vec![PageEntry {
            group: EvidenceGroup::Records,
            title: "Hemoglobin".into(),
            lines: vec![
                "<13.456 g/dL".into(),
                "Collected: 2026-09-21".into(),
                "Source: Example Hospital".into(),
            ],
        }],
    }
}

#[test]
fn full_snapshot_and_reference_geometry() {
    let layout = layout_approved(&fixture());
    assert!(layout.fits(), "{:?}", layout.blockers());
    assert_eq!(layout.font_size(), 10.5);
    assert_eq!(layout.preview_height(), 792.0);
    assert_eq!(layout.text().matches("Alex Rivera").count(), 2);
    for text in [
        "For my doctor",
        "1984-06-18",
        "Prepared Sep 21, 2026",
        "What I want to discuss",
        "My questions",
        "From my records",
        "<13.456 g/dL",
        "Source: Example Hospital",
        duet_page::ATTRIBUTION,
        duet_page::DISCLAIMER,
    ] {
        assert!(layout.text().contains(text), "missing {text}");
    }
    assert!(
        layout
            .lines()
            .iter()
            .all(|line| line.x >= 36.0 && line.y < 756.0 && line.width <= 540.01)
    );
    assert!(layout.pdf().unwrap().starts_with(b"%PDF-"));
}

#[test]
fn overflow_retains_all_text_and_never_exports() {
    let mut page = fixture();
    let words = (1..=90)
        .map(|n| format!("Line {n}: these are my words and must remain intact."))
        .collect::<Vec<_>>()
        .join("\n");
    page.concern = Some(words.clone());
    let layout = layout_approved(&page);
    assert_eq!(page.concern.as_ref().unwrap(), &words);
    assert!(layout.text().contains(&words));
    assert_eq!(layout.font_size(), 9.5);
    assert!(layout.preview_height() > 792.0);
    assert!(layout.blockers().contains(&ExportBlocker::Overflow));
    assert!(layout.pdf().is_err());
    assert!(layout.preview_svg().contains("Line 90:"));
    assert!(
        layout
            .lines()
            .iter()
            .any(|line| line.text.contains("Line 90:"))
    );
}

#[test]
fn groups_and_literal_unknown_fields_are_preserved_in_order() {
    let mut page = fixture();
    page.entries.insert(
        0,
        PageEntry {
            group: EvidenceGroup::RecordedMedications,
            title: "Prior prescription".into(),
            lines: vec![
                "Status: stopped".into(),
                "Unknown: {\"literal\":true}".into(),
            ],
        },
    );
    page.entries.push(PageEntry {
        group: EvidenceGroup::Devices,
        title: "Dated snapshot".into(),
        lines: vec!["2026-09-20: 53.250 ms".into(), "Source: My watch".into()],
    });
    page.entries.push(PageEntry {
        group: EvidenceGroup::ConfirmedMedications,
        title: "What I take".into(),
        lines: vec![
            "Dose: 12.50 mg; Route: oral; Status: current".into(),
            "Confirmed: 2026-09-21".into(),
        ],
    });
    let layout = layout_approved(&page);
    for entry in &page.entries {
        assert!(layout.text().contains(&entry.title));
        for line in &entry.lines {
            assert!(layout.text().contains(line));
        }
    }
    let mut previous = 0;
    for group in [
        EvidenceGroup::Records,
        EvidenceGroup::Devices,
        EvidenceGroup::ConfirmedMedications,
        EvidenceGroup::RecordedMedications,
    ] {
        let index = layout.text().find(group.heading()).unwrap();
        assert!(index > previous);
        previous = index;
    }
}

#[test]
fn question_limit_does_not_drop_sixth_question() {
    let mut page = fixture();
    page.questions = (1..=6).map(|n| format!("Question {n}")).collect();
    let layout = layout_approved(&page);
    assert!(layout.text().contains("6. Question 6"));
    assert!(layout.blockers().contains(&ExportBlocker::TooManyQuestions));
    assert!(layout.pdf().is_err());
}

#[test]
fn unsupported_unicode_is_explicit_and_preserved() {
    let mut page = fixture();
    page.concern = Some("Keep my words: 🫶🏽 中文 \u{10ffff}".into());
    let layout = layout_approved(&page);
    assert!(layout.text().contains(page.concern.as_ref().unwrap()));
    assert!(
        layout
            .blockers()
            .iter()
            .any(|b| matches!(b, ExportBlocker::UnsupportedText(_)))
    );
    assert!(layout.pdf().is_err());
    assert!(layout.preview_svg().contains("Unsupported:"));
}

#[test]
fn whitespace_and_long_unbroken_text_are_not_elided() {
    let mut page = fixture();
    page.concern = Some(format!(
        "  Literal  spacing\tand tabs\n\n{}",
        "W".repeat(180)
    ));
    let layout = layout_approved(&page);
    assert!(layout.text().contains(page.concern.as_ref().unwrap()));
    assert!(layout.lines().iter().all(|line| line.width <= 540.01));
    let rendered: String = layout
        .lines()
        .iter()
        .map(|line| line.text.as_str())
        .collect();
    assert_eq!(rendered.matches('W').count(), 181); // the concern heading includes W
}

#[test]
fn widths_are_shaped_not_character_counts() {
    let mut wide = fixture();
    wide.concern = Some("W".repeat(100));
    let wide = layout_approved(&wide);
    let mut narrow = fixture();
    narrow.concern = Some("i".repeat(100));
    let narrow = layout_approved(&narrow);
    let wide_lines = wide
        .lines()
        .iter()
        .filter(|l| l.text.starts_with("WWW"))
        .count();
    let narrow_lines = narrow
        .lines()
        .iter()
        .filter(|l| l.text.starts_with("iii"))
        .count();
    assert!(wide_lines > narrow_lines);
}

#[test]
fn reference_size_steps_are_used_before_overflow() {
    let mut sizes = std::collections::BTreeSet::new();
    for count in 18..42 {
        let mut page = fixture();
        page.concern = Some(
            (0..count)
                .map(|n| format!("Line {n}: measured words."))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        let layout = layout_approved(&page);
        if layout.fits() {
            sizes.insert((layout.font_size() * 10.0) as u32);
        }
    }
    assert_eq!(sizes, [95, 100, 105].into_iter().collect());
}

#[test]
fn control_characters_are_not_silently_discarded() {
    let mut page = fixture();
    page.concern = Some("Before\0After".into());
    let layout = layout_approved(&page);
    assert!(layout.text().contains("Before\0After"));
    assert!(
        layout
            .blockers()
            .contains(&ExportBlocker::UnsupportedText("\0".into()))
    );
    assert!(layout.pdf().is_err());
}

// Independent parser and rasterizer proof is deliberately mandatory, not skipped
// when Poppler is unavailable. Run inside the repository's Nix development shell.
#[test]
fn independent_pdf_extraction_render_and_preview_parity() {
    let mut page = fixture();
    page.name = Some("Zoë Muñoz — Мария".into());
    page.concern = Some("Café, naïve, e\u{301}, μg/mL, ≤0.125, ≥2.0, Ελληνικά.".into());
    for (group, title, field) in [
        (
            EvidenceGroup::Devices,
            "Watch snapshot",
            "Measured: 2026-09-20; 53.250 ms",
        ),
        (
            EvidenceGroup::ConfirmedMedications,
            "Confirmed medicine",
            "Dose: 12.50 mg; Route: oral; Source: patient",
        ),
        (
            EvidenceGroup::RecordedMedications,
            "Recorded medicine",
            "Status: stopped; Unknown field: {literal=true}",
        ),
    ] {
        page.entries.push(PageEntry {
            group,
            title: title.into(),
            lines: vec![field.into()],
        });
    }
    let layout = layout_approved(&page);
    assert!(layout.fits(), "{:?}", layout.blockers());
    let directory = std::env::temp_dir().join(format!("duet-page-renderer-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let pdf_path = directory.join("page.pdf");
    fs::write(&pdf_path, layout.pdf().unwrap()).unwrap();
    let text = Command::new("pdftotext")
        .args(["-enc", "UTF-8", "-layout"])
        .arg(&pdf_path)
        .arg("-")
        .output()
        .expect("Nix shell must provide pdftotext");
    assert!(
        text.status.success(),
        "{}",
        String::from_utf8_lossy(&text.stderr)
    );
    let text = String::from_utf8(text.stdout).unwrap();
    for exact in [
        "Zoë Muñoz — Мария",
        "1984-06-18",
        "<13.456 g/dL",
        "Collected: 2026-09-21",
        "Source: Example Hospital",
        "Café",
        "naïve",
        "μg/mL",
        "≤0.125",
        "≥2.0",
        "Ελληνικά",
        duet_page::ATTRIBUTION,
        duet_page::DISCLAIMER,
    ] {
        assert!(
            text.contains(exact),
            "Missing {exact:?} in extracted PDF:\n{text}"
        );
    }
    for entry in &page.entries {
        for literal in std::iter::once(&entry.title).chain(&entry.lines) {
            assert!(
                text.contains(literal),
                "Missing exact evidence field {literal:?}"
            );
        }
        assert!(text.contains(entry.group.heading()));
    }
    // Combining sequences are preserved without NFC normalization in ToUnicode.
    assert!(
        text.contains("e\u{301}"),
        "combining source sequence changed: {text}"
    );
    assert_eq!(text.matches("Zoë Muñoz — Мария").count(), 2);
    let info = Command::new("pdfinfo")
        .arg(&pdf_path)
        .output()
        .expect("Nix shell must provide pdfinfo");
    assert!(info.status.success());
    let info = String::from_utf8(info.stdout).unwrap();
    assert!(
        info.lines()
            .any(|l| l.starts_with("Pages:") && l.trim_end().ends_with('1'))
    );
    assert!(info.contains("612 x 792 pts (letter)"), "{info}");
    let render = Command::new("pdftoppm")
        .args(["-r", "72", "-singlefile"])
        .arg(&pdf_path)
        .arg(directory.join("page"))
        .output()
        .expect("Nix shell must provide pdftoppm");
    assert!(
        render.status.success(),
        "{}",
        String::from_utf8_lossy(&render.stderr)
    );
    let ppm = fs::read(directory.join("page.ppm")).unwrap();
    assert!(ppm.starts_with(b"P6\n612 792\n255\n"));
    assert!(
        ppm.iter().filter(|&&byte| byte < 128).count() > 1000,
        "PDF raster must not be blank"
    );
    let svg = layout.preview_svg();
    assert!(svg.contains("height=\"792\""));
    assert!(
        !svg.contains("<text"),
        "preview must use shaped outlines, not browser reflow"
    );
    assert_eq!(
        svg.matches("<path ").count(),
        layout
            .glyphs()
            .iter()
            .filter(|g| {
                let (data, index) = layout.font_data(g.font_index).unwrap();
                ttf_parser::Face::parse(data, index)
                    .unwrap()
                    .glyph_bounding_box(ttf_parser::GlyphId(g.glyph_id))
                    .is_some()
            })
            .count()
    );
    // A second renderer rasterizes the SVG independently. Compare ink occupancy
    // with Poppler allowing a two-pixel antialiasing/hinting neighborhood.
    let tree = resvg::usvg::Tree::from_str(&svg, &Default::default()).unwrap();
    let mut preview = resvg::tiny_skia::Pixmap::new(612, 792).unwrap();
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut preview.as_mut(),
    );
    let pdf_pixels = &ppm[b"P6\n612 792\n255\n".len()..];
    let pdf_ink: Vec<_> = pdf_pixels
        .as_chunks::<3>()
        .0
        .iter()
        .map(|rgb| rgb[0] < 180)
        .collect();
    let preview_ink: Vec<_> = preview
        .data()
        .as_chunks::<4>()
        .0
        .iter()
        .map(|rgba| rgba[0] < 180)
        .collect();
    for (source, target) in [(&pdf_ink, &preview_ink), (&preview_ink, &pdf_ink)] {
        let mut total = 0;
        let mut matches = 0;
        for y in 2..790usize {
            for x in 2..610usize {
                if source[y * 612 + x] {
                    total += 1;
                    if (y - 2..=y + 2).any(|yy| (x - 2..=x + 2).any(|xx| target[yy * 612 + xx])) {
                        matches += 1;
                    }
                }
            }
        }
        assert!(total > 1000);
        assert!(
            matches as f64 / total as f64 > 0.99,
            "preview/PDF ink mismatch: {matches}/{total}"
        );
    }
    fs::remove_dir_all(directory).unwrap();
}

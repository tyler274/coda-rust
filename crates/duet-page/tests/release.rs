use duet_page::{
    ExportBlocker, MARGIN, PAGE_HEIGHT, PAGE_WIDTH, PageLayout, ReleaseAuthorization,
    ReleaseExpiration, ReleaseParty, ReleasePatient, ReleaseRecipient, ReleaseScope,
    SensitiveDisclosure, layout_release,
};
use std::{fs, path::PathBuf, process::Command};

fn fixture() -> ReleaseAuthorization {
    ReleaseAuthorization {
        patient: ReleasePatient {
            name: "Alex Rivera".into(),
            date_of_birth: "1984-06-18".into(),
            address: "101 Patient Lane, Example City, CA 90001".into(),
            phone: "555-0101".into(),
        },
        discloser: ReleaseParty {
            name: "Example Clinic".into(),
            npi: Some("1234567893".into()),
            address: "202 Clinic Road, Example City, CA 90002".into(),
            phone: "555-0102".into(),
            fax: "555-0103".into(),
        },
        recipient: ReleaseRecipient::CareTeamMember(ReleaseParty {
            name: "Recipient Clinic".into(),
            npi: Some("1245319599".into()),
            address: "303 Recipient Street, Example City, CA 90003".into(),
            phone: "555-0104".into(),
            fax: "555-0105".into(),
        }),
        scope: ReleaseScope {
            full_record: true,
            date_range: Some(("2025-01-01".into(), "2026-10-22".into())),
            visit_notes: true,
            labs: true,
            imaging: true,
            medications: true,
            mental_health: SensitiveDisclosure {
                requested: true,
                initials: "AR".into(),
            },
            substance_use: SensitiveDisclosure {
                requested: true,
                initials: "AR".into(),
            },
            hiv_sti: SensitiveDisclosure {
                requested: true,
                initials: "AR".into(),
            },
            genetic: SensitiveDisclosure {
                requested: true,
                initials: "AR".into(),
            },
        },
        expiration: Some(ReleaseExpiration::Date("2027-01-01".into())),
        signed_name: Some("Alex Rivera".into()),
        signed_date: Some("2026-10-22".into()),
        ..Default::default()
    }
}

fn extracted(layout: &PageLayout, suffix: &str) -> String {
    let directory =
        std::env::temp_dir().join(format!("duet-release-{}-{suffix}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let pdf_path = directory.join("release.pdf");
    assert!(layout.fits(), "{:?}", layout.blockers());
    fs::write(&pdf_path, layout.pdf().unwrap()).unwrap();
    let output = Command::new("pdftotext")
        .args(["-enc", "UTF-8", "-layout"])
        .arg(&pdf_path)
        .arg("-")
        .output()
        .expect("Nix shell must provide pdftotext");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let info = Command::new("pdfinfo")
        .arg(&pdf_path)
        .output()
        .expect("Nix shell must provide pdfinfo");
    assert!(info.status.success());
    let info = String::from_utf8(info.stdout).unwrap();
    let pages = info
        .lines()
        .find(|line| line.starts_with("Pages:"))
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert_eq!(pages, layout.page_count());
    assert!(info.contains("612 x 792 pts (letter)"));
    fs::remove_dir_all(directory).unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn pdf_contains_elements_statements_and_safety_limits() {
    let layout = layout_release(fixture());
    let text = extracted(&layout, "elements")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for expected in [
        "Authorization to release health information",
        "Alex Rivera",
        "1984-06-18",
        "101 Patient Lane",
        "555-0101",
        "Who may disclose",
        "Example Clinic",
        "1234567893",
        "202 Clinic Road",
        "555-0102",
        "555-0103",
        "Who may receive",
        "Care-team member",
        "Recipient Clinic",
        "1245319599",
        "303 Recipient Street",
        "555-0104",
        "555-0105",
        "[X] Full record",
        "[X] Date range: 2025-01-01 through 2026-10-22",
        "[X] Visit notes",
        "[X] Labs",
        "[X] Imaging",
        "[X] Medications",
        "[X] Mental health - Initials: AR",
        "[X] Substance use (42 CFR Part 2) - Initials: AR",
        "[X] HIV/STI - Initials: AR",
        "[X] Genetic - Initials: AR",
        "Psychotherapy notes are excluded",
        "require a separate authorization",
        "At the request of the individual",
        "Expiration date: 2027-01-01",
        "Right to revoke",
        "in writing",
        "action already taken in reliance",
        "No conditioning",
        "Treatment, payment, enrollment or eligibility for benefits",
        "Potential for redisclosure",
        "may no longer be protected by HIPAA",
        "Right to a copy",
        "Typed name: Alex Rivera",
        "Date supplied: 2026-10-22",
        "Signature:",
        "not a guarantee of legal compliance or clinic acceptance",
        "Sensitive categories require separate review before disclosure",
        "Initials alone do not satisfy 42 CFR Part 2 or applicable state law",
        "A typed name alone does not prove a valid electronic signature",
    ] {
        assert!(text.contains(expected), "Missing {expected:?}:\n{text}");
    }
}

#[test]
fn overflow_paginates_without_losing_lines_or_overlapping_margins() {
    let mut auth = fixture();
    let words = (0..160)
        .map(|n| format!("Exact patient purpose line {n:03}."))
        .collect::<Vec<_>>()
        .join("\n");
    auth.purpose = Some(words.clone());
    let layout = layout_release(auth);
    assert!(layout.page_count() >= 5);
    assert!(layout.text().contains(&words));
    assert_eq!(
        layout.preview_height(),
        layout.page_count() as f32 * PAGE_HEIGHT
    );
    let text = extracted(&layout, "overflow");
    for n in 0..160 {
        assert_eq!(
            text.matches(&format!("Exact patient purpose line {n:03}."))
                .count(),
            1
        );
    }
    for line in layout.lines() {
        assert!(line.x >= MARGIN && line.width <= PAGE_WIDTH - 2.0 * MARGIN + 0.01);
        let y = line.y % PAGE_HEIGHT;
        assert!((MARGIN..=PAGE_HEIGHT - MARGIN).contains(&y), "{line:?}");
    }
    // Check actual outlines as well as line advances, on every physical page.
    for glyph in layout.glyphs() {
        let (bytes, index) = layout.font_data(glyph.font_index).unwrap();
        let face = ttf_parser::Face::parse(bytes, index).unwrap();
        if let Some(bounds) = face.glyph_bounding_box(ttf_parser::GlyphId(glyph.glyph_id)) {
            let scale = glyph.size / f32::from(face.units_per_em());
            let y = glyph.y % PAGE_HEIGHT;
            assert!(y - f32::from(bounds.y_max) * scale >= MARGIN - 0.01);
            assert!(y - f32::from(bounds.y_min) * scale <= PAGE_HEIGHT - MARGIN + 0.01);
        }
    }
}

#[test]
fn recipients_expiration_and_incomplete_drafts_remain_explicit() {
    let draft = layout_release(ReleaseAuthorization::default());
    assert!(draft.text().contains("The patient themself"));
    assert!(draft.text().contains("Typed name: Not provided"));
    assert!(draft.text().contains("Not provided - enter an expiration"));
    assert!(draft.text().contains("[ ] Full record"));
    assert!(
        draft
            .text()
            .contains("[ ] Substance use (42 CFR Part 2) - Initials: Not provided")
    );
    let mut auth = fixture();
    auth.expiration = Some(ReleaseExpiration::Event(
        "Completion of this referral".into(),
    ));
    auth.purpose = Some("Second opinion".into());
    auth.recipient = ReleaseRecipient::Other(ReleaseParty {
        name: "A < B & C".into(),
        ..Default::default()
    });
    auth.representative_authority = Some("Court-appointed guardian".into());
    let layout = layout_release(auth);
    let text = extracted(&layout, "other");
    for expected in [
        "Other recipient",
        "A < B & C",
        "Second opinion",
        "Expiration event: Completion of this referral",
        "Court-appointed guardian",
    ] {
        assert!(text.contains(expected));
    }
    assert!(layout.preview_svg().contains("A &lt; B &amp; C"));
}

#[test]
fn unsupported_text_blocks_entire_release_export_without_dropping_content() {
    let mut auth = fixture();
    auth.patient.name = "Before\0After\u{10ffff}".into();
    let layout = layout_release(auth);
    assert!(layout.text().contains("Before\0After\u{10ffff}"));
    assert!(
        layout
            .blockers()
            .iter()
            .any(|b| matches!(b, ExportBlocker::UnsupportedText(_)))
    );
    assert!(layout.pdf().is_err());
    assert!(layout.preview_svg().contains("[U+0000]"));
}

#[test]
fn long_wrapped_unicode_fields_survive_page_boundaries() {
    let mut auth = fixture();
    auth.patient.name = "Zoë Muñoz — Мария".into();
    auth.purpose = Some(format!(
        "{}\n{}",
        "Café e\u{301} Ελληνικά. ".repeat(500),
        "W".repeat(1200)
    ));
    let layout = layout_release(auth);
    let text = extracted(&layout, "unicode-wrap");
    assert!(layout.page_count() > 3);
    assert!(text.contains("Zoë Muñoz — Мария"));
    assert_eq!(text.matches("Café").count(), 500);
    assert_eq!(text.matches("e\u{301}").count(), 500);
    assert_eq!(text.matches("Ελληνικά.").count(), 500);
    let rendered = layout
        .lines()
        .iter()
        .filter(|line| line.text.starts_with('W'))
        .map(|line| line.text.matches('W').count())
        .sum::<usize>();
    // The two "Who may" headings also start with W.
    assert_eq!(rendered, 1202);
}

#[test]
fn release_svg_snapshot() {
    let layout = layout_release(fixture());
    let svg = layout.preview_svg();
    assert!(
        !svg.contains("<text"),
        "Use the exact PDF outlines, not browser text reflow"
    );
    let tree = resvg::usvg::Tree::from_str(&svg, &Default::default()).unwrap();
    assert_eq!(tree.size().width(), PAGE_WIDTH);
    assert_eq!(tree.size().height(), layout.preview_height());
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots/release.svg");
    if std::env::var_os("UPDATE_RELEASE_SNAPSHOT").is_some() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &svg).unwrap();
    }
    assert_eq!(
        fs::read_to_string(path).expect("committed SVG snapshot"),
        svg
    );
}

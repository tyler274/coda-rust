//! Patient-owned release template; rendering does not establish legal authority.
use super::*;

/// Patient identity, supplied and reviewed by the caller.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReleasePatient {
    /// Patient name.
    pub name: String,
    /// Calendar date of birth, as displayed.
    pub date_of_birth: String,
    /// Postal address.
    pub address: String,
    /// Contact phone.
    pub phone: String,
}

/// An identified discloser or recipient; directory data is not verified authority.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReleaseParty {
    /// Person or organization name.
    pub name: String,
    /// Public directory identifier, when applicable.
    pub npi: Option<String>,
    /// Postal/practice address.
    pub address: String,
    /// Contact phone.
    pub phone: String,
    /// Fax destination, to be confirmed before sending.
    pub fax: String,
}

/// Explicit destination selected by the patient.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ReleaseRecipient {
    /// Use the patient identity and address on this template.
    #[default]
    Patient,
    /// A caller-projected care-team snapshot.
    CareTeamMember(ReleaseParty),
    /// A patient-entered destination.
    Other(ReleaseParty),
}
/// Sensitive disclosure request, not a determination of legal consent.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SensitiveDisclosure {
    /// Whether the patient requested this category for separate review.
    pub requested: bool,
    /// Patient-entered initials. These do not establish valid authorization.
    pub initials: String,
}

/// Explicit record selection; no clinical selection is inferred by the renderer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReleaseScope {
    /// Full record, subject to exclusions and separate-review limitations.
    pub full_record: bool,
    /// Optional inclusive date-range display values (start, end).
    pub date_range: Option<(String, String)>,
    /// Visit notes, excluding psychotherapy notes.
    pub visit_notes: bool,
    /// Laboratory results.
    pub labs: bool,
    /// Imaging records.
    pub imaging: bool,
    /// Medication records.
    pub medications: bool,
    /// Mental health information requested for separate review.
    pub mental_health: SensitiveDisclosure,
    /// Substance use information requested for separate Part 2 review.
    pub substance_use: SensitiveDisclosure,
    /// HIV/STI information requested for separate review.
    pub hiv_sti: SensitiveDisclosure,
    /// Genetic information requested for separate review.
    pub genetic: SensitiveDisclosure,
}

/// Expiration related to the patient or purpose of the disclosure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseExpiration {
    /// Calendar date, formatted by the caller.
    Date(String),
    /// Explicit expiration event, supplied by the patient.
    Event(String),
}

/// Editable authorization template. Missing values remain visibly incomplete.
/// The caller owns identity, consent, signature evidence, validation and storage.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReleaseAuthorization {
    /// Patient identity.
    pub patient: ReleasePatient,
    /// Authorized disclosing person or organization.
    pub discloser: ReleaseParty,
    /// Recipient destination.
    pub recipient: ReleaseRecipient,
    /// Requested record categories.
    pub scope: ReleaseScope,
    /// Disclosure purpose; absent/blank defaults to the individual's request.
    pub purpose: Option<String>,
    /// Expiration date or event; absent remains incomplete.
    pub expiration: Option<ReleaseExpiration>,
    /// Typed name only, not a verified electronic signature.
    pub signed_name: Option<String>,
    /// Caller-supplied signing date; never inferred from rendering time.
    pub signed_date: Option<String>,
    /// Optional representative's authority description, if signing for the patient.
    pub representative_authority: Option<String>,
}

fn value(text: &str) -> &str {
    if text.trim().is_empty() {
        "Not provided"
    } else {
        text
    }
}
fn optional(text: &Option<String>) -> &str {
    value(text.as_deref().unwrap_or_default())
}
fn checkbox(selected: bool) -> &'static str {
    if selected { "[X]" } else { "[ ]" }
}
fn party(party: &ReleaseParty) -> String {
    format!(
        "Name: {}\nNPI: {}\nAddress: {}\nPhone: {}\nFax: {}",
        value(&party.name),
        optional(&party.npi),
        value(&party.address),
        value(&party.phone),
        value(&party.fax)
    )
}

fn blocks(auth: &ReleaseAuthorization) -> Vec<Block> {
    let mut result = vec![
        block(
            "Authorization to release health information",
            16.0,
            true,
            6.0,
        ),
        block(
            "Review template - not a guarantee of legal compliance or clinic acceptance. Confirm all entries and the receiving destination with the disclosing organization before use.",
            10.0,
            false,
            8.0,
        ),
    ];
    let mut section = |title: &str, text: String| {
        result.push(block(title, 11.0, true, 3.0));
        result.push(block(text, 10.0, false, 8.0));
    };
    section(
        "Patient",
        format!(
            "Name: {}\nDate of birth: {}\nAddress: {}\nPhone: {}",
            value(&auth.patient.name),
            value(&auth.patient.date_of_birth),
            value(&auth.patient.address),
            value(&auth.patient.phone)
        ),
    );
    section("Who may disclose", party(&auth.discloser));
    section(
        "Who may receive",
        match &auth.recipient {
            ReleaseRecipient::Patient => format!(
                "The patient themself\nName: {}\nAddress: {}\nPhone: {}",
                value(&auth.patient.name),
                value(&auth.patient.address),
                value(&auth.patient.phone)
            ),
            ReleaseRecipient::CareTeamMember(recipient) => {
                format!("Care-team member\n{}", party(recipient))
            }
            ReleaseRecipient::Other(recipient) => format!("Other recipient\n{}", party(recipient)),
        },
    );
    let scope = &auth.scope;
    let range = scope
        .date_range
        .as_ref()
        .map(|(start, end)| format!("{} through {}", value(start), value(end)))
        .unwrap_or_else(|| "Not selected".into());
    section(
        "Information to disclose",
        format!(
            "{} Full record (subject to exclusions below)\n{} Date range: {range}\n{} Visit notes\n{} Labs\n{} Imaging\n{} Medications\nOnly selected categories are requested. A selected date range limits the selected records. Full record does not override the sensitive-information review or psychotherapy-note exclusion.",
            checkbox(scope.full_record),
            checkbox(scope.date_range.is_some()),
            checkbox(scope.visit_notes),
            checkbox(scope.labs),
            checkbox(scope.imaging),
            checkbox(scope.medications)
        ),
    );
    let mut sensitive = String::new();
    for (label, selection) in [
        ("Mental health", &scope.mental_health),
        ("Substance use (42 CFR Part 2)", &scope.substance_use),
        ("HIV/STI", &scope.hiv_sti),
        ("Genetic", &scope.genetic),
    ] {
        let _ = writeln!(
            sensitive,
            "{} {label} - Initials: {}",
            checkbox(selection.requested),
            value(&selection.initials)
        );
    }
    sensitive.push_str("Sensitive categories require separate review before disclosure. Initials alone do not satisfy 42 CFR Part 2 or applicable state law; additional consent, statements, restrictions or forms may be required. Unselected sensitive categories are excluded, including from a full-record request.\nPsychotherapy notes are excluded from this authorization and require a separate authorization.");
    section(
        "Sensitive information - separate review required",
        sensitive,
    );
    section(
        "Purpose",
        auth.purpose
            .as_ref()
            .filter(|p| !p.trim().is_empty())
            .cloned()
            .unwrap_or_else(|| "At the request of the individual".into()),
    );
    section("Expiration", match &auth.expiration {
        Some(ReleaseExpiration::Date(date)) => format!("Expiration date: {}", value(date)),
        Some(ReleaseExpiration::Event(event)) => format!("Expiration event: {}", value(event)),
        None => "Not provided - enter an expiration date or an event related to the patient or purpose.".into(),
    });
    section(
        "Your rights and important statements",
        String::from(
            "Right to revoke: You may revoke this authorization in writing at any time by delivering a written revocation to the disclosing person or organization at the address above. Revocation does not affect action already taken in reliance on this authorization. If authorization was obtained as a condition of insurance coverage, other law may permit the insurer to contest a claim or the policy. Confirm the discloser's revocation procedure before use.\nNo conditioning: Treatment, payment, enrollment or eligibility for benefits generally may not be conditioned on signing this authorization. Exceptions permitted by law include research-related treatment, certain health-plan enrollment or eligibility determinations, and care provided solely to create information for a third party. If an exception applies, the discloser must explain the consequences of refusing before you sign.\nPotential for redisclosure: Information disclosed under this authorization may be redisclosed by the recipient and may no longer be protected by HIPAA. Other laws may still restrict use or redisclosure, including for substance use records.\nRight to a copy: You have the right to receive a copy of this signed authorization. Keep a copy for your records.",
        ),
    );
    section(
        "Signature and date",
        format!(
            "I authorize the discloser identified above to disclose the selected information to the identified recipient for the stated purpose until the stated expiration, subject to the exclusions and separate-review requirements above.\nTyped name: {}\nDate supplied: {}\nRepresentative authority (if applicable): {}\nA typed name alone does not prove a valid electronic signature. This template does not verify identity, intent or signing authority. Complete the discloser's accepted signing process before use.\nSignature: ________________________________________\nDate: ____________________",
            optional(&auth.signed_name),
            optional(&auth.signed_date),
            optional(&auth.representative_authority)
        ),
    );
    result
}

/// Lay out a release template across as many US Letter pages as needed.
/// The return type and PDF/preview API are shared with doctor pages, which retain
/// their one-page overflow policy. No signature or legal-validity determination is
/// made here, and [`PageLayout::fits`] means only that rendering is supported.
pub fn layout_release(auth: ReleaseAuthorization) -> PageLayout {
    let mut engine = Engine::new();
    let mut result = PageLayout {
        text: String::new(),
        size: 10.0,
        height: PAGE_HEIGHT,
        lines: Vec::new(),
        glyphs: Vec::new(),
        fonts: Vec::new(),
        blockers: Vec::new(),
        title: "Authorization to release health information",
        page_starts: vec![0],
    };
    let mut y = MARGIN;
    // Reserve a separate footer area. Split at shaped visual lines, never bytes.
    let bottom = PAGE_HEIGHT - MARGIN - 24.0;
    for block in blocks(&auth) {
        result.text.push_str(&block.text);
        result.text.push('\n');
        let shaped = engine.shape_block(
            &block.text,
            block.size,
            block.bold,
            MARGIN,
            0.0,
            PAGE_WIDTH - 2.0 * MARGIN,
            &mut result.blockers,
        );
        for mut line in shaped.lines {
            if y + line.height > bottom {
                finish_page(&mut engine, &mut result);
                result.page_starts.push(result.glyphs.len());
                y = MARGIN;
            }
            let offset = (result.page_count() - 1) as f32 * PAGE_HEIGHT + y - line.top;
            result.lines.push(PreviewLine {
                text: line.text,
                x: MARGIN,
                y: offset + line.baseline,
                width: line.width,
                size: block.size,
                bold: block.bold,
            });
            for glyph in &mut line.glyphs {
                glyph.y += offset;
            }
            result.glyphs.extend(line.glyphs);
            y += line.height;
        }
        y += block.gap;
    }
    finish_page(&mut engine, &mut result);
    result.height = result.page_count() as f32 * PAGE_HEIGHT;
    result.fonts = engine.fonts;
    result
}

fn finish_page(engine: &mut Engine, result: &mut PageLayout) {
    let index = result.page_count() - 1;
    let footer = block(
        format!("Release authorization template - Page {}", index + 1),
        9.0,
        false,
        0.0,
    );
    engine.shape(
        &[footer],
        index as f32 * PAGE_HEIGHT + PAGE_HEIGHT - MARGIN - 13.0,
        result,
    );
    let mut glyphs = result.glyphs[result.page_starts[index]..].to_vec();
    for glyph in &mut glyphs {
        glyph.y -= index as f32 * PAGE_HEIGHT;
    }
    if ink_overflows(&engine.fonts, &glyphs, PAGE_HEIGHT) {
        add_blocker(&mut result.blockers, ExportBlocker::Overflow);
    }
}

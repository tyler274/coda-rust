//! Root states for the four Duet tabs. Sentences are the ones the iPhone app shows.

use duet_client::{
    ConversationPage, DuetDoctorPlanView, EvidenceLibraryPage, InsightCyclePage,
    IntelligenceSettings,
};
use duet_wire::models::conversation_availability::Reason;
use duet_wire::models::conversation_message::Role;
use duet_wire::models::doctor_action_result::Status as DoctorStatus;
use duet_wire::models::doctor_page_proposal_result::Status as ProposalStatus;
use duet_wire::models::follow_up_action_result::Status as FollowStatus;
use duet_wire::models::saved_routine_action_result::Status as RoutineStatus;
use duet_wire::models::{
    ActionsInner, ConversationMessage, DoctorActionResult, DoctorCollection, DoctorCollectionItem,
    DoctorPageProposalResult, FollowUp, FollowUpActionResult, FollowUpOperation, SavedRoutine,
    SavedRoutineActionResult,
};

pub const CONNECTING: &str = "Connecting…";
pub const CHAT_EMPTY_TITLE: &str = "What’s on your mind?";
pub const CHAT_EMPTY_MESSAGE: &str = "Ask anything about your health.";
pub const CHAT_UNAVAILABLE: &str = "Chat isn’t available right now.";
pub const REVIEWS_PAUSED: &str = "Automatic reviews are paused";
pub const REVIEWS_PAUSED_DETAIL: &str =
    "Your saved insights and health information are still available.";
pub const CHECK_UNAVAILABLE: &str = "Duet couldn’t check right now";
pub const CHECK_UNAVAILABLE_DETAIL: &str =
    "This check couldn’t finish. You can still talk with Duet or explore your health information.";
pub const LIBRARY_EMPTY: &str = "No records or measurements yet.";
pub const LIBRARY_UNAVAILABLE: &str =
    "Your health information isn’t available right now. Try again in a moment.";
pub const TRY_AGAIN: &str = "Try again";
pub const ALLOW_NOTIFICATIONS: &str = "Allow notifications";
pub const INSIGHT_NOTIFICATIONS: &str = "New insight notifications";
pub const INSIGHT_NOTIFICATIONS_DETAIL: &str = "Notify me when Duet finds something worth showing.";
pub const OPEN_NOTIFICATION_SETTINGS: &str = "Open iPhone Settings";
pub const YOUR_DOCTORS_PLAN: &str = "your doctor's plan";

/// One read-only item in `"your doctor's plan"` (`ENG-54`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorPlanRow {
    pub title: String,
    pub body: String,
}

/// Read-only `"your doctor's plan"` section displayed in Duet (`ENG-54`),
/// strictly separated from Duet's adaptive routines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorPlanSection {
    pub heading: &'static str,
    pub read_only: bool,
    pub attribution: String,
    pub items: Vec<DoctorPlanRow>,
}

/// Builds the read-only `"your doctor's plan"` section from an approved physician plan.
pub fn doctor_plan_section(plan: Option<&DuetDoctorPlanView>) -> Option<DoctorPlanSection> {
    let plan = plan?;
    if plan.items.is_empty() {
        return None;
    }
    Some(DoctorPlanSection {
        heading: YOUR_DOCTORS_PLAN,
        read_only: true,
        attribution: format!("From {} · v{}", plan.physician_name, plan.version),
        items: plan
            .items
            .iter()
            .map(|item| DoctorPlanRow {
                title: item.title.clone(),
                body: item.body.clone(),
            })
            .collect(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoctorPage {
    /// The collection loaded and has nothing to list.
    Empty,
    /// Items, note, questions, and cover note, in that order.
    Lines(Vec<ConversationEntry>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationEntry {
    pub title: String,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardTarget {
    /// The saved Plan this card names.
    Routine(uuid::Uuid),
    /// The reminder or check-in this card names.
    FollowUp(uuid::Uuid),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteOpen {
    /// The For my doctor collection already loaded with the thread.
    Doctor,
    /// A Plan whose id is in the loaded routine list.
    Routine(uuid::Uuid),
    /// A reminder or check-in whose id is in the loaded follow-up list.
    FollowUp(uuid::Uuid),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedItemPage {
    /// **Plan**, **Reminder**, or **Check-in**.
    pub heading: String,
    /// The routine or follow-up title.
    pub title: Option<String>,
    /// The routine body, or the follow-up schedule.
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationCard {
    pub receipt: String,
    pub title: Option<String>,
    pub detail: Option<String>,
    pub schedule: Option<String>,
    pub entries: Vec<ConversationEntry>,
    pub target: Option<CardTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationNote {
    Card(ConversationCard),
    Receipt(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationSourceChip {
    pub message_id: uuid::Uuid,
    pub source_id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationLine {
    pub role: String,
    pub text: String,
    pub from_duet: bool,
    pub notes: Vec<ConversationNote>,
    pub sources: Vec<ConversationSourceChip>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationRoot {
    Empty {
        title: &'static str,
        message: &'static str,
    },
    Messages(Vec<ConversationLine>),
}

fn availability_copy(reason: Option<Reason>) -> (&'static str, &'static str) {
    match reason {
        None => (CHAT_EMPTY_TITLE, CHAT_EMPTY_MESSAGE),
        Some(Reason::PermissionRequired) => (
            "Chat with Duet",
            "Review AI permissions in Settings before starting a conversation.",
        ),
        Some(Reason::Disabled) => ("Chat with Duet", "Chat is turned off in Settings."),
        Some(Reason::DailyLimit) => (
            "Chat with Duet",
            "You’ve reached today’s limit. Duet will be ready again tomorrow.",
        ),
        Some(Reason::Unavailable) => ("Chat with Duet", CHAT_UNAVAILABLE),
    }
}

fn availability_reason(page: &ConversationPage) -> Option<Reason> {
    page.availability.reason.as_ref().and_then(|reason| *reason)
}

/// The composer is shown when this page allows a send, including when messages
/// are already on screen.
pub fn composer_open(page: &ConversationPage) -> bool {
    page.availability.can_send
}

/// Reason sentences for a page that already has messages and cannot send.
/// An empty page keeps those sentences in [`conversation_root`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosedSend {
    /// The title the empty chat uses for this reason.
    pub title: &'static str,
    /// The sentence the empty chat uses for this reason.
    pub message: &'static str,
}

/// Returns the existing reason sentences when messages are on screen and a send
/// is not allowed. Returns nothing when the page can send, or when it has no
/// messages, because the empty state already carries the sentences.
pub fn closed_send(page: &ConversationPage) -> Option<ClosedSend> {
    if page.availability.can_send || page.messages.is_empty() {
        return None;
    }
    let reason = availability_reason(page)?;
    let (title, message) = availability_copy(Some(reason));
    Some(ClosedSend { title, message })
}

/// A failed send keeps the draft. An accepted send clears it.
pub fn draft_after_send(draft: &str, accepted: bool) -> String {
    if accepted {
        String::new()
    } else {
        draft.to_owned()
    }
}

/// A retry of an uncertain send reuses its id while the draft is unchanged.
/// A new or edited draft takes a fresh id.
pub fn send_id(pending: Option<(uuid::Uuid, &str)>, text: &str, fresh: uuid::Uuid) -> uuid::Uuid {
    match pending {
        Some((id, pending_text)) if pending_text == text => id,
        _ => fresh,
    }
}

/// The reloaded page keeps the accepted user message. A reply that is not on
/// the page stays absent.
pub fn page_with_accepted(
    mut page: ConversationPage,
    accepted: ConversationMessage,
) -> ConversationPage {
    if page
        .messages
        .iter()
        .any(|message| message.id == accepted.id)
    {
        return page;
    }
    page.messages.push(accepted);
    page
}

pub fn conversation_root(
    page: &ConversationPage,
    routines: &[SavedRoutine],
    follow_ups: &[FollowUp],
    collection: Option<&DoctorCollection>,
) -> ConversationRoot {
    if page.messages.is_empty() {
        let (title, message) = availability_copy(availability_reason(page));
        return ConversationRoot::Empty { title, message };
    }
    ConversationRoot::Messages(
        page.messages
            .iter()
            .map(|message| ConversationLine {
                role: match message.role {
                    Role::User => "You".to_owned(),
                    Role::Assistant => "Duet".to_owned(),
                },
                text: message.text.clone(),
                from_duet: message.role == Role::Assistant,
                notes: message_notes(message, routines, follow_ups, collection),
                sources: message
                    .sources
                    .iter()
                    .flatten()
                    .map(|source| ConversationSourceChip {
                        message_id: message.id,
                        source_id: source.id.clone(),
                        title: source.title.clone(),
                    })
                    .collect(),
            })
            .collect(),
    )
}

fn message_notes(
    message: &ConversationMessage,
    routines: &[SavedRoutine],
    follow_ups: &[FollowUp],
    collection: Option<&DoctorCollection>,
) -> Vec<ConversationNote> {
    message
        .actions
        .iter()
        .flatten()
        .filter_map(|action| note_for(action, routines, follow_ups, collection))
        .collect()
}

fn note_for(
    action: &ActionsInner,
    routines: &[SavedRoutine],
    follow_ups: &[FollowUp],
    collection: Option<&DoctorCollection>,
) -> Option<ConversationNote> {
    match action {
        ActionsInner::CreateSavedRoutine(result) => Some(routine_note(result, false, routines)),
        ActionsInner::UpdateSavedRoutine(result) => Some(routine_note(result, true, routines)),
        ActionsInner::CreateFollowUp(result) => {
            Some(follow_note(result, FollowKind::Create, follow_ups))
        }
        ActionsInner::UpdateFollowUp(result) => {
            Some(follow_note(result, FollowKind::Update, follow_ups))
        }
        ActionsInner::PauseFollowUp(result) => {
            Some(follow_note(result, FollowKind::Pause, follow_ups))
        }
        ActionsInner::ResumeFollowUp(result) => {
            Some(follow_note(result, FollowKind::Resume, follow_ups))
        }
        ActionsInner::CancelFollowUp(result) => {
            Some(follow_note(result, FollowKind::Cancel, follow_ups))
        }
        ActionsInner::SaveDoctorQuestion(result) => {
            Some(saved_doctor_note(result, false, collection))
        }
        ActionsInner::PrepareDoctorSummary(result) => {
            Some(saved_doctor_note(result, true, collection))
        }
        ActionsInner::ProposeDoctorPage(result) => Some(proposal_note(result, collection)),
    }
}

fn saved_doctor_note(
    result: &DoctorActionResult,
    prepared: bool,
    collection: Option<&DoctorCollection>,
) -> ConversationNote {
    let words = if prepared {
        "For my doctor saved"
    } else {
        "Saved to For my doctor"
    };
    let changed = matches!(result.status, DoctorStatus::Changed);
    let receipt = marked(words, (changed, false));
    let entries = collection.map(saved_entries).unwrap_or_default();
    if entries.is_empty() {
        return ConversationNote::Receipt(receipt);
    }
    ConversationNote::Card(ConversationCard {
        receipt,
        title: None,
        detail: None,
        schedule: None,
        entries,
        target: None,
    })
}

fn proposal_note(
    result: &DoctorPageProposalResult,
    collection: Option<&DoctorCollection>,
) -> ConversationNote {
    let changed = matches!(result.status, ProposalStatus::Changed);
    let entries = collection
        .and_then(|collection| collection.proposal.as_ref())
        .and_then(|proposal| proposal.as_ref())
        .map(|proposal| proposal_entries(proposal))
        .unwrap_or_default();
    ConversationNote::Card(ConversationCard {
        receipt: marked("For my doctor", (changed, false)),
        title: Some("Review".to_owned()),
        detail: Some("Review and save your page when you’re ready.".to_owned()),
        schedule: None,
        entries,
        target: None,
    })
}

/// Whether this receipt opens the For my doctor page.
pub fn opens_doctor(note: &ConversationNote) -> bool {
    let receipt = match note {
        ConversationNote::Card(card) => card.receipt.as_str(),
        ConversationNote::Receipt(sentence) => sentence.as_str(),
    };
    receipt.starts_with("Saved to For my doctor") || receipt.starts_with("For my doctor")
}

/// The lines a loaded For my doctor collection already contains.
pub fn doctor_page(collection: &DoctorCollection) -> DoctorPage {
    let mut entries = item_entries(collection.items.as_deref().unwrap_or(&[]));
    if let Some(note) = supplied(&collection.note) {
        entries.push(ConversationEntry {
            title: note,
            detail: None,
        });
    }
    if let Some(page) = collection.page.as_deref() {
        if let Some(questions) = &page.questions {
            for question in questions {
                if let Some(title) = nonempty(question) {
                    entries.push(ConversationEntry {
                        title,
                        detail: None,
                    });
                }
            }
        }
        if let Some(cover) = supplied(&page.cover_note) {
            entries.push(ConversationEntry {
                title: cover,
                detail: Some("Cover note (not included in PDF)".into()),
            });
        }
    }
    if entries.is_empty() {
        DoctorPage::Empty
    } else {
        DoctorPage::Lines(entries)
    }
}

fn saved_entries(collection: &DoctorCollection) -> Vec<ConversationEntry> {
    let mut entries = item_entries(collection.items.as_deref().unwrap_or(&[]));
    if let Some(note) = supplied(&collection.note) {
        entries.push(ConversationEntry {
            title: note,
            detail: None,
        });
    }
    entries
}

fn proposal_entries(proposal: &duet_wire::models::DoctorPageProposal) -> Vec<ConversationEntry> {
    let mut entries = item_entries(&proposal.items);
    if let Some(questions) = &proposal.page.questions {
        for question in questions {
            if let Some(title) = nonempty(question) {
                entries.push(ConversationEntry {
                    title,
                    detail: None,
                });
            }
        }
    }
    if let Some(note) = proposal.note.as_deref().and_then(nonempty) {
        entries.push(ConversationEntry {
            title: note,
            detail: None,
        });
    }
    entries
}

fn item_entries(items: &[DoctorCollectionItem]) -> Vec<ConversationEntry> {
    items
        .iter()
        .filter_map(|item| {
            let title = nonempty(&item.title)?;
            Some(ConversationEntry {
                title,
                detail: nonempty(&item.detail),
            })
        })
        .collect()
}

fn routine_note(
    result: &SavedRoutineActionResult,
    updated: bool,
    routines: &[SavedRoutine],
) -> ConversationNote {
    let receipt = if updated {
        "Plan updated"
    } else {
        "Plan saved"
    };
    let receipt = marked(receipt, routine_marks(result.status));
    let Some(routine) = routines
        .iter()
        .find(|routine| routine.id == result.routine_id)
    else {
        return ConversationNote::Receipt(receipt);
    };
    let title = nonempty(&routine.title);
    let detail = nonempty(&routine.body);
    if title.is_none() && detail.is_none() {
        return ConversationNote::Receipt(receipt);
    }
    ConversationNote::Card(ConversationCard {
        receipt,
        title,
        detail,
        schedule: None,
        entries: vec![],
        target: Some(CardTarget::Routine(routine.id)),
    })
}

enum FollowKind {
    Create,
    Update,
    Pause,
    Resume,
    Cancel,
}

fn follow_note(
    result: &FollowUpActionResult,
    kind: FollowKind,
    follow_ups: &[FollowUp],
) -> ConversationNote {
    let item = follow_ups
        .iter()
        .find(|follow_up| follow_up.id == result.follow_up_id);
    let marks = follow_marks(result.status);
    match kind {
        FollowKind::Create | FollowKind::Update => {
            let Some(item) = item else {
                let words = match kind {
                    FollowKind::Update => "Updated in Coming up",
                    _ => "Added to Coming up",
                };
                return ConversationNote::Receipt(marked(words, marks));
            };
            let receipt = marked(follow_kind(item.operation), marks);
            let title = follow_title(item);
            let schedule = nonempty(&item.schedule_summary);
            if title.is_none() && schedule.is_none() {
                return ConversationNote::Receipt(receipt);
            }
            ConversationNote::Card(ConversationCard {
                receipt,
                title,
                detail: None,
                schedule,
                entries: vec![],
                target: Some(CardTarget::FollowUp(item.id)),
            })
        }
        FollowKind::Pause | FollowKind::Resume | FollowKind::Cancel => {
            let words = match (kind, item) {
                (FollowKind::Pause, Some(item)) => {
                    format!("{} paused", follow_kind(item.operation))
                }
                (FollowKind::Resume, Some(item)) => {
                    format!("{} resumed", follow_kind(item.operation))
                }
                (FollowKind::Cancel, Some(item)) => {
                    format!("{} cancelled", follow_kind(item.operation))
                }
                (FollowKind::Pause, None) => "Paused in Coming up".to_owned(),
                (FollowKind::Resume, None) => "Resumed in Coming up".to_owned(),
                _ => "Removed from Coming up".to_owned(),
            };
            ConversationNote::Receipt(marked(&words, marks))
        }
    }
}

/// The Plan page for a routine the thread already loaded.
pub fn plan_page(routine: &SavedRoutine) -> SavedItemPage {
    SavedItemPage {
        heading: "Plan".to_owned(),
        title: nonempty(&routine.title),
        detail: nonempty(&routine.body),
    }
}

/// The reminder or check-in page for a follow-up the thread already loaded.
pub fn reminder_page(follow_up: &FollowUp) -> SavedItemPage {
    SavedItemPage {
        heading: follow_kind(follow_up.operation).to_owned(),
        title: follow_title(follow_up),
        detail: nonempty(&follow_up.schedule_summary),
    }
}

/// The page a note opens when its item is already loaded.
pub fn note_open(
    note: &ConversationNote,
    has_doctor: bool,
    routines: &[SavedRoutine],
    follow_ups: &[FollowUp],
) -> Option<NoteOpen> {
    if has_doctor && opens_doctor(note) {
        return Some(NoteOpen::Doctor);
    }
    let ConversationNote::Card(card) = note else {
        return None;
    };
    match card.target {
        Some(CardTarget::Routine(id)) if routines.iter().any(|routine| routine.id == id) => {
            Some(NoteOpen::Routine(id))
        }
        Some(CardTarget::FollowUp(id)) if follow_ups.iter().any(|follow_up| follow_up.id == id) => {
            Some(NoteOpen::FollowUp(id))
        }
        _ => None,
    }
}

fn follow_kind(operation: FollowUpOperation) -> &'static str {
    match operation {
        FollowUpOperation::Reminder => "Reminder",
        FollowUpOperation::CheckIn => "Check-in",
    }
}

fn follow_title(follow_up: &FollowUp) -> Option<String> {
    supplied(&follow_up.display_title).or_else(|| nonempty(&follow_up.accepted_instruction))
}

fn routine_marks(status: RoutineStatus) -> (bool, bool) {
    match status {
        RoutineStatus::Confirmed => (false, false),
        RoutineStatus::Changed => (true, false),
    }
}

fn follow_marks(status: FollowStatus) -> (bool, bool) {
    match status {
        FollowStatus::Confirmed => (false, false),
        FollowStatus::Changed => (true, false),
        FollowStatus::Cancelled => (false, true),
    }
}

fn marked(words: &str, marks: (bool, bool)) -> String {
    match marks {
        (_, true) => format!("{words}; cancelled since then"),
        (true, _) => format!("{words}; changed since then"),
        _ => words.to_owned(),
    }
}

fn nonempty(text: &str) -> Option<String> {
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsightCard {
    pub title: String,
    pub sentence: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TodayRoot {
    Quiet,
    Paused,
    Insights(Vec<InsightCard>),
}

/// Insights outrank a paused setting. A cycle with no sentences is quiet.
/// Reviews are paused when intelligence is off and there is nothing to show.
pub fn today_root(settings: &IntelligenceSettings, page: &InsightCyclePage) -> TodayRoot {
    let mut cards = Vec::new();
    for cycle in &page.items {
        for insight in cycle.insights.iter().flatten() {
            cards.push(InsightCard {
                title: insight.title.clone(),
                sentence: insight.detail.clone(),
            });
        }
    }
    if !cards.is_empty() {
        return TodayRoot::Insights(cards);
    }
    if !settings.enabled {
        TodayRoot::Paused
    } else {
        TodayRoot::Quiet
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryKind {
    HealthEvidence,
    ClinicalNote,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryRecord {
    pub id: String,
    pub kind: LibraryKind,
    pub title: String,
    pub summary: Option<String>,
    pub when: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibrarySection {
    pub name: &'static str,
    pub measured: bool,
    pub records: Vec<LibraryRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryRoot {
    Empty,
    Sections(Vec<LibrarySection>),
}

const SECTIONS: &[(&[&str], &str, bool)] = &[
    (&["medication"], "Medications", false),
    (&["allergy"], "Allergies", false),
    (
        &["currentCondition", "conditionOnRecord", "pastCondition"],
        "Conditions",
        false,
    ),
    (&["lab"], "Recent labs", true),
    (&["procedure"], "Procedures", false),
    (&["imaging"], "Imaging", false),
    (
        &[
            "resultReport",
            "clinicalDocument",
            "encounter",
            "immunization",
            "coverage",
            "visionPrescription",
            "other",
        ],
        "Records & documents",
        false,
    ),
];

pub fn library_root(page: &EvidenceLibraryPage) -> LibraryRoot {
    let mut present = std::collections::HashSet::new();
    for (key, count) in &page.counts {
        if *count > 0 {
            present.insert(key.clone());
        }
    }
    for entry in &page.entries {
        present.insert(entry.category.clone());
    }
    if present.is_empty() && page.entries.is_empty() && page.total_count == 0 {
        return LibraryRoot::Empty;
    }
    let sections = SECTIONS
        .iter()
        .filter(|(keys, _, _)| keys.iter().any(|key| present.contains(*key)))
        .map(|(keys, name, measured)| LibrarySection {
            name,
            measured: *measured,
            records: page
                .entries
                .iter()
                .filter(|entry| keys.contains(&entry.category.as_str()))
                .map(library_record)
                .collect(),
        })
        .collect();
    LibraryRoot::Sections(sections)
}

fn library_record(entry: &duet_wire::models::EvidenceLibraryEntry) -> LibraryRecord {
    let when = entry
        .date
        .map(|date| calendar_line(date.date_naive()))
        .or_else(|| entry.day.flatten().map(calendar_line));
    LibraryRecord {
        id: entry.id.clone(),
        kind: match entry.entity {
            duet_wire::models::evidence_library_entry::Entity::HealthEvidence => {
                LibraryKind::HealthEvidence
            }
            duet_wire::models::evidence_library_entry::Entity::ClinicalNote => {
                LibraryKind::ClinicalNote
            }
        },
        title: entry.title.clone(),
        summary: supplied(&entry.summary),
        when,
        source: supplied(&entry.source_label),
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoctorPreview {
    Fits(Vec<String>),
    Overflow,
}

/// Places the saved For my doctor page with the letter layout.
/// A collection that has no page content yet returns nothing.
#[cfg(test)]
pub fn doctor_preview(collection: &DoctorCollection) -> Option<DoctorPreview> {
    let content = collection.page.as_ref()?;
    let page = content.as_ref();
    let name = supplied(&page.name).unwrap_or_default();
    let date_of_birth = page
        .date_of_birth
        .flatten()
        .map(|day| day.format("%Y-%m-%d").to_string())
        .unwrap_or_default();
    let questions = page
        .questions
        .iter()
        .flatten()
        .map(|question| question.trim().to_owned())
        .filter(|question| !question.is_empty())
        .collect::<Vec<_>>();
    let cover_note = supplied(&page.cover_note);
    if name.is_empty() && date_of_birth.is_empty() && questions.is_empty() && cover_note.is_none() {
        return None;
    }
    Some(
        match duet_page::lay_out(&duet_page::DoctorPage {
            name,
            date_of_birth,
            questions,
            cover_note,
        }) {
            duet_page::LaidOut::Page { lines, .. } => {
                DoctorPreview::Fits(lines.into_iter().map(|line| line.text).collect())
            }
            duet_page::LaidOut::Overflow => DoctorPreview::Overflow,
        },
    )
}

fn supplied(value: &Option<Option<String>>) -> Option<String> {
    value
        .as_ref()
        .and_then(|text| text.as_ref())
        .map(|text| text.trim())
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn calendar_line(day: chrono::NaiveDate) -> String {
    let number = day.format("%e").to_string();
    format!(
        "{} {}, {}",
        day.format("%B"),
        number.trim(),
        day.format("%Y")
    )
}

pub fn greeting(hour: u32) -> &'static str {
    match hour {
        5..12 => "Good morning",
        12..17 => "Good afternoon",
        _ => "Good evening",
    }
}

pub fn date_line(date: chrono::DateTime<chrono::Local>) -> String {
    let day = date.format("%e").to_string();
    format!(
        "{}, {} {}",
        date.format("%A"),
        date.format("%B"),
        day.trim()
    )
}

pub fn default_api_base() -> &'static str {
    if cfg!(target_os = "android") {
        "http://10.0.2.2:8000"
    } else {
        "http://127.0.0.1:8000"
    }
}

/// Returns the Duet API origin for this build.
pub fn api_base_url() -> String {
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    if let Some(window) = web_sys::window() {
        return window.location().origin().unwrap_or_default();
    }
    match option_env!("DUET_API_URL") {
        Some(url) if !url.is_empty() => url.to_owned(),
        _ => default_api_base().to_owned(),
    }
}

/// Returns the bearer this build sends.
pub fn api_bearer() -> String {
    // The dedicated Duet web build never embeds a bearer; the mode shell's
    // synthetic browser journeys still compile one in.
    #[cfg(not(all(target_arch = "wasm32", feature = "web")))]
    if let Some(bearer) = option_env!("DUET_BEARER")
        && !bearer.is_empty()
    {
        return bearer.to_owned();
    }
    recalled_session().unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
const SESSION_KEY: &str = "coda-duet-session";

/// Remove the old insecure browser credential. Native refresh is owned by AccountRoot.
pub fn recalled_session() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.remove_item(SESSION_KEY);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn json<T: serde::de::DeserializeOwned>(body: &str) -> T {
        serde_json::from_str(body).unwrap()
    }

    #[test]
    fn conversation_empty_and_loaded() {
        let empty = json::<ConversationPage>(
            r#"{"availability":{"can_send":true},"messages":[],"revision":1}"#,
        );
        assert_eq!(
            conversation_root(&empty, &[], &[], None),
            ConversationRoot::Empty {
                title: CHAT_EMPTY_TITLE,
                message: CHAT_EMPTY_MESSAGE
            }
        );
        let loaded = json::<ConversationPage>(
            r#"{"availability":{"can_send":true},"messages":[{"created_at":"2026-10-09T15:00:00Z","id":"00000000-0000-0000-0000-000000000001","revision":1,"role":"user","sequence":1,"state":"complete","text":"How did I sleep?","updated_at":"2026-10-09T15:00:00Z"},{"created_at":"2026-10-09T15:00:01Z","id":"00000000-0000-0000-0000-000000000002","revision":1,"role":"assistant","sequence":2,"state":"complete","text":"About seven hours.","updated_at":"2026-10-09T15:00:01Z"}],"revision":2}"#,
        );
        assert_eq!(
            conversation_root(&loaded, &[], &[], None),
            ConversationRoot::Messages(vec![
                ConversationLine {
                    role: "You".into(),
                    text: "How did I sleep?".into(),
                    from_duet: false,
                    notes: vec![],
                    sources: vec![],
                },
                ConversationLine {
                    role: "Duet".into(),
                    text: "About seven hours.".into(),
                    from_duet: true,
                    notes: vec![],
                    sources: vec![],
                },
            ])
        );
        assert!(composer_open(&empty));
        assert!(composer_open(&loaded));
        assert_eq!(closed_send(&empty), None);
        assert_eq!(closed_send(&loaded), None);
    }

    #[test]
    fn a_message_keeps_its_plan_and_reminder() {
        let page = json::<ConversationPage>(
            r#"{"availability":{"can_send":true},"messages":[{"created_at":"2026-10-09T15:00:02Z","id":"00000000-0000-0000-0000-000000000005","revision":1,"role":"assistant","sequence":3,"state":"complete","text":"Evening walk is saved.","updated_at":"2026-10-09T15:00:02Z","actions":[{"kind":"create_saved_routine","base_generation":0,"confirmed_at":"2026-10-09T15:00:02Z","current_generation":1,"id":"00000000-0000-0000-0000-000000000043","message_id":"00000000-0000-0000-0000-000000000005","result_generation":1,"routine_id":"00000000-0000-0000-0000-000000000041","run_id":"00000000-0000-0000-0000-000000000099","status":"confirmed"},{"kind":"create_follow_up","confirmed_at":"2026-10-09T15:00:02Z","current_revision":1,"follow_up_id":"00000000-0000-0000-0000-000000000042","id":"00000000-0000-0000-0000-000000000044","message_id":"00000000-0000-0000-0000-000000000005","result_revision":1,"run_id":"00000000-0000-0000-0000-000000000099","status":"confirmed"},{"kind":"save_doctor_question","base_generation":0,"confirmed_at":"2026-10-09T15:00:02Z","current_generation":1,"id":"00000000-0000-0000-0000-000000000045","message_id":"00000000-0000-0000-0000-000000000005","result_generation":1,"run_id":"00000000-0000-0000-0000-000000000099","status":"confirmed"}]},{"created_at":"2026-10-09T15:00:00Z","id":"00000000-0000-0000-0000-000000000001","revision":1,"role":"user","sequence":1,"state":"complete","text":"How did I sleep?","updated_at":"2026-10-09T15:00:00Z"}],"revision":2}"#,
        );
        let routines = vec![json::<SavedRoutine>(
            r#"{"body":"A short walk after dinner.","created_at":"2026-10-09T15:00:02Z","id":"00000000-0000-0000-0000-000000000041","revision":1,"title":"Evening walk","updated_at":"2026-10-09T15:00:02Z"}"#,
        )];
        let follow_ups = vec![json::<FollowUp>(
            r#"{"accepted_instruction":"A short walk after dinner.","created_at":"2026-10-09T15:00:02Z","display_title":"Evening walk","id":"00000000-0000-0000-0000-000000000042","operation":"reminder","revision":1,"schedule":{"kind":"once","local_date":"2026-10-11","local_time":"19:00","timezone":"America/Los_Angeles"},"schedule_summary":"Every evening","source_basis":[],"state":"active","updated_at":"2026-10-09T15:00:02Z"}"#,
        )];
        let collection = json::<DoctorCollection>(
            r#"{"items":[{"detail":"How the week of walks has felt.","evidence":[],"id":"00000000-0000-0000-0000-000000000046","kind":"fact","position":0,"saved_at":"2026-10-09T15:00:02Z","source_id":"question-1","title":"Ask about sleep"}],"note_generation":1}"#,
        );
        assert_eq!(
            conversation_root(&page, &routines, &follow_ups, Some(&collection)),
            ConversationRoot::Messages(vec![
                ConversationLine {
                    role: "Duet".into(),
                    text: "Evening walk is saved.".into(),
                    from_duet: true,
                    notes: vec![
                        ConversationNote::Card(ConversationCard {
                            receipt: "Plan saved".into(),
                            title: Some("Evening walk".into()),
                            detail: Some("A short walk after dinner.".into()),
                            schedule: None,
                            entries: vec![],
                            target: Some(CardTarget::Routine(routines[0].id)),
                        }),
                        ConversationNote::Card(ConversationCard {
                            receipt: "Reminder".into(),
                            title: Some("Evening walk".into()),
                            detail: None,
                            schedule: Some("Every evening".into()),
                            entries: vec![],
                            target: Some(CardTarget::FollowUp(follow_ups[0].id)),
                        }),
                        ConversationNote::Card(ConversationCard {
                            receipt: "Saved to For my doctor".into(),
                            title: None,
                            detail: None,
                            schedule: None,
                            entries: vec![ConversationEntry {
                                title: "Ask about sleep".into(),
                                detail: Some("How the week of walks has felt.".into()),
                            }],
                            target: None,
                        }),
                    ],
                    sources: vec![],
                },
                ConversationLine {
                    role: "You".into(),
                    text: "How did I sleep?".into(),
                    from_duet: false,
                    notes: vec![],
                    sources: vec![],
                },
            ])
        );
        assert_eq!(
            plan_page(&routines[0]),
            SavedItemPage {
                heading: "Plan".into(),
                title: Some("Evening walk".into()),
                detail: Some("A short walk after dinner.".into()),
            }
        );
        assert_eq!(
            reminder_page(&follow_ups[0]),
            SavedItemPage {
                heading: "Reminder".into(),
                title: Some("Evening walk".into()),
                detail: Some("Every evening".into()),
            }
        );
        let opened = conversation_root(&page, &routines, &follow_ups, Some(&collection));
        let ConversationRoot::Messages(lines) = &opened else {
            panic!("expected messages");
        };
        assert_eq!(
            note_open(&lines[0].notes[0], false, &routines, &follow_ups),
            Some(NoteOpen::Routine(routines[0].id))
        );
        assert_eq!(note_open(&lines[0].notes[0], false, &[], &follow_ups), None);
        assert_eq!(
            note_open(&lines[0].notes[1], false, &routines, &follow_ups),
            Some(NoteOpen::FollowUp(follow_ups[0].id))
        );
        assert_eq!(note_open(&lines[0].notes[1], false, &routines, &[]), None);
        assert!(
            conversation_root(&page, &[], &[], None).eq(&ConversationRoot::Messages(vec![
                ConversationLine {
                    role: "Duet".into(),
                    text: "Evening walk is saved.".into(),
                    from_duet: true,
                    notes: vec![
                        ConversationNote::Receipt("Plan saved".into()),
                        ConversationNote::Receipt("Added to Coming up".into()),
                        ConversationNote::Receipt("Saved to For my doctor".into()),
                    ],
                    sources: vec![],
                },
                ConversationLine {
                    role: "You".into(),
                    text: "How did I sleep?".into(),
                    from_duet: false,
                    notes: vec![],
                    sources: vec![],
                },
            ]))
        );
    }

    #[test]
    fn doctor_page_lists_the_saved_question() {
        let collection = json::<DoctorCollection>(
            r#"{"items":[{"detail":"How the week of walks has felt.","evidence":[],"id":"00000000-0000-0000-0000-000000000046","kind":"fact","position":0,"saved_at":"2026-10-09T15:00:02Z","source_id":"question-1","title":"Ask about sleep"}],"note_generation":1}"#,
        );
        assert_eq!(
            doctor_page(&collection),
            DoctorPage::Lines(vec![ConversationEntry {
                title: "Ask about sleep".into(),
                detail: Some("How the week of walks has felt.".into()),
            }])
        );
        let empty = json::<DoctorCollection>(r#"{"note_generation":0}"#);
        assert_eq!(doctor_page(&empty), DoctorPage::Empty);
    }

    #[test]
    fn a_message_source_is_a_chip_on_its_line() {
        let page = json::<ConversationPage>(
            r#"{"availability":{"can_send":true},"messages":[{"created_at":"2026-10-09T15:00:01Z","id":"00000000-0000-0000-0000-000000000002","revision":1,"role":"assistant","sequence":2,"sources":[{"id":"sleep-note","kind":"record","source":"Sleep","title":"Sleep note"}],"state":"complete","text":"About seven hours.","updated_at":"2026-10-09T15:00:01Z"}],"revision":2}"#,
        );
        let ConversationRoot::Messages(lines) = conversation_root(&page, &[], &[], None) else {
            panic!("expected messages");
        };
        assert_eq!(
            lines[0].sources,
            vec![ConversationSourceChip {
                message_id: uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
                source_id: "sleep-note".into(),
                title: "Sleep note".into(),
            }]
        );
    }

    #[test]
    fn a_saved_doctor_page_uses_the_letter_layout() {
        let collection = json::<DoctorCollection>(
            r#"{"items":[],"note_generation":1,"page":{"name":"Ada Lovelace","date_of_birth":"1815-12-10","questions":["What changed?"],"cover_note":"Please review."}}"#,
        );
        match doctor_preview(&collection) {
            Some(DoctorPreview::Fits(lines)) => {
                assert!(lines.iter().any(|line| line.contains("Ada Lovelace")));
                assert!(lines.iter().any(|line| line.contains("1. What changed?")));
                assert!(!lines.iter().any(|line| line.contains("Please review.")));
            }
            other => panic!("expected a fitted page, got {other:?}"),
        }
        // The opened saved-page summary is rendered outside DoctorLetter's printable preview.
        let DoctorPage::Lines(entries) = doctor_page(&collection) else {
            panic!("expected the saved-page summary");
        };
        assert!(entries.iter().any(|entry| entry.title == "Please review."
            && entry.detail.as_deref() == Some("Cover note (not included in PDF)")));
        // Exercise the actual current preview path too, not just the legacy layout adapter.
        let preview = crate::saved_preview(&collection).unwrap();
        assert!(preview.layout.text().contains("Ada Lovelace"));
        assert!(preview.layout.text().contains("What changed?"));
        assert!(!preview.layout.text().contains("Please review."));
        let mut without_cover = collection.clone();
        without_cover.page.as_mut().unwrap().cover_note = None;
        let plain = crate::saved_preview(&without_cover).unwrap();
        assert_eq!(preview.layout.preview_svg(), plain.layout.preview_svg());
        assert_eq!(preview.layout.pdf().unwrap(), plain.layout.pdf().unwrap());
        let overflow = json::<DoctorCollection>(
            r#"{"items":[],"note_generation":1,"page":{"name":"Ada","date_of_birth":"1815-12-10","questions":["One","Two","Three","Four","Five","Six"]}}"#,
        );
        assert_eq!(doctor_preview(&overflow), Some(DoctorPreview::Overflow));
        let empty = json::<DoctorCollection>(r#"{"note_generation":0}"#);
        assert_eq!(doctor_preview(&empty), None);
    }

    #[test]
    fn a_closed_page_hides_the_composer_and_keeps_the_reason() {
        let empty = json::<ConversationPage>(
            r#"{"availability":{"can_send":false,"reason":"daily_limit"},"messages":[],"revision":1}"#,
        );
        assert!(!composer_open(&empty));
        assert_eq!(closed_send(&empty), None);
        assert_eq!(
            conversation_root(&empty, &[], &[], None),
            ConversationRoot::Empty {
                title: "Chat with Duet",
                message: "You’ve reached today’s limit. Duet will be ready again tomorrow."
            }
        );
        let loaded = json::<ConversationPage>(
            r#"{"availability":{"can_send":false,"reason":"disabled"},"messages":[{"created_at":"2026-10-09T15:00:00Z","id":"00000000-0000-0000-0000-000000000001","revision":1,"role":"user","sequence":1,"state":"complete","text":"How did I sleep?","updated_at":"2026-10-09T15:00:00Z"}],"revision":2}"#,
        );
        assert!(!composer_open(&loaded));
        assert_eq!(
            closed_send(&loaded),
            Some(ClosedSend {
                title: "Chat with Duet",
                message: "Chat is turned off in Settings."
            })
        );
    }

    #[test]
    fn a_failed_send_keeps_the_draft_and_its_id() {
        let draft = "How did I sleep?";
        assert_eq!(draft_after_send(draft, false), draft);
        assert_eq!(draft_after_send(draft, true), "");
        let id = uuid::Uuid::nil();
        let fresh = uuid::Uuid::from_u128(1);
        assert_eq!(send_id(Some((id, draft)), draft, fresh), id);
        assert_eq!(send_id(Some((id, draft)), "A different note", fresh), fresh);
        assert_eq!(send_id(None, draft, fresh), fresh);
    }

    #[test]
    fn an_accepted_send_shows_the_user_line_without_inventing_a_reply() {
        let page = json::<ConversationPage>(
            r#"{"availability":{"can_send":true},"messages":[],"revision":1}"#,
        );
        let accepted = json::<ConversationMessage>(
            r#"{"created_at":"2026-10-09T15:00:00Z","id":"00000000-0000-0000-0000-000000000001","revision":1,"role":"user","sequence":1,"state":"complete","text":"How did I sleep?","updated_at":"2026-10-09T15:00:00Z"}"#,
        );
        let shown = page_with_accepted(page, accepted.clone());
        assert_eq!(
            conversation_root(&shown, &[], &[], None),
            ConversationRoot::Messages(vec![ConversationLine {
                role: "You".into(),
                text: "How did I sleep?".into(),
                from_duet: false,
                notes: vec![],
                sources: vec![],
            }])
        );
        let again = page_with_accepted(shown, accepted);
        assert_eq!(again.messages.len(), 1);
    }

    #[test]
    fn today_quiet_paused_and_loaded() {
        let quiet_cycle = r#"{"context_snapshot_id":null,"first_weekday":1,"generated_at":"2026-10-09T15:00:00Z","id":"00000000-0000-0000-0000-000000000003","insights":[],"kind":"daily","observed_through":"2026-10-09T15:00:00Z","timezone":"America/Los_Angeles","window_end":"2026-10-10T07:00:00Z","window_start":"2026-10-09T07:00:00Z"}"#;
        let enabled = json::<IntelligenceSettings>(r#"{"enabled":true}"#);
        let quiet = json::<InsightCyclePage>(&format!(r#"{{"items":[{quiet_cycle}]}}"#));
        assert_eq!(today_root(&enabled, &quiet), TodayRoot::Quiet);
        let paused_settings = json::<IntelligenceSettings>(r#"{"enabled":false}"#);
        assert_eq!(today_root(&paused_settings, &quiet), TodayRoot::Paused);
        let loaded = json::<InsightCyclePage>(
            r#"{"items":[{"context_snapshot_id":null,"first_weekday":1,"generated_at":"2026-10-09T15:00:00Z","id":"00000000-0000-0000-0000-000000000003","insights":[{"detail":"Walking has been steady this week.","evidence":[],"id":"00000000-0000-0000-0000-000000000004","title":"Steady walks"}],"kind":"daily","observed_through":"2026-10-09T15:00:00Z","timezone":"America/Los_Angeles","window_end":"2026-10-10T07:00:00Z","window_start":"2026-10-09T07:00:00Z"}]}"#,
        );
        assert_eq!(
            today_root(&paused_settings, &loaded),
            TodayRoot::Insights(vec![InsightCard {
                title: "Steady walks".into(),
                sentence: "Walking has been steady this week.".into()
            }])
        );
        assert_eq!(CHECK_UNAVAILABLE, "Duet couldn’t check right now");
    }

    #[test]
    fn library_empty_and_sections() {
        let empty = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{},"entries":[],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":0}"#,
        );
        assert_eq!(library_root(&empty), LibraryRoot::Empty);
        let loaded = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{"medication":1,"lab":2},"entries":[],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":3}"#,
        );
        assert_eq!(
            library_root(&loaded),
            LibraryRoot::Sections(vec![
                LibrarySection {
                    name: "Medications",
                    measured: false,
                    records: vec![],
                },
                LibrarySection {
                    name: "Recent labs",
                    measured: true,
                    records: vec![],
                },
            ])
        );
        let listed = json::<EvidenceLibraryPage>(
            r#"{"catalog_revision":null,"counts":{"medication":1},"entries":[{"category":"medication","coding_identities":[],"condition_status_conflict":false,"date":"2026-10-01T15:00:00Z","entity":"health_evidence","group_kind":null,"id":"med-1","member_count":1,"preview":null,"record_id":"00000000-0000-0000-0000-000000000021","source_label":"Pharmacy","status_conflict":false,"summary":"10 mg","title":"Lisinopril"}],"health_records_generation":0,"next_cursor":null,"revision":1,"total_count":1}"#,
        );
        assert_eq!(
            library_root(&listed),
            LibraryRoot::Sections(vec![LibrarySection {
                name: "Medications",
                measured: false,
                records: vec![LibraryRecord {
                    id: "med-1".into(),
                    kind: LibraryKind::HealthEvidence,
                    title: "Lisinopril".into(),
                    summary: Some("10 mg".into()),
                    when: Some("October 1, 2026".into()),
                    source: Some("Pharmacy".into()),
                }],
            }])
        );
        assert_eq!(
            LIBRARY_UNAVAILABLE,
            "Your health information isn’t available right now. Try again in a moment."
        );
    }

    #[test]
    fn notification_controls_name_their_permission() {
        assert!(ALLOW_NOTIFICATIONS.contains("notifications"));
        assert_eq!(INSIGHT_NOTIFICATIONS, "New insight notifications");
        assert_ne!(INSIGHT_NOTIFICATIONS, ALLOW_NOTIFICATIONS);
        assert!(INSIGHT_NOTIFICATIONS_DETAIL.contains("worth showing"));
        assert_eq!(OPEN_NOTIFICATION_SETTINGS, "Open iPhone Settings");
    }

    #[test]
    fn android_debug_api_reaches_the_emulator_host() {
        let host = if cfg!(target_os = "android") {
            "10.0.2.2"
        } else {
            "127.0.0.1"
        };
        assert!(api_base_url().contains(host) || option_env!("DUET_API_URL").is_some());
    }

    #[test]
    fn doctors_plan_section_is_read_only_and_labeled_your_doctors_plan() {
        let plan = DuetDoctorPlanView {
            label: YOUR_DOCTORS_PLAN.into(),
            read_only: true,
            practice_id: "practice-coda-1".into(),
            patient_workspace_id: "pat-elena".into(),
            physician_name: "Dr. Rivera".into(),
            version: 1,
            approved_at: "2026-10-13T10:00:00Z".into(),
            items: vec![duet_client::DoctorPlanItemView {
                id: "item-1".into(),
                title: "Repeat lipid panel".into(),
                body: "Fasting lipid panel in 6 weeks.".into(),
            }],
        };
        let section = doctor_plan_section(Some(&plan)).expect("plan section");
        assert_eq!(section.heading, "your doctor's plan");
        assert!(section.read_only);
        assert_eq!(section.attribution, "From Dr. Rivera · v1");
        assert_eq!(section.items.len(), 1);
    }
}

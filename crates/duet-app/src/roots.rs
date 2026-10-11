//! Root states for the four Duet tabs. Sentences are the ones the iPhone app shows.

use duet_client::{
    ConversationPage, EvidenceLibraryPage, InsightCyclePage, IntelligenceSettings,
    SharingOverviewResponse, SharingPeer,
};
use duet_wire::models::ConversationMessage;
use duet_wire::models::conversation_availability::Reason;
use duet_wire::models::conversation_message::Role;

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
pub const NOT_SHARING: &str = "You’re not sharing yet.";
pub const SHARING_UNCHECKED: &str = "Duet couldn’t check whether sharing is still allowed.";
pub const SHARING_SAVED: &str = "Duet couldn’t check whether sharing is still allowed. Saved values stay visible for up to 24 hours after the last successful check.";
pub const TRY_AGAIN: &str = "Try again";
pub const AWAITING: &str = "Awaiting acceptance";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationLine {
    pub role: String,
    pub text: String,
    pub from_duet: bool,
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

/// The text a send uses. The field wins when it has something in it: on Android
/// the controlled value can drop keystrokes while the person still sees them.
pub fn composer_text(field: Option<&str>, signal: &str) -> String {
    match field {
        Some(text) if !text.trim().is_empty() => text.to_owned(),
        _ => signal.to_owned(),
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

pub fn conversation_root(page: &ConversationPage) -> ConversationRoot {
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
            })
            .collect(),
    )
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryRecord {
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
        title: entry.title.clone(),
        summary: supplied(&entry.summary),
        when,
        source: supplied(&entry.source_label),
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedNotice {
    NotSharing,
    CouldNotCheck,
    SavedStayVisible,
    AwaitingAcceptance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedRoot {
    pub people: Vec<String>,
    pub notice: Option<SharedNotice>,
}

pub fn people_in(overview: &SharingOverviewResponse) -> Vec<String> {
    let mut people = Vec::new();
    for grants in [
        &overview.incoming,
        &overview.outgoing,
        &overview.pending_incoming,
        &overview.pending_outgoing,
        &overview.reconnect_required,
    ] {
        if let Some(grants) = grants {
            for grant in grants {
                people.push(peer_name(&grant.peer));
            }
        }
    }
    people
}

fn peer_name(peer: &SharingPeer) -> String {
    peer.display_name
        .clone()
        .flatten()
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| peer.account_label.clone())
}

pub fn shared_root(
    previous: &[String],
    result: Result<&SharingOverviewResponse, ()>,
) -> SharedRoot {
    match result {
        Ok(overview) => {
            let people = people_in(overview);
            let invitations = overview.invitations.as_ref().map(Vec::len).unwrap_or(0);
            if people.is_empty() && invitations == 0 {
                SharedRoot {
                    people,
                    notice: Some(SharedNotice::NotSharing),
                }
            } else if people.is_empty() {
                SharedRoot {
                    people,
                    notice: Some(SharedNotice::AwaitingAcceptance),
                }
            } else {
                SharedRoot {
                    people,
                    notice: None,
                }
            }
        }
        Err(()) if previous.is_empty() => SharedRoot {
            people: Vec::new(),
            notice: Some(SharedNotice::CouldNotCheck),
        },
        Err(()) => SharedRoot {
            people: previous.to_vec(),
            notice: Some(SharedNotice::SavedStayVisible),
        },
    }
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
    match option_env!("DUET_API_URL") {
        Some(url) if !url.is_empty() => url.to_owned(),
        _ => default_api_base().to_owned(),
    }
}

/// Returns the bearer this build sends.
pub fn api_bearer() -> String {
    option_env!("DUET_BEARER").unwrap_or("").to_owned()
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
            conversation_root(&empty),
            ConversationRoot::Empty {
                title: CHAT_EMPTY_TITLE,
                message: CHAT_EMPTY_MESSAGE
            }
        );
        let loaded = json::<ConversationPage>(
            r#"{"availability":{"can_send":true},"messages":[{"created_at":"2026-10-09T15:00:00Z","id":"00000000-0000-0000-0000-000000000001","revision":1,"role":"user","sequence":1,"state":"complete","text":"How did I sleep?","updated_at":"2026-10-09T15:00:00Z"},{"created_at":"2026-10-09T15:00:01Z","id":"00000000-0000-0000-0000-000000000002","revision":1,"role":"assistant","sequence":2,"state":"complete","text":"About seven hours.","updated_at":"2026-10-09T15:00:01Z"}],"revision":2}"#,
        );
        assert_eq!(
            conversation_root(&loaded),
            ConversationRoot::Messages(vec![
                ConversationLine {
                    role: "You".into(),
                    text: "How did I sleep?".into(),
                    from_duet: false
                },
                ConversationLine {
                    role: "Duet".into(),
                    text: "About seven hours.".into(),
                    from_duet: true
                },
            ])
        );
        assert!(composer_open(&empty));
        assert!(composer_open(&loaded));
        assert_eq!(closed_send(&empty), None);
        assert_eq!(closed_send(&loaded), None);
    }

    #[test]
    fn a_closed_page_hides_the_composer_and_keeps_the_reason() {
        let empty = json::<ConversationPage>(
            r#"{"availability":{"can_send":false,"reason":"daily_limit"},"messages":[],"revision":1}"#,
        );
        assert!(!composer_open(&empty));
        assert_eq!(closed_send(&empty), None);
        assert_eq!(
            conversation_root(&empty),
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
        assert_eq!(composer_text(Some("Hello"), ""), "Hello");
        assert_eq!(composer_text(Some("  "), draft), draft);
        assert_eq!(composer_text(None, draft), draft);
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
            conversation_root(&shown),
            ConversationRoot::Messages(vec![ConversationLine {
                role: "You".into(),
                text: "How did I sleep?".into(),
                from_duet: false
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
    fn shared_people_stay_when_a_later_read_fails() {
        let empty = json::<SharingOverviewResponse>(r#"{"account_label":"Ada"}"#);
        assert_eq!(
            shared_root(&[], Ok(&empty)),
            SharedRoot {
                people: vec![],
                notice: Some(SharedNotice::NotSharing)
            }
        );
        let loaded = json::<SharingOverviewResponse>(
            r#"{"account_label":"me","outgoing":[{"connection_id":"00000000-0000-0000-0000-000000000010","created_at":"2026-10-09T15:00:00Z","grant_id":"00000000-0000-0000-0000-000000000011","peer":{"account_label":"ada@example","display_name":"Ada","user_id":"00000000-0000-0000-0000-000000000012"},"recipient_key_version":1,"recipient_user_id":"00000000-0000-0000-0000-000000000012","revision":1,"sender_key_version":1,"sender_user_id":"00000000-0000-0000-0000-000000000013","state":"active","updated_at":"2026-10-09T15:00:00Z"}]}"#,
        );
        let people = people_in(&loaded);
        assert_eq!(people, vec!["Ada".to_owned()]);
        assert_eq!(
            shared_root(&people, Err(())),
            SharedRoot {
                people: vec!["Ada".into()],
                notice: Some(SharedNotice::SavedStayVisible)
            }
        );
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
}

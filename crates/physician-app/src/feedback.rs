//! The physician's feedback on Coda's responses in the open chat: a thumb
//! under each response and, once it is rated, optional details in a dialog.
//!
//! The shell keeps one [`Desk`] for the open chat. It reads the chat's saved
//! feedback once per chat and again whenever the conversation is reread, and
//! holds one [`RatingState`] per response. A save carries its chat, so an
//! answer for a chat no longer open changes nothing. The thumbs show the
//! saved rating, or the choice while it saves; a failed save says so and
//! shows the saved rating again. A failed read never shows a response as
//! unrated: its thumbs stay disabled and say the rating couldn't be loaded.
//! Only hosted chats carry the places feedback names, so other sites show no
//! controls.
use std::collections::{BTreeMap, BTreeSet};

use dioxus::prelude::*;
use physician_client::PhysicianClient;
use physician_wire::{
    ChatFeedback, FeedbackDetails, FeedbackRating, FeedbackReason, ResponseFeedback,
};
use ui::{ActionRow, Button, ButtonVariant, Dialog, TextField, TextKind, ToggleChip};

const RATE: &str = "Rate this response";
const GOOD: &str = "Good response";
const POOR: &str = "Poor response";
const ACTIONS: &str = "Response actions";
const ADD_DETAILS: &str = "Add details";
const EDIT_DETAILS: &str = "Edit details";
const DETAILS_TITLE: &str = "Share feedback";
const DETAILS_SUBTITLE: &str = "What would have made this response better?";
const DETAILS_LABEL: &str = "Details";
const SAVE_DETAILS: &str = "Save details";
const SAVING_DETAILS: &str = "Saving details";
const CLOSE: &str = "Close";
pub(crate) const NOT_SAVED: &str = "Couldn’t save";
pub(crate) const NOT_LOADED: &str = "Couldn’t load your rating";

/// The reasons a physician can pick, in the order the dialog shows them.
const REASONS: [(FeedbackReason, &str); 4] = [
    (
        FeedbackReason::IncorrectInformation,
        "Incorrect information",
    ),
    (FeedbackReason::MissingContext, "Missing context"),
    (FeedbackReason::HardToFollow, "Hard to follow"),
    (FeedbackReason::NotUseful, "Not useful"),
];

/// The chat feedback belongs to: one patient workspace and chat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChatKey {
    pub(crate) workspace: String,
    pub(crate) chat: String,
}

/// One response's rating as the page knows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RatingState {
    /// The chat's feedback is being read.
    Loading,
    /// The read failed: the saved rating is not known.
    Unknown,
    /// Read, and not rated.
    Unrated,
    /// A save of `choice` is on its way over the `saved` rating.
    Saving {
        choice: FeedbackRating,
        saved: Option<FeedbackRating>,
    },
    /// Saved.
    Rated(FeedbackRating),
    /// The save of `choice` failed; `saved` still stands.
    Failed {
        choice: FeedbackRating,
        saved: Option<FeedbackRating>,
    },
}

impl RatingState {
    /// The rating the saved state holds, when it is known.
    fn saved(self) -> Option<FeedbackRating> {
        match self {
            Self::Rated(rating) => Some(rating),
            Self::Saving { saved, .. } | Self::Failed { saved, .. } => saved,
            Self::Loading | Self::Unknown | Self::Unrated => None,
        }
    }

    /// The saving state for pressing `choice`, or `None` when the press
    /// starts no save: the rating isn't known, a save is on its way, or the
    /// choice is already saved.
    pub(crate) fn choose(self, choice: FeedbackRating) -> Option<Self> {
        match self {
            Self::Loading | Self::Unknown | Self::Saving { .. } => None,
            Self::Rated(rating) if rating == choice => None,
            Self::Unrated | Self::Rated(_) | Self::Failed { .. } => Some(Self::Saving {
                choice,
                saved: self.saved(),
            }),
        }
    }

    /// The state after the save answered: the saved rating, or failed.
    pub(crate) fn answered(self, saved: Option<FeedbackRating>) -> Self {
        let Self::Saving {
            choice,
            saved: before,
        } = self
        else {
            return self;
        };
        match saved {
            Some(rating) => Self::Rated(rating),
            None => Self::Failed {
                choice,
                saved: before,
            },
        }
    }

    /// The thumb shown pressed: the choice while it saves, else the saved one.
    pub(crate) fn shown(self) -> Option<FeedbackRating> {
        match self {
            Self::Saving { choice, .. } => Some(choice),
            _ => self.saved(),
        }
    }

    /// What the controls say beside the thumbs, if anything.
    pub(crate) fn notice(self) -> Option<&'static str> {
        match self {
            Self::Failed { .. } => Some(NOT_SAVED),
            Self::Unknown => Some(NOT_LOADED),
            _ => None,
        }
    }
}

/// One response's feedback on the page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) rating: RatingState,
    pub(crate) details: Option<FeedbackDetails>,
    /// The desk's clock when the page last changed this entry itself.
    changed: u64,
}

/// When a read of the chat's feedback started, from [`Desk::read_started`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReadStart(u64);

/// The open chat's feedback.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Desk {
    chat: Option<ChatKey>,
    /// `None` until the first read answers; `Some(false)` when it failed.
    read: Option<bool>,
    entries: BTreeMap<usize, Entry>,
    /// Counts the page's own changes, so a read older than one never undoes it.
    clock: u64,
    /// The clock when this chat was opened; reads started before it are of an earlier visit.
    opened: u64,
}

impl Desk {
    /// Opens `chat`. Another chat starts over; the same chat keeps what it
    /// knows while it is read again.
    pub(crate) fn open(&mut self, chat: Option<ChatKey>) {
        if self.chat != chat {
            let opened = self.tick();
            *self = Self {
                chat,
                clock: opened,
                opened,
                ..Self::default()
            };
        }
    }

    /// The open chat.
    pub(crate) fn chat(&self) -> Option<&ChatKey> {
        self.chat.as_ref()
    }

    /// Marks a read of the chat's feedback as starting now.
    pub(crate) fn read_started(&self) -> ReadStart {
        ReadStart(self.clock)
    }

    fn tick(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }

    /// A read of `chat`, started at `started`, answered. A failed reread
    /// keeps what was read before. A save on its way, and anything the page
    /// changed after the read started, are not overwritten.
    pub(crate) fn read_answered(
        &mut self,
        chat: &ChatKey,
        started: ReadStart,
        answer: Result<ChatFeedback, ()>,
    ) {
        if self.chat.as_ref() != Some(chat) || started.0 < self.opened {
            return;
        }
        let Ok(read) = answer else {
            self.read.get_or_insert(false);
            return;
        };
        self.read = Some(true);
        let saved: BTreeMap<_, _> = read
            .feedback
            .into_iter()
            .map(|feedback| (feedback.response, feedback))
            .collect();
        let kept = |entry: &Entry| {
            entry.changed > started.0 || matches!(entry.rating, RatingState::Saving { .. })
        };
        self.entries
            .retain(|response, entry| saved.contains_key(response) || kept(entry));
        for (response, feedback) in saved {
            let entry = self.entries.entry(response).or_insert(Entry {
                rating: RatingState::Unrated,
                details: None,
                changed: 0,
            });
            if !kept(entry) {
                entry.rating = RatingState::Rated(feedback.rating);
                entry.details = feedback.details;
            }
        }
    }

    /// One response's feedback as the page shows it.
    pub(crate) fn entry(&self, response: usize) -> Entry {
        if let Some(entry) = self.entries.get(&response) {
            return entry.clone();
        }
        Entry {
            rating: match self.read {
                None => RatingState::Loading,
                Some(false) => RatingState::Unknown,
                Some(true) => RatingState::Unrated,
            },
            details: None,
            changed: 0,
        }
    }

    /// Presses `choice` on a response; answers the chat to save to when the
    /// press starts a save.
    pub(crate) fn choose(&mut self, response: usize, choice: FeedbackRating) -> Option<ChatKey> {
        let chat = self.chat.clone()?;
        let current = self.entry(response);
        let saving = current.rating.choose(choice)?;
        let changed = self.tick();
        self.entries.insert(
            response,
            Entry {
                rating: saving,
                details: current.details,
                changed,
            },
        );
        Some(chat)
    }

    /// A rating save on `chat` answered with the saved feedback, or failed.
    pub(crate) fn rating_answered(
        &mut self,
        chat: &ChatKey,
        response: usize,
        answer: Result<ResponseFeedback, ()>,
    ) {
        if self.chat.as_ref() != Some(chat) {
            return;
        }
        let changed = self.tick();
        let Some(entry) = self.entries.get_mut(&response) else {
            return;
        };
        entry.changed = changed;
        match answer {
            Ok(saved) => {
                entry.rating = entry.rating.answered(Some(saved.rating));
                entry.details = saved.details;
            }
            Err(()) => entry.rating = entry.rating.answered(None),
        }
    }

    /// A details save on `chat` answered with the saved feedback.
    pub(crate) fn details_saved(
        &mut self,
        chat: &ChatKey,
        response: usize,
        saved: ResponseFeedback,
    ) {
        if self.chat.as_ref() != Some(chat) {
            return;
        }
        let changed = self.tick();
        let entry = self.entries.entry(response).or_insert(Entry {
            rating: RatingState::Rated(saved.rating),
            details: None,
            changed,
        });
        entry.changed = changed;
        if !matches!(entry.rating, RatingState::Saving { .. }) {
            entry.rating = RatingState::Rated(saved.rating);
        }
        entry.details = saved.details;
    }
}

/// The open chat's feedback and the client its saves go through.
#[derive(Clone, Copy)]
struct Feedback {
    desk: Signal<Desk>,
    client: Signal<PhysicianClient>,
}

/// Keeps the open chat's feedback for the [`ResponseRating`] controls
/// below it, read again whenever `reread` moves.
pub(crate) fn use_feedback(
    client: Signal<PhysicianClient>,
    workspace: Signal<Option<String>>,
    chat: Signal<Option<String>>,
    reread: Signal<u32>,
) {
    let mut desk = use_signal(Desk::default);
    use_context_provider(|| Feedback { desk, client });
    use_effect(move || {
        let _reread = reread();
        let open = workspace()
            .zip(chat())
            .map(|(workspace, chat)| ChatKey { workspace, chat });
        desk.write().open(open.clone());
        let Some(open) = open else {
            return;
        };
        let client = client.peek().clone();
        let started = desk.peek().read_started();
        spawn(async move {
            let answer = client
                .chat_feedback(&open.workspace, &open.chat)
                .await
                .map_err(|_| ());
            desk.write().read_answered(&open, started, answer);
        });
    });
}

/// The thumbs and details under one of Coda's responses, named by its place.
#[component]
pub(crate) fn ResponseRating(response: usize) -> Element {
    let Feedback { mut desk, client } = use_context::<Feedback>();
    let mut open = use_signal(|| false);
    let entry = desk.read().entry(response);
    let rating = entry.rating;
    let rate = move |choice: ui::Rating| {
        let choice = match choice {
            ui::Rating::Up => FeedbackRating::Up,
            ui::Rating::Down => FeedbackRating::Down,
        };
        let Some(chat) = desk.write().choose(response, choice) else {
            return;
        };
        let client = client.peek().clone();
        spawn(async move {
            let answer = client
                .rate_response(&chat.workspace, &chat.chat, response, choice)
                .await
                .map_err(|_| ());
            desk.write().rating_answered(&chat, response, answer);
        });
    };
    let selected = rating.shown().map(|rating| match rating {
        FeedbackRating::Up => ui::Rating::Up,
        FeedbackRating::Down => ui::Rating::Down,
    });
    let busy = matches!(rating, RatingState::Saving { .. });
    rsx! {
        ui::MessageActions { label: ACTIONS, shown: rating.saved().is_some() || rating.notice().is_some() || busy,
            ui::FeedbackRating {
                selected,
                label: RATE,
                up_label: GOOD,
                down_label: POOR,
                busy,
                disabled: matches!(rating, RatingState::Loading | RatingState::Unknown),
                on_rate: rate,
            }
            if rating.saved().is_some() {
                Button {
                    variant: ButtonVariant::Quiet,
                    compact: true,
                    class: "feedback-details-open",
                    onclick: move |_| open.set(true),
                    if entry.details.is_some() { "{EDIT_DETAILS}" } else { "{ADD_DETAILS}" }
                }
            }
            if let Some(notice) = rating.notice() {
                span { class: "quiet-line feedback-notice", role: "status", "{notice}" }
            }
        }
        if open() {
            DetailsDialog {
                response,
                saved: entry.details.clone().unwrap_or_default(),
                on_close: move |_| open.set(false),
            }
        }
    }
}

/// The details dialog: reasons, the physician's words and its own Save.
/// Closing it without saving leaves the rating and any saved details.
#[component]
fn DetailsDialog(response: usize, saved: FeedbackDetails, on_close: EventHandler) -> Element {
    let Feedback { mut desk, client } = use_context::<Feedback>();
    let initial = saved.clone();
    let mut reasons = use_signal(|| initial.reasons.iter().copied().collect::<BTreeSet<_>>());
    let mut text = use_signal(|| initial.text.clone());
    let mut saving = use_signal(|| false);
    let mut failed = use_signal(|| false);
    let save = move |_| {
        let Some(chat) = desk.peek().chat().cloned() else {
            return;
        };
        let details = FeedbackDetails {
            reasons: reasons.peek().iter().copied().collect(),
            text: text.peek().clone(),
        };
        saving.set(true);
        failed.set(false);
        let client = client.peek().clone();
        spawn(async move {
            let answer = client
                .save_feedback_details(&chat.workspace, &chat.chat, response, &details)
                .await;
            saving.set(false);
            match answer {
                Ok(saved) => {
                    desk.write().details_saved(&chat, response, saved);
                    on_close.call(());
                }
                Err(_) => failed.set(true),
            }
        });
    };
    rsx! {
        Dialog {
            class: "feedback-details",
            title: DETAILS_TITLE,
            subtitle: DETAILS_SUBTITLE,
            close_label: CLOSE,
            on_close: move |_| on_close.call(()),
            div { class: "feedback-reasons",
                for (reason, label) in REASONS {
                    ToggleChip {
                        key: "{label}",
                        pressed: reasons.read().contains(&reason),
                        disabled: saving(),
                        onclick: move |_| {
                            let mut chosen = reasons.write();
                            if !chosen.remove(&reason) {
                                chosen.insert(reason);
                            }
                        },
                        "{label}"
                    }
                }
            }
            TextField {
                label: DETAILS_LABEL,
                kind: TextKind::Multiline,
                value: text(),
                on_input: move |words| text.set(words),
                disabled: saving(),
            }
            ActionRow {
                if failed() {
                    span { class: "quiet-line feedback-notice", role: "status", "{NOT_SAVED}" }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    busy: saving(),
                    pending_label: SAVING_DETAILS,
                    onclick: save,
                    "{SAVE_DETAILS}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use FeedbackRating::{Down, Up};

    fn chat(name: &str) -> ChatKey {
        ChatKey {
            workspace: "ws".into(),
            chat: name.into(),
        }
    }

    fn saved(response: usize, rating: FeedbackRating) -> ResponseFeedback {
        ResponseFeedback {
            response,
            rating,
            details: None,
        }
    }

    fn read(desk: &mut Desk, feedback: Vec<ResponseFeedback>) {
        let started = desk.read_started();
        desk.read_answered(&chat("a"), started, Ok(ChatFeedback { feedback }));
    }

    fn read_fails(desk: &mut Desk) {
        let started = desk.read_started();
        desk.read_answered(&chat("a"), started, Err(()));
    }

    #[test]
    fn a_read_started_before_a_save_never_undoes_it() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        let stale = desk.read_started();
        desk.choose(1, Down).unwrap();
        desk.rating_answered(&chat("a"), 1, Ok(saved(1, Down)));
        desk.choose(2, Up).unwrap();
        desk.rating_answered(&chat("a"), 2, Ok(saved(2, Up)));
        desk.read_answered(
            &chat("a"),
            stale,
            Ok(ChatFeedback {
                feedback: vec![saved(1, Up)],
            }),
        );
        assert_eq!(desk.entry(1).rating, RatingState::Rated(Down));
        assert_eq!(desk.entry(2).rating, RatingState::Rated(Up));
        // A read started after the saves shows what the store holds.
        read(&mut desk, vec![saved(1, Up)]);
        assert_eq!(desk.entry(1).rating, RatingState::Rated(Up));
        assert_eq!(desk.entry(2).rating, RatingState::Unrated);
    }

    #[test]
    fn a_rating_saves_shows_the_choice_and_lands_saved() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        assert_eq!(desk.entry(1).rating, RatingState::Loading);
        assert_eq!(
            desk.choose(1, Up),
            None,
            "nothing to save over before the read"
        );
        read(&mut desk, vec![]);
        assert_eq!(desk.entry(1).rating, RatingState::Unrated);
        assert_eq!(desk.choose(1, Up), Some(chat("a")));
        let saving = desk.entry(1).rating;
        assert_eq!((saving.shown(), saving.notice()), (Some(Up), None));
        assert_eq!(desk.choose(1, Down), None, "one save at a time");
        desk.rating_answered(&chat("a"), 1, Ok(saved(1, Up)));
        assert_eq!(desk.entry(1).rating, RatingState::Rated(Up));
        assert_eq!(desk.choose(1, Up), None, "already saved");
        assert_eq!(desk.entry(3).rating, RatingState::Unrated);
    }

    #[test]
    fn a_failed_save_says_so_and_shows_the_saved_rating_again() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        desk.choose(1, Down).unwrap();
        desk.rating_answered(&chat("a"), 1, Err(()));
        let failed = desk.entry(1).rating;
        assert_eq!(
            failed,
            RatingState::Failed {
                choice: Down,
                saved: Some(Up)
            }
        );
        assert_eq!(
            (failed.shown(), failed.notice()),
            (Some(Up), Some(NOT_SAVED))
        );
        // Pressing again retries over the same saved rating.
        desk.choose(1, Down).unwrap();
        desk.rating_answered(&chat("a"), 1, Ok(saved(1, Down)));
        assert_eq!(desk.entry(1).rating, RatingState::Rated(Down));

        let unrated = RatingState::Unrated.choose(Up).unwrap().answered(None);
        assert_eq!((unrated.shown(), unrated.notice()), (None, Some(NOT_SAVED)));
    }

    #[test]
    fn a_failed_read_is_unknown_never_unrated() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read_fails(&mut desk);
        let unknown = desk.entry(1).rating;
        assert_eq!(unknown, RatingState::Unknown);
        assert_eq!(
            (unknown.shown(), unknown.notice()),
            (None, Some(NOT_LOADED))
        );
        assert_eq!(desk.choose(1, Up), None);
        // A later read that works shows the saved rating.
        read(&mut desk, vec![saved(1, Down)]);
        assert_eq!(desk.entry(1).rating, RatingState::Rated(Down));
        // A failed reread keeps what was read.
        read_fails(&mut desk);
        assert_eq!(desk.entry(1).rating, RatingState::Rated(Down));
        assert_eq!(desk.entry(3).rating, RatingState::Unrated);
    }

    #[test]
    fn answers_for_another_chat_change_nothing() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![]);
        desk.choose(1, Up).unwrap();
        let earlier = desk.read_started();
        desk.open(Some(chat("b")));
        desk.rating_answered(&chat("a"), 1, Ok(saved(1, Up)));
        let started = desk.read_started();
        desk.read_answered(
            &chat("a"),
            started,
            Ok(ChatFeedback {
                feedback: vec![saved(1, Up)],
            }),
        );
        assert_eq!(desk.entry(1).rating, RatingState::Loading);
        assert_eq!(desk.chat(), Some(&chat("b")));

        // Back on chat a, a read from the earlier visit is out of date.
        desk.open(Some(chat("a")));
        desk.read_answered(
            &chat("a"),
            earlier,
            Ok(ChatFeedback {
                feedback: vec![saved(1, Up)],
            }),
        );
        assert_eq!(desk.entry(1).rating, RatingState::Loading);
    }

    #[test]
    fn a_reread_keeps_a_save_on_its_way_and_details_follow_the_rating() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        desk.choose(1, Down).unwrap();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        assert!(matches!(
            desk.entry(1).rating,
            RatingState::Saving { choice: Down, .. }
        ));
        let details = FeedbackDetails {
            reasons: vec![FeedbackReason::MissingContext],
            text: "Sodium was rechecked.".into(),
        };
        desk.rating_answered(
            &chat("a"),
            1,
            Ok(ResponseFeedback {
                details: Some(details.clone()),
                ..saved(1, Down)
            }),
        );
        assert_eq!(desk.entry(1).details, Some(details.clone()));
        desk.details_saved(
            &chat("a"),
            1,
            ResponseFeedback {
                details: None,
                ..saved(1, Down)
            },
        );
        assert_eq!(desk.entry(1).details, None);
        assert_eq!(desk.entry(1).rating, RatingState::Rated(Down));
    }
}

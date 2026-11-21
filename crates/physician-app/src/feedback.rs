//! The physician's feedback on Coda's responses: a thumb under each response
//! and, once it is rated, optional details in a dialog.
//!
//! The chat snapshot carries each response's saved feedback. When the
//! snapshot couldn't read feedback it still carries the conversation and
//! says so ([`Read::Unavailable`]): a response the page neither saved nor
//! read feedback on then shows disabled thumbs saying its rating couldn't be
//! loaded, rather than showing as unrated. The page keeps one [`Desk`] for
//! every chat: one [`State`] per response it saved or is saving, and the
//! newest feedback a snapshot read, kept across chat switches. Every saved
//! feedback carries the server's revision, and what shows is the highest
//! revision seen, from the snapshot or a save's answer, so an older answer
//! never replaces a newer one. One save replaces the rating and details
//! together, one at a time per response. Only hosted chats name each
//! response by its response id, which feedback keys on, so other sites show
//! no controls.
use std::collections::{BTreeMap, BTreeSet};

use crate::SessionState;
use dioxus::core::{Runtime, current_scope_id};
use dioxus::prelude::*;
use physician_client::PhysicianClient;
use physician_wire::{
    FeedbackDetails, FeedbackRating, FeedbackReason, ResponseFeedback, ResponseId, SaveFeedback,
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ChatKey {
    pub(crate) workspace: String,
    pub(crate) chat: String,
}

/// One response: its chat and its response id.
type Key = (ChatKey, ResponseId);

/// The feedback a response's snapshot turn carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Read {
    /// The snapshot read the physician's feedback: this response's, or
    /// `None` when it is unrated.
    Loaded(Option<ResponseFeedback>),
    /// The snapshot couldn't read feedback, so this response may be rated.
    Unavailable,
}

impl Read {
    /// The saved feedback the snapshot read, if any.
    fn saved(&self) -> Option<&ResponseFeedback> {
        match self {
            Self::Loaded(saved) => saved.as_ref(),
            Self::Unavailable => None,
        }
    }
}

/// What one response's controls show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Controls {
    /// Its feedback couldn't be read and this page saved none: the thumbs
    /// are disabled and say so, since unrated might not be true.
    Unavailable,
    /// Unrated thumbs.
    Unrated,
    /// The feedback the page knows.
    Rated(State),
}

/// What one save sends: the rating and the details together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Draft {
    pub(crate) rating: FeedbackRating,
    pub(crate) details: Option<FeedbackDetails>,
}

/// One response's feedback as the page shows it. `over` is the newest saved
/// feedback known while a save is on its way or after it failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum State {
    Saved(ResponseFeedback),
    Saving {
        draft: Draft,
        over: Option<ResponseFeedback>,
    },
    Failed {
        draft: Draft,
        over: Option<ResponseFeedback>,
    },
}

/// The higher revision of two saved feedbacks.
fn newest(a: Option<&ResponseFeedback>, b: Option<&ResponseFeedback>) -> Option<ResponseFeedback> {
    match (a, b) {
        (Some(a), Some(b)) if b.revision > a.revision => Some(b.clone()),
        (Some(a), _) => Some(a.clone()),
        (None, b) => b.cloned(),
    }
}

impl State {
    /// The newest saved feedback this state knows.
    pub(crate) fn saved(&self) -> Option<&ResponseFeedback> {
        match self {
            Self::Saved(saved) => Some(saved),
            Self::Saving { over, .. } | Self::Failed { over, .. } => over.as_ref(),
        }
    }

    /// This state with `read`, the snapshot's feedback, merged in by revision.
    fn with(&self, read: Option<&ResponseFeedback>) -> Self {
        let over = newest(self.saved(), read);
        match self {
            Self::Saved(saved) => Self::Saved(over.unwrap_or_else(|| saved.clone())),
            Self::Saving { draft, .. } => Self::Saving {
                draft: draft.clone(),
                over,
            },
            Self::Failed { draft, .. } => Self::Failed {
                draft: draft.clone(),
                over,
            },
        }
    }

    pub(crate) fn busy(&self) -> bool {
        matches!(self, Self::Saving { .. })
    }

    /// The thumb shown pressed: the rating on its way, else the saved one.
    pub(crate) fn shown(&self) -> Option<FeedbackRating> {
        match self {
            Self::Saving { draft, .. } => Some(draft.rating),
            _ => self.saved().map(|saved| saved.rating),
        }
    }

    /// What the controls say beside the thumbs, if anything.
    pub(crate) fn notice(&self) -> Option<&'static str> {
        matches!(self, Self::Failed { .. }).then_some(NOT_SAVED)
    }

    /// The details the dialog opens with: those on their way or that failed
    /// to save, else the saved ones.
    pub(crate) fn draft_details(&self) -> FeedbackDetails {
        match self {
            Self::Saving { draft, .. } | Self::Failed { draft, .. } => draft.details.clone(),
            Self::Saved(saved) => saved.details.clone(),
        }
        .unwrap_or_default()
    }
}

/// A save the page started, from [`Desk::choose`] or [`Desk::save_details`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Save {
    pub(crate) key: Key,
    pub(crate) draft: Draft,
}

/// Every response's feedback the page saved, is saving or has read, in any
/// chat.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Desk {
    states: BTreeMap<Key, State>,
    /// The newest saved feedback a snapshot read, per response. A later
    /// snapshot that couldn't read feedback says nothing about it, so what
    /// was read still shows, and a save sends its details.
    read: BTreeMap<Key, ResponseFeedback>,
    /// The response whose details dialog is open.
    dialog: Option<Key>,
}

impl Desk {
    /// One response's feedback as shown, given `read`, the feedback its
    /// snapshot turn carries; `None` when it is not rated. It is the highest
    /// revision among `read`, every earlier snapshot's read and the page's
    /// own saves.
    pub(crate) fn state(&self, key: &Key, read: Option<&ResponseFeedback>) -> Option<State> {
        let read = newest(read, self.read.get(key));
        match self.states.get(key) {
            Some(state) => Some(state.with(read.as_ref())),
            None => read.map(State::Saved),
        }
    }

    /// Keeps the feedback a snapshot of `chat` read, per response, wherever it
    /// is newer than any read before it.
    pub(crate) fn remember<'a>(
        &mut self,
        chat: &ChatKey,
        reads: impl IntoIterator<Item = (&'a ResponseId, &'a ResponseFeedback)>,
    ) {
        for (response, saved) in reads {
            let key = (chat.clone(), response.clone());
            if self
                .read
                .get(&key)
                .is_none_or(|known| saved.revision > known.revision)
            {
                self.read.insert(key, saved.clone());
            }
        }
    }

    /// What `key`'s controls show given `read`. Feedback an earlier snapshot
    /// read, or a save this page made or is making, still shows when a later
    /// snapshot couldn't read feedback.
    pub(crate) fn controls(&self, key: &Key, read: &Read) -> Controls {
        match (self.state(key, read.saved()), read) {
            (Some(state), _) => Controls::Rated(state),
            (None, Read::Loaded(_)) => Controls::Unrated,
            (None, Read::Unavailable) => Controls::Unavailable,
        }
    }

    /// Starts saving `draft` over what `key` shows.
    fn begin(&mut self, key: &Key, shown: Option<&State>, draft: Draft) -> Save {
        let over = shown.and_then(State::saved).cloned();
        self.states.insert(
            key.clone(),
            State::Saving {
                draft: draft.clone(),
                over,
            },
        );
        Save {
            key: key.clone(),
            draft,
        }
    }

    /// Presses `choice` on a response; answers the save to make, or `None`
    /// while a save is on its way or the choice is already saved. The save
    /// keeps the details: a failed save's, else the saved ones.
    pub(crate) fn choose(
        &mut self,
        key: &Key,
        read: Option<&ResponseFeedback>,
        choice: FeedbackRating,
    ) -> Option<Save> {
        let shown = self.state(key, read);
        let details = match &shown {
            None => None,
            Some(State::Saving { .. }) => return None,
            Some(State::Failed { draft, .. }) => draft.details.clone(),
            Some(State::Saved(saved)) if saved.rating == choice => return None,
            Some(State::Saved(saved)) => saved.details.clone(),
        };
        let draft = Draft {
            rating: choice,
            details,
        };
        Some(self.begin(key, shown.as_ref(), draft))
    }

    /// Presses `choice` on a response its snapshot turn read as `read`, as
    /// [`Self::choose`] does; answers `None` while its controls are
    /// [`Controls::Unavailable`], since its saved details are unknown.
    pub(crate) fn rate(&mut self, key: &Key, read: &Read, choice: FeedbackRating) -> Option<Save> {
        match self.controls(key, read) {
            Controls::Unavailable => None,
            Controls::Unrated | Controls::Rated(_) => self.choose(key, read.saved(), choice),
        }
    }

    /// Saves `details` with the shown rating; answers the save to make, or
    /// `None` while the response has no saved rating or a save is on its way.
    pub(crate) fn save_details(
        &mut self,
        key: &Key,
        read: Option<&ResponseFeedback>,
        details: FeedbackDetails,
    ) -> Option<Save> {
        let shown = self.state(key, read).filter(|state| !state.busy())?;
        let rating = shown.saved()?.rating;
        let draft = Draft {
            rating,
            details: Some(details),
        };
        Some(self.begin(key, Some(&shown), draft))
    }

    /// `save` answered with the saved feedback, or failed. A landed save
    /// shows the higher of its revision and any newer one read meanwhile,
    /// and closes its dialog; a failed one keeps its draft to retry.
    pub(crate) fn answered(&mut self, save: &Save, answer: Result<ResponseFeedback, ()>) {
        let Some(State::Saving { draft, over }) = self.states.remove(&save.key) else {
            return;
        };
        let state = match answer {
            Ok(saved) => {
                if self.dialog.as_ref() == Some(&save.key) {
                    self.dialog = None;
                }
                State::Saved(newest(Some(&saved), over.as_ref()).unwrap_or(saved))
            }
            Err(()) => State::Failed { draft, over },
        };
        self.states.insert(save.key.clone(), state);
    }

    /// Opens the details dialog on a rated response no save is on its way to.
    pub(crate) fn open_details(&mut self, key: &Key, read: Option<&ResponseFeedback>) {
        if self
            .state(key, read)
            .is_some_and(|state| !state.busy() && state.saved().is_some())
        {
            self.dialog = Some(key.clone());
        }
    }

    /// Closes the details dialog. A save on its way keeps going.
    pub(crate) fn close_details(&mut self) {
        self.dialog = None;
    }

    pub(crate) fn details_open(&self, key: &Key) -> bool {
        self.dialog.as_ref() == Some(key)
    }
}

/// Every chat's feedback, the client its saves go through, the session a
/// refused save reports to, and the scope that keeps them: a save outlives
/// the controls that started it.
#[derive(Clone, Copy)]
struct Feedback {
    desk: Signal<Desk>,
    client: Signal<PhysicianClient>,
    session: SessionState,
    /// Whether the shell has shown the practice; see [`SessionState::refused`].
    shown: Signal<bool>,
    owner: ScopeId,
}

/// Keeps the feedback for the [`ResponseRating`] controls below it, and
/// closes the details dialog when the open chat changes. Answers the desk a
/// snapshot's reads are [remembered](Desk::remember) in.
pub(crate) fn use_feedback(
    client: Signal<PhysicianClient>,
    session: SessionState,
    shown: Signal<bool>,
    workspace: Signal<Option<String>>,
    chat: Signal<Option<String>>,
) -> Signal<Desk> {
    let mut desk = use_signal(Desk::default);
    use_context_provider(|| Feedback {
        desk,
        client,
        session,
        shown,
        owner: current_scope_id(),
    });
    use_effect(move || {
        let _open = (workspace(), chat());
        desk.write().close_details();
    });
    desk
}

/// Sends `save` under the session it began with and records its answer, in
/// the scope that outlives the controls. A refused session goes to its
/// owner, which covers the page or signs out; the save shows as not saved.
fn send(feedback: Feedback, save: Save) {
    let Feedback {
        mut desk,
        client,
        session,
        shown,
        owner,
    } = feedback;
    let client = client.peek().pinned();
    let presented = client.session().get();
    Runtime::current().in_scope(owner, || {
        spawn(async move {
            let ((chat, response), draft) = (&save.key, &save.draft);
            let body = SaveFeedback {
                rating: draft.rating,
                details: draft.details.clone(),
            };
            let answer = client
                .save_feedback(&chat.workspace, &chat.chat, response, &body)
                .await
                .map_err(|error| {
                    if let Some(refused) = crate::refusal(&error) {
                        session.refused(refused, *shown.peek(), presented);
                    }
                });
            desk.write().answered(&save, answer);
        })
    });
}

/// The thumbs and details under one of Coda's responses in `chat`, named by
/// its response id; `read` is the feedback its snapshot turn carries. When
/// the snapshot couldn't read feedback and this page saved or read none, the thumbs
/// are disabled and say the rating couldn't be loaded; the chat stays usable.
#[component]
pub(crate) fn ResponseRating(
    chat: ChatKey,
    response: ResponseId,
    read: Read,
    #[props(default)] extra: Option<Element>,
) -> Element {
    let feedback = use_context::<Feedback>();
    let mut desk = feedback.desk;
    let key = (chat, response);
    let (controls, open) = {
        let desk = desk.read();
        (desk.controls(&key, &read), desk.details_open(&key))
    };
    let (rated, turn) = (key.clone(), read.clone());
    let rate = move |choice: ui::Rating| {
        let choice = match choice {
            ui::Rating::Up => FeedbackRating::Up,
            ui::Rating::Down => FeedbackRating::Down,
        };
        if let Some(save) = desk.write().rate(&rated, &turn, choice) {
            send(feedback, save);
        }
    };
    let read = read.saved().cloned();
    let (state, unavailable) = match controls {
        Controls::Rated(state) => (Some(state), false),
        Controls::Unrated => (None, false),
        Controls::Unavailable => (None, true),
    };
    let saved = state.as_ref().and_then(State::saved).cloned();
    let notice = match &state {
        Some(state) => state.notice(),
        None => unavailable.then_some(NOT_LOADED),
    };
    let busy = state.as_ref().is_some_and(State::busy);
    let selected = state
        .as_ref()
        .and_then(State::shown)
        .map(|rating| match rating {
            FeedbackRating::Up => ui::Rating::Up,
            FeedbackRating::Down => ui::Rating::Down,
        });
    let (opened, opened_read) = (key.clone(), read.clone());
    rsx! {
        ui::MessageActions { label: ACTIONS, shown: saved.is_some() || notice.is_some() || busy,
            ui::FeedbackRating {
                selected,
                label: RATE,
                up_label: GOOD,
                down_label: POOR,
                busy,
                disabled: unavailable,
                on_rate: rate,
            }
            if let Some(saved) = &saved {
                Button {
                    variant: ButtonVariant::Quiet,
                    compact: true,
                    class: "feedback-details-open",
                    disabled: busy,
                    onclick: move |_| desk.write().open_details(&opened, opened_read.as_ref()),
                    if saved.details.is_some() { "{EDIT_DETAILS}" } else { "{ADD_DETAILS}" }
                }
            }
            if let Some(notice) = notice {
                span { class: "quiet-line feedback-notice", role: "status", "{notice}" }
            }
            if let Some(extra) = extra {
                {extra}
            }
        }
        if let (true, Some(state)) = (open, state) {
            // Keys pressed anywhere in the dialog are its own, never the page's
            // shortcuts or Send; the dialog answers Escape itself.
            div {
                class: "feedback-details-host",
                onkeydown: move |event| event.stop_propagation(),
                DetailsDialog { response: key, read, state }
            }
        }
    }
}

/// The details dialog: reasons, the physician's words and its own Save,
/// which sends them with the shown rating. Closing it without saving leaves
/// the saved feedback; a save on its way keeps going, and reopening shows it.
#[component]
fn DetailsDialog(response: Key, read: Option<ResponseFeedback>, state: State) -> Element {
    let feedback = use_context::<Feedback>();
    let mut desk = feedback.desk;
    let draft = state.draft_details();
    let mut reasons = use_signal(|| draft.reasons.iter().copied().collect::<BTreeSet<_>>());
    let mut text = use_signal(|| draft.text.clone());
    let saving = state.busy();
    let save = move |_| {
        let details = FeedbackDetails {
            reasons: reasons.peek().iter().copied().collect(),
            text: text.peek().clone(),
        };
        if let Some(save) = desk.write().save_details(&response, read.as_ref(), details) {
            send(feedback, save);
        }
    };
    let screen_hidden = crate::is_screen_hidden();
    rsx! {
        if !screen_hidden {
        Dialog {
            class: "feedback-details",
            title: DETAILS_TITLE,
            subtitle: DETAILS_SUBTITLE,
            close_label: CLOSE,
            on_close: move |_| desk.write().close_details(),
            div { class: "feedback-reasons",
                for (reason, label) in REASONS {
                    ToggleChip {
                        key: "{label}",
                        pressed: reasons.read().contains(&reason),
                        disabled: saving,
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
                disabled: saving,
            }
            ActionRow {
                if let Some(notice) = state.notice() {
                    span { class: "quiet-line feedback-notice", role: "status", "{notice}" }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    busy: saving,
                    pending_label: SAVING_DETAILS,
                    onclick: save,
                    "{SAVE_DETAILS}"
                }
            }
        }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use FeedbackRating::{Down, Up};

    fn key(chat: &str, n: usize) -> Key {
        (
            ChatKey {
                workspace: "ws".into(),
                chat: chat.into(),
            },
            ResponseId(format!("a{n}:reply")),
        )
    }

    fn saved(revision: u64, rating: FeedbackRating) -> ResponseFeedback {
        ResponseFeedback {
            revision,
            rating,
            details: None,
        }
    }

    fn words(text: &str) -> FeedbackDetails {
        FeedbackDetails {
            reasons: vec![FeedbackReason::MissingContext],
            text: text.into(),
        }
    }

    fn with_words(revision: u64, rating: FeedbackRating, text: &str) -> ResponseFeedback {
        ResponseFeedback {
            details: Some(words(text)),
            ..saved(revision, rating)
        }
    }

    #[test]
    fn the_snapshot_shows_until_the_page_saves() {
        let desk = Desk::default();
        let a1 = key("a", 1);
        assert_eq!(desk.state(&a1, None), None, "unrated");
        assert_eq!(
            desk.state(&a1, Some(&saved(2, Up))),
            Some(State::Saved(saved(2, Up)))
        );
    }

    #[test]
    fn an_older_revision_never_replaces_a_newer_one() {
        let mut desk = Desk::default();
        let a1 = key("a", 1);
        let save = desk.choose(&a1, Some(&saved(1, Up)), Down).unwrap();
        desk.answered(&save, Ok(saved(2, Down)));
        // A snapshot read before the save landed is older.
        assert_eq!(
            desk.state(&a1, Some(&saved(1, Up))),
            Some(State::Saved(saved(2, Down)))
        );
        // A newer one, saved elsewhere, shows.
        assert_eq!(
            desk.state(&a1, Some(&saved(3, Up))),
            Some(State::Saved(saved(3, Up)))
        );

        // A save's answer older than a snapshot read meanwhile loses to it.
        let save = desk.choose(&a1, Some(&saved(3, Up)), Down).unwrap();
        let newer = saved(5, Up);
        let saving = desk.state(&a1, Some(&newer)).unwrap();
        assert_eq!(
            saving.shown(),
            Some(Down),
            "the save on its way shows its choice"
        );
        assert_eq!(saving.saved(), Some(&newer), "over is raised");
        desk.answered(&save, Ok(saved(4, Down)));
        assert_eq!(desk.state(&a1, Some(&newer)), Some(State::Saved(newer)));
        // A save's answer never falls below what it was saved over.
        let save = desk.choose(&a1, Some(&saved(7, Down)), Up).unwrap();
        desk.answered(&save, Ok(saved(6, Up)));
        assert_eq!(desk.state(&a1, None), Some(State::Saved(saved(7, Down))));
    }

    #[test]
    fn one_save_at_a_time_and_the_choice_shows_while_it_saves() {
        let mut desk = Desk::default();
        let a1 = key("a", 1);
        let save = desk.choose(&a1, None, Up).unwrap();
        assert_eq!(
            save.draft,
            Draft {
                rating: Up,
                details: None
            }
        );
        let saving = desk.state(&a1, None).unwrap();
        assert_eq!((saving.shown(), saving.notice()), (Some(Up), None));
        assert!(saving.busy());
        assert_eq!(desk.choose(&a1, None, Down), None, "one save at a time");
        assert_eq!(desk.save_details(&a1, None, words("x")), None);
        desk.answered(&save, Ok(saved(1, Up)));
        assert_eq!(desk.state(&a1, None), Some(State::Saved(saved(1, Up))));
        assert_eq!(desk.choose(&a1, None, Up), None, "already saved");
        assert_eq!(desk.state(&key("a", 3), None), None);
    }

    #[test]
    fn a_failed_save_says_so_shows_the_saved_rating_and_retries() {
        let mut desk = Desk::default();
        let a1 = key("a", 1);
        let read = saved(1, Up);
        let save = desk.choose(&a1, Some(&read), Down).unwrap();
        desk.answered(&save, Err(()));
        let failed = desk.state(&a1, Some(&read)).unwrap();
        assert_eq!(
            (failed.shown(), failed.notice()),
            (Some(Up), Some(NOT_SAVED))
        );
        // Pressing again retries.
        let save = desk.choose(&a1, Some(&read), Down).unwrap();
        desk.answered(&save, Ok(saved(2, Down)));
        assert_eq!(
            desk.state(&a1, Some(&read)),
            Some(State::Saved(saved(2, Down)))
        );

        let a2 = key("a", 2);
        let save = desk.choose(&a2, None, Up).unwrap();
        desk.answered(&save, Err(()));
        let unrated = desk.state(&a2, None).unwrap();
        assert_eq!((unrated.shown(), unrated.notice()), (None, Some(NOT_SAVED)));
    }

    #[test]
    fn a_rating_change_keeps_the_saved_details() {
        let mut desk = Desk::default();
        let a1 = key("a", 1);
        let read = with_words(2, Up, "Trend missing.");
        let save = desk.choose(&a1, Some(&read), Down).unwrap();
        assert_eq!(save.draft.details, Some(words("Trend missing.")));
        desk.answered(&save, Err(()));
        let retry = desk.choose(&a1, Some(&read), Down).unwrap();
        assert_eq!(retry.draft.details, Some(words("Trend missing.")));
    }

    /// A thumb pressed after a details save failed sends those unsaved
    /// details with the new rating, on purpose: the words the physician
    /// wrote are what they want saved, and one save replaces both.
    #[test]
    fn a_thumb_after_a_failed_details_save_sends_those_details() {
        let mut desk = Desk::default();
        let a2 = key("a", 2);
        let read = with_words(1, Up, "Saved words.");
        let save = desk
            .save_details(&a2, Some(&read), words("Recheck."))
            .unwrap();
        desk.answered(&save, Err(()));
        let failed = desk.state(&a2, Some(&read)).unwrap();
        assert_eq!(failed.saved(), Some(&read), "the saved words still show");
        let save = desk.choose(&a2, Some(&read), Down).unwrap();
        assert_eq!(
            save.draft,
            Draft {
                rating: Down,
                details: Some(words("Recheck."))
            }
        );
        desk.answered(&save, Ok(with_words(2, Down, "Recheck.")));
        assert_eq!(
            desk.state(&a2, Some(&read)),
            Some(State::Saved(with_words(2, Down, "Recheck.")))
        );
    }

    #[test]
    fn unavailable_feedback_disables_thumbs_the_page_saved_nothing_on() {
        let mut desk = Desk::default();
        let (a1, a2) = (key("a", 1), key("a", 2));
        assert_eq!(
            desk.controls(&a1, &Read::Unavailable),
            Controls::Unavailable
        );
        assert_eq!(desk.rate(&a1, &Read::Unavailable, Up), None, "disabled");
        assert_eq!(desk.controls(&a1, &Read::Loaded(None)), Controls::Unrated);
        // A save this page made still shows when a later read fails.
        let save = desk
            .rate(&a2, &Read::Loaded(Some(saved(1, Up))), Down)
            .unwrap();
        desk.answered(&save, Ok(saved(2, Down)));
        assert_eq!(
            desk.controls(&a2, &Read::Unavailable),
            Controls::Rated(State::Saved(saved(2, Down)))
        );
        assert!(desk.rate(&a2, &Read::Unavailable, Up).is_some());
    }

    #[test]
    fn a_read_the_page_saw_still_shows_when_a_later_read_fails() {
        let mut desk = Desk::default();
        let a1 = key("a", 1);
        let read = with_words(2, Down, "missed the potassium");
        desk.remember(&a1.0, [(&a1.1, &read)]);
        desk.open_details(&a1, Some(&read));
        // The rating stays and its dialog stays open, so unsaved words stay.
        assert_eq!(
            desk.controls(&a1, &Read::Unavailable),
            Controls::Rated(State::Saved(read.clone()))
        );
        assert!(desk.details_open(&a1));
        // A thumb press then keeps the details it read.
        let save = desk.rate(&a1, &Read::Unavailable, Up).unwrap();
        assert_eq!(save.draft.details, read.details);
    }

    #[test]
    fn a_failed_read_never_falls_back_below_a_newer_revision_read() {
        let mut desk = Desk::default();
        let a1 = key("a", 1);
        let save = desk.rate(&a1, &Read::Loaded(None), Up).unwrap();
        desk.answered(&save, Ok(saved(1, Up)));
        // Another device saved revision 2; this page read it.
        let newer = with_words(2, Down, "elsewhere");
        desk.remember(&a1.0, [(&a1.1, &newer)]);
        assert_eq!(
            desk.controls(&a1, &Read::Unavailable),
            Controls::Rated(State::Saved(newer.clone()))
        );
        let save = desk.rate(&a1, &Read::Unavailable, Up).unwrap();
        assert_eq!(save.draft.details, newer.details, "revision 2's details");
        // An older read never replaces a newer one kept.
        desk.remember(&a1.0, [(&a1.1, &saved(1, Up))]);
        assert_eq!(desk.read.get(&a1), Some(&newer));
    }

    #[test]
    fn details_save_with_the_shown_rating_and_a_landed_save_closes_its_dialog() {
        let mut desk = Desk::default();
        let a1 = key("a", 1);
        let read = saved(1, Down);
        desk.open_details(&key("a", 2), None);
        assert!(!desk.details_open(&key("a", 2)), "no rating, no details");
        desk.open_details(&a1, Some(&read));
        assert!(desk.details_open(&a1));
        let save = desk
            .save_details(&a1, Some(&read), words("Trend missing."))
            .unwrap();
        assert_eq!(
            save.draft,
            Draft {
                rating: Down,
                details: Some(words("Trend missing."))
            }
        );
        // Reopened while on its way, the dialog shows the save's words.
        desk.close_details();
        desk.open_details(&a1, Some(&read));
        assert!(!desk.details_open(&a1), "the dialog waits for the save");
        assert_eq!(
            desk.state(&a1, Some(&read)).unwrap().draft_details(),
            words("Trend missing.")
        );
        desk.dialog = Some(a1.clone());
        desk.answered(&save, Ok(with_words(2, Down, "Trend missing.")));
        assert!(!desk.details_open(&a1), "a landed save closes its dialog");

        // A failure keeps the words to retry, and says so.
        let shown = desk.state(&a1, Some(&read)).unwrap();
        assert_eq!(shown.draft_details(), words("Trend missing."));
        desk.open_details(&a1, Some(&read));
        let save = desk
            .save_details(&a1, Some(&read), words("Second try."))
            .unwrap();
        desk.answered(&save, Err(()));
        assert!(
            desk.details_open(&a1),
            "a failed save leaves its dialog open"
        );
        let failed = desk.state(&a1, Some(&read)).unwrap();
        assert_eq!(failed.notice(), Some(NOT_SAVED));
        assert_eq!(failed.draft_details(), words("Second try."));
        assert_eq!(failed.saved(), Some(&with_words(2, Down, "Trend missing.")));
    }

    #[test]
    fn a_save_outlives_a_chat_switch_and_a_failure_keeps_its_draft() {
        let mut desk = Desk::default();
        let (a1, b1) = (key("a", 1), key("b", 1));
        let save = desk.choose(&a1, None, Up).unwrap();
        // The same response id in another chat is another response.
        assert_eq!(desk.state(&b1, None), None);
        let other = desk.choose(&b1, None, Down).unwrap();
        desk.answered(&save, Ok(saved(1, Up)));
        desk.answered(&other, Err(()));
        assert_eq!(desk.state(&a1, None), Some(State::Saved(saved(1, Up))));
        assert_eq!(
            desk.state(&b1, None),
            Some(State::Failed {
                draft: Draft {
                    rating: Down,
                    details: None
                },
                over: None
            })
        );
    }
}

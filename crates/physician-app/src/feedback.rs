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
//! Only hosted chats name the run behind each response, which feedback keys
//! on, so other sites show no controls.
use std::collections::{BTreeMap, BTreeSet};

use dioxus::core::{Runtime, current_scope_id};
use dioxus::prelude::*;
use physician_client::PhysicianClient;
use physician_wire::{
    ChatFeedback, FeedbackDetails, FeedbackRating, FeedbackReason, ResponseFeedback, RunId,
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

/// Where the physician's latest details save on a response stands.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum DetailsSaving {
    /// No details save is on its way, and the last one, if any, landed.
    #[default]
    Idle,
    /// These details are on their way.
    Saving(FeedbackDetails),
    /// These details failed to save; the saved details still stand.
    Failed(FeedbackDetails),
}

/// One response's feedback on the page. One save at a time, of either kind,
/// so an older answer never replaces what a newer save changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) rating: RatingState,
    pub(crate) details: Option<FeedbackDetails>,
    pub(crate) details_saving: DetailsSaving,
    /// The desk's clock when the page last changed this entry itself.
    changed: u64,
    /// The desk's clock when its latest save, of either kind, started.
    saving: u64,
}

impl Entry {
    fn new(rating: RatingState) -> Self {
        Self {
            rating,
            details: None,
            details_saving: DetailsSaving::Idle,
            changed: 0,
            saving: 0,
        }
    }

    /// A save of either kind is on its way.
    pub(crate) fn busy(&self) -> bool {
        matches!(self.rating, RatingState::Saving { .. })
            || matches!(self.details_saving, DetailsSaving::Saving(_))
    }

    /// What the controls say beside the thumbs, if anything.
    pub(crate) fn notice(&self) -> Option<&'static str> {
        match self.details_saving {
            DetailsSaving::Failed(_) => Some(NOT_SAVED),
            _ => self.rating.notice(),
        }
    }

    /// The details the dialog opens with: those on their way or that failed
    /// to save, else the saved ones.
    pub(crate) fn draft(&self) -> FeedbackDetails {
        match &self.details_saving {
            DetailsSaving::Saving(draft) | DetailsSaving::Failed(draft) => draft.clone(),
            DetailsSaving::Idle => self.details.clone().unwrap_or_default(),
        }
    }
}

/// A rating save the page started, from [`Desk::choose`]. Only the latest
/// save on a response settles it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RatingSave {
    pub(crate) chat: ChatKey,
    run: RunId,
    started: u64,
}

/// A details save the page started, from [`Desk::save_details`]. Only the
/// latest save on a response settles it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DetailsSave {
    pub(crate) chat: ChatKey,
    run: RunId,
    details: FeedbackDetails,
    started: u64,
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
    entries: BTreeMap<RunId, Entry>,
    /// Counts the page's own changes, so a read older than one never undoes it.
    clock: u64,
    /// The clock when this chat was opened; reads started before it are of an earlier visit.
    opened: u64,
    /// Saves still on their way, in any chat: leaving a chat never forgets
    /// them, and returning shows them saving again.
    pending: BTreeMap<(ChatKey, RunId), Entry>,
    /// Details whose save failed, in any chat, kept until a later details
    /// save on that response starts, so the words and the failure survive
    /// leaving the chat.
    failed: BTreeMap<(ChatKey, RunId), FeedbackDetails>,
    /// The response whose details dialog is open in this chat.
    dialog: Option<RunId>,
}

impl Desk {
    /// Opens `chat`. Another chat starts over; the same chat keeps what it
    /// knows while it is read again.
    pub(crate) fn open(&mut self, chat: Option<ChatKey>) {
        if self.chat == chat {
            return;
        }
        let opened = self.tick();
        let entries = self
            .pending
            .iter()
            .filter(|((pending, _), _)| Some(pending) == chat.as_ref())
            .map(|((_, run), entry)| (run.clone(), entry.clone()))
            .collect();
        *self = Self {
            chat,
            read: None,
            entries,
            clock: opened,
            opened,
            pending: std::mem::take(&mut self.pending),
            failed: std::mem::take(&mut self.failed),
            dialog: None,
        };
    }

    /// The open chat.
    #[cfg(test)]
    fn chat(&self) -> Option<&ChatKey> {
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
            .map(|feedback| (feedback.run.clone(), feedback))
            .collect();
        let kept = |entry: &Entry| entry.changed > started.0 || entry.busy();
        self.entries
            .retain(|run, entry| saved.contains_key(run) || kept(entry));
        for (run, feedback) in saved {
            let entry = self
                .entries
                .entry(run)
                .or_insert(Entry::new(RatingState::Unrated));
            if !kept(entry) {
                entry.rating = RatingState::Rated(feedback.rating);
                entry.details = feedback.details;
            }
        }
    }

    /// One response's feedback as the page shows it.
    pub(crate) fn entry(&self, run: &RunId) -> Entry {
        let mut entry = self.entries.get(run).cloned().unwrap_or_else(|| {
            Entry::new(match self.read {
                None => RatingState::Loading,
                Some(false) => RatingState::Unknown,
                Some(true) => RatingState::Unrated,
            })
        });
        let failed = self
            .chat
            .clone()
            .and_then(|chat| self.failed.get(&(chat, run.clone())));
        if let (DetailsSaving::Idle, Some(details)) = (&entry.details_saving, failed) {
            entry.details_saving = DetailsSaving::Failed(details.clone());
        }
        entry
    }

    /// Records `entry` as changed by a save starting now, kept while it is
    /// on its way; answers the save's start.
    fn start_save(&mut self, chat: &ChatKey, run: &RunId, mut entry: Entry) -> u64 {
        let started = self.tick();
        entry.changed = started;
        entry.saving = started;
        self.pending
            .insert((chat.clone(), run.clone()), entry.clone());
        self.entries.insert(run.clone(), entry);
        started
    }

    /// Forgets the pending save that started at `started`, if it is the
    /// latest on its response, and answers the open entry it settles: none
    /// when its chat is closed or a newer save started since.
    fn settle(&mut self, chat: &ChatKey, run: &RunId, started: u64) -> Option<&mut Entry> {
        let key = (chat.clone(), run.clone());
        if self
            .pending
            .get(&key)
            .is_some_and(|entry| entry.saving == started)
        {
            self.pending.remove(&key);
        }
        if self.chat.as_ref() != Some(chat) {
            return None;
        }
        let changed = self.tick();
        let entry = self
            .entries
            .get_mut(run)
            .filter(|entry| entry.saving == started)?;
        entry.changed = changed;
        Some(entry)
    }

    /// Presses `choice` on a response; answers the save to make when the
    /// press starts one.
    pub(crate) fn choose(&mut self, run: &RunId, choice: FeedbackRating) -> Option<RatingSave> {
        let chat = self.chat.clone()?;
        let current = self.entry(run);
        if current.busy() {
            return None;
        }
        let rating = current.rating.choose(choice)?;
        let started = self.start_save(&chat, run, Entry { rating, ..current });
        Some(RatingSave {
            chat,
            run: run.clone(),
            started,
        })
    }

    /// Opens the details dialog on a rated response whose rating isn't saving.
    pub(crate) fn open_details(&mut self, run: &RunId) {
        let rating = self.entry(run).rating;
        if rating.saved().is_some() && !matches!(rating, RatingState::Saving { .. }) {
            self.dialog = Some(run.clone());
        }
    }

    /// Closes the details dialog. A details save on its way keeps going.
    pub(crate) fn close_details(&mut self) {
        self.dialog = None;
    }

    /// Whether the details dialog is open on `run`.
    pub(crate) fn details_open(&self, run: &RunId) -> bool {
        self.dialog.as_ref() == Some(run)
    }

    /// Saves `details` on a rated response; answers the save to make, or
    /// `None` while the rating isn't saved or another save is on its way.
    pub(crate) fn save_details(
        &mut self,
        run: &RunId,
        details: FeedbackDetails,
    ) -> Option<DetailsSave> {
        let chat = self.chat.clone()?;
        let current = self.entry(run);
        if current.busy() || current.rating.saved().is_none() {
            return None;
        }
        self.failed.remove(&(chat.clone(), run.clone()));
        let entry = Entry {
            details_saving: DetailsSaving::Saving(details.clone()),
            ..current
        };
        let started = self.start_save(&chat, run, entry);
        Some(DetailsSave {
            chat,
            run: run.clone(),
            details,
            started,
        })
    }

    /// `save` answered with the saved feedback, or failed. It settles its
    /// response only as the latest save there; while its chat is closed it
    /// leaves the store for the next read to show.
    pub(crate) fn rating_answered(
        &mut self,
        save: &RatingSave,
        answer: Result<ResponseFeedback, ()>,
    ) {
        let Some(entry) = self.settle(&save.chat, &save.run, save.started) else {
            return;
        };
        match answer {
            Ok(saved) => {
                entry.rating = entry.rating.answered(Some(saved.rating));
                entry.details = saved.details;
            }
            Err(()) => entry.rating = entry.rating.answered(None),
        }
    }

    /// `save` answered with the saved feedback, or failed. A landed save
    /// closes its dialog; a failed one keeps its details for the dialog to
    /// show again, open or reopened, even after leaving the chat. Like a
    /// rating save, it settles its response only as the latest save there.
    pub(crate) fn details_answered(
        &mut self,
        save: &DetailsSave,
        answer: Result<ResponseFeedback, ()>,
    ) {
        let key = (save.chat.clone(), save.run.clone());
        let latest = self
            .pending
            .get(&key)
            .is_some_and(|entry| entry.saving == save.started);
        if latest && answer.is_err() {
            self.failed.insert(key, save.details.clone());
        }
        let Some(entry) = self.settle(&save.chat, &save.run, save.started) else {
            return;
        };
        entry.details_saving = DetailsSaving::Idle;
        let Ok(saved) = answer else {
            return;
        };
        entry.rating = RatingState::Rated(saved.rating);
        entry.details = saved.details;
        if self.dialog.as_ref() == Some(&save.run) {
            self.dialog = None;
        }
    }
}

/// The open chat's feedback, the client its saves go through, and the scope
/// that keeps them: a rating save outlives the controls that started it, so
/// leaving the chat never strands it as saving.
#[derive(Clone, Copy)]
struct Feedback {
    desk: Signal<Desk>,
    client: Signal<PhysicianClient>,
    owner: ScopeId,
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
    use_context_provider(|| Feedback {
        desk,
        client,
        owner: current_scope_id(),
    });
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

/// The thumbs and details under one of Coda's responses, named by the run
/// that published it.
#[component]
pub(crate) fn ResponseRating(run: RunId) -> Element {
    let Feedback {
        mut desk,
        client,
        owner,
    } = use_context::<Feedback>();
    let (entry, open) = {
        let desk = desk.read();
        (desk.entry(&run), desk.details_open(&run))
    };
    let rating = entry.rating;
    let notice = entry.notice();
    let rated = run.clone();
    let rate = move |choice: ui::Rating| {
        let choice = match choice {
            ui::Rating::Up => FeedbackRating::Up,
            ui::Rating::Down => FeedbackRating::Down,
        };
        let Some(save) = desk.write().choose(&rated, choice) else {
            return;
        };
        let client = client.peek().clone();
        Runtime::current().in_scope(owner, || {
            spawn(async move {
                let answer = client
                    .rate_response(&save.chat.workspace, &save.chat.chat, &save.run, choice)
                    .await
                    .map_err(|_| ());
                desk.write().rating_answered(&save, answer);
            })
        });
    };
    let selected = rating.shown().map(|rating| match rating {
        FeedbackRating::Up => ui::Rating::Up,
        FeedbackRating::Down => ui::Rating::Down,
    });
    let busy = matches!(rating, RatingState::Saving { .. });
    let opened = run.clone();
    rsx! {
        ui::MessageActions { label: ACTIONS, shown: rating.saved().is_some() || notice.is_some() || entry.busy(),
            ui::FeedbackRating {
                selected,
                label: RATE,
                up_label: GOOD,
                down_label: POOR,
                busy,
                disabled: matches!(rating, RatingState::Loading | RatingState::Unknown)
                    || (entry.busy() && !busy),
                on_rate: rate,
            }
            if rating.saved().is_some() {
                Button {
                    variant: ButtonVariant::Quiet,
                    compact: true,
                    class: "feedback-details-open",
                    disabled: busy,
                    onclick: move |_| desk.write().open_details(&opened),
                    if entry.details.is_some() { "{EDIT_DETAILS}" } else { "{ADD_DETAILS}" }
                }
            }
            if let Some(notice) = notice {
                span { class: "quiet-line feedback-notice", role: "status", "{notice}" }
            }
        }
        if open {
            // Keys pressed anywhere in the dialog are its own, never the page's
            // shortcuts or Send; the dialog answers Escape itself.
            div {
                class: "feedback-details-host",
                onkeydown: move |event| event.stop_propagation(),
                DetailsDialog { run, entry }
            }
        }
    }
}

/// The details dialog: reasons, the physician's words and its own Save.
/// Closing it without saving leaves the rating and any saved details; a
/// save on its way keeps going, and reopening shows it.
#[component]
fn DetailsDialog(run: RunId, entry: Entry) -> Element {
    let Feedback {
        mut desk,
        client,
        owner,
    } = use_context::<Feedback>();
    let draft = entry.draft();
    let mut reasons = use_signal(|| draft.reasons.iter().copied().collect::<BTreeSet<_>>());
    let mut text = use_signal(|| draft.text.clone());
    let saving = matches!(entry.details_saving, DetailsSaving::Saving(_));
    let failed = matches!(entry.details_saving, DetailsSaving::Failed(_));
    let save = move |_| {
        let details = FeedbackDetails {
            reasons: reasons.peek().iter().copied().collect(),
            text: text.peek().clone(),
        };
        let Some(save) = desk.write().save_details(&run, details) else {
            return;
        };
        let client = client.peek().clone();
        Runtime::current().in_scope(owner, || {
            spawn(async move {
                let answer = client
                    .save_feedback_details(
                        &save.chat.workspace,
                        &save.chat.chat,
                        &save.run,
                        &save.details,
                    )
                    .await
                    .map_err(|_| ());
                desk.write().details_answered(&save, answer);
            })
        });
    };
    rsx! {
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
                if failed {
                    span { class: "quiet-line feedback-notice", role: "status", "{NOT_SAVED}" }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    busy: saving,
                    disabled: entry.busy() && !saving,
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

    fn run(n: usize) -> RunId {
        RunId(format!("run-{n}"))
    }

    fn saved(n: usize, rating: FeedbackRating) -> ResponseFeedback {
        ResponseFeedback {
            run: run(n),
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
        let save = desk.choose(&run(1), Down).unwrap();
        desk.rating_answered(&save, Ok(saved(1, Down)));
        let save = desk.choose(&run(2), Up).unwrap();
        desk.rating_answered(&save, Ok(saved(2, Up)));
        desk.read_answered(
            &chat("a"),
            stale,
            Ok(ChatFeedback {
                feedback: vec![saved(1, Up)],
            }),
        );
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Down));
        assert_eq!(desk.entry(&run(2)).rating, RatingState::Rated(Up));
        // A read started after the saves shows what the store holds.
        read(&mut desk, vec![saved(1, Up)]);
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Up));
        assert_eq!(desk.entry(&run(2)).rating, RatingState::Unrated);
    }

    #[test]
    fn a_save_outlives_leaving_its_chat() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![]);
        let save = desk.choose(&run(1), Up).unwrap();
        desk.open(Some(chat("b")));
        desk.open(Some(chat("a")));
        read(&mut desk, vec![]);
        assert!(
            matches!(
                desk.entry(&run(1)).rating,
                RatingState::Saving { choice: Up, .. }
            ),
            "returning forgot the save on its way"
        );
        assert_eq!(desk.choose(&run(1), Down), None, "one save at a time");
        desk.rating_answered(&save, Ok(saved(1, Up)));
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Up));

        // A save that answers while its chat is closed leaves the next read
        // to show it, and is not restored as saving.
        let save = desk.choose(&run(1), Down).unwrap();
        desk.open(Some(chat("b")));
        desk.rating_answered(&save, Ok(saved(1, Down)));
        desk.open(Some(chat("a")));
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Loading);
        read(&mut desk, vec![saved(1, Down)]);
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Down));
    }

    #[test]
    fn only_the_latest_save_settles_a_response() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![]);
        let failed = desk.choose(&run(1), Up).unwrap();
        desk.rating_answered(&failed, Err(()));
        let retry = desk.choose(&run(1), Down).unwrap();
        desk.rating_answered(&failed, Ok(saved(1, Up)));
        assert!(
            matches!(
                desk.entry(&run(1)).rating,
                RatingState::Saving { choice: Down, .. }
            ),
            "an answered save settled the newer one"
        );
        desk.rating_answered(&retry, Ok(saved(1, Down)));
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Down));
    }

    #[test]
    fn a_rating_saves_shows_the_choice_and_lands_saved() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Loading);
        assert_eq!(
            desk.choose(&run(1), Up),
            None,
            "nothing to save over before the read"
        );
        read(&mut desk, vec![]);
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Unrated);
        let save = desk.choose(&run(1), Up).unwrap();
        assert_eq!(save.chat, chat("a"));
        let saving = desk.entry(&run(1)).rating;
        assert_eq!((saving.shown(), saving.notice()), (Some(Up), None));
        assert_eq!(desk.choose(&run(1), Down), None, "one save at a time");
        desk.rating_answered(&save, Ok(saved(1, Up)));
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Up));
        assert_eq!(desk.choose(&run(1), Up), None, "already saved");
        assert_eq!(desk.entry(&run(3)).rating, RatingState::Unrated);
    }

    #[test]
    fn a_failed_save_says_so_and_shows_the_saved_rating_again() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        let save = desk.choose(&run(1), Down).unwrap();
        desk.rating_answered(&save, Err(()));
        let failed = desk.entry(&run(1)).rating;
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
        let save = desk.choose(&run(1), Down).unwrap();
        desk.rating_answered(&save, Ok(saved(1, Down)));
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Down));

        let unrated = RatingState::Unrated.choose(Up).unwrap().answered(None);
        assert_eq!((unrated.shown(), unrated.notice()), (None, Some(NOT_SAVED)));
    }

    #[test]
    fn a_failed_read_is_unknown_never_unrated() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read_fails(&mut desk);
        let unknown = desk.entry(&run(1)).rating;
        assert_eq!(unknown, RatingState::Unknown);
        assert_eq!(
            (unknown.shown(), unknown.notice()),
            (None, Some(NOT_LOADED))
        );
        assert_eq!(desk.choose(&run(1), Up), None);
        // A later read that works shows the saved rating.
        read(&mut desk, vec![saved(1, Down)]);
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Down));
        // A failed reread keeps what was read.
        read_fails(&mut desk);
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Down));
        assert_eq!(desk.entry(&run(3)).rating, RatingState::Unrated);
    }

    #[test]
    fn answers_for_another_chat_change_nothing() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![]);
        let save = desk.choose(&run(1), Up).unwrap();
        let earlier = desk.read_started();
        desk.open(Some(chat("b")));
        desk.rating_answered(&save, Ok(saved(1, Up)));
        let started = desk.read_started();
        desk.read_answered(
            &chat("a"),
            started,
            Ok(ChatFeedback {
                feedback: vec![saved(1, Up)],
            }),
        );
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Loading);
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
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Loading);
    }

    #[test]
    fn a_reread_keeps_a_save_on_its_way_and_details_follow_the_rating() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        let save = desk.choose(&run(1), Down).unwrap();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        assert!(matches!(
            desk.entry(&run(1)).rating,
            RatingState::Saving { choice: Down, .. }
        ));
        let details = FeedbackDetails {
            reasons: vec![FeedbackReason::MissingContext],
            text: "Sodium was rechecked.".into(),
        };
        desk.rating_answered(
            &save,
            Ok(ResponseFeedback {
                details: Some(details.clone()),
                ..saved(1, Down)
            }),
        );
        assert_eq!(desk.entry(&run(1)).details, Some(details.clone()));
        let cleared = desk
            .save_details(&run(1), FeedbackDetails::default())
            .unwrap();
        desk.details_answered(
            &cleared,
            Ok(ResponseFeedback {
                details: Some(FeedbackDetails::default()),
                ..saved(1, Down)
            }),
        );
        assert_eq!(
            desk.entry(&run(1)).details,
            Some(FeedbackDetails::default())
        );
        assert_eq!(desk.entry(&run(1)).rating, RatingState::Rated(Down));
    }

    fn words(text: &str) -> FeedbackDetails {
        FeedbackDetails {
            reasons: vec![FeedbackReason::MissingContext],
            text: text.into(),
        }
    }

    #[test]
    fn a_details_save_outlives_closing_its_dialog() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        desk.open_details(&run(1));
        let save = desk.save_details(&run(1), words("Trend missing.")).unwrap();
        desk.close_details();
        assert!(!desk.details_open(&run(1)));
        // Reopened while on its way, the dialog shows the save, not the old details.
        desk.open_details(&run(1));
        let entry = desk.entry(&run(1));
        assert_eq!(
            entry.details_saving,
            DetailsSaving::Saving(words("Trend missing."))
        );
        assert_eq!(entry.draft(), words("Trend missing."));
        desk.close_details();
        desk.details_answered(
            &save,
            Ok(ResponseFeedback {
                details: Some(words("Trend missing.")),
                ..saved(1, Up)
            }),
        );
        let entry = desk.entry(&run(1));
        assert_eq!(entry.details, Some(words("Trend missing.")));
        assert_eq!(entry.details_saving, DetailsSaving::Idle);

        // A failure after the dialog closed says so and keeps the words to retry.
        let save = desk.save_details(&run(1), words("Second try.")).unwrap();
        desk.details_answered(&save, Err(()));
        let entry = desk.entry(&run(1));
        assert_eq!(entry.notice(), Some(NOT_SAVED));
        assert_eq!(entry.details, Some(words("Trend missing.")));
        assert_eq!(entry.draft(), words("Second try."));
    }

    #[test]
    fn a_details_save_outlives_leaving_its_chat() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        desk.open_details(&run(1));
        let save = desk.save_details(&run(1), words("Trend missing.")).unwrap();
        desk.open(Some(chat("b")));
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        assert!(!desk.details_open(&run(1)), "a new visit opens no dialog");
        assert!(desk.entry(&run(1)).busy(), "returning forgot the save");
        desk.details_answered(
            &save,
            Ok(ResponseFeedback {
                details: Some(words("Trend missing.")),
                ..saved(1, Up)
            }),
        );
        assert_eq!(desk.entry(&run(1)).details, Some(words("Trend missing.")));

        // A details save that fails while its chat is closed keeps the words
        // and says so on return, until a later details save starts.
        desk.open_details(&run(1));
        let save = desk
            .save_details(&run(1), words("Recheck sodium."))
            .unwrap();
        desk.close_details();
        desk.open(Some(chat("b")));
        desk.details_answered(&save, Err(()));
        assert_eq!(desk.entry(&run(1)).notice(), None, "chat b has no failure");
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        let entry = desk.entry(&run(1));
        assert_eq!(entry.notice(), Some(NOT_SAVED));
        assert_eq!(entry.draft(), words("Recheck sodium."));
        let retry = desk
            .save_details(&run(1), words("Recheck sodium."))
            .unwrap();
        assert!(desk.entry(&run(1)).busy());
        desk.details_answered(
            &retry,
            Ok(ResponseFeedback {
                details: Some(words("Recheck sodium.")),
                ..saved(1, Up)
            }),
        );
        let entry = desk.entry(&run(1));
        assert_eq!(entry.notice(), None);
        assert_eq!(entry.details, Some(words("Recheck sodium.")));
    }

    #[test]
    fn one_save_at_a_time_so_an_older_answer_never_undoes_newer_details() {
        let mut desk = Desk::default();
        desk.open(Some(chat("a")));
        read(&mut desk, vec![saved(1, Up)]);
        let rating = desk.choose(&run(1), Down).unwrap();
        assert_eq!(
            desk.save_details(&run(1), words("Trend missing.")),
            None,
            "details wait for the rating save"
        );
        desk.open_details(&run(1));
        assert!(!desk.details_open(&run(1)), "the dialog waits too");
        desk.rating_answered(&rating, Ok(saved(1, Down)));

        let details = desk.save_details(&run(1), words("Trend missing.")).unwrap();
        assert_eq!(
            desk.choose(&run(1), Up),
            None,
            "the rating waits for details"
        );
        desk.details_answered(
            &details,
            Ok(ResponseFeedback {
                details: Some(words("Trend missing.")),
                ..saved(1, Down)
            }),
        );
        // A late answer of the earlier rating save changes nothing.
        desk.rating_answered(&rating, Ok(saved(1, Down)));
        assert_eq!(desk.entry(&run(1)).details, Some(words("Trend missing.")));
    }
}

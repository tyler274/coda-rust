//! The composer's draft text, its autosave and its Send, kept per draft target.
//!
//! A target is one patient workspace and one chat (the chat id `draft` before
//! a chat is open). The shell is rebuilt for each physician, so everything
//! here already belongs to the signed-in physician.
//!
//! Each target keeps the text the physician sees, an edit generation that
//! every keystroke advances, and the baseline: the server's text and revision
//! as this page last read or saved them. Whether the target needs saving is
//! derived (`text != saved`), never a second dirty flag. One save runs per
//! target; every save carries its target, generation, expected revision and
//! text, so a late answer can only move its own target's baseline and can
//! never replace newer typing. Edits made while a save runs are still unsaved
//! when it returns, so the shell schedules a trailing save.
//!
//! A save the server refuses with a validated conflict pauses autosave and
//! keeps the newer revision it reported. **Retry** saves this page's text over
//! exactly that revision (if it moved again, another conflict shows); **Use
//! newer text** discards the local edit and shows the newer text. A transient
//! failure is not a conflict: it shows Not saved and waits for the next edit
//! or the shell's slow retry, never a tight loop.
//!
//! Send captures the target, text, edit generation and command key. Once the
//! server accepts it, only the submitted text is removed from the composer
//! (anything typed after Send stays), and the durable clear is an ordinary
//! autosave queued behind any save already running, so it never races a
//! revision-zero save. A clear refused because another tab saved newer text
//! is a conflict like any other; nothing is cleared automatically over it.
// Recovery is browser-only; native builds use the rest.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The chat id the page saves under before a chat is open.
pub(crate) const UNOPENED_CHAT: &str = "draft";

/// One patient workspace and chat: what a draft belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct DraftTarget {
    pub(crate) workspace: String,
    pub(crate) chat: String,
}

impl DraftTarget {
    pub(crate) fn new(workspace: impl Into<String>, chat: Option<&str>) -> Self {
        Self {
            workspace: workspace.into(),
            chat: chat.unwrap_or(UNOPENED_CHAT).to_owned(),
        }
    }
}

/// One save on its way: what it saves, where, from which edit, and over
/// which revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SaveAttempt {
    pub(crate) id: u64,
    pub(crate) target: DraftTarget,
    pub(crate) generation: u64,
    pub(crate) expected_revision: i64,
    pub(crate) text: String,
}

/// How the server answered a save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SaveOutcome {
    /// Saved; the draft is now at this revision.
    Saved(i64),
    /// A validated conflict: the server's current revision and text. Nothing
    /// was written.
    Conflict(i64, String),
    /// The save did not reach the server, or was refused without a validated
    /// current draft. Never an authorization to overwrite.
    Failed,
}

/// The newer saved draft a conflict shows the physician.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ConflictDraft {
    pub(crate) revision: i64,
    pub(crate) text: String,
}

/// A Send whose acceptance is not yet known: the exact command (with its
/// idempotency key) and the text and edit generation it submitted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct PendingSend {
    pub(crate) key: String,
    pub(crate) text: String,
    pub(crate) generation: u64,
    pub(crate) command: serde_json::Value,
}

/// How the server answered a Send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SendAnswer {
    Accepted,
    Refused,
}

/// What same-tab recovery keeps for one target: enough to rebuild the
/// composer after a reload and reconcile it with the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Recovery {
    pub(crate) text: String,
    pub(crate) saved: String,
    pub(crate) revision: i64,
    pub(crate) generation: u64,
    #[serde(default)]
    pub(crate) conflict: Option<ConflictDraft>,
    #[serde(default)]
    pub(crate) pending_send: Option<PendingSend>,
    #[serde(default)]
    pub(crate) pending_clear: bool,
    /// An unanswered save is replayed at its original revision before any
    /// trailing edit or accepted Send cleanup. A read cannot settle it.
    #[serde(default)]
    pub(crate) pending_save: Option<ConflictDraft>,
}

/// One target's composer text and its baseline.
#[derive(Debug, Clone, Default, PartialEq)]
struct Held {
    text: String,
    /// Advanced by every local edit.
    generation: u64,
    /// The server's text at `revision`, as this page last read or saved it.
    saved: String,
    /// The revision `saved` is at; 0 is the server's "no draft yet".
    revision: i64,
    in_flight: Option<u64>,
    pending_save: Option<ConflictDraft>,
    conflict: Option<ConflictDraft>,
    /// The last save failed; autosave waits for an edit or a slow retry.
    failed: bool,
    /// Saves that failed in a row; the shell's slow retry backs off on it.
    failures: u32,
    pending_send: Option<PendingSend>,
    /// An accepted Send's text is not yet durably cleared.
    pending_clear: bool,
}

impl Held {
    fn unsaved(&self) -> bool {
        self.text != self.saved
    }

    /// Takes the server's text as the baseline when it is newer or equal.
    /// Local unsaved text stays; a newer, different server text is a conflict.
    fn reconcile(&mut self, revision: i64, text: &str) {
        if revision < self.revision || self.pending_save.is_some() {
            return;
        }
        // Even saved text belongs to an unresolved Send. Do not replace it
        // with another writer's text under the submitted edit generation.
        if !self.unsaved() && self.pending_send.is_none() && !self.pending_clear {
            self.text = text.to_owned();
            self.saved = text.to_owned();
            self.revision = revision;
            self.conflict = None;
        } else if self.text == text {
            self.saved = text.to_owned();
            self.revision = revision;
            self.conflict = None;
        } else if revision > self.revision || self.saved != text {
            self.conflict = Some(ConflictDraft {
                revision,
                text: text.to_owned(),
            });
        }
    }

    /// A pending clear is done once nothing of the cleared text is unsaved.
    fn settle(&mut self) {
        if self.pending_clear
            && self.in_flight.is_none()
            && self.pending_save.is_none()
            && self.conflict.is_none()
            && !self.unsaved()
        {
            self.pending_clear = false;
        }
    }
}

/// Every draft target's text this page holds, and the one it shows.
///
/// Readers and writers name the target the page shows now, so a render or a
/// click that lands before [`Composer::open`] has run never reads or writes
/// another target's text or mark.
#[derive(Debug, Clone, Default)]
pub(crate) struct Composer {
    shown: Option<DraftTarget>,
    held: HashMap<DraftTarget, Held>,
    /// Another composer notice, such as dictation being unavailable.
    said: Option<&'static str>,
    next_attempt: u64,
}

/// The line beside the composer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Notice {
    None,
    NotSaved,
    Conflict,
    Said(&'static str),
}

impl Composer {
    /// Shows `target`'s composer. Another target clears the notice; its own
    /// text and revision come back as this page last held them.
    pub(crate) fn open(&mut self, target: Option<&DraftTarget>) {
        if self.shown.as_ref() == target {
            return;
        }
        self.said = None;
        if let Some(target) = target {
            self.held.entry(target.clone()).or_default();
        }
        self.shown = target.cloned();
    }

    fn held(&self, current: Option<&DraftTarget>) -> Option<&Held> {
        current.and_then(|target| self.held.get(target))
    }

    /// The composer text of the target shown now.
    pub(crate) fn text(&self, current: Option<&DraftTarget>) -> String {
        self.held(current)
            .map(|held| held.text.clone())
            .unwrap_or_default()
    }

    /// Whether a save of the target is running.
    #[cfg(test)]
    pub(crate) fn saving(&self, current: Option<&DraftTarget>) -> bool {
        self.held(current)
            .is_some_and(|held| held.in_flight.is_some())
    }

    /// The newer saved draft a conflict is waiting on.
    pub(crate) fn conflict(&self, current: Option<&DraftTarget>) -> Option<ConflictDraft> {
        self.held(current).and_then(|held| held.conflict.clone())
    }

    /// Whether an accepted Send's text is still waiting to be durably cleared.
    pub(crate) fn clearing(&self, current: Option<&DraftTarget>) -> bool {
        self.held(current).is_some_and(|held| held.pending_clear)
    }

    /// The Send whose acceptance is not yet known on this target.
    pub(crate) fn pending_send(&self, target: &DraftTarget) -> Option<PendingSend> {
        self.held
            .get(target)
            .and_then(|held| held.pending_send.clone())
    }

    /// Every target holding a Send of unknown acceptance.
    #[cfg(test)]
    pub(crate) fn pending_sends(&self) -> Vec<(DraftTarget, PendingSend)> {
        self.held
            .iter()
            .filter_map(|(target, held)| {
                held.pending_send.clone().map(|send| (target.clone(), send))
            })
            .collect()
    }

    /// The edit generation of `target`.
    pub(crate) fn generation(&self, target: &DraftTarget) -> u64 {
        self.held.get(target).map_or(0, |held| held.generation)
    }

    /// The line beside the composer of the target shown now. Routine saves
    /// say nothing; only a conflict or a failure does.
    pub(crate) fn notice(&self, current: Option<&DraftTarget>) -> Notice {
        if self.shown.as_ref() != current {
            return Notice::None;
        }
        if let Some(words) = self.said {
            return Notice::Said(words);
        }
        let Some(held) = self.held(current) else {
            return Notice::None;
        };
        if held.conflict.is_some() {
            Notice::Conflict
        } else if held.failed && held.in_flight.is_none() {
            Notice::NotSaved
        } else {
            Notice::None
        }
    }

    /// The physician typed. A failed save becomes eligible again.
    pub(crate) fn edit(&mut self, current: Option<&DraftTarget>, text: String) {
        self.open(current);
        let Some(target) = current else { return };
        if let Some(held) = self.held.get_mut(target) {
            if held.text == text {
                return;
            }
            held.text = text;
            held.generation += 1;
            held.failed = false;
            held.settle();
        }
    }

    /// Another composer notice replaces the line until the next save or view.
    pub(crate) fn say(&mut self, words: &'static str) {
        self.said = Some(words);
    }

    /// The server's saved draft for `target` arrived. It never replaces
    /// typing or a newer revision this page saved itself; a newer, different
    /// revision beside unsaved text is a conflict the physician resolves.
    pub(crate) fn loaded(&mut self, target: &DraftTarget, revision: i64, text: &str) {
        let held = self.held.entry(target.clone()).or_default();
        if held.in_flight.is_some() {
            // The running save's answer settles the baseline.
            return;
        }
        held.reconcile(revision, text);
        held.settle();
    }

    /// Restores a reloaded tab's unsent text. Ignored once the physician has
    /// typed here, so recovery never replaces typing. Returns whether it was
    /// taken.
    pub(crate) fn restore(&mut self, target: &DraftTarget, recovery: Recovery) -> bool {
        let held = self.held.entry(target.clone()).or_default();
        if held.generation > 0 || held.in_flight.is_some() || held.pending_send.is_some() {
            return false;
        }
        let server = (held.revision, held.saved.clone());
        let known = server.0 > 0 || !server.1.is_empty();
        held.text = recovery.text;
        held.saved = recovery.saved;
        held.revision = recovery.revision;
        held.generation = recovery.generation.max(1);
        held.conflict = recovery.conflict;
        held.pending_send = recovery.pending_send;
        held.pending_clear = recovery.pending_clear;
        held.pending_save = recovery.pending_save;
        if known {
            held.reconcile(server.0, &server.1);
        }
        held.settle();
        true
    }

    /// What same-tab recovery should keep for `target`, or `None` when the
    /// server already holds everything this page has (the record is removed).
    pub(crate) fn recovery(&self, target: &DraftTarget) -> Option<Recovery> {
        let held = self.held.get(target)?;
        let needed = held.unsaved()
            || held.in_flight.is_some()
            || held.pending_save.is_some()
            || held.conflict.is_some()
            || held.pending_send.is_some()
            || held.pending_clear;
        needed.then(|| Recovery {
            text: held.text.clone(),
            saved: held.saved.clone(),
            revision: held.revision,
            generation: held.generation,
            conflict: held.conflict.clone(),
            pending_send: held.pending_send.clone(),
            pending_clear: held.pending_clear,
            pending_save: held.pending_save.clone(),
        })
    }

    /// Whether `target` has text an autosave should write now.
    pub(crate) fn wants_save(&self, target: &DraftTarget) -> bool {
        self.held.get(target).is_some_and(|held| {
            (held.unsaved() || held.pending_save.is_some())
                && held.in_flight.is_none()
                && held.conflict.is_none()
                && !held.failed
        })
    }

    /// Starts saving `target`'s text over its baseline revision. Nothing
    /// starts while its save runs, while a conflict waits, after a failure
    /// (until an edit or [`Composer::retry_failed`]) or with nothing unsaved.
    pub(crate) fn begin(&mut self, target: &DraftTarget) -> Option<SaveAttempt> {
        if !self.wants_save(target) {
            return None;
        }
        let id = self.next_attempt + 1;
        let shown = self.shown.as_ref() == Some(target);
        let held = self.held.get_mut(target)?;
        self.next_attempt = id;
        held.in_flight = Some(id);
        let pending = held
            .pending_save
            .get_or_insert_with(|| ConflictDraft {
                revision: held.revision,
                text: held.text.clone(),
            })
            .clone();
        if shown {
            self.said = None;
        }
        Some(SaveAttempt {
            id,
            target: target.clone(),
            generation: held.generation,
            expected_revision: pending.revision,
            text: pending.text,
        })
    }

    /// The server answered `attempt`. The baseline moves even when another
    /// surface is shown.
    pub(crate) fn finish(&mut self, attempt: &SaveAttempt, outcome: SaveOutcome) {
        let Some(held) = self.held.get_mut(&attempt.target) else {
            return;
        };
        if held.in_flight != Some(attempt.id) {
            return;
        }
        held.in_flight = None;
        match outcome {
            SaveOutcome::Saved(revision) => {
                held.pending_save = None;
                if revision >= held.revision {
                    held.revision = revision;
                    held.saved = attempt.text.clone();
                }
                held.failed = false;
                held.failures = 0;
            }
            SaveOutcome::Conflict(revision, text) => {
                held.pending_save = None;
                held.failures = 0;
                // The server holds this page's text already (a lost answer):
                // take its revision as the baseline instead of a conflict.
                if text == held.text || text == attempt.text {
                    if revision >= held.revision {
                        held.revision = revision;
                        held.saved = text;
                    }
                } else {
                    held.conflict = Some(ConflictDraft { revision, text });
                }
            }
            SaveOutcome::Failed => {
                held.failed = true;
                held.failures = held.failures.saturating_add(1);
            }
        }
        held.settle();
    }

    /// How many saves of `target` failed in a row while its last one is
    /// still failed; `None` when nothing is waiting on a retry.
    pub(crate) fn failing(&self, target: &DraftTarget) -> Option<u32> {
        self.held
            .get(target)
            .filter(|held| held.failed)
            .map(|held| held.failures)
    }

    /// Forgets every target of `workspace` (its access was removed). A save
    /// still running for it lands on nothing.
    pub(crate) fn forget_workspace(&mut self, workspace: &str) {
        self.held.retain(|target, _| target.workspace != workspace);
    }

    /// A failed save may run again (the shell's slow retry).
    pub(crate) fn retry_failed(&mut self, target: &DraftTarget) {
        if let Some(held) = self.held.get_mut(target) {
            held.failed = false;
        }
    }

    /// **Retry**: replace the saved draft with this page's text, over exactly
    /// the revision the conflict showed. The next save names that revision;
    /// if the draft moved again, another conflict shows.
    pub(crate) fn retry(&mut self, current: Option<&DraftTarget>) {
        let Some(held) = current.and_then(|target| self.held.get_mut(target)) else {
            return;
        };
        let Some(conflict) = held.conflict.take() else {
            return;
        };
        held.revision = conflict.revision;
        held.saved = conflict.text;
        held.failed = false;
        held.settle();
    }

    /// **Use newer text**: discard the local edit and show the newer draft.
    /// A clear waiting behind the conflict is abandoned: the newer text is
    /// the physician's choice.
    pub(crate) fn take_newer(&mut self, current: Option<&DraftTarget>) {
        let Some(held) = current.and_then(|target| self.held.get_mut(target)) else {
            return;
        };
        let Some(conflict) = held.conflict.take() else {
            return;
        };
        held.text = conflict.text.clone();
        held.saved = conflict.text;
        held.revision = conflict.revision;
        held.generation += 1;
        held.failed = false;
        held.pending_clear = false;
        held.settle();
    }

    /// Captures a Send of `text` on `target` with its command. Refused when
    /// the text is not what the composer shows, is blank, or another Send on
    /// this target has not been answered yet.
    pub(crate) fn begin_send(
        &mut self,
        target: &DraftTarget,
        key: String,
        text: &str,
        command: serde_json::Value,
    ) -> Option<PendingSend> {
        let held = self.held.get_mut(target)?;
        if held.pending_send.is_some() || held.text != text || text.trim().is_empty() {
            return None;
        }
        let send = PendingSend {
            key,
            text: text.to_owned(),
            generation: held.generation,
            command,
        };
        held.pending_send = Some(send.clone());
        Some(send)
    }

    /// The server answered the Send keyed `key` on `target`. Acceptance
    /// removes only the submitted text and queues its durable clear behind any
    /// running save; refusal keeps the text as an ordinary unsent draft.
    pub(crate) fn sent(&mut self, target: &DraftTarget, key: &str, answer: SendAnswer) {
        let Some(held) = self.held.get_mut(target) else {
            return;
        };
        let Some(send) = held.pending_send.take_if(|send| send.key == key) else {
            return;
        };
        if answer == SendAnswer::Refused {
            return;
        }
        let rest = if held.generation == send.generation || held.text == send.text {
            Some(String::new())
        } else {
            held.text
                .strip_prefix(&send.text)
                .map(|rest| rest.trim_start().to_owned())
        };
        if let Some(rest) = rest
            && rest != held.text
        {
            held.text = rest;
            held.generation += 1;
        }
        held.pending_clear = true;
        held.settle();
    }
}

#[cfg(test)]
mod tests {
    use super::{Composer, ConflictDraft, DraftTarget, Notice, Recovery, SaveOutcome, SendAnswer};

    fn chat(id: &str) -> DraftTarget {
        DraftTarget::new("pw-1", Some(id))
    }

    fn opened(target: &DraftTarget, revision: i64, text: &str) -> Composer {
        let mut composer = Composer::default();
        composer.open(Some(target));
        composer.loaded(target, revision, text);
        composer
    }

    fn command() -> serde_json::Value {
        serde_json::json!({ "type": "message.submit" })
    }

    #[test]
    fn before_a_chat_opens_the_draft_is_saved_under_the_draft_chat_id() {
        assert_eq!(DraftTarget::new("pw-1", None).chat, "draft");
    }

    #[test]
    fn a_never_saved_draft_saves_over_revision_zero() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "first".into());
        let attempt = composer.begin(&one).unwrap();
        assert_eq!(
            (attempt.expected_revision, attempt.text.as_str()),
            (0, "first")
        );
        assert_eq!(composer.notice(at), Notice::None);
        composer.finish(&attempt, SaveOutcome::Saved(1));
        assert_eq!(composer.notice(at), Notice::None);
        composer.edit(at, "first, then more".into());
        assert_eq!(composer.notice(at), Notice::None);
        assert_eq!(composer.begin(&one).unwrap().expected_revision, 1);
    }

    #[test]
    fn nothing_saves_when_the_text_is_already_the_saved_text() {
        let one = chat("chat-1");
        let mut composer = opened(&one, 2, "saved");
        assert!(!composer.wants_save(&one));
        composer.edit(Some(&one), "saved!".into());
        composer.edit(Some(&one), "saved".into());
        assert!(composer.begin(&one).is_none());
    }

    #[test]
    fn the_saved_draft_is_shown_and_saved_over_its_own_revision() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 2, "from the other tab");
        assert_eq!(composer.text(at), "from the other tab");
        composer.edit(at, "from the other tab, edited here".into());
        assert_eq!(composer.begin(&one).unwrap().expected_revision, 2);
    }

    /// Typing during a long PUT: the late answer moves the baseline to the
    /// text it saved, never over the newer typing, and a trailing save is due.
    #[test]
    fn edits_during_a_save_keep_the_typing_and_need_a_trailing_save() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let first = composer.begin(&one).unwrap();
        composer.edit(at, "ab".into());
        assert!(composer.begin(&one).is_none(), "one save per target");
        composer.finish(&first, SaveOutcome::Saved(1));
        assert_eq!(composer.text(at), "ab");
        assert_eq!(composer.notice(at), Notice::None);
        assert!(composer.wants_save(&one));
        let trailing = composer.begin(&one).unwrap();
        assert_eq!(
            (trailing.expected_revision, trailing.text.as_str()),
            (1, "ab")
        );
        assert!(trailing.generation > first.generation);
        composer.finish(&trailing, SaveOutcome::Saved(2));
        assert_eq!(composer.notice(at), Notice::None);
        assert!(!composer.wants_save(&one));
        assert!(composer.recovery(&one).is_none());
    }

    /// The stale-write fence: this page read revision 0, another tab saved
    /// revision 2. Autosave pauses and shows the newer text's revision.
    #[test]
    fn a_validated_conflict_pauses_autosave_until_the_physician_chooses() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "older text".into());
        let attempt = composer.begin(&one).unwrap();
        composer.finish(&attempt, SaveOutcome::Conflict(2, "newer".into()));
        assert_eq!(composer.notice(at), Notice::Conflict);
        assert_eq!(
            composer.conflict(at),
            Some(ConflictDraft {
                revision: 2,
                text: "newer".into()
            })
        );
        assert_eq!(composer.text(at), "older text");
        // Typing does not resume autosave over the conflict.
        composer.edit(at, "older text, more".into());
        assert!(composer.begin(&one).is_none());
    }

    #[test]
    fn retry_names_exactly_the_conflicting_revision_and_conflicts_again_if_it_moved() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "mine".into());
        let attempt = composer.begin(&one).unwrap();
        composer.finish(&attempt, SaveOutcome::Conflict(2, "theirs".into()));
        composer.retry(at);
        let again = composer.begin(&one).unwrap();
        assert_eq!((again.expected_revision, again.text.as_str()), (2, "mine"));
        // A third tab saved in between: another conflict, nothing overwritten.
        composer.finish(&again, SaveOutcome::Conflict(3, "third".into()));
        assert_eq!(composer.conflict(at).unwrap().revision, 3);
        composer.retry(at);
        let last = composer.begin(&one).unwrap();
        assert_eq!(last.expected_revision, 3);
        composer.finish(&last, SaveOutcome::Saved(4));
        assert_eq!(composer.notice(at), Notice::None);
        assert_eq!(composer.text(at), "mine");
    }

    #[test]
    fn use_newer_text_discards_the_local_edit_and_takes_the_baseline() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "mine".into());
        let attempt = composer.begin(&one).unwrap();
        composer.finish(&attempt, SaveOutcome::Conflict(5, "theirs".into()));
        let before = composer.generation(&one);
        composer.take_newer(at);
        assert_eq!(composer.text(at), "theirs");
        assert!(composer.generation(&one) > before);
        assert_eq!(composer.notice(at), Notice::None);
        assert!(!composer.wants_save(&one));
        assert!(composer.recovery(&one).is_none());
        composer.edit(at, "theirs, edited".into());
        assert_eq!(composer.begin(&one).unwrap().expected_revision, 5);
    }

    /// The save landed but its answer was lost; the next save conflicts with
    /// this page's own text, which is a baseline, not a conflict.
    #[test]
    fn a_lost_answer_reconciles_when_the_server_holds_this_pages_text() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "mine".into());
        let lost = composer.begin(&one).unwrap();
        composer.finish(&lost, SaveOutcome::Failed);
        assert_eq!(composer.notice(at), Notice::NotSaved);
        assert!(composer.begin(&one).is_none(), "no tight retry loop");
        composer.retry_failed(&one);
        let again = composer.begin(&one).unwrap();
        assert_eq!(again.expected_revision, 0);
        composer.finish(&again, SaveOutcome::Conflict(1, "mine".into()));
        assert_eq!(composer.conflict(at), None);
        assert!(!composer.wants_save(&one));
    }

    #[test]
    fn a_failure_is_not_a_conflict_and_an_edit_makes_the_save_eligible_again() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let attempt = composer.begin(&one).unwrap();
        composer.finish(&attempt, SaveOutcome::Failed);
        assert_eq!(composer.conflict(at), None);
        assert_eq!(composer.notice(at), Notice::NotSaved);
        composer.edit(at, "ab".into());
        assert_eq!(composer.begin(&one).unwrap().expected_revision, 0);
    }

    #[test]
    fn text_typed_before_the_saved_draft_arrives_is_kept_and_a_newer_draft_conflicts() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = Composer::default();
        composer.edit(at, "typed early".into());
        composer.loaded(&one, 3, "saved elsewhere");
        assert_eq!(composer.text(at), "typed early");
        assert_eq!(composer.conflict(at).unwrap().revision, 3);
        // No draft on the server yet: typing early simply saves over zero.
        let two = chat("chat-2");
        composer.edit(Some(&two), "early".into());
        composer.loaded(&two, 0, "");
        assert_eq!(composer.conflict(Some(&two)), None);
        assert_eq!(composer.begin(&two).unwrap().expected_revision, 0);
    }

    #[test]
    fn a_read_older_than_this_pages_own_save_or_during_it_changes_nothing() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "mine".into());
        let attempt = composer.begin(&one).unwrap();
        composer.loaded(&one, 0, "");
        composer.finish(&attempt, SaveOutcome::Saved(1));
        composer.loaded(&one, 0, "");
        assert_eq!(composer.text(at), "mine");
        composer.edit(at, "mine, more".into());
        assert_eq!(composer.begin(&one).unwrap().expected_revision, 1);
    }

    #[test]
    fn a_newer_notice_outlives_the_save_it_replaced() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let attempt = composer.begin(&one).unwrap();
        composer.say("Dictation is unavailable.");
        composer.finish(&attempt, SaveOutcome::Failed);
        assert_eq!(
            composer.notice(at),
            Notice::Said("Dictation is unavailable.")
        );
    }

    #[test]
    fn an_answer_for_an_attempt_that_is_not_running_changes_nothing() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let first = composer.begin(&one).unwrap();
        composer.finish(&first, SaveOutcome::Saved(1));
        composer.edit(at, "ab".into());
        let second = composer.begin(&one).unwrap();
        composer.finish(&first, SaveOutcome::Failed);
        assert_eq!(composer.notice(at), Notice::None);
        composer.finish(&second, SaveOutcome::Saved(2));
        assert_eq!(composer.notice(at), Notice::None);
    }

    /// Navigating mid-save: the other chat never shows the first chat's
    /// notice, and the first chat keeps its text and the revision it saved.
    #[test]
    fn a_save_that_ends_after_navigation_marks_neither_surface() {
        let (one, two) = (chat("chat-1"), chat("chat-2"));
        let mut composer = opened(&one, 0, "");
        composer.edit(Some(&one), "for chat one".into());
        let attempt = composer.begin(&one).unwrap();
        assert_eq!(
            (composer.notice(Some(&two)), composer.text(Some(&two))),
            (Notice::None, String::new())
        );
        composer.open(Some(&two));
        assert!(!composer.saving(Some(&two)));
        composer.finish(&attempt, SaveOutcome::Saved(1));
        assert_eq!(composer.notice(Some(&two)), Notice::None);
        composer.open(Some(&one));
        assert_eq!(
            (composer.notice(Some(&one)), composer.text(Some(&one))),
            (Notice::None, "for chat one".to_owned())
        );
        composer.edit(Some(&one), "for chat one!".into());
        assert_eq!(composer.begin(&one).unwrap().expected_revision, 1);
    }

    #[test]
    fn leaving_the_patient_page_clears_the_mark() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let attempt = composer.begin(&one).unwrap();
        composer.open(None);
        assert_eq!(composer.notice(None), Notice::None);
        composer.open(at);
        composer.finish(&attempt, SaveOutcome::Saved(1));
        assert_eq!(composer.notice(at), Notice::None);
    }

    #[test]
    fn another_patients_chat_of_the_same_id_is_its_own_draft() {
        let one = chat("chat-1");
        let mut composer = opened(&one, 0, "");
        composer.edit(Some(&one), "for pw-1".into());
        let other = DraftTarget::new("pw-2", Some("chat-1"));
        composer.open(Some(&other));
        assert_eq!(composer.text(Some(&other)), "");
        composer.loaded(&other, 4, "pw-2's draft");
        assert_eq!(composer.text(Some(&other)), "pw-2's draft");
        composer.edit(Some(&other), "pw-2's draft, more".into());
        assert_eq!(composer.begin(&other).unwrap().expected_revision, 4);
    }

    #[test]
    fn an_accepted_send_clears_only_the_submitted_text() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "Check potassium".into());
        let send = composer
            .begin_send(&one, "k1".into(), "Check potassium", command())
            .unwrap();
        assert!(
            composer
                .begin_send(&one, "k2".into(), "Check potassium", command())
                .is_none(),
            "one unanswered Send per target"
        );
        composer.edit(at, "Check potassium and sodium".into());
        composer.sent(&one, &send.key, SendAnswer::Accepted);
        assert_eq!(composer.text(at), "and sodium");
        assert!(composer.clearing(at));
        // The clear is an ordinary save of what remains.
        let clear = composer.begin(&one).unwrap();
        assert_eq!(clear.text, "and sodium");
        composer.finish(&clear, SaveOutcome::Saved(1));
        assert!(!composer.clearing(at));
    }

    #[test]
    fn edited_text_that_no_longer_starts_with_the_message_is_kept_whole() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "hello".into());
        let send = composer
            .begin_send(&one, "k1".into(), "hello", command())
            .unwrap();
        composer.edit(at, "rewritten".into());
        composer.sent(&one, &send.key, SendAnswer::Accepted);
        assert_eq!(composer.text(at), "rewritten");
    }

    #[test]
    fn a_refused_send_keeps_the_text_as_an_ordinary_draft() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "hello".into());
        let send = composer
            .begin_send(&one, "k1".into(), "hello", command())
            .unwrap();
        composer.sent(&one, &send.key, SendAnswer::Refused);
        assert_eq!(composer.text(at), "hello");
        assert!(composer.pending_send(&one).is_none());
        assert!(!composer.clearing(at));
    }

    /// Acceptance lands after the physician moved to another chat: it clears
    /// the chat it was sent from, never the one now shown.
    #[test]
    fn acceptance_after_navigation_clears_the_sending_chat_only() {
        let (one, two) = (chat("chat-1"), chat("chat-2"));
        let mut composer = opened(&one, 0, "");
        composer.edit(Some(&one), "for one".into());
        let send = composer
            .begin_send(&one, "k1".into(), "for one", command())
            .unwrap();
        composer.open(Some(&two));
        composer.edit(Some(&two), "for two".into());
        composer.sent(&one, &send.key, SendAnswer::Accepted);
        assert_eq!(composer.text(Some(&two)), "for two");
        assert_eq!(composer.text(Some(&one)), "");
    }

    /// Send while the first (revision-zero) autosave is running: the clear
    /// waits for it and then saves over revision 1, never a concurrent PUT.
    #[test]
    fn a_clear_waits_behind_the_revision_zero_save() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "hello".into());
        let first = composer.begin(&one).unwrap();
        let send = composer
            .begin_send(&one, "k1".into(), "hello", command())
            .unwrap();
        composer.sent(&one, &send.key, SendAnswer::Accepted);
        assert_eq!(composer.text(at), "");
        assert!(composer.begin(&one).is_none(), "no concurrent empty PUT");
        assert!(composer.clearing(at));
        composer.finish(&first, SaveOutcome::Saved(1));
        assert!(composer.clearing(at), "the server still holds the message");
        let clear = composer.begin(&one).unwrap();
        assert_eq!((clear.expected_revision, clear.text.as_str()), (1, ""));
        composer.finish(&clear, SaveOutcome::Saved(2));
        assert!(!composer.clearing(at));
        assert!(composer.recovery(&one).is_none());
    }

    #[test]
    fn a_send_that_was_never_autosaved_needs_no_durable_clear() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "hello".into());
        let send = composer
            .begin_send(&one, "k1".into(), "hello", command())
            .unwrap();
        composer.sent(&one, &send.key, SendAnswer::Accepted);
        assert!(!composer.clearing(at));
        assert!(!composer.wants_save(&one));
    }

    /// Another tab saved newer text before this tab's clear: the clear
    /// conflicts and waits for the physician; nothing is cleared over it.
    #[test]
    fn another_tabs_newer_revision_is_never_cleared_automatically() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 1, "hello");
        let send = composer
            .begin_send(&one, "k1".into(), "hello", command())
            .unwrap();
        composer.sent(&one, &send.key, SendAnswer::Accepted);
        let clear = composer.begin(&one).unwrap();
        composer.finish(
            &clear,
            SaveOutcome::Conflict(2, "hello from tab two".into()),
        );
        assert_eq!(composer.notice(at), Notice::Conflict);
        assert!(composer.clearing(at));
        assert!(composer.begin(&one).is_none());
        composer.take_newer(at);
        assert_eq!(composer.text(at), "hello from tab two");
        assert!(!composer.clearing(at));
    }

    #[test]
    fn recovery_restores_unsent_text_and_drops_once_saved() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "unsaved".into());
        let record = composer.recovery(&one).unwrap();
        assert_eq!((record.text.as_str(), record.revision), ("unsaved", 0));

        // A reload: a fresh page restores before or after its read.
        let mut reloaded = Composer::default();
        reloaded.open(at);
        assert!(reloaded.restore(&one, record.clone()));
        reloaded.loaded(&one, 0, "");
        assert_eq!(reloaded.text(at), "unsaved");
        assert_eq!(reloaded.conflict(at), None);
        let save = reloaded.begin(&one).unwrap();
        reloaded.finish(&save, SaveOutcome::Saved(1));
        assert!(reloaded.recovery(&one).is_none());

        let mut late = Composer::default();
        late.open(at);
        late.loaded(&one, 0, "");
        assert!(late.restore(&one, record));
        assert_eq!(late.text(at), "unsaved");
    }

    #[test]
    fn recovery_never_replaces_typing_after_reload() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = Composer::default();
        composer.open(at);
        composer.edit(at, "typed now".into());
        let record = Recovery {
            text: "old".into(),
            saved: String::new(),
            revision: 0,
            generation: 3,
            conflict: None,
            pending_send: None,
            pending_clear: false,
            pending_save: None,
        };
        assert!(!composer.restore(&one, record));
        assert_eq!(composer.text(at), "typed now");
    }

    /// Reload during a PUT whose answer was lost: the server holds the
    /// recovered text at a newer revision, which reconciles silently.
    #[test]
    fn a_stale_recovered_baseline_reconciles_or_asks() {
        let one = chat("chat-1");
        let at = Some(&one);
        let record = Recovery {
            text: "mine".into(),
            saved: String::new(),
            revision: 0,
            generation: 2,
            conflict: None,
            pending_send: None,
            pending_clear: false,
            pending_save: None,
        };
        let mut same = Composer::default();
        same.open(at);
        same.restore(&one, record.clone());
        same.loaded(&one, 1, "mine");
        assert_eq!(same.conflict(at), None);
        assert!(!same.wants_save(&one));

        let mut other = Composer::default();
        other.open(at);
        other.loaded(&one, 4, "another tab");
        other.restore(&one, record);
        assert_eq!(other.text(at), "mine");
        assert_eq!(other.conflict(at).unwrap().revision, 4);
    }

    #[test]
    fn a_pending_send_survives_reload_until_answered() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "hello".into());
        composer
            .begin_send(&one, "k1".into(), "hello", command())
            .unwrap();
        let record = composer.recovery(&one).unwrap();
        assert_eq!(record.pending_send.as_ref().unwrap().key, "k1");

        let mut reloaded = Composer::default();
        reloaded.open(at);
        reloaded.restore(&one, record);
        let pending = reloaded.pending_sends();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].1.command, command());
        reloaded.sent(&one, "k1", SendAnswer::Accepted);
        assert_eq!(reloaded.text(at), "");
        assert!(reloaded.recovery(&one).is_none());
    }

    #[test]
    fn recovered_send_never_clears_another_writers_newer_draft() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut original = opened(&one, 1, "submitted");
        original
            .begin_send(&one, "k1".into(), "submitted", command())
            .unwrap();
        let record = original.recovery(&one).unwrap();
        // Both orders of the independent workspace/recovery and draft reads.
        for read_first in [false, true] {
            let mut page = opened(&one, 0, "");
            if read_first {
                page.loaded(&one, 2, "another writer");
            }
            page.restore(&one, record.clone());
            if !read_first {
                page.loaded(&one, 2, "another writer");
            }
            page.sent(&one, "k1", SendAnswer::Accepted);
            assert_eq!(page.text(at), "");
            assert_eq!(page.conflict(at).unwrap().text, "another writer");
            assert!(page.begin(&one).is_none());
        }
    }

    #[test]
    fn reload_replays_unanswered_first_save_before_clearing_sent_text() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut original = opened(&one, 0, "");
        original.edit(at, "submitted".into());
        original.begin(&one).unwrap();
        original
            .begin_send(&one, "k1".into(), "submitted", command())
            .unwrap();
        original.sent(&one, "k1", SendAnswer::Accepted);
        let record: Recovery = serde_json::from_str(
            &serde_json::to_string(&original.recovery(&one).unwrap()).unwrap(),
        )
        .unwrap();
        for committed in [false, true] {
            let mut page = opened(&one, 0, "");
            page.restore(&one, record.clone());
            page.loaded(
                &one,
                if committed { 1 } else { 0 },
                if committed { "submitted" } else { "" },
            );
            assert!(page.clearing(at));
            assert_eq!(page.text(at), "");
            let replay = page.begin(&one).unwrap();
            assert_eq!(
                (replay.expected_revision, replay.text.as_str()),
                (0, "submitted")
            );
            page.finish(
                &replay,
                if committed {
                    SaveOutcome::Conflict(1, "submitted".into())
                } else {
                    SaveOutcome::Saved(1)
                },
            );
            let clear = page.begin(&one).unwrap();
            assert_eq!((clear.expected_revision, clear.text.as_str()), (1, ""));
            page.finish(&clear, SaveOutcome::Saved(2));
            assert!(page.recovery(&one).is_none());
        }
    }

    #[test]
    fn replayed_first_save_preserves_another_writers_text() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut original = opened(&one, 0, "");
        original.edit(at, "submitted".into());
        original.begin(&one).unwrap();
        original
            .begin_send(&one, "k1".into(), "submitted", command())
            .unwrap();
        original.sent(&one, "k1", SendAnswer::Accepted);
        let mut page = opened(&one, 0, "");
        page.restore(&one, original.recovery(&one).unwrap());
        let replay = page.begin(&one).unwrap();
        page.finish(&replay, SaveOutcome::Conflict(1, "another writer".into()));
        assert!(page.begin(&one).is_none());
        assert_eq!(page.conflict(at).unwrap().text, "another writer");
    }

    #[test]
    fn a_pending_clear_survives_reload_until_confirmed() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 1, "hello");
        let send = composer
            .begin_send(&one, "k1".into(), "hello", command())
            .unwrap();
        composer.sent(&one, &send.key, SendAnswer::Accepted);
        let record = composer.recovery(&one).unwrap();
        assert!(record.pending_clear);
        let mut reloaded = Composer::default();
        reloaded.open(at);
        reloaded.restore(&one, record);
        reloaded.loaded(&one, 1, "hello");
        assert!(reloaded.clearing(at));
        let clear = reloaded.begin(&one).unwrap();
        assert_eq!((clear.expected_revision, clear.text.as_str()), (1, ""));
        reloaded.finish(&clear, SaveOutcome::Saved(2));
        assert!(reloaded.recovery(&one).is_none());
    }
}

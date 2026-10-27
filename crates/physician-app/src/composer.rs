//! The composer's draft text and its Save mark, kept per draft target.
//!
//! A target is one patient workspace and one chat (the chat id `draft` before
//! a chat is open). The shell is rebuilt for each physician, so everything
//! here already belongs to the signed-in physician.
//!
//! Each target remembers the server revision its text was read or last saved
//! at. Save names that revision, so a tab holding older text than another
//! tab's newer save is refused (409) and shows Not saved instead of
//! overwriting it. One save runs per target at a time, every save carries its
//! own attempt id, and only that attempt, still beside its own target's
//! composer, may set the mark. Showing another target clears the mark.
use std::collections::HashMap;

/// The chat id the page saves under before a chat is open.
pub(crate) const UNOPENED_CHAT: &str = "draft";

/// One patient workspace and chat: what a draft belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// The mark beside the composer.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum Mark {
    #[default]
    None,
    /// The named attempt is running.
    Saving(u64),
    Saved,
    NotSaved,
    /// Another composer notice, such as dictation being unavailable.
    Said(&'static str),
}

/// One save on its way: what it saves, where, and over which revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SaveAttempt {
    pub(crate) id: u64,
    pub(crate) target: DraftTarget,
    pub(crate) expected_revision: i64,
    pub(crate) text: String,
}

/// How the server answered a save.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SaveOutcome {
    /// Saved; the draft is now at this revision.
    Saved(i64),
    /// Another save got there first (409); nothing was written.
    Conflict,
    /// The save did not reach the server or was refused for another reason.
    Failed,
}

/// One target's composer text.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Held {
    text: String,
    /// The server's text at `revision`, as this page last read or saved it.
    saved: String,
    /// The revision `saved` is at; 0 is the server's "no draft yet".
    revision: i64,
    in_flight: Option<u64>,
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
    mark: Mark,
    next_attempt: u64,
}

impl Composer {
    /// Shows `target`'s composer. Another target clears the mark; its own
    /// text and revision come back as this page last held them.
    pub(crate) fn open(&mut self, target: Option<&DraftTarget>) {
        if self.shown.as_ref() == target {
            return;
        }
        self.mark = Mark::None;
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

    /// Whether a save of the target shown now is running; Save waits for it.
    pub(crate) fn saving(&self, current: Option<&DraftTarget>) -> bool {
        self.held(current)
            .is_some_and(|held| held.in_flight.is_some())
    }

    /// The mark's words beside the composer of the target shown now.
    pub(crate) fn notice(&self, current: Option<&DraftTarget>) -> &'static str {
        if self.shown.as_ref() != current {
            return "";
        }
        match self.mark {
            Mark::None => "",
            Mark::Saving(_) => crate::SAVING,
            Mark::Saved => crate::SAVED,
            Mark::NotSaved => crate::NOT_SAVED,
            Mark::Said(words) => words,
        }
    }

    /// The physician typed. "Saved" described the saved text; an edit makes it untrue.
    pub(crate) fn edit(&mut self, current: Option<&DraftTarget>, text: String) {
        self.open(current);
        let Some(target) = current else { return };
        if let Some(held) = self.held.get_mut(target) {
            held.text = text;
        }
        if self.mark == Mark::Saved {
            self.mark = Mark::None;
        }
    }

    /// Another composer notice replaces the mark; a running save keeps going
    /// but no longer speaks when it ends.
    pub(crate) fn say(&mut self, words: &'static str) {
        self.mark = Mark::Said(words);
    }

    /// The server's saved draft for `target` arrived. It is taken only when
    /// the composer holds nothing unsaved for that target and no save is
    /// running, and never replaces a newer revision this page saved itself.
    pub(crate) fn loaded(&mut self, target: &DraftTarget, revision: i64, text: &str) {
        let held = self.held.entry(target.clone()).or_default();
        if held.text != held.saved || held.in_flight.is_some() || revision < held.revision {
            return;
        }
        held.text = text.to_owned();
        held.saved = text.to_owned();
        held.revision = revision;
    }

    /// Starts saving the shown text over the revision it was read or saved
    /// at. Nothing starts without a target or while its save runs.
    pub(crate) fn begin(&mut self, current: Option<&DraftTarget>) -> Option<SaveAttempt> {
        self.open(current);
        let target = current?.clone();
        let id = self.next_attempt + 1;
        let held = self.held.get_mut(&target)?;
        if held.in_flight.is_some() {
            return None;
        }
        self.next_attempt = id;
        held.in_flight = Some(id);
        let attempt = SaveAttempt {
            id,
            target,
            expected_revision: held.revision,
            text: held.text.clone(),
        };
        self.mark = Mark::Saving(id);
        Some(attempt)
    }

    /// The server answered `attempt`. A save moves its target's revision
    /// even when another surface is shown; the mark changes only while this
    /// attempt's own Saving mark is still beside its target's composer.
    pub(crate) fn finish(&mut self, attempt: &SaveAttempt, outcome: SaveOutcome) {
        let Some(held) = self.held.get_mut(&attempt.target) else {
            return;
        };
        if held.in_flight != Some(attempt.id) {
            return;
        }
        held.in_flight = None;
        if let SaveOutcome::Saved(revision) = outcome {
            held.revision = revision;
            held.saved = attempt.text.clone();
        }
        let unchanged = held.text == attempt.text;
        if self.shown.as_ref() != Some(&attempt.target) || self.mark != Mark::Saving(attempt.id) {
            return;
        }
        self.mark = match outcome {
            SaveOutcome::Saved(_) if unchanged => Mark::Saved,
            SaveOutcome::Saved(_) => Mark::None,
            SaveOutcome::Conflict | SaveOutcome::Failed => Mark::NotSaved,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{Composer, DraftTarget, Mark, SaveOutcome};

    fn chat(id: &str) -> DraftTarget {
        DraftTarget::new("pw-1", Some(id))
    }

    fn opened(target: &DraftTarget, revision: i64, text: &str) -> Composer {
        let mut composer = Composer::default();
        composer.open(Some(target));
        composer.loaded(target, revision, text);
        composer
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
        let attempt = composer.begin(at).unwrap();
        assert_eq!(
            (attempt.expected_revision, attempt.text.as_str()),
            (0, "first")
        );
        assert_eq!(composer.notice(at), "Saving…");
        composer.finish(&attempt, SaveOutcome::Saved(1));
        assert_eq!(composer.notice(at), "Saved");
        composer.edit(at, "first, then more".into());
        assert_eq!(composer.notice(at), "");
        assert_eq!(composer.begin(at).unwrap().expected_revision, 1);
    }

    #[test]
    fn the_saved_draft_is_shown_and_saved_over_its_own_revision() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 2, "from the other tab");
        assert_eq!(composer.text(at), "from the other tab");
        composer.edit(at, "from the other tab, edited here".into());
        assert_eq!(composer.begin(at).unwrap().expected_revision, 2);
    }

    /// The stale-write fence: this page's text was read at revision 0, so the
    /// newer revision another tab saved refuses it and nothing is overwritten.
    #[test]
    fn a_stale_tab_names_the_revision_it_read_and_shows_not_saved() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "older text".into());
        let attempt = composer.begin(at).unwrap();
        assert_eq!(attempt.expected_revision, 0);
        composer.finish(&attempt, SaveOutcome::Conflict);
        assert_eq!(composer.notice(at), "Not saved");
        assert_eq!(composer.text(at), "older text");
        // Saving again still names the revision this text came from.
        assert_eq!(composer.begin(at).unwrap().expected_revision, 0);
    }

    #[test]
    fn text_typed_before_the_saved_draft_arrives_is_kept_and_stays_fenced() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = Composer::default();
        composer.edit(at, "typed early".into());
        composer.loaded(&one, 3, "saved elsewhere");
        assert_eq!(composer.text(at), "typed early");
        assert_eq!(composer.begin(at).unwrap().expected_revision, 0);
    }

    #[test]
    fn a_read_older_than_this_pages_own_save_changes_nothing() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "mine".into());
        let attempt = composer.begin(at).unwrap();
        composer.loaded(&one, 0, "");
        composer.finish(&attempt, SaveOutcome::Saved(1));
        composer.loaded(&one, 0, "");
        assert_eq!(composer.text(at), "mine");
        assert_eq!(composer.begin(at).unwrap().expected_revision, 1);
    }

    #[test]
    fn one_save_runs_per_target_and_save_waits_for_it() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let first = composer.begin(at).unwrap();
        assert!(composer.saving(at));
        assert!(composer.begin(at).is_none());
        composer.finish(&first, SaveOutcome::Saved(1));
        assert!(!composer.saving(at));
        assert_eq!(composer.begin(at).unwrap().id, first.id + 1);
    }

    #[test]
    fn an_edit_during_the_save_leaves_no_saved_mark_but_keeps_the_revision() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let attempt = composer.begin(at).unwrap();
        composer.edit(at, "ab".into());
        composer.finish(&attempt, SaveOutcome::Saved(1));
        assert_eq!(composer.notice(at), "");
        assert_eq!(composer.begin(at).unwrap().expected_revision, 1);
    }

    #[test]
    fn a_newer_notice_outlives_the_save_it_replaced() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let attempt = composer.begin(at).unwrap();
        composer.say("Dictation is unavailable.");
        composer.finish(&attempt, SaveOutcome::Failed);
        assert_eq!(composer.mark, Mark::Said("Dictation is unavailable."));
    }

    #[test]
    fn an_answer_for_an_attempt_that_is_not_running_changes_nothing() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let first = composer.begin(at).unwrap();
        composer.finish(&first, SaveOutcome::Saved(1));
        let second = composer.begin(at).unwrap();
        // A repeated answer for the first attempt is not the running save's.
        composer.finish(&first, SaveOutcome::Failed);
        assert_eq!(composer.mark, Mark::Saving(second.id));
        composer.finish(&second, SaveOutcome::Saved(2));
        assert_eq!(composer.notice(at), "Saved");
    }

    /// Navigating mid-save: the other chat never shows the first chat's
    /// Saved, and the first chat keeps its text and the revision it saved.
    #[test]
    fn a_save_that_ends_after_navigation_marks_neither_surface() {
        let (one, two) = (chat("chat-1"), chat("chat-2"));
        let mut composer = opened(&one, 0, "");
        composer.edit(Some(&one), "for chat one".into());
        let attempt = composer.begin(Some(&one)).unwrap();
        // Before the navigation effect runs, the next render already reads
        // the other chat's (empty) composer and no mark.
        assert_eq!(
            (composer.notice(Some(&two)), composer.text(Some(&two))),
            ("", String::new())
        );
        composer.open(Some(&two));
        assert!(!composer.saving(Some(&two)));
        composer.finish(&attempt, SaveOutcome::Saved(1));
        assert_eq!(composer.notice(Some(&two)), "");
        composer.open(Some(&one));
        assert_eq!(
            (composer.notice(Some(&one)), composer.text(Some(&one))),
            ("", "for chat one".to_owned())
        );
        assert_eq!(composer.begin(Some(&one)).unwrap().expected_revision, 1);
    }

    #[test]
    fn leaving_the_patient_page_clears_the_mark() {
        let one = chat("chat-1");
        let at = Some(&one);
        let mut composer = opened(&one, 0, "");
        composer.edit(at, "a".into());
        let attempt = composer.begin(at).unwrap();
        composer.open(None);
        assert_eq!(composer.notice(None), "");
        assert!(composer.begin(None).is_none());
        composer.open(at);
        composer.finish(&attempt, SaveOutcome::Saved(1));
        assert_eq!(composer.notice(at), "");
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
        assert_eq!(composer.begin(Some(&other)).unwrap().expected_revision, 4);
    }
}

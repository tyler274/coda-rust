//! The chat window, assembled from the catalog's parts and live: the person's side is real input in
//! the real composer, and Coda's side is scripted. While Coda works one line says the step under
//! way, its steps folded under it until the person opens them; then the line says what was done and
//! the answer arrives whole, with its citations, Sources and Basis.
//! Only the stage, its size presets and the list of conversations at its left are the gallery's
//! own.

use std::collections::HashMap;
use std::time::Duration;

use dioxus::prelude::*;
use ui::{
    AnswerEnding, AnswerSources, Author, Basis, Button, ButtonVariant, ChatColumn, ChatHeader,
    CitationPreview, CitedPlace, Composer, EndAction, Ending, ExpandSource, FeedbackRating, Icon,
    IconName, KeyHint, ListRow, Located, Message, PreviewContent, Prose, ProsePart, QueuePlace,
    Rating, RowGroup, SectionHeader, Segmented, SourcePane, SourceReading, SourceState, StepState,
    WorkStep, WorkTrace,
};

use crate::{
    Cited, Confirming, LAB_TABLE, PATIENT, QueuedMessages, SlowMotion, Waiting, discharge_passage,
    discharge_record, panel_record, patient_facts, prose_names,
};

/// The stage's sizes, each a window the scene fills.
const STAGES: [(&str, u32, u32); 4] = [
    ("1440 × 900", 1440, 900),
    ("1280 × 800", 1280, 800),
    ("900 × 800", 900, 800),
    ("390 × 800", 390, 800),
];

/// The scene, on a stage of the chosen size, with the queue where the switch puts it. Replay
/// starts it over.
#[component]
pub(crate) fn ChatWindowDemo() -> Element {
    let mut stage = use_signal(|| 1usize);
    let mut place = use_signal(QueuePlace::default);
    let mut replays = use_signal(|| 0u32);
    let (_, width, height) = STAGES[stage()];
    rsx! {
        div { class: "scene",
            div { class: "scene-tools",
                Segmented {
                    label: "Window size",
                    options: STAGES.iter().map(|(words, _, _)| (*words).into()).collect::<Vec<_>>(),
                    selected: stage(),
                    on_select: move |index| stage.set(index),
                }
                Segmented {
                    label: "Queued messages",
                    options: vec![("Under the answer", "Queued messages under the answer").into(), ("Above the composer", "Queued messages above the composer").into()],
                    selected: match place() { QueuePlace::UnderAnswer => 0, QueuePlace::AboveComposer => 1 },
                    on_select: move |index| place.set(if index == 0 { QueuePlace::UnderAnswer } else { QueuePlace::AboveComposer }),
                }
                Button { compact: true, onclick: move |_| replays.set(replays() + 1),
                    Icon { name: IconName::Refresh }
                    "Replay"
                }
            }
            div { class: "scene-scroll",
                // A key only counts among a list's items, so the stage is a list of one.
                for replay in std::iter::once(replays()) {
                    div { key: "{replay}", class: "scene-stage", style: "width: {width}px; height: {height}px",
                        nav { class: "scene-nav", aria_label: "Conversations",
                            SectionHeader { title: "Mary Louis" }
                            RowGroup {
                                li { ListRow { title: "Potassium after discharge", detail: "Today", aria_current: "page", onclick: move |_| {} } }
                                li { ListRow { title: "Echo follow-up", detail: "April 30", onclick: move |_| {} } }
                                li { ListRow { title: "Medication review", detail: "April 12", onclick: move |_| {} } }
                            }
                        }
                        div { class: "scene-main",
                            ChatWindow { place: place() }
                        }
                    }
                }
            }
        }
    }
}

/// One step of a scripted run: its icon, its words while it runs and once done, what it found, and
/// how long it runs.
struct Step {
    icon: IconName,
    running: &'static str,
    done: &'static str,
    detail: &'static str,
    ms: u64,
}

/// What Coda's side plays for a turn.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Script {
    /// The earlier turn, already finished when the scene opens.
    Admission,
    /// The answer to the person's first question.
    Potassium,
    /// The answer to the follow-up: the panel's results as a table.
    Panel,
    /// The answer to the one after.
    Creatinine,
}

impl Script {
    fn steps(self) -> &'static [Step] {
        match self {
            Script::Admission => &[
                Step {
                    icon: IconName::File,
                    running: "Reading the discharge summary",
                    done: "Read the discharge summary",
                    detail: "April 25, 2024",
                    ms: 1200,
                },
                Step {
                    icon: IconName::File,
                    running: "Reading the outside cardiology consult",
                    done: "Read the outside cardiology consult",
                    detail: "Fax, page 2",
                    ms: 1400,
                },
            ],
            Script::Potassium => &[
                Step {
                    icon: IconName::Search,
                    running: "Searching her results for potassium",
                    done: "Searched her results for potassium",
                    detail: "6 found",
                    ms: 1400,
                },
                Step {
                    icon: IconName::File,
                    running: "Reading the basic metabolic panel",
                    done: "Read the basic metabolic panel",
                    detail: "April 25, 2024",
                    ms: 1800,
                },
                Step {
                    icon: IconName::File,
                    running: "Reading the discharge summary",
                    done: "Read the discharge summary",
                    detail: "April 25, 2024",
                    ms: 1300,
                },
                Step {
                    icon: IconName::Search,
                    running: "Checking for results after discharge",
                    done: "Checked for results after discharge",
                    detail: "None found",
                    ms: 900,
                },
            ],
            Script::Panel => &[Step {
                icon: IconName::File,
                running: "Reading the basic metabolic panel",
                done: "Read the basic metabolic panel",
                detail: "April 25, 2024",
                ms: 1500,
            }],
            Script::Creatinine => &[
                Step {
                    icon: IconName::File,
                    running: "Reading the basic metabolic panel",
                    done: "Read the basic metabolic panel",
                    detail: "Creatinine and eGFR",
                    ms: 1300,
                },
                Step {
                    icon: IconName::File,
                    running: "Reading the discharge summary",
                    done: "Read the discharge summary",
                    detail: "Renal course",
                    ms: 1600,
                },
            ],
        }
    }

    /// What the folded line says once the work is done, and how long it took.
    fn folded(self) -> (&'static str, &'static str) {
        match self {
            Script::Admission => ("Read 2 records", "9s"),
            Script::Potassium => ("Read 2 records and searched twice", "8s"),
            Script::Panel => ("Read 1 record", "3s"),
            Script::Creatinine => ("Read 2 records", "5s"),
        }
    }

    /// The sources the answer cites, by their numbers.
    fn sources(self) -> &'static [Cited] {
        match self {
            Script::Admission => &[Cited::Discharge, Cited::Fax],
            Script::Potassium => &[Cited::Discharge, Cited::Panel, Cited::Fax],
            Script::Panel => &[Cited::Panel],
            Script::Creatinine => &[Cited::Discharge, Cited::Panel],
        }
    }
}

/// One question and what Coda does with it: the messages the person sent for it, together or
/// taken in while it ran, and the script Coda's side plays.
#[derive(Clone, Debug, PartialEq)]
struct Turn {
    asked: Vec<String>,
    script: Script,
}

/// How a turn that is over ended.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Ended {
    /// The answer arrived.
    Finished,
    /// Stop ended it after this many steps had appeared; the person has since sent something else.
    Stopped { steps: usize },
}

/// A turn that is over.
#[derive(Clone, Debug, PartialEq)]
struct Settled {
    turn: Turn,
    ended: Ended,
}

/// The scene's one run, holding the turn it is about.
#[derive(Clone, Debug, PartialEq)]
enum Run {
    /// Nothing runs.
    Idle,
    /// Coda works on `turn`: `steps` of its steps have appeared, the last of them running.
    /// `attempt` tells this run from the ones Stop ended.
    Working {
        turn: Turn,
        steps: usize,
        attempt: u32,
    },
    /// Stop ended the run on `turn` after `steps` steps; Resume takes it up again until the person
    /// sends something else.
    Stopped { turn: Turn, steps: usize },
}

/// The scene's state, every part of it with one owner.
#[derive(Clone, Copy)]
struct Scene {
    history: Signal<Vec<Settled>>,
    run: Signal<Run>,
    /// The queued messages: Send next adds to it, a finish sends it, Stop returns it.
    queue: Signal<Vec<Waiting>>,
    editing: Signal<Option<(u32, String)>>,
    draft: Signal<String>,
    keys: Signal<u32>,
    attempts: Signal<u32>,
    slow: Option<Signal<bool>>,
}

impl Scene {
    /// What the person sends: queued while Coda works if `next`, otherwise taken in by the run
    /// under way, or the start of a new turn.
    fn send(mut self, text: String, next: bool) {
        let text = text.trim().to_owned();
        if text.is_empty() {
            return;
        }
        self.draft.set(String::new());
        let running = matches!(*self.run.peek(), Run::Working { .. });
        match (running, next) {
            (true, true) => {
                let key = *self.keys.peek() + 1;
                self.keys.set(key);
                self.queue.write().push(Waiting {
                    key,
                    text,
                    confirming: Confirming::Settled,
                });
            }
            (true, false) => self.steer(text),
            (false, _) => self.ask(vec![text]),
        }
    }

    /// A message the run under way takes in.
    fn steer(mut self, text: String) {
        if let Run::Working { turn, .. } = &mut *self.run.write() {
            turn.asked.push(text);
        }
    }

    /// Starts a new turn. A stopped turn waiting on Resume settles as stopped first.
    fn ask(mut self, asked: Vec<String>) {
        let stopped = self.run.peek().clone();
        if let Run::Stopped { turn, steps } = stopped {
            self.history.write().push(Settled {
                turn,
                ended: Ended::Stopped { steps },
            });
        }
        let script = self.next_script();
        self.start(Turn { asked, script }, 1);
    }

    /// The person's turns take the three scripted answers in turn.
    fn next_script(self) -> Script {
        let asked_before = self.history.peek().len().saturating_sub(1);
        match asked_before % 3 {
            0 => Script::Potassium,
            1 => Script::Panel,
            _ => Script::Creatinine,
        }
    }

    fn start(mut self, turn: Turn, steps: usize) {
        let attempt = *self.attempts.peek() + 1;
        self.attempts.set(attempt);
        self.run.set(Run::Working {
            turn,
            steps,
            attempt,
        });
        spawn(self.play(attempt));
    }

    /// Stop ends the run where it is. Nothing queued is sent: its words return to the composer.
    fn stop(mut self) {
        let running = self.run.peek().clone();
        let Run::Working { turn, steps, .. } = running else {
            return;
        };
        self.run.set(Run::Stopped { turn, steps });
        let queued: Vec<String> = self.queue.write().drain(..).map(|held| held.text).collect();
        self.editing.set(None);
        if !queued.is_empty() {
            let mut words = self.draft.peek().clone();
            for text in queued {
                if !words.is_empty() {
                    words.push('\n');
                }
                words.push_str(&text);
            }
            self.draft.set(words);
        }
    }

    /// Resume takes the stopped run up again at the step it stopped on.
    fn resume(self) {
        let stopped = self.run.peek().clone();
        if let Run::Stopped { turn, steps } = stopped {
            self.start(turn, steps);
        }
    }

    fn current(self, attempt: u32) -> bool {
        matches!(*self.run.peek(), Run::Working { attempt: running, .. } if running == attempt)
    }

    /// Plays run `attempt`: each step appears after the one before has run its time; after the last,
    /// the answer arrives whole. What was queued meanwhile is then sent together as the next turn.
    async fn play(mut self, attempt: u32) {
        loop {
            loop {
                let (script, steps) = match &*self.run.peek() {
                    Run::Working {
                        turn,
                        steps,
                        attempt: running,
                    } if *running == attempt => (turn.script, *steps),
                    _ => return,
                };
                self.pause(script.steps()[steps - 1].ms).await;
                if !self.current(attempt) {
                    return;
                }
                if steps == script.steps().len() {
                    break;
                }
                if let Run::Working { steps, .. } = &mut *self.run.write() {
                    *steps += 1;
                }
            }
            let finished = self.run.peek().clone();
            let Run::Working { turn, .. } = finished else {
                return;
            };
            self.history.write().push(Settled {
                turn,
                ended: Ended::Finished,
            });
            let queued: Vec<String> = self.queue.write().drain(..).map(|held| held.text).collect();
            self.editing.set(None);
            if queued.is_empty() {
                self.run.set(Run::Idle);
                return;
            }
            let script = self.next_script();
            self.run.set(Run::Working {
                turn: Turn {
                    asked: queued,
                    script,
                },
                steps: 1,
                attempt,
            });
        }
    }

    /// Waits `ms`, four times as long in the gallery's slow motion.
    async fn pause(self, ms: u64) {
        let slow = self.slow.is_some_and(|slow| *slow.peek());
        futures_timer::Delay::new(Duration::from_millis(if slow { ms * 4 } else { ms })).await;
    }
}

/// A citation in the scene's answers: which source it cites, at which place.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cite {
    Admission,
    Potassium,
    Creatinine,
    ConsultPage,
}

impl Cite {
    fn source(self) -> Cited {
        match self {
            Cite::Admission => Cited::Discharge,
            Cite::Potassium | Cite::Creatinine => Cited::Panel,
            Cite::ConsultPage => Cited::Fax,
        }
    }

    fn place(self) -> CitedPlace {
        match self {
            Cite::Admission => CitedPlace::Passage(
                "Admitted with decompensated heart failure; diuresed 6 kg.".into(),
            ),
            Cite::Potassium => CitedPlace::Field {
                name: "Potassium".into(),
                value: "4.0 mmol/L".into(),
            },
            Cite::Creatinine => CitedPlace::Field {
                name: "Creatinine".into(),
                value: "124 µmol/L".into(),
            },
            Cite::ConsultPage => CitedPlace::Page("Page 2".into()),
        }
    }

    /// The source as the pane reads it, at this citation's place; a fax has no reading of its own.
    fn reading(self) -> Option<SourceState> {
        match self {
            Cite::Admission => Some(SourceState::Shown(Box::new(discharge_record(
                discharge_passage(),
            )))),
            Cite::Potassium => Some(SourceState::Shown(Box::new(panel_record(Located::Found(
                "component[0]".into(),
            ))))),
            Cite::Creatinine => Some(SourceState::Shown(Box::new(panel_record(Located::Found(
                "component[3]".into(),
            ))))),
            Cite::ConsultPage => None,
        }
    }
}

/// A citation mark that previews its source and, where the pane can read it, opens it there.
#[component]
fn Mark(cite: Cite, on_open: EventHandler<Cite>) -> Element {
    let cited = cite.source();
    let index = cited.index();
    let title = cited.title();
    rsx! {
        CitationPreview {
            index,
            label: title,
            eyebrow: "Source {index}",
            citation_label: "Source {index}: {title}. Preview source.",
            close_label: "Close source preview",
            content: PreviewContent::Cited { about: cited.about(), place: cite.place() },
            expand: cite.reading().map(|_| ExpandSource { label: "Expand".into(), on_expand: EventHandler::new(move |_| on_open.call(cite)) }),
            original: cited.original(),
        }
    }
}

/// A script's answer, as it arrives.
#[component]
fn AnswerWords(script: Script, on_open: EventHandler<Cite>) -> Element {
    match script {
        Script::Admission => rsx! {
            p {
                "She was admitted from April 18 to 25 with decompensated heart failure. She came in after three days of worsening breathlessness, orthopnea and leg swelling, and her chest radiograph showed congestion with small effusions."
            }
            p {
                "She was diuresed 6 kg,"
                Mark { cite: Cite::Admission, on_open }
                " and her renal function held through it. Her echocardiogram showed an ejection fraction of 35 percent, lower than the 45 percent recorded in 2022."
            }
            p {
                "She went home on furosemide 40 mg daily, with follow-up in cardiology clinic within two weeks and a basic metabolic panel the morning she left."
            }
            p {
                "Riverside Cardiology has since faxed a consult asking for a repeat echocardiogram in three months, after her diuretic dose has settled."
                Mark { cite: Cite::ConsultPage, on_open }
            }
            p {
                "Her discharge weight was 71.4 kg, down from 77.6 kg on admission. The summary asks her to weigh herself each morning and to call if she gains more than 2 kg in three days."
            }
            p {
                "Her other medications went home unchanged: metoprolol succinate 50 mg daily, lisinopril 10 mg daily and atorvastatin 40 mg at night. Spironolactone was not started, with her potassium to be rechecked first."
            }
            p {
                "Nothing in the record yet shows the cardiology visit taking place."
            }
        },
        Script::Potassium => rsx! {
            p {
                "Her potassium was 4.0 mmol/L on the morning of discharge,"
                Mark { cite: Cite::Potassium, on_open }
                " after diuresis for decompensated heart failure."
                Mark { cite: Cite::Admission, on_open }
                " Nothing has been drawn since, so there is no level on furosemide at home yet. Riverside Cardiology asked for a repeat echocardiogram, not labs."
                Mark { cite: Cite::ConsultPage, on_open }
            }
        },
        Script::Panel => rsx! {
            Prose {
                names: prose_names(),
                parts: vec![
                    ProsePart::Words(format!(
                        "The basic metabolic panel drawn the morning she left, still preliminary:\n\n{LAB_TABLE}\n\nOnly creatinine stands outside its reference range of 60 to 110 µmol/L."
                    )),
                    ProsePart::Inline(rsx! { Mark { cite: Cite::Creatinine, on_open } }),
                ],
            }
        },
        Script::Creatinine => rsx! {
            p {
                "Her creatinine was 124 µmol/L on the same panel, above its reference range,"
                Mark { cite: Cite::Creatinine, on_open }
                " and the discharge summary says her renal function stayed stable through diuresis."
                Mark { cite: Cite::Admission, on_open }
            }
        },
    }
}

/// What a script's answer rests on and what could change it.
#[component]
fn BasisWords(script: Script, on_open: EventHandler<Cite>) -> Element {
    match script {
        Script::Admission => rsx! {
            p {
                "Rests on the discharge summary"
                Mark { cite: Cite::Admission, on_open }
                " and the faxed consult."
                Mark { cite: Cite::ConsultPage, on_open }
                " The consult's first page did not come through, so anything it said there is missing here."
            }
        },
        Script::Potassium => rsx! {
            p {
                "Rests on the panel drawn that morning,"
                Mark { cite: Cite::Potassium, on_open }
                " which is still preliminary, and the discharge summary."
                Mark { cite: Cite::Admission, on_open }
                " Results drawn outside this practice would not show here."
            }
        },
        Script::Panel => rsx! {
            p {
                "Rests on the one panel drawn that morning,"
                Mark { cite: Cite::Creatinine, on_open }
                " which is still preliminary. Results drawn outside this practice would not show here."
            }
        },
        Script::Creatinine => rsx! {
            p {
                "Rests on one preliminary panel"
                Mark { cite: Cite::Creatinine, on_open }
                " and the discharge summary."
                Mark { cite: Cite::Admission, on_open }
                " With one value, there is no trend to read."
            }
        },
    }
}

/// Where a turn's reply stands, as it is drawn.
#[derive(Clone, Copy, PartialEq)]
enum Reply {
    Working { steps: usize },
    Finished,
    Stopped { steps: usize, resumable: bool },
}

/// The chat window itself: who it is about, the transcript, the composer docked at its foot, and
/// the source pane beside it. What Coda is doing shows only in the transcript; the composer holds
/// only its controls.
#[component]
fn ChatWindow(place: QueuePlace) -> Element {
    let history = use_signal(|| {
        vec![Settled {
            turn: Turn {
                asked: vec!["Why was she admitted in April?".into()],
                script: Script::Admission,
            },
            ended: Ended::Finished,
        }]
    });
    let scene = Scene {
        history,
        run: use_signal(|| Run::Idle),
        queue: use_signal(Vec::new),
        editing: use_signal(|| None),
        draft: use_signal(String::new),
        keys: use_signal(|| 0),
        attempts: use_signal(|| 0),
        slow: try_use_context::<SlowMotion>().map(|SlowMotion(slow)| slow),
    };
    let mut typing = use_signal(|| false);
    let mut ratings = use_signal(HashMap::<usize, Rating>::new);
    let mut opened = use_signal(|| None::<(Cite, bool)>);
    let mut openings = use_signal(|| 0u32);
    let open_source = move |cite: Cite| {
        let opening = openings() + 1;
        openings.set(opening);
        opened.set(Some((cite, false)));
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(350)).await;
            if openings() == opening {
                opened.set(Some((cite, true)));
            }
        });
    };
    let source = opened().and_then(|(cite, read)| {
        let reading = if read {
            cite.reading()?
        } else {
            SourceState::Loading("Loading source".into())
        };
        Some(SourcePane {
            eyebrow: format!("Source {}", cite.source().index()),
            title: cite.source().title().into(),
            state: reading,
            back: "Back".into(),
            close: "Close source".into(),
        })
    });

    let run = scene.run.read().clone();
    let settled = history.read().clone();
    let mut turns: Vec<(Turn, Reply)> = settled
        .iter()
        .map(|done| {
            let reply = match done.ended {
                Ended::Finished => Reply::Finished,
                Ended::Stopped { steps } => Reply::Stopped {
                    steps,
                    resumable: false,
                },
            };
            (done.turn.clone(), reply)
        })
        .collect();
    match &run {
        Run::Idle => {}
        Run::Working { turn, steps, .. } => {
            turns.push((turn.clone(), Reply::Working { steps: *steps }))
        }
        Run::Stopped { turn, steps } => turns.push((
            turn.clone(),
            Reply::Stopped {
                steps: *steps,
                resumable: true,
            },
        )),
    }
    let replies = settled.len();
    let sent = turns
        .iter()
        .map(|(turn, _)| turn.asked.len())
        .sum::<usize>();
    let working = matches!(run, Run::Working { .. });
    let queue = scene.queue;
    let has_queue = !queue.read().is_empty();

    rsx! {
        SourceReading { source, on_close: move |_| opened.set(None),
            ChatColumn {
                // A toolbar's height of room over the header, standing for the shell's toolbar row.
                header: rsx! {
                    div {
                        div { class: "scene-toolbar" }
                        ChatHeader { name: PATIENT, facts: patient_facts() }
                    }
                },
                replies,
                sent,
                jump_label: "Jump to new response",
                queue: has_queue.then(|| rsx! {
                    QueuedMessages { queue, editing: scene.editing, on_send_now: move |text| scene.steer(text) }
                }),
                queue_place: place,
                composer: rsx! {
                    div {
                        class: "scene-composer",
                        onkeydown: move |event: KeyboardEvent| {
                            if !typing() || event.key() != Key::Enter || event.is_composing() || event.modifiers().shift() {
                                return;
                            }
                            event.prevent_default();
                            let next = event.modifiers().meta() || event.modifiers().ctrl();
                            let words = scene.draft.peek().clone();
                            scene.send(words, next);
                        },
                        Composer {
                            label: "Message Coda",
                            placeholder: "Ask about Mary",
                            value: scene.draft.read().clone(),
                            on_input: move |text| { let mut draft = scene.draft; draft.set(text); },
                            on_submit: move |text| scene.send(text, false),
                            on_editing: move |editing| typing.set(editing),
                            if working {
                                Button { variant: ButtonVariant::Quiet, onclick: move |_| scene.stop(), "Stop" }
                                Button {
                                    onclick: move |_| {
                                        let words = scene.draft.peek().clone();
                                        scene.send(words, true);
                                    },
                                    "Send next",
                                    KeyHint { key_label: "⌘↵" }
                                }
                                Button { variant: ButtonVariant::Primary, submit: true, "Send now", KeyHint { key_label: "↵" } }
                            } else {
                                Button { variant: ButtonVariant::Primary, submit: true, "Send", KeyHint { key_label: "↵" } }
                            }
                        }
                    }
                },
                for (index, (turn, reply)) in turns.into_iter().enumerate() {
                    div { key: "{index}", class: "scene-turn",
                        for (said, words) in turn.asked.iter().enumerate() {
                            Message { key: "{said}", author: Author::Person, by: "You", p { "{words}" } }
                        }
                        Message { author: Author::Assistant, by: "Coda",
                            Trace { script: turn.script, reply }
                            match reply {
                                Reply::Working { .. } => rsx! {},
                                Reply::Finished => rsx! {
                                    AnswerEnding {
                                        ending: Ending::Finished {
                                            words: rsx! { AnswerWords { script: turn.script, on_open: open_source } },
                                            sources: Some(AnswerSources {
                                                label: "Sources".into(),
                                                sources: turn.script.sources().iter().map(|cited| cited.indexed()).collect(),
                                            }),
                                            basis: Basis {
                                                title: "Basis and limitations".into(),
                                                words: rsx! { BasisWords { script: turn.script, on_open: open_source } },
                                            },
                                            rating: rsx! {
                                                FeedbackRating {
                                                    selected: ratings.read().get(&index).copied(),
                                                    label: "Rate this response",
                                                    up_label: "Good response",
                                                    down_label: "Poor response",
                                                    on_rate: move |choice| { ratings.write().insert(index, choice); },
                                                }
                                            },
                                        },
                                    }
                                },
                                Reply::Stopped { resumable, .. } => rsx! {
                                    AnswerEnding {
                                        ending: Ending::Stopped {
                                            mark: "Stopped".into(),
                                            resume: resumable.then(|| EndAction { label: "Resume".into(), on_press: EventHandler::new(move |_| scene.resume()) }),
                                        },
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A turn's steps under one line, folded until the person opens them, the step
/// Stop ended marked as stopped.
#[component]
fn Trace(script: Script, reply: Reply) -> Element {
    let all = script.steps();
    let (label, detail, shown) = match reply {
        Reply::Working { steps } => (all[steps - 1].running, None, steps),
        Reply::Finished => {
            let (label, detail) = script.folded();
            (label, Some(detail), all.len())
        }
        Reply::Stopped { steps, .. } => (all[steps - 1].running, None, steps),
    };
    rsx! {
        WorkTrace { label, working: matches!(reply, Reply::Working { .. }), detail: detail.map(str::to_owned),
            for (index, step) in all.iter().enumerate().take(shown) {
                WorkStep {
                    key: "{index}",
                    icon: step.icon,
                    label: if index + 1 < shown || reply == Reply::Finished { step.done } else { step.running },
                    state: match reply {
                        _ if index + 1 < shown => StepState::Done,
                        Reply::Working { .. } => StepState::Running,
                        Reply::Finished => StepState::Done,
                        Reply::Stopped { .. } => StepState::Stopped,
                    },
                    detail: match reply {
                        Reply::Stopped { .. } if index + 1 == shown => Some("Stopped".to_owned()),
                        _ if index + 1 < shown || reply == Reply::Finished => Some(step.detail.to_owned()),
                        _ => None,
                    },
                }
            }
        }
    }
}

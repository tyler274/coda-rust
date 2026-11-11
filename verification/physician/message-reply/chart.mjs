// The words of the reply chart, for PStack's statechart skill. Run it with Charts.tla and chart.cfg:
// one physician who may access both patients, and one patient message. The patient writes, Coda
// reviews, and the reply ends answered by the practice, closed by you as needing no reply, or with no
// reply needed. The practice sends its own words, which answer the patient. It leaves out a practice
// message that doesn't answer, drafts (chart-draft draws them), moves (chart-move), the channel
// (chart-channel), copying, Coda restarting, concerns, and the patient writing again.

export const title = "Coda's reply to a patient message";

const msg = { "Not written": "No message yet", Open: "Message open", Answered: "Answered by the practice",
  "Needs no reply": "Needs no reply" };
const reply = { "No reply": "", Ready: "; reply ready", "Closed, Undo shows": "; reply closed, Undo shows" };

const reviewing = (v) => v.woke.length > 0 || v.review.for !== "Nobody";

const practice = (v) => (v.sent.text[1] === "Nothing" ? "" : "; the practice answered");

export const state = (v) =>
  `${msg[v.msg[1]]}${reviewing(v) ? "; Coda is reviewing" : ""}${reply[v.reply.state]}${practice(v)}`;

const steps = {
  PatientWrites: "the patient writes",
  ReviewStarts: "Coda starts reviewing",
  ReviewReadsAgain: "Coda reads again",
  YouCloseNoReply: "you close it: no reply needed",
  YouUndoClose: "you press Undo",
  UndoGoesAway: "Undo goes away",
};

export const step = (action, from, to) => {
  if (action === "ReviewFinishes")
    return from.msg[1] !== "Open" ? "Coda finishes: nothing is open"
      : to.reply.state === "Ready" ? "Coda writes a reply"
      : to.msg[1] === "Answered" ? "Coda closes it as answered"
      : to.msg[1] === "Needs no reply" ? "Coda finds no reply would help"
      : "Coda finishes";
  if (action === "Sends") return "the practice answers";
  return steps[action] ?? null;
};

export const group = (v) =>
  v.msg[1] === "Open" ? (v.reply.state === "Ready" ? "Reply ready" : "Reviewing") : v.written === 0 ? "Start" : "Handled";

export const colors = { Start: "tan", Reviewing: "orange", "Reply ready": "blue", Handled: "teal" };

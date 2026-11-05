// The words of the chart, for PStack's statechart skill. Run it with chart.cfg: one patient, one
// physician, one message from the patient and one from the practice, in the physician's words.
// Every label is read from the states before and after a step, since TLC keeps one state for each
// group it counts as the same.

export const title = "A patient's message and its reply";

const items = (v) => Object.values(v.inbox ?? {});
const you = "Dr Ames";
const patientAt = (v) => items(v).findIndex((x) => x.from === "Patient") + 1;
const practiceAt = (v) => items(v).findIndex((x) => x.from === "Practice") + 1;
const answered = (v) => (v.history ?? []).some((h) => h.answered);

export const state = (v) => {
  const message = patientAt(v);
  const sent = practiceAt(v);
  if (!message) return "No message yet";
  if (v.filed < message) return "In Spruce, not yet filed";
  const match = v.match[message];
  if (match === "Waiting") return "Waiting in Messages to match";
  if (match === "Unlinked") return "Left unlinked";
  if ((v.handled ?? []).includes(message)) return answered(v) ? "Settled: the practice answered" : "Settled: no reply needed";
  const seen = sent > message && v.filed >= sent;
  if (v.reply.open) {
    const text = v.draft[you].on ? "your draft" : "Coda's draft";
    return `Reply on Today, ${text}${seen ? "; the practice wrote, Coda checking" : ""}`;
  }
  return seen ? "Coda checking what the practice wrote" : "Coda reviewing";
};

// Copying, revising and the practice writing before Coda sees it change nothing the physician sees
// on Today, so the chart draws none of them.
export const step = (action, from, to) => {
  if (action === "Rest" || action === "OpenReply") return null;
  if (action === "Files") {
    const item = items(to)[to.filed - 1];
    if (item.from === "Practice") return "Coda sees what the practice sent";
    return to.match[to.filed] === "Waiting" ? "Coda files it [no sure match]" : "Coda files it to the patient";
  }
  if (action === "PracticeSends") {
    const w = items(to).at(-1).words;
    if (w === "An automatic reply") return "Spruce sends an automatic reply";
    if (w === "Words written in the channel") return "you write in Spruce";
    return "you paste the reply in Spruce and send";
  }
  if (action === "ReviewFinishes") {
    if (answered(to) && !answered(from)) return "Coda finds the practice answered";
    if (to.reply.open) return from.reply.open ? "Coda updates the reply" : "Coda drafts a reply";
    return "Coda decides no reply is needed";
  }
  const words = {
    PatientWrites: "the patient writes",
    Confirm: "you confirm the patient",
    LeaveUnlinked: "you leave it unlinked",
    ReviewReads: "Coda reads the conversation and the chart",
    Edit: "you edit",
    TakeUpdate: "you take Coda's update",
    Close: "you mark No reply needed",
  };
  return words[action] ?? null;
};

export const group = (v) => {
  const s = state(v);
  if (s.startsWith("Reply")) return "On Today";
  if (s.startsWith("Coda")) return "Coda working";
  if (s.startsWith("Settled") || s === "Left unlinked") return "Settled";
  return "Arriving";
};

export const colors = { Arriving: "blue", "Coda working": "purple", "On Today": "teal", Settled: "tan" };

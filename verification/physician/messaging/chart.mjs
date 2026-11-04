// The words of the patient-message chart, for PStack's statechart skill. Run it with chart.cfg,
// which draws one patient message and one reply, for one physician and one patient.

export const title = "One patient message, from the channel to the practice's reply";

const items = (v) => Object.values(v.inbox ?? {});
const wordsOf = {
  "Coda's draft": "Coda's draft",
  "Your edit": "your edit",
  "Words written in the channel": "words written in the channel",
};

// Where the patient's message stands, and what the physician sees.
export const state = (v) => {
  const it = items(v);
  const message = it.findIndex((x) => x.from === "Patient");
  const reply = it.findIndex((x) => x.from === "Practice");
  if (message < 0) return "No patient message yet";
  if (v.filed <= message) return "In the channel, not yet filed";
  const sent = reply > message;
  const match = v.match[message + 1];
  if (match === "Waiting") return "Waiting in Messages to match";
  if (match === "Unlinked") return "Left unlinked";
  // The patient's history is the chart's one place for the words sent; the rule WhatWasSent
  // checks that it matches the channel.
  const recorded = (v.history ?? [])[0];
  if (recorded) return `Answered: history shows ${wordsOf[recorded.words]}`;
  const handled = (v.handled ?? []).includes(message + 1);
  if (handled) return sent ? "Settled; your reply not yet seen" : "Settled with no reply";
  if (sent && v.card.open) return "Sent in the channel; the row stays until Coda sees it";
  if (sent) return "Sent in the channel; Coda is still reviewing";
  if (v.card.open) {
    if (v.card.state === "Coda is revising") return "Coda is revising the reply";
    if (v.card.text === "Yours") return "Reply on Today: your edit";
    const copied = Object.values(v.copied).some((w) => w !== "Nothing");
    return copied ? "Reply on Today: Coda's draft, copied" : "Reply on Today: Coda's draft";
  }
  return "Coda is reviewing";
};

const steps = {
  PatientWrites: "the patient writes in the channel",
  PracticeSends: "you send a reply in the channel",
  Confirm: "a physician confirms the patient",
  LeaveUnlinked: "a physician leaves it unlinked",
  ReviewReads: "Coda reads the conversation and the chart",
  RevisionFinishes: "Coda finishes the revision",
  OpenCard: "you open the reply",
  Edit: "you edit the reply",
  AskRevision: "you ask Coda to revise",
  TakeUpdate: "you take Coda's update",
  CopyReply: "you copy the reply",
  Close: "you mark No reply needed",
};

// The chart draws revising, editing and No reply needed from the plain draft, and keeps one way out
// of each other card state. The model, and the rules, have them from every card state.
const hidden = {
  AskRevision: ["Reply on Today: Coda's draft, copied", "Reply on Today: your edit"],
  Edit: ["Coda is revising the reply", "Reply on Today: Coda's draft, copied"],
  Close: ["Reply on Today: Coda's draft, copied", "Reply on Today: your edit"],
  PracticeSends: ["Coda is revising the reply"],
};

export const step = (action, from, to) => {
  if (action === "Rest") return null;
  if (hidden[action]?.includes(state(from))) return null;
  if (action === "Files") {
    const after = state(to);
    if (after === "Waiting in Messages to match") return "Coda files it [no sure match]";
    if (after.startsWith("Answered")) return `Coda sees the reply you sent [${after.slice(after.indexOf("shows ") + 6)}]`;
    if (from.link !== "Nobody") return "Coda files it [conversation already linked]";
    return "Coda files it [the channel names the patient]";
  }
  if (action === "Confirm" && state(to).startsWith("Answered")) {
    return "a physician confirms the patient [the practice already replied]";
  }
  if (action === "ReviewFinishes") {
    return to.card.open ? "Coda drafts a reply" : "Coda decides no reply is needed";
  }
  return steps[action] ?? action;
};

export const group = (v) => {
  const s = state(v);
  if (s.startsWith("Answered") || s.startsWith("Settled") || s === "Left unlinked") return "Settled";
  if (s.startsWith("Reply on Today") || s.endsWith("the row stays until Coda sees it")) return "On Today";
  if (s.startsWith("Coda") || s.endsWith("Coda is still reviewing")) return "Coda working";
  return "Arriving";
};

export const colors = { Arriving: "blue", "Coda working": "purple", "On Today": "teal", Settled: "tan" };

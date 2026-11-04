// The words of the write-again chart, for PStack's statechart skill. Run it with chart-again.cfg:
// one patient writes twice while the reply card is open, for one physician.

export const title = "The patient writes again while the reply is open";

const patientItems = (v) => Object.values(v.inbox ?? {});

export const state = (v) => {
  const items = patientItems(v);
  const c = v.card;
  if (!c.open) {
    if (items.length === 0) return "No message yet";
    const waiting = Object.values(v.match).includes("Waiting");
    if (waiting) return "Waiting in Messages to match";
    if (Object.values(v.match).includes("Unlinked")) return "A message left unlinked";
    if ((v.handled ?? []).length === items.length && v.filed === items.length) return "Settled";
    return v.filed < items.length && v.filed === 0 ? "In the channel, not yet filed" : "Coda reviewing";
  }
  // A second message Coda hasn't filed isn't on screen yet, so it doesn't change the label.
  if (c.state === "Coda is revising") return `Coda revising the reply${c.told ? ", says the patient wrote again" : ""}`;
  const text = c.text === "Yours" ? "your edit" : "Coda's draft";
  if (c.offer) return `Reply: ${text}, Coda's update offered`;
  if (c.told) return `Reply: ${text}, says the patient wrote again`;
  const both = v.filed === 2 && c.basis === 2 ? " covering both messages" : "";
  return `Reply: ${text}${both}`;
};

// No reply needed and revising are drawn from the first draft, and the edit that stops a revision
// from the revision the patient's news reached. The model has each from every card state.
const drawnFrom = {
  Close: ["Reply: Coda's draft"],
  AskRevision: ["Reply: Coda's draft", "Reply: Coda's draft, says the patient wrote again"],
  Edit: ["Reply: Coda's draft", "Reply: Coda's draft, says the patient wrote again",
         "Coda revising the reply, says the patient wrote again"],
};

const steps = {
  PatientWrites: "the patient writes",
  Confirm: "a physician confirms the patient",
  LeaveUnlinked: "a physician leaves it unlinked",
  ReviewReads: "Coda reads the conversation and the chart",
  OpenCard: "you open the reply",
  Edit: "you edit the reply",
  AskRevision: "you ask Coda to revise",
  RevisionFinishes: "Coda finishes the revision on both messages",
  TakeUpdate: "you take Coda's update",
  Close: "you mark No reply needed",
};

const hidden = ["CopyReply", "Rest"];

export const step = (action, from, to) => {
  if (hidden.includes(action)) return null;
  if (drawnFrom[action] && !drawnFrom[action].includes(state(from))) return null;
  if (action === "Files") {
    if (from.card.open) return "Coda files the second message";
    return state(to) === "Waiting in Messages to match" ? "Coda files it [no sure match]" : "Coda files it [conversation linked]";
  }
  if (action === "ReviewFinishes") {
    if (!to.card.open) return "Coda decides no reply is needed";
    if (!from.card.open) return "Coda drafts a reply";
    if (from.card.state === "Coda is revising") return "the review leaves the news to the revision";
    return from.card.edited ? "Coda keeps your edit and offers its update" : "Coda updates its draft in place";
  }
  if (action === "Edit" && from.card.state === "Coda is revising") return "you edit during the revision, which stops";
  return steps[action] ?? action;
};

export const group = (v) => {
  const s = state(v);
  if (s.includes("wrote again") || s.includes("offered")) return "Patient wrote again";
  if (s.startsWith("Reply") || s.startsWith("Coda revising")) return "On Today";
  if (s === "Settled" || s.includes("unlinked")) return "Settled";
  return "Arriving";
};

export const colors = { Arriving: "blue", "On Today": "teal", "Patient wrote again": "yellow", Settled: "tan" };

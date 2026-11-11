// The words of the draft chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-draft.cfg: one physician, and Coda's reply to one message is ready. You edit it, ask Coda to
// revise your text, and restore it, while the patient writes again and Coda writes a newer reply.
// Each state names what you see on the card. It leaves out copying (a step that changes nothing),
// closing, moves, the practice and the channel.

export const title = "Your draft of Coda's reply";

const draft = (v) => {
  const d = v.draft.you;
  if (d.kind === "No draft" || d.on !== v.reply.id) return "You see Coda's reply";
  return d.kind === "Your text" ? "You see your text" : "You see Coda's revision of your text";
};

const reviewing = (v) => v.woke.length > 0 || v.review.for !== "Nobody";

export const state = (v) => {
  const offer = v.offers.includes("you") ? "; Coda offers its update" : "";
  const news = v.written < 2 ? "" : reviewing(v) ? "; the patient wrote again, Coda is reviewing" : "; Coda's reply is newer";
  return `${draft(v)}${offer}${news}`;
};

const steps = {
  YouEdit: "you edit",
  YouAskRevise: "you ask Coda to revise",
  YouRestore: "you restore your version",
  YouChooseUpdate: "you choose Coda's update",
  PatientWrites: "the patient writes again",
  ReviewStarts: "Coda starts reviewing",
  ReviewFinishes: "Coda writes a newer reply",
};

export const step = (action) => steps[action] ?? null;

export const group = (v) => (draft(v) === "You see Coda's reply" ? "Coda's reply" : "Your draft");

export const colors = { "Coda's reply": "teal", "Your draft": "blue" };

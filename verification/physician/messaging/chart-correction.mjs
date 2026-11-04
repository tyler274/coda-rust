// The words of the correction chart, for PStack's statechart skill. Run it with chart-correction.cfg:
// one message, two patients, Dr Ames who may see both and Dr Bell who may see only one of them, and
// one correction by the messaging manager, Dr Ames. The chart is about who sees the reply, so a
// label names Dr Bell when he has it open and leaves Dr Ames's open reply out.

export const title = "Wrong patient: a correction moves the conversation";

export const state = (v) => {
  if (Object.values(v.inbox ?? {}).length === 0) return "No message yet";
  if (v.filed === 0) return "In the channel, not yet filed";
  const match = v.match[1];
  if (match === "Waiting") return "Waiting in Messages to match";
  if (match === "Unlinked") return "Left unlinked";
  const where = v.corrections > 0 ? "Moved" : "Linked";
  if ((v.handled ?? []).includes(1)) return `${where}: settled`;
  const c = v.card;
  if (!c.open) return `${where}: Coda reviewing`;
  const bell = v.viewing["Dr Bell"] ? ", open for Dr Bell" : "";
  if (c.state === "Withheld after a correction") return `Moved: reply withheld${c.edited ? ", your edit kept" : ""}`;
  const text = c.text === "Yours" ? (c.offer ? "your edit, Coda's update offered" : "your edit") : "Coda's draft";
  return `${where}: reply on Today, ${text}${bell}`;
};

const steps = {
  PatientWrites: "the patient writes",
  Confirm: "a physician confirms a patient",
  LeaveUnlinked: "a physician leaves it unlinked",
  Correct: "Dr Ames: Wrong patient? moves the conversation",
  ReviewReads: "Coda reads the conversation and the chart",
  ReviewLetsGo: "the old patient's review lets go",
  Edit: "a physician edits the reply",
  Close: "a physician marks No reply needed",
  TakeUpdate: "a physician takes Coda's update",
};

// Revising and copying are drawn in the other charts.
const hidden = ["AskRevision", "RevisionFinishes", "CopyReply", "Rest"];

export const step = (action, from, to) => {
  if (hidden.includes(action)) return null;
  if (action === "Files") return "Coda files it [no sure match]";
  if (action === "OpenCard") return to.viewing["Dr Bell"] && !from.viewing["Dr Bell"] ? "Dr Bell opens the reply" : null;
  if (action === "Correct") {
    return from.viewing["Dr Bell"] && !to.viewing["Dr Bell"]
      ? `${steps.Correct} [to a patient Dr Bell may not see]`
      : steps.Correct;
  }
  if (action === "ReviewFinishes") {
    if (!to.card.open) return from.card.open ? "the re-review closes the withheld reply" : "Coda decides no reply is needed";
    if (from.card.state === "Withheld after a correction") {
      return from.card.edited ? "the re-review keeps your edit and offers its update" : "the re-review redrafts the reply";
    }
    return "Coda drafts a reply";
  }
  return steps[action] ?? action;
};

export const group = (v) => {
  const s = state(v);
  if (s.startsWith("Moved")) return "After the correction";
  if (s.startsWith("Linked")) return "Before the correction";
  return "Arriving";
};

export const colors = { Arriving: "blue", "Before the correction": "teal", "After the correction": "orange" };

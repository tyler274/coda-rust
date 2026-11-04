// The words of the disconnect chart, for PStack's statechart skill. Run it with
// chart-disconnect.cfg: one message and one reply, and the practice disconnecting once.

export const title = "The practice disconnects the channel";

const items = (v) => Object.values(v.inbox ?? {});

export const state = (v) => {
  const it = items(v);
  const message = it.findIndex((x) => x.from === "Patient");
  const reply = it.findIndex((x) => x.from === "Practice");
  // The channel's state shows only where it changes what the physician sees: on the review and
  // the card. Before a message is linked, and once it is settled, disconnecting changes nothing
  // on screen except Today's one disconnected line.
  const off = v.channel === "Disconnected";
  if (message < 0) return "No patient message yet";
  if (v.filed <= message) return "In the channel, not yet filed";
  const match = v.match[message + 1];
  if (match === "Waiting") return "Waiting in Messages to match";
  if (match === "Unlinked") return "Left unlinked";
  if ((v.history ?? []).length) return "Answered: history shows the reply sent";
  const sent = reply > message ? ", your reply not yet seen" : "";
  if ((v.handled ?? []).includes(message + 1)) return `Settled${sent}`;
  const c = v.card;
  if (!c.open) return `Coda reviewing${sent}${off ? ", channel disconnected" : ""}`;
  if (c.state === "Channel disconnected") {
    return off ? `Card empty: channel disconnected${sent}` : `Card empty, reconnected: waits for Done or your reply${sent}`;
  }
  return `Reply on Today${sent}${off ? ", channel disconnected" : ""}`;
};

const steps = {
  PatientWrites: "the patient writes",
  PracticeSends: "you send a reply in the channel",
  Confirm: "a physician confirms the patient",
  LeaveUnlinked: "a physician leaves it unlinked",
  ReviewReads: "Coda reads the conversation and the chart",
  Disconnect: "the practice disconnects the channel",
  Reconnect: "the practice reconnects the channel",
  OpenCard: "you open the reply",
};

// Editing, revising and copying are drawn in the other charts; here a card shows only whether it
// has a draft.
const hidden = ["Edit", "AskRevision", "RevisionFinishes", "CopyReply", "TakeUpdate", "Rest"];

export const step = (action, from, to) => {
  if (hidden.includes(action)) return null;
  if (action === "Files") {
    if ((to.history ?? []).length) return "Coda sees the reply you sent";
    return state(to).startsWith("Waiting") ? "Coda files it [no sure match]" : "Coda files it [conversation linked]";
  }
  if (action === "ReviewFinishes") {
    if (!to.card.open) return "Coda decides no reply is needed";
    return to.card.state === "Channel disconnected" ? "Coda's review opens the card empty" : "Coda drafts a reply";
  }
  if (action === "Close") return from.card.state === "Channel disconnected" ? "you mark it Done" : "you mark No reply needed";
  return steps[action] ?? action;
};

export const group = (v) => {
  const s = state(v);
  if (s.includes("disconnected")) return "Disconnected";
  if (s.startsWith("Answered") || s.startsWith("Settled") || s === "Left unlinked") return "Settled";
  if (s.startsWith("Reply")) return "On Today";
  return "Arriving";
};

export const colors = { Arriving: "blue", "On Today": "teal", Disconnected: "pink", Settled: "tan" };

// The words of the move chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-move.cfg. Coda's reply to one message is ready on patient A, and you have a draft of it. You
// may access both patients; another physician may access only patient A. Coda acts on a move to
// patient B or to not a patient, and patient B's review writes the reply again. It leaves out the
// patient writing, editing, closing, the practice and the channel.

export const title = "Coda acts on a move";

const reply = (v) => {
  if (v.link === "Not a patient") return "Moved to not a patient: the reply and the message are closed";
  const on = `on ${v.reply.for}`;
  return v.reply.state === "Withheld" ? `Reply withheld ${on}, Coda is writing it again` : `Reply ready ${on}`;
};

export const state = (v) => {
  if (v.link === "Not a patient") return reply(v);
  const yours = v.draft.you.on === v.reply.id ? "you see your draft" : "your draft stays on patient A's reply";
  const other = v.reply.for === "patient A" ? "the other physician sees it" : "the other physician lost it";
  return `${reply(v)}; ${yours}; ${other}`;
};

export const step = (action, from, to) => {
  if (action === "CodaActsOnMove")
    return to.link === "Not a patient" ? "Coda acts on a move to not a patient" : `Coda acts on a move to ${to.link}`;
  return { ReviewStarts: "patient B's review starts", ReviewFinishes: "patient B's review writes the reply" }[action] ?? null;
};

export const group = (v) => (v.link === "patient A" ? "On patient A" : v.link === "Not a patient" ? "Closed" : "On patient B");

export const colors = { "On patient A": "teal", "On patient B": "blue", Closed: "tan" };

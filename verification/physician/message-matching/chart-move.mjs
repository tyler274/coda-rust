// The words of the move chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-move.cfg. The conversation starts linked to patient A with one message. You may access both
// patients; another physician may access only patient A and has the conversation open. You move it
// to patient B or to not a patient, then press Undo or let Undo go away. The patient writes again
// only while Undo shows. The chart draws a new message as one state from the time the patient writes
// it, so it does not draw Coda deciding about it: it follows the conversation. It leaves out the new
// message being filtered, a new message at other times, opening and closing the conversation, and
// the other physician moving it.

export const title = "You move a conversation to the right place";

const where = (v) => (v.link === "Not a patient" ? "Not a patient" : `On ${v.link}`);

export const state = (v) => {
  const undo = v.undo.by !== "Nobody" ? ", Undo shows" : v.acted ? ", Coda acted on the move" : "";
  const other = v.viewing.includes("another physician") ? "the other physician has it open" : "closed for the other physician";
  const news = v.msg[2] === "Not written" ? "" : "; a new message in it";
  return `${where(v)}${undo}; ${other}${news}`;
};

const steps = {
  UndoIt: "you press Undo",
  UndoGoesAway: "Undo goes away, and Coda acts on the move",
  PatientWrites: "the patient writes again",
};

export const step = (action, from, to) =>
  action === "Move" ? (to.link === "Not a patient" ? "you move it to not a patient" : "you move it to patient B")
    : steps[action] ?? null;

export const group = (v) => (v.undo.by !== "Nobody" ? "Undo shows" : v.link === "patient A" ? "On patient A" : "Moved");

export const colors = { "On patient A": "teal", "Undo shows": "blue", Moved: "tan" };

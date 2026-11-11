// The words of the move chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-move.cfg. The conversation starts linked to patient A with one message. You may access both
// patients; another physician may access only patient A and has the conversation open. You move it
// to patient B or to not a patient, then press Undo or let Undo go away. The patient writes again
// only while Undo shows; the chart draws the write and Coda's read as one arrow, since nothing shows
// in between. The new message shows nowhere until Coda decides, and then it follows the
// conversation. The channel keeps working. It leaves out the new message being filtered, a new message at other times, opening
// and closing the conversation, and the other physician moving it.

export const title = "You move a conversation to the right place";

const where = (v) => (v.link === "Not a patient" ? "Not a patient" : `On ${v.link}`);

export const state = (v) => {
  const undo = v.undo.by !== "Nobody" ? `, Undo shows (back to ${v.undo.from})` : v.acted ? ", Coda acted on the move" : "";
  const other = v.viewing.includes("another physician") ? "the other physician has it open" : "closed for the other physician";
  const news = { undefined: "", "Not read": "", "Coda decides": "; a new message, Coda decides (shows nowhere)" }[v.msg[2]] ?? "; a new message in it";
  return `${where(v)}${undo}; ${other}${news}`;
};

const steps = {
  UndoIt: "you press Undo",
  UndoGoesAway: "Undo goes away, and Coda acts on the move",
  CodaReads: "the patient writes again, and Coda reads it",
  CodaDecides: "Coda decides it's from a patient, or 15 minutes pass: it follows the conversation",
  FifteenMinutesPass: "Coda decides it's from a patient, or 15 minutes pass: it follows the conversation",
};

export const step = (action, from, to) =>
  action === "Move" ? (to.link === "Not a patient" ? "you move it to not a patient" : `you move it to ${to.link}`)
    : steps[action] ?? null;

export const group = (v) => (v.undo.by !== "Nobody" ? "Undo shows" : v.link === "patient A" ? "On patient A" : "Moved");

export const colors = { "On patient A": "teal", "Undo shows": "blue", Moved: "tan" };

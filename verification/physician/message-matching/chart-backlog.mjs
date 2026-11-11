// The words of the backlog chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-backlog.cfg: one physician who may access both patients, and one message. Coda keeps the
// message, you disconnect the channel and connect it again, and Coda offers to review the messages
// from before it connected. A kept message shows as linked or not linked yet, whatever else Coda
// decides about it; chart draws that. It leaves out a message Coda can't read (chart-cant-read draws
// that), the channel stopping (chart-channel), linking, moves, opening the conversation, and a
// delivery of a message Coda already kept, which changes nothing.

export const title = "The backlog after you connect again";

const message = (v) => {
  const m = v.msg[1];
  if (m === "Not read") return v.written === 0 ? "No message yet" : v.start < 1 ? "A message Coda hasn't read" : "A message from before you connected";
  if (m === "Deleted") return "Deleted";
  return v.everLinked[1] ? "Linked to its patient" : "Coda kept the message, not linked yet";
};

const offer = { "no offer": "", offered: "; Coda offers the backlog", accepted: "; you accepted the backlog" };

export const state = (v) =>
  v.channel === "Disconnected" ? `${message(v)}; disconnected` : `${message(v)}${offer[v.backlog]}`;

const steps = {
  PatientWrites: "the patient writes",
  YouDisconnect: "you disconnect the channel",
  YouConnect: "you connect the channel",
  YouAcceptBacklog: "you accept the offer to review the backlog",
};

export const step = (action, from, to) => {
  if (action === "CodaReads") return "Coda reads and keeps it";
  if (action === "CodaDecides" || action === "FifteenMinutesPass")
    return to.msg[1] === "Linked" ? "Coda decides it's from a patient, or 15 minutes pass: it links on its own" : null;
  return steps[action] ?? null;
};

export const group = (v) => (v.backlog === "no offer" ? "No offer" : v.backlog === "offered" ? "Offered" : "Accepted");

export const colors = { "No offer": "teal", Offered: "orange", Accepted: "blue" };

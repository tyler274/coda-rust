// The words of the can't-read chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-cant-read.cfg: one physician who may access both patients, and one message. Coda reads the
// message and keeps it, or keeps that it can't read it, and the channel can be disconnected and
// connected again. A kept message shows as linked or not linked yet, whatever else Coda decides about
// it; chart draws that. It leaves out the channel stopping (chart-channel draws that), the backlog
// offer (chart-backlog draws that), linking, moves, opening the conversation, and a delivery of a
// message Coda already kept, which changes nothing.

export const title = "A message Coda can't read";

const message = (v) => {
  const m = v.msg[1];
  if (m === "Not read")
    return v.written === 0 ? "No message yet" : v.start < 1 ? "A message Coda hasn't read" : "Backlog from before you connected";
  if (m === "Coda can't read it") return "On Today: Coda can't read the message";
  if (m === "Marked Done") return "Marked Done";
  if (m === "Deleted") return "Deleted";
  return v.everLinked[1] ? "Linked to its patient" : "Coda kept the message, not linked yet";
};

export const state = (v) => (v.channel === "Disconnected" ? `${message(v)}; disconnected` : message(v));

const steps = {
  PatientWrites: "the patient writes",
  CodaReadsItNow: "Coda reads the message it couldn't read",
  YouMarkDone: "you mark it Done",
  YouDisconnect: "you disconnect the channel",
  YouConnect: "you connect the channel",
};

export const step = (action, from, to) => {
  if (action === "CodaReads") return to.msg[1] === "Coda can't read it" ? "Coda can't read the message" : "Coda reads and keeps it";
  if (action === "CodaDecides" || action === "FifteenMinutesPass")
    return to.msg[1] === "Linked" ? "Coda decides it's from a patient, or 15 minutes pass: it links on its own" : null;
  return steps[action] ?? null;
};

export const group = (v) =>
  v.msg[1] === "Coda can't read it" ? "On Today" : ["Marked Done", "Deleted"].includes(v.msg[1]) ? "Gone" : "Other";

export const colors = { "On Today": "orange", Gone: "tan", Other: "teal" };

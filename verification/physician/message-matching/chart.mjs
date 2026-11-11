// The words of the message matching chart, for PStack's statechart skill. Run it with Charts.tla and
// chart.cfg: one physician who may access both patients, and one message. It leaves out a second
// message (it follows the link, or waits again after the first is left unlinked), opening the
// conversation, and moves; chart-move draws moves. The channel keeps working: chart-channel draws
// a message Coda can't read, the channel stopping, and a disconnect. Patient A and patient B show as one "its patient".

export const title = "A patient message gets to its patient";

const places = {
  "Coda decides": "Coda decides (shows nowhere)",
  Filtered: "On the Filtered list",
  "Waiting to match": "Waiting on Messages to match",
  Linked: "Linked to its patient",
  "Left unlinked": "Left unlinked",
};

export const state = (v) =>
  v.msg[1] === "Not read" ? (v.written === 0 ? "No message yet" : "In the channel, not read yet") : places[v.msg[1]];

const result = (to) => (to.msg[1] === "Linked" ? "it links on its own" : "it waits for a physician");

export const step = (action, from, to) => {
  switch (action) {
    case "PatientWrites": return "the patient writes";
    case "CodaReads": return "Coda reads it";
    case "CodaDecides":
    case "FifteenMinutesPass": return `Coda decides it's from a patient, or 15 minutes pass: ${result(to)}`;
    case "CodaFilters": return "Coda decides it isn't from a patient";
    case "ThisIsAPatient": return `you mark This is a patient: ${result(to)}`;
    case "LinkIt": return "you link it to a patient you may access";
    case "LeaveUnlinked": return "you leave it unlinked";
    default: return null;
  }
};

export const group = (v) =>
  ["Not read", "Coda decides"].includes(v.msg[1]) ? "Nowhere"
    : v.msg[1] === "Waiting to match" ? "Waits for a physician" : "Matching done";

export const colors = { Nowhere: "tan", "Waits for a physician": "blue", "Matching done": "teal" };

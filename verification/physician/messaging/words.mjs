// The physician's words that every chart shares, for PStack's statechart skill: what you, the
// physician reading the chart, see of the conversation, and what each step looks like to you. Each
// chart-*.mjs picks from these.

export const you = "you";
const items = (v) => Object.values(v.inbox ?? {});
const at = (seq, i) => (seq ?? {})[i];
const patientItems = (v) =>
  items(v).map((x, k) => ({ ...x, k: k + 1 })).filter((x) => x.from === "Patient" && (v.read ?? []).includes(x.k));

// Why a handled message is off Today.
const handled = {
  "Practice answered": "History says the practice answered",
  "No reply needed": "no reply needed",
  "Left unlinked": "left unlinked",
  "Not a patient": "not a patient",
  Done: "marked Done",
  Deleted: "deleted with the disconnect",
};

// What Today says about the channel, in Arrival.tla.
export const channelLine = (v) => {
  const c = v.channel;
  if (!c || typeof c === "string") return "";
  if (c.state === "Can't read") {
    return c.reason === "an outage" ? "Coda can't read Spruce: an outage" : "Coda can't read Spruce, Reconnect";
  }
  if (c.state === "Disconnected") return "Spruce disconnected, Reconnect";
  if (c.state === "Ended") return "Spruce was disconnected";
  return "";
};

// What you see of the conversation in Arrival.tla: the lists before it's linked, then the
// patient's Today row.
export const messages = (v) => {
  const parts = [];
  const ms = patientItems(v);
  const where = (k) => at(v.match, k);
  const deciding = ms.some((m) => where(m.k) === "Sorting");
  if (ms.some((m) => where(m.k) === "Filtered")) parts.push("on Filtered");
  if (ms.some((m) => where(m.k) === "Waiting")) parts.push("on Messages to match, with Coda's suggestion");
  const open = ms.filter((m) => where(m.k) === "Linked" && !at(v.settled, m.k).why);
  if (open.length) {
    const row = [];
    if (open.some((m) => !m.readable)) row.push("Couldn't read the message");
    if (v.reply.open) row.push(v.reply.withheld ? "reply withheld" : "Reply ready");
    else if (v.review[v.link]?.on) row.push("Coda is reviewing");
    parts.push(`Today: ${row.join(", ") || "Coda is reviewing"}`);
  } else if (v.reply.open) parts.push("Today: Reply ready");
  // A message Coda is deciding shows nowhere, so it's named only when nothing else shows.
  if (!parts.length && deciding) parts.push("Coda is deciding, shown nowhere");
  if (!parts.length && ms.length) {
    const last = ms.at(-1);
    const why = at(v.match, last.k) === "Deleted" ? "Deleted" : at(v.settled, last.k).why;
    parts.push(`Off Today: ${handled[why] ?? "Nothing on Today"}`);
  }
  return parts.length ? parts.join("; ") : "Nothing on Today";
};

// The conversation and, when Coda can't read the channel, what Today says about it.
export const arrival = (v) => [messages(v), channelLine(v)].filter((x) => x && x !== "Nothing on Today").join("; ") || "Nothing on Today";

// Your own draft on the reply, or Coda's, in Reply.tla.
export const draftText = (v, d = you) => {
  const own = v.reply.open && v.draft[d].of === v.reply.no;
  if (!own) return "Coda's draft";
  return {
    Yours: "your draft",
    Revising: "Coda revising your draft",
    Revised: "Coda's revision of your draft",
    Taken: "Coda's update, which you chose",
  }[v.draft[d].text];
};

const offered = (v, d = you) =>
  v.reply.open && v.draft[d].of === v.reply.no && v.draft[d].behind && v.draft[d].text !== "Revising";

// What you see in Reply.tla: off Today and why, Coda reviewing, or the reply and its card.
export const reply = (v, { draft = true, copies = false, practice = false, cut = false } = {}) => {
  const ms = patientItems(v);
  if (!ms.length) return v.inbox && Object.keys(v.inbox).length ? "Nothing on Today, Coda hasn't read it" : "Nothing on Today";
  const access = v.access[you] ?? [];
  const patient = v.reply.open ? v.reply.patient : v.link;
  if (patient !== "Not a patient" && !access.includes(patient)) return "Nothing on Today: you may not access its patient";
  const concern = v.concern.state === "Open" ? ", a concern" : "";
  if (!v.reply.open) {
    const open = ms.filter((m) => !at(v.settled, m.k).why);
    if (open.length && v.review[v.link]?.on) return `Today: Coda is reviewing${concern}`;
    if (concern) return "Today: a concern";
    const last = ms.at(-1);
    return `Off Today: ${handled[at(v.settled, last.k).why] ?? "Coda is reviewing"}`;
  }
  if (v.reply.withheld) return `The reply withheld${concern}`;
  const bits = draft ? [draftText(v)] : [];
  if (v.reply.told) {
    const newest = Math.max(...ms.map((m) => m.k));
    const basis = v.reply.basis ?? [];
    bits.push(basis.some((k) => k > newest) || basis.includes(newest) ? "an older message shown late" : "the patient wrote again");
  }
  if (offered(v)) bits.push("Coda's update offered");
  if (copies && v.copied.who !== "Nobody") bits.push(v.copied.who === you ? "you copied it" : "another physician copied it");
  if (practice) {
    const sent = items(v).findIndex((x) => x.from === "Practice") + 1;
    if (sent) bits.push((v.read ?? []).includes(sent) ? "showing what the practice wrote" : "the practice wrote in Spruce, Coda hasn't seen it");
  }
  if (cut && v.reply.cut) bits.push("says Spruce is disconnected");
  return bits.length ? `The reply: ${bits.join("; ")}${concern}` : `The reply ready${concern}`;
};

// What a finished review did, read from the states before and after.
export const reviewed = (from, to) => {
  const n = Object.keys(to.settled ?? {});
  const newly = n.filter((k) => to.settled[k].why && !from.settled[k].why).map((k) => to.settled[k].why);
  const flags = to.concern && to.concern.state === "Open" && from.concern.state !== "Open" ? " and flags a concern" : "";
  if (newly.includes("Practice answered")) return `Coda finds the practice answered${flags}`;
  if (to.reply.open) {
    if (from.reply.withheld) return `Coda redrafts the withheld reply${flags}`;
    return `${from.reply.open ? "Coda updates the reply" : "Coda drafts a reply"}${flags}`;
  }
  if (from.reply.withheld) return `Coda closes the withheld reply${flags}`;
  return `Coda finds no reply is needed${flags}`;
};

// What the practice sent, as the step that sent it.
export const sends = (to) => {
  const w = items(to).at(-1).words;
  if (w === "An automatic reply") return "Spruce sends an automatic reply";
  if (w === "Words written in the channel") return "you write in Spruce";
  return "you paste the reply in Spruce and send";
};

// A verb as another physician does it: "you copy" is "another physician copies".
const third = (verb) => (/[^aeiou]y$/.test(verb) ? `${verb.slice(0, -1)}ies` : /o$/.test(verb) ? `${verb}es` : `${verb}s`);

// Where Coda's decision sends a message, from the states before and after: it links on its own when
// its conversation, its sender or the channel points to one patient, and otherwise it waits.
const decided = (from, to) => {
  const k = Object.keys(to.match).find((i) => from.match[i] === "Sorting" && to.match[i] !== "Sorting");
  if (to.match[k] === "Linked") return "it links on its own";
  if (to.match[k] === "Not a patient") return "it follows the conversation to not a patient";
  return "it waits for a person";
};

// The step's label in your words, from the step just taken.
export const label = (from, to) => {
  const e = to.event;
  const who = e.who === you ? "you" : e.who;
  switch (e.name) {
    case "Coda's review finishes": return reviewed(from, to);
    case "Coda decides it's from a patient":
    case "15 minutes pass":
      return `Coda decides it's from a patient or 15 minutes pass, and ${decided(from, to)}`;
    case "the practice sends in Spruce": return sends(to);
    case "you move it to another patient": return "you move it to the other patient";
    case "a physician moves it to another patient": return "a physician moves it to the other patient";
    case "Coda can't read Spruce":
      return to.channel.reason === "an outage" ? "Spruce has an outage" : "the grant is revoked or an administrator blocks Coda";
    default: return who === "you" ? e.name : e.name.replace(/^you (\w+)/, (_, verb) => `${who} ${third(verb)}`);
  }
};

export const groupOf = (s) =>
  s.startsWith("Off") || s.startsWith("Nothing") ? "Off Today" : s.includes("reply") || s.includes("Reply") ? "Reply" : "Arriving";
export const colors = { Arriving: "blue", Reply: "teal", "Off Today": "tan" };

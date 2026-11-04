// The words of the scenes, for PStack's statechart skill. Run it with Flows.tla and flows.cfg: each
// flow's path is told step by step, with what stands after each step.

export const title = "Duet sign-in scenes";

const whose = (a) => (a === "Yours" ? "your" : "another person's");

// What the phone shows, whose it is, and what it keeps or waits on.
const phoneWords = (v) => {
  const p = v.phone;
  const notes = [];
  if (p.screen !== "Duet closed" && p.whose !== "Nobody") notes.push(`${whose(p.whose)} account`);
  if (p.screen === "Duet closed" && p.holds !== "No session") notes.push(`keeps ${whose(v.session[p.holds].split(",")[0])} session`);
  if (p.draft !== "Nobody") notes.push("a draft");
  if (p.entries !== "Nobody") notes.push("entries Duet hasn't confirmed");
  if (p.asks !== "Nothing asked") notes.push(p.asks.startsWith("Checking") ? "checking the session" : "agreeing to the Terms");
  if (p.sync !== "No entries going") notes.push("sending the entries");
  if (p.signIn !== "No link opening") notes.push("redeeming a link");
  if (p.told) notes.push("needs to check with Duet");
  if (p.owes.length) notes.push("owes Duet a sign-out");
  return `the phone: ${p.screen}${notes.length ? ` (${notes.join(", ")})` : ""}`;
};

// What Duet holds: each session started, and which accounts have accepted the Terms.
const duetWords = (v) => {
  const sessions = Object.entries(v.session).filter(([, s]) => s !== "Not signed in yet")
    .map(([name, s]) => `${name.toLowerCase()} ${s.replace("Yours", "yours").replace("Theirs", "another person's")}`);
  const terms = Object.entries(v.accepted).filter(([, yes]) => yes).map(([a]) => whose(a));
  const parts = [];
  if (sessions.length) parts.push(sessions.join(", "));
  if (terms.length) parts.push(`Terms accepted for ${terms.join(" and ")} account`);
  return parts.length ? `Duet: ${parts.join("; ")}` : "Duet: no session";
};

export const state = (v) => [phoneWords(v), duetWords(v), v.removed && "something was removed unwarned"]
  .filter(Boolean).join("; ");

export const step = (action, from, to) => to.event.name.replace(" for Yours", " for your email")
  .replace(" for Theirs", " for another person's email");

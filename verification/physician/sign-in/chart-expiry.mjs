// The words of the chart of a session expiring with two tabs open, for PStack's statechart skill.
// Run it with Charts.tla and chart-expiry.cfg: tab A starts at a patient and tab B at Today, the
// session expires, and you sign back in from the link, which opens in tab B. It leaves out losing
// access, another physician, asking for a link, and opening anything but the link in tab B before
// you sign back in.

export const title = "Your session expires with two tabs open";

const screen = (t) => {
  if (t.shows === "Your workspace") return t.opens === "The linked patient" ? "your workspace at the patient" : "your workspace";
  if (t.shows === "Opening your workspace") return "opening your workspace";
  if (t.shows === "Sign-in prompt over your work") return t.opens === "The linked patient" ? "the prompt over the patient" : "the prompt over your work";
  return t.shows;
};

// The tab a step moved: the one whose screen or sign-in changed, else the one whose read changed.
// The chart's states leave out which step was taken, so the label reads it from the change.
const same = (a, b, keys) => keys.every((k) => a[k] === b[k]);
const moved = (from, to) => {
  const tabs = Object.keys(to.tab);
  return tabs.find((k) => !same(from.tab[k], to.tab[k], ["shows", "signIn", "whose"]))
    ?? tabs.find((k) => !same(from.tab[k], to.tab[k], ["read", "presents", "opens", "link"]))
    ?? tabs.find((k) => !same(from.tab[k], to.tab[k], ["news"])) ?? "";
};

// Whether your first session has expired with no new one signed in since.
const expired = (v) => v.session["Session 1"] === "Yours, expired" && v.session["Session 2"] === "Not signed in yet";

export const state = (v) => `${expired(v) ? "Expired · " : ""}A: ${screen(v.tab.A)} · B: ${screen(v.tab.B)}`;

const steps = {
  Expires: "your session expires",
  WorkspaceReads: "the workspace reads",
  RequestLink: "you ask for a sign-in link",
  OpenSignInLink: "you open your sign-in link",
  CodaChecksLink: "Coda checks the link",
  CodaSignsIn: "Coda signs you in",
  CodaAnswersWho: "Coda answers who signed in",
  HearsNews: "hears the news",
};

export const step = (action, from, to) => {
  const t = moved(from, to);
  if (action === "Expires") return steps.Expires;
  if (action === "CodaAnswersRead") return `${t}: Coda answers the read: ${screen(to.tab[t])}`;
  return `${t}: ${steps[action] ?? action}`;
};

export const group = (v) => (expired(v) ? "Session ended" : "Signed in");
export const colors = { "Session ended": "orange", "Signed in": "teal" };

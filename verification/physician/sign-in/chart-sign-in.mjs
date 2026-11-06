// The words of the chart of signing in with two tabs open, for PStack's statechart skill. Run it
// with Charts.tla and chart-sign-in.cfg: tab A, opened on a patient link while signed out, waits on
// Check your email, you open your sign-in link in tab B and tab A goes on to that patient, then you open your link again while signed in
// and your session stands. It leaves out a link that fails, Coda being unreachable, expiry, losing
// access, signing out and another physician.

export const title = "You sign in with two tabs open";

const screen = (t) => {
  if (t.shows === "Closed") return "closed";
  if (t.shows === "Your workspace") return t.opens === "The linked patient" ? "your workspace at the patient" : "your workspace at Today";
  if (t.shows === "Opening your workspace") return "opening your workspace";
  // Your own link while signed in shows the same page, checked against the session you have.
  if (t.shows === "Signing you in" && t.signIn.startsWith("Checking")) return "Signing you in again";
  if (t.link === "Holding a patient link") return `${t.shows}, holding a patient link`;
  return t.shows;
};

// The tab a step moved: the one whose screen or sign-in changed, else the one whose read or news
// changed. The chart's states leave out which step was taken, so the label reads it from the change.
const same = (a, b, keys) => keys.every((k) => a[k] === b[k]);
const moved = (from, to) => {
  const tabs = Object.keys(to.tab);
  return tabs.find((k) => !same(from.tab[k], to.tab[k], ["shows", "signIn", "whose"]))
    ?? tabs.find((k) => !same(from.tab[k], to.tab[k], ["read", "presents"]))
    ?? tabs.find((k) => !same(from.tab[k], to.tab[k], ["news"])) ?? "";
};

const signedIn = (v) => v.stores !== "No session";

export const state = (v) => `${signedIn(v) ? "Signed in · " : ""}A: ${screen(v.tab.A)} · B: ${screen(v.tab.B)}`;

const steps = {
  OpenSignInLink: "you open your sign-in link",
  CodaChecksLink: "Coda keeps your session",
  CodaSignsIn: "Coda signs you in",
  CodaAnswersWho: "Coda answers who signed in",
  HearsNews: "hears the news",
  WorkspaceReads: "the workspace reads",
};

export const step = (action, from, to) => {
  const t = moved(from, to);
  if (action === "CodaAnswersRead") return `${t}: Coda answers the read: ${screen(to.tab[t])}`;
  return `${t}: ${steps[action] ?? action}`;
};

export const group = (v) => (signedIn(v) ? "Signed in" : "Signed out");
export const colors = { "Signed in": "teal", "Signed out": "orange" };

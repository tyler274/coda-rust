// The words of the chart of signing out with two tabs open, for PStack's statechart skill. Run it
// with Charts.tla and chart-sign-out.cfg: both tabs start at your workspace, you sign out in tab B,
// and tab A follows when it hears the news, or sooner when Coda refuses its read or a request. It
// leaves out signing back in, expiry, losing access and another physician.

export const title = "You sign out with two tabs open";

const screen = (t) => {
  if (t.shows === "Your workspace") return "your workspace";
  if (t.shows === "Opening your workspace") return "opening your workspace";
  if (t.shows === "Sign-in prompt, with no patient content") return "the sign-in prompt";
  return t.shows;
};

// The tab a step moved: the one whose screen changed, else the one whose read or news changed.
// The chart's states leave out which step was taken, so the label reads it from the change.
const same = (a, b, keys) => keys.every((k) => a[k] === b[k]);
const moved = (from, to) => {
  const tabs = Object.keys(to.tab);
  return tabs.find((k) => !same(from.tab[k], to.tab[k], ["shows", "whose"]))
    ?? tabs.find((k) => !same(from.tab[k], to.tab[k], ["read", "presents"]))
    ?? tabs.find((k) => !same(from.tab[k], to.tab[k], ["news"])) ?? "";
};

const signedOut = (v) => v.stores === "No session";

export const state = (v) => `${signedOut(v) ? "Signed out · " : ""}A: ${screen(v.tab.A)} · B: ${screen(v.tab.B)}`;

const steps = {
  SignOut: "you sign out",
  WorkspaceReads: "the workspace reads",
  HearsNews: "hears the news",
  ActsInTab: "you act in the workspace, and Coda refuses it",
};

export const step = (action, from, to) => {
  const t = moved(from, to);
  if (action === "CodaAnswersRead") return `${t}: Coda answers the read: ${screen(to.tab[t])}`;
  return `${t}: ${steps[action] ?? action}`;
};

export const group = (v) => (signedOut(v) ? "Signed out" : "Signed in");
export const colors = { "Signed out": "orange", "Signed in": "teal" };

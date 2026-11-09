// The words of the chart of another physician signing in, for PStack's statechart skill. Run it
// with Charts.tla and chart-another.cfg: tab A starts at your workspace, and another physician
// signs in from tab B. It leaves out expiry, losing access, Coda being unreachable when it checks
// the link, and opening anything else. Acting in tab A changes nothing it shows, since Coda refuses
// the request and tab A catches up, so the chart draws no arrow for it.

export const title = "Another physician signs in from another tab";

const screen = (t) => {
  if (t.shows === "Closed") return "closed";
  const whose = t.whose === "Yours" ? "your" : t.whose === "Nobody's" ? "no one's" : "their";
  // Opening and showing their workspace lead on the same way, so they share a state.
  if (t.shows === "Your workspace" || (t.shows === "Opening your workspace" && whose === "their")) return `${whose} workspace`;
  if (t.shows === "Opening your workspace") return `opening ${whose} workspace`;
  if (t.shows === "Sign-in prompt, with no patient content") return `the sign-in prompt for ${whose} session`;
  // The link page checks with Coda who is signed in before it redeems the link.
  if (t.signIn.startsWith("Checking")) return "checking their link";
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

export const state = (v) => `A: ${screen(v.tab.A)} · B: ${screen(v.tab.B)}`;

const steps = {
  OpenSignInLink: "opens their sign-in link",
  CodaChecksLink: "Coda checks the link: you're signed in, so you're signed out first",
  CodaSignsIn: "Coda signs them in",
  CodaAnswersWho: "Coda answers who signed in",
  WorkspaceReads: "the workspace reads",
  HearsNews: "hears the news",
};

export const step = (action, from, to) => {
  const t = moved(from, to);
  if (action === "ActsInTab") return `${t}: you act: Coda refuses it, your session has ended`;
  if (action === "CodaAnswersRead") return `${t}: Coda answers the read: ${screen(to.tab[t])}`;
  // Whether B has finished signing them in decides where A's news takes it.
  if (action === "HearsNews") return `${t}: hears the news [${from.stores === "No session" ? "no one signed in yet" : "they're signed in"}]`;
  return `${t}: ${steps[action] ?? action}`;
};

export const group = (v) => (v.tab.A.whose === "Yours" ? "Tab A still yours" : "Tab A caught up");
export const colors = { "Tab A still yours": "orange", "Tab A caught up": "teal" };

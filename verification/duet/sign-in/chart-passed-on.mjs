// The words of the chart of the phone passing to another person, for PStack's statechart skill.
// Run it with Charts.tla and chart-passed-on.cfg: you're signed in with a draft and unconfirmed
// entries, you sign out, and another person signs in on the same phone. It leaves out a session
// ended elsewhere, a link that no longer works, writing or logging anything new, and your own link.

export const title = "One phone, another person";

const screens = {
  "Enter your email": "Enter your email",
  "Check your email": "Check your email",
  "Signing you in": "Signing in",
  "Duet couldn't be reached": "Duet couldn't be reached",
  "This link no longer works": "This link no longer works",
  Terms: "Terms",
  Conversation: "Open",
  "Signing out: sending unconfirmed entries": "Signing out, sending entries",
};

// What the warning says goes, from what the phone still holds.
const warned = (p) =>
  p.entries === "Nobody" ? "Warned the draft goes"
    : p.draft === "Nobody" ? "Warned the entries go" : "Warned the draft and entries go";

export const state = (v) => {
  const p = v.phone;
  const screen = p.screen === "Signing out removes what Duet hasn't confirmed" ? warned(p) : screens[p.screen] ?? p.screen;
  if (p.whose === "Nobody") return screen;
  if (screen === "Terms" && p.asks.startsWith("Accepting")) return "Terms agreed, waiting for Duet: theirs";
  return `${screen}: ${p.whose === "Yours" ? "yours" : "theirs"}`;
};

const steps = {
  EnterEmail: "they enter their email",
  OpenLink: "they open the link",
  DuetSignsIn: "Duet signs them in",
  AgreeTerms: "they agree to the Terms",
  DuetRecordsTerms: "Duet records the Terms",
  StartSignOut: "you sign out",
  DuetConfirmsEntries: "Duet confirms your entries",
  StaySignedIn: "you stay signed in",
  SignOutAnyway: "you sign out anyway",
  TellDuet: "the phone tells Duet you signed out",
  SignOutUnheard: "the phone tells Duet you signed out, but the answer is lost",
  CantReach: "Duet can't be reached",
};

export const step = (action, from) => {
  if (action === "StartSignOut") {
    const { entries, draft } = from.phone;
    const who = from.phone.whose === "Yours" ? "you sign out" : "they sign out";
    return `${who} [${entries !== "Nobody" ? "with unconfirmed entries" : draft !== "Nobody" ? "with a draft" : "nothing unconfirmed"}]`;
  }
  if (action === "DuetConfirmsEntries" && from.phone.screen.startsWith("Signing out:")) {
    return from.phone.draft === "Nobody" ? `${steps[action]}, nothing else unconfirmed` : `${steps[action]}, but the draft is left`;
  }
  if (action === "CantReach" && from.phone.screen.startsWith("Signing out")) return "Duet can't confirm your entries";
  return steps[action];
};

export const group = (v) => (v.phone.whose === "Yours" ? "Yours" : v.phone.whose === "Nobody" ? "Signed out" : "Theirs");
export const colors = { Yours: "teal", "Signed out": "tan", Theirs: "purple" };

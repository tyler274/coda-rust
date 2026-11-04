// The words of the chart of the phone passing to another person, for PStack's statechart skill.
// Run it with Charts.tla and chart-passed-on.cfg: you're signed in with a draft and unconfirmed
// entries, you sign out, and another person signs in on the same phone.

export const title = "One phone, another person";

const screens = {
  "Enter your email": "Enter your email",
  "Check your email": "Check your email",
  "Signing you in": "Signing in",
  "Duet couldn't be reached": "Duet couldn't be reached",
  "This link no longer works": "This link no longer works",
  Terms: "Terms",
  Conversation: "Conversation",
  "Signing out: sending unconfirmed entries": "Signing out, sending entries",
  "Signing out removes what Duet hasn't confirmed": "Warned before signing out",
};

export const state = (v) => {
  const p = v.phone;
  const screen = screens[p.screen] ?? p.screen;
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
  SendEntries: "the phone sends your entries",
  StartSignOut: "you sign out",
  DuetTakesEntries: "Duet takes your entries",
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
  if (action === "DuetTakesEntries" && from.phone.screen.startsWith("Signing out")) {
    return `${steps[action]} [${from.phone.draft === "Nobody" ? "nothing else unconfirmed" : "with a draft"}]`;
  }
  return steps[action];
};

export const group = (v) => (v.phone.whose === "Yours" ? "Yours" : v.phone.whose === "Nobody" ? "Signed out" : "Theirs");
export const colors = { Yours: "teal", "Signed out": "tan", Theirs: "purple" };

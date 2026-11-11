// The words of the chart of a returning person signing in, for PStack's statechart skill. Run it
// with Charts.tla and chart-returning.cfg: you signed in before and accepted the Terms, signed
// out, and sign back in. It leaves out another person, a session ended elsewhere, and anything
// done once signed in.

export const title = "Signing back in after accepting the Terms";

const screens = {
  "Enter your email": "Enter your email",
  "Check your email": "Check your email",
  "Signing you in": "Signing you in",
  "This link no longer works": "This link no longer works",
  "Duet couldn't be reached": "Duet couldn't be reached",
  Terms: "Terms",
  Conversation: "Open",
};

export const state = (v) => {
  const p = v.phone;
  if (p.screen === "Duet closed") {
    return p.holds === "No session" ? "Closed, signed out" : "Closed, reopens where you were";
  }
  const screen = screens[p.screen] ?? p.screen;
  if (p.holds !== "No session" && p.asks.startsWith("Checking")) return `${screen}, checking`;
  return screen;
};

const steps = {
  OpenDuet: "you open Duet",
  CloseDuet: "Duet closes",
  EnterEmail: "you enter your email",
  OpenLink: "you open the link",
  DuetSignsIn: "Duet signs you in",
  SignInUnheard: "Duet signs you in, but the answer is lost",
  LinkNoLongerWorks: "the link no longer works",
  CantReach: "Duet can't be reached",
  PhoneChecks: "the phone checks with Duet",
  DuetAnswersCheck: "Duet answers the check",
};

// Closing is drawn only once signed in, where Duet keeps the person signed in; closing a sign-in
// screen reopens on Enter your email, as the main chart's opening shows.
export const step = (action, from) =>
  action === "CloseDuet" && from.phone.holds === "No session" ? null : steps[action];

export const group = (v) =>
  v.phone.screen === "Duet closed" ? "Closed" : v.phone.holds === "No session" ? "Signing in" : "Signed in";
export const colors = { Closed: "tan", "Signing in": "blue", "Signed in": "teal" };

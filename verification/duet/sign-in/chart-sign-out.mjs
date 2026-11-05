// The words of the chart of signing out with unsent work, for PStack's statechart skill. Run it
// with Charts.tla and chart-sign-out.cfg: you're signed in with a draft and entries Duet hasn't
// confirmed, and you sign out of this phone. It leaves out a session ended elsewhere, which
// chart-ended draws, telling Duet of the sign-out, signing back in, and writing or logging
// anything new.

export const title = "Signing out with a draft and unconfirmed entries";

const unsent = (p) =>
  p.draft !== "Nobody" && p.entries !== "Nobody" ? "draft and entries unsent"
    : p.draft !== "Nobody" ? "draft unsent" : p.entries !== "Nobody" ? "entries unsent" : "nothing unsent";

// What the warning says goes, from what the phone still holds.
const warned = (p) =>
  p.entries === "Nobody" ? "Warned the draft goes"
    : p.draft === "Nobody" ? "Warned the entries go" : "Warned the draft and entries go";

export const state = (v) => {
  const p = v.phone;
  if (p.holds === "No session") return "Signed out";
  if (p.screen === "Duet closed") return `Closed, ${unsent(p)}`;
  if (p.screen === "Signing out: sending unconfirmed entries") return "Signing out, sending entries";
  if (p.screen === "Signing out removes what Duet hasn't confirmed") return warned(p);
  return `Conversation, ${unsent(p)}${p.asks.startsWith("Checking") ? ", checking" : ""}`;
};

const steps = {
  OpenDuet: "you open Duet",
  CloseDuet: "Duet closes",
  StartSignOut: "you sign out",
  DuetConfirmsEntries: "Duet confirms the entries",
  CantReach: "Duet can't be reached",
  DuetAnswersCheck: "Duet answers the check",
  StaySignedIn: "you stay signed in",
  SignOutAnyway: "you sign out anyway",
};

export const step = (action, from) =>
  action === "CantReach" && from.phone.screen.startsWith("Signing out:") ? "Duet can't confirm the entries" : steps[action];

export const group = (v) =>
  v.phone.holds === "No session" ? "Signed out" : v.phone.screen === "Duet closed" ? "Closed"
    : v.phone.screen.startsWith("Signing out") ? "Signing out" : "Signed in";
export const colors = { "Signed in": "teal", "Signing out": "yellow", Closed: "tan", "Signed out": "blue" };

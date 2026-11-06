// The words of the chart of signing out with work Duet hasn't confirmed, for PStack's statechart
// skill. Run it with Charts.tla and chart-sign-out.cfg: you're signed in with entries Duet hasn't
// confirmed, may write a draft, and sign out of this phone. Closing Duet ends the scene; the main
// chart draws reopening in the conversation. It leaves out a session ended elsewhere, which
// chart-ended draws, telling Duet of the sign-out, signing back in, and logging more entries.

export const title = "Signing out with work Duet hasn't confirmed";

const held = (p) =>
  p.draft !== "Nobody" && p.entries !== "Nobody" ? "draft unsent, entries unconfirmed"
    : p.draft !== "Nobody" ? "draft unsent" : p.entries !== "Nobody" ? "entries unconfirmed" : "nothing unconfirmed";

// What the warning says goes, from what the phone still holds.
const warned = (p) =>
  p.entries === "Nobody" ? "Warned the draft goes"
    : p.draft === "Nobody" ? "Warned the entries go" : "Warned the draft and entries go";

// Sending says whether a draft is held, since that decides where Duet's confirmation leads.
export const state = (v) => {
  const p = v.phone;
  if (p.holds === "No session") return "Signed out";
  if (p.screen === "Duet closed") return `Closed, reopens the conversation: ${held(p)}`;
  if (p.screen === "Signing out: sending unconfirmed entries") {
    return `Signing out, sending entries${p.draft !== "Nobody" ? ", draft unsent" : ""}`;
  }
  if (p.screen === "Signing out removes what Duet hasn't confirmed") return warned(p);
  return `Conversation, ${held(p)}`;
};

const steps = {
  WriteDraft: "you write a draft",
  CloseDuet: "Duet closes",
  StartSignOut: "you sign out",
  DuetConfirmsEntries: "Duet confirms the entries",
  CantReach: "Duet can't be reached",
  StaySignedIn: "you stay signed in",
  SignOutAnyway: "you sign out anyway",
};

export const step = (action, from) =>
  action === "CantReach" && from.phone.screen.startsWith("Signing out:") ? "Duet can't confirm the entries" : steps[action];

export const group = (v) =>
  v.phone.holds === "No session" ? "Signed out" : v.phone.screen === "Duet closed" ? "Closed"
    : v.phone.screen.startsWith("Signing out") ? "Signing out" : "Signed in";
export const colors = { "Signed in": "teal", "Signing out": "yellow", Closed: "tan", "Signed out": "blue" };

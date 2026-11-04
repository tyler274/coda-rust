// The words of the chart of a session Duet ends elsewhere, for PStack's statechart skill. Run it
// with Charts.tla and chart-ended.cfg: you're signed in with a draft and unconfirmed entries, and
// Duet ends the session while the phone may not reach it.

export const title = "Duet ends your session elsewhere";

const ended = (v) => v.phone.holds !== "No session" && v.session[v.phone.holds].endsWith("ended");

export const state = (v) => {
  const p = v.phone;
  if (p.screen === "Enter your email") return "Signed out";
  if (p.screen.startsWith("You were signed out")) return "Signed out, told the draft and entries were removed";
  if (p.holds === "No session") return p.screen === "Duet closed" ? "Closed, signed out" : p.screen;
  const where = p.screen === "Duet closed" ? "Closed" : "Conversation, draft and entries kept";
  const knows = p.asks !== "Nothing asked" ? ", checking" : p.told ? ", needs to check" : "";
  return `${where}, session ${ended(v) ? "ended elsewhere" : "live"}${p.screen === "Duet closed" ? "" : knows}`;
};

const steps = {
  OpenDuet: "you open Duet",
  CloseDuet: "Duet closes",
  DuetEndsSession: "Duet ends the session: revoked, or signed out everywhere",
  NoticeArrives: "Duet's notice reaches the app",
  PhoneChecks: "the phone checks with Duet",
  CantReach: "Duet can't be reached",
  DuetAnswersCheck: "Duet answers the check",
  DuetRefuses: "Duet refuses the session",
};

export const step = (action) => steps[action];

export const group = (v) => (ended(v) ? "Ended, still shown" : v.phone.holds === "No session" ? "Signed out" : "Signed in");
export const colors = { "Signed in": "teal", "Ended, still shown": "orange", "Signed out": "tan" };

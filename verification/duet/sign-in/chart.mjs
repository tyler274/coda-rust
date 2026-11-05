// The words of the Duet sign-in chart, for PStack's statechart skill. Run it with Charts.tla and
// chart.cfg, which draws one person's sign-in with nothing unsent: one account and one session.
// chart-sign-out draws signing out with a draft or entries Duet hasn't confirmed. This chart
// leaves out a second person on the phone and signing in again after signing out. It draws the two
// sign-in screens that didn't finish as one, and a session Duet ended elsewhere on the arrows of
// its refusal, since both look and lead on the same way. It doesn't draw closing Duet from a
// sign-in screen or while the Terms are being accepted, or opening a link anywhere but where the
// person waits for it.

export const title = "Duet sign-in on one phone";

const screens = {
  "Enter your email": "Enter your email",
  "Check your email": "Check your email",
  "Signing you in": "Signing you in",
  // Both sign-in screens that didn't finish lead on the same way; the arrows into them say which.
  "This link no longer works": "Link no longer works, or Duet couldn't be reached",
  "Duet couldn't be reached": "Link no longer works, or Duet couldn't be reached",
  Terms: "Terms",
  Conversation: "Conversation",
};

export const state = (v) => {
  const p = v.phone;
  if (p.screen === "Duet closed") {
    if (p.holds === "No session") return "Closed, signed out";
    return `Closed, reopens ${p.place === "Terms" ? "the Terms" : "the conversation"}`;
  }
  if (p.screen === "Terms" && p.asks.startsWith("Accepting")) return "Terms agreed, waiting for Duet";
  return screens[p.screen] ?? p.screen;
};

const steps = {
  OpenDuet: "you open Duet",
  CloseDuet: "Duet closes",
  EnterEmail: "you enter your email",
  OpenLink: "you open the link",
  OpenLinkSignedIn: "you open Duet",
  DuetSignsIn: "Duet signs you in",
  SignInUnheard: "Duet signs you in, but the answer is lost",
  LinkNoLongerWorks: "the link no longer works",
  CantReach: "Duet can't be reached",
  PhoneChecks: "the phone checks with Duet",
  DuetRefuses: "Duet refuses the session it ended elsewhere",
  DuetAnswersCheck: "Duet answers the check",
  AgreeTerms: "you agree to the Terms",
  DuetRecordsTerms: "Duet records the Terms",
  TermsUnheard: "Duet records the Terms, but the answer is lost",
  StartSignOut: "you sign out",
  TellDuet: "the phone tells Duet it signed out",
  SignOutUnheard: "the phone tells Duet it signed out, but the answer is lost",
  DuetEndsSession: "Duet ends the session elsewhere",
  NoticeArrives: "Duet's notice reaches the app",
};

// Any screen can close, and a link can open on any sign-in screen. The chart draws the link
// only from where the person waits for it, and closing only from the signed-in screens, since
// reopening is where Duet keeps the person signed in. Closing while the Terms are being
// accepted reopens the Terms, as closing on the Terms already shows. A link opened while signed
// in opens Duet as it was, the same as opening Duet.
const waiting = ["Closed, signed out", "Check your email"];
const notDrawnClosing = ["Enter your email", "Check your email", "Signing you in",
  "Link no longer works, or Duet couldn't be reached", "Terms agreed, waiting for Duet"];

export const step = (action, from) => {
  const at = state(from);
  if (action === "CloseDuet" && notDrawnClosing.includes(at)) return null;
  if (action === "OpenLink" && !waiting.includes(at)) return null;
  return steps[action];
};

export const group = (v) =>
  v.phone.screen === "Duet closed" ? "Closed"
    : ["Terms", "Conversation"].includes(v.phone.screen) ? "Signed in" : "Signing in";

export const colors = { Closed: "tan", "Signing in": "blue", "Signed in": "teal" };

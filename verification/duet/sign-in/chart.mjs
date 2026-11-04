// The words of the Duet sign-in chart, for PStack's statechart skill. Run it with chart.cfg,
// which draws one person's sign-in: one account and one session.

export const title = "Duet sign-in on one phone";

const screens = {
  "Duet closed": "Closed",
  "Enter your email": "Enter your email",
  "Check your email": "Check your email",
  "Signing you in": "Signing you in",
  "This link no longer works": "Link no longer works",
  "Duet couldn't be reached": "Duet couldn't be reached",
  "You were signed out; what Duet hadn't confirmed was removed from this phone":
    "Signed out, told what was removed",
  Terms: "Terms",
  Conversation: "Conversation",
  "Signing out: sending unconfirmed entries": "Signing out, sending entries",
  "Signing out removes what Duet hasn't confirmed": "Warned before signing out",
};

const sessionOf = (v) => {
  const s = Object.values(v.session)[0];
  if (s.endsWith(", live")) return "Live";
  if (s.endsWith(", ended")) return "Ended";
  return "None yet";
};

const ended = (v) => sessionOf(v) === "Ended";

export const state = (v) => {
  const screen = screens[v.phone.screen];
  if (screen === "Closed") return v.phone.holds === "No session" ? "Closed, signed out" : "Closed, signed in";
  if (screen === "Conversation" && ended(v)) return "Conversation, session ended elsewhere";
  if (screen === "Terms" && v.phone.asks.startsWith("Accepting")) return "Terms agreed, waiting for Duet";
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
  DuetRefuses: "Duet refuses the session",
  DuetAnswersCheck: "Duet answers the check",
  AgreeTerms: "you agree to the Terms",
  DuetRecordsTerms: "Duet records the Terms",
  TermsUnheard: "Duet records the Terms, but the answer is lost",
  WriteDraft: "you write a draft",
  AddEntries: "you add entries",
  SendEntries: "the phone sends entries",
  StartSignOut: "you sign out",
  DuetTakesEntries: "Duet takes the entries",
  StaySignedIn: "you stay signed in",
  SignOutAnyway: "you sign out anyway",
  TellDuet: "the phone tells Duet it signed out",
  SignOutUnheard: "the phone tells Duet it signed out, but the answer is lost",
  DuetEndsSession: "Duet ends the session elsewhere",
  NoticeArrives: "Duet's notice reaches the app",
};

// Any sign-in screen can close, and a link can open on any of them; the chart draws closing
// and the link only from where the person waits for them, and the signed-in screens' closing,
// since reopening is where Duet keeps the person signed in.
const waiting = ["Closed, signed out", "Check your email"];
const signInScreens = ["Enter your email", "Check your email", "Signing you in", "Link no longer works",
  "Duet couldn't be reached", "Signed out, told what was removed"];

export const step = (action, from) => {
  const at = state(from);
  if (action === "CloseDuet" && signInScreens.includes(at)) return null;
  if (action === "OpenLink" && !waiting.includes(at)) return null;
  // Where these go depends on work the screen doesn't show, so the label says which.
  // Both signing-out screens start "Signing out".
  if (action === "DuetTakesEntries" && from.phone.screen.startsWith("Signing out")) {
    return `${steps[action]} [${from.phone.draft === "Nobody" ? "nothing else unconfirmed" : "with a draft"}]`;
  }
  if (action === "StartSignOut" || action === "DuetRefuses") {
    const { entries, draft } = from.phone;
    const work = entries === "Nobody" && draft === "Nobody" ? "nothing unconfirmed"
      : action === "DuetRefuses" ? "with unconfirmed work"
      : entries !== "Nobody" ? "with unconfirmed entries" : "with a draft";
    return `${steps[action]} [${work}]`;
  }
  return steps[action];
};

export const group = (v) =>
  v.phone.screen === "Duet closed"
    ? "Closed"
    : ["Terms", "Conversation"].includes(v.phone.screen) || v.phone.screen.startsWith("Signing out")
      ? "Signed in"
      : "Signing in";

export const colors = { Closed: "tan", "Signing in": "blue", "Signed in": "teal" };

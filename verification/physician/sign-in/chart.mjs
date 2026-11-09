// The words of the physician sign-in chart, for PStack's statechart skill. Run it with
// chart.cfg, which draws one tab with one session. It leaves out a second tab, whose news and
// races the rules are checked against, and signing in again, which needs a second session. It
// doesn't draw opening Coda or a link from a screen other than a closed tab, or a sign-in link
// from anywhere but there and Check your email.

export const title = "Physician sign-in in one tab";

const tabOf = (v) => Object.values(v.tab)[0];

export const state = (v) => {
  const t = tabOf(v);
  switch (t.shows) {
    case "Closed":
      return v.stores === "No session" ? "Closed, signed out" : "Closed, signed in";
    case "Opening your workspace":
      return "Opening the workspace";
    case "Your workspace":
      return t.opens === "The linked patient" ? "Workspace at the linked patient" : "Workspace at Today";
    default:
      return t.shows;
  }
};

const steps = {
  OpenCoda: "you open Coda",
  OpenPatientLink: "you open a patient link",
  OpenSignInLink: "you open a sign-in link",
  RequestLink: "you ask for a sign-in link",
  UseAnotherEmail: "you use another email",
  CodaChecksLink: "Coda checks the link",
  CodaSignsIn: "Coda signs in the link",
  CodaAnswersWho: "Coda answers who signed in",
  LinkUsed: "the link was used or has expired",
  LinkNoAccess: "the link no longer grants access",
  CantReach: "Coda can't reach sign-in",
  CantSendLink: "Coda can't send the link",
  SignOut: "you sign out",
  WorkspaceReads: "the workspace reads",
  CodaAnswersRead: "Coda answers the read",
  HearsNews: "the tab hears the browser's news",
  Expires: "the session expires",
  RemoveAccess: "the practice removes your access",
  ActsInTab: "you act in the workspace",
};

// A page can load from any screen. The chart draws loading Coda or a patient link only from a
// closed tab, and a sign-in link only from there and from Check your email, where the person
// waits for it.
const loads = ["OpenCoda", "OpenPatientLink", "OpenSignInLink"];
const waitsForLink = ["Check your email"];

// What Coda's answer to a read says decides where it goes, so the label names it.
const readFinds = {
  "Sign-in prompt, with no patient content": "Coda answers the read: the session ended",
  "No access to this practice": "Coda answers the read: your access was removed",
  "Enter your work email": "Coda answers the read: no session",
};

export const step = (action, from, to) => {
  const at = state(from);
  if (loads.includes(action) && !at.startsWith("Closed")
      && !(action === "OpenSignInLink" && waitsForLink.includes(at))) return null;
  if (action === "CantReach" && Object.values(from.tab)[0].signIn.startsWith("Checking")) {
    return `${steps[action]} [the browser still stores the session]`;
  }
  if (action === "CodaAnswersRead") {
    const held = Object.values(from.tab)[0].link === "Holding a patient link";
    return readFinds[state(to)] ?? `${steps[action]} [${held ? "holding a patient link" : "no patient link"}]`;
  }
  return steps[action];
};

export const group = (v) => {
  const shows = tabOf(v).shows;
  if (shows === "Closed") return "Closed";
  if (["Opening your workspace", "Your workspace", "Sign-in prompt, with no patient content"].includes(shows))
    return "Workspace";
  return "Signing in";
};

export const colors = { Closed: "tan", "Signing in": "blue", Workspace: "teal" };

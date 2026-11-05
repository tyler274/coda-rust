------------------------------- MODULE Charts -------------------------------
(***************************************************************************)
(* Where each chart of DuetSignIn.tla starts, and the steps it draws. A   *)
(* chart runs the same Next from a chosen moment, with only the steps of  *)
(* its scene; each chart's view says what it leaves out.                  *)
(***************************************************************************)
EXTENDS DuetSignIn

\* You're signed in, in the conversation, with entries Duet hasn't confirmed and `draft`'s unsent draft.
SignedInWith(draft) ==
  /\ session = [s \in Sessions |-> IF s = SessionOrder[1] THEN Live("Yours") ELSE Unused]
  /\ accepted = [a \in Accounts |-> a = "Yours"]
  /\ phone = [screen |-> Conversation, place |-> Conversation, holds |-> SessionOrder[1], whose |-> "Yours",
              heard |-> TRUE, draft |-> draft, entries |-> "Yours", asks |-> NoAsk, sync |-> NoSync,
              signIn |-> NoLink, told |-> FALSE, owes |-> {}]
  /\ removed = FALSE
  /\ refused = {}
  /\ event = "start"
SignedInWithWork == SignedInWith("Yours")

\* One person signs in with nothing unsent.
NothingUnsentSteps == event' \notin {"you write a draft", "you add entries"}

\* You sign out of this phone with entries Duet hasn't confirmed, and maybe a draft you write first,
\* until Duet closes.
SignOutSteps ==
  event' \in {"you write a draft", "you sign out", "Duet confirms the entries", "Duet can't be reached",
              "you stay signed in", "you sign out anyway", "Duet closes"}
SignOutSpec == SignedInWith(Nobody) /\ [][Next]_vars

\* Duet ends your session elsewhere while the phone may not reach Duet.
EndedSteps ==
  \/ event' \in {"Duet closes", "Duet answers the check", "Duet can't be reached", "the phone checks with Duet",
                 "Duet refuses the session", "Duet's notice reaches the app", "Duet ends " \o SessionOrder[1]}
  \/ event' = "you open Duet" /\ phone.holds # NoSession
EndedSpec == SignedInWithWork /\ [][Next]_vars

\* You sign out, and another person signs in on the same phone.
PassedOnSteps ==
  event' \in {"you enter your email", "you open the link for Theirs", "Duet signs you in", "you agree to the Terms",
              "Duet records the Terms", "Duet confirms the entries", "Duet can't be reached", "you sign out",
              "you stay signed in", "you sign out anyway", "the phone tells Duet it signed out",
              "the phone tells Duet it signed out, but the answer is lost"}
PassedOnSpec == SignedInWithWork /\ [][Next]_vars

\* You signed in before, accepted the Terms and signed out, and Duet heard it; Duet is closed.
SignedOutReturning ==
  /\ session = [s \in Sessions |-> IF s = SessionOrder[1] THEN Ended("Yours") ELSE Unused]
  /\ accepted = [a \in Accounts |-> a = "Yours"]
  /\ phone = [screen |-> Closed, place |-> Conversation, holds |-> NoSession, whose |-> Nobody,
              heard |-> FALSE, draft |-> Nobody, entries |-> Nobody, asks |-> NoAsk, sync |-> NoSync,
              signIn |-> NoLink, told |-> FALSE, owes |-> {}]
  /\ removed = FALSE
  /\ refused = {}
  /\ event = "start"

\* You sign back in.
ReturningSteps ==
  event' \in {"you open Duet", "Duet closes", "you enter your email", "you open the link for Yours",
              "Duet signs you in", "Duet signs you in, but the answer is lost", "the link no longer works",
              "Duet can't be reached", "the phone checks with Duet", "Duet answers the check"}
ReturningSpec == SignedOutReturning /\ [][Next]_vars
=============================================================================

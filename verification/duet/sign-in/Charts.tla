------------------------------- MODULE Charts -------------------------------
(***************************************************************************)
(* Where each chart of DuetSignIn.tla starts, and the steps it draws. A   *)
(* chart runs the same Next from a chosen moment, with only the steps of  *)
(* its scene; each chart's view says what it leaves out.                  *)
(***************************************************************************)
EXTENDS DuetSignIn

\* You're signed in, in the conversation, with a draft and entries Duet hasn't confirmed.
SignedInWithWork ==
  /\ session = [s \in Sessions |-> IF s = SessionOrder[1] THEN Live("Yours") ELSE Unused]
  /\ accepted = [a \in Accounts |-> a = "Yours"]
  /\ phone = [screen |-> Conversation, place |-> Conversation, holds |-> SessionOrder[1], whose |-> "Yours",
              heard |-> TRUE, draft |-> "Yours", entries |-> "Yours", asks |-> NoAsk, sync |-> NoSync,
              signIn |-> NoLink, told |-> FALSE, owes |-> {}]
  /\ removed = FALSE
  /\ refused = {}
  /\ event = "start"

\* Duet ends your session elsewhere while the phone may not reach Duet.
EndedSteps ==
  \/ event' \in {"Duet closes", "Duet answers the check", "Duet can't be reached", "the phone checks with Duet",
                 "Duet refuses the session", "Duet's notice reaches the app", "Duet ends " \o SessionOrder[1]}
  \/ event' = "you open Duet" /\ phone.holds # NoSession
EndedSpec == SignedInWithWork /\ [][Next]_vars

\* You sign out, and another person signs in on the same phone.
PassedOnSteps ==
  event' \in {"you enter your email", "you open the link for Theirs", "Duet signs you in", "you agree to the Terms",
              "Duet records the Terms", "the phone sends its entries", "Duet takes the entries", "Duet can't be reached", "you sign out",
              "you stay signed in", "you sign out anyway", "the phone tells Duet it signed out",
              "the phone tells Duet it signed out, but the answer is lost"}
PassedOnSpec == SignedInWithWork /\ [][Next]_vars
=============================================================================

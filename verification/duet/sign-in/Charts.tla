------------------------------- MODULE Charts -------------------------------
(***************************************************************************)
(* Where each chart of DuetSignIn.tla starts, and the steps it draws. A   *)
(* chart runs the same Next from a chosen moment, with only the steps of  *)
(* its scene; the README says what each chart leaves out.                 *)
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
  /\ event = [name |-> "start", flow |-> "start"]

\* Duet ends your session elsewhere while the phone may not reach Duet.
EndedSteps == event'.flow \in {"F10", "F12", "F14", "F15", "F16", "F17"}
EndedSpec == SignedInWithWork /\ [][Next]_vars

\* You sign out, and another person signs in on the same phone.
PassedOnSteps ==
  /\ event'.flow \in {"F1", "F2", "F6", "F7", "F13", "F18", "F19", "F20", "F21", "F22"}
  /\ event'.name \notin {"you write a draft", "you add entries"}
  /\ event'.name # "you open the link for Yours"
PassedOnSpec == SignedInWithWork /\ [][Next]_vars
=============================================================================

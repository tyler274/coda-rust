------------------------------- MODULE Charts -------------------------------
(***************************************************************************)
(* Where each chart of SignIn.tla starts, and the steps it draws. A chart *)
(* runs the same Next from a chosen moment, with only the steps of its    *)
(* scene; each chart's view says what it leaves out.                      *)
(***************************************************************************)
EXTENDS SignIn

\* A tab at your workspace at Today, holding the browser's first session.
AtWork(s) == [shows |-> Work, presents |-> s, whose |-> OwnerTab("yours"), read |-> NoRead,
              signIn |-> NoSignIn, link |-> NoLink, opens |-> Today, news |-> NoNews, seen |-> Saw,
              before |-> NothingOld, owes |-> OwesNothing, linkFor |-> NoLinkPage]
\* A tab at your workspace at the patient it opened from a link.
AtPatient(s) == [AtWork(s) EXCEPT !.opens = Patient]
ClosedTab == [AtWork(NoSession) EXCEPT !.shows = Closed, !.whose = Nobodys]
SignedInAs(t0) ==
  /\ session = [s \in Sessions |-> IF s = SessionOrder[1] THEN YoursLive ELSE Unused]
  /\ stores = SessionOrder[1]
  /\ access = Granted
  /\ tab = t0
  /\ actedForAnother = FALSE
  /\ event = [name |-> "start", tab |-> ""]

\* Tab A at your workspace, tab B closed; another physician signs in from tab B.
AnotherInit == SignedInAs([t \in Tabs |-> IF t = "A" THEN AtWork(SessionOrder[1]) ELSE ClosedTab])
AnotherSteps ==
  /\ event'.name \in
       {"open another physician's sign-in link", "Coda checks the link", "Coda signs in the link",
        "Coda answers who signed in", "Coda answers the read", "the workspace reads", "this tab hears the news",
        "act in the workspace"}
  /\ event'.name = "open another physician's sign-in link" => event'.tab = "B" /\ tab["B"].shows = Closed
AnotherSpec == AnotherInit /\ [][Next]_vars

\* Tab A at a patient and tab B at Today when the session expires; you sign back in from tab B.
ExpiryInit == SignedInAs([t \in Tabs |-> IF t = "A" THEN AtPatient(SessionOrder[1]) ELSE AtWork(SessionOrder[1])])
ExpirySteps ==
  /\ event'.name \in
       {SessionOrder[1] \o " expires", "the workspace reads", "Coda answers the read", "open your sign-in link",
        "Coda checks the link", "Coda signs in the link", "Coda answers who signed in", "this tab hears the news"}
  /\ event'.name = "open your sign-in link" =>
       event'.tab = "B" /\ session[SessionOrder[1]] = YoursExpired /\ session[SessionOrder[2]] = Unused
ExpirySpec == ExpiryInit /\ [][Next]_vars

\* Both tabs at your workspace; you sign out in tab B, and tab A follows.
SignOutInit == SignedInAs([t \in Tabs |-> AtWork(SessionOrder[1])])
SignOutSteps ==
  /\ event'.name \in
       {"sign out", "the workspace reads", "Coda answers the read", "this tab hears the news",
        "act in the workspace"}
  /\ event'.name = "sign out" => event'.tab = "B"
SignOutSpec == SignOutInit /\ [][Next]_vars

\* Tab A, opened on a patient link while signed out, waits on Check your email while you open your
\* link in tab B; then you open your link again, signed in.
WaitingTab == [ClosedTab EXCEPT !.shows = Check, !.link = Holding, !.owes = OwesPatient]
SignInInit ==
  /\ session = [s \in Sessions |-> Unused]
  /\ stores = NoSession
  /\ access = Granted
  /\ tab = [t \in Tabs |-> IF t = "A" THEN WaitingTab ELSE ClosedTab]
  /\ actedForAnother = FALSE
  /\ event = [name |-> "start", tab |-> ""]
SignInSteps ==
  /\ event'.name \in
       {"open your sign-in link", "Coda checks the link", "Coda signs in the link", "Coda answers who signed in",
        "this tab hears the news", "the workspace reads", "Coda answers the read"}
  /\ event'.name = "open your sign-in link" =>
       event'.tab = "B" /\ (tab["B"].shows = Closed \/ (stores # NoSession /\ tab["B"].shows = Work))
SignInSpec == SignInInit /\ [][Next]_vars
=============================================================================

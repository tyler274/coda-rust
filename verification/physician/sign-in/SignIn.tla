------------------------------- MODULE SignIn -------------------------------
(***************************************************************************)
(* How a physician signs in to Coda Agents: the design drawn from         *)
(* README.md and intent/physician.md A1. Each INVARIANT is one agreed      *)
(* sentence. The code carries the design out; each action's comment says  *)
(* where.                                                                  *)
(*                                                                         *)
(* The session belongs to the browser: the session it stores, and its     *)
(* owner beside it, decide whether you are signed in, and the browser     *)
(* counts its sign-outs. Two tabs run the same design, so whatever one    *)
(* tab can do, the other can do too, in any order. A tab follows the      *)
(* browser a moment late: news from another tab only tells it to catch    *)
(* up, and it then takes whatever the browser stores.                     *)
(*                                                                         *)
(* Each wait in the code is a step of its own: Coda answers reads, link   *)
(* checks and sign-ins whenever it does, and news reaches a tab whenever  *)
(* it does. Signing out is one step for the browser: the tab forgets the  *)
(* stored session, counts the sign-out, tells the other tabs and asks     *)
(* Coda to end the session without waiting; Coda ending it on arrival is  *)
(* taken as at once, since every tab still holding it has news on its     *)
(* way. A screen that goes away drops what it was still waiting on; a     *)
(* page load drops everything. When a session ends, Coda ends the event   *)
(* stream of a workspace that holds it, and the workspace reads again.    *)
(* The browser holds up to three sessions in a run; asking for a link     *)
(* while Coda answers is one step.                                         *)
(*                                                                         *)
(* `event` names the step just taken, for traces; VIEW leaves it out of   *)
(* the state.                                                              *)
(***************************************************************************)
EXTENDS Naturals, Sequences

Tabs == {"A", "B"}
Other(t) == IF t = "A" THEN "B" ELSE "A"
SessionOrder == <<"Session 1", "Session 2", "Session 3">>
Sessions == {"Session 1", "Session 2", "Session 3"}
NoSession == "No session"

\* A session, as Coda knows it.
Unused == "Not signed in yet"
YoursLive == "Yours, live"
YoursExpired == "Yours, expired"
YoursOut == "Yours, signed out"
TheirsLive == "Another physician's, live"
TheirsExpired == "Another physician's, expired"
TheirsOut == "Another physician's, signed out"
Yours == {YoursLive, YoursExpired, YoursOut}
Live == {YoursLive, TheirsLive}
Ended(s) == IF s \in Yours THEN YoursOut ELSE TheirsOut
Granted == "Granted"
Removed == "Removed by the practice"

\* What a tab shows.
Closed == "Closed"
Opening == "Opening your workspace"
Email == "Enter your work email"
Check == "Check your email"
Signing == "Signing you in"
Used == "This link no longer works"
NoAccess == "No access to this practice"
Unreached == "Coda couldn't reach sign-in"
Work == "Your workspace"
Prompt == "Sign-in prompt over your work"
SignInPages == {Email, Check, Signing, Used, NoAccess, Unreached}
Shell == {Opening, Work, Prompt}

\* Whose workspace a tab is, and what it waits on.
Nobodys == "Nobody's"
OwnerTab(o) == IF o = "yours" THEN "Yours" ELSE "Another physician's"
NoRead == "No read out"
ReadNone == "Reading with no session"
Reading(s) == "Reading with " \o s
NoSignIn == "No sign-in out"
Checking(s) == "Checking the link with " \o s
Verifying(o) == IF o = "yours" THEN "Verifying your link" ELSE "Verifying another physician's link"
Asking(o) == IF o = "yours" THEN "Asking Coda who signed in: you" ELSE "Asking Coda who signed in: another physician"
NoLink == "No patient link"
Holding == "Holding a patient link"
Today == "Today"
Patient == "The linked patient"
NoNews == "No news"
News == "The browser changed its session"
Saw == "Every sign-out"
Missed == "Not the latest sign-out"
NothingOld == "Nothing from before a sign-out"
OldWork == "Your work from before a sign-out"
\* What a tab owes you: the patient whose link you opened there while signed out; a tab whose
\* session Coda refuses, holding a link with no sign-out since, owes it too. Landing doesn't let
\* it off, so the rule can see where it landed; only signing out, losing access, another physician
\* taking the tab or loading another page there does. The design never reads it.
OwesNothing == "Nothing"
OwesPatient == "The patient you opened while signed out"

VARIABLES session, stores, access, tab, event
state == <<session, stores, access, tab>>
vars == <<session, stores, access, tab, event>>

Owner(s) == IF session[s] \in Yours THEN "yours" ELSE "theirs"
\* The first session not yet signed in, which Coda gives the next sign-in.
Index(s) == CHOOSE i \in 1..3 : SessionOrder[i] = s
IsNext(s) == session[s] = Unused /\ \A j \in 1..(Index(s) - 1) : session[SessionOrder[j]] # Unused

Init ==
  /\ session = [s \in Sessions |-> Unused]
  /\ stores = NoSession
  /\ access = Granted
  /\ tab = [t \in Tabs |-> [shows |-> Closed, presents |-> NoSession, whose |-> Nobodys,
             read |-> NoRead, signIn |-> NoSignIn, link |-> NoLink, opens |-> Today,
             news |-> NoNews, seen |-> Saw, before |-> NothingOld, owes |-> OwesNothing]]
  /\ event = [name |-> "start", tab |-> ""]

Did(name, t) == event' = [name |-> name, tab |-> t]

\* A tab forgets where it was: the patient link it held and the place it opens.
ForgetPlace(r) == [r EXCEPT !.link = NoLink, !.opens = Today, !.before = NothingOld]
SignedOutHere(r) == [r EXCEPT !.read = NoRead, !.shows = Email, !.presents = NoSession, !.whose = Nobodys]
\* A fresh page: nothing it waited on survives, and it has read the sign-out count.
FreshPage(r) == [ForgetPlace(r) EXCEPT !.read = NoRead, !.signIn = NoSignIn, !.news = NoNews,
                                       !.seen = Saw, !.owes = OwesNothing]
HoldsAPlace(r) == r.shows \in Shell \/ r.link = Holding
\* When a session stops being live, Coda ends the event stream of a workspace that holds it, and
\* the workspace reads again.
StreamEnds(r, s) == IF r.shows = Work /\ r.presents = s /\ r.read = NoRead
                    THEN [r EXCEPT !.read = Reading(s)] ELSE r

(* The browser signs out, from tab t: it forgets the session it stores, or the tab's own when it  *)
(* stores none, counts the sign-out and tells the other tab; Coda ends that session. Every tab's  *)
(* place from before is now work from before a sign-out. Here(r) is what tab t shows after.      *)
SignsOut(t, Here(_)) ==
  LET ending == IF stores # NoSession THEN stores ELSE tab[t].presents
      ends == ending \in Sessions
      after(u) == LET marked == IF HoldsAPlace(tab[u]) THEN [tab[u] EXCEPT !.before = OldWork] ELSE tab[u]
                  IN IF ends /\ session[ending] \in Live THEN StreamEnds(marked, ending) ELSE marked
  IN /\ tab' = [u \in Tabs |->
          IF u = t THEN [Here(after(u)) EXCEPT !.seen = Saw, !.owes = OwesNothing]
          ELSE [after(u) EXCEPT !.seen = Missed, !.news = News, !.owes = OwesNothing]]
     /\ stores' = NoSession
     /\ session' = IF ends /\ session[ending] # YoursOut /\ session[ending] # TheirsOut
                   THEN [session EXCEPT ![ending] = Ended(@)] ELSE session

(* Catching up: the tab takes what the browser stores. Only a sign-out it missed makes it forget  *)
(* where it was. With nothing stored, a signed-in tab signs out. A signed-out tab, which a tab    *)
(* still opening with no session is too, takes the stored session and keeps the patient link it  *)
(* holds. Your own tab takes it where it was and reads with it; any other tab, or one that missed *)
(* a sign-out, starts a new workspace at Today.                                                   *)
CatchUp(r) ==
  LET missed == r.seen = Missed
      b == [(IF missed THEN ForgetPlace(r) ELSE r) EXCEPT !.news = NoNews, !.seen = Saw]
      s == stores
      took(x) == [x EXCEPT !.presents = s, !.read = Reading(s), !.whose = OwnerTab(Owner(s))]
  IN CASE stores = NoSession /\ r.shows \in Shell -> SignedOutHere(b)
       [] stores = NoSession -> [b EXCEPT !.presents = NoSession]
       [] ~missed /\ r.shows \in Shell /\ r.presents = s -> b
       [] r.shows \in Shell /\ r.whose = Nobodys -> took(b)
       [] r.shows \notin Shell -> took([b EXCEPT !.signIn = NoSignIn, !.shows = Opening])
       [] ~missed /\ r.whose = OwnerTab(Owner(s)) /\ r.shows = Prompt -> took([b EXCEPT !.shows = Work])
       [] ~missed /\ r.whose = OwnerTab(Owner(s)) -> took(b)
       \* A tab that was someone's becomes the new physician's at Today, owing nothing.
       [] OTHER -> took([ForgetPlace(b) EXCEPT !.shows = Opening, !.owes = OwesNothing])

SetTab(t, r) == tab' = [tab EXCEPT ![t] = r]

(* Open Coda, or open a patient link: a new tab, a reload, or Coda reopened after closing it    *)
(* opens from what the browser stores; its first read decides. A patient link is held: the tab  *)
(* opens that patient once someone is signed in.                                                 *)
(* lib.rs PhysicianRoot: the client takes the stored session; PhysicianShell: the first /today   *)
(* read decides; main.rs ?patient=; lib.rs PhysicianRoot held_link.                              *)
Loads(t, held) ==
  LET r == [FreshPage(tab[t]) EXCEPT !.shows = Opening, !.link = held]
  IN /\ SetTab(t, IF stores # NoSession
                  THEN [r EXCEPT !.presents = stores, !.read = Reading(stores), !.whose = OwnerTab(Owner(stores))]
                  ELSE [r EXCEPT !.presents = NoSession, !.read = ReadNone, !.whose = Nobodys,
                                 !.owes = IF held = Holding THEN OwesPatient ELSE OwesNothing])
     /\ UNCHANGED <<session, stores, access>>
OpenCoda(t) == Loads(t, NoLink) /\ Did("open Coda", t)
OpenPatientLink(t) == Loads(t, Holding) /\ Did("open a patient link", t)

(* The emailed link opens a page that checks it against the session the browser stores: the     *)
(* same physician's session stands if Coda still knows it, another physician's is signed out     *)
(* first, and otherwise Coda redeems the link.                                                   *)
(* main.rs sign_in_link loads the page; lib.rs SignInScreen link effect compares the stored      *)
(* owner with the link.                                                                           *)
OpenSignInLink(t, o) ==
  LET page(r) == [FreshPage(r) EXCEPT !.shows = Signing, !.whose = Nobodys]
      verify(r) == [page(r) EXCEPT !.presents = NoSession, !.signIn = Verifying(o)]
  IN /\ CASE stores # NoSession /\ Owner(stores) = o ->
               /\ SetTab(t, [page(tab[t]) EXCEPT !.presents = stores, !.signIn = Checking(stores)])
               /\ UNCHANGED <<session, stores>>
          [] stores # NoSession -> SignsOut(t, verify)
          [] OTHER -> SetTab(t, verify(tab[t])) /\ UNCHANGED <<session, stores>>
     /\ UNCHANGED access
     /\ Did(IF o = "yours" THEN "open your sign-in link" ELSE "open another physician's sign-in link", t)

(* Coda always answers "Check your email", and an address with no account gets no email.        *)
(* lib.rs SignInScreen request_token: Mailed shows Check your email; the prompt mails inside      *)
(* itself.                                                                                         *)
RequestLink(t) ==
  /\ tab[t].shows \in {Email, Check, Used, Unreached, Prompt}
  /\ SetTab(t, IF tab[t].shows = Prompt THEN tab[t] ELSE [tab[t] EXCEPT !.shows = Check])
  /\ UNCHANGED <<session, stores, access>>
  /\ Did("request a sign-in link", t)

\* Back to the email field. lib.rs SignInScreen "Use a different email".
UseAnotherEmail(t) ==
  /\ tab[t].shows \in {Check, Used, NoAccess, Unreached}
  /\ SetTab(t, [tab[t] EXCEPT !.shows = Email])
  /\ UNCHANGED <<session, stores, access>>
  /\ Did("use another email", t)

CheckedWith(r) == CHOOSE s \in Sessions : r.signIn = Checking(s)
IsChecking(r) == \E s \in Sessions : r.signIn = Checking(s)

(* A session the browser still stores and Coda still knows for the same physician stands, and   *)
(* the tab opens the workspace; otherwise Coda redeems the link.                                *)
(* lib.rs SignInScreen link effect: whoami with the held session; on_signed_in while the browser *)
(* still stores it, else verify_link.                                                            *)
CodaChecksLink(t) ==
  /\ IsChecking(tab[t])
  /\ LET s == CheckedWith(tab[t])
         o == Owner(s)
     IN SetTab(t, IF stores = s /\ session[s] \in Live /\ (o = "yours" => access = Granted)
                  THEN [tab[t] EXCEPT !.signIn = NoSignIn, !.shows = Opening, !.read = Reading(s), !.whose = OwnerTab(o)]
                  ELSE [tab[t] EXCEPT !.signIn = Verifying(o)])
  /\ UNCHANGED <<session, stores, access>>
  /\ Did("Coda checks the link", t)

(* Coda starts a new session; the browser stores it with its owner and tells the other tabs.    *)
(* The email never carries a patient.                                                            *)
(* lib.rs verify_link Ok: store_session with the link email, read the sign-out count, announce.  *)
CodaSignsIn(t) ==
  \E o \in {"yours", "theirs"}, s \in Sessions :
    /\ tab[t].signIn = Verifying(o)
    /\ IsNext(s)
    /\ o = "yours" => access = Granted
    /\ session' = [session EXCEPT ![s] = IF o = "yours" THEN YoursLive ELSE TheirsLive]
    /\ stores' = s
    /\ tab' = [tab EXCEPT ![t] = [@ EXCEPT !.presents = s, !.signIn = Asking(o), !.seen = Saw],
                          ![Other(t)] = [@ EXCEPT !.news = News]]
    /\ UNCHANGED access
    /\ Did("Coda signs in the link", t)

\* The tab opens the workspace at Today. lib.rs verify_link: on_signed_in(whoami).
CodaAnswersWho(t) ==
  \E o \in {"yours", "theirs"} :
    /\ tab[t].signIn = Asking(o)
    /\ LET r == [tab[t] EXCEPT !.signIn = NoSignIn, !.shows = Opening, !.whose = OwnerTab(o)]
       IN SetTab(t, IF r.presents \in Sessions THEN [r EXCEPT !.read = Reading(r.presents)]
                    ELSE [r EXCEPT !.read = ReadNone])
    /\ UNCHANGED <<session, stores, access>>
    /\ Did("Coda answers who signed in", t)

\* The link is rejected. lib.rs verify_link 401: SIGN_IN_INVALID.
LinkUsed(t) ==
  /\ tab[t].signIn \in {Verifying("yours"), Verifying("theirs")}
  /\ SetTab(t, [tab[t] EXCEPT !.signIn = NoSignIn, !.shows = Used])
  /\ UNCHANGED <<session, stores, access>>
  /\ Did("the link was used or has expired", t)

\* Shows "No access to this practice". lib.rs verify_link 403: NoAccess.
LinkNoAccess(t) ==
  /\ tab[t].signIn = Verifying("yours")
  /\ access = Removed
  /\ SetTab(t, [tab[t] EXCEPT !.signIn = NoSignIn, !.shows = NoAccess])
  /\ UNCHANGED <<session, stores, access>>
  /\ Did("the link no longer grants access", t)

(* Coda couldn't send the link or confirm it, and says so with a way to try again. A link check  *)
(* Coda can't answer keeps the session the browser still stores, or else redeems the link.       *)
(* lib.rs request_token Err and verify_link failure: Recovery; the link effect on another whoami *)
(* failure: on_signed_in(None).                                                                   *)
CantReach(t) ==
  LET r == tab[t]
  IN /\ \/ r.signIn \in {Verifying("yours"), Verifying("theirs")}
        \/ IsChecking(r)
        \/ r.signIn = NoSignIn /\ r.shows \in {Email, Check, Used, Unreached}
     /\ SetTab(t, CASE r.signIn \in {Verifying("yours"), Verifying("theirs")} ->
                         [r EXCEPT !.signIn = NoSignIn, !.shows = Unreached]
                    [] IsChecking(r) /\ stores = CheckedWith(r) ->
                         [r EXCEPT !.signIn = NoSignIn, !.shows = Opening, !.read = Reading(stores),
                                   !.whose = OwnerTab(Owner(stores))]
                    [] IsChecking(r) -> [r EXCEPT !.signIn = Verifying(Owner(CheckedWith(r)))]
                    [] OTHER -> [r EXCEPT !.shows = Unreached])
     /\ UNCHANGED <<session, stores, access>>
     /\ Did("Coda can't reach sign-in", t)

(* Signs out every tab and forgets where you were.                                               *)
(* lib.rs SessionState::end: forgets the session the browser stores, or the tab's own, counts    *)
(* the sign-out, announces, and asks Coda to end it.                                              *)
SignOut(t) ==
  /\ tab[t].shows \in Shell
  /\ SignsOut(t, LAMBDA r : SignedOutHere(ForgetPlace(r)))
  /\ UNCHANGED access
  /\ Did("sign out", t)

\* The workspace reads with the session the tab holds. lib.rs PhysicianShell reads capture the
\* presented bearer.
WorkspaceReads(t) ==
  /\ tab[t].shows = Work /\ tab[t].read = NoRead /\ tab[t].presents \in Sessions
  /\ SetTab(t, [tab[t] EXCEPT !.read = Reading(tab[t].presents)])
  /\ UNCHANGED <<session, stores, access>>
  /\ Did("the workspace reads", t)

(* A live session reads, and an opening workspace shows, on the patient link it held. A refusal  *)
(* of a session the tab no longer holds changes nothing. An ended session covers a shown         *)
(* workspace with the prompt, and sends an opening one to the sign-in page with its patient      *)
(* link. Removed access signs the tab out and says so.                                           *)
(* lib.rs PhysicianShell /today answer; SessionState::refused(refusal, shown, presented).          *)
CodaAnswersRead(t) ==
  LET r == tab[t]
      done == [r EXCEPT !.read = NoRead]
  IN /\ r.read # NoRead
     /\ IF r.read = ReadNone
        THEN /\ SetTab(t, CASE r.presents # NoSession -> done
                            [] r.link = Holding /\ r.seen = Saw -> [SignedOutHere(r) EXCEPT !.owes = OwesPatient]
                            [] OTHER -> SignedOutHere(r))
             /\ UNCHANGED stores
        ELSE LET s == CHOOSE x \in Sessions : r.read = Reading(x)
                 answers == (session[s] = YoursLive /\ access = Granted) \/ session[s] = TheirsLive
                 forget == stores' = IF stores = s THEN NoSession ELSE stores
             IN CASE answers /\ r.shows = Opening /\ r.link = Holding ->
                       SetTab(t, [done EXCEPT !.shows = Work, !.link = NoLink, !.opens = Patient]) /\ UNCHANGED stores
                  [] answers /\ r.shows = Opening -> SetTab(t, [done EXCEPT !.shows = Work]) /\ UNCHANGED stores
                  [] answers -> SetTab(t, done) /\ UNCHANGED stores
                  [] r.presents # s -> SetTab(t, done) /\ UNCHANGED stores
                  [] session[s] = YoursLive /\ access = Removed ->
                       /\ SetTab(t, [ForgetPlace(done) EXCEPT !.shows = NoAccess, !.presents = NoSession,
                                                             !.whose = Nobodys, !.owes = OwesNothing])
                       /\ forget
                  [] r.shows \in {Work, Prompt} -> SetTab(t, [done EXCEPT !.shows = Prompt]) /\ forget
                  [] r.link = Holding /\ r.seen = Saw -> SetTab(t, [SignedOutHere(r) EXCEPT !.owes = OwesPatient]) /\ forget
                  [] OTHER -> SetTab(t, SignedOutHere(r)) /\ forget
     /\ UNCHANGED <<session, access>>
     /\ Did("Coda answers the read", t)

(* The tab takes what the browser stores: a signed-out tab opens the patient link it holds, your *)
(* own tab picks up where it was, any other tab starts at Today, and a browser that stores        *)
(* nothing signs the tab out.                                                                       *)
(* lib.rs SessionState::catch_up on tab news: reads the stored session, its owner and the         *)
(* sign-outs counted.                                                                               *)
HearsNews(t) ==
  /\ tab[t].news = News /\ tab[t].shows # Closed
  /\ SetTab(t, CatchUp(tab[t]))
  /\ UNCHANGED <<session, stores, access>>
  /\ Did("this tab hears the news", t)

\* The session reaches its end. store.rs session: expires_at; the event stream ends.
Expires(s) ==
  /\ session[s] \in Live
  /\ session' = [session EXCEPT ![s] = IF @ = YoursLive THEN YoursExpired ELSE TheirsExpired]
  /\ tab' = [u \in Tabs |-> StreamEnds(tab[u], s)]
  /\ UNCHANGED <<stores, access>>
  /\ event' = [name |-> s \o " expires", tab |-> ""]

\* Coda refuses your sessions and your links from now on. auth.rs: refusal reason access_removed;
\* the event stream ends.
RemoveAccess ==
  /\ access = Granted
  /\ access' = Removed
  /\ tab' = [u \in Tabs |->
       IF tab[u].shows = Work /\ tab[u].read = NoRead /\ tab[u].presents \in Sessions
          /\ session[tab[u].presents] = YoursLive
       THEN [tab[u] EXCEPT !.read = Reading(tab[u].presents)] ELSE tab[u]]
  /\ UNCHANGED <<session, stores>>
  /\ event' = [name |-> "the practice removes your access", tab |-> ""]

TabStep(t) ==
  \/ OpenCoda(t) \/ OpenPatientLink(t)
  \/ OpenSignInLink(t, "yours") \/ OpenSignInLink(t, "theirs")
  \/ RequestLink(t) \/ UseAnotherEmail(t)
  \/ CodaChecksLink(t) \/ CodaSignsIn(t) \/ CodaAnswersWho(t)
  \/ LinkUsed(t) \/ LinkNoAccess(t) \/ CantReach(t)
  \/ SignOut(t) \/ WorkspaceReads(t) \/ CodaAnswersRead(t) \/ HearsNews(t)

Next == (\E t \in Tabs : TabStep(t)) \/ (\E s \in Sessions : Expires(s)) \/ RemoveAccess

Spec == Init /\ [][Next]_vars

-----------------------------------------------------------------------------
\* A tab has caught up when it waits on nothing and has heard every news.
Settled(t) == tab[t].read = NoRead /\ tab[t].signIn = NoSignIn /\ tab[t].news = NoNews

\* Once you sign out, your workspace is gone from this tab.
OnceOut == \A t \in Tabs :
  (Settled(t) /\ tab[t].shows = Work)
    => (tab[t].before # OldWork
        /\ (tab[t].presents \in Sessions => session[tab[t].presents] \notin {YoursOut, TheirsOut}))

\* When your session expires mid-work, a sign-in prompt covers the screen.
Expiry == \A t \in Tabs :
  (Settled(t) /\ tab[t].shows = Work /\ tab[t].presents \in Sessions)
    => session[tab[t].presents] \notin {YoursExpired, TheirsExpired}

\* While you are signed in, Coda never asks you to sign in or tells you sign-in failed.
SignedIn == \A t \in Tabs :
  (Settled(t) /\ stores \in Sessions
   /\ ((session[stores] = YoursLive /\ access = Granted) \/ session[stores] = TheirsLive))
    => tab[t].shows \notin SignInPages \cup {Prompt}

\* An emailed sign-in link always opens Today.
TodayFirst == \A t \in Tabs : tab[t].shows = Signing => tab[t].opens = Today

\* A patient link opened while signed out lands on that patient once you sign in.
Lands == \A t \in Tabs :
  (Settled(t) /\ tab[t].shows = Work /\ tab[t].owes = OwesPatient) => tab[t].opens = Patient
=============================================================================

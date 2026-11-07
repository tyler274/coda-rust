------------------------------- MODULE SignIn -------------------------------
(***************************************************************************)
(* How a physician signs in to Coda Agents: the design drawn from         *)
(* README.md and intent/physician.md A1. Each rule is one agreed          *)
(* sentence, an INVARIANT or a PROPERTY. The code carries the design out; *)
(* each action's comment says where.                                       *)
(*                                                                         *)
(* The session belongs to the browser: the session it stores, and its     *)
(* owner beside it, decide whether you are signed in, and the browser     *)
(* counts its sign-outs. Every tab runs the same design, so whatever one  *)
(* tab can do, another can do too, in any order. A tab follows the       *)
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
(* Asking for a link while Coda answers is one step, whether Coda sends  *)
(* it or can't. Coda and the browser eventually answer whatever a tab    *)
(* waits on; that fairness is assumed only for CatchesUp. Each request a *)
(* workspace makes carries the session its tab holds, so Coda acts for   *)
(* that session's physician or refuses it.                               *)
(*                                                                         *)
(* Not modeled, since they rest on Coda's records or the page's own       *)
(* contents: an address with no account gets no email, a link signs in   *)
(* once and expires, the email carries no patient, accounts are           *)
(* invite-only, drafts and the open reader survive signing back in,       *)
(* Coda's running work never notices, and drafts and seen state stay with *)
(* each physician. The verify skill names the tests that hold them.       *)
(*                                                                         *)
(* `event` names the step just taken, for traces; VIEW leaves it out of   *)
(* the state.                                                              *)
(***************************************************************************)
EXTENDS Naturals, Sequences, TLC

\* SignIn.cfg has two tabs, so one can lag while the other signs in or out, and three sessions, so
\* a tab can still hold a session two sign-ins old. chart.cfg draws one tab with one session.

\* The config sets the scope: the browser's tabs, and how many sessions Coda starts in a run.
CONSTANTS Tabs, SessionCount
SessionOrder == [i \in 1..SessionCount |-> "Session " \o ToString(i)]
Sessions == {SessionOrder[i] : i \in 1..SessionCount}
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
Prompt == "Sign-in prompt, with no patient content"
\* A sign-in page. "Signing you in" is the link's own page, not one of them.
SignInPages == {Email, Check, Used, NoAccess, Unreached}
Shell == {Opening, Work, Prompt}

\* Whose workspace a tab is, and what it waits on.
Nobodys == "Nobody's"
OwnerTab(o) == IF o = "yours" THEN "Yours" ELSE "Another physician's"
NoRead == "No read out"
ReadNone == "Reading with no session"
Reading(s) == "Reading with " \o s
NoSignIn == "No sign-in out"
Checking(s, o) == IF o = "yours" THEN "Checking the link with " \o s
                  ELSE "Checking the link for another physician with " \o s
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
\* What a tab owes you: the patient whose link you opened there while signed out, which it learns
\* when it opens with nothing stored, or when Coda refuses its session while it holds the link with
\* no sign-out since. Landing doesn't let it off, so the rule can see where it landed; only a
\* sign-out, or that tab losing access, being taken over by another physician or loading another
\* page, does. The design never reads it.
OwesNothing == "Nothing"
OwesPatient == "The patient you opened while signed out"
\* Whose sign-in link opened the tab's page: "yours", "theirs", or none. Loading another page or
\* being signed out there leaves it.
NoLinkPage == "Not a sign-in link's page"

\* actedForAnother: Coda has acted, on a request from a tab, for a physician other than the one whose
\* workspace the tab showed, or other than the one signed in to the browser. The design never reads it.
VARIABLES session, stores, access, tab, actedForAnother, event
state == <<session, stores, access, tab, actedForAnother>>
vars == <<session, stores, access, tab, actedForAnother, event>>

Owner(s) == IF session[s] \in Yours THEN "yours" ELSE "theirs"
\* The first session not yet signed in, which Coda gives the next sign-in.
Index(s) == CHOOSE i \in 1..SessionCount : SessionOrder[i] = s
\* Coda answers a request made with session s: it's live, and for your sessions your access stands.
Answering(s) == (session[s] = YoursLive /\ access = Granted) \/ session[s] = TheirsLive
\* Signed in: the browser stores a session Coda still accepts, one that hasn't expired or been signed
\* out, while your access stands.
SignedInNow == stores \in Sessions /\ Answering(stores)
IsNext(s) == session[s] = Unused /\ \A j \in 1..(Index(s) - 1) : session[SessionOrder[j]] # Unused

Init ==
  /\ session = [s \in Sessions |-> Unused]
  /\ stores = NoSession
  /\ access = Granted
  /\ tab = [t \in Tabs |-> [shows |-> Closed, presents |-> NoSession, whose |-> Nobodys,
             read |-> NoRead, signIn |-> NoSignIn, link |-> NoLink, opens |-> Today,
             news |-> NoNews, seen |-> Saw, before |-> NothingOld, owes |-> OwesNothing,
             linkFor |-> NoLinkPage]]
  /\ actedForAnother = FALSE
  /\ event = [name |-> "start", tab |-> ""]

Did(name, t) == event' = [name |-> name, tab |-> t]

\* A tab forgets where it was: the patient link it held and the place it opens.
ForgetPlace(r) == [r EXCEPT !.link = NoLink, !.opens = Today, !.before = NothingOld]
SignedOutHere(r) == [r EXCEPT !.read = NoRead, !.shows = Email, !.presents = NoSession, !.whose = Nobodys,
                              !.linkFor = NoLinkPage]
\* A fresh page: nothing it waited on survives, and it has read the sign-out count.
FreshPage(r) == [ForgetPlace(r) EXCEPT !.read = NoRead, !.signIn = NoSignIn, !.news = NoNews,
                                       !.seen = Saw, !.owes = OwesNothing, !.linkFor = NoLinkPage]
HoldsAPlace(r) == r.shows \in Shell \/ r.link = Holding
\* When a session stops being live, Coda ends the event stream of a workspace that holds it, and
\* the workspace reads again.
StreamEnds(r, s) == IF r.shows = Work /\ r.presents = s /\ r.read = NoRead
                    THEN [r EXCEPT !.read = Reading(s)] ELSE r

(* The browser signs out, from tab t: it forgets the session it stores, or the tab's own when it  *)
(* stores none, counts the sign-out and tells the other tab; Coda ends that session. Every tab's  *)
(* place from before is now work from before a sign-out. Here(r) is what tab t shows after. When  *)
(* you sign out, or a physician was signed in, no tab owes you its patient any more; a sign-out   *)
(* with nobody signed in leaves what each tab owes.                                               *)
SignsOut(t, Here(_), yours) ==
  LET ending == IF stores # NoSession THEN stores ELSE tab[t].presents
      ends == ending \in Sessions
      off(r) == IF yours \/ SignedInNow THEN [r EXCEPT !.owes = OwesNothing] ELSE r
      after(u) == LET marked == IF HoldsAPlace(tab[u]) THEN [tab[u] EXCEPT !.before = OldWork] ELSE tab[u]
                  IN IF ends /\ session[ending] \in Live THEN StreamEnds(marked, ending) ELSE marked
  IN /\ tab' = [u \in Tabs |->
          IF u = t THEN off([Here(after(u)) EXCEPT !.seen = Saw])
          ELSE off([after(u) EXCEPT !.seen = Missed, !.news = News])]
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
     /\ UNCHANGED <<session, stores, access, actedForAnother>>
OpenCoda(t) == Loads(t, NoLink) /\ Did("open Coda", t)
OpenPatientLink(t) == Loads(t, Holding) /\ Did("open a patient link", t)

(* The emailed link opens a page that checks it with Coda against the session the browser        *)
(* stores, and Coda redeems the link when nothing is stored.                                     *)
(* main.rs sign_in_link loads the page; lib.rs SignInScreen link effect compares the stored      *)
(* owner with the link.                                                                           *)
OpenSignInLink(t, o) ==
  LET page(r) == [FreshPage(r) EXCEPT !.shows = Signing, !.whose = Nobodys, !.linkFor = o]
      verify(r) == [page(r) EXCEPT !.presents = NoSession, !.signIn = Verifying(o)]
  IN /\ SetTab(t, IF stores # NoSession
                  THEN [page(tab[t]) EXCEPT !.presents = stores, !.signIn = Checking(stores, o)]
                  ELSE verify(tab[t]))
     /\ UNCHANGED <<session, stores, access, actedForAnother>>
     /\ Did(IF o = "yours" THEN "open your sign-in link" ELSE "open another physician's sign-in link", t)

(* Coda always answers "Check your email", and an address with no account gets no email.        *)
(* lib.rs SignInScreen request_token: Mailed shows Check your email; the prompt mails inside      *)
(* itself.                                                                                         *)
LinkPages == {Email, Check, Used, Unreached, Prompt}
RequestLink(t) ==
  /\ tab[t].shows \in LinkPages
  /\ SetTab(t, IF tab[t].shows = Prompt THEN tab[t] ELSE [tab[t] EXCEPT !.shows = Check])
  /\ UNCHANGED <<session, stores, access, actedForAnother>>
  /\ Did("request a sign-in link", t)

(* Coda can't send the link you asked for, and says so with a way to try again: a sign-in page   *)
(* shows Coda couldn't reach sign-in, and the prompt says so inside itself.                      *)
(* lib.rs SignInScreen request_token Err: Recovery; the prompt's own retry.                       *)
CantSendLink(t) ==
  /\ tab[t].shows \in LinkPages
  /\ SetTab(t, IF tab[t].shows = Prompt THEN tab[t] ELSE [tab[t] EXCEPT !.shows = Unreached])
  /\ UNCHANGED <<session, stores, access, actedForAnother>>
  /\ Did("Coda can't send the link", t)

\* Back to the email field. lib.rs SignInScreen "Use a different email".
UseAnotherEmail(t) ==
  /\ tab[t].shows \in {Check, Used, NoAccess, Unreached}
  /\ SetTab(t, [tab[t] EXCEPT !.shows = Email])
  /\ UNCHANGED <<session, stores, access, actedForAnother>>
  /\ Did("use another email", t)

CheckedWith(r) == CHOOSE s \in Sessions : \E o \in {"yours", "theirs"} : r.signIn = Checking(s, o)
LinkOf(r) == CHOOSE o \in {"yours", "theirs"} : \E s \in Sessions : r.signIn = Checking(s, o)
IsChecking(r) == \E s \in Sessions, o \in {"yours", "theirs"} : r.signIn = Checking(s, o)
\* The link page, done checking, redeems the link; after signing another physician out, it holds
\* no session.
Redeems(r) == [r EXCEPT !.signIn = Verifying(LinkOf(r))]
SignedOutRedeems(r) == [Redeems(r) EXCEPT !.presents = NoSession]

(* While the browser still stores the session and Coda still knows it, someone is signed in: the *)
(* same physician's session stands and the tab opens the workspace, and another physician is     *)
(* signed out first. Otherwise nobody is signed in, and Coda redeems the link.                   *)
(* lib.rs SignInScreen link effect: whoami with the held session; on_signed_in for the link's    *)
(* physician while the browser still stores it, SessionState::end for another, else verify_link. *)
CodaChecksLink(t) ==
  /\ IsChecking(tab[t])
  /\ LET s == CheckedWith(tab[t])
         o == LinkOf(tab[t])
     IN CASE stores = s /\ Answering(s) /\ Owner(s) = o ->
               /\ SetTab(t, [tab[t] EXCEPT !.signIn = NoSignIn, !.shows = Opening, !.read = Reading(s),
                                           !.whose = OwnerTab(o)])
               /\ UNCHANGED <<session, stores>>
          [] stores = s /\ Answering(s) -> SignsOut(t, SignedOutRedeems, FALSE)
          [] OTHER -> SetTab(t, Redeems(tab[t])) /\ UNCHANGED <<session, stores>>
  /\ UNCHANGED <<access, actedForAnother>>
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
    /\ tab' = [u \in Tabs |-> IF u = t THEN [tab[u] EXCEPT !.presents = s, !.signIn = Asking(o), !.seen = Saw]
                               ELSE [tab[u] EXCEPT !.news = News]]
    /\ UNCHANGED <<access, actedForAnother>>
    /\ Did("Coda signs in the link", t)

\* The tab opens the workspace at Today. lib.rs verify_link: on_signed_in(whoami).
CodaAnswersWho(t) ==
  \E o \in {"yours", "theirs"} :
    /\ tab[t].signIn = Asking(o)
    /\ LET r == [tab[t] EXCEPT !.signIn = NoSignIn, !.shows = Opening, !.whose = OwnerTab(o)]
       IN SetTab(t, IF r.presents \in Sessions THEN [r EXCEPT !.read = Reading(r.presents)]
                    ELSE [r EXCEPT !.read = ReadNone])
    /\ UNCHANGED <<session, stores, access, actedForAnother>>
    /\ Did("Coda answers who signed in", t)

\* The link is rejected. lib.rs verify_link 401: SIGN_IN_INVALID.
LinkUsed(t) ==
  /\ tab[t].signIn \in {Verifying("yours"), Verifying("theirs")}
  /\ SetTab(t, [tab[t] EXCEPT !.signIn = NoSignIn, !.shows = Used])
  /\ UNCHANGED <<session, stores, access, actedForAnother>>
  /\ Did("the link was used or has expired", t)

\* Shows "No access to this practice". lib.rs verify_link 403: NoAccess.
LinkNoAccess(t) ==
  /\ tab[t].signIn = Verifying("yours")
  /\ access = Removed
  /\ SetTab(t, [tab[t] EXCEPT !.signIn = NoSignIn, !.shows = NoAccess])
  /\ UNCHANGED <<session, stores, access, actedForAnother>>
  /\ Did("the link no longer grants access", t)

(* Coda couldn't confirm the link, and says so with a way to try again. When Coda can't answer a *)
(* link check, the session the browser still stores stands for the same physician; for another   *)
(* physician nobody is signed out, and the page says Coda couldn't confirm sign-in. If the        *)
(* browser no longer stores the session checked, the link is redeemed.                            *)
(* lib.rs verify_link failure: Recovery; the link effect on another whoami failure:               *)
(* on_signed_in(None) for the link's physician, Recovery for another.                              *)
CantReach(t) ==
  LET r == tab[t]
  IN /\ r.signIn \in {Verifying("yours"), Verifying("theirs")} \/ IsChecking(r)
     /\ CASE IsChecking(r) /\ stores = CheckedWith(r) /\ Owner(stores) = LinkOf(r) ->
               SetTab(t, [r EXCEPT !.signIn = NoSignIn, !.shows = Opening, !.read = Reading(stores),
                                   !.whose = OwnerTab(Owner(stores))])
          [] IsChecking(r) /\ stores # CheckedWith(r) -> SetTab(t, Redeems(r))
          [] OTHER -> SetTab(t, [r EXCEPT !.signIn = NoSignIn, !.shows = Unreached])
     /\ UNCHANGED <<session, stores, access, actedForAnother>>
     /\ Did("Coda can't reach sign-in", t)

(* Signs out every tab and forgets where you were.                                               *)
(* lib.rs SessionState::end: forgets the session the browser stores, or the tab's own, counts    *)
(* the sign-out, announces, and asks Coda to end it.                                              *)
SignOut(t) ==
  /\ tab[t].shows \in Shell
  /\ SignsOut(t, LAMBDA r : SignedOutHere(ForgetPlace(r)), TRUE)
  /\ UNCHANGED <<access, actedForAnother>>
  /\ Did("sign out", t)

\* The workspace reads with the session the tab holds. lib.rs PhysicianShell reads capture the
\* presented bearer.
WorkspaceReads(t) ==
  /\ tab[t].shows = Work /\ tab[t].read = NoRead /\ tab[t].presents \in Sessions
  /\ SetTab(t, [tab[t] EXCEPT !.read = Reading(tab[t].presents)])
  /\ UNCHANGED <<session, stores, access, actedForAnother>>
  /\ Did("the workspace reads", t)

(* A live session reads, and an opening workspace shows, on the patient link it held. A refusal  *)
(* of a session the tab no longer holds changes nothing. An ended session covers a shown         *)
(* workspace with the prompt, and sends an opening one to the sign-in page with its patient      *)
(* link; the browser keeps storing it, since only signing out forgets the stored session.        *)
(* Removed access signs the tab out and says so.                                                 *)
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
                 answers == Answering(s)
                 forget == stores' = IF stores = s THEN NoSession ELSE stores
             IN CASE answers /\ r.shows = Opening /\ r.link = Holding ->
                       SetTab(t, [done EXCEPT !.shows = Work, !.link = NoLink, !.opens = Patient]) /\ UNCHANGED stores
                  [] answers /\ r.shows = Opening -> SetTab(t, [done EXCEPT !.shows = Work]) /\ UNCHANGED stores
                  [] answers -> SetTab(t, done) /\ UNCHANGED stores
                  [] r.presents # s -> SetTab(t, done) /\ UNCHANGED stores
                  [] session[s] = YoursLive /\ access = Removed ->
                       /\ SetTab(t, [ForgetPlace(done) EXCEPT !.shows = NoAccess, !.presents = NoSession,
                                                             !.whose = Nobodys, !.owes = OwesNothing,
                                                             !.linkFor = NoLinkPage])
                       /\ forget
                  [] r.shows \in {Work, Prompt} ->
                       SetTab(t, [ForgetPlace(done) EXCEPT !.shows = Prompt, !.owes = OwesNothing])
                       /\ UNCHANGED stores
                  [] r.link = Holding /\ r.seen = Saw ->
                       SetTab(t, [SignedOutHere(r) EXCEPT !.owes = OwesPatient]) /\ UNCHANGED stores
                  [] OTHER -> SetTab(t, SignedOutHere(r)) /\ UNCHANGED stores
     /\ UNCHANGED <<session, access, actedForAnother>>
     /\ Did("Coda answers the read", t)

(* The tab takes what the browser stores: a signed-out tab opens the patient link it holds, your *)
(* own tab picks up where it was, any other tab starts at Today, and a browser that stores        *)
(* nothing signs the tab out.                                                                       *)
(* lib.rs SessionState::catch_up on tab news: reads the stored session, its owner and the         *)
(* sign-outs counted.                                                                               *)
HearsNews(t) ==
  /\ tab[t].news = News /\ tab[t].shows # Closed
  /\ SetTab(t, CatchUp(tab[t]))
  /\ UNCHANGED <<session, stores, access, actedForAnother>>
  /\ Did("this tab hears the news", t)

\* The session reaches its end. store.rs session: expires_at; the event stream ends.
Expires(s) ==
  /\ session[s] \in Live
  /\ session' = [session EXCEPT ![s] = IF @ = YoursLive THEN YoursExpired ELSE TheirsExpired]
  /\ tab' = [u \in Tabs |-> StreamEnds(tab[u], s)]
  /\ UNCHANGED <<stores, access, actedForAnother>>
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
  /\ UNCHANGED <<session, stores, actedForAnother>>
  /\ event' = [name |-> "the practice removes your access", tab |-> ""]

TabStep(t) ==
  \/ OpenCoda(t) \/ OpenPatientLink(t)
  \/ OpenSignInLink(t, "yours") \/ OpenSignInLink(t, "theirs")
  \/ RequestLink(t) \/ CantSendLink(t) \/ UseAnotherEmail(t)
  \/ CodaChecksLink(t) \/ CodaSignsIn(t) \/ CodaAnswersWho(t)
  \/ LinkUsed(t) \/ LinkNoAccess(t) \/ CantReach(t)
  \/ SignOut(t) \/ WorkspaceReads(t) \/ CodaAnswersRead(t) \/ HearsNews(t)

(* You act in a workspace: send, save or mark something. The request carries the session the tab *)
(* holds. Coda acts for that session's physician while the session answers and they are the      *)
(* physician signed in to the browser. A request from a tab whose physician isn't the browser's  *)
(* is refused, and the tab catches up. A request whose session has ended is refused, and the      *)
(* workspace reads again, so the answer covers it.                                                *)
ActsInTab(t) ==
  /\ tab[t].shows = Work /\ tab[t].presents \in Sessions
  /\ LET s == tab[t].presents
         answers == Answering(s)
         current == SignedInNow /\ Owner(stores) = Owner(s)
     IN CASE answers /\ current ->
               /\ actedForAnother' = (actedForAnother \/ OwnerTab(Owner(s)) # tab[t].whose
                                     \/ ~SignedInNow \/ Owner(stores) # Owner(s))
               /\ UNCHANGED tab
          [] answers -> SetTab(t, CatchUp(tab[t])) /\ UNCHANGED actedForAnother
          [] OTHER ->
               /\ SetTab(t, IF tab[t].read = NoRead THEN [tab[t] EXCEPT !.read = Reading(s)] ELSE tab[t])
               /\ UNCHANGED actedForAnother
  /\ UNCHANGED <<session, stores, access>>
  /\ Did("act in the workspace", t)

Next == (\E t \in Tabs : TabStep(t) \/ ActsInTab(t)) \/ (\E s \in Sessions : Expires(s)) \/ RemoveAccess

Spec == Init /\ [][Next]_vars

\* Coda's answer to a tab's sign-in link, in any of its outcomes.
AnswersLink(t) == CodaChecksLink(t) \/ CodaSignsIn(t) \/ CodaAnswersWho(t) \/ LinkUsed(t)
                  \/ LinkNoAccess(t) \/ CantReach(t)
\* Acting: opening Coda, a patient link or a sign-in link, asking for a link, using a different email,
\* signing out, or working in your workspace.
YouAct == \E t \in Tabs :
            \/ OpenCoda(t) \/ OpenPatientLink(t) \/ OpenSignInLink(t, "yours") \/ OpenSignInLink(t, "theirs")
            \/ RequestLink(t) \/ CantSendLink(t) \/ UseAnotherEmail(t) \/ SignOut(t) \/ ActsInTab(t)
\* Coda and the browser eventually answer whatever a tab waits on: its link, its read, and the news.
FairSpec == Spec /\ \A t \in Tabs : WF_vars(AnswersLink(t)) /\ WF_vars(CodaAnswersRead(t)) /\ WF_vars(HearsNews(t))

-----------------------------------------------------------------------------
\* Caught up: a tab has heard every sign-in and sign-out from the other tabs, and waits on no answer
\* from Coda.
Settled(t) == tab[t].read = NoRead /\ tab[t].signIn = NoSignIn /\ tab[t].news = NoNews

\* After a sign-out, a caught-up tab never shows the workspace from before it, or any workspace on the
\* session signed out.
OnceOut == \A t \in Tabs :
  (Settled(t) /\ tab[t].shows = Work)
    => (tab[t].before # OldWork
        /\ (tab[t].presents \in Sessions => session[tab[t].presents] \notin {YoursOut, TheirsOut}))

\* A caught-up tab never shows your workspace on an expired session.
Expiry == \A t \in Tabs :
  (Settled(t) /\ tab[t].shows = Work /\ tab[t].presents \in Sessions)
    => session[tab[t].presents] \notin {YoursExpired, TheirsExpired}

\* Your work, shown or under the prompt, leaves for a sign-in page only when you sign out in that tab,
\* after a sign-out the tab missed, or when your access is removed.
KeepsWork ==
  [][\A t \in Tabs :
       (tab[t].shows \in {Work, Prompt} /\ tab'[t].shows \in SignInPages)
         => \/ event'.tab = t /\ event'.name = "sign out"
            \/ tab[t].seen = Missed
            \/ access' = Removed]_vars

\* While you're signed in, a caught-up tab never shows a sign-in page or the sign-in prompt, except on
\* a page another physician's sign-in link opened.
SignedIn == \A t \in Tabs :
  (Settled(t) /\ SignedInNow)
    => /\ tab[t].shows # Prompt
       /\ tab[t].shows \in SignInPages => tab[t].linkFor \notin {NoLinkPage, Owner(stores)}

\* A workspace opened from a sign-in link's page shows Today.
TodayFirst == \A t \in Tabs : (tab[t].shows = Work /\ tab[t].linkFor # NoLinkPage) => tab[t].opens = Today

\* A patient link opened while you're signed out lands on that patient once you sign in, unless before
\* then comes a sign-out, or that tab loses access, is taken over by another physician, or loads
\* another page.
Lands == \A t \in Tabs :
  (Settled(t) /\ tab[t].shows = Work /\ tab[t].owes = OwesPatient) => tab[t].opens = Patient

\* Coda acts only for the physician a tab shows, and only while that physician is signed in to the
\* browser, whether or not the tab has caught up.
ActsAsShown == ~actedForAnother

\* The sign-in prompt replaces the workspace, so it holds no patient page and no patient link.
PromptIsEmpty == \A t \in Tabs :
  tab[t].shows = Prompt =>
    tab[t].opens = Today /\ tab[t].link = NoLink /\ tab[t].owes = OwesNothing

\* Signing out ends that session on the server. A later answer for it is a refusal.
ServerRefusesSignedOut ==
  [][event'.name = "sign out" /\ stores \in Sessions
       => session'[stores] \in {YoursOut, TheirsOut}]_vars

\* Once you stop acting, every open tab catches up.
CatchesUp ==
  (<>[][~YouAct]_vars)
    => \A t \in Tabs : (tab[t].shows # Closed /\ ~Settled(t)) ~> (Settled(t) \/ tab[t].shows = Closed)
=============================================================================

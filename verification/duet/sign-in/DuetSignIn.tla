----------------------------- MODULE DuetSignIn -----------------------------
(***************************************************************************)
(* How a person signs in to Duet on iPhone or Android, stays signed in and *)
(* signs out: the design drawn from README.md and intent/duet.md D1 and    *)
(* D5.4. Each INVARIANT is one agreed sentence.                            *)
(*                                                                         *)
(* One phone and two accounts, so the phone can pass from one person to    *)
(* another. Duet owns the sessions and whether each account has accepted  *)
(* the Terms; the phone keeps its session, where it was, its draft and     *)
(* unsent entries across a restart, and knows only what Duet last         *)
(* answered. When Duet ends a session, the phone finds out when Duet       *)
(* refuses its next check or request; nothing depends on Duet's notice to  *)
(* the open app. Another phone signing out everywhere, or Duet revoking    *)
(* the session, is the one step "Duet ends the session".                   *)
(*                                                                         *)
(* Each wait is a step of its own: Duet answers whenever it does, a        *)
(* request can fail before Duet acts on it, Duet can act and its answer    *)
(* never reach the phone, and the app can close at any moment. Signing    *)
(* out of this phone is one step for the phone: it forgets the session at *)
(* once and owes Duet the sign-out, which it tells Duet when it can. Duet *)
(* starts up to three sessions in a run, and the phone keeps one draft and *)
(* one batch of entries at a time.                                         *)
(*                                                                         *)
(* `event` names the step just taken, for traces; VIEW leaves it out of   *)
(* the state.                                                              *)
(***************************************************************************)
EXTENDS Naturals, Sequences

Accounts == {"Yours", "Theirs"}
SessionOrder == <<"Session 1", "Session 2", "Session 3">>
Sessions == {"Session 1", "Session 2", "Session 3"}
NoSession == "No session"
Nobody == "Nobody"

\* A session, as Duet knows it.
Unused == "Not signed in yet"
Live(a) == a \o ", live"
Ended(a) == a \o ", ended"

\* What the phone shows.
Closed == "Duet closed"
Email == "Enter your email"
CheckEmail == "Check your email"
Signing == "Signing you in"
LinkDead == "This link no longer works"
Unreached == "Duet couldn't be reached"
Unsaved == "You were signed out; what's unsent here couldn't be saved"
Terms == "Terms"
Conversation == "Conversation"
Saving == "Signing out: sending unsent entries"
Warning == "Unsent work would be lost"
SignInScreens == {Email, CheckEmail, Signing, LinkDead, Unreached, Unsaved}
\* Screens that show the account's health content.
Content == {Conversation, Saving, Warning}

\* What the phone waits on.
NoAsk == "Nothing asked"
Checking(s) == "Checking " \o s
Accepting(s) == "Accepting the Terms with " \o s
NoSync == "No entries going"
Sending(s) == "Sending entries with " \o s
NoLink == "No link opening"
Redeeming(a) == "Redeeming a link for " \o a

VARIABLES
  session,   \* Duet: each session's account and whether it is live
  accepted,  \* Duet: whether each account has accepted the Terms
  phone,     \* the phone: see Init
  lost,      \* history: unsent work went without the person warned first or told after
  refused,   \* history: the sessions Duet has refused this phone
  event
state == <<session, accepted, phone, lost, refused>>
vars == <<session, accepted, phone, lost, refused, event>>

Of(s) == IF session[s] \in {Live("Yours"), Ended("Yours")} THEN "Yours" ELSE "Theirs"
IsLive(s) == session[s] = Live(Of(s))
Index(s) == CHOOSE i \in 1..3 : SessionOrder[i] = s
\* The first session not yet signed in, which Duet gives the next sign-in.
IsNext(s) == session[s] = Unused /\ \A j \in 1..(Index(s) - 1) : session[SessionOrder[j]] # Unused
AskedWith(r) == CHOOSE s \in Sessions : r.asks \in {Checking(s), Accepting(s)}
SentWith(r) == CHOOSE s \in Sessions : r.sync = Sending(s)
LinkFor(r) == CHOOSE a \in Accounts : r.signIn = Redeeming(a)

Init ==
  /\ session = [s \in Sessions |-> Unused]
  /\ accepted = [a \in Accounts |-> FALSE]
  /\ phone = [screen |-> Closed,       \* what it shows
              place |-> Conversation,  \* where it reopens while it holds a session
              holds |-> NoSession,     \* the session it keeps across restarts
              whose |-> Nobody,        \* whose account it shows
              heard |-> FALSE,         \* it has heard Duet recorded that account's Terms
              draft |-> Nobody,       \* whose unsent message draft it keeps
              entries |-> Nobody,      \* whose unsent notes or medications it keeps
              asks |-> NoAsk,          \* a check or Terms acceptance in flight
              sync |-> NoSync,         \* entries in flight
              signIn |-> NoLink,       \* a sign-in link being redeemed
              told |-> FALSE,          \* it still needs to check: Duet ended its session, or a check failed
              owes |-> {}]             \* sign-outs Duet hasn't heard yet
  /\ lost = FALSE
  /\ refused = {}
  /\ event = "start"

Running == phone.screen # Closed
Unsent(r) == r.draft # Nobody \/ r.entries # Nobody

(* What `lost` records, whatever the step: a draft or entries leave the    *)
(* phone without Duet taking the entries, the person having seen the       *)
(* warning, or the next screen saying what couldn't be saved.              *)
Dropped(k) == phone[k] # Nobody /\ phone'[k] = Nobody
TakenByDuet == phone.sync # NoSync /\ phone'.sync = NoSync /\ IsLive(SentWith(phone))
               /\ \A s \in Sessions : session'[s] = session[s]
Excused == phone.screen = Warning \/ phone'.screen = Unsaved
Silent == (Dropped("draft") /\ ~Excused) \/ (Dropped("entries") /\ ~Excused /\ ~TakenByDuet)

\* Each action ends here, after its other changes: it names the step and keeps the history.
History(name) == /\ event' = name
                 /\ lost' = (lost \/ Silent)
Did(name) == History(name) /\ UNCHANGED refused

(* The phone leaves the account: the session, the draft and entries and   *)
(* everything it waited on go.                                              *)
Leave(r, to) == [r EXCEPT !.screen = to, !.place = Conversation, !.holds = NoSession,
                          !.whose = Nobody, !.heard = FALSE, !.draft = Nobody, !.entries = Nobody,
                          !.asks = NoAsk, !.sync = NoSync, !.told = FALSE]

(* Signing out of this phone takes effect at once, even offline: the phone *)
(* forgets the session and owes Duet the sign-out.                          *)
SignsOutHere(r) == [Leave(r, Email) EXCEPT !.owes = @ \cup {r.holds}]

(* Duet refuses the session: the phone leaves the account, and when it     *)
(* held unsent work, the sign-in screen says what couldn't be saved.       *)
Refused(r) == Leave(r, IF Unsent(r) THEN Unsaved ELSE Email)

(* D1.11, D1.15: Duet opens where the person was, from what the phone      *)
(* keeps, and checks the session. Without one, it asks for an email.       *)
OpenDuet ==
  /\ ~Running
  /\ phone' = IF phone.holds = NoSession
              THEN [phone EXCEPT !.screen = Email]
              ELSE [phone EXCEPT !.screen = phone.place, !.whose = Of(phone.holds),
                                 !.asks = Checking(phone.holds)]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you open Duet")

(* The person closes Duet, or the phone ends it. What it waited on goes;   *)
(* the session, where it was, the draft and entries stay. A sign-out still *)
(* sending entries stops, so Duet reopens in the conversation.              *)
CloseDuet ==
  /\ Running
  /\ phone' = [phone EXCEPT !.screen = Closed, !.asks = NoAsk, !.sync = NoSync, !.signIn = NoLink,
                            !.told = FALSE,
                            !.place = CASE phone.screen = Saving -> Conversation
                                        [] phone.screen \in {Terms, Conversation, Warning} -> phone.screen
                                        [] OTHER -> @]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("Duet closes")

(* D1.2: signing up and signing in are one flow. Duet emails a link for   *)
(* whatever email it's given, which signs in to that email's account or    *)
(* creates it, and the phone says "Check your email".                       *)
EnterEmail ==
  /\ phone.screen \in {Email, CheckEmail, LinkDead, Unreached, Unsaved}
  /\ phone' = [phone EXCEPT !.screen = CheckEmail]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you enter your email")

(* The link, or its code typed in, opens Duet and signs in a phone that    *)
(* holds no session. Opened while signed in, it opens Duet as it was.      *)
OpenLink(a) ==
  /\ phone.holds = NoSession /\ phone.signIn = NoLink
  /\ phone' = [phone EXCEPT !.screen = Signing, !.signIn = Redeeming(a)]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you open the link for " \o a)

(* D1.2-4: Duet starts a session for the link's account. A new account,    *)
(* or one that hasn't accepted the Terms, goes to the Terms first.          *)
DuetSignsIn ==
  \E s \in Sessions :
    /\ phone.signIn # NoLink /\ IsNext(s)
    /\ LET a == LinkFor(phone)
       IN /\ session' = [session EXCEPT ![s] = Live(a)]
          /\ phone' = [phone EXCEPT !.signIn = NoLink, !.holds = s, !.whose = a, !.heard = accepted[a],
                                    !.screen = IF accepted[a] THEN Conversation ELSE Terms]
    /\ UNCHANGED accepted
    /\ Did("Duet signs you in")

\* Duet starts the session, but its answer never reaches the phone, which says Duet couldn't be reached.
SignInUnheard ==
  \E s \in Sessions :
    /\ phone.signIn # NoLink /\ IsNext(s)
    /\ session' = [session EXCEPT ![s] = Live(LinkFor(phone))]
    /\ phone' = [phone EXCEPT !.signIn = NoLink, !.screen = Unreached]
    /\ UNCHANGED accepted
    /\ Did("Duet signs you in, but the answer is lost")

\* A used or expired link signs nobody in.
LinkNoLongerWorks ==
  /\ phone.signIn # NoLink
  /\ phone' = [phone EXCEPT !.signIn = NoLink, !.screen = LinkDead]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("the link no longer works")

(* D1.15: a request doesn't get through, or Duet's answer to a check, a     *)
(* refusal or entries is lost, which the phone can't tell apart. A sign-in *)
(* says Duet couldn't be reached. A check or Terms acceptance leaves the  *)
(* phone as it was, still needing to ask. Entries that don't go leave a   *)
(* sign-out on its warning.                                                 *)
CantReach ==
  LET r == phone
  IN /\ \/ r.signIn # NoLink
        \/ r.screen = CheckEmail
        \/ r.asks # NoAsk
        \/ r.sync # NoSync
     /\ phone' = CASE r.signIn # NoLink -> [r EXCEPT !.signIn = NoLink, !.screen = Unreached]
                   [] r.asks # NoAsk -> [r EXCEPT !.asks = NoAsk, !.told = TRUE]
                   [] r.sync # NoSync ->
                        [r EXCEPT !.sync = NoSync, !.screen = IF @ = Saving THEN Warning ELSE @]
                   [] OTHER -> [r EXCEPT !.screen = Unreached]
     /\ UNCHANGED <<session, accepted>>
     /\ Did("Duet can't be reached")

\* Told that its session ended, or still needing to ask, the open app checks.
PhoneChecks ==
  /\ Running /\ phone.told /\ phone.asks = NoAsk /\ phone.holds # NoSession
  /\ phone' = [phone EXCEPT !.told = FALSE, !.asks = Checking(phone.holds)]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("the phone checks with Duet")

(* D1.14: Duet refuses a check or request made with a session that has    *)
(* ended, and the phone leaves the account.                                 *)
DuetRefuses ==
  \E s \in Sessions :
    /\ (phone.asks \in {Checking(s), Accepting(s)} \/ phone.sync = Sending(s))
    /\ ~IsLive(s)
    /\ phone' = Refused(phone)
    /\ refused' = refused \cup {s}
    /\ UNCHANGED <<session, accepted>>
    /\ History("Duet refuses the session")

(* Duet answers a check on a live session from its records: the Terms      *)
(* until they're accepted, and past them once they are; a sign-out under  *)
(* way stays on screen.                                                     *)
DuetAnswersCheck ==
  /\ \E s \in Sessions : phone.asks = Checking(s) /\ IsLive(s)
  /\ LET r == [phone EXCEPT !.asks = NoAsk, !.heard = accepted[Of(AskedWith(phone))]]
     IN phone' = CASE r.screen \in {Saving, Warning} -> r
                   [] ~accepted[Of(AskedWith(phone))] -> [r EXCEPT !.screen = Terms]
                   [] r.screen = Terms -> [r EXCEPT !.screen = Conversation]
                   [] OTHER -> r
  /\ UNCHANGED <<session, accepted>>
  /\ Did("Duet answers the check")

(* D1.4: one explicit "I agree and continue". Accepting also confirms the  *)
(* person is 18 or over and in the U.S.                                     *)
AgreeTerms ==
  /\ phone.screen = Terms /\ phone.asks = NoAsk
  /\ phone' = [phone EXCEPT !.asks = Accepting(phone.holds)]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you agree to the Terms")

\* Duet records the acceptance on a live session and the phone goes on.
DuetRecordsTerms ==
  /\ \E s \in Sessions : phone.asks = Accepting(s) /\ IsLive(s)
  /\ accepted' = [accepted EXCEPT ![Of(AskedWith(phone))] = TRUE]
  /\ phone' = [phone EXCEPT !.asks = NoAsk, !.heard = TRUE, !.screen = Conversation]
  /\ UNCHANGED session
  /\ Did("Duet records the Terms")

(* Duet records the acceptance, but its answer never reaches the phone,    *)
(* which stays on the Terms and checks again. Agreeing again changes        *)
(* nothing Duet holds.                                                      *)
TermsUnheard ==
  /\ \E s \in Sessions : phone.asks = Accepting(s) /\ IsLive(s)
  /\ accepted' = [accepted EXCEPT ![Of(AskedWith(phone))] = TRUE]
  /\ phone' = [phone EXCEPT !.asks = NoAsk, !.told = TRUE]
  /\ UNCHANGED session
  /\ Did("Duet records the Terms, but the answer is lost")

\* The person starts a message and leaves it unsent.
WriteDraft ==
  /\ phone.screen = Conversation /\ phone.draft = Nobody
  /\ phone' = [phone EXCEPT !.draft = phone.whose]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you write a draft")

\* The person logs notes or medications, which wait on the phone until Duet takes them.
AddEntries ==
  /\ phone.screen = Conversation /\ phone.entries = Nobody
  /\ phone' = [phone EXCEPT !.entries = phone.whose]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you add entries")

\* The phone sends unsent entries.
SendEntries ==
  /\ Running /\ phone.holds # NoSession /\ phone.entries # Nobody /\ phone.sync = NoSync
  /\ phone' = [phone EXCEPT !.sync = Sending(phone.holds)]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("the phone sends its entries")

(* D5.4: signing out of this phone, also from under the Terms. With unsent *)
(* entries, Duet tries to send them first. With anything that would still  *)
(* be lost, counting a draft, Duet warns first. Otherwise it signs out at  *)
(* once.                                                                    *)
StartSignOut ==
  /\ phone.screen \in {Conversation, Terms}
  /\ IF phone.entries # Nobody
     THEN phone' = [phone EXCEPT !.screen = Saving,
                                 !.sync = IF @ = NoSync THEN Sending(phone.holds) ELSE @]
     ELSE phone' = IF phone.draft # Nobody THEN [phone EXCEPT !.screen = Warning]
                   ELSE SignsOutHere(phone)
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you sign out")

(* Duet takes the entries on a live session. Signing out goes on: at once  *)
(* with nothing left, or to the warning for a draft.                        *)
DuetTakesEntries ==
  /\ phone.sync # NoSync /\ IsLive(SentWith(phone))
  /\ LET r == [phone EXCEPT !.sync = NoSync, !.entries = Nobody]
     IN phone' = CASE r.screen = Saving /\ r.draft = Nobody -> SignsOutHere(r)
                   [] r.screen = Saving -> [r EXCEPT !.screen = Warning]
                   [] OTHER -> r
  /\ UNCHANGED <<session, accepted>>
  /\ Did("Duet takes the entries")

StaySignedIn ==
  /\ phone.screen = Warning
  /\ phone' = [phone EXCEPT !.screen = Conversation]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you stay signed in")

\* Warned, the person signs out anyway, and the draft and entries go.
SignOutAnyway ==
  /\ phone.screen = Warning
  /\ phone' = SignsOutHere(phone)
  /\ UNCHANGED <<session, accepted>>
  /\ Did("you sign out anyway")

\* The open phone tells Duet of a sign-out it owes, and Duet ends that session.
TellDuet ==
  \E s \in phone.owes :
    /\ Running
    /\ session' = IF IsLive(s) THEN [session EXCEPT ![s] = Ended(Of(s))] ELSE session
    /\ phone' = [phone EXCEPT !.owes = @ \ {s}]
    /\ UNCHANGED accepted
    /\ Did("the phone tells Duet it signed out")

\* Duet ends the session, but its answer never reaches the phone, which still owes the sign-out.
SignOutUnheard ==
  \E s \in phone.owes :
    /\ Running
    /\ session' = IF IsLive(s) THEN [session EXCEPT ![s] = Ended(Of(s))] ELSE session
    /\ UNCHANGED <<phone, accepted>>
    /\ Did("the phone tells Duet it signed out, but the answer is lost")

(* D1.14: Duet ends a session from elsewhere, by revoking it or because    *)
(* the person signed out everywhere on another phone. The phone doesn't    *)
(* hear of it in the same step.                                             *)
DuetEndsSession ==
  \E s \in Sessions :
    /\ IsLive(s)
    /\ session' = [session EXCEPT ![s] = Ended(Of(s))]
    /\ UNCHANGED <<phone, accepted>>
    /\ Did("Duet ends " \o s)

(* Duet's notice that the session ended reaches the open app, which then  *)
(* checks. Nothing depends on this step happening.                          *)
NoticeArrives ==
  /\ Running /\ phone.holds \in Sessions /\ ~IsLive(phone.holds) /\ ~phone.told
  /\ phone' = [phone EXCEPT !.told = TRUE]
  /\ UNCHANGED <<session, accepted>>
  /\ Did("Duet's notice reaches the app")

Next ==
  \/ OpenDuet \/ CloseDuet \/ EnterEmail \/ \E a \in Accounts : OpenLink(a)
  \/ DuetSignsIn \/ SignInUnheard \/ LinkNoLongerWorks \/ CantReach \/ PhoneChecks
  \/ DuetRefuses \/ DuetAnswersCheck \/ AgreeTerms \/ DuetRecordsTerms \/ TermsUnheard
  \/ WriteDraft \/ AddEntries \/ SendEntries \/ StartSignOut \/ DuetTakesEntries
  \/ StaySignedIn \/ SignOutAnyway \/ TellDuet \/ SignOutUnheard \/ DuetEndsSession
  \/ NoticeArrives

Spec == Init /\ [][Next]_vars

-----------------------------------------------------------------------------
\* A phone shows only the account signed in on it: never another account's screens, draft or entries.
OnlyItsAccount ==
  LET mine == IF phone.holds = NoSession THEN Nobody ELSE Of(phone.holds)
  IN /\ phone.whose \in {Nobody, mine}
     /\ phone.draft \in {Nobody, mine}
     /\ phone.entries \in {Nobody, mine}

\* Once Duet refuses the session, the phone shows none of that account's content.
GoneOnceRefused == phone.screen \in Content => phone.holds \notin refused

\* While the phone holds a live session, Duet never asks the person to sign in, including after a restart.
StaysSignedIn ==
  (phone.holds \in Sessions /\ IsLive(phone.holds)) => phone.screen \notin SignInScreens

\* Duet shows the account only after its Terms are accepted, and the Terms only until the phone hears they're accepted.
TermsFirst ==
  /\ phone.screen \in Content => accepted[phone.whose]
  /\ phone.screen = Terms => ~phone.heard

\* Unsent entries and a draft are never dropped without the person being warned first or told after.
NothingLostSilently == ~lost
=============================================================================

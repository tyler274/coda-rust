----------------------------- MODULE Invitation -----------------------------
(***************************************************************************)
(* How a practice invites a patient to connect through Duet, and how the   *)
(* patient opens the invitation, verifies their date of birth and joins    *)
(* with Share: the design drawn from README.md, which                      *)
(* intent/connected-care.md C1.1 links. Each INVARIANT or PROPERTY quotes  *)
(* one agreed sentence.                                                    *)
(*                                                                         *)
(* One patient, Practices practices, and Accounts Duet accounts. Both      *)
(* Chords and Duet read connection state from the single owner (`conn`).   *)
(* Each practice holds at most one open invitation for the patient at a    *)
(* time (`openCount[p]`). Up to MaxWrongDob (5) wrong date-of-birth        *)
(* answers lock an open invitation (`locked = TRUE`); `Resend` applies     *)
(* only while unlocked, mails a fresh token while keeping prior mailed     *)
(* tokens (`1..mailed`) valid, and `Cancel` closes any open invitation     *)
(* (unlocked or locked) while keeping `hasClosedToken` attemptable so TLC  *)
(* exercises stale links and short codes across cancel, accept, and resend.*)
(*                                                                         *)
(* Actions model the real owners:                                          *)
(* - Invite, Resend, MailFails, Cancel: crates/front-door & physician-app  *)
(* - OpenInvite, OpenClosedInvite, SignInAtInvite, SubmitMatchingDob,      *)
(*   SubmitWrongDob, TapShare, LeavePreview: duet-app, duet-api, front-door*)
(* - EndConnection: patient leave or practice discharge via front-door     *)
(*                                                                         *)
(* Not modeled, since they rest on records rather than step order:         *)
(* email prefill from chart demographics (member may edit or type), any    *)
(* practice member acting (and other practices blocked), no reminder or    *)
(* expiry, existing Duet account lookup, App/Play Store redirect, and zero *)
(* join notifications.                                                     *)
(*                                                                         *)
(* `event` names the step just taken, for traces; VIEW leaves it out.      *)
(***************************************************************************)
EXTENDS Naturals, Sequences, TLC

CONSTANTS Practices, Accounts, MaxInvites, MaxWrongDob

Nobody == "Nobody"

\* Connection states shown in both Chords and Duet.
NotInvited == "Not invited"
Invited == "Invited"
Connected == "Connected"
Ended == "Ended"

\* Invitation slot states per practice.
None == "None"
Open == "Open"
Accepted == "Accepted"
Cancelled == "Cancelled"

\* Duet invitation screens.
Idle == "Idle"
SignInPrompt == "Sign in or sign up by email"
DobCheck == "Date-of-birth check"
LockedScreen == "Invitation locked"
SharePreview == "Practice, physician and sharing preview"
ConnectedNotice == "Connected notice"
NotOpenNotice == "Invitation is no longer open"

VARIABLES
  conn,         \* [p \in Practices |-> [status, account, shared]]
  inv,          \* [p \in Practices |-> current invitation slot + closed-token history]
  openCount,    \* [p \in Practices |-> 0..1 count of open invitations]
  inviteCount,  \* total invitations sent in this run (0..MaxInvites)
  duet,         \* [signedIn, screen, activePractice, activeGen, dobVerified]
  event

state == <<conn, inv, openCount, inviteCount, duet>>
vars == <<conn, inv, openCount, inviteCount, duet, event>>

InitInv == [
  status         |-> None,
  gen            |-> 0,
  locked         |-> FALSE,
  wrongDob       |-> 0,
  mailed         |-> 0,
  mailFailed     |-> FALSE,
  dobPassed      |-> FALSE,
  hasClosedToken |-> FALSE
]

Init ==
  /\ conn = [p \in Practices |-> [status |-> NotInvited, account |-> Nobody, shared |-> FALSE]]
  /\ inv = [p \in Practices |-> InitInv]
  /\ openCount = [p \in Practices |-> 0]
  /\ inviteCount = 0
  /\ duet = [signedIn |-> Nobody, screen |-> Idle, activePractice |-> Nobody, activeGen |-> 0, dobVerified |-> FALSE]
  /\ event = "start"

\* Single-owner connection state read by both Chords and Duet.
ChordsConnectionState(p) == conn[p].status
DuetConnectionState(p) == conn[p].status

\* A practice member invites the patient when Not invited or Ended.
\* Code owner: crates/front-door + crates/physician-app
Invite(p) ==
  /\ inviteCount < MaxInvites
  /\ conn[p].status \in {NotInvited, Ended}
  /\ openCount[p] = 0
  /\ inv' = [inv EXCEPT ![p] = [
                status         |-> Open,
                gen            |-> inviteCount + 1,
                locked         |-> FALSE,
                wrongDob       |-> 0,
                mailed         |-> 1,
                mailFailed     |-> FALSE,
                dobPassed      |-> FALSE,
                hasClosedToken |-> inv[p].hasClosedToken
              ]]
  /\ openCount' = [openCount EXCEPT ![p] = @ + 1]
  /\ conn' = [conn EXCEPT ![p].status = Invited]
  /\ inviteCount' = inviteCount + 1
  /\ UNCHANGED duet
  /\ event' = "a member invites the patient to " \o p

\* Outbound invitation mail fails while the invitation remains open.
\* Code owner: crates/front-door mail delivery
MailFails(p) ==
  /\ inv[p].status = Open
  /\ ~inv[p].mailFailed
  /\ inv' = [inv EXCEPT ![p].mailFailed = TRUE]
  /\ UNCHANGED <<conn, openCount, inviteCount, duet>>
  /\ event' = "the mail can't be sent for " \o p

\* A member resends an open unlocked invitation: mails the same invitation again
\* without creating a second one, closing it, or unlocking a locked one.
\* Code owner: crates/front-door + crates/physician-app
Resend(p) ==
  /\ inv[p].status = Open
  /\ ~inv[p].locked
  /\ inv' = [inv EXCEPT ![p].mailed = IF @ < 2 THEN @ + 1 ELSE @,
                        ![p].mailFailed = FALSE]
  /\ UNCHANGED <<conn, openCount, inviteCount, duet>>
  /\ event' = "a member resends for " \o p

\* A member cancels an open invitation (unlocked or locked).
\* All previously mailed links and short codes stop connecting, and a member may invite again.
\* Code owner: crates/front-door + crates/physician-app
Cancel(p) ==
  /\ inv[p].status = Open
  /\ inv' = [inv EXCEPT ![p] = [
                status         |-> Cancelled,
                gen            |-> inv[p].gen,
                locked         |-> FALSE,
                wrongDob       |-> 0,
                mailed         |-> 0,
                mailFailed     |-> FALSE,
                dobPassed      |-> FALSE,
                hasClosedToken |-> TRUE
              ]]
  /\ openCount' = [openCount EXCEPT ![p] = 0]
  /\ conn' = [conn EXCEPT ![p].status = NotInvited]
  /\ UNCHANGED <<inviteCount, duet>>
  /\ event' = "a member cancels for " \o p

\* Patient opens any mailed link or short code (`tok \in 1..inv[p].mailed`) for an open invitation.
\* Code owner: crates/duet-app (invitation.rs) + crates/duet-api
OpenInvite(p, tok) ==
  /\ inv[p].status = Open
  /\ tok \in 1..inv[p].mailed
  /\ duet.screen \in {Idle, ConnectedNotice, NotOpenNotice, LockedScreen}
  /\ duet' = [duet EXCEPT
                !.activePractice = p,
                !.activeGen = inv[p].gen,
                !.dobVerified = inv[p].dobPassed,
                !.screen = IF duet.signedIn = Nobody THEN SignInPrompt
                           ELSE IF inv[p].locked THEN LockedScreen
                           ELSE IF inv[p].dobPassed THEN SharePreview
                           ELSE DobCheck]
  /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
  /\ event' = "the patient opens link or code " \o ToString(tok) \o " for " \o p

\* Patient opens an old link or short code from a cancelled or already-accepted invitation.
\* Never opens DOB/preview or connects; Duet says so in plain words.
\* Code owner: crates/duet-app + crates/front-door
OpenClosedInvite(p) ==
  /\ inv[p].hasClosedToken
  /\ duet.screen \in {Idle, ConnectedNotice, NotOpenNotice, LockedScreen}
  /\ duet' = [duet EXCEPT !.screen = NotOpenNotice, !.activePractice = Nobody, !.activeGen = 0, !.dobVerified = FALSE]
  /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
  /\ event' = "the patient opens a closed link or code for " \o p

\* Signed-out patient signs in or signs up by email at the invitation.
\* Code owner: crates/duet-app + crates/duet-api sign-in
SignInAtInvite(a) ==
  /\ duet.screen = SignInPrompt
  /\ LET p == duet.activePractice
         g == duet.activeGen
     IN duet' = [duet EXCEPT
                   !.signedIn = a,
                   !.dobVerified = (inv[p].status = Open /\ inv[p].gen = g /\ inv[p].dobPassed),
                   !.screen = IF inv[p].status # Open \/ inv[p].gen # g THEN NotOpenNotice
                              ELSE IF inv[p].locked THEN LockedScreen
                              ELSE IF inv[p].dobPassed THEN SharePreview
                              ELSE DobCheck]
  /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
  /\ event' = "the patient signs in as " \o a

\* Patient enters a matching date of birth; reveals practice, physician, and preview.
\* Shares nothing until Share is tapped.
\* Code owner: crates/duet-app + crates/front-door verify_dob
SubmitMatchingDob ==
  /\ duet.screen = DobCheck
  /\ duet.signedIn # Nobody
  /\ LET p == duet.activePractice
         g == duet.activeGen
     IN IF inv[p].status # Open \/ inv[p].gen # g THEN
          /\ duet' = [duet EXCEPT !.screen = NotOpenNotice, !.activePractice = Nobody, !.activeGen = 0, !.dobVerified = FALSE]
          /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
        ELSE IF inv[p].locked THEN
          /\ duet' = [duet EXCEPT !.screen = LockedScreen, !.dobVerified = FALSE]
          /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
        ELSE
          /\ inv' = [inv EXCEPT ![p].dobPassed = TRUE]
          /\ duet' = [duet EXCEPT !.screen = SharePreview, !.dobVerified = TRUE]
          /\ UNCHANGED <<conn, openCount, inviteCount>>
  /\ event' = "the patient enters a matching date of birth"

\* Patient enters a wrong date of birth; five wrong answers lock the invitation.
\* A locked invitation takes no more date-of-birth answers.
\* Code owner: crates/front-door verify_dob
SubmitWrongDob ==
  /\ duet.screen = DobCheck
  /\ duet.signedIn # Nobody
  /\ LET p == duet.activePractice
         g == duet.activeGen
     IN IF inv[p].status # Open \/ inv[p].gen # g THEN
          /\ duet' = [duet EXCEPT !.screen = NotOpenNotice, !.activePractice = Nobody, !.activeGen = 0, !.dobVerified = FALSE]
          /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
        ELSE IF inv[p].locked THEN
          /\ duet' = [duet EXCEPT !.screen = LockedScreen, !.dobVerified = FALSE]
          /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
        ELSE
          /\ LET nextWrong == inv[p].wrongDob + 1
                 nowLocked == nextWrong >= MaxWrongDob
             IN /\ inv' = [inv EXCEPT ![p].wrongDob = nextWrong,
                                      ![p].locked = nowLocked]
                /\ duet' = [duet EXCEPT !.screen = IF nowLocked THEN LockedScreen ELSE DobCheck]
          /\ UNCHANGED <<conn, openCount, inviteCount>>
  /\ event' = "the patient enters a wrong date of birth"

\* Patient taps Share after passing DOB; connects only if invitation is still open.
\* Code owner: crates/duet-app + crates/duet-api + crates/front-door accept_invitation
TapShare ==
  /\ duet.screen = SharePreview
  /\ duet.signedIn # Nobody
  /\ LET p == duet.activePractice
         g == duet.activeGen
         a == duet.signedIn
     IN IF inv[p].status # Open \/ inv[p].gen # g \/ inv[p].locked \/ ~inv[p].dobPassed THEN
          /\ duet' = [duet EXCEPT
                        !.screen = IF inv[p].status # Open \/ inv[p].gen # g THEN NotOpenNotice ELSE LockedScreen,
                        !.activePractice = Nobody,
                        !.activeGen = 0,
                        !.dobVerified = FALSE]
          /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
        ELSE
          /\ inv' = [inv EXCEPT ![p] = [
                        status         |-> Accepted,
                        gen            |-> g,
                        locked         |-> FALSE,
                        wrongDob       |-> 0,
                        mailed         |-> 0,
                        mailFailed     |-> FALSE,
                        dobPassed      |-> FALSE,
                        hasClosedToken |-> TRUE
                      ]]
          /\ openCount' = [openCount EXCEPT ![p] = 0]
          /\ conn' = [conn EXCEPT ![p] = [status |-> Connected, account |-> a, shared |-> TRUE]]
          /\ duet' = [duet EXCEPT !.screen = ConnectedNotice, !.activePractice = Nobody, !.activeGen = 0, !.dobVerified = FALSE]
          /\ UNCHANGED inviteCount
  /\ event' = "the patient taps Share"

\* Patient leaves without tapping Share: invitation stays open, nothing shared.
\* Code owner: crates/duet-app
LeavePreview ==
  /\ duet.screen \in {SignInPrompt, DobCheck, SharePreview}
  /\ duet' = [duet EXCEPT !.screen = Idle, !.activePractice = Nobody, !.activeGen = 0, !.dobVerified = FALSE]
  /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
  /\ event' = "the patient leaves without tapping Share"

DismissNotice ==
  /\ duet.screen \in {LockedScreen, ConnectedNotice, NotOpenNotice}
  /\ duet' = [duet EXCEPT !.screen = Idle, !.activePractice = Nobody, !.activeGen = 0, !.dobVerified = FALSE]
  /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
  /\ event' = "the patient closes the invitation screen"

SwitchAccount(a) ==
  /\ duet.screen = Idle
  /\ duet.signedIn # a
  /\ duet' = [duet EXCEPT !.signedIn = a]
  /\ UNCHANGED <<conn, inv, openCount, inviteCount>>
  /\ event' = "Duet signs in as " \o a

\* A connected patient leaves or is discharged: connection moves to Ended.
\* Code owner: crates/front-door end_connection
EndConnection(p) ==
  /\ conn[p].status = Connected
  /\ conn' = [conn EXCEPT ![p].status = Ended]
  /\ UNCHANGED <<inv, openCount, inviteCount, duet>>
  /\ event' = "the connection ends for " \o p

Finished ==
  /\ inviteCount = MaxInvites
  /\ \A p \in Practices : conn[p].status \in {NotInvited, Ended}
  /\ duet.screen = Idle
  /\ UNCHANGED vars

Next ==
  \/ (\E p \in Practices : Invite(p) \/ MailFails(p) \/ Resend(p) \/ Cancel(p)
                           \/ OpenClosedInvite(p) \/ EndConnection(p))
  \/ (\E p \in Practices, tok \in {1, 2} : OpenInvite(p, tok))
  \/ (\E a \in Accounts : SignInAtInvite(a) \/ SwitchAccount(a))
  \/ SubmitMatchingDob \/ SubmitWrongDob \/ TapShare \/ LeavePreview \/ DismissNotice
  \/ Finished

Spec == Init /\ [][Next]_vars

-----------------------------------------------------------------------------
\* 1. A patient and a practice have at most one open invitation.
AtMostOneOpen ==
  \A p \in Practices : openCount[p] <= 1 /\ (inv[p].status = Open <=> openCount[p] = 1)

\* 2. A patient connects to a practice only by tapping Share on an invitation that the practice holds open at that moment.
ConnectsOnlyOnOpenShare ==
  [][
    \A p \in Practices :
      (conn[p].status # Connected /\ conn'[p].status = Connected) =>
        (duet.screen = SharePreview /\ duet.activePractice = p /\ inv[p].status = Open
         /\ ~inv[p].locked /\ inv[p].dobPassed /\ inv'[p].status = Accepted)
  ]_vars

\* 3. Only Share grants consent: sending, resending, opening an invitation and passing the date-of-birth check share nothing.
OnlyShareGrantsConsent ==
  [][
    \A p \in Practices :
      (~conn[p].shared /\ conn'[p].shared) =>
        (duet.screen = SharePreview /\ duet.activePractice = p /\ inv[p].status = Open /\ inv'[p].status = Accepted)
  ]_vars

\* 4. A resend applies only to an open unlocked invitation, never makes a second invitation, never closes the open one, and never unlocks a locked invitation, so every link and short code already mailed for it still works.
ResendNeverSplitsOrUnlocks ==
  [][
    \A p \in Practices :
      (inv[p].status = Open /\ inv[p].locked) =>
        (inv'[p].status # Open \/ (inv'[p].locked /\ inv'[p].mailed = inv[p].mailed))
  ]_vars

\* 5. After a member cancels an invitation, whether unlocked or locked, none of its links or short codes connects, and a member may invite the patient again.
CancelledNeverConnects ==
  [][
    \A p \in Practices :
      inv[p].status = Cancelled => (conn'[p].status # Connected /\ inv'[p].status # Accepted)
  ]_vars

\* 6. The patient sees the practice, the physician and what the connection shares only after the date of birth they entered matches.
PreviewOnlyAfterDob ==
  /\ (duet.screen = SharePreview =>
        (duet.activePractice \in Practices /\ duet.activeGen > 0 /\ duet.dobVerified /\ duet.signedIn # Nobody))
  /\ (duet.dobVerified => duet.screen = SharePreview)

\* Transition check for Rule 6: dobVerified can become TRUE only when that open invitation already passed DOB.
DobVerifiedOnlyAfterMatch ==
  [][(~duet.dobVerified /\ duet'.dobVerified) =>
       (duet'.activePractice \in Practices
        /\ inv[duet'.activePractice].status = Open
        /\ inv[duet'.activePractice].gen = duet'.activeGen
        /\ ~inv[duet'.activePractice].locked
        /\ inv'[duet'.activePractice].dobPassed)]_vars

\* 7. Five wrong date-of-birth answers lock an open invitation, and a locked invitation takes no more date-of-birth answers.
LockedAfterFiveWrongDob ==
  /\ \A p \in Practices :
       inv[p].status = Open =>
         ((inv[p].locked <=> inv[p].wrongDob >= MaxWrongDob) /\ inv[p].wrongDob <= MaxWrongDob)
  /\ ~(\E p \in Practices : inv[p].status = Open /\ inv[p].locked /\ inv[p].dobPassed)

\* 8. A mail that can't be sent leaves the invitation open.
FailedMailLeavesOpen ==
  \A p \in Practices :
    inv[p].mailFailed => (inv[p].status = Open /\ conn[p].status = Invited)

\* 9. Share connects the account that is signed in when the patient taps it, and never makes a second account.
ShareConnectsSignedInAccount ==
  /\ \A p \in Practices : conn[p].account \in Accounts \cup {Nobody}

\* 10. An invitation from one practice never changes the patient's connection with another practice.
PracticesStayIndependent ==
  [][
    \A p \in Practices :
      (inv'[p] # inv[p] \/ openCount'[p] # openCount[p]) =>
        (\A q \in Practices \ {p} : conn'[q] = conn[q])
  ]_vars

\* 11. Chords and Duet show the same connection state.
SameConnectionState ==
  \A p \in Practices : ChordsConnectionState(p) = DuetConnectionState(p)

\* Action assertions complementing Rules 7 & 9 across state transitions.
LockedRejectsDobAndShareUsesSignedIn ==
  [][
    /\ (\A p \in Practices :
          (inv[p].status = Open /\ inv[p].locked) =>
            (inv'[p].status # Open \/ (inv'[p].wrongDob = inv[p].wrongDob /\ ~inv'[p].dobPassed)))
    /\ (\A p \in Practices :
          (conn[p].status # Connected /\ conn'[p].status = Connected) =>
            (duet.signedIn \in Accounts /\ conn'[p].account = duet.signedIn))
  ]_vars
=============================================================================

------------------------------- MODULE Shared -------------------------------
(***************************************************************************)
(* How you mark records and write notes for your doctor in Duet, and send  *)
(* them to a practice you're connected to: the design drawn from           *)
(* README.md, which intent/duet.md D4 links. Each INVARIANT quotes one     *)
(* agreed sentence.                                                        *)
(*                                                                         *)
(* One Duet account, Practices practices, Records clinical records and     *)
(* Notes notes. Records begin present at version 1 and may be corrected or *)
(* removed by their source at any time. Marks and notes live in To send    *)
(* until cleared or deleted; a failed save stays visibly unsaved, and an   *)
(* edit from another phone updates the saved text while keeping unsaved    *)
(* local words. Each confirmed send uses one idempotent send slot          *)
(* (`share_key`), shows the non-urgent / irreversible disclosure before    *)
(* confirming, and freezes the chosen items (plus any note's linked        *)
(* record) at their current versions when Duet saves it.                   *)
(*                                                                         *)
(* Actions model the real owners:                                          *)
(* - MarkRecord, UnmarkRecord, WriteNote, EditNote, DeleteNote,            *)
(*   OtherPhoneEditsNote: crates/duet-app (for_my_doctor.rs) + duet-api    *)
(* - SourceCorrectsRecord, SourceRemovesRecord: clinical source ingestion  *)
(* - StartSendFromShared, StartOneRecordSend, Toggle*, PickPractice,       *)
(*   ConfirmSend, DuetSavesSend, SendUncertain, TryAgain*: duet-app/api    *)
(* - PracticePicksUpSend, JoinPractice, EndConnection: crates/front-door   *)
(*                                                                         *)
(* Not modeled, since they rest on records or screen layout: list vs chat  *)
(* layout, no "physician looked" status, no prompts/categories on notes,   *)
(* marked trend reading ranges, hiding lab reference ranges, no PDF/link.  *)
(*                                                                         *)
(* `event` names the step just taken, for traces; VIEW leaves it out.      *)
(***************************************************************************)
EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS Practices, Records, Notes, MaxSends, MaxVersions, Nobody

SendSlots == 1..MaxSends

\* Connection states.
Disconnected == "Disconnected"
Connected == "Connected"
Ended == "Ended"

\* Mark local states.
NotMarked == "NotMarked"
NotSaved == "NotSaved"
Marked == "Marked"

\* Note statuses.
NotWritten == "NotWritten"
Saved == "Saved"

\* Composer phases.
Idle == "Idle"
Choosing == "Choosing"
Sending == "Sending"
Uncertain == "Uncertain"
NotSent == "NotSent"
CantConfirmEnded == "CantConfirmEnded"

\* Send list UI statuses.
Empty == "Empty"
Sent == "Sent"
Received == "Received"

VARIABLES
  conn,       \* [p \in Practices |-> Disconnected | Connected | Ended]
  rec,        \* [r \in Records |-> [version, present, removedNotice]]
  mark,       \* [r \in Records |-> [saved, local]]
  note,       \* [n \in Notes |-> [status, savedVer, localUnsaved, linkedRec]]
  composer,   \* active Choosing / Sending / Uncertain flow on this phone
  sends,      \* [k \in SendSlots |-> send state across Duet and practice]
  nextSend,   \* next available send slot (1..MaxSends+1)
  badStep,    \* history flags catching any broken transition rule
  event

state == <<conn, rec, mark, note, composer, sends, nextSend, badStep>>
vars == <<conn, rec, mark, note, composer, sends, nextSend, badStep, event>>

Sym == Permutations(Practices) \cup Permutations(Records) \cup Permutations(Notes)

InitComposer == [
  phase       |-> Idle,
  practice    |-> Nobody,
  chosenRecs  |-> {},
  chosenNotes |-> {},
  disclosed   |-> FALSE,
  pendingSlot |-> 0
]

EmptySend == [
  duetUi           |-> Empty,
  serverSaved      |-> FALSE,
  practiceReceived |-> 0,
  practicePickedUp |-> FALSE,
  practice         |-> Nobody,
  itemsRecs        |-> [r \in Records |-> 0],
  itemsNotes       |-> {},
  endedAfterSent   |-> FALSE,
  confirmed        |-> FALSE,
  disclosed        |-> FALSE
]

InitBad == [
  unconnectedSend    |-> FALSE,
  sendMutated        |-> FALSE,
  linkedMissing      |-> FALSE,
  confirmClearedBad  |-> FALSE,
  endedBecameRecv    |-> FALSE,
  otherPhoneOverwrote|-> FALSE,
  joinSent           |-> FALSE,
  leaveDeleted       |-> FALSE,
  removeBad          |-> FALSE,
  uncertainCleared   |-> FALSE
]

Init ==
  /\ conn = [p \in Practices |-> Disconnected]
  /\ rec = [r \in Records |-> [version |-> 1, present |-> TRUE, removedNotice |-> FALSE]]
  /\ mark = [r \in Records |-> [saved |-> FALSE, local |-> NotMarked]]
  /\ note = [n \in Notes |-> [status |-> NotWritten, savedVer |-> 0, localUnsaved |-> FALSE, linkedRec |-> Nobody]]
  /\ composer = InitComposer
  /\ sends = [k \in SendSlots |-> EmptySend]
  /\ nextSend = 1
  /\ badStep = InitBad
  /\ event = "start"

ToSendRecs == {r \in Records : mark[r].local # NotMarked}
ToSendNotes == {n \in Notes : note[n].status # NotWritten}
ConnectedPractices == {p \in Practices : conn[p] = Connected}

EffectiveRecs(recs, notes) ==
  {r \in recs : rec[r].present} \cup
  {note[n].linkedRec : n \in {m \in notes : IF note[m].linkedRec # Nobody THEN rec[note[m].linkedRec].present ELSE FALSE}}

FreezeRecs(recs, notes) ==
  [r \in Records |-> IF r \in EffectiveRecs(recs, notes) THEN rec[r].version ELSE 0]

\* Check transition properties across each step and record any violation in badStep'.
Step(name) ==
  /\ event' = name
  /\ badStep' = [
       unconnectedSend |-> badStep.unconnectedSend \/
         (\E k \in SendSlots :
            ((~sends[k].serverSaved /\ sends'[k].serverSaved) \/ (sends[k].practiceReceived = 0 /\ sends'[k].practiceReceived > 0))
            /\ conn[sends'[k].practice] # Connected),
       sendMutated |-> badStep.sendMutated \/
         (\E k \in SendSlots :
            sends[k].serverSaved /\
            (sends'[k].itemsRecs # sends[k].itemsRecs \/ sends'[k].itemsNotes # sends[k].itemsNotes \/ sends'[k].practice # sends[k].practice)),
       linkedMissing |-> badStep.linkedMissing \/
         (\E k \in SendSlots :
            (~sends[k].serverSaved /\ sends'[k].serverSaved) /\
            (\E n \in sends'[k].itemsNotes :
               IF note[n].linkedRec # Nobody
               THEN rec[note[n].linkedRec].present /\ sends'[k].itemsRecs[note[n].linkedRec] # rec[note[n].linkedRec].version
               ELSE FALSE)),
       confirmClearedBad |-> badStep.confirmClearedBad \/
         (\E k \in SendSlots :
            (sends[k].duetUi = Empty /\ sends'[k].duetUi = Sent) /\
            ((\E r \in Records : IF sends'[k].itemsRecs[r] > 0 THEN mark'[r].local # NotMarked ELSE mark'[r] # mark[r])
             \/ (\E n \in Notes : IF n \in sends'[k].itemsNotes THEN note'[n].status # NotWritten ELSE note'[n] # note[n]))),
       endedBecameRecv |-> badStep.endedBecameRecv \/
         (\E k \in SendSlots : sends[k].endedAfterSent /\ sends'[k].duetUi # Sent),
       otherPhoneOverwrote |-> badStep.otherPhoneOverwrote \/
         (name = "you edit the note on another phone" /\
          (\E n \in Notes : note[n].status = NotSaved /\ ~note'[n].localUnsaved)),
       joinSent |-> badStep.joinSent \/
         ((\E p \in Practices : conn[p] # Connected /\ conn'[p] = Connected) /\ (sends' # sends \/ nextSend' # nextSend)),
       leaveDeleted |-> badStep.leaveDeleted \/
         (\E k \in SendSlots : sends[k].duetUi \in {Sent, Received} /\ sends'[k].duetUi \notin {Sent, Received}),
       removeBad |-> badStep.removeBad \/
         (\E r \in Records : (rec[r].present /\ ~rec'[r].present) /\
                             (mark'[r].local # NotMarked \/ (mark[r].local # NotMarked /\ ~rec'[r].removedNotice))),
       uncertainCleared |-> badStep.uncertainCleared \/
         (((composer.phase = Uncertain /\ composer'.phase = CantConfirmEnded) \/ (composer.phase = Sending /\ composer'.phase = NotSent))
          /\ (conn[composer.practice] # Ended \/ mark' # mark \/ note' # note))
     ]

\* --- Practice connections ---

JoinPractice(p) ==
  /\ conn[p] \in {Disconnected, Ended}
  /\ conn' = [conn EXCEPT ![p] = Connected]
  /\ UNCHANGED <<rec, mark, note, composer, sends, nextSend>>
  /\ Step("you join a practice")

EndConnection(p) ==
  /\ conn[p] = Connected
  /\ conn' = [conn EXCEPT ![p] = Ended]
  /\ sends' = [k \in SendSlots |->
                 IF sends[k].practice = p /\ sends[k].duetUi = Sent
                 THEN [sends[k] EXCEPT !.endedAfterSent = TRUE]
                 ELSE sends[k]]
  /\ UNCHANGED <<rec, mark, note, composer, nextSend>>
  /\ Step("the connection ends")

\* --- Marks ---

MarkRecord(r, saveNow) ==
  /\ rec[r].present
  /\ mark[r].local = NotMarked
  /\ mark' = [mark EXCEPT ![r] = [saved |-> saveNow, local |-> IF saveNow THEN Marked ELSE NotSaved]]
  /\ UNCHANGED <<conn, rec, note, composer, sends, nextSend>>
  /\ Step(IF saveNow THEN "you mark a record" ELSE "you mark a record, save fails")

SaveMarkCompletes(r) ==
  /\ rec[r].present
  /\ mark[r].local = NotSaved
  /\ mark' = [mark EXCEPT ![r] = [saved |-> TRUE, local |-> Marked]]
  /\ UNCHANGED <<conn, rec, note, composer, sends, nextSend>>
  /\ Step("Duet saves the mark")

UnmarkRecord(r) ==
  /\ mark[r].local # NotMarked
  /\ mark' = [mark EXCEPT ![r] = [saved |-> FALSE, local |-> NotMarked]]
  /\ UNCHANGED <<conn, rec, note, composer, sends, nextSend>>
  /\ Step("you unmark a record")

\* --- Notes ---
\* savedVer: 0 = never saved on server, 1 = saved version, 2 = newer saved version from another phone.
\* localUnsaved: TRUE iff the phone currently holds unsaved local words.

WriteNote(n, r, saveNow) ==
  /\ note[n].status = NotWritten
  /\ IF r = Nobody THEN TRUE ELSE rec[r].present
  /\ note' = [note EXCEPT ![n] = [
                status       |-> IF saveNow THEN Saved ELSE NotSaved,
                savedVer     |-> IF saveNow THEN 1 ELSE 0,
                localUnsaved |-> ~saveNow,
                linkedRec    |-> r
              ]]
  /\ UNCHANGED <<conn, rec, mark, composer, sends, nextSend>>
  /\ Step("you write a note")

EditNote(n, saveNow) ==
  /\ note[n].status \in {Saved, NotSaved}
  /\ (~saveNow => ~note[n].localUnsaved)
  /\ note' = [note EXCEPT ![n].status = IF saveNow THEN Saved ELSE NotSaved,
                          ![n].savedVer = IF saveNow THEN 1 ELSE note[n].savedVer,
                          ![n].localUnsaved = ~saveNow]
  /\ UNCHANGED <<conn, rec, mark, composer, sends, nextSend>>
  /\ Step("you edit a note")

SaveNoteCompletes(n) ==
  /\ note[n].status = NotSaved
  /\ note' = [note EXCEPT ![n].status = Saved, ![n].savedVer = 1, ![n].localUnsaved = FALSE]
  /\ UNCHANGED <<conn, rec, mark, composer, sends, nextSend>>
  /\ Step("Duet saves the note")

OtherPhoneEditsNote(n) ==
  /\ note[n].status \in {Saved, NotSaved}
  /\ note[n].savedVer = 1
  /\ note' = [note EXCEPT ![n].savedVer = 2]
  /\ UNCHANGED <<conn, rec, mark, composer, sends, nextSend>>
  /\ Step("you edit the note on another phone")

DeleteNote(n) ==
  /\ note[n].status # NotWritten
  /\ note' = [note EXCEPT ![n] = [status |-> NotWritten, savedVer |-> 0, localUnsaved |-> FALSE, linkedRec |-> Nobody]]
  /\ UNCHANGED <<conn, rec, mark, composer, sends, nextSend>>
  /\ Step("you delete a note")

\* --- Clinical source changes ---

SourceCorrectsRecord(r) ==
  /\ rec[r].present
  /\ rec[r].version < MaxVersions
  /\ rec' = [rec EXCEPT ![r].version = @ + 1]
  /\ UNCHANGED <<conn, mark, note, composer, sends, nextSend>>
  /\ Step("its source corrects a record")

SourceRemovesRecord(r) ==
  /\ rec[r].present
  /\ rec' = [rec EXCEPT ![r].present = FALSE,
                        ![r].removedNotice = (mark[r].local # NotMarked)]
  /\ mark' = [mark EXCEPT ![r] = [saved |-> FALSE, local |-> NotMarked]]
  /\ composer' = IF composer.phase = Choosing
                 THEN [composer EXCEPT !.chosenRecs = @ \ {r}]
                 ELSE composer
  /\ UNCHANGED <<conn, note, sends, nextSend>>
  /\ Step("its source removes a record")

\* --- Choosing and Sending ---

StartSendFromShared(p) ==
  /\ composer.phase = Idle
  /\ nextSend <= MaxSends
  /\ p \in ConnectedPractices
  /\ (ToSendRecs # {} \/ ToSendNotes # {})
  /\ composer' = [
       phase       |-> Choosing,
       practice    |-> p,
       chosenRecs  |-> ToSendRecs,
       chosenNotes |-> ToSendNotes,
       disclosed   |-> FALSE,
       pendingSlot |-> 0
     ]
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("you tap Send")

StartOneRecordSend(r, p) ==
  /\ composer.phase = Idle
  /\ nextSend <= MaxSends
  /\ rec[r].present
  /\ p \in ConnectedPractices
  /\ composer' = [
       phase       |-> Choosing,
       practice    |-> p,
       chosenRecs  |-> {r},
       chosenNotes |-> {},
       disclosed   |-> FALSE,
       pendingSlot |-> 0
     ]
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("you send one record")

ToggleRecordInChoosing(r) ==
  /\ composer.phase = Choosing
  /\ rec[r].present
  /\ r \in (ToSendRecs \cup composer.chosenRecs)
  /\ composer' = [composer EXCEPT !.chosenRecs = IF r \in @ THEN @ \ {r} ELSE @ \cup {r}]
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("you toggle a record in Choosing")

ToggleNoteInChoosing(n) ==
  /\ composer.phase = Choosing
  /\ n \in (ToSendNotes \cup composer.chosenNotes)
  /\ composer' = [composer EXCEPT !.chosenNotes = IF n \in @ THEN @ \ {n} ELSE @ \cup {n}]
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("you toggle a note in Choosing")

PickPracticeInChoosing(p) ==
  /\ composer.phase = Choosing
  /\ p \in ConnectedPractices
  /\ composer.practice # p
  /\ composer' = [composer EXCEPT !.practice = p]
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("you pick a practice")

LeaveChoosing ==
  /\ composer.phase = Choosing
  /\ composer' = InitComposer
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("you leave without confirming")

ConfirmSend ==
  /\ composer.phase = Choosing
  /\ composer.practice \in ConnectedPractices
  /\ (EffectiveRecs(composer.chosenRecs, composer.chosenNotes) # {} \/ composer.chosenNotes # {})
  /\ nextSend <= MaxSends
  /\ LET k == nextSend
     IN /\ sends' = [sends EXCEPT ![k].confirmed = TRUE,
                                  ![k].disclosed = TRUE,
                                  ![k].practice = composer.practice]
        /\ composer' = [composer EXCEPT !.phase = Sending, !.disclosed = TRUE, !.pendingSlot = k]
        /\ nextSend' = nextSend + 1
  /\ UNCHANGED <<conn, rec, mark, note>>
  /\ Step("you confirm the send")

ClearCarried(recsCarried, notesCarried) ==
  /\ mark' = [r \in Records |->
                IF r \in recsCarried
                THEN [saved |-> FALSE, local |-> NotMarked]
                ELSE mark[r]]
  /\ note' = [n \in Notes |->
                IF n \in notesCarried
                THEN [status |-> NotWritten, savedVer |-> 0, localUnsaved |-> FALSE, linkedRec |-> Nobody]
                ELSE note[n]]

DuetSavesSend ==
  /\ composer.phase = Sending
  /\ LET k == composer.pendingSlot
         p == composer.practice
         recs == EffectiveRecs(composer.chosenRecs, composer.chosenNotes)
     IN /\ conn[p] = Connected
        /\ (recs # {} \/ composer.chosenNotes # {})
        /\ sends' = [sends EXCEPT ![k] = [
                       @ EXCEPT
                         !.duetUi = Sent,
                         !.serverSaved = TRUE,
                         !.practiceReceived = 1,
                         !.practice = p,
                         !.itemsRecs = FreezeRecs(composer.chosenRecs, composer.chosenNotes),
                         !.itemsNotes = composer.chosenNotes
                     ]]
        /\ ClearCarried(recs, composer.chosenNotes)
        /\ composer' = InitComposer
  /\ UNCHANGED <<conn, rec, nextSend>>
  /\ Step("Duet saves the send")

SendUncertain(savedOnServer) ==
  /\ composer.phase = Sending
  /\ LET k == composer.pendingSlot
         p == composer.practice
         recs == EffectiveRecs(composer.chosenRecs, composer.chosenNotes)
     IN /\ (savedOnServer => (conn[p] = Connected /\ (recs # {} \/ composer.chosenNotes # {})))
        /\ sends' = IF savedOnServer
                    THEN [sends EXCEPT ![k] = [
                            @ EXCEPT
                              !.serverSaved = TRUE,
                              !.practiceReceived = 1,
                              !.practice = p,
                              !.itemsRecs = FreezeRecs(composer.chosenRecs, composer.chosenNotes),
                              !.itemsNotes = composer.chosenNotes
                          ]]
                    ELSE sends
        /\ composer' = [composer EXCEPT !.phase = Uncertain]
  /\ UNCHANGED <<conn, rec, mark, note, nextSend>>
  /\ Step("Duet can't tell whether it saved")

ConnectionEndsDuringSending ==
  /\ composer.phase = Sending
  /\ conn[composer.practice] = Ended
  /\ ~sends[composer.pendingSlot].serverSaved
  /\ composer' = [InitComposer EXCEPT !.phase = NotSent, !.practice = composer.practice]
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("the connection ends while sending")

TryAgainSucceeds ==
  /\ composer.phase = Uncertain
  /\ LET k == composer.pendingSlot
         p == composer.practice
         recs == EffectiveRecs(composer.chosenRecs, composer.chosenNotes)
     IN /\ conn[p] = Connected
        /\ (sends[k].serverSaved \/ recs # {} \/ composer.chosenNotes # {})
        /\ IF sends[k].serverSaved THEN
             /\ sends' = [sends EXCEPT ![k].duetUi = Sent]
             /\ ClearCarried({r \in Records : sends[k].itemsRecs[r] > 0}, sends[k].itemsNotes)
           ELSE
             /\ sends' = [sends EXCEPT ![k] = [
                            @ EXCEPT
                              !.duetUi = Sent,
                              !.serverSaved = TRUE,
                              !.practiceReceived = 1,
                              !.practice = p,
                              !.itemsRecs = FreezeRecs(composer.chosenRecs, composer.chosenNotes),
                              !.itemsNotes = composer.chosenNotes
                          ]]
             /\ ClearCarried(recs, composer.chosenNotes)
        /\ composer' = InitComposer
  /\ UNCHANGED <<conn, rec, nextSend>>
  /\ Step("you try again and Duet saves it")

ConnectionEndsDuringUncertain ==
  /\ composer.phase = Uncertain
  /\ conn[composer.practice] = Ended
  /\ composer' = [InitComposer EXCEPT !.phase = CantConfirmEnded, !.practice = composer.practice]
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("the connection ends before confirmation")

DismissEndedOutcome ==
  /\ composer.phase \in {NotSent, CantConfirmEnded}
  /\ composer' = InitComposer
  /\ UNCHANGED <<conn, rec, mark, note, sends, nextSend>>
  /\ Step("you close the send notice")

PracticePicksUpSend(k) ==
  /\ sends[k].duetUi = Sent
  /\ sends[k].serverSaved
  /\ ~sends[k].endedAfterSent
  /\ conn[sends[k].practice] = Connected
  /\ sends' = [sends EXCEPT ![k].practicePickedUp = TRUE, ![k].duetUi = Received]
  /\ UNCHANGED <<conn, rec, mark, note, composer, nextSend, badStep>>
  /\ event' = "the practice's agents pick the send up"

Finished ==
  /\ nextSend > MaxSends
  /\ composer.phase = Idle
  /\ \A k \in SendSlots : sends[k].duetUi \in {Empty, Received} \/ sends[k].endedAfterSent
  /\ \A p \in Practices : conn[p] = Ended
  /\ UNCHANGED vars

Next ==
  \/ (\E p \in Practices : JoinPractice(p) \/ EndConnection(p))
  \/ (\E r \in Records : MarkRecord(r, TRUE) \/ MarkRecord(r, FALSE) \/ SaveMarkCompletes(r)
                         \/ UnmarkRecord(r) \/ SourceCorrectsRecord(r) \/ SourceRemovesRecord(r))
  \/ (\E n \in Notes : (\E r \in Records \cup {Nobody} : WriteNote(n, r, TRUE) \/ WriteNote(n, r, FALSE))
                       \/ EditNote(n, TRUE) \/ EditNote(n, FALSE) \/ SaveNoteCompletes(n)
                       \/ OtherPhoneEditsNote(n) \/ DeleteNote(n))
  \/ (\E p \in Practices : StartSendFromShared(p) \/ PickPracticeInChoosing(p))
  \/ (\E r \in Records, p \in Practices : StartOneRecordSend(r, p))
  \/ (\E r \in Records : ToggleRecordInChoosing(r))
  \/ (\E n \in Notes : ToggleNoteInChoosing(n))
  \/ LeaveChoosing \/ ConfirmSend \/ DuetSavesSend
  \/ SendUncertain(TRUE) \/ SendUncertain(FALSE) \/ ConnectionEndsDuringSending
  \/ TryAgainSucceeds \/ ConnectionEndsDuringUncertain \/ DismissEndedOutcome
  \/ (\E k \in SendSlots : PracticePicksUpSend(k))
  \/ Finished

Spec == Init /\ [][Next]_vars

-----------------------------------------------------------------------------
\* 1. Nothing in For my doctor reaches a practice except in a send you confirmed.
NothingReachesWithoutConfirm ==
  \A k \in SendSlots : sends[k].practiceReceived > 0 => sends[k].confirmed

\* 2. A send goes to one practice, and only to a practice connected to you at that moment.
SendToOneConnectedPractice ==
  /\ ~badStep.unconnectedSend
  /\ \A k \in SendSlots : sends[k].serverSaved => sends[k].practice \in Practices

\* 3. A send holds exactly the records and notes you chose, each record at its current version at that moment, and nothing changes it afterwards. A note written from a record takes that record with it.
SendHoldsExactImmutableContent ==
  /\ ~badStep.sendMutated
  /\ ~badStep.linkedMissing

\* 4. A send reaches the practice at most once, however many times you try again.
SendReachesAtMostOnce ==
  \A k \in SendSlots : sends[k].practiceReceived <= 1

\* 5. When Duet confirms a send as Sent, it clears the marks it carried and takes the notes it carried out of To send, and leaves the rest; an unconfirmed or failed send keeps its marks and notes in To send.
ConfirmedSendClearsCarriedOnly == ~badStep.confirmClearedBad

\* 6. To send shows a record exactly while it's marked, and a note exactly until you send or delete it.
ToSendExact ==
  /\ \A r \in Records : (r \in ToSendRecs) <=> (mark[r].local # NotMarked)
  /\ \A r \in Records : ~rec[r].present => r \notin ToSendRecs
  /\ \A n \in Notes : (n \in ToSendNotes) <=> (note[n].status # NotWritten)

\* 7. Duet shows Sent only after it saved the send, and Received by the care team only after the practice's agents picked it up. A send whose connection ended first stays Sent.
SentAndReceivedTruth ==
  /\ ~badStep.endedBecameRecv
  /\ \A k \in SendSlots :
       /\ (sends[k].duetUi = Sent => sends[k].serverSaved)
       /\ (sends[k].duetUi = Received => (sends[k].serverSaved /\ sends[k].practicePickedUp /\ ~sends[k].endedAfterSent))

\* 8. Before every send, Duet says that a send can't be taken back and isn't for urgent needs or for questions that need an answer.
DisclosureBeforeSend ==
  /\ (composer.phase \in {Sending, Uncertain} => composer.disclosed)
  /\ (\A k \in SendSlots : sends[k].serverSaved => sends[k].disclosed)

\* 9. A mark or a note never shows as saved unless Duet saved it, and Duet never replaces words you haven't saved.
SaveHonesty ==
  /\ ~badStep.otherPhoneOverwrote
  /\ \A r \in Records : mark[r].local = Marked => mark[r].saved
  /\ \A n \in Notes : note[n].status = Saved => (note[n].savedVer > 0 /\ ~note[n].localUnsaved)

\* 10. A source correction never blocks a send and never changes an earlier send.
SourceCorrectionSafe ==
  /\ ~badStep.sendMutated
  /\ \A r \in Records : rec[r].version \in 1..MaxVersions

\* 11. Joining a practice sends nothing from For my doctor, and leaving one removes no send from your list.
JoinAndLeaveSafe ==
  /\ ~badStep.joinSent
  /\ ~badStep.leaveDeleted
  /\ \A k \in SendSlots : sends[k].endedAfterSent => sends[k].duetUi = Sent

\* 12. When a source removes a marked record, Duet clears its mark from To send, shows a plain notice in You, and leaves earlier sends that carried the record unchanged.
SourceRemovalClearsMark ==
  /\ ~badStep.removeBad
  /\ ~badStep.sendMutated
  /\ \A r \in Records : ~rec[r].present => mark[r].local = NotMarked

\* 13. When the connection ends while a send is in Sending or Couldn't confirm the send, Duet removes "Try again", stops sending, and keeps its marks and notes in To send.
ConnectionEndStopsUncertain ==
  /\ ~badStep.uncertainCleared
  /\ (composer.phase \in {NotSent, CantConfirmEnded} =>
        (composer.practice \in Practices /\ composer.pendingSlot = 0 /\ ~composer.disclosed))
=============================================================================

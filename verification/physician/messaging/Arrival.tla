------------------------------ MODULE Arrival ------------------------------
(***************************************************************************)
(* How a patient's message in the practice's channel reaches the right    *)
(* patient: the arrival half of patient messages, drawn from README.md    *)
(* and intent/physician.md P3. Each rule this half holds is one agreed   *)
(* sentence, an INVARIANT or a PROPERTY, quoted word for word.           *)
(*                                                                         *)
(* Reply.tla is the other half. The two meet at the link: this half      *)
(* decides which patient a conversation belongs to, and Reply.tla takes  *)
(* the link as given and moves it whenever a correction could. Here the  *)
(* reply is only open, withheld or closed, for the patient it belongs    *)
(* to: its drafts, copying and Undo of No reply needed are in Reply.tla, *)
(* and none of them changes which patient a message reaches or whether   *)
(* it shows.                                                              *)
(*                                                                         *)
(* Which patients each physician may access, and who sees a message      *)
(* before it's linked, are open (P2), so they are inputs: TLC checks the *)
(* rules for every setting in which each patient is someone's and        *)
(* someone sees unlinked messages. Whom the confirmed sender and the     *)
(* channel point to are inputs too.                                       *)
(*                                                                         *)
(* One conversation. Coda reads the channel's items in any order, and    *)
(* decides whether each patient message is from a patient before it reads *)
(* the next. Whether a message is from a patient, whether a reply would  *)
(* help, and whether the practice answered are Coda's judgment; here     *)
(* each is a choice. Coda's retries while reading are not steps. The     *)
(* window for Undo stays open until it expires or something it would    *)
(* restore changes.                                                       *)
(*                                                                         *)
(* Not modeled, since they rest on Coda's records or the channel: Coda   *)
(* saves each message before moving on and reads a delivered-twice      *)
(* message once, one reader per channel, a review starting once across  *)
(* restarts, the patient's words unchanged, moved messages kept as       *)
(* entered in error, and one failure never holding up the others. The   *)
(* backlog offer, and what a row or suggestion says, are not steps.      *)
(*                                                                         *)
(* `event` names the step just taken; VIEW leaves it out of the state.   *)
(***************************************************************************)
EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS
  Patients, Physicians,
  ChannelMayTie,       \* the channel can tie the conversation to one patient
  MayBeUnreadable,     \* a patient message can arrive that Coda can't read
  MaxPatientMessages, MaxPracticeMessages, MaxCorrections,
  MaxStops             \* how many times the channel stops being read

Nobody == "Nobody"
NotPatient == "Not a patient"
Coda == "Coda"
Review == "Coda's review"
OwnWords == "Words written in the channel"
AutoReply == "An automatic reply"
Reasons == {"a revoked grant", "an administrator's block", "an outage"}
AsksReconnect == {"a revoked grant", "an administrator's block"}

MaxItems == MaxPatientMessages + MaxPracticeMessages
ItemKey(i) == "Item " \o ToString(i)
CorrectionKey(k) == "Correction " \o ToString(k)
Events == {ItemKey(i) : i \in 1..MaxItems} \cup {CorrectionKey(k) : k \in 1..MaxCorrections}

NotSettled == [why |-> "", by |-> Nobody, cites |-> 0]
NoReply == [open |-> FALSE, patient |-> Nobody, withheld |-> FALSE]
NoUndo == [by |-> Nobody, before |-> <<>>]

VARIABLES
  access,      \* input: the patients each physician may access
  matchers,    \* input: who sees a message before it's linked, on Messages to match and Filtered
  sender,      \* input: the patients the conversation's sender is confirmed for
  says,        \* input: the patient the channel ties the conversation to, or Nobody
  channel,     \* whether Coda can read the channel, and why not
  inbox,       \* the conversation as the channel holds it, in the order it was sent
  read,        \* the items Coda has read
  match,       \* each item: Unread, Sorting, Filtered, Waiting, Unlinked, Held, Linked, Not a patient or Deleted
  settled,     \* for each patient message: why it is handled, by whom, citing which item
  suggests,    \* Coda's suggested patient for the waiting messages
  link,        \* the patient the conversation is linked to, Nobody, or Not a patient
  review,      \* each patient's review: running, and which items it has read
  starts,      \* how many reviews each event started for each patient
  reply,       \* the conversation's reply: open, for which patient, withheld
  history,     \* what the patient's history says the practice sent
  corrections, stops,
  undo,        \* what Undo of a correction would restore, and who may use it
  event

vars == <<access, matchers, sender, says, channel, inbox, read, match, settled, suggests, link,
          review, starts, reply, history, corrections, stops, undo, event>>
state == <<access, matchers, sender, says, channel, inbox, read, match, settled, suggests, link,
           review, starts, reply, history, corrections, stops, undo>>

Patient(i) == inbox[i].from = "Patient"
Answer(i) == inbox[i].from = "Practice" /\ inbox[i].words # AutoReply
IsSettled(i) == settled[i].why # ""
Settle(s, S, why, by, cites) ==
  [i \in DOMAIN s |-> IF i \in S THEN [why |-> why, by |-> by, cites |-> cites] ELSE s[i]]
Linked == {i \in read : match[i] = "Linked"}
Open == {i \in Linked : Patient(i) /\ ~IsSettled(i)}
OpenReadable == {i \in Open : inbox[i].readable}
Waiting == {i \in read : match[i] = "Waiting"}
Sorting == {i \in read : match[i] = "Sorting"}

\* The patients the confirmed sender and the channel point to; the message links on its own only to
\* exactly one.
Pointed == (IF Cardinality(sender) = 1 THEN sender ELSE {}) \cup (IF says \in Patients THEN {says} ELSE {})
Target == CASE link # Nobody -> link
            [] Cardinality(Pointed) = 1 -> CHOOSE p \in Pointed : TRUE
            [] OTHER -> Nobody

\* What a viewer of Messages to match sees as Coda's suggestion.
Shown(d) == IF suggests \in access[d] THEN suggests ELSE Nobody

\* What Today says about the channel, and whether it asks for Reconnect.
TodayChannel == CASE channel.state = "Can't read" -> "Coda can't read Spruce, because of " \o channel.reason
                  [] channel.state = "Disconnected" -> "Spruce is disconnected"
                  [] channel.state = "Ended" -> "Spruce was disconnected"
                  [] OTHER -> ""
AsksForReconnect == \/ channel.state = "Can't read" /\ channel.reason \in AsksReconnect
                    \/ channel.state = "Disconnected"

Ev(n, w) == event' = [name |-> n, who |-> w]

Init ==
  /\ access \in {a \in [Physicians -> SUBSET Patients] : \A p \in Patients : \E d \in Physicians : p \in a[d]}
  /\ matchers \in SUBSET Physicians \ {{}}
  /\ sender \in SUBSET Patients
  /\ says \in (IF ChannelMayTie THEN Patients \cup {Nobody} ELSE {Nobody})
  /\ channel = [state |-> "Reading", reason |-> ""]
  /\ inbox = <<>>
  /\ read = {}
  /\ match = [i \in 1..MaxItems |-> "Unread"]
  /\ settled = [i \in 1..MaxItems |-> NotSettled]
  /\ suggests = Nobody
  /\ link = Nobody
  /\ review = [p \in Patients |-> [on |-> FALSE, read |-> {}]]
  /\ starts = [e \in Events, p \in Patients |-> 0]
  /\ reply = NoReply
  /\ history = {}
  /\ corrections = 0
  /\ stops = 0
  /\ undo = NoUndo
  /\ event = [name |-> "start", who |-> Nobody]

\* An event wakes a patient's review. A review already running takes the news in when it reads
\* again, so the event starts nothing new.
Woken(rv, p) == IF rv[p].on THEN rv ELSE [rv EXCEPT ![p] = [on |-> TRUE, read |-> {}]]
Counted(rv, st, p, e) == IF rv[p].on THEN st ELSE [st EXCEPT ![<<e, p>>] = @ + 1]
Wake(p, e) == review' = Woken(review, p) /\ starts' = Counted(review, starts, p, e)

Inputs == <<access, matchers, sender, says>>

\* A patient message goes where the link, the sender and the channel say: to its patient, to not a
\* patient when the conversation was moved there, or to Messages to match with Coda's suggestion.
RouteTo(i) ==
  CASE Target \in Patients ->
         /\ match' = [match EXCEPT ![i] = "Linked"]
         /\ link' = Target
         /\ Wake(Target, ItemKey(i))
         /\ UNCHANGED <<settled, suggests>>
    [] Target = NotPatient ->
         /\ match' = [match EXCEPT ![i] = "Not a patient"]
         /\ settled' = Settle(settled, {i}, "Not a patient", Coda, 0)
         /\ UNCHANGED <<link, review, starts, suggests>>
    [] OTHER ->
         /\ match' = [match EXCEPT ![i] = "Waiting"]
         /\ suggests' \in (IF suggests = Nobody THEN Patients ELSE {suggests})
         /\ UNCHANGED <<link, review, starts, settled>>

-----------------------------------------------------------------------------
(* The world: the patient writes and the practice sends, in the channel.  *)

PatientWrites ==
  /\ Cardinality({i \in 1..Len(inbox) : Patient(i)}) < MaxPatientMessages
  /\ \E unreadable \in (IF MayBeUnreadable THEN BOOLEAN ELSE {FALSE}) :
       inbox' = Append(inbox, [from |-> "Patient", readable |-> ~unreadable, words |-> ""])
  /\ Ev("the patient writes", "Patient")
  /\ UNCHANGED <<Inputs, channel, read, match, settled, suggests, link, review, starts, reply, history,
                 corrections, stops>>

PracticeSends ==
  /\ Cardinality({i \in 1..Len(inbox) : ~Patient(i)}) < MaxPracticeMessages
  /\ \E w \in {OwnWords, AutoReply} :
       inbox' = Append(inbox, [from |-> "Practice", readable |-> TRUE, words |-> w])
  /\ Ev("the practice sends in Spruce", "Practice")
  /\ UNCHANGED <<Inputs, channel, read, match, settled, suggests, link, review, starts, reply, history,
                 corrections, stops>>

-----------------------------------------------------------------------------
(* Coda reads an item while it can read the channel, once Coda has decided *)
(* about the last patient message. A patient message waits for that       *)
(* decision. A practice message joins the linked patient's history and   *)
(* wakes their review, or is held while the conversation is unlinked.     *)
CodaReads ==
  /\ channel.state = "Reading" /\ Sorting = {}
  /\ \E i \in (1..Len(inbox)) \ read :
       /\ read' = read \cup {i}
       /\ IF Patient(i)
          THEN /\ match' = [match EXCEPT ![i] = "Sorting"]
               /\ UNCHANGED <<link, review, starts, history>>
          ELSE IF link \in Patients
               THEN /\ match' = [match EXCEPT ![i] = "Linked"]
                    /\ history' = history \cup {[patient |-> link, item |-> i, words |-> inbox[i].words]}
                    /\ Wake(link, ItemKey(i))
                    /\ UNCHANGED link
               ELSE /\ match' = [match EXCEPT ![i] = "Held"]
                    /\ UNCHANGED <<link, review, starts, history>>
  /\ Ev("Coda reads the channel", Coda)
  /\ UNCHANGED <<Inputs, channel, inbox, settled, suggests, reply, corrections, stops>>

(* Coda decides the message is from a patient, or 15 minutes pass first; either way it goes where *)
(* the link, the sender and the channel say.                                                       *)
CodaDecidesPatient(name) ==
  /\ \E i \in Sorting : RouteTo(i)
  /\ Ev(name, Coda)
  /\ UNCHANGED <<Inputs, channel, inbox, read, reply, history, corrections, stops>>

(* Coda judges the message isn't from a patient: it goes to the Filtered list. *)
CodaFilters ==
  /\ \E i \in Sorting : match' = [match EXCEPT ![i] = "Filtered"]
  /\ Ev("Coda judges it isn't from a patient", Coda)
  /\ UNCHANGED <<Inputs, channel, inbox, read, settled, suggests, link, review, starts, reply,
                 history, corrections, stops>>

(* Someone who sees the Filtered list brings a message back with This is a patient. *)
ThisIsAPatient(d) ==
  /\ d \in matchers
  /\ \E i \in read : match[i] = "Filtered" /\ RouteTo(i)
  /\ Ev("This is a patient", d)
  /\ UNCHANGED <<Inputs, channel, inbox, read, reply, history, corrections, stops>>

(* A physician who sees Messages to match links the waiting messages to a patient they may access. *)
(* The conversation is linked, the practice's held messages join that patient's history, and the  *)
(* review wakes once.                                                                               *)
Confirm(d, p) ==
  /\ d \in matchers /\ p \in access[d] /\ Waiting # {}
  /\ LET first == CHOOSE i \in Waiting : \A j \in Waiting : i <= j
         held == {j \in read : match[j] = "Held"}
     IN /\ link' = p
        /\ match' = [i \in DOMAIN match |-> IF i \in Waiting \cup held THEN "Linked" ELSE match[i]]
        /\ history' = history \cup {[patient |-> p, item |-> j, words |-> inbox[j].words] : j \in held}
        /\ Wake(p, ItemKey(first))
  /\ suggests' = Nobody
  /\ Ev("you link it to the patient", d)
  /\ UNCHANGED <<Inputs, channel, inbox, read, settled, reply, corrections, stops>>

(* Leaving a message unlinked covers only that message. *)
LeaveUnlinked(d) ==
  /\ d \in matchers
  /\ \E i \in Waiting :
       /\ match' = [match EXCEPT ![i] = "Unlinked"]
       /\ settled' = Settle(settled, {i}, "Left unlinked", d, 0)
       /\ suggests' = IF Waiting = {i} THEN Nobody ELSE suggests
  /\ Ev("you leave it unlinked", d)
  /\ UNCHANGED <<Inputs, channel, inbox, read, link, review, starts, reply, history, corrections, stops>>

(* A physician who may access the patient moves the conversation to another patient they may      *)
(* access, or to not a patient. Nothing of it stays readable on the patient it left: their review  *)
(* lets go and repairs what relied on it, and history moves. A reply is withheld for the new       *)
(* patient's review to redraft or close; moving to not a patient closes it. One correction starts *)
(* one review for each patient it affects.                                                          *)
Correct(d, q) ==
  /\ link \in access[d]
  /\ q \in (access[d] \cup {NotPatient}) \ {link}
  /\ corrections < MaxCorrections
  /\ LET old == link
         e == CorrectionKey(corrections + 1)
         letGo == [review EXCEPT ![old] = [on |-> TRUE, read |-> {}]]
         st1 == Counted(review, starts, old, e)
     IN /\ link' = q
        /\ review' = IF q \in Patients THEN Woken(letGo, q) ELSE letGo
        /\ starts' = IF q \in Patients THEN Counted(letGo, st1, q, e) ELSE st1
        /\ history' = {[h EXCEPT !.patient = q] : h \in history}
        /\ IF q \in Patients
           THEN /\ reply' = IF reply.open THEN [reply EXCEPT !.patient = q, !.withheld = TRUE] ELSE reply
                /\ UNCHANGED <<match, settled>>
           ELSE /\ reply' = NoReply
                /\ match' = [i \in DOMAIN match |-> IF match[i] = "Linked" THEN "Not a patient" ELSE match[i]]
                /\ settled' = Settle(settled, Open, "Not a patient", d, 0)
        /\ corrections' = corrections + 1
        /\ undo' = [by |-> d, before |-> <<link, review, reply, history, match, settled>>]
  /\ Ev(IF q = NotPatient THEN "you move it to not a patient" ELSE "you move it to another patient", d)
  /\ UNCHANGED <<Inputs, channel, inbox, read, suggests, stops>>

(* Undo within a few seconds puts the correction back. *)
UndoCorrection(d) ==
  /\ undo.by = d
  /\ link' = undo.before[1]
  /\ review' = undo.before[2]
  /\ reply' = undo.before[3]
  /\ history' = undo.before[4]
  /\ match' = undo.before[5]
  /\ settled' = undo.before[6]
  /\ undo' = NoUndo
  /\ Ev("you undo the move", d)
  /\ UNCHANGED <<Inputs, channel, inbox, read, suggests, starts, corrections, stops>>

UndoExpires ==
  /\ undo.by # Nobody
  /\ undo' = NoUndo
  /\ Ev("Undo expires", Nobody)
  /\ UNCHANGED <<Inputs, channel, inbox, read, match, settled, suggests, link, review, starts, reply,
                 history, corrections, stops>>

-----------------------------------------------------------------------------
(* The review, and the reply as open, withheld or closed.                  *)

ReviewReads(p) ==
  /\ review[p].on /\ link = p /\ review[p].read # Linked
  /\ review' = [review EXCEPT ![p].read = Linked]
  /\ Ev("Coda's review reads the conversation", Coda)
  /\ UNCHANGED <<Inputs, channel, inbox, read, match, settled, suggests, link, starts, reply, history,
                 corrections, stops>>

(* A review of a patient the conversation left repairs what relied on it and ends. *)
ReviewRepairs(p) ==
  /\ review[p].on /\ link # p
  /\ review' = [review EXCEPT ![p] = [on |-> FALSE, read |-> {}]]
  /\ Ev("Coda repairs what relied on it", Coda)
  /\ UNCHANGED <<Inputs, channel, inbox, read, match, settled, suggests, link, starts, reply, history,
                 corrections, stops>>

(* The review finishes on what it read. It may find that a practice message it read answered the  *)
(* patient's messages before it: those are handled, citing that message. For any left, it drafts *)
(* or updates the reply, or finds no reply would help.                                             *)
ReviewFinishes(p) ==
  /\ review[p].on /\ link = p /\ review[p].read = Linked
  /\ \E cite \in {0} \cup {j \in review[p].read : Answer(j)} :
     \E propose \in BOOLEAN :
     LET answered == IF cite = 0 THEN {} ELSE {i \in Open : i < cite}
         left == OpenReadable \ answered
         drafts == propose /\ left # {}
         s1 == Settle(settled, answered, "Practice answered", Review, cite)
     IN /\ cite # 0 => answered # {} /\ (left = {} \/ propose)
        /\ settled' = IF drafts THEN s1 ELSE Settle(s1, left, "No reply needed", Review, 0)
        /\ reply' = IF drafts THEN [open |-> TRUE, patient |-> p, withheld |-> FALSE] ELSE NoReply
  /\ review' = [review EXCEPT ![p] = [on |-> FALSE, read |-> {}]]
  /\ Ev("Coda's review finishes", Review)
  /\ UNCHANGED <<Inputs, channel, inbox, read, match, suggests, link, starts, history, corrections, stops>>

(* A physician who sees the reply closes it, as No reply needed, or Done while the channel is     *)
(* disconnected.                                                                                    *)
Close(d) ==
  /\ reply.open /\ ~reply.withheld /\ reply.patient \in access[d]
  /\ settled' = Settle(settled, OpenReadable, "No reply needed", d, 0)
  /\ reply' = NoReply
  /\ Ev("you mark No reply needed", d)
  /\ UNCHANGED <<Inputs, channel, inbox, read, match, suggests, link, review, starts, history,
                 corrections, stops>>

(* A message Coda can't read waits on Today until someone marks it Done. *)
DoneUnreadable(d) ==
  /\ link \in access[d]
  /\ \E i \in Open : ~inbox[i].readable /\ settled' = Settle(settled, {i}, "Done", d, 0)
  /\ Ev("you mark the unreadable message Done", d)
  /\ UNCHANGED <<Inputs, channel, inbox, read, match, suggests, link, review, starts, reply, history,
                 corrections, stops>>

-----------------------------------------------------------------------------
(* The channel stops: Coda can't read it, for a reason, or the practice    *)
(* disconnects it. A completed disconnect deletes every message never     *)
(* linked to a patient.                                                    *)

ChannelSteps == <<Inputs, inbox, read, match, settled, suggests, link, review, starts, reply, history,
                  corrections>>

CantRead ==
  /\ channel.state = "Reading" /\ stops < MaxStops
  /\ \E r \in Reasons : channel' = [state |-> "Can't read", reason |-> r]
  /\ stops' = stops + 1
  /\ Ev("Coda can't read Spruce", Coda)
  /\ UNCHANGED ChannelSteps

ReadsAgain ==
  /\ channel.state = "Can't read" /\ channel.reason \notin AsksReconnect
  /\ channel' = [state |-> "Reading", reason |-> ""]
  /\ Ev("Coda reads Spruce again", Coda)
  /\ UNCHANGED <<ChannelSteps, stops>>

Reconnect ==
  /\ AsksForReconnect
  /\ channel' = [state |-> "Reading", reason |-> ""]
  /\ Ev("the practice reconnects Spruce", "Practice")
  /\ UNCHANGED <<ChannelSteps, stops>>

Disconnect ==
  /\ \/ channel.state = "Reading" /\ stops < MaxStops /\ stops' = stops + 1
     \/ channel.state = "Can't read" /\ stops' = stops
  /\ channel' = [state |-> "Disconnected", reason |-> ""]
  /\ Ev("the practice disconnects Spruce", "Practice")
  /\ UNCHANGED ChannelSteps

Unlinked == {"Sorting", "Waiting", "Filtered", "Unlinked", "Held"}

CompleteDisconnect ==
  /\ channel.state = "Disconnected"
  /\ channel' = [state |-> "Ended", reason |-> ""]
  /\ match' = [i \in DOMAIN match |-> IF match[i] \in Unlinked THEN "Deleted" ELSE match[i]]
  /\ settled' = Settle(settled, {i \in read : Patient(i) /\ match[i] \in Unlinked /\ ~IsSettled(i)},
                       "Deleted", Coda, 0)
  /\ suggests' = Nobody
  /\ Ev("the disconnect completes", "Practice")
  /\ UNCHANGED <<Inputs, inbox, read, link, review, starts, reply, history, corrections, stops>>

-----------------------------------------------------------------------------

\* Steps that leave Undo alone unless they change something it would restore.
Plain ==
  \/ PatientWrites \/ PracticeSends \/ CodaReads
  \/ CodaDecidesPatient("Coda decides it's from a patient") \/ CodaDecidesPatient("15 minutes pass")
  \/ CodaFilters
  \/ \E d \in Physicians : ThisIsAPatient(d) \/ LeaveUnlinked(d) \/ Close(d) \/ DoneUnreadable(d)
  \/ \E d \in Physicians, p \in Patients : Confirm(d, p)
  \/ \E p \in Patients : ReviewReads(p) \/ ReviewRepairs(p) \/ ReviewFinishes(p)
  \/ CantRead \/ ReadsAgain \/ Reconnect \/ Disconnect \/ CompleteDisconnect

Restorable == <<link, reply, history, match, settled>>

Busy ==
  \/ Plain /\ undo' = IF Restorable' = Restorable THEN undo ELSE NoUndo
  \/ \E d \in Physicians, q \in Patients \cup {NotPatient} : Correct(d, q)
  \/ \E d \in Physicians : UndoCorrection(d)
  \/ UndoExpires

\* Everything that could happen has happened.
Rest == ~ENABLED Busy /\ UNCHANGED vars

Next == Busy \/ Rest

Spec == Init /\ [][Next]_vars

Symmetry == Permutations(Patients) \cup Permutations(Physicians)

-----------------------------------------------------------------------------
(* The rules, in the README's numbering.                                   *)

\* 2. A suggestion never names a patient the viewer may not access.
SuggestionsWithAccess == \A d \in matchers : Shown(d) \in access[d] \cup {Nobody}

\* 3. A message links on its own only when its conversation is already linked to that patient, its
\* sender is confirmed for that one patient, or the channel ties its conversation to that one patient.
LinksOnItsOwn == [][\A i \in read' :
  (inbox[i].from = "Patient" /\ match[i] # "Linked" /\ match'[i] = "Linked"
   /\ event'.name \notin {"you link it to the patient", "you undo the move"})
    => (link = link' \/ sender = {link'} \/ says = link')]_vars

\* 4. Once a conversation moves to another patient, nothing of it stays readable on the patient it left.
RightPatient ==
  /\ \A p \in Patients \ {link} : review[p].read = {}
  /\ reply.open => reply.patient = link
  /\ \A h \in history : h.patient = link

\* 5. Once Coda has decided or 15 minutes have passed, every patient message shows to someone who can
\* act on it until it's handled.
NeverInvisible == \A i \in read :
  (Patient(i) /\ ~IsSettled(i) /\ match[i] # "Sorting") =>
    \/ match[i] \in {"Waiting", "Filtered"} /\ matchers # {}
    \/ /\ match[i] = "Linked"
       /\ \E d \in Physicians :
            /\ link \in access[d]
            /\ ~inbox[i].readable \/ review[link].on \/ reply.open

\* 6. Only a review that read the practice's message closes a reply as answered, and an automatic
\* reply never answers it.
OnlyTheReviewAnswers == \A i \in 1..MaxItems :
  settled[i].why = "Practice answered" =>
    /\ settled[i].by = Review
    /\ settled[i].cites \in read /\ settled[i].cites > i
    /\ Answer(settled[i].cites)

\* 11. One message, practice message or correction starts at most one review for each patient it affects.
OneReview == \A e \in Events, p \in Patients : starts[<<e, p>>] <= 1

\* 12. What the patient's history says the practice sent is what the channel shows it sent.
WhatWasSent == \A h \in history : h.words = inbox[h.item].words

\* 13. While Coda can't read the channel, Today says so.
TodaySaysSo == channel.state # "Reading" => TodayChannel # ""

\* 14. A completed disconnect leaves no message that was never linked to a patient.
NothingUnlinkedKept == channel.state = "Ended" => \A i \in read : match[i] \notin Unlinked
=============================================================================

------------------------------- MODULE Reply -------------------------------
(***************************************************************************)
(* How Coda's reply works once a conversation is linked, until the        *)
(* practice has answered: the reply half of patient messages, drawn from  *)
(* README.md and intent/physician.md P3. Each rule this half holds is one *)
(* agreed sentence, an INVARIANT or a PROPERTY, quoted word for word.     *)
(*                                                                         *)
(* Arrival.tla is the other half. The two meet at the link: here the      *)
(* conversation starts linked to a patient, each message Coda reads joins *)
(* that patient at once, and a correction can move the conversation to   *)
(* another patient or to not a patient at any moment. How a message      *)
(* reaches its patient, and who may correct it, are Arrival.tla's.        *)
(*                                                                         *)
(* Which patients each physician may access is open (P2), so it is an     *)
(* input: TLC checks the rules for every setting.                          *)
(*                                                                         *)
(* One conversation. Coda reads the channel's items in any order, so a    *)
(* message can arrive late, older than the reply. Whether a reply would   *)
(* help, whether to flag a concern, and whether the practice answered are *)
(* the review's judgment; here each is a choice. The card shows a change  *)
(* at once, so what a physician sees is the state. The window for Undo    *)
(* stays open until it expires, something it would restore changes, or    *)
(* Coda reads a new message.                                               *)
(*                                                                         *)
(* Not modeled, since they rest on Coda's records or the page: the card   *)
(* showing the messages and the reply from one version, the patient's     *)
(* words unchanged, and Undo restoring the reply's exact text. What the   *)
(* row and the card say, Basis and Sources, are not steps.                *)
(*                                                                         *)
(* `event` names the step just taken; VIEW leaves it out of the state.    *)
(***************************************************************************)
EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS
  Patients, Physicians,
  MayFlag,             \* the review can raise a concern about the messages
  MaxPatientMessages, MaxPracticeMessages, MaxCorrections, MaxDisconnects

Nobody == "Nobody"
NotPatient == "Not a patient"
Coda == "Coda"
Review == "Coda's review"

\* What the practice sends: words someone chose, or the channel's automatic reply.
CodaWords == "The reply Coda drafted"
YourWords_ == "Your draft"
OwnWords == "Words written in the channel"
AutoReply == "An automatic reply"
NoWords == "Nothing"

\* A draft's text once the physician has made it theirs: edited, being revised at their request,
\* revised, or Coda's update they chose.
Theirs == {"Yours", "Revising", "Revised", "Taken"}

NotSettled == [why |-> "", by |-> Nobody, cites |-> 0]
NoDraft == [of |-> 0, text |-> "Coda's", behind |-> FALSE, mine |-> FALSE]
NoUndo == [kind |-> "", by |-> Nobody, before |-> <<>>]

VARIABLES
  access,      \* input: the patients each physician may access
  link,        \* the patient the conversation is linked to, or Not a patient
  channel,     \* "Connected", "Disconnected", or "Ended" once a disconnect completes
  inbox,       \* the conversation as the channel holds it, in the order it was sent
  read,        \* the items Coda has read
  settled,     \* for each patient message: why it is handled, by whom, citing which item
  review,      \* each patient's review: running, and which items it has read
  reply,       \* the conversation's one reply, shared by everyone who may access its patient
  concern,     \* a concern about the messages, and why it closed
  draft,       \* each physician's draft: its reply, its text, whether Coda's update is newer, and
               \* whether they have their own version to restore
  viewing,     \* which physicians have the reply open
  copied,      \* who last copied the reply and what, which the others see
  history,     \* what the patient's history says the practice sent
  corrections, disconnects,
  undo,        \* what Undo would restore, and who may use it
  event

vars == <<access, link, channel, inbox, read, settled, review, reply, concern, draft, viewing, copied,
          history, corrections, disconnects, undo, event>>
state == <<access, link, channel, inbox, read, settled, review, reply, concern, draft, viewing, copied,
           history, corrections, disconnects, undo>>

MaxItems == MaxPatientMessages + MaxPracticeMessages
Patient(i) == inbox[i].from = "Patient"
Answer(i) == inbox[i].from = "Practice" /\ inbox[i].words # AutoReply
IsSettled(i) == settled[i].why # ""
Settle(s, S, why, by, cites) ==
  [i \in DOMAIN s |-> IF i \in S THEN [why |-> why, by |-> by, cites |-> cites] ELSE s[i]]
PatientRead == {i \in read : Patient(i)}
Open == {i \in PatientRead : ~IsSettled(i)}

\* A physician sees the reply while they may access its patient.
CanSee(d) == reply.open /\ reply.patient \in access[d]
\* Their draft is on the card when it belongs to this reply; otherwise the card shows Coda's.
On(d) == reply.open /\ draft[d].of = reply.no
Text(d) == IF On(d) THEN draft[d].text ELSE "Coda's"
\* Coda's update is offered beside the physician's own text that came before it.
Offered(d) == On(d) /\ draft[d].text \in Theirs \ {"Revising"} /\ draft[d].behind
\* What the card shows a physician, and so what Copy reply copies.
CardWords(d) == IF Text(d) \in {"Coda's", "Taken"} THEN CodaWords ELSE YourWords_
Saves(d, text, behind, mine) ==
  draft' = [draft EXCEPT ![d] = [of |-> reply.no, text |-> text, behind |-> behind, mine |-> mine]]

Ev(n, w) == event' = [name |-> n, who |-> w]

Init ==
  /\ access \in [Physicians -> SUBSET Patients]
  /\ link \in Patients
  /\ channel = "Connected"
  /\ inbox = <<>>
  /\ read = {}
  /\ settled = [i \in 1..MaxItems |-> NotSettled]
  /\ review = [p \in Patients |-> [on |-> FALSE, read |-> {}]]
  /\ reply = [open |-> FALSE, no |-> 0, patient |-> Nobody, withheld |-> FALSE, basis |-> {},
              told |-> FALSE, cut |-> FALSE]
  /\ concern = [state |-> "None", closedBy |-> Nobody]
  /\ draft = [d \in Physicians |-> NoDraft]
  /\ viewing = [d \in Physicians |-> FALSE]
  /\ copied = [who |-> Nobody, words |-> NoWords]
  /\ history = {}
  /\ corrections = 0
  /\ disconnects = 0
  /\ undo = NoUndo
  /\ event = [name |-> "start", who |-> Nobody]

Woken(rv, p) == IF rv[p].on THEN rv ELSE [rv EXCEPT ![p] = [on |-> TRUE, read |-> {}]]
Closed(r) == [r EXCEPT !.open = FALSE, !.patient = Nobody, !.withheld = FALSE, !.basis = {},
                       !.told = FALSE, !.cut = FALSE]

-----------------------------------------------------------------------------
(* The world: the patient writes and the practice sends, in the channel.  *)

World == <<access, link, channel, read, settled, review, reply, concern, draft, viewing, copied,
           history, corrections, disconnects>>

PatientWrites ==
  /\ Cardinality({i \in 1..Len(inbox) : Patient(i)}) < MaxPatientMessages
  /\ inbox' = Append(inbox, [from |-> "Patient", words |-> ""])
  /\ Ev("the patient writes", "Patient")
  /\ UNCHANGED World

(* Someone at the practice sends in the conversation: the reply they copied, their own words, or  *)
(* the channel's automatic reply. Coda never sends.                                                *)
PracticeSends ==
  /\ Cardinality({i \in 1..Len(inbox) : ~Patient(i)}) < MaxPracticeMessages
  /\ \E w \in {OwnWords, AutoReply} \cup ({copied.words} \ {NoWords}) :
       inbox' = Append(inbox, [from |-> "Practice", words |-> w])
  /\ Ev("the practice sends in Spruce", "Practice")
  /\ UNCHANGED World

(* Coda reads an item while the channel is connected, in any order. A patient message joins the *)
(* patient and wakes their review, and an open reply's card shows it at once. A practice message  *)
(* joins the history and wakes the review; Coda has now seen what was sent, so who copied the     *)
(* reply and the disconnected line both clear.                                                     *)
CodaReads ==
  /\ channel = "Connected"
  /\ \E i \in (1..Len(inbox)) \ read :
       /\ read' = read \cup {i}
       /\ IF Patient(i)
          THEN /\ IF link \in Patients
                  THEN /\ review' = Woken(review, link)
                       /\ reply' = IF reply.open THEN [reply EXCEPT !.told = TRUE] ELSE reply
                       /\ UNCHANGED settled
                  ELSE /\ settled' = Settle(settled, {i}, "Not a patient", Coda, 0)
                       /\ UNCHANGED <<review, reply>>
               /\ UNCHANGED <<history, copied>>
          ELSE /\ history' = history \cup {[item |-> i, words |-> inbox[i].words]}
               /\ review' = IF link \in Patients THEN Woken(review, link) ELSE review
               /\ copied' = [who |-> Nobody, words |-> NoWords]
               /\ reply' = [reply EXCEPT !.cut = FALSE]
               /\ UNCHANGED settled
  /\ Ev("Coda reads the channel", Coda)
  /\ UNCHANGED <<access, link, channel, inbox, concern, draft, viewing, corrections, disconnects>>

(* A physician moves the conversation to another patient, or to not a patient (Arrival.tla says   *)
(* who may). The patient it left lets go: their review repairs what relied on it. The reply and a *)
(* concern are withheld for the new patient's review to redraft or close; the withheld reply is a  *)
(* new reply, so each physician's draft stays theirs and never moves to it. Anyone who may not     *)
(* access the new patient loses the reply. Moving to not a patient closes the reply and concern.   *)
Correct(q) ==
  /\ link \in Patients
  /\ q \in (Patients \cup {NotPatient}) \ {link}
  /\ corrections < MaxCorrections
  /\ link' = q
  /\ review' = LET letGo == [review EXCEPT ![link] = [on |-> TRUE, read |-> {}]]
               IN IF q \in Patients THEN Woken(letGo, q) ELSE letGo
  /\ IF q \in Patients
     THEN /\ reply' = IF reply.open THEN [reply EXCEPT !.patient = q, !.withheld = TRUE, !.no = @ + 1]
                      ELSE reply
          /\ concern' = IF concern.state = "Open" THEN [concern EXCEPT !.state = "Withheld"] ELSE concern
          /\ UNCHANGED settled
     ELSE /\ reply' = Closed(reply)
          /\ concern' = IF concern.state \in {"Open", "Withheld"}
                        THEN [state |-> "Closed", closedBy |-> "Correction"] ELSE concern
          /\ settled' = Settle(settled, Open, "Not a patient", "A physician", 0)
  /\ viewing' = [x \in Physicians |-> viewing[x] /\ q \in access[x]]
  /\ corrections' = corrections + 1
  /\ undo' = [kind |-> "correction", by |-> "A physician", before |-> <<link, review, reply, concern, settled>>]
  /\ Ev(IF q = NotPatient THEN "a physician moves it to not a patient" ELSE "a physician moves it to another patient",
        "A physician")
  /\ UNCHANGED <<access, channel, inbox, read, draft, copied, history, disconnects>>

(* The practice disconnects the channel, and reconnects it or completes the disconnect. Replies    *)
(* and drafts stay, and an open reply says the channel is disconnected.                            *)
Disconnect ==
  /\ channel = "Connected" /\ disconnects < MaxDisconnects
  /\ channel' = "Disconnected"
  /\ reply' = [reply EXCEPT !.cut = reply.open]
  /\ disconnects' = disconnects + 1
  /\ Ev("the practice disconnects Spruce", "Practice")
  /\ UNCHANGED <<access, link, inbox, read, settled, review, concern, draft, viewing, copied, history,
                 corrections>>

Reconnect ==
  /\ channel = "Disconnected"
  /\ channel' = "Connected"
  /\ Ev("the practice reconnects Spruce", "Practice")
  /\ UNCHANGED <<access, link, inbox, read, settled, review, reply, concern, draft, viewing, copied,
                 history, corrections, disconnects>>

CompleteDisconnect ==
  /\ channel = "Disconnected"
  /\ channel' = "Ended"
  /\ Ev("the disconnect completes", "Practice")
  /\ UNCHANGED <<access, link, inbox, read, settled, review, reply, concern, draft, viewing, copied,
                 history, corrections, disconnects>>

-----------------------------------------------------------------------------
(* The review.                                                              *)

Keep == <<access, link, channel, inbox, read, history, corrections, disconnects>>

ReviewReads(p) ==
  /\ review[p].on /\ link = p /\ review[p].read # read
  /\ review' = [review EXCEPT ![p].read = read]
  /\ Ev("Coda's review reads the conversation", Coda)
  /\ UNCHANGED <<Keep, settled, reply, concern, draft, viewing, copied>>

ReviewRepairs(p) ==
  /\ review[p].on /\ link # p
  /\ review' = [review EXCEPT ![p] = [on |-> FALSE, read |-> {}]]
  /\ Ev("Coda repairs what relied on it", Coda)
  /\ UNCHANGED <<Keep, settled, reply, concern, draft, viewing, copied>>

(* The review finishes on what it read. It may find that a practice message it read answered the  *)
(* patient's messages before it: those are handled, citing that message. For any left, it drafts  *)
(* or updates the reply, or decides no reply would help. It may raise a concern. A withheld reply  *)
(* is redrafted or closed, and so is a withheld concern. An update leaves each physician's own     *)
(* draft as it is, with Coda's update offered beside it.                                           *)
ReviewFinishes(p) ==
  /\ review[p].on /\ link = p /\ review[p].read = read
  /\ \E cite \in {0} \cup {j \in read : Answer(j)} :
     \E propose \in BOOLEAN :
     \E flag \in (IF MayFlag THEN BOOLEAN ELSE {FALSE}) :
     LET answered == IF cite = 0 THEN {} ELSE {i \in Open : i < cite}
         left == Open \ answered
         drafts == propose /\ left # {}
         s1 == Settle(settled, answered, "Practice answered", Review, cite)
     IN /\ cite # 0 => answered # {} /\ (left = {} \/ propose)
        /\ settled' = IF drafts THEN s1 ELSE Settle(s1, left, "No reply needed", Review, 0)
        /\ reply' = CASE drafts /\ reply.open ->
                           [reply EXCEPT !.withheld = FALSE, !.basis = read, !.told = FALSE]
                      [] drafts ->
                           [open |-> TRUE, no |-> reply.no + 1, patient |-> p, withheld |-> FALSE,
                            basis |-> read, told |-> FALSE, cut |-> FALSE]
                      [] OTHER -> Closed(reply)
        /\ draft' = IF drafts /\ reply.open
                    THEN [d \in Physicians |->
                            IF On(d) /\ draft[d].text \in Theirs \ {"Revising"}
                            THEN [draft[d] EXCEPT !.behind = TRUE] ELSE draft[d]]
                    ELSE draft
        /\ concern' = CASE flag -> [state |-> "Open", closedBy |-> Nobody]
                        [] concern.state = "Withheld" -> [state |-> "Closed", closedBy |-> "Re-review"]
                        [] OTHER -> concern
        /\ viewing' = IF drafts THEN viewing ELSE [d \in Physicians |-> FALSE]
  /\ review' = [review EXCEPT ![p] = [on |-> FALSE, read |-> {}]]
  /\ Ev("Coda's review finishes", Review)
  /\ UNCHANGED <<Keep, copied>>

-----------------------------------------------------------------------------
(* Each physician, on the reply. *)

OpenReply(d) ==
  /\ CanSee(d) /\ ~viewing[d]
  /\ viewing' = [viewing EXCEPT ![d] = TRUE]
  /\ Ev("you open the reply", d)
  /\ UNCHANGED <<Keep, settled, review, reply, concern, draft, copied>>

(* The physician edits their own draft; editing during their revision stops it. *)
Edit(d) ==
  /\ viewing[d] /\ ~reply.withheld /\ Text(d) # "Yours"
  /\ Saves(d, "Yours", On(d) /\ draft[d].behind, TRUE)
  /\ Ev("you edit", d)
  /\ UNCHANGED <<Keep, settled, review, reply, concern, viewing, copied>>

(* Asking Coda to revise lets Coda replace this physician's text, and only theirs. *)
AskRevision(d) ==
  /\ viewing[d] /\ ~reply.withheld /\ Text(d) # "Revising"
  /\ Saves(d, "Revising", FALSE, On(d) /\ draft[d].mine)
  /\ Ev("you ask Coda to revise", d)
  /\ UNCHANGED <<Keep, settled, review, reply, concern, viewing, copied>>

RevisionFinishes(d) ==
  /\ On(d) /\ ~reply.withheld /\ draft[d].text = "Revising"
  /\ Saves(d, "Revised", FALSE, draft[d].mine)
  /\ Ev("Coda's revision lands", Coda)
  /\ UNCHANGED <<Keep, settled, review, reply, concern, viewing, copied>>

(* Coda's update replaces the physician's text only when they choose it. *)
TakeUpdate(d) ==
  /\ viewing[d] /\ Offered(d)
  /\ Saves(d, "Taken", FALSE, draft[d].mine)
  /\ Ev("you take Coda's update", d)
  /\ UNCHANGED <<Keep, settled, review, reply, concern, viewing, copied>>

(* Restore my version brings back the physician's own text. *)
Restore(d) ==
  /\ viewing[d] /\ ~reply.withheld /\ On(d) /\ draft[d].mine /\ draft[d].text # "Yours"
  /\ Saves(d, "Yours", draft[d].behind, TRUE)
  /\ Ev("you restore your version", d)
  /\ UNCHANGED <<Keep, settled, review, reply, concern, viewing, copied>>

(* Copying changes nothing about the reply; the other physicians see who copied it. *)
CopyReply(d) ==
  /\ viewing[d] /\ ~reply.withheld /\ Text(d) # "Revising"
  /\ copied' = [who |-> d, words |-> CardWords(d)]
  /\ Ev("you copy the reply", d)
  /\ UNCHANGED <<Keep, settled, review, reply, concern, draft, viewing>>

(* No reply needed, or Done while the channel is disconnected, closes the reply for everyone, with *)
(* a short Undo. A concern about the same messages stays.                                          *)
Close(d) ==
  /\ viewing[d] /\ ~reply.withheld
  /\ settled' = Settle(settled, Open, IF channel = "Connected" THEN "No reply needed" ELSE "Done", d, 0)
  /\ reply' = Closed(reply)
  /\ viewing' = [e \in Physicians |-> FALSE]
  /\ undo' = [kind |-> "close", by |-> d, before |-> <<reply, settled>>]
  /\ Ev(IF channel = "Connected" THEN "you mark No reply needed" ELSE "you mark it Done", d)
  /\ UNCHANGED <<Keep, review, concern, draft, copied>>

(* Undo puts the reply back; its card shows at once anything Coda read meanwhile. *)
UndoClose(d) ==
  /\ undo.kind = "close" /\ undo.by = d
  /\ reply' = LET r == undo.before[1]
              IN [r EXCEPT !.told = r.told \/ (PatientRead \ r.basis # {})]
  /\ settled' = undo.before[2]
  /\ undo' = NoUndo
  /\ Ev("you undo No reply needed", d)
  /\ UNCHANGED <<Keep, review, concern, draft, viewing, copied>>

(* Undo puts the correction back. Anyone who may not access the patient it returns to loses the *)
(* reply, as with the move itself.                                                                *)
UndoCorrection ==
  /\ undo.kind = "correction"
  /\ link' = undo.before[1]
  /\ review' = undo.before[2]
  /\ reply' = undo.before[3]
  /\ concern' = undo.before[4]
  /\ settled' = undo.before[5]
  /\ viewing' = [x \in Physicians |-> viewing[x] /\ undo.before[1] \in access[x]]
  /\ undo' = NoUndo
  /\ Ev("a physician undoes the move", "A physician")
  /\ UNCHANGED <<access, channel, inbox, read, draft, copied, history, corrections, disconnects>>

UndoExpires ==
  /\ undo.kind # ""
  /\ undo' = NoUndo
  /\ Ev("Undo expires", Nobody)
  /\ UNCHANGED <<World, inbox>>

ConcernDone(d) ==
  /\ concern.state = "Open" /\ link \in access[d]
  /\ concern' = [state |-> "Closed", closedBy |-> "Done"]
  /\ Ev("you mark the concern Done", d)
  /\ UNCHANGED <<Keep, settled, review, reply, draft, viewing, copied>>

-----------------------------------------------------------------------------

Plain ==
  \/ PatientWrites \/ PracticeSends \/ CodaReads
  \/ Disconnect \/ Reconnect \/ CompleteDisconnect
  \/ \E p \in Patients : ReviewReads(p) \/ ReviewRepairs(p) \/ ReviewFinishes(p)
  \/ \E d \in Physicians : OpenReply(d) \/ Edit(d) \/ AskRevision(d) \/ RevisionFinishes(d)
                           \/ TakeUpdate(d) \/ Restore(d) \/ CopyReply(d) \/ ConcernDone(d)

\* What Undo would restore. Undo ends when any of it changes, and when Coda reads a new message,
\* as it does in Arrival.tla, so Undo never rolls back a review that message woke.
Restorable == <<link, reply, concern, settled, read>>

Busy ==
  \/ Plain /\ undo' = IF Restorable' = Restorable THEN undo ELSE NoUndo
  \/ \E q \in Patients \cup {NotPatient} : Correct(q)
  \/ \E d \in Physicians : Close(d) \/ UndoClose(d)
  \/ UndoCorrection
  \/ UndoExpires

\* Everything that could happen has happened.
Rest == ~ENABLED Busy /\ UNCHANGED vars

Next == Busy \/ Rest

Spec == Init /\ [][Next]_vars

Symmetry == Permutations(Patients) \cup Permutations(Physicians)

-----------------------------------------------------------------------------
(* The rules, in the README's numbering.                                   *)

\* 1. A physician sees a conversation, its reply or their draft only while they may access its patient.
OnlyWithAccess == \A d \in Physicians : viewing[d] => CanSee(d)

\* 6. Only a review that read the practice's message closes a reply as answered, and an automatic
\* reply never answers it.
OnlyTheReviewAnswers == \A i \in 1..MaxItems :
  settled[i].why = "Practice answered" =>
    /\ settled[i].by = Review
    /\ settled[i].cites \in read /\ settled[i].cites > i
    /\ Answer(settled[i].cites)

\* 7. Closing a reply never closes a concern about the same messages, unless the conversation moves to
\* not a patient.
ConcernStays == [][(reply.open /\ ~reply'.open /\ link' # NotPatient)
                     => (concern.state = "Open" => concern'.state = "Open")]_vars

\* 8. Coda never replaces a physician's draft unless they ask it to revise or choose its update, and
\* one physician never changes another's draft.
YourWords == [][\A d \in Physicians :
  <<draft'[d].of, draft'[d].text>> # <<draft[d].of, draft[d].text>> =>
    \/ event'.who = d
    \/ draft[d].text = "Revising" /\ draft'[d].text = "Revised"]_vars

\* 9. Copy reply copies exactly the text the physician sees.
CopiesWhatYouSee == [][(copied'.who # Nobody /\ copied' # copied) => copied'.words = CardWords(copied'.who)]_vars

\* 10. The card shows every patient message linked after its reply was drafted.
CurrentReply == reply.open => (PatientRead \subseteq reply.basis \/ reply.told)

\* 12. What the patient's history says the practice sent is what the channel shows it sent.
WhatWasSent == \A h \in history : h.words = inbox[h.item].words
=============================================================================

------------------------------ MODULE Messaging ------------------------------
(***************************************************************************)
(* How a patient's message reaches the right patient, and how its reply   *)
(* works until the practice has answered: the design in README.md, from   *)
(* intent/physician.md P3. Each rule in the README is one INVARIANT or    *)
(* PROPERTY here.                                                          *)
(*                                                                         *)
(* The channel owns the conversation. Coda reads it a moment late and     *)
(* never writes to it. The link owns which patient the conversation       *)
(* belongs to; each patient's review owns whether a reply would help and  *)
(* what it says; each physician owns their draft. Today is read from      *)
(* these through each physician's current access, never stored.           *)
(*                                                                         *)
(* Whether mail is from a patient, whether a reply would help, whether to *)
(* flag a concern, and whether the practice answered the patient are the  *)
(* review's judgment; here each is a choice the review may make. `event`  *)
(* names who acted and why; VIEW leaves it out.                           *)
(***************************************************************************)
EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS
  Patients,
  Physicians,
  Manager,             \* the messaging manager, who corrects a confirmed link
  Restricted, Panel,   \* a Restricted physician may access only the patients in Panel
  Repliers,            \* who can reply in the conversation, by the channel's rules
  ChannelIdentifies,   \* the channel can tie the conversation to exactly one patient
  MayBeUnreadable,     \* a patient message can arrive that Coda can't read
  MayFlag,             \* the review can raise a concern about the messages
  MaxPatientMessages, MaxPracticeMessages, MaxCorrections, MaxDisconnects

Nobody == "Nobody"
NotPatient == "Not a patient"
Access(d) == IF d \in Restricted THEN Panel ELSE Patients
Coda == "Coda"

\* What the practice sends: an answer in words someone chose, or an automatic reply.
CodaDraft == "Coda's draft"
YourEdit == "Your edit"
OwnWords == "Words written in the channel"
AutoReply == "An automatic reply"
NoWords == "Nothing"

\* A draft's text: Coda's (it follows the reply), the physician's own, or a revision they asked for.
Theirs == {"Yours", "Revised"}

\* Each channel item and each correction is one event; a review it starts is counted per patient.
ItemKey(i) == "Item " \o ToString(i)
CorrectionKey(k) == "Correction " \o ToString(k)
MaxItems == MaxPatientMessages + MaxPracticeMessages
Events == {ItemKey(i) : i \in 1..MaxItems} \cup {CorrectionKey(k) : k \in 1..MaxCorrections}

NoReply == [open |-> FALSE, patient |-> Nobody, withheld |-> FALSE, basis |-> 0, told |-> FALSE]
NoDraft == [on |-> FALSE, text |-> "Coda's", behind |-> FALSE]
NoCopy == [who |-> Nobody, words |-> NoWords]

VARIABLES
  channel,     \* "Connected" or "Disconnected"
  inbox,       \* the conversation as the channel holds it, in order
  says,        \* the patient the channel ties the conversation to, or Nobody; fixed per run
  filed,       \* how many of the channel's items Coda has filed
  link,        \* the patient the conversation is linked to, Nobody, or NotPatient
  match,       \* each filed patient message: "Waiting", "Unlinked", "Linked" or "Filtered"
  handled,     \* patient messages that are settled
  review,      \* each patient's review: running, and how many items it has read
  reply,       \* the conversation's one reply, shared by everyone who may see it
  concern,     \* a concern about the messages: "None", "Open" or "Withheld"
  draft,       \* each physician's own draft of the reply
  viewing,     \* which physicians have the reply open
  copied,      \* who last copied the reply and what, which the others see
  history,     \* what the patient's history says the practice sent, and which answered the patient
  starts,      \* how many reviews each event started for each patient
  corrections, disconnects, event

state == <<channel, inbox, says, filed, link, match, handled, review, reply, concern, draft,
           viewing, copied, history, starts, corrections, disconnects>>
vars == <<state, event>>

Patient(i) == inbox[i].from = "Patient"
Answer(i) == inbox[i].from = "Practice" /\ inbox[i].words \notin {AutoReply}
PatientCount == Cardinality({i \in 1..Len(inbox) : Patient(i)})
PracticeCount == Len(inbox) - PatientCount
LastPatient == LET ps == {i \in 1..filed : Patient(i)} IN
                 IF ps = {} THEN 0 ELSE CHOOSE i \in ps : \A j \in ps : j <= i
\* Filed patient messages of the linked patient that nobody has settled.
Open == {i \in 1..filed : Patient(i) /\ match[i] = "Linked" /\ i \notin handled}
OpenReadable == {i \in Open : inbox[i].readable}

Init ==
  /\ channel = "Connected"
  /\ inbox = <<>>
  /\ says \in (IF ChannelIdentifies THEN Patients \cup {Nobody} ELSE {Nobody})
  /\ filed = 0
  /\ link = Nobody
  /\ match = [i \in 1..MaxItems |-> "Not filed"]
  /\ handled = {}
  /\ review = [p \in Patients |-> [on |-> FALSE, read |-> 0]]
  /\ reply = NoReply
  /\ concern = "None"
  /\ draft = [d \in Physicians |-> NoDraft]
  /\ viewing = [d \in Physicians |-> FALSE]
  /\ copied = NoCopy
  /\ history = {}
  /\ starts = [e \in Events, p \in Patients |-> 0]
  /\ corrections = 0
  /\ disconnects = 0
  /\ event = [who |-> Nobody, why |-> "start", cites |-> 0]

Did(who, why) == event' = [who |-> who, why |-> why, cites |-> 0]

\* An event wakes a patient's review. A review already running takes the news in when it reads
\* again, so the event starts nothing new.
Woken(rv, p) == IF rv[p].on THEN rv ELSE [rv EXCEPT ![p] = [on |-> TRUE, read |-> 0]]
Counted(rv, st, p, e) == IF rv[p].on THEN st ELSE [st EXCEPT ![<<e, p>>] = @ + 1]
Wake(p, e) == review' = Woken(review, p) /\ starts' = Counted(review, starts, p, e)

\* A physician sees the reply while they may access its patient and can reply in the conversation.
CanSee(d) == reply.open /\ reply.patient \in Access(d) /\ d \in Repliers
\* What the physician's card shows: their own draft of this reply, or Coda's.
Text(d) == IF draft[d].on THEN draft[d].text ELSE "Coda's"
\* Coda's update is offered beside a physician's own text that came before it.
Offered(d) == draft[d].on /\ draft[d].text \in Theirs /\ draft[d].behind

Quiet == <<channel, inbox, says, corrections, disconnects>>

-----------------------------------------------------------------------------
(* The world: the patient writes and the practice sends, in the channel.  *)

PatientWrites ==
  /\ PatientCount < MaxPatientMessages
  /\ \E unreadable \in (IF MayBeUnreadable THEN BOOLEAN ELSE {FALSE}) :
       inbox' = Append(inbox, [from |-> "Patient", readable |-> ~unreadable, words |-> NoWords])
  /\ UNCHANGED <<channel, says, filed, link, match, handled, review, reply, concern, draft, viewing,
                 copied, history, starts, corrections, disconnects>>
  /\ Did("The patient", "writes")

(* Someone at the practice sends in the conversation: the reply they copied, edited or not, their  *)
(* own words, or the channel's automatic reply. Coda never sends.                                   *)
PracticeSends ==
  /\ PracticeCount < MaxPracticeMessages
  /\ \E w \in {OwnWords, AutoReply} \cup ({copied.words} \ {NoWords}) :
       inbox' = Append(inbox, [from |-> "Practice", readable |-> TRUE, words |-> w])
  /\ UNCHANGED <<channel, says, filed, link, match, handled, review, reply, concern, draft, viewing,
                 copied, history, starts, corrections, disconnects>>
  /\ Did("The practice", "sends in the channel")

-----------------------------------------------------------------------------
(* Coda files the channel's next item, in order, while it can read it. A  *)
(* patient message routes when the conversation is linked or the channel  *)
(* ties it to one patient, and otherwise waits for a person. Once linked  *)
(* it wakes the patient's review, and an open reply says at once that the *)
(* patient wrote again. A practice message joins the patient's history as *)
(* sent and wakes the review; it settles nothing itself.                  *)
Files ==
  /\ channel = "Connected"
  /\ filed < Len(inbox)
  /\ LET i == filed + 1
         to == IF link # Nobody THEN link ELSE says
     IN /\ filed' = i
        /\ IF Patient(i)
           THEN CASE to \in Patients ->
                       /\ link' = to
                       /\ match' = [match EXCEPT ![i] = "Linked"]
                       /\ Wake(to, ItemKey(i))
                       /\ reply' = IF reply.open THEN [reply EXCEPT !.told = TRUE] ELSE reply
                       /\ UNCHANGED <<handled, history>>
                       /\ Did(Coda, "files the message to the patient")
                  [] to = NotPatient ->
                       /\ match' = [match EXCEPT ![i] = "Filtered"]
                       /\ handled' = handled \cup {i}
                       /\ UNCHANGED <<link, review, starts, reply, history>>
                       /\ Did(Coda, "files the message as not a patient's")
                  [] OTHER ->
                       /\ match' = [match EXCEPT ![i] = "Waiting"]
                       /\ UNCHANGED <<link, review, starts, reply, handled, history>>
                       /\ Did(Coda, "files the message to Messages to match")
           ELSE IF link \in Patients
                THEN /\ history' = history \cup {[patient |-> link, item |-> i, words |-> inbox[i].words, answered |-> FALSE]}
                     /\ Wake(link, ItemKey(i))
                     /\ UNCHANGED <<link, match, handled, reply>>
                     /\ Did(Coda, "sees what the practice sent")
                ELSE /\ UNCHANGED <<link, match, handled, review, starts, reply, history>>
                     /\ Did(Coda, "files what the practice sent, waiting with the conversation")
  /\ UNCHANGED <<Quiet, concern, draft, viewing, copied>>

(* A physician confirms a waiting message for a patient they may access; the messaging manager   *)
(* for any patient. The conversation is linked, its waiting messages join that patient, the      *)
(* practice's earlier messages join the history, and the review wakes once.                      *)
Confirm(d, p) ==
  /\ p \in Access(d) \/ d = Manager
  /\ \E i \in 1..filed : match[i] = "Waiting"
  /\ LET first == CHOOSE i \in 1..filed : match[i] = "Waiting" /\ \A j \in 1..(i - 1) : match[j] # "Waiting"
     IN /\ link' = p
        /\ match' = [i \in DOMAIN match |-> IF match[i] = "Waiting" THEN "Linked" ELSE match[i]]
        /\ history' = {[patient |-> p, item |-> j, words |-> inbox[j].words, answered |-> FALSE] : j \in {j \in 1..filed : ~Patient(j)}}
        /\ Wake(p, ItemKey(first))
  /\ UNCHANGED <<Quiet, filed, handled, reply, concern, draft, viewing, copied>>
  /\ Did(d, "confirms the patient")

(* A person leaves a waiting message unlinked. That covers only this message. *)
LeaveUnlinked(d) ==
  /\ \E i \in 1..filed :
       /\ match[i] = "Waiting"
       /\ match' = [match EXCEPT ![i] = "Unlinked"]
       /\ handled' = handled \cup {i}
  /\ UNCHANGED <<Quiet, filed, link, review, starts, reply, concern, draft, viewing, copied, history>>
  /\ Did(d, "leaves the message unlinked")

(* The messaging manager moves the conversation to another patient, or to not a patient. Nothing   *)
(* of it stays with the patient it left: their review lets go and repairs what relied on it, and   *)
(* history moves. The reply and a concern are withheld for the new patient's review to redraft or  *)
(* close; each physician's draft stays theirs and never moves to the new reply. Anyone who may not *)
(* access the new patient loses the reply. One correction starts one review for each patient.      *)
Correct(q) ==
  /\ link \in Patients
  /\ q \in (Patients \cup {NotPatient}) \ {link}
  /\ corrections < MaxCorrections
  /\ LET old == link
         e == CorrectionKey(corrections + 1)
         letGo == [review EXCEPT ![old] = [on |-> TRUE, read |-> 0]]
         st1 == Counted(review, starts, old, e)
     IN /\ link' = q
        /\ review' = IF q \in Patients THEN Woken(letGo, q) ELSE letGo
        /\ starts' = IF q \in Patients THEN Counted(letGo, st1, q, e) ELSE st1
        /\ history' = {[h EXCEPT !.patient = q] : h \in history}
        /\ IF q \in Patients
           THEN /\ reply' = IF reply.open THEN [reply EXCEPT !.patient = q, !.withheld = TRUE] ELSE reply
                /\ concern' = IF concern = "None" THEN "None" ELSE "Withheld"
                /\ UNCHANGED <<match, handled>>
           ELSE /\ reply' = NoReply
                /\ concern' = "None"
                /\ match' = [i \in DOMAIN match |-> IF match[i] = "Linked" THEN "Filtered" ELSE match[i]]
                /\ handled' = handled \cup Open
        /\ draft' = [d \in Physicians |-> [draft[d] EXCEPT !.on = FALSE]]
        /\ viewing' = [d \in Physicians |-> viewing[d] /\ q \in Access(d)]
        /\ corrections' = corrections + 1
  /\ UNCHANGED <<channel, inbox, says, filed, copied, disconnects>>
  /\ Did(Manager, "corrects the patient")

-----------------------------------------------------------------------------
(* The review.                                                              *)

(* The review reads the whole linked conversation as it stands, and reads again if something new  *)
(* was filed before it finishes.                                                                    *)
ReviewReads(p) ==
  /\ review[p].on /\ link = p /\ review[p].read < filed
  /\ review' = [review EXCEPT ![p].read = filed]
  /\ UNCHANGED <<Quiet, filed, link, match, handled, starts, reply, concern, draft, viewing, copied, history>>
  /\ Did(Coda, "reads the conversation and the chart")

(* A review of a patient the conversation left repairs what relied on it and ends. *)
ReviewRepairs(p) ==
  /\ review[p].on /\ link # p
  /\ review' = [review EXCEPT ![p] = [on |-> FALSE, read |-> 0]]
  /\ UNCHANGED <<Quiet, filed, link, match, handled, starts, reply, concern, draft, viewing, copied, history>>
  /\ Did(Coda, "repairs the patient the conversation left")

(* The review finishes on what it read. It may find that a practice message it read answered the   *)
(* patient's messages before it: those settle, citing that message. For any messages left, it      *)
(* drafts or updates the reply, or decides no reply would help. It may raise a concern. A withheld *)
(* reply is redrafted or closed, and so is a withheld concern.                                     *)
ReviewFinishes(p) ==
  /\ review[p].on /\ link = p /\ review[p].read = filed
  /\ \E cite \in {0} \cup {j \in 1..review[p].read : Answer(j)} :
     \E propose \in BOOLEAN :
     \E flag \in (IF MayFlag THEN BOOLEAN ELSE {FALSE}) :
     LET answered == IF cite = 0 THEN {} ELSE {i \in Open : i < cite}
         left == OpenReadable \ answered
         drafts == propose /\ left # {}
     IN /\ cite # 0 => answered # {} /\ (left = {} \/ propose)
        /\ handled' = handled \cup answered \cup (IF drafts THEN {} ELSE left)
        /\ reply' = IF drafts
                    THEN IF reply.open
                         THEN [reply EXCEPT !.withheld = FALSE, !.basis = filed, !.told = FALSE]
                         ELSE [open |-> TRUE, patient |-> p, withheld |-> FALSE, basis |-> filed, told |-> FALSE]
                    ELSE NoReply
        /\ concern' = CASE flag -> "Open"
                        [] concern = "Withheld" -> "None"
                        [] OTHER -> concern
        /\ draft' = IF drafts /\ reply.open
                    THEN [d \in Physicians |-> IF draft[d].on /\ draft[d].text \in Theirs
                                               THEN [draft[d] EXCEPT !.behind = TRUE] ELSE draft[d]]
                    ELSE [d \in Physicians |-> [draft[d] EXCEPT !.on = FALSE]]
        /\ viewing' = IF drafts THEN viewing ELSE [d \in Physicians |-> FALSE]
        /\ history' = {IF h.item = cite THEN [h EXCEPT !.answered = TRUE] ELSE h : h \in history}
        /\ event' = [who |-> Coda,
                     why |-> CASE cite # 0 -> "finds the practice answered"
                               [] drafts -> "drafts the reply"
                               [] OTHER -> "decides no reply is needed",
                     cites |-> cite]
  /\ review' = [review EXCEPT ![p] = [on |-> FALSE, read |-> 0]]
  /\ UNCHANGED <<Quiet, filed, link, match, starts, copied>>

-----------------------------------------------------------------------------
(* Each physician, on the reply. *)

Keep == <<Quiet, filed, link, match, review, starts, history>>

OpenReply(d) ==
  /\ CanSee(d) /\ ~viewing[d]
  /\ viewing' = [viewing EXCEPT ![d] = TRUE]
  /\ UNCHANGED <<Keep, handled, reply, concern, draft, copied>>
  /\ Did(d, "opens the reply")

(* The physician edits their own draft; editing during their revision stops it. *)
Edit(d) ==
  /\ viewing[d] /\ ~reply.withheld /\ Text(d) # "Yours"
  /\ draft' = [draft EXCEPT ![d] = [on |-> TRUE, text |-> "Yours", behind |-> draft[d].on /\ draft[d].behind]]
  /\ UNCHANGED <<Keep, handled, reply, concern, viewing, copied>>
  /\ Did(d, "edits their draft")

(* Asking Coda to revise lets Coda replace this physician's text, and only theirs. *)
AskRevision(d) ==
  /\ viewing[d] /\ ~reply.withheld /\ Text(d) # "Revising"
  /\ draft' = [draft EXCEPT ![d] = [on |-> TRUE, text |-> "Revising", behind |-> FALSE]]
  /\ UNCHANGED <<Keep, handled, reply, concern, viewing, copied>>
  /\ Did(d, "asks Coda to revise their draft")

(* The revision finishes on the conversation as it stands. *)
RevisionFinishes(d) ==
  /\ reply.open /\ ~reply.withheld /\ draft[d].on /\ draft[d].text = "Revising"
  /\ draft' = [draft EXCEPT ![d] = [on |-> TRUE, text |-> "Revised", behind |-> FALSE]]
  /\ UNCHANGED <<Keep, handled, reply, concern, viewing, copied>>
  /\ Did(d, "gets Coda's revision of their draft")

(* Coda's update replaces the physician's text only when they choose it. *)
TakeUpdate(d) ==
  /\ viewing[d] /\ Offered(d)
  /\ draft' = [draft EXCEPT ![d] = NoDraft]
  /\ UNCHANGED <<Keep, handled, reply, concern, viewing, copied>>
  /\ Did(d, "takes Coda's update")

(* Copying changes nothing about the reply; the other physicians see who copied it. *)
CopyReply(d) ==
  /\ viewing[d] /\ ~reply.withheld /\ Text(d) # "Revising"
  /\ copied' = [who |-> d, words |-> IF Text(d) = "Coda's" THEN CodaDraft ELSE YourEdit]
  /\ UNCHANGED <<Keep, handled, reply, concern, draft, viewing>>
  /\ Did(d, "copies the reply")

(* No reply needed, or Done while the channel is disconnected, closes the reply for everyone. A   *)
(* concern about the same messages stays.                                                          *)
Close(d) ==
  /\ viewing[d] /\ ~reply.withheld
  /\ handled' = handled \cup OpenReadable
  /\ reply' = NoReply
  /\ draft' = [e \in Physicians |-> [draft[e] EXCEPT !.on = FALSE]]
  /\ viewing' = [e \in Physicians |-> FALSE]
  /\ UNCHANGED <<Keep, concern, copied>>
  /\ Did(d, "marks No reply needed")

ConcernDone(d) ==
  /\ concern = "Open" /\ link \in Access(d)
  /\ concern' = "None"
  /\ UNCHANGED <<Keep, handled, reply, draft, viewing, copied>>
  /\ Did(d, "marks the concern Done")

(* A message Coda can't read waits on Today until someone marks it Done. *)
DoneUnreadable(d) ==
  /\ link \in Access(d)
  /\ \E i \in Open : ~inbox[i].readable /\ handled' = handled \cup {i}
  /\ UNCHANGED <<Keep, reply, concern, draft, viewing, copied>>
  /\ Did(d, "marks the unreadable message Done")

-----------------------------------------------------------------------------
(* The practice disconnects the channel and reconnects it. Replies and     *)
(* drafts stay; Coda files nothing while disconnected.                     *)
Disconnect ==
  /\ channel = "Connected" /\ disconnects < MaxDisconnects
  /\ channel' = "Disconnected"
  /\ disconnects' = disconnects + 1
  /\ UNCHANGED <<inbox, says, filed, link, match, handled, review, reply, concern, draft, viewing,
                 copied, history, starts, corrections>>
  /\ Did("The practice", "disconnects the channel")

Reconnect ==
  /\ channel = "Disconnected"
  /\ channel' = "Connected"
  /\ UNCHANGED <<inbox, says, filed, link, match, handled, review, reply, concern, draft, viewing,
                 copied, history, starts, corrections, disconnects>>
  /\ Did("The practice", "reconnects the channel")

Busy ==
  \/ PatientWrites \/ PracticeSends \/ Files
  \/ \E d \in Physicians : \E p \in Patients : Confirm(d, p)
  \/ \E d \in Physicians : LeaveUnlinked(d)
  \/ \E q \in Patients \cup {NotPatient} : Correct(q)
  \/ \E p \in Patients : ReviewReads(p) \/ ReviewRepairs(p) \/ ReviewFinishes(p)
  \/ \E d \in Physicians : OpenReply(d) \/ Edit(d) \/ AskRevision(d) \/ RevisionFinishes(d)
                           \/ TakeUpdate(d) \/ CopyReply(d) \/ Close(d) \/ ConcernDone(d)
                           \/ DoneUnreadable(d)
  \/ Disconnect \/ Reconnect

\* Everything that could happen has happened.
Rest == ~ENABLED Busy /\ UNCHANGED vars

Next == Busy \/ Rest

Spec == Init /\ [][Next]_vars

-----------------------------------------------------------------------------
(* The rules, in the README's order.                                        *)

\* A physician sees a conversation, its reply and their draft only while they may access its patient.
OnlyWithAccess == \A d \in Physicians : viewing[d] => CanSee(d)

\* A conversation reaches only the patient it's linked to: once it moves, nothing of it stays
\* readable on the patient it left.
RightPatient ==
  /\ \A p \in Patients \ {link} : review[p].read = 0
  /\ reply.open => reply.patient = link
  /\ \A h \in history : h.patient = link

\* Until it's settled, every patient message Coda filed shows to someone who can act on it: waiting
\* to match, or, to a physician who may access its patient, unreadable, being reviewed, or on the
\* reply.
NeverInvisible == \A i \in 1..filed :
  (Patient(i) /\ i \notin handled) =>
    \/ match[i] = "Waiting"
    \/ /\ match[i] = "Linked"
       /\ \E d \in Physicians :
            /\ link \in Access(d)
            /\ \/ ~inbox[i].readable
               \/ review[link].on
               \/ CanSee(d)

\* Only a review that read the practice's message closes a reply as answered. Copying, opening, an
\* automatic reply or an acknowledgment never does: a message settles only by a person, or by a review
\* that decides no reply is needed or cites a practice answer, after the message, that it read.
OnlyTheReviewAnswers == [][\A i \in handled' \ handled :
    \/ event'.who \in Physicians \cup {Manager}
    \/ /\ event'.who = Coda
       /\ event'.why = "decides no reply is needed"
    \/ /\ event'.who = Coda
       /\ event'.why = "finds the practice answered"
       /\ Answer(event'.cites) /\ i < event'.cites /\ event'.cites <= filed
    \/ /\ event'.who = Coda
       /\ event'.why = "files the message as not a patient's"]_vars

\* Closing a reply never closes a concern about the same messages.
ConcernStays == [][concern = "Open" /\ concern' # "Open" =>
                    \/ event'.why = "marks the concern Done"
                    \/ event'.why = "corrects the patient"]_vars

\* Coda never replaces a physician's draft unless they choose its update, and one physician never
\* changes another's draft.
YourWords == [][\A d \in Physicians :
                 draft[d].text \in Theirs /\ draft'[d].text # draft[d].text => event'.who = d]_vars

\* A reply never misses a message Coda filed after it without the card saying so.
CurrentReply == (reply.open /\ ~reply.withheld) => (reply.basis >= LastPatient \/ reply.told)

\* One message, practice reply or correction starts at most one review for each patient it affects.
OneReview == \A e \in Events, p \in Patients : starts[<<e, p>>] <= 1

\* What the patient's history says the practice sent is what the channel shows it sent.
WhatWasSent == \A h \in history : h.words = inbox[h.item].words
=============================================================================

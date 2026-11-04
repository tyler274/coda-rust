------------------------------ MODULE Messaging ------------------------------
(***************************************************************************)
(* How a patient's message reaches the right patient, and how its reply  *)
(* card works until the practice has answered: the design drawn from      *)
(* README.md and intent/physician.md P3. Each INVARIANT is one rule. Each *)
(* step's comment names the spec lines it carries out.                    *)
(*                                                                         *)
(* The channel is the practice's messaging service: Spruce in the MVP,    *)
(* any channel after (physician Principle 10, Scope: Later). It owns the  *)
(* conversation; Coda reads it a moment late and never writes to it       *)
(* (P3.1, P3.18, Rules: Authority and sending). Patients write and the    *)
(* practice sends in the channel whenever they do. Coda files each item   *)
(* in order, links the conversation to one patient, wakes that patient's  *)
(* review, and keeps one reply card for the conversation's unanswered     *)
(* messages. Today is not stored: what each physician sees is read from   *)
(* the link, the card, the reviews and the channel, through the           *)
(* physician's current access (PRINCIPLES 11; physician Today).           *)
(*                                                                         *)
(* What differs between channels is configuration: who can reply in the  *)
(* conversation (Repliers, P3.13), whether the channel can say the        *)
(* conversation is exactly one patient's (ChannelIdentifies, P3.7), and   *)
(* who corrects a link (Manager, P3 Wrong patient; DECISIONS P2). The     *)
(* rules don't change with any of them.                                    *)
(*                                                                         *)
(* Whether mail is patient mail, what the review reads, and whether a     *)
(* reply would help are the model's judgment (P3.6, P3.12; PRINCIPLES 1,  *)
(* 4); here each is an outcome Coda may choose. Starting a run exactly    *)
(* once, retrying it and recognizing a delivery twice rest on stored      *)
(* records the README lists. `event` names the step just taken; VIEW      *)
(* leaves it out.                                                          *)
(***************************************************************************)
EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS
  Patients,            \* the patients the conversation could belong to
  Physicians,
  Manager,             \* the messaging manager, who corrects a confirmed link
  Restricted, Panel,   \* a Restricted physician may access only the patients in Panel
  Repliers,            \* who can reply in the conversation, by the channel's rules
  ChannelIdentifies,   \* the channel can say the conversation is exactly one patient's
  MayBeUnreadable,     \* a patient message can arrive that Coda can't read
  MaxPatientMessages, MaxReplies, MaxCorrections, MaxDisconnects

Nobody == "Nobody"
Access(d) == IF d \in Restricted THEN Panel ELSE Patients

\* What a sent reply says: the card's text the physician copied, or their own words in the channel.
CodaDraft == "Coda's draft"
YourEdit == "Your edit"
OwnWords == "Words written in the channel"
NoWords == "Nothing"

\* Each channel item and each correction is one event that can start a review.
ItemKey(i) == "Item " \o ToString(i)
CorrectionKey(k) == "Correction " \o ToString(k)
MaxItems == MaxPatientMessages + MaxReplies
Events == {ItemKey(i) : i \in 1..MaxItems} \cup {CorrectionKey(k) : k \in 1..MaxCorrections}

\* How the reply card shows. Its text is separate: "None", "Coda's" or "Yours".
Ready == "Ready"
Revising == "Coda is revising"
Withheld == "Withheld after a correction"
Disconnected == "Channel disconnected"
ClosedCard == [open |-> FALSE, patient |-> Nobody, state |-> Ready, text |-> "None",
               edited |-> FALSE, basis |-> 0, told |-> FALSE, offer |-> FALSE, offerBasis |-> 0]
NoViews == [d \in Physicians |-> FALSE]
NoCopies == [d \in Physicians |-> NoWords]

VARIABLES
  channel,     \* "Connected" or "Disconnected"
  inbox,       \* the conversation as the channel holds it, in order
  says,        \* the patient the channel says the conversation is, or Nobody; fixed per run
  filed,       \* how many of the channel's items Coda has filed
  link,        \* the patient the conversation is linked to, or Nobody
  match,       \* each filed patient message: "Waiting", "Unlinked" or "Linked"
  handled,     \* patient messages a person or Coda has settled
  review,      \* each patient's review: running, and how many items it has read
  card,        \* the conversation's reply card
  viewing,     \* which physicians have the card open
  copied,      \* what each physician last copied from the card
  history,     \* what the patient's history says the practice sent
  starts,      \* how many reviews each event started
  corrections, disconnects, event

state == <<channel, inbox, says, filed, link, match, handled, review, card, viewing, copied,
           history, starts, corrections, disconnects>>
vars == <<state, event>>

Patient(i) == inbox[i].from = "Patient"
PatientCount == Cardinality({i \in 1..Len(inbox) : Patient(i)})
ReplyCount == Cardinality({i \in 1..Len(inbox) : ~Patient(i)})
\* Filed patient messages linked to the conversation's patient and not yet settled.
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
  /\ card = ClosedCard
  /\ viewing = NoViews
  /\ copied = NoCopies
  /\ history = {}
  /\ starts = [e \in Events |-> 0]
  /\ corrections = 0
  /\ disconnects = 0
  /\ event = [flow |-> "start", says |-> "start"]

\* Every step names the README flow it carries out, and says what happened in the README's words.
Did(flow, line) == event' = [flow |-> flow, says |-> line]

\* An event wakes the patient's review. A review already running takes the news in when it reads
\* again, so the event starts nothing new (physician Patient review: one event starts one review;
\* if records change before it finishes, it looks again).
Woken(rv, p) == IF rv[p].on THEN rv ELSE [rv EXCEPT ![p] = [on |-> TRUE, read |-> 0]]
Counted(rv, st, p, e) == IF rv[p].on THEN st ELSE [st EXCEPT ![e] = @ + 1]
Wake(p, e) == review' = Woken(review, p) /\ starts' = Counted(review, starts, p, e)

\* The card closes for everyone at once (P3.13), and nothing copied from it stays offered.
CloseCard == card' = ClosedCard /\ viewing' = NoViews /\ copied' = NoCopies

-----------------------------------------------------------------------------
(* The world: the patient writes and the practice sends, in the channel.  *)

(* A patient writes. The channel takes it whether or not Coda is connected (P3.6). *)
PatientWrites ==
  /\ PatientCount < MaxPatientMessages
  /\ \E unreadable \in (IF MayBeUnreadable THEN BOOLEAN ELSE {FALSE}) :
       inbox' = Append(inbox, [from |-> "Patient", readable |-> ~unreadable, words |-> NoWords])
  /\ UNCHANGED <<channel, says, filed, link, match, handled, review, card, viewing, copied,
                 history, starts, corrections, disconnects>>
  /\ Did("F1", "the patient writes in the channel")

(* A physician who can reply in the conversation sends there: what they copied, edited or not,  *)
(* or words of their own. Coda never sends (P3.18; Rules: Authority and sending).              *)
PracticeSends(d) ==
  /\ d \in Repliers
  /\ ReplyCount < MaxReplies
  /\ \E w \in ({copied[d], OwnWords} \ {NoWords}) :
       /\ inbox' = Append(inbox, [from |-> "Practice", readable |-> TRUE, words |-> w])
       /\ Did("F18", d \o " sends a reply in the channel: " \o w)
  /\ UNCHANGED <<channel, says, filed, link, match, handled, review, card, viewing, copied,
                 history, starts, corrections, disconnects>>

-----------------------------------------------------------------------------
(* Coda files the channel's next item, in order, while connected (P3.3,      *)
(* P3.4; physician Engineering decisions: Mail).                             *)
(* A patient message routes on its own when the conversation is linked, or  *)
(* when the channel says it is exactly one patient's; otherwise it waits    *)
(* for a person (P3.7, P3.8, P3.9). Once linked it wakes the patient's      *)
(* review (P3.11), and an open card says at once that the patient wrote     *)
(* again (P3.17). A reply the practice sent goes into the patient's history *)
(* as sent, settles the messages before it, closes the card and wakes the   *)
(* review (P3.19). In a conversation not yet linked it stays with the       *)
(* conversation, which becomes evidence once linked (P3.11).                *)
Files ==
  /\ channel = "Connected"
  /\ filed < Len(inbox)
  /\ LET i == filed + 1
         item == inbox[i]
         to == IF link # Nobody THEN link ELSE says
         m == "message " \o ToString(Cardinality({j \in 1..i : Patient(j)}))
         flow == CASE Patient(i) /\ to # Nobody /\ card.open -> "F15"
                   [] Patient(i) /\ link # Nobody -> "F2"
                   [] Patient(i) /\ to # Nobody -> "F3"
                   [] Patient(i) /\ \E j \in 1..(i - 1) : match[j] = "Unlinked" -> "F7"
                   [] Patient(i) -> "F4"
                   [] link # Nobody /\ card.open /\ card.state = Disconnected -> "F25"
                   [] link # Nobody -> "F19"
                   [] OTHER -> "F6"
         words == CASE flow = "F15" -> "Coda files " \o m \o "; the card says the patient wrote again"
                    [] flow = "F2" -> "Coda files " \o m \o " to " \o to \o ", whose conversation it is"
                    [] flow = "F3" -> "Coda files " \o m \o " to " \o to \o ", whom the channel names"
                    [] flow \in {"F4", "F7"} -> "Coda files " \o m \o "; no sure match, so it waits in Messages to match"
                    [] flow = "F25" -> "Coda, reconnected, sees the reply sent: " \o item.words
                    [] flow = "F19" -> "Coda sees the reply sent: " \o item.words
                    [] OTHER -> "Coda files the practice's reply; it waits with the conversation"
     IN /\ filed' = i
        /\ Did(flow, words)
        /\ IF Patient(i)
           THEN IF to # Nobody
                THEN /\ link' = to
                     /\ match' = [match EXCEPT ![i] = "Linked"]
                     /\ Wake(to, ItemKey(i))
                     /\ card' = IF card.open THEN [card EXCEPT !.told = TRUE] ELSE card
                     /\ UNCHANGED <<handled, viewing, copied, history>>
                ELSE /\ match' = [match EXCEPT ![i] = "Waiting"]
                     /\ UNCHANGED <<link, review, starts, card, handled, viewing, copied, history>>
           ELSE IF link # Nobody
                THEN /\ history' = history \cup {[patient |-> link, item |-> i, words |-> item.words]}
                     /\ handled' = handled \cup {j \in 1..i : Patient(j) /\ match[j] = "Linked"}
                     /\ CloseCard
                     /\ Wake(link, ItemKey(i))
                     /\ UNCHANGED <<link, match>>
                ELSE UNCHANGED <<link, match, handled, review, starts, card, viewing, copied, history>>
  /\ UNCHANGED <<channel, inbox, says, corrections, disconnects>>

(* Any physician matches a waiting message from Messages to match: they confirm Coda's            *)
(* suggestion or pick another patient (P3.9; Rules: Access and privacy). The conversation is      *)
(* linked and its waiting messages join that patient (P3.8). The whole conversation becomes       *)
(* evidence, including replies the practice already sent there, so history records that the       *)
(* practice answered and the messages those replies answered are settled (P3.11, P3.19). The      *)
(* review wakes once.                                                                               *)
Confirm(p) ==
  /\ \E i \in 1..filed : match[i] = "Waiting"
  /\ LET first == CHOOSE i \in 1..filed : match[i] = "Waiting" /\ \A j \in 1..(i - 1) : match[j] # "Waiting"
         linked == [i \in DOMAIN match |-> IF match[i] = "Waiting" THEN "Linked" ELSE match[i]]
         replies == {j \in 1..filed : ~Patient(j)}
     IN /\ link' = p
        /\ match' = linked
        /\ history' = {[patient |-> p, item |-> j, words |-> inbox[j].words] : j \in replies}
        /\ handled' = handled \cup {i \in 1..filed : Patient(i) /\ linked[i] = "Linked"
                                                   /\ \E j \in replies : i < j}
        /\ Wake(p, ItemKey(first))
        /\ Did(IF replies = {} THEN "F5" ELSE "F6",
               "a physician confirms " \o p \o (IF replies = {} THEN "" ELSE "; the practice's earlier reply joins the history"))
  /\ UNCHANGED <<channel, inbox, says, filed, card, viewing, copied, corrections, disconnects>>

(* A person leaves a waiting message unlinked. That covers only this message (P3.9). *)
LeaveUnlinked ==
  /\ \E i \in 1..filed :
       /\ match[i] = "Waiting"
       /\ match' = [match EXCEPT ![i] = "Unlinked"]
       /\ handled' = handled \cup {i}
  /\ UNCHANGED <<channel, inbox, says, filed, link, review, starts, card, viewing, copied, history,
                 corrections, disconnects>>
  /\ Did("F7", "a physician leaves the message unlinked")

(* The messaging manager corrects the link: Wrong patient? The whole conversation moves, and later *)
(* messages route to the new patient (P3 Wrong patient). The old patient's review lets go of it at *)
(* once and ends with nothing left to review (P2 Nothing left to review). The card moves, withheld *)
(* until the new patient's review decides (P3 Wrong link after use), and closes for anyone who     *)
(* can't see the new patient (P3.2). History moves with it (Rules: Data and consent). One          *)
(* correction wakes one review, on the new patient (P3 Wrong patient).                              *)
Correct(q) ==
  /\ link # Nobody
  /\ q # link
  /\ corrections < MaxCorrections
  /\ LET old == link
         e == CorrectionKey(corrections + 1)
         letGo == [review EXCEPT ![old] = [on |-> @.on, read |-> 0]]
     IN /\ link' = q
        /\ review' = Woken(letGo, q)
        /\ starts' = Counted(letGo, starts, q, e)
        /\ history' = {[h EXCEPT !.patient = q] : h \in history}
        /\ card' = IF card.open THEN [card EXCEPT !.patient = q, !.state = Withheld, !.offer = FALSE] ELSE card
        /\ viewing' = [d \in Physicians |-> viewing[d] /\ q \in Access(d)]
        /\ corrections' = corrections + 1
  /\ UNCHANGED <<channel, inbox, says, filed, match, handled, copied, disconnects>>
  /\ Did("F22", "the messaging manager moves the conversation to " \o q)

-----------------------------------------------------------------------------
(* The review.                                                              *)

(* The review reads the whole linked conversation as it stands, including the practice's own    *)
(* replies (P3.11). It reads again whenever something new was filed before it finishes (P3.16;  *)
(* physician Patient review).                                                                     *)
ReviewReads(p) ==
  /\ review[p].on /\ link = p /\ review[p].read < filed
  /\ review' = [review EXCEPT ![p].read = filed]
  /\ UNCHANGED <<channel, inbox, says, filed, link, match, handled, starts, card, viewing, copied,
                 history, corrections, disconnects>>
  /\ Did("F8", "Coda reads " \o p \o "'s conversation and chart")

(* A review of a patient the conversation left has nothing left to review (P2 Nothing left to   *)
(* review).                                                                                       *)
ReviewLetsGo(p) ==
  /\ review[p].on /\ link # p
  /\ review' = [review EXCEPT ![p] = [on |-> FALSE, read |-> 0]]
  /\ UNCHANGED <<channel, inbox, says, filed, link, match, handled, starts, card, viewing, copied,
                 history, corrections, disconnects>>
  /\ Did("F22", p \o "'s review lets go of the conversation: nothing left to review")

(* The review proposes a reply (P3.12, P3.13). A new card opens with Coda's draft; while the      *)
(* channel is disconnected it opens without one (P3.5). An untouched draft updates in place; an    *)
(* edited one keeps the physician's text and offers Coda's update (P3.17). A card the disconnect   *)
(* emptied keeps waiting (P3.5); a revision in progress takes the news in itself (P3.16). A        *)
(* withheld card is redrafted (P3 Wrong link after use).                                            *)
Proposes(p) ==
  /\ OpenReadable # {}
  /\ card' = IF ~card.open
             THEN IF channel = "Connected"
                  THEN [ClosedCard EXCEPT !.open = TRUE, !.patient = p, !.text = "Coda's", !.basis = filed]
                  ELSE [ClosedCard EXCEPT !.open = TRUE, !.patient = p, !.state = Disconnected]
             ELSE CASE card.state \in {Disconnected, Revising} -> card
                    [] card.edited -> [card EXCEPT !.state = Ready, !.offer = TRUE, !.offerBasis = filed]
                    [] OTHER -> [card EXCEPT !.state = Ready, !.text = "Coda's", !.basis = filed,
                                             !.told = FALSE, !.offer = FALSE]
  /\ UNCHANGED <<handled, viewing, copied>>
  /\ Did(CASE ~card.open /\ channel = "Connected" -> "F9"
           [] ~card.open \/ card.state = Disconnected -> "F24"
           [] card.state = Revising -> "F13"
           [] card.state = Withheld -> "F23"
           [] card.edited -> "F16"
           [] OTHER -> "F15",
         CASE ~card.open /\ channel = "Connected" -> "Coda drafts a reply for " \o p
           [] ~card.open -> "Coda's review finds a reply would help; the card opens empty while the channel is disconnected"
           [] card.state = Disconnected -> "Coda's review finds a reply would help; the card stays empty until reconnected"
           [] card.state = Revising -> "Coda's review leaves the news to the revision in progress"
           [] card.state = Withheld /\ card.edited -> "the re-review keeps your edit and offers Coda's update"
           [] card.state = Withheld -> "the re-review redrafts the withheld reply"
           [] card.edited -> "Coda keeps your edit and offers its update"
           [] OTHER -> "Coda updates its untouched draft in place")

(* The review decides no reply would help: an acknowledgment, or a conversation already resolved, *)
(* settled with no row (P3.12). A card it withheld closes, unless the physician had edited it     *)
(* (P3 Wrong link after use; P3.17).                                                               *)
NoReply ==
  /\ ~card.open \/ (card.state = Withheld /\ ~card.edited)
  /\ handled' = handled \cup OpenReadable
  /\ IF card.open THEN CloseCard ELSE UNCHANGED <<card, viewing, copied>>
  /\ IF card.open THEN Did("F23", "the re-review closes the withheld reply: no reply needed")
                   ELSE Did("F10", "Coda decides no reply is needed")

ReviewFinishes(p) ==
  /\ review[p].on /\ link = p /\ review[p].read = filed
  /\ Proposes(p) \/ NoReply
  /\ review' = [review EXCEPT ![p] = [on |-> FALSE, read |-> 0]]
  /\ UNCHANGED <<channel, inbox, says, filed, link, match, starts, history, corrections, disconnects>>

(* A revision the physician asked for finishes on the conversation as it stands now (P3.16;       *)
(* physician Engineering decisions: Runs).                                                          *)
RevisionFinishes ==
  /\ card.open /\ card.state = Revising
  /\ card' = [card EXCEPT !.state = Ready, !.text = "Coda's", !.basis = filed, !.told = FALSE,
                          !.offer = FALSE]
  /\ UNCHANGED <<channel, inbox, says, filed, link, match, handled, review, starts, viewing, copied,
                 history, corrections, disconnects>>
  /\ Did("F13", "Coda finishes the revision on the conversation as it stands")

-----------------------------------------------------------------------------
(* The physician, on the card. One card per conversation, shared by         *)
(* everyone who may access the patient and can reply there (P3.13).         *)

CanSee(d) == card.open /\ card.patient \in Access(d) /\ d \in Repliers
Keep == <<channel, inbox, says, filed, link, match, review, starts, history, corrections, disconnects>>

(* The physician opens the reply from Today or the patient page (P3.14). *)
OpenCard(d) ==
  /\ CanSee(d) /\ ~viewing[d]
  /\ viewing' = [viewing EXCEPT ![d] = TRUE]
  /\ UNCHANGED <<Keep, handled, card, copied>>
  /\ Did("F11", d \o " opens the reply")

(* The physician edits the reply; editing during a revision cancels it and their text stands     *)
(* (P3.16; physician-site: a physician save makes a revision stale).                               *)
Edit(d) ==
  /\ viewing[d] /\ card.text # "None" /\ card.state \in {Ready, Revising}
  /\ ~(card.text = "Yours" /\ card.state = Ready)
  /\ card' = [card EXCEPT !.text = "Yours", !.edited = TRUE, !.state = Ready]
  /\ UNCHANGED <<Keep, handled, viewing, copied>>
  /\ IF card.state = Revising THEN Did("F14", d \o " edits during the revision, which stops")
                              ELSE Did("F12", d \o " edits the reply")

(* Asking Coda to revise is the physician's choice to let Coda replace the text (P3.16). *)
AskRevision(d) ==
  /\ viewing[d] /\ card.text # "None" /\ card.state = Ready
  /\ card' = [card EXCEPT !.state = Revising, !.edited = FALSE, !.offer = FALSE]
  /\ UNCHANGED <<Keep, handled, viewing, copied>>
  /\ Did("F13", d \o " asks Coda to revise")

(* Coda's update replaces the physician's text only when they choose it (P3.17). *)
TakeUpdate(d) ==
  /\ viewing[d] /\ card.offer
  /\ card' = [card EXCEPT !.text = "Coda's", !.edited = FALSE, !.offer = FALSE,
                          !.basis = card.offerBasis, !.told = card.offerBasis < filed]
  /\ UNCHANGED <<Keep, handled, viewing, copied>>
  /\ Did("F16", d \o " takes Coda's update")

(* Copy reply changes nothing Coda holds: the row stays until Coda sees the reply sent (P3.18,    *)
(* P3.19; Rules: silence or a finished run never proves approval).                                 *)
CopyReply(d) ==
  /\ viewing[d] /\ card.text # "None" /\ card.state = Ready
  /\ copied' = [copied EXCEPT ![d] = IF card.text = "Coda's" THEN CodaDraft ELSE YourEdit]
  /\ UNCHANGED <<Keep, handled, card, viewing>>
  /\ Did("F17", d \o " copies the reply")

(* No reply needed, or Done on a card the disconnect emptied, settles it for everyone (P3 No     *)
(* reply needed, P3.5, P3.13).                                                                     *)
Close(d) ==
  /\ viewing[d] /\ card.state \in {Ready, Disconnected}
  /\ handled' = handled \cup OpenReadable
  /\ CloseCard
  /\ UNCHANGED Keep
  /\ IF card.state = Disconnected THEN Did("F25", d \o " marks the empty card Done")
                                  ELSE Did("F20", d \o " marks No reply needed")

(* A message Coda can't read waits on Today with a link to the channel until someone marks it    *)
(* Done (P3 Message Coda can't read).                                                              *)
DoneUnreadable(d) ==
  /\ link \in Access(d)
  /\ \E i \in Open : ~inbox[i].readable /\ handled' = handled \cup {i}
  /\ UNCHANGED <<Keep, card, viewing, copied>>
  /\ Did("F21", d \o " marks the unreadable message Done")

-----------------------------------------------------------------------------
(* The practice disconnects the channel: reply drafts are deleted, and an   *)
(* open card stays without its draft, says the channel is disconnected, and *)
(* waits for Done or for Coda, reconnected, to see the reply (P3.5).        *)
Disconnect ==
  /\ channel = "Connected" /\ disconnects < MaxDisconnects
  /\ channel' = "Disconnected"
  /\ disconnects' = disconnects + 1
  /\ card' = IF card.open
             THEN [card EXCEPT !.state = Disconnected, !.text = "None", !.edited = FALSE,
                               !.told = FALSE, !.offer = FALSE]
             ELSE card
  /\ copied' = NoCopies
  /\ UNCHANGED <<inbox, says, filed, link, match, handled, review, starts, viewing, history, corrections>>
  /\ Did("F24", "the practice disconnects the channel")

Reconnect ==
  /\ channel = "Disconnected"
  /\ channel' = "Connected"
  /\ UNCHANGED <<inbox, says, filed, link, match, handled, review, starts, card, viewing, copied,
                 history, corrections, disconnects>>
  /\ Did("F25", "the practice reconnects the channel")

Busy ==
  \/ PatientWrites \/ (\E d \in Physicians : PracticeSends(d))
  \/ Files \/ (\E p \in Patients : Confirm(p)) \/ LeaveUnlinked \/ (\E q \in Patients : Correct(q))
  \/ \E p \in Patients : ReviewReads(p) \/ ReviewLetsGo(p) \/ ReviewFinishes(p)
  \/ RevisionFinishes
  \/ \E d \in Physicians : OpenCard(d) \/ Edit(d) \/ AskRevision(d) \/ TakeUpdate(d)
                           \/ CopyReply(d) \/ Close(d) \/ DoneUnreadable(d)
  \/ Disconnect \/ Reconnect

\* Everything that could happen has happened.
Rest == ~ENABLED Busy /\ UNCHANGED vars

Next == Busy \/ Rest

Spec == Init /\ [][Next]_vars

-----------------------------------------------------------------------------
(* The rules.                                                               *)

\* A physician has a reply open only while they may access its patient and can reply in its
\* conversation. (P3.2, P3.13; Rules: Access and privacy)
OnlyWithAccess == \A d \in Physicians : viewing[d] => CanSee(d)

\* A conversation reaches only the patient it is linked to: no other patient's review holds it, and
\* its card and the practice's replies in history show only on that patient. (P3.8, P3 Wrong
\* patient; Rules: Access and privacy, Data and consent)
RightPatient ==
  /\ \A p \in Patients \ {link} : review[p].read = 0
  /\ card.open => card.patient = link
  /\ \A h \in history : h.patient = link

\* Until a person or Coda settles it, every patient message Coda filed shows to someone who can
\* act: waiting for a match on the practice-wide list, or, to a physician who may access its
\* patient, unreadable, being reviewed, on a reply card they can open, or behind the disconnected
\* channel's line. (P3.9, P3 Message Coda can't read; physician Today, Principle 8)
NeverInvisible == \A i \in 1..filed :
  (Patient(i) /\ i \notin handled) =>
    \/ match[i] = "Waiting"
    \/ /\ match[i] = "Linked"
       /\ \E d \in Physicians :
            /\ link \in Access(d)
            /\ \/ ~inbox[i].readable
               \/ review[link].on
               \/ CanSee(d)
               \/ channel = "Disconnected"

\* Coda never replaces the physician's edited reply unless they choose its update. (P3.16, P3.17)
YourWords == card.edited => card.text = "Yours"

\* A card's draft never misses something filed after it without saying the patient wrote again.
\* (P3.16, P3.17, P3.19)
CurrentDraft == (card.open /\ card.text # "None") => (card.basis = filed \/ card.told)

\* A message, a sent reply or a correction starts at most one review. (physician Patient review;
\* Rules: Work integrity; P3 Wrong patient)
OneReview == \A e \in Events : starts[e] <= 1

\* What the patient's history says the practice sent is what the channel shows it sent. (P3.19;
\* physician Principle 3)
WhatWasSent == \A h \in history : h.words = inbox[h.item].words
=============================================================================

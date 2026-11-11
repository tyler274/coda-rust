--------------------------- MODULE MessageMatching ---------------------------
(***************************************************************************)
(* How a patient's message gets to the right patient, and how a physician *)
(* corrects a wrong link: the design in README.md. Each INVARIANT or      *)
(* PROPERTY is one rule, and quotes its sentence.                          *)
(*                                                                         *)
(* One conversation in one practice's channel. The state is what the      *)
(* physicians see: where each patient message is, the patient the         *)
(* conversation is linked to, the patient it shows on, the suggestion on  *)
(* "Messages to match", whose conversation is open, the Undo of a move,   *)
(* and what Today says about the channel. What Coda wrote from the        *)
(* conversation shows wherever the conversation shows, so `on` holds it   *)
(* too. Beside them are Coda's own facts: how far it has read the channel, *)
(* how many times it kept each message, which messages were ever linked,  *)
(* and whether it has acted on the move. Opening and closing the           *)
(* conversation are screen steps for rule 1.                              *)
(*                                                                         *)
(* One step is one thing a person, the channel or Coda does, and the       *)
(* screens change in the same step. The patient writes into the channel.  *)
(* The channel delivers each message after Coda's place in it, in any     *)
(* order and any number of times, while Coda can read it. Coda keeps a    *)
(* message, or keeps that it can't read it, in the step it reads it, and  *)
(* moves its place on only past messages it kept. Coda decides about the  *)
(* messages in any order. If a message is from a patient, whom its sender *)
(* and the channel point to, and which patient Coda suggests, are Coda's  *)
(* judgment: here each is a choice. The choice of sender and channel is  *)
(* made each time Coda links, since a contact can be saved between        *)
(* messages, and the step records it. The time for Undo is one step,      *)
(* "Undo goes away", which can come at any moment. A physician who moves   *)
(* it again while their Undo shows makes the same move. Another physician *)
(* moving it in those few seconds is not modeled.                          *)
(*                                                                         *)
(* Which patients each physician may access is open, so it is an         *)
(* input: TLC checks the rules for every setting in which each patient   *)
(* is someone's. Which physicians see a message before it is linked, and *)
(* who may disconnect the channel, are open too, so here any physician    *)
(* takes those steps, and rule 2 is checked for every physician.          *)
(*                                                                         *)
(* Time: 15 minutes always pass for a message Coda has not decided       *)
(* about, Undo always goes away unless someone presses it, a channel Coda *)
(* can't read for a while comes back, and Coda keeps reading while it     *)
(* can. Spec says so with fairness, and rules 6, 8 and 12 check that each *)
(* happens in the end. After Coda connects again, it offers to review the *)
(* messages from before, and reads them only once you accept.             *)
(*                                                                         *)
(* Not modeled, since they rest on Coda's records or its judgment: moved  *)
(* messages marked entered in error and never deleted, the Spruce tie,    *)
(* scores, a review never reading an unlinked message, and one patient's  *)
(* failure never stopping another patient's messages.                      *)
(*                                                                         *)
(* `event` names the step just taken; VIEW leaves it out of the state.   *)
(***************************************************************************)
EXTENDS Naturals, Sequences, FiniteSets, TLC

\* MessageMatching.cfg sets the scope: two patients, so a message can get to the wrong one and move to
\* a patient a physician may not access; two physicians, so access is different for each; two patient
\* messages, so a later message can follow the link, arrive during Undo, wait again after one is left
\* unlinked, or come before the other in the channel; and two moves, so a physician can move it again
\* while Undo shows, or after it goes away. The chart configs set smaller scopes, and Charts.tla the
\* moment and the steps of each chart.

CONSTANTS Patients, Physicians, MaxMessages, MaxMoves

Nobody == "Nobody"
NotPatient == "Not a patient"
Coda == "Coda"
ThePatient == "the patient"

\* Where a patient message is in Coda.
NotRead == "Not read"
CantRead == "Coda can't read it"
Done == "Marked Done"
Deciding == "Coda decides"
Filtered == "Filtered"
Waiting == "Waiting to match"
Linked == "Linked"
Unlinked == "Left unlinked"
NotAPatient == "Not a patient"
Deleted == "Deleted"

\* The channel, as Coda finds it.
Reading == "Coda reads it"
Failing == "Coda can't read it for a while"
Revoked == "The grant is revoked or blocked"
Disconnected == "Disconnected"

\* The offer to review the backlog, after Coda connects again.
NoOffer == "no offer"
Offered == "offered"
Accepted == "accepted"

\* What Today says about the channel.
Quiet == "nothing"
SaysCantRead == "Coda can't read the channel"
SaysReconnect == "Coda can't read the channel: Reconnect"

NoUndo == [by |-> Nobody, from |-> Nobody]

VARIABLES
  access,     \* input: the patients each physician may access
  msg,        \* where each patient message is in Coda
  link,       \* the patient the conversation is linked to, Nobody, or Not a patient
  on,         \* the patients the conversation shows on
  shown,      \* the suggestion each physician sees on "Messages to match"
  viewing,    \* the physicians who have the conversation open
  undo,       \* the Undo of a move: who may use it, and the patient it was on before the move
  moves,      \* how many moves there have been
  acted,      \* Coda has acted on the last move
  channel,    \* the channel, as Coda finds it
  today,      \* what Today says about the channel
  written,    \* how many messages the patient has written into the channel
  place,      \* Coda's place in the channel: it has moved on past messages 1..place
  start,      \* where Coda started in the channel when it last connected
  kept,       \* how many times Coda kept each message
  backlog,    \* the offer to review the messages from before Coda connected
  everLinked, \* the messages that were ever linked to a patient
  event

state == <<access, msg, link, on, shown, viewing, undo, moves, acted, channel, today, written, place, start, kept, backlog, everLinked>>
vars == <<access, msg, link, on, shown, viewing, undo, moves, acted, channel, today, written, place, start, kept, backlog, everLinked, event>>

Messages == 1..MaxMessages

Init ==
  /\ access \in {a \in [Physicians -> SUBSET Patients] : \A p \in Patients : \E d \in Physicians : p \in a[d]}
  /\ msg = [i \in Messages |-> NotRead]
  /\ link = Nobody
  /\ on = {}
  /\ shown = [d \in Physicians |-> Nobody]
  /\ viewing = {}
  /\ undo = NoUndo
  /\ moves = 0
  /\ acted = FALSE
  /\ channel = Reading
  /\ today = Quiet
  /\ written = 0
  /\ place = 0
  /\ start = 0
  /\ kept = [i \in Messages |-> 0]
  /\ backlog = NoOffer
  /\ everLinked = [i \in Messages |-> FALSE]
  /\ event = [name |-> "start", who |-> Nobody, sender |-> {}, says |-> Nobody]

\* The step's name, who took it, and whom the sender and the channel point to when Coda links.
Ev(name, who) == event' = [name |-> name, who |-> who, sender |-> {}, says |-> Nobody]

\* A message that is linked now was linked once.
Remember == everLinked' = [i \in Messages |-> everLinked[i] \/ msg'[i] = Linked]

\* The channel's parts of the state, unchanged.
SameChannel == UNCHANGED <<channel, today, written, place, start, kept, backlog>>

\* "Messages to match" shows Coda's suggestion to each physician who may access that patient.
Suggest(s) == [d \in Physicians |-> IF s \in access[d] THEN s ELSE Nobody]
NoSuggestion == [d \in Physicians |-> Nobody]

\* An open conversation stays open only for a physician who may access the patient it now shows on.
StillOpen(p) == {d \in viewing : p \in access[d]}

(* Message i, and every message still waiting, join patient p: the        *)
(* conversation is linked to p and shows on p.                             *)
Join(i, p) ==
  /\ msg' = [j \in Messages |-> IF j = i \/ msg[j] = Waiting THEN Linked ELSE msg[j]]
  /\ link' = p
  /\ on' = {p}
  /\ shown' = NoSuggestion

(* The patients the sender is confirmed for and the patient the channel  *)
(* ties the conversation to.                                               *)
Points(sender, says) == sender \cup ({says} \ {Nobody})

(* Where Coda links a message in a conversation not yet linked: the one   *)
(* patient that the sender and the channel point to. Nobody when they     *)
(* point to more than one patient, or to none.                            *)
OwnTarget(sender, says) ==
  IF Cardinality(Points(sender, says)) = 1 THEN CHOOSE p \in Points(sender, says) : TRUE ELSE Nobody

(* Message i is a patient's: it follows the conversation's link, links on *)
(* its own, or waits on "Messages to match" with Coda's suggestion.       *)
Route(i, name, who) ==
  CASE link \in Patients ->
         /\ msg' = [msg EXCEPT ![i] = Linked]
         /\ UNCHANGED <<link, on, shown>>
         /\ Ev(name, who)
    [] link = NotPatient ->
         /\ msg' = [msg EXCEPT ![i] = NotAPatient]
         /\ UNCHANGED <<link, on, shown>>
         /\ Ev(name, who)
    [] OTHER ->
         \E sender \in SUBSET Patients, says \in Patients \cup {Nobody} :
           /\ event' = [name |-> name, who |-> who, sender |-> sender, says |-> says]
           /\ LET t == OwnTarget(sender, says)
              IN IF t \in Patients
                 THEN Join(i, t)
                 ELSE /\ msg' = [msg EXCEPT ![i] = Waiting]
                      /\ \E s \in Patients :
                           shown' = IF \E j \in Messages : msg[j] = Waiting THEN shown ELSE Suggest(s)
                      /\ UNCHANGED <<link, on>>

-----------------------------------------------------------------------------
(* Reading the channel.                                                     *)

\* The patient writes into the channel.
PatientWrites ==
  /\ written < MaxMessages
  /\ written' = written + 1
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, undo, moves, acted, channel, today, place, start, kept, backlog, everLinked>>
  /\ Ev("the patient writes", ThePatient)

(* The channel delivers message i, after Coda's place, while Coda can     *)
(* read it. Once you accept the backlog offer, it also delivers the        *)
(* messages from before Coda connected. Coda keeps the message in the     *)
(* same step, or keeps that it can't read it. A message Coda already kept *)
(* changes nothing.                                                        *)
CodaReads(i) ==
  /\ channel = Reading /\ i <= written
  /\ place < i \/ (backlog = Accepted /\ i <= start)
  /\ IF kept[i] = 0
     THEN /\ kept' = [kept EXCEPT ![i] = @ + 1]
          /\ \E r \in {Deciding, CantRead} :
               /\ msg' = [msg EXCEPT ![i] = r]
               /\ Ev(IF r = CantRead THEN "Coda can't read the message" ELSE "Coda reads the message", Coda)
     ELSE /\ UNCHANGED <<kept, msg>>
          /\ Ev("the channel delivers it again", Coda)
  /\ UNCHANGED <<access, link, on, shown, viewing, undo, moves, acted, channel, today, written, place, start, backlog, everLinked>>

\* Coda moves its place on past a message it kept.
CodaMovesOn ==
  /\ channel = Reading /\ place < written
  /\ kept[place + 1] >= 1
  /\ place' = place + 1
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, undo, moves, acted, channel, today, written, start, kept, backlog, everLinked>>
  /\ Ev("Coda moves on in the channel", Coda)

\* Coda reads a message it couldn't read before.
CodaReadsItNow(i) ==
  /\ msg[i] = CantRead /\ channel = Reading
  /\ msg' = [msg EXCEPT ![i] = Deciding]
  /\ UNCHANGED <<access, link, on, shown, viewing, undo, moves, acted, channel, today, written, place, start, kept, backlog, everLinked>>
  /\ Ev("Coda reads the message it couldn't read", Coda)

\* A physician marks a message Coda can't read Done.
YouMarkDone(d, i) ==
  /\ msg[i] = CantRead
  /\ msg' = [msg EXCEPT ![i] = Done]
  /\ UNCHANGED <<access, link, on, shown, viewing, undo, moves, acted, channel, today, written, place, start, kept, backlog, everLinked>>
  /\ Ev("you mark it Done", d)

-----------------------------------------------------------------------------
(* The channel stops.                                                       *)

\* Coda can't read the channel for a while, or the grant is revoked or blocked. Today says so at once.
ChannelStops(r) ==
  /\ channel = Reading
  /\ channel' = r
  /\ today' = IF r = Revoked THEN SaysReconnect ELSE SaysCantRead
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, undo, moves, acted, written, place, start, kept, backlog, everLinked>>
  /\ Ev(IF r = Revoked THEN "the grant is revoked or blocked" ELSE "Coda can't read the channel for a while", Coda)

\* A channel Coda couldn't read for a while comes back.
ChannelComesBack ==
  /\ channel = Failing
  /\ channel' = Reading
  /\ today' = Quiet
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, undo, moves, acted, written, place, start, kept, backlog, everLinked>>
  /\ Ev("Coda can read the channel again", Coda)

\* A physician reconnects a revoked channel. Coda goes on from its place, so nothing is lost or kept twice.
YouReconnect(d) ==
  /\ channel = Revoked
  /\ channel' = Reading
  /\ today' = Quiet
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, undo, moves, acted, written, place, start, kept, backlog, everLinked>>
  /\ Ev("you press Reconnect", d)

(* A physician disconnects the channel. The completed disconnect deletes  *)
(* every message that was never linked to a patient.                       *)
YouDisconnect(d) ==
  /\ channel # Disconnected
  /\ channel' = Disconnected
  /\ today' = Quiet
  /\ msg' = [i \in Messages |-> IF kept[i] >= 1 /\ ~everLinked[i] THEN Deleted ELSE msg[i]]
  /\ kept' = [i \in Messages |-> IF msg'[i] = Deleted THEN 0 ELSE kept[i]]
  /\ shown' = NoSuggestion
  /\ backlog' = NoOffer
  /\ UNCHANGED <<access, link, on, viewing, undo, moves, acted, written, place, start, everLinked>>
  /\ Ev("you disconnect the channel", d)

\* A physician connects the channel again. Coda starts with new messages, and offers to review the backlog.
YouConnect(d) ==
  /\ channel = Disconnected
  /\ channel' = Reading
  /\ place' = written
  /\ start' = written
  /\ backlog' = IF written > 0 THEN Offered ELSE NoOffer
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, undo, moves, acted, today, written, kept, everLinked>>
  /\ Ev("you connect the channel", d)

\* A physician accepts the offer to review the backlog.
YouAcceptBacklog(d) ==
  /\ backlog = Offered
  /\ backlog' = Accepted
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, undo, moves, acted, channel, today, written, place, start, kept, everLinked>>
  /\ Ev("you accept the backlog offer", d)

-----------------------------------------------------------------------------
(* Coda decides, and a physician links.                                    *)

CodaDecides(i) ==
  /\ msg[i] = Deciding
  /\ Route(i, "Coda decides it's from a patient", Coda)
  /\ Remember
  /\ UNCHANGED <<access, viewing, undo, moves, acted>> /\ SameChannel

FifteenMinutesPass(i) ==
  /\ msg[i] = Deciding
  /\ Route(i, "15 minutes pass", Coda)
  /\ Remember
  /\ UNCHANGED <<access, viewing, undo, moves, acted>> /\ SameChannel

CodaFilters(i) ==
  /\ msg[i] = Deciding
  /\ msg' = [msg EXCEPT ![i] = Filtered]
  /\ UNCHANGED <<access, link, on, shown, viewing, undo, moves, acted, everLinked>> /\ SameChannel
  /\ Ev("Coda decides it isn't from a patient", Coda)

ThisIsAPatient(d, i) ==
  /\ msg[i] = Filtered
  /\ Route(i, "you mark This is a patient", d)
  /\ Remember
  /\ UNCHANGED <<access, viewing, undo, moves, acted>> /\ SameChannel

\* A physician links a waiting message to a patient they may access.
LinkIt(d, i, p) ==
  /\ msg[i] = Waiting /\ p \in access[d]
  /\ Join(i, p)
  /\ Remember
  /\ UNCHANGED <<access, viewing, undo, moves, acted>> /\ SameChannel
  /\ Ev("you link it to a patient", d)

\* Leaving a message unlinked covers only that message.
LeaveUnlinked(d, i) ==
  /\ msg[i] = Waiting
  /\ msg' = [msg EXCEPT ![i] = Unlinked]
  /\ shown' = IF \E j \in Messages \ {i} : msg[j] = Waiting THEN shown ELSE NoSuggestion
  /\ UNCHANGED <<access, link, on, viewing, undo, moves, acted, everLinked>> /\ SameChannel
  /\ Ev("you leave it unlinked", d)

-----------------------------------------------------------------------------
(* A physician opens the conversation on its patient.                       *)

OpenIt(d) ==
  /\ d \notin viewing /\ link \in on /\ link \in access[d]
  /\ viewing' = viewing \cup {d}
  /\ UNCHANGED <<access, msg, link, on, shown, undo, moves, acted, everLinked>> /\ SameChannel
  /\ Ev("you open the conversation", d)

CloseIt(d) ==
  /\ d \in viewing
  /\ viewing' = viewing \ {d}
  /\ UNCHANGED <<access, msg, link, on, shown, undo, moves, acted, everLinked>> /\ SameChannel
  /\ Ev("you close the conversation", d)

-----------------------------------------------------------------------------
(* Wrong patient.                                                           *)

(* A physician who may access the patient moves the linked conversation    *)
(* to a different patient they may access, or to not a patient. At once  *)
(* it shows only in its new place, its messages follow it, and a          *)
(* physician who may not access the new patient loses it. Undo shows, and *)
(* Coda does not act on the move yet. A move again while Undo shows is the *)
(* same move, so Undo still goes back to where it was before. A move back *)
(* to there ends the move: Undo stops showing, and Coda does nothing.      *)
Move(d, q) ==
  /\ link \in Patients /\ link \in access[d] /\ moves < MaxMoves
  /\ undo.by \in {Nobody, d}
  /\ q \in (access[d] \cup {NotPatient}) \ {link}
  /\ link' = q
  /\ on' = {q} \cap Patients
  /\ msg' = IF q = NotPatient THEN [i \in Messages |-> IF msg[i] = Linked THEN NotAPatient ELSE msg[i]] ELSE msg
  /\ viewing' = StillOpen(q)
  /\ undo' = IF undo.by = d /\ q = undo.from THEN NoUndo
             ELSE [by |-> d, from |-> IF undo.by = d THEN undo.from ELSE link]
  /\ moves' = moves + 1
  /\ acted' = FALSE
  /\ UNCHANGED <<access, shown, everLinked>> /\ SameChannel
  /\ Ev(IF q = NotPatient THEN "you move it to not a patient" ELSE "you move it to the other patient", d)

(* Undo puts the conversation back on the patient it was on before the    *)
(* move, with every message, also one that arrived while Undo showed. A  *)
(* physician who may not access that patient loses it.                    *)
UndoIt(d) ==
  /\ undo.by = d
  /\ link' = undo.from
  /\ on' = {undo.from}
  /\ msg' = [i \in Messages |-> IF msg[i] = NotAPatient THEN Linked ELSE msg[i]]
  /\ viewing' = StillOpen(undo.from)
  /\ undo' = NoUndo
  /\ Remember
  /\ UNCHANGED <<access, shown, moves, acted>> /\ SameChannel
  /\ Ev("you press Undo", d)

\* Undo goes away, and Coda acts on the move.
UndoGoesAway ==
  /\ undo.by # Nobody
  /\ undo' = NoUndo
  /\ acted' = TRUE
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, moves, everLinked>> /\ SameChannel
  /\ Ev("Undo goes away, and Coda acts on the move", Coda)

-----------------------------------------------------------------------------

Busy ==
  \/ PatientWrites \/ CodaMovesOn \/ ChannelComesBack
  \/ \E i \in Messages : CodaReads(i) \/ CodaReadsItNow(i)
  \/ \E r \in {Failing, Revoked} : ChannelStops(r)
  \/ \E i \in Messages : CodaDecides(i) \/ FifteenMinutesPass(i) \/ CodaFilters(i)
  \/ \E d \in Physicians, i \in Messages : ThisIsAPatient(d, i) \/ LeaveUnlinked(d, i) \/ YouMarkDone(d, i)
  \/ \E d \in Physicians, i \in Messages, p \in Patients : LinkIt(d, i, p)
  \/ \E d \in Physicians : OpenIt(d) \/ CloseIt(d) \/ UndoIt(d) \/ YouReconnect(d) \/ YouDisconnect(d) \/ YouConnect(d)
     \/ YouAcceptBacklog(d)
  \/ \E d \in Physicians, q \in Patients \cup {NotPatient} : Move(d, q)
  \/ UndoGoesAway

\* Nothing is left to do.
Rest == ~ENABLED Busy /\ UNCHANGED vars

Next == Busy \/ Rest

(* 15 minutes pass for a message Coda has not decided about, Undo goes    *)
(* away unless someone presses it, a channel Coda can't read for a while  *)
(* comes back, and Coda keeps reading and moving on while it can.         *)
Fairness == /\ \A i \in Messages : WF_vars(FifteenMinutesPass(i)) /\ WF_vars(CodaReads(i))
            /\ WF_vars(UndoGoesAway)
            /\ WF_vars(ChannelComesBack)
            /\ WF_vars(CodaMovesOn)

Spec == Init /\ [][Next]_vars /\ Fairness

-----------------------------------------------------------------------------
(* The rules, in the README's numbering.                                   *)

\* 1. You see a conversation only while you may access its patient.
OnlyWithAccess == \A d \in viewing : link \in on /\ link \in access[d]

\* 2. A suggestion never names a patient you may not access.
SuggestionsWithAccess == \A d \in Physicians : shown[d] \in access[d] \cup {Nobody}

(* A message joins a patient: it becomes linked by any step except Undo,  *)
(* which only puts back messages that were linked before the move.        *)
Joins(i) == msg[i] # Linked /\ msg'[i] = Linked /\ event'.name # "you press Undo"

\* 3. A message joins only the patient its conversation is linked to.
JoinsItsPatient == [][\A i \in Messages : Joins(i) /\ link \in Patients => link' = link]_vars

\* The steps in which Coda routes a message it decided is a patient's.
RouteSteps == {"Coda decides it's from a patient", "15 minutes pass", "you mark This is a patient"}

\* 4. A conversation that isn't linked yet links on its own when, and only when, its sender is confirmed
\* for exactly one patient or the channel ties it to exactly one patient, and nothing points to a different
\* patient.
LinksOnItsOwn == [][\A i \in Messages :
  /\ Joins(i) /\ link \notin Patients
       => \/ event'.name = "you link it to a patient"
          \/ Points(event'.sender, event'.says) = {link'}
  /\ msg[i] # msg'[i] /\ link = Nobody /\ event'.name \in RouteSteps
     /\ OwnTarget(event'.sender, event'.says) \in Patients
       => msg'[i] = Linked /\ link' = OwnTarget(event'.sender, event'.says)]_vars

\* 5. After a conversation moves to a different patient, nothing of it stays readable on the patient it left.
RightPatient == on \subseteq {link}

(* A message is handled only by its own step: a physician leaves that    *)
(* one message unlinked, Coda decides it isn't from a patient, it goes    *)
(* where the conversation is not a patient, or a completed disconnect     *)
(* deletes it, which it does only to a message never linked.               *)
HandledBy(i) ==
  CASE msg'[i] = Unlinked ->
         event'.name = "you leave it unlinked" /\ \A j \in Messages \ {i} : msg'[j] = Unlinked => msg[j] = Unlinked
    [] msg'[i] = Filtered ->
         event'.name = "Coda decides it isn't from a patient" /\ \A j \in Messages \ {i} : msg'[j] = Filtered => msg[j] = Filtered
    [] msg'[i] = Deleted -> event'.name = "you disconnect the channel" /\ ~everLinked[i]
    [] OTHER -> link' = NotPatient

\* 6. After Coda decides, or after 15 minutes, every patient message shows to a physician who can act on
\* it until it's handled.
NeverInvisible ==
  /\ [](\A i \in Messages : msg[i] = Linked => link \in on /\ \E d \in Physicians : link \in access[d])
  /\ [][\A i \in Messages : (msg[i] # msg'[i] /\ msg'[i] \in {Unlinked, Filtered, NotAPatient, Deleted}) => HandledBy(i)]_vars
  /\ \A i \in Messages : msg[i] = Deciding ~> msg[i] # Deciding

\* 7. Undo puts the conversation back on the patient it was on before the move.
\* A second move while Undo shows is part of the same move, so Undo keeps the patient it goes back to.
UndoPutsBack ==
  [][/\ (event'.name = "you press Undo" =>
            link' = undo.from /\ on' = {undo.from} /\ \A i \in Messages : msg'[i] # NotAPatient)
      /\ (undo.by # Nobody /\ undo'.by # Nobody => undo'.from = undo.from)]_vars

\* 8. Coda acts on a move only after Undo goes away, and then it always acts on it.
\* A move puts the conversation on a different patient, so Undo shows only while it is away from the
\* patient it was on before. Undo always goes away, unless you press it or end the move by moving the
\* conversation back.
ActsAfterUndo ==
  /\ [](undo.by # Nobody => link # undo.from)
  /\ [][/\ (~acted /\ acted' => event'.name = "Undo goes away, and Coda acts on the move")
         /\ (undo'.by # Nobody => ~acted')
         /\ (event'.name = "Undo goes away, and Coda acts on the move" => acted')
         /\ (undo.by # Nobody /\ undo'.by = Nobody => link' = undo.from \/ event'.name = "Undo goes away, and Coda acts on the move")]_vars
  /\ undo.by # Nobody ~> undo.by = Nobody

\* 9. You link a message, or move a conversation, only between patients you may access.
OnlyBetweenTheirPatients == [][
  /\ event'.name = "you link it to a patient" => link' \in access[event'.who]
  /\ event'.name \in {"you move it to the other patient", "you move it to not a patient"} =>
       link \in access[event'.who] /\ link' \in access[event'.who] \cup {NotPatient}]_vars

\* 10. Today says Coda can't read the channel exactly while it can't, and asks you to "Reconnect" only for a
\* revoked grant or an administrator's block.
TodaySaysSo ==
  /\ (today # Quiet <=> channel \in {Failing, Revoked})
  /\ (today = SaysReconnect <=> channel = Revoked)

\* 11. A completed disconnect leaves no message that was never linked to a patient.
ClearedOnDisconnect == [][event'.name = "you disconnect the channel" =>
  \A i \in Messages : ~everLinked'[i] => msg'[i] \in {NotRead, Deleted} /\ kept'[i] = 0]_vars

\* 12. Coda keeps each message exactly once, however many times and in whatever order the channel delivers
\* it.
\* It keeps each message written since it last connected, while it can read the channel.
ExactlyOnce ==
  /\ [](\A i \in Messages : kept[i] <= 1)
  /\ \A i \in Messages : (start < i /\ i <= written /\ channel = Reading) ~> (kept[i] >= 1 \/ channel # Reading)

\* 13. A message Coda can't read shows on Today until Coda reads it, you mark it Done, or a completed
\* disconnect deletes it.
CantReadShows == [][/\ event'.name = "Coda can't read the message" => \E i \in Messages : msg[i] # CantRead /\ msg'[i] = CantRead
                    /\ \A i \in Messages : msg[i] = CantRead /\ msg'[i] # CantRead =>
                         event'.name \in {"Coda reads the message it couldn't read", "you mark it Done", "you disconnect the channel"}]_vars

\* 14. Coda reads a message from before it connected only after you accept the offer to review the backlog.
BacklogOnlyWhenAccepted == [][\A i \in Messages : kept'[i] > kept[i] /\ i <= start => backlog = Accepted]_vars
=============================================================================

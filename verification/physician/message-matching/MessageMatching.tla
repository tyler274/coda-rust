--------------------------- MODULE MessageMatching ---------------------------
(***************************************************************************)
(* How a patient's message gets to the right patient, and how a physician *)
(* corrects a wrong link: the design in README.md. Each INVARIANT or      *)
(* PROPERTY is one rule, and quotes its sentence.                          *)
(*                                                                         *)
(* One conversation. The state is what the physicians see: where each    *)
(* patient message is, the patient the conversation is linked to, the    *)
(* patient it shows on, the suggestion on Messages to match, whose        *)
(* conversation is open, and the Undo of a move. Beside them is one fact: *)
(* whether Coda has acted on the move.                                     *)
(*                                                                         *)
(* One step is one thing a person or Coda does, and the screens change in *)
(* the same step. The patient writes and Coda reads the message in one    *)
(* step, since nothing shows in between. Coda decides about the messages *)
(* in any order. If a message is from a patient, whom its sender and the *)
(* channel point to, and which patient Coda suggests, are Coda's          *)
(* judgment: here each is a choice. The choice of sender and channel is  *)
(* made each time Coda links, since a contact can be saved between        *)
(* messages, and the step records it. The time for Undo is one step,      *)
(* "Undo goes away", which can come at any moment.                          *)
(*                                                                         *)
(* Which patients each physician may access is open (P2), so it is an    *)
(* input: TLC checks the rules for every setting in which each patient   *)
(* is someone's. Which physicians see a message before it is linked is   *)
(* open too, so here a physician links, leaves unlinked or brings back a *)
(* message, and rule 2 is checked for every physician.                    *)
(*                                                                         *)
(* Not modeled, since they rest on Coda's records or the channel: reading *)
(* each message one time and saving it first, one reader for each         *)
(* channel, moved messages marked entered in error and never deleted, and *)
(* one failure never stopping the others. Not modeled, since they are not *)
(* order problems: a message Coda can't read, the channel stopping        *)
(* (rules 7 and 8), and what a suggestion or a row says.                   *)
(*                                                                         *)
(* `event` names the step just taken; VIEW leaves it out of the state.   *)
(***************************************************************************)
EXTENDS Naturals, Sequences, FiniteSets, TLC

\* MessageMatching.cfg sets the scope: two patients, so a message can get to the wrong one and move to
\* a patient a physician may not access; two physicians, so access is different for each; two patient
\* messages, so a later message can follow the link, arrive during Undo, or wait again after one is
\* left unlinked; and one move. chart.cfg and chart-move.cfg set smaller scopes, and Charts.tla the
\* moment and the steps of each chart.

CONSTANTS Patients, Physicians, MaxMessages, MaxMoves

Nobody == "Nobody"
NotPatient == "Not a patient"
Coda == "Coda"
ThePatient == "the patient"

\* Where a patient message is.
NotWritten == "Not written"
Deciding == "Coda decides"
Filtered == "Filtered"
Waiting == "Waiting to match"
Linked == "Linked"
Unlinked == "Left unlinked"
NotAPatient == "Not a patient"

NoUndo == [by |-> Nobody, from |-> Nobody]

VARIABLES
  access,   \* input: the patients each physician may access
  msg,      \* where each patient message is
  link,     \* the patient the conversation is linked to, Nobody, or Not a patient
  on,       \* the patients the conversation shows on
  shown,    \* the suggestion each physician sees on Messages to match
  viewing,  \* the physicians who have the conversation open
  undo,     \* the Undo of a move: who may use it, and the patient it goes back to
  moves,    \* how many moves there have been
  acted,    \* Coda has acted on the last move
  event

state == <<access, msg, link, on, shown, viewing, undo, moves, acted>>
vars == <<access, msg, link, on, shown, viewing, undo, moves, acted, event>>

Messages == 1..MaxMessages

Init ==
  /\ access \in {a \in [Physicians -> SUBSET Patients] : \A p \in Patients : \E d \in Physicians : p \in a[d]}
  /\ msg = [i \in Messages |-> NotWritten]
  /\ link = Nobody
  /\ on = {}
  /\ shown = [d \in Physicians |-> Nobody]
  /\ viewing = {}
  /\ undo = NoUndo
  /\ moves = 0
  /\ acted = FALSE
  /\ event = [name |-> "start", who |-> Nobody, sender |-> {}, says |-> Nobody]

\* The step's name, who took it, and whom the sender and the channel point to when Coda links.
Ev(name, who) == event' = [name |-> name, who |-> who, sender |-> {}, says |-> Nobody]

\* Messages to match shows Coda's suggestion to each physician who may access that patient.
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

(* Where Coda links a message in a conversation not yet linked: the one   *)
(* patient the sender is confirmed for, else the one the channel ties it  *)
(* to. Nobody when the two point to different patients, or neither points *)
(* to one patient.                                                          *)
OwnTarget(sender, says) ==
  LET one == IF Cardinality(sender) = 1 THEN CHOOSE p \in sender : TRUE ELSE Nobody
  IN CASE one # Nobody /\ says # Nobody /\ one # says -> Nobody
       [] one # Nobody -> one
       [] OTHER -> says

(* Message i is a patient's: it follows the conversation's link, links on *)
(* its own, or waits on Messages to match with Coda's suggestion.         *)
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
(* The patient writes, and Coda reads the message.                          *)

PatientWrites ==
  \E i \in Messages :
    /\ msg[i] = NotWritten /\ \A j \in 1..(i - 1) : msg[j] # NotWritten
    /\ msg' = [msg EXCEPT ![i] = Deciding]
    /\ UNCHANGED <<access, link, on, shown, viewing, undo, moves, acted>>
    /\ Ev("the patient writes", ThePatient)

-----------------------------------------------------------------------------
(* Coda decides, and a physician links.                                    *)

CodaDecides(i) ==
  /\ msg[i] = Deciding
  /\ Route(i, "Coda decides it's from a patient", Coda)
  /\ UNCHANGED <<access, viewing, undo, moves, acted>>

FifteenMinutesPass(i) ==
  /\ msg[i] = Deciding
  /\ Route(i, "15 minutes pass", Coda)
  /\ UNCHANGED <<access, viewing, undo, moves, acted>>

CodaFilters(i) ==
  /\ msg[i] = Deciding
  /\ msg' = [msg EXCEPT ![i] = Filtered]
  /\ UNCHANGED <<access, link, on, shown, viewing, undo, moves, acted>>
  /\ Ev("Coda decides it isn't from a patient", Coda)

ThisIsAPatient(d, i) ==
  /\ msg[i] = Filtered
  /\ Route(i, "you mark This is a patient", d)
  /\ UNCHANGED <<access, viewing, undo, moves, acted>>

\* A physician links a waiting message to a patient they may access.
LinkIt(d, i, p) ==
  /\ msg[i] = Waiting /\ p \in access[d]
  /\ Join(i, p)
  /\ UNCHANGED <<access, viewing, undo, moves, acted>>
  /\ Ev("you link it to a patient", d)

\* Leaving a message unlinked covers only that message.
LeaveUnlinked(d, i) ==
  /\ msg[i] = Waiting
  /\ msg' = [msg EXCEPT ![i] = Unlinked]
  /\ shown' = IF \E j \in Messages \ {i} : msg[j] = Waiting THEN shown ELSE NoSuggestion
  /\ UNCHANGED <<access, link, on, viewing, undo, moves, acted>>
  /\ Ev("you leave it unlinked", d)

-----------------------------------------------------------------------------
(* A physician opens the conversation on its patient.                       *)

OpenIt(d) ==
  /\ d \notin viewing /\ link \in on /\ link \in access[d]
  /\ viewing' = viewing \cup {d}
  /\ UNCHANGED <<access, msg, link, on, shown, undo, moves, acted>>
  /\ Ev("you open the conversation", d)

CloseIt(d) ==
  /\ d \in viewing
  /\ viewing' = viewing \ {d}
  /\ UNCHANGED <<access, msg, link, on, shown, undo, moves, acted>>
  /\ Ev("you close the conversation", d)

-----------------------------------------------------------------------------
(* Wrong patient.                                                           *)

(* A physician who may access the patient moves the linked conversation    *)
(* to a different patient they may access, or to not a patient. At once  *)
(* it shows only in its new place, its messages follow it, and a          *)
(* physician who may not access the new patient loses it. Undo shows, and *)
(* Coda does not act on the move yet.                                       *)
Move(d, q) ==
  /\ link \in Patients /\ link \in access[d] /\ moves < MaxMoves
  /\ q \in (access[d] \cup {NotPatient}) \ {link}
  /\ link' = q
  /\ on' = {q} \cap Patients
  /\ msg' = IF q = NotPatient THEN [i \in Messages |-> IF msg[i] = Linked THEN NotAPatient ELSE msg[i]] ELSE msg
  /\ viewing' = StillOpen(q)
  /\ undo' = [by |-> d, from |-> link]
  /\ moves' = moves + 1
  /\ acted' = FALSE
  /\ UNCHANGED <<access, shown>>
  /\ Ev(IF q = NotPatient THEN "you move it to not a patient" ELSE "you move it to the other patient", d)

(* Undo puts the conversation back on the patient it left, with every     *)
(* message, also one that arrived while Undo showed. A physician who may *)
(* not access that patient loses it.                                        *)
UndoIt(d) ==
  /\ undo.by = d
  /\ link' = undo.from
  /\ on' = {undo.from}
  /\ msg' = [i \in Messages |-> IF msg[i] = NotAPatient THEN Linked ELSE msg[i]]
  /\ viewing' = StillOpen(undo.from)
  /\ undo' = NoUndo
  /\ UNCHANGED <<access, shown, moves, acted>>
  /\ Ev("you press Undo", d)

\* Undo goes away, and Coda acts on the move.
UndoGoesAway ==
  /\ undo.by # Nobody
  /\ undo' = NoUndo
  /\ acted' = TRUE
  /\ UNCHANGED <<access, msg, link, on, shown, viewing, moves>>
  /\ Ev("Undo goes away, and Coda acts on the move", Coda)

-----------------------------------------------------------------------------

Busy ==
  \/ PatientWrites
  \/ \E i \in Messages : CodaDecides(i) \/ FifteenMinutesPass(i) \/ CodaFilters(i)
  \/ \E d \in Physicians, i \in Messages : ThisIsAPatient(d, i) \/ LeaveUnlinked(d, i)
  \/ \E d \in Physicians, i \in Messages, p \in Patients : LinkIt(d, i, p)
  \/ \E d \in Physicians : OpenIt(d) \/ CloseIt(d) \/ UndoIt(d)
  \/ \E d \in Physicians, q \in Patients \cup {NotPatient} : Move(d, q)
  \/ UndoGoesAway

\* Nothing is left to do.
Rest == ~ENABLED Busy /\ UNCHANGED vars

Next == Busy \/ Rest

Spec == Init /\ [][Next]_vars

Symmetry == Permutations(Patients) \cup Permutations(Physicians)

-----------------------------------------------------------------------------
(* The rules, in the README's numbering. Rules 7 and 8 are not modeled.    *)

\* 1. A physician sees a conversation only while they may access its patient.
OnlyWithAccess == \A d \in viewing : link \in on /\ link \in access[d]

\* 2. A suggestion never names a patient the viewer may not access.
SuggestionsWithAccess == \A d \in Physicians : shown[d] \in access[d] \cup {Nobody}

\* 3. A message links on its own only when its conversation is already linked to that patient, its
\* sender is confirmed for that one patient, or the channel ties its conversation to that one patient.
LinksOnItsOwn == [][\A i \in Messages :
  (msg[i] # Linked /\ msg'[i] = Linked /\ event'.name \notin {"you link it to a patient", "you press Undo"})
    => \/ link \in Patients /\ link' = link
       \/ event'.sender = {link'}
       \/ event'.says = link']_vars

\* 4. After a conversation moves to a different patient, nothing of it stays readable on the patient it left.
RightPatient == on \subseteq {link}

\* 5. After Coda decides, or after 15 minutes, every patient message shows to a physician who can act on
\* it until it is handled.
NeverInvisible == \A i \in Messages :
  msg[i] = Linked => link \in on /\ \E d \in Physicians : link \in access[d]

\* 6. Undo puts the conversation back on the patient it left, and Coda acts on a move only after Undo
\* goes away.
UndoPutsBack == [][/\ event'.name = "you press Undo" => link' = undo.from /\ on' = {undo.from}
                   /\ undo'.by # Nobody => ~acted']_vars
=============================================================================

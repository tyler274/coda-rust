------------------------------- MODULE Charts -------------------------------
(***************************************************************************)
(* Where each chart of MessageMatching.tla starts, and the steps it draws. *)
(* A chart runs the same Next with only the steps of its scene. Each       *)
(* chart's view says what it leaves out.                                    *)
(***************************************************************************)
EXTENDS MessageMatching

\* chart.cfg: one message gets to its patient. Nobody opens the conversation, and the channel keeps working.
LinkingSteps == event'.name \notin {"you open the conversation", "you close the conversation",
  "Coda can't read the message", "the channel delivers it again", "Coda moves on in the channel",
  "Coda can't read the channel for a while", "the grant is revoked or blocked", "you disconnect the channel"}

\* The conversation is linked to patient A, with one message. You may access both patients; another
\* physician may access only patient A and has the conversation open.
LinkedToA ==
  /\ access = [d \in Physicians |-> IF d = "you" THEN Patients ELSE {"patient A"}]
  /\ msg = [i \in Messages |-> IF i = 1 THEN Linked ELSE NotRead]
  /\ link = "patient A"
  /\ on = {"patient A"}
  /\ shown = NoSuggestion
  /\ viewing = {"another physician"}
  /\ undo = NoUndo
  /\ moves = 0
  /\ acted = FALSE
  /\ channel = Reading
  /\ today = Quiet
  /\ written = 1
  /\ place = 1
  /\ start = 0
  /\ kept = [i \in Messages |-> IF i = 1 THEN 1 ELSE 0]
  /\ backlog = NoOffer
  /\ everLinked = [i \in Messages |-> i = 1]
  /\ event = [name |-> "start", who |-> Nobody, sender |-> {}, says |-> Nobody]

\* chart-move.cfg: you move the conversation, and press Undo or let Undo go away. The patient writes
\* again only while Undo shows.
MoveSpec == LinkedToA /\ [][Next]_vars /\ Fairness
MoveSteps ==
  /\ event'.who \in {"you", Coda, ThePatient}
  /\ event'.name \notin {"you open the conversation", "you close the conversation",
                         "Coda decides it isn't from a patient", "Coda can't read the message",
                         "the channel delivers it again", "Coda moves on in the channel",
                         "Coda can't read the channel for a while", "the grant is revoked or blocked",
                         "you disconnect the channel"}
  /\ event'.name \in {"the patient writes", "Coda reads the message"} => undo.by # Nobody

\* chart-move-again.cfg: from the same moment, you move the conversation, and move it again while Undo
\* shows or after it goes away. Nobody writes, and the other physician does not move it.
MoveAgainSteps ==
  /\ event'.who \in {"you", Coda}
  /\ event'.name \notin {"you open the conversation", "you close the conversation",
                         "Coda can't read the channel for a while", "the grant is revoked or blocked",
                         "you disconnect the channel"}

\* chart-channel.cfg: the channel works, stops, comes back, or is disconnected. Nobody writes.
ChannelSteps == event'.name # "the patient writes"

\* chart-cant-read.cfg: one message Coda reads or can't read, while the channel works or is disconnected.
\* Nobody links, filters, moves or opens it.
CantReadSteps == event'.name \notin {"you open the conversation", "you close the conversation",
  "Coda decides it isn't from a patient", "you mark This is a patient", "you link it to a patient",
  "you leave it unlinked", "you move it to the other patient", "you move it to not a patient",
  "you press Undo", "Undo goes away, and Coda acts on the move",
  "Coda can't read the channel for a while", "the grant is revoked or blocked", "you accept the backlog offer"}

\* chart-backlog.cfg: Coda has kept one message, not linked yet. You disconnect the channel, connect it
\* again, and Coda offers the backlog. Nobody links, filters, moves or opens it, and Coda can read every
\* message and the channel.
KeptOne ==
  /\ access = [d \in Physicians |-> Patients]
  /\ msg = [i \in Messages |-> Deciding]
  /\ link = Nobody
  /\ on = {}
  /\ shown = NoSuggestion
  /\ viewing = {}
  /\ undo = NoUndo
  /\ moves = 0
  /\ acted = FALSE
  /\ channel = Reading
  /\ today = Quiet
  /\ written = 1
  /\ place = 1
  /\ start = 0
  /\ kept = [i \in Messages |-> 1]
  /\ backlog = NoOffer
  /\ everLinked = [i \in Messages |-> FALSE]
  /\ event = [name |-> "start", who |-> Nobody, sender |-> {}, says |-> Nobody]
BacklogSpec == KeptOne /\ [][Next]_vars /\ Fairness
BacklogSteps == event'.name \notin {"you open the conversation", "you close the conversation",
  "Coda decides it isn't from a patient", "you mark This is a patient", "you link it to a patient",
  "you leave it unlinked", "you move it to the other patient", "you move it to not a patient",
  "you press Undo", "Undo goes away, and Coda acts on the move", "Coda can't read the message",
  "Coda can't read the channel for a while", "the grant is revoked or blocked"}
=============================================================================

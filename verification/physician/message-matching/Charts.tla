------------------------------- MODULE Charts -------------------------------
(***************************************************************************)
(* Where each chart of MessageMatching.tla starts, and the steps it draws. *)
(* A chart runs the same Next with only the steps of its scene. Each       *)
(* chart's view says what it leaves out.                                    *)
(***************************************************************************)
EXTENDS MessageMatching

\* chart.cfg: one message gets to its patient. Nobody opens the conversation.
LinkingSteps == event'.name \notin {"you open the conversation", "you close the conversation"}

\* The conversation is linked to patient A, with one message. You may access both patients; another
\* physician may access only patient A and has the conversation open.
LinkedToA ==
  /\ access = [d \in Physicians |-> IF d = "you" THEN Patients ELSE {"patient A"}]
  /\ msg = [i \in Messages |-> IF i = 1 THEN Linked ELSE NotWritten]
  /\ link = "patient A"
  /\ on = {"patient A"}
  /\ shown = NoSuggestion
  /\ viewing = {"another physician"}
  /\ undo = NoUndo
  /\ moves = 0
  /\ acted = FALSE
  /\ event = [name |-> "start", who |-> Nobody, sender |-> {}, says |-> Nobody]

\* chart-move.cfg: you move the conversation, and press Undo or let Undo go away. The patient writes
\* again only while Undo shows.
MoveSpec == LinkedToA /\ [][Next]_vars /\ Fairness
MoveSteps ==
  /\ event'.who \in {"you", Coda, ThePatient}
  /\ event'.name \notin {"you open the conversation", "you close the conversation",
                         "Coda decides it isn't from a patient"}
  /\ event'.name = "the patient writes" => undo.by # Nobody

\* chart-move-again.cfg: from the same moment, you move the conversation, and move it again while Undo
\* shows or after it goes away. Nobody writes, and the other physician does not move it.
MoveAgainSteps ==
  /\ event'.who \in {"you", Coda}
  /\ event'.name \notin {"you open the conversation", "you close the conversation"}
=============================================================================

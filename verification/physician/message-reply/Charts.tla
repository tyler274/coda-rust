------------------------------- MODULE Charts -------------------------------
(***************************************************************************)
(* Where each chart of MessageReply.tla starts, and the steps it draws. A *)
(* chart runs the same Next with only the steps of its scene. Each        *)
(* chart's view says what it leaves out.                                   *)
(***************************************************************************)
EXTENDS MessageReply

\* chart.cfg: from no message, the patient writes, Coda reviews, and the reply is closed by the practice's
\* answer or by you. Nobody edits, copies or moves it, the channel stays connected, Coda flags nothing, and
\* the practice sends only its own words that answer the patient.
ReplySteps ==
  /\ event'.name \notin {"you edit your draft", "you ask Coda to revise", "you restore your version",
                         "you choose Coda's update", "you copy the reply", "Coda restarts", "you close the concern",
                         "the channel is disconnected", "Coda acts on a move to another patient",
                         "Coda acts on a move to not a patient"}
  /\ sent'.text = Nothing \/ (sent'.text = OwnWords /\ sent'.answers)
  /\ ~concern'

\* The conversation is linked to patient A, Coda's reply to its one message is ready, and each physician has
\* the draft dr. Nothing else has happened.
ReadyWith(dr) ==
  /\ access = [d \in Physicians |-> IF d = "you" THEN Patients ELSE {"patient A"}]
  /\ link = "patient A"
  /\ moves = 0
  /\ written = 1
  /\ msg = [i \in Messages |-> IF i = 1 THEN Open ELSE NotWritten]
  /\ sent = NoSent
  /\ history = Nothing
  /\ review = NoReview
  /\ woke = {}
  /\ reply = [state |-> Ready, v |-> 1, id |-> 1, for |-> "patient A", msgs |-> {1}]
  /\ concern = FALSE
  /\ draft = dr
  /\ offers = {}
  /\ channel = Connected
  /\ event = [name |-> "start", who |-> Nobody, text |-> Nothing]

ReadyOnA == ReadyWith([d \in Physicians |-> NoDraftHere])

ReadySpec == ReadyOnA /\ [][Next]_vars /\ Fairness

\* chart-draft.cfg: from ReadyOnA, you edit, ask Coda to revise and restore, while the patient writes again
\* and Coda writes a newer reply. Nobody copies, closes or moves it, the practice sends nothing, and the
\* channel stays connected.
DraftSteps ==
  /\ event'.name \in {"you edit your draft", "you ask Coda to revise", "you restore your version",
                      "you choose Coda's update", "the patient writes", "a review starts", "the review writes a reply"}
  /\ ~concern'

\* chart-move.cfg: from ReadyOnA, with your draft, Coda acts on a move, and the new patient's review writes
\* the reply again. Another physician may access only patient A. Nobody writes, edits or closes it.
YourDraftOnA == ReadyWith([d \in Physicians |-> IF d = "you" THEN [kind |-> Mine, on |-> 1] ELSE NoDraftHere])

MoveSpec == YourDraftOnA /\ [][Next]_vars /\ Fairness
MoveSteps ==
  /\ event'.name \in {"Coda acts on a move to another patient", "Coda acts on a move to not a patient",
                      "a review starts", "the review writes a reply"}
  /\ ~concern'

\* chart-channel.cfg: from ReadyOnA, the channel is disconnected, and you mark the reply Done or Coda
\* reconnects. Nothing else happens.
ChannelSteps ==
  event'.name \in {"the channel is disconnected", "Coda reconnects and sees the reply", "you mark it Done"}
=============================================================================

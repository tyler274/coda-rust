---------------------------- MODULE ReplyCharts ----------------------------
(***************************************************************************)
(* The steps each chart of Reply.tla draws. A chart runs the model's own   *)
(* Spec from Init, with every access setting, at a small scope its config  *)
(* sets. Its ACTION_CONSTRAINT keeps the steps of its journey in their     *)
(* likely order, so rare interleavings stay in the full check and out of   *)
(* the chart.                                                               *)
(***************************************************************************)
EXTENDS Reply

Named(names) == event'.name \in names

\* Coda reads the channel's items in the order they were sent.
InOrder == \A i \in read' \ read : \A j \in 1..i - 1 : j \in read
\* The practice writes once Coda has read the patient's message.
PracticeAfterPatient == event'.name = "the practice sends in Spruce" => \E i \in read : inbox[i].from = "Patient"
\* A physician moves a conversation whose reply they have open.
MovesASeenReply == event'.name \in {"a physician moves it to another patient", "a physician moves it to not a patient"}
                     => reply.open /\ \E d \in Physicians : viewing[d]
\* The practice disconnects Spruce while a reply is open.
DisconnectsAnOpenReply == event'.name = "the practice disconnects Spruce" => reply.open
Likely == InOrder /\ PracticeAfterPatient /\ MovesASeenReply /\ DisconnectsAnOpenReply

Base == {"the patient writes", "Coda reads the channel", "Coda's review reads the conversation", "Coda's review finishes",
         "you open the reply"}
ReplySteps == Likely /\ Named(Base \cup {"you copy the reply", "the practice sends in Spruce", "you mark No reply needed"})
DraftSteps == Likely /\ Named(Base \cup {"you edit", "you take Coda's update"})
ReviseSteps == Likely /\ Named(Base \cup {"you edit", "you ask Coda to revise", "Coda's revision lands", "you restore your version"})
TwoSteps == Likely /\ Named(Base \cup {"you edit", "you copy the reply", "you mark No reply needed"})
MoveSteps == Likely /\ Named(Base \cup {"you edit", "a physician moves it to another patient", "a physician moves it to not a patient",
                                       "a physician undoes the move", "Undo expires", "Coda repairs what relied on it"})
DisconnectSteps == Likely /\ Named(Base \cup {"the practice sends in Spruce", "the practice disconnects Spruce",
                                             "the practice reconnects Spruce", "the disconnect completes", "you mark it Done"})
ConcernSteps == Likely /\ Named(Base \cup {"you mark No reply needed", "you undo No reply needed", "Undo expires",
                                          "you mark the concern Done"})
=============================================================================

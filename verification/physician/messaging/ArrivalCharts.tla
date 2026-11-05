--------------------------- MODULE ArrivalCharts ---------------------------
(***************************************************************************)
(* The steps each chart of Arrival.tla draws. A chart runs the model's own *)
(* Spec from Init, with every sender and channel tie the config allows, at *)
(* a small scope its config sets. Its ACTION_CONSTRAINT keeps the steps of *)
(* its journey in their likely order, so rare interleavings stay in the    *)
(* full check and out of the chart.                                         *)
(***************************************************************************)
EXTENDS Arrival

Named(names) == event'.name \in names

\* Coda reads the channel's items in the order they were sent.
InOrder == \A i \in read' \ read : \A j \in 1..i - 1 : j \in read
\* A physician moves a conversation once its reply is open.
MovesAnOpenReply == event'.name \in {"you move it to another patient", "you move it to not a patient"} => reply.open
\* The patient writes again after the move.
WritesAgainAfterAMove == event'.name = "the patient writes" /\ inbox # <<>> => corrections > 0
Likely == InOrder /\ MovesAnOpenReply

Base == {"the patient writes", "Coda reads the channel", "Coda decides it's from a patient"}
Linking == Base \cup {"you link it to the patient"}
LinkSteps == Likely /\ Named(Linking \cup {"15 minutes pass", "Coda judges it isn't from a patient", "This is a patient",
                                       "you leave it unlinked"})
MoveSteps == Likely /\ WritesAgainAfterAMove
             /\ Named(Base \cup {"Coda's review reads the conversation", "Coda's review finishes", "you move it to another patient",
                                 "you move it to not a patient", "you undo the move", "Undo expires",
                                 "Coda repairs what relied on it"})
StopSteps == Likely /\ Named(Linking \cup {"Coda can't read Spruce", "Coda reads Spruce again", "the practice reconnects Spruce",
                                       "the practice disconnects Spruce", "the disconnect completes", "you leave it unlinked"})
ReadSteps == Likely /\ Named(Linking \cup {"Coda's review reads the conversation", "Coda's review finishes",
                                       "you mark the unreadable message Done", "you mark No reply needed"})
=============================================================================

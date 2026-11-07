---------------------------- MODULE MessageReply ----------------------------
(***************************************************************************)
(* How Coda proposes a reply to a patient's message, and how a physician  *)
(* revises it and sends it: the design in README.md. Each INVARIANT or    *)
(* PROPERTY is one rule, and quotes its sentence.                          *)
(*                                                                         *)
(* One conversation, linked to a patient by message matching. The state   *)
(* follows the README's two graphs. The conversation's graph: each       *)
(* patient message is open or handled, Coda is reviewing or not, and the  *)
(* reply is ready, withheld, closed with Undo, or none. Each physician's  *)
(* graph: no draft, your text, or Coda's revision of it, and the reply it *)
(* belongs to. Beside them are the concern, the practice's message, what  *)
(* the patient's history records, the physicians Coda offers its update   *)
(* to, and the channel. The card is not kept: Card(d) says what it shows  *)
(* from this state. Copying changes nothing, so the copied text is only   *)
(* in the step. Coda's own facts are the events waiting for a review and  *)
(* the running review.                                                     *)
(*                                                                         *)
(* One step is one thing a person, the practice, the channel or Coda      *)
(* does. A text is Coda's reply at a version, your text, Coda's revision  *)
(* of your text, the practice's own words, or an automatic reply. Whether *)
(* a reply would help, whether a review flags a concern, and whether the  *)
(* practice's message answers the patient are judgment: here each is a    *)
(* choice. An automatic reply never answers. Message matching owns        *)
(* linking, moves and their Undo; here "Coda acts on a move" is one step  *)
(* from outside, after that Undo went away. The model keeps your draft of *)
(* the reply in front of you; drafts of earlier replies stay as records.  *)
(*                                                                         *)
(* Which patients each physician may access is open, so it is an         *)
(* input: TLC checks the rules for every setting in which each patient   *)
(* is someone's. Patients and physicians are only names, and the rules    *)
(* treat each the same, so the conversation starts on one patient, and    *)
(* Init keeps one setting of each kind: any other is one of these with    *)
(* the names changed.                                                      *)
(*                                                                         *)
(* Time: a review that woke always starts, and a running review always    *)
(* reads what is new and finishes, however often Coda restarts; the "No  *)
(* reply needed" Undo always goes away unless someone presses it. Spec    *)
(* says so with fairness, and rules 2 and 9 check what follows from it.   *)
(*                                                                         *)
(* Not modeled, since they rest on records, judgment or screen copy: the *)
(* Today row's words, the sender's name, the card's order, Basis and      *)
(* Sources, formatting and unsafe content, citations, which conversations *)
(* need no reply, who copied and when, and the record's policy.           *)
(*                                                                         *)
(* `event` names the step just taken; VIEW leaves it out of the state.   *)
(***************************************************************************)
EXTENDS Naturals, FiniteSets, TLC

\* MessageReply.cfg sets the scope: two patients, so a move has a patient it goes to; two physicians, so
\* access, drafts and copies are different for each; two patient messages, so the patient can write again
\* after a reply or during Undo; one practice message; and one move.

CONSTANTS Patients, Physicians, MaxMessages, MaxMoves

Nobody == "Nobody"
NotPatient == "Not a patient"
Coda == "Coda"
ThePatient == "the patient"
ThePractice == "the practice"

\* Where a patient message is.
NotWritten == "Not written"
Open == "Open"
Answered == "Answered"
NeedsNoReply == "Needs no reply"
Done == "Marked Done"
MovedAway == "Moved to not a patient"

\* The reply.
None == "No reply"
Ready == "Ready"
Withheld == "Withheld"
Closed == "Closed, Undo shows"

\* A physician's draft.
NoDraft == "No draft"
Mine == "Your text"
Revised == "Coda's revision"

\* Texts.
Nothing == <<"Nothing", 0>>
OwnWords == <<"The practice's own words", 0>>
Automatic == <<"An automatic reply", 0>>
CodaText(v) == <<Coda, v>>

\* The channel.
Connected == "Connected"
Disconnected == "Disconnected"

Messages == 1..MaxMessages
\* The events that wake a review.
Wrote(i) == <<"the patient writes", i>>
PracticeMessage == <<"the practice's message", 0>>
Move == <<"the move", 0>>

NoReview == [for |-> Nobody, current |-> FALSE, sawSent |-> FALSE]
NoSent == [text |-> Nothing, answers |-> FALSE, asked |-> 0]
NoDraftHere == [kind |-> NoDraft, on |-> 0]

VARIABLES
  access,  \* input: the patients each physician may access
  link,    \* the patient the conversation is linked to, or Not a patient
  moves,   \* how many moves Coda has acted on
  written, \* how many messages the patient has written
  msg,     \* where each patient message is
  sent,    \* the practice's message: its text, whether it answers, and how many patient messages came before it
  history, \* the practice's words the patient's history records
  review,  \* the running review: its patient, whether it has read what is new, and whether it read the practice's message
  woke,    \* the events waiting for a review
  reply,   \* the reply: its state, version, which reply it is, its patient, and the messages it answers
  concern, \* Coda has flagged a concern about the messages
  draft,   \* each physician's draft: what it is, and the reply it belongs to
  offers,  \* the physicians whose draft is older than Coda's newest version of its reply
  channel, \* the channel, as Coda finds it
  event

state == <<access, link, moves, written, msg, sent, history, review, woke, reply, concern, draft, offers, channel>>
vars == <<access, link, moves, written, msg, sent, history, review, woke, reply, concern, draft, offers, channel, event>>

Start == CHOOSE p \in Patients : TRUE

\* The physicians in an order, and a key for each set of patients, so Init keeps one setting of each kind.
Order == CHOOSE f \in [1..Cardinality(Physicians) -> Physicians] : \A i, j \in DOMAIN f : i # j => f[i] # f[j]
Key(S) == 2 * Cardinality(S) + (IF Start \in S THEN 1 ELSE 0)

Init ==
  /\ access \in {a \in [Physicians -> SUBSET Patients] :
                  /\ \A p \in Patients : \E d \in Physicians : p \in a[d]
                  /\ \A i \in 1..Cardinality(Physicians) - 1 : Key(a[Order[i]]) <= Key(a[Order[i + 1]])}
  /\ link = Start
  /\ moves = 0
  /\ written = 0
  /\ msg = [i \in Messages |-> NotWritten]
  /\ sent = NoSent
  /\ history = Nothing
  /\ review = NoReview
  /\ woke = {}
  /\ reply = [state |-> None, v |-> 0, id |-> 0, for |-> Nobody, msgs |-> {}]
  /\ concern = FALSE
  /\ draft = [d \in Physicians |-> NoDraftHere]
  /\ offers = {}
  /\ channel = Connected
  /\ event = [name |-> "start", who |-> Nobody, text |-> Nothing]

Ev(name, who) == event' = [name |-> name, who |-> who, text |-> Nothing]

-----------------------------------------------------------------------------
(* What the physicians see, from the state.                                *)

OpenMsgs == {i \in Messages : msg[i] = Open}
Reviewing == woke # {} \/ review.for # Nobody
ReplyOpen == reply.state \in {Ready, Withheld}

\* You see the reply's card.
Sees(d) == ReplyOpen /\ reply.for \in access[d]

\* Your draft shows on the reply it belongs to.
ShowsDraft(d) == draft[d].kind # NoDraft /\ draft[d].on = reply.id

\* The reply text you see. A withheld reply shows no text.
Shown(d) == IF reply.state # Ready THEN Nothing ELSE IF ShowsDraft(d) THEN <<d, draft[d].kind>> ELSE CodaText(reply.v)

\* What a card shows: the open messages, the text, whether Coda is reviewing, whether the reply is
\* withheld, whether Coda offers its update, and the disconnect note.
Card(d) == [msgs |-> OpenMsgs, text |-> Shown(d), reviewing |-> Reviewing, withheld |-> reply.state = Withheld,
            offer |-> d \in offers, note |-> channel = Disconnected]

\* "Undo" goes away when something new arrives, or a review finishes.
EndUndo == IF reply.state = Closed THEN [reply EXCEPT !.state = None, !.msgs = {}] ELSE reply

-----------------------------------------------------------------------------
(* The conversation's graph.                                               *)

\* The patient writes, and the message wakes the review.
PatientWrites ==
  /\ channel = Connected /\ link \in Patients /\ written < MaxMessages
  /\ written' = written + 1
  /\ msg' = [msg EXCEPT ![written + 1] = Open]
  /\ woke' = woke \cup {Wrote(written + 1)}
  /\ reply' = EndUndo
  /\ UNCHANGED <<access, link, moves, sent, history, review, concern, draft, offers, channel>>
  /\ Ev("the patient writes", ThePatient)

\* A review starts for the conversation's patient, takes the events that woke it, and reads the full
\* conversation, with what the practice sent. The history records what it read.
ReviewStarts ==
  /\ woke # {} /\ review.for = Nobody /\ link \in Patients
  /\ review' = [for |-> link, current |-> TRUE, sawSent |-> sent.text # Nothing]
  /\ woke' = {}
  /\ history' = IF sent.text # Nothing THEN sent.text ELSE history
  /\ UNCHANGED <<access, link, moves, written, msg, sent, reply, concern, draft, offers, channel>>
  /\ Ev("a review starts", Coda)

\* Something new arrived, or Coda restarted: the running review reads the full conversation again.
ReviewReadsAgain ==
  /\ review.for # Nobody /\ (woke # {} \/ ~review.current)
  /\ review' = [review EXCEPT !.current = TRUE, !.sawSent = sent.text # Nothing]
  /\ woke' = {}
  /\ history' = IF sent.text # Nothing THEN sent.text ELSE history
  /\ UNCHANGED <<access, link, moves, written, msg, sent, reply, concern, draft, offers, channel>>
  /\ Ev("the review reads again", Coda)

\* Coda restarts during a review. The review goes on, and must read again.
CodaRestarts ==
  /\ review.for # Nobody
  /\ review' = [review EXCEPT !.current = FALSE]
  /\ UNCHANGED <<access, link, moves, written, msg, sent, history, woke, reply, concern, draft, offers, channel>>
  /\ Ev("Coda restarts", Coda)

(* The review finishes, having read everything. It closes as answered    *)
(* the open messages the practice's message answers. For the others, it  *)
(* writes a reply if one would help, and otherwise closes them as needing *)
(* no reply. A closed reply's "Undo" goes away. It can flag a concern.    *)
ReviewFinishes(helps, flag) ==
  /\ review.for = link /\ woke = {} /\ review.current
  /\ LET A == IF review.sawSent /\ sent.answers THEN {i \in OpenMsgs : i <= sent.asked} ELSE {}
         R == OpenMsgs \ A
     IN /\ helps \in (IF R = {} THEN {FALSE} ELSE BOOLEAN)
        /\ flag \in (IF R = {} THEN {FALSE} ELSE BOOLEAN)
        /\ msg' = [i \in Messages |-> IF i \in A THEN Answered
                                      ELSE IF i \in R /\ ~helps THEN NeedsNoReply ELSE msg[i]]
        /\ reply' = IF helps
                    THEN [state |-> Ready, v |-> reply.v + 1, id |-> IF ReplyOpen THEN reply.id ELSE reply.id + 1,
                          for |-> link, msgs |-> R]
                    ELSE IF OpenMsgs = {} THEN EndUndo ELSE [reply EXCEPT !.state = None, !.msgs = {}]
        /\ offers' = IF helps THEN {d \in Physicians : draft[d].kind # NoDraft /\ draft[d].on = reply'.id} ELSE offers
        /\ concern' = (concern \/ flag)
        /\ Ev(IF helps THEN "the review writes a reply" ELSE "the review finishes with no reply", Coda)
  /\ review' = NoReview
  /\ UNCHANGED <<access, link, moves, written, sent, history, woke, draft, channel>>

Sendable == {CodaText(k) : k \in 1..reply.v} \cup {<<d, k>> : d \in {e \in Physicians : link \in access[e]}, k \in {Mine, Revised}}
            \cup {OwnWords, Automatic}

\* The practice sends a message in the channel: a text a physician copied, its own words, or an automatic
\* reply. A copied text is Coda's reply at any version so far, or a physician's own text. It wakes the
\* review.
PracticeSends(t, answers) ==
  /\ channel = Connected /\ link \in Patients /\ written >= 1 /\ sent.text = Nothing
  /\ t \in Sendable
  /\ answers \in (IF t = Automatic THEN {FALSE} ELSE BOOLEAN)
  /\ sent' = [text |-> t, answers |-> answers, asked |-> written]
  /\ woke' = woke \cup {PracticeMessage}
  /\ reply' = EndUndo
  /\ UNCHANGED <<access, link, moves, written, msg, history, review, concern, draft, offers, channel>>
  /\ Ev("the practice sends a message", ThePractice)

(* Coda acts on a move. To another patient: the running review stops, an  *)
(* open reply is withheld as a new reply for the new patient, and the move *)
(* wakes that patient's review, whether or not a message is open. To not a *)
(* patient: the open messages are handled, and the reply and the concern   *)
(* close.                                                                  *)
CodaActsOnMove(q) ==
  /\ link \in Patients /\ moves < MaxMoves
  /\ q \in (Patients \ {link}) \cup {NotPatient}
  /\ moves' = moves + 1
  /\ link' = q
  /\ review' = NoReview
  /\ IF q = NotPatient
     THEN /\ msg' = [i \in Messages |-> IF msg[i] = Open THEN MovedAway ELSE msg[i]]
          /\ reply' = [reply EXCEPT !.state = None, !.msgs = {}]
          /\ concern' = FALSE
          /\ woke' = {}
          /\ Ev("Coda acts on a move to not a patient", Coda)
     ELSE /\ reply' = IF ReplyOpen THEN [reply EXCEPT !.state = Withheld, !.id = reply.id + 1, !.for = q] ELSE EndUndo
          /\ woke' = woke \cup {Move}
          /\ UNCHANGED <<msg, concern>>
          /\ Ev("Coda acts on a move to another patient", Coda)
  /\ offers' = {}
  /\ UNCHANGED <<access, written, sent, history, draft, channel>>

ChannelDisconnects ==
  /\ channel = Connected
  /\ channel' = Disconnected
  /\ UNCHANGED <<access, link, moves, written, msg, sent, history, review, woke, reply, concern, draft, offers>>
  /\ Ev("the channel is disconnected", Nobody)

CodaReconnects ==
  /\ channel = Disconnected
  /\ channel' = Connected
  /\ UNCHANGED <<access, link, moves, written, msg, sent, history, review, woke, reply, concern, draft, offers>>
  /\ Ev("Coda reconnects and sees the reply", Coda)

-----------------------------------------------------------------------------
(* The physician's steps on the card.                                      *)

Keep == UNCHANGED <<access, link, moves, written, sent, history, review, woke, channel>>

\* You edit the reply: it becomes your text, on this reply.
YouEdit(d) ==
  /\ Sees(d) /\ reply.state = Ready
  /\ draft' = [draft EXCEPT ![d] = [kind |-> Mine, on |-> reply.id]]
  /\ Keep /\ UNCHANGED <<msg, reply, concern, offers>>
  /\ Ev("you edit your draft", d)

\* You ask Coda to revise your text.
YouAskRevise(d) ==
  /\ Sees(d) /\ reply.state = Ready /\ ShowsDraft(d) /\ draft[d].kind = Mine
  /\ draft' = [draft EXCEPT ![d].kind = Revised]
  /\ Keep /\ UNCHANGED <<msg, reply, concern, offers>>
  /\ Ev("you ask Coda to revise", d)

\* "Restore my version" brings back your own text.
YouRestore(d) ==
  /\ Sees(d) /\ reply.state = Ready /\ ShowsDraft(d) /\ draft[d].kind = Revised
  /\ draft' = [draft EXCEPT ![d].kind = Mine]
  /\ Keep /\ UNCHANGED <<msg, reply, concern, offers>>
  /\ Ev("you restore your version", d)

\* Coda wrote a newer reply, and you choose its update over your draft.
YouChooseUpdate(d) ==
  /\ Sees(d) /\ reply.state = Ready /\ Card(d).offer
  /\ draft' = [draft EXCEPT ![d] = NoDraftHere]
  /\ offers' = offers \ {d}
  /\ Keep /\ UNCHANGED <<msg, reply, concern>>
  /\ Ev("you choose Coda's update", d)

\* "Copy reply" copies the text on the card, and changes nothing.
YouCopy(d) ==
  /\ Sees(d) /\ reply.state = Ready
  /\ UNCHANGED state
  /\ event' = [name |-> "you copy the reply", who |-> d, text |-> Card(d).text]

\* You close the reply for all physicians: its messages need no reply. "Undo" shows.
YouCloseNoReply(d) ==
  /\ Sees(d) /\ reply.state = Ready
  /\ msg' = [i \in Messages |-> IF i \in reply.msgs /\ msg[i] = Open THEN NeedsNoReply ELSE msg[i]]
  /\ reply' = [reply EXCEPT !.state = Closed]
  /\ Keep /\ UNCHANGED <<concern, draft, offers>>
  /\ Ev("you close it: no reply needed", d)

\* "Undo" brings back the reply it closed, with its messages.
YouUndoClose(d) ==
  /\ reply.state = Closed /\ reply.for \in access[d]
  /\ reply' = [reply EXCEPT !.state = Ready]
  /\ msg' = [i \in Messages |-> IF i \in reply.msgs /\ msg[i] = NeedsNoReply THEN Open ELSE msg[i]]
  /\ Keep /\ UNCHANGED <<concern, draft, offers>>
  /\ Ev("you press Undo", d)

\* "Undo" goes away.
UndoGoesAway ==
  /\ reply.state = Closed
  /\ reply' = EndUndo
  /\ Keep /\ UNCHANGED <<msg, concern, draft, offers>>
  /\ Ev("Undo goes away", Coda)

\* You close the concern.
YouCloseConcern(d) ==
  /\ concern /\ link \in access[d]
  /\ concern' = FALSE
  /\ Keep /\ UNCHANGED <<msg, reply, draft, offers>>
  /\ Ev("you close the concern", d)

\* The channel is disconnected, and you mark the open reply Done: its messages are handled.
YouMarkDone(d) ==
  /\ Sees(d) /\ Card(d).note
  /\ msg' = [i \in Messages |-> IF i \in reply.msgs /\ msg[i] = Open THEN Done ELSE msg[i]]
  /\ reply' = [reply EXCEPT !.state = None, !.msgs = {}]
  /\ Keep /\ UNCHANGED <<concern, draft, offers>>
  /\ Ev("you mark it Done", d)

-----------------------------------------------------------------------------

Finishes == \E helps, flag \in BOOLEAN : ReviewFinishes(helps, flag)
Sends == \E t \in Sendable, answers \in BOOLEAN : PracticeSends(t, answers)

Busy ==
  \/ PatientWrites \/ ReviewStarts \/ ReviewReadsAgain \/ CodaRestarts \/ Finishes
  \/ Sends
  \/ \E q \in Patients \cup {NotPatient} : CodaActsOnMove(q)
  \/ ChannelDisconnects \/ CodaReconnects \/ UndoGoesAway
  \/ \E d \in Physicians :
       \/ YouEdit(d) \/ YouAskRevise(d) \/ YouRestore(d) \/ YouChooseUpdate(d) \/ YouCopy(d)
       \/ YouCloseNoReply(d) \/ YouUndoClose(d) \/ YouCloseConcern(d) \/ YouMarkDone(d)

\* Nothing is left to do.
Rest == ~ENABLED Busy /\ UNCHANGED vars

Next == Busy \/ Rest

(* A review that woke always starts, a running review always reads what   *)
(* is new and finishes, however often Coda restarts, and Undo goes away    *)
(* unless someone presses it.                                              *)
Fairness == /\ WF_vars(ReviewStarts) /\ WF_vars(ReviewReadsAgain) /\ SF_vars(Finishes)
            /\ WF_vars(UndoGoesAway)

Spec == Init /\ [][Next]_vars /\ Fairness

-----------------------------------------------------------------------------
(* The rules, in the README's numbering.                                   *)

\* 1. You see a reply or your draft, and act on it, only while you may access its patient.
OnlyWithAccess ==
  /\ [](\A d \in Physicians : Sees(d) => link \in access[d])
  /\ [][event'.who \in Physicians => link \in access[event'.who]]_vars

FinishSteps == {"the review writes a reply", "the review finishes with no reply"}

\* 2. Only a review that read the practice's message closes a reply as answered, an automatic reply never
\* answers it, and the messages a practice message answers always end handled.
OnlyTheReviewAnswers ==
  /\ [][\A i \in Messages : msg[i] # Answered /\ msg'[i] = Answered =>
          /\ event'.name \in FinishSteps /\ review.sawSent
          /\ sent.text # Automatic /\ sent.answers /\ i <= sent.asked]_vars
  /\ sent.answers ~> \A i \in Messages : i <= sent.asked => msg[i] # Open

\* 3. Closing a reply never closes a concern about the same messages, and a move to not a patient closes
\* both.
ConcernStays ==
  [][/\ concern /\ ~concern' => event'.name \in {"you close the concern", "Coda acts on a move to not a patient"}
     /\ event'.name = "Coda acts on a move to not a patient" => ~concern' /\ reply'.state = None]_vars

\* 4. Coda never replaces your draft unless you ask it to revise or choose its update, one physician never
\* changes another's draft, your draft shows only on the reply you wrote it for, and without a draft you see
\* Coda's newest reply.
YourDraft ==
  /\ [][\A d \in Physicians : draft'[d] # draft[d] =>
          /\ event'.who = d
          /\ draft'[d].kind = Mine \/ event'.name \in {"you ask Coda to revise", "you choose Coda's update"}]_vars
  /\ [](\A d \in Physicians : Sees(d) /\ reply.state = Ready =>
          Card(d).text = IF draft[d].kind # NoDraft /\ draft[d].on = reply.id THEN <<d, draft[d].kind>> ELSE CodaText(reply.v))

\* 5. "Copy reply" copies exactly the text you see, and changes nothing else.
CopiesWhatYouSee ==
  [][event'.name = "you copy the reply" =>
       /\ event'.text = Card(event'.who).text
       /\ UNCHANGED state]_vars

\* 6. The card shows every patient message linked after its reply was written, and says "Coda is reviewing"
\* until the reply answers it.
CardShowsNew == \A d \in Physicians : Sees(d) => \A i \in OpenMsgs :
  /\ i \in Card(d).msgs
  /\ i \notin reply.msgs \/ reply.state = Withheld => Card(d).reviewing

\* 7. One message, practice message or move starts at most one review for each patient it affects.
\* An event wakes a review only in the step it happens. A review starts only for an event that waits,
\* and a review that starts or reads again takes every event that waits.
OneReviewEach ==
  [][/\ woke' \ woke \subseteq CASE event'.name = "the patient writes" -> {Wrote(written')}
                                 [] event'.name = "the practice sends a message" -> {PracticeMessage}
                                 [] event'.name = "Coda acts on a move to another patient" -> {Move}
                                 [] OTHER -> {}
     /\ review.for = Nobody /\ review'.for # Nobody => woke # {} /\ woke' = {}
     /\ event'.name = "the review reads again" => woke' = {}]_vars

\* 8. What the patient's history says the practice sent is what the channel shows it sent.
HistoryTrue == history \in {Nothing, sent.text}

\* 9. Every linked patient message shows, as "Coda is reviewing" or on a ready reply that answers it, to each
\* physician who may access its patient, until it's handled.
\* Someone may access its patient, and Coda keeps working until the review is done.
ShowsUntilHandled ==
  /\ [](\A i \in OpenMsgs :
          /\ \E d \in Physicians : link \in access[d]
          /\ \A d \in Physicians : link \in access[d] => Reviewing \/ (Sees(d) /\ reply.state = Ready /\ i \in reply.msgs))
  /\ Reviewing ~> ~Reviewing

\* 10. After Coda acts on a move to another patient, the reply is withheld until a review of the new patient
\* writes it again, or it closes.
WithheldAfterMove ==
  /\ [](\A d \in Physicians : reply.state = Withheld /\ Sees(d) => Card(d).withheld /\ Card(d).text = Nothing)
  /\ [][/\ event'.name = "Coda acts on a move to another patient" /\ ReplyOpen =>
             reply'.state = Withheld /\ reply'.for = link'
        /\ reply.state = Withheld /\ reply'.state \notin {Withheld, None} =>
             event'.name = "the review writes a reply" /\ review.for = link]_vars

\* 11. An open reply says the channel is disconnected exactly while it is.
NoteWhileDisconnected == \A d \in Physicians : Sees(d) => (Card(d).note <=> channel = Disconnected)

EndsUndo == {"the patient writes", "the practice sends a message", "Coda acts on a move to another patient",
             "Coda acts on a move to not a patient"} \cup FinishSteps

\* 12. "Undo" after "No reply needed" brings back the reply it closed, with its messages, and shows only until
\* something new arrives in the conversation or a review finishes.
UndoBringsBack ==
  [][/\ event'.name = "you press Undo" =>
          /\ reply' = [reply EXCEPT !.state = Ready]
          /\ \A i \in Messages : msg'[i] = IF i \in reply.msgs /\ msg[i] = NeedsNoReply THEN Open ELSE msg[i]
     /\ reply.state = Closed /\ event'.name \in EndsUndo => reply'.state # Closed]_vars
=============================================================================

# Physician journeys

Useful outcomes a physician reaches in [the graph](physician.mmd), each replayed
from the graph's start. Rules and proofs show what never happens; these show the
product does what it is for: each step is an event the graph allows at that
point, and the states after the arrow hold once it happens. A journey fails when
the graph stops allowing it, so a change that strands a useful outcome is caught.

Several journeys port the walks written for the earlier physician design
(journey.md's "Walk across the boundaries"); they keep the same outcome in
today's graph. They show the graph allows each outcome; how the app reaches it
is coverage.md's, and what each record holds is data-promises.md's.

Check it from the repository root:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs journey verification/physician-experience/physician.mmd verification/physician-experience/journeys.md
```

When the graph changes a journey's events or states, update the journey in the
same PR, or say why its outcome is no longer intended.

## You sign in and see Today

1. sign in to the practice → Signed in to the practice
2. Today loads → Showing Today

## A patient's first records give Coda its initial understanding

From: You sign in and see Today

1. clinical access granted → Clinical access
2. new records arrive → Records kept.Records current
3. Coda starts the initial understanding
4. Coda finishes the initial understanding quietly → Initial understanding finished

## New records become a concern you can trace to the work that raised it

A concern always comes with the update that explains it, and opening it leads to
that work; Done clears it.

From: A patient's first records give Coda its initial understanding

1. new records arrive → patient review in hand
2. Coda starts the patient review
3. Coda finishes the patient review with an update for you → Needs attention; Written to you, new
4. open the patient
5. open the concern → Reading the concern
6. open the work that raised the concern → Reading past work
7. mark the concern done
8. your Done on the concern goes through → No concern needs you.Marked done

## Coda replies to a patient's message in your exact words

Coda proposes a reply; you edit it; Send sends exactly the text in your reply
box, on the linked conversation, shown as sent by Coda for you.

From: A patient's first records give Coda its initial understanding

1. connect the channel → Connecting the channel; You manage the messaging connection
2. the provider authorizes the channel → Channel live
3. a message arrives from a known sender → No correction waiting on Undo.Linked to the patient
4. Coda starts the patient review
5. Coda finishes the patient review with an update for you → Reply work open
6. open the patient
7. edit the reply → Your edited reply
8. send the reply
9. your Send on the reply goes through → Sending, as Coda for you
10. the channel sends the reply → No open reply.Closed, sent by Coda for you

## A message moved to another patient holds back what relied on it until Coda re-reviews

N2: when the correction takes effect, what relied on the moved message is
withheld, and the immediate re-review decides it from the patient's own record.

From: A patient's first records give Coda its initial understanding

1. connect the channel
2. the provider authorizes the channel → Channel live
3. a message arrives from a known sender
4. Coda starts the patient review
5. Coda finishes the patient review with an update for you → Reply work open; Needs attention
6. open the patient
7. another patient chosen for the message
8. your Move to another patient on the message match goes through → Moved to another patient, Undo shown
9. the correction's Undo ends → No correction waiting on Undo.Another patient's message; What relied on the moved message is withheld until Coda re-reviews; No open reply
10. Coda starts the patient review
11. Coda finishes the patient review with an update for you → Nothing withheld

## Undo puts a corrected message back, with your reply exactly as it was

From: A patient's first records give Coda its initial understanding

1. connect the channel
2. the provider authorizes the channel
3. a message arrives from a known sender
4. Coda starts the patient review
5. Coda finishes the patient review with an update for you → Reply work open
6. open the patient
7. edit the reply → Your edited reply
8. the message is marked not a patient
9. your Mark not a patient on the message match goes through → Marked not a patient, Undo shown
10. undo the correction
11. your Undo the correction on the message match goes through → No correction waiting on Undo.Linked to the patient; Your edited reply; Nothing withheld

## A reply revision takes in what arrived while it worked before it finishes

Pinned sources: the revision finishes having read everything that arrived before
it finished.

From: A patient's first records give Coda its initial understanding

1. connect the channel
2. the provider authorizes the channel
3. a message arrives from a known sender
4. Coda starts the patient review
5. Coda finishes the patient review with an update for you → Reply work open
6. open the patient
7. ask Coda to revise the reply
8. your Ask Coda to revise on the reply goes through → reply revision in hand
9. Coda starts the reply revision
10. new records arrive → Something new arrived since it started@revision
11. Coda takes in what arrived in the reply revision → Nothing new since it started@revision
12. Coda finishes the reply revision quietly → reply revision not in hand; Reply work open

## You ask Coda about a patient and get an answer

From: A patient's first records give Coda its initial understanding

1. open the patient
2. type in the composer
3. send
4. your Send on the conversation goes through
5. Coda starts the answer
6. Coda finishes the answer quietly → answer not in hand.answer given

## A moved visit keeps its earlier briefing as history

The old briefing stays readable as history; the moved visit doesn't silently
inherit it.

From: A patient's first records give Coda its initial understanding

1. a visit is scheduled
2. the visit comes into preparation range → Visit ahead.Upcoming, in preparation range
3. Coda starts the visit preparation
4. Coda finishes the visit preparation with an update for you → Briefing ready for the upcoming visit
5. the visit is moved → Earlier briefing, kept as history

## A follow-up comes due, Coda reviews it, and you complete it

Clearing a concern doesn't end the follow-up; completing it does.

From: A patient's first records give Coda its initial understanding

1. open the patient
2. create a follow-up
3. your Create on the follow-up goes through → Active follow-up.Not yet due
4. the follow-up comes due → Active follow-up.Due, its review owed
5. Coda starts the follow-up review
6. Coda finishes the follow-up review with an update for you → Active follow-up
7. complete the follow-up
8. your Complete on the follow-up goes through → Completed

## Your edited reply stays yours when the patient writes again

Newer work offers Coda's update beside your text rather than replacing it; you
choose, and one restore point brings your text back.

From: A patient's first records give Coda its initial understanding

1. connect the channel
2. the provider authorizes the channel
3. a message arrives from a known sender
4. Coda starts the patient review
5. Coda finishes the patient review with an update for you → Reply work open
6. open the patient
7. edit the reply → Your edited reply
8. the patient writes again → Reviewing a new message; Your edited reply
9. Coda starts the patient review
10. Coda finishes the patient review with an update for you → Reply ready.Ready to send or copy; Your reply, with Coda's update offered
11. use Coda's update
12. your Use Coda's update on the reply goes through → Coda's text replaced yours, Restore available
13. restore my version
14. your Restore my version on the reply goes through → Your edited reply

## Clearing a concern leaves its follow-up running

Done is about the concern; the follow-up carries on until it is completed or
cancelled.

From: A patient's first records give Coda its initial understanding

1. open the patient
2. create a follow-up
3. your Create on the follow-up goes through → Active follow-up.Not yet due
4. new records arrive
5. Coda starts the patient review
6. Coda finishes the patient review with an update for you → Needs attention; Active follow-up.Not yet due
7. open the concern
8. mark the concern done
9. your Done on the concern goes through → No concern needs you.Marked done; Active follow-up.Not yet due
10. the follow-up comes due → Active follow-up.Due, its review owed

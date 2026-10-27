# Connected care journeys

Useful outcomes across Duet and the physician web app in [the
composition](connection.mmd), each replayed from both graphs' start. Rules and
proofs show what never happens; these show the connection does what it is for:
each step is an event the composition allows at that point, and the states after
the arrow hold once it happens, on both sides. A state is named as `duet:` or
`physician:` and its caption, as in the composition's rules.

They port the earlier Duet examples of joining, sharing, leaving and discharge
(Dev's October 22 answers Q15 to Q17 and Q20) to today's graphs. What reaches the
practice, for which patient and at which version, is data/shared_records.rs's.

Check it from the repository root:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs journey verification/connected-care/connection.mmd verification/connected-care/journeys.md
```

When either graph or the composition changes a journey's events or states,
update the journey in the same PR, or say why its outcome is no longer intended.

## A set-up patient and a signed-in physician

1. sign in → duet: Agreeing to the Terms
2. agree and continue
3. your Agree and continue on your setup goes through
4. save your display name
5. your Save name on your setup goes through
6. not now for AI
7. your Not now on your setup goes through → duet: Set up; duet: No practice connection
8. sign in to the practice → physician: Signed in to the practice
9. identity access granted → physician: Not connected

## The practice invites the patient, who joins

Joining is by invitation only (Q15, Q17).

From: A set-up patient and a signed-in physician

1. invite the patient
2. your Invite on the patient's connection goes through → physician: Invited
3. the invitation is delivered → duet: A practice's invitation waits for you; physician: Invited.Invitation sent
4. join your practice
5. your Share on your practice goes through → duet: Connected to your practice, sharing; physician: Connected

## A cancelled invitation leaves Duet with no practice

From: A set-up patient and a signed-in physician

1. invite the patient
2. your Invite on the patient's connection goes through
3. the invitation is delivered → duet: A practice's invitation waits for you
4. cancel the invitation
5. your Cancel the invitation on the patient's connection goes through → duet: No practice connection; physician: Not connected

## What the patient shares reaches the practice while connected

Each change to what the patient shares is news for the practice's agents, here
taken in by the initial understanding already running; saving For my doctor
shares nothing, and sending it does.

From: The practice invites the patient, who joins

1. Coda starts the initial understanding → physician: initial understanding in hand.initial understanding active.Coda is working on the initial understanding; physician: Nothing new since it started@understanding
2. add a note
3. your Add on the note goes through → duet: Note saved; physician: Something new arrived since it started@understanding
4. Coda takes in what arrived in the initial understanding → physician: Nothing new since it started@understanding
5. save For my doctor
6. your Save on For my doctor goes through → duet: For my doctor saved; physician: Nothing new since it started@understanding
7. send For my doctor to your practice
8. your Send to my practice on For my doctor goes through → physician: Something new arrived since it started@understanding

## The patient leaves, and both sides show the connection ended

Leaving ends sharing; the patient's account and history stay (Q16).

From: The practice invites the patient, who joins

1. leave your practice
2. your Leave on your practice goes through → duet: Connection ended.You left your practice; physician: Connection ended.Ended, the patient left; duet: Set up

## The practice discharges the patient, who keeps Duet

From: The practice invites the patient, who joins

1. discharge the patient
2. your Discharge on the patient's connection goes through → duet: Connection ended.Your practice ended the connection, with a dated notice; physician: Connection ended.Ended, discharged by the practice; duet: Set up

## Deleting the account ends the connection for the practice too

From: The practice invites the patient, who joins

1. delete your account
2. your Delete account on your account goes through → duet: Deleting your account; physician: Connection ended.Ended, the patient's account was deleted
3. the deletion finishes → duet: No account; physician: Connection ended.Ended, the patient's account was deleted

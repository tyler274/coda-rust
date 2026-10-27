# Duet journeys

Useful outcomes a person reaches in [the graph](duet.mmd), each replayed from the
graph's start. Rules and proofs show what never happens; these show the product
does what it is for: each step is an event the graph allows at that point, and
the states after the arrow hold once it happens. A journey fails when the graph
stops allowing it, so a change that strands a useful outcome is caught.

Several journeys port examples written for the earlier Duet design (Dev's
October 22 answers); they keep the same outcome in today's graph. They show the
graph allows each outcome; how the app reaches it is coverage.md's, and what
each record holds is data-promises.md's.

Check it from the repository root:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs journey verification/duet-experience/duet.mmd verification/duet-experience/journeys.md
```

When the graph changes a journey's events or states, update the journey in the
same PR, or say why its outcome is no longer intended.

## You set up Duet and talk to it, with no practice or records

Conversation comes first; records and a practice are optional (Q1).

1. sign in → Agreeing to the Terms
2. agree and continue
3. your Agree and continue on your setup goes through → Choosing a display name
4. save your display name
5. your Save name on your setup goes through → Choosing about AI
6. allow AI
7. your Allow AI on your setup goes through → Set up; AI allowed
8. type in the composer
9. send your message → Sending
10. Duet receives your message → Sent; Duet is writing
11. Duet finishes the reply → Reply finished

## A message whose answer was lost still arrives once

From: You set up Duet and talk to it, with no practice or records

1. type in the composer
2. send your message → Sending
3. your message isn't confirmed → Not confirmed yet
4. try sending again → Sending
5. Duet receives your message → Sent

## A refused message comes back to edit and goes again

From: You set up Duet and talk to it, with no practice or records

1. type in the composer
2. send your message → Sending
3. Duet refuses your message → Not sent, with Try again and Edit
4. edit the unsent message → Nothing on its way
5. send your message → Sending
6. Duet receives your message → Sent

## A new message steers the reply in progress

From: You set up Duet and talk to it, with no practice or records

1. type in the composer
2. send your message
3. Duet receives your message → Duet is writing
4. type in the composer
5. send your message → Sending; Duet is writing
6. Duet receives your message → Sent; Duet is writing
7. Duet finishes the reply → Reply finished

## A Reminder keeps working with AI off, and pauses and resumes as you choose

A fixed Reminder needs no AI (Q5); pausing and resuming are yours.

From: You set up Duet and talk to it, with no practice or records

1. type in the composer
2. send your message
3. Duet receives your message → Duet is writing
4. Duet saves the Reminder you asked for → Reminder scheduled.Reminder active
5. Duet finishes the reply
6. turn AI off
7. your Turn off on your AI settings goes through → AI off
8. Duet delivers the Reminder → Reminder scheduled.Reminder active
9. pause the Reminder
10. your Pause on the Reminder goes through → Reminder paused.Reminder paused by you
11. resume the Reminder
12. your Resume on the Reminder goes through → Reminder scheduled.Reminder active

## A Check-in comes due, runs and waits for your answer

From: You set up Duet and talk to it, with no practice or records

1. type in the composer
2. send your message
3. Duet receives your message
4. Duet saves the Check-in you asked for → Check-in scheduled.Check-in active
5. Duet finishes the reply
6. the Check-in comes due → Check-in due
7. Duet starts the Check-in → Check-in running
8. Duet delivers the Check-in → Check-in scheduled.Check-in active; A delivered Check-in waits for your answer

## You join your practice when its invitation reaches you

Joining is by invitation only, one practice (Q15, Q17).

From: You set up Duet and talk to it, with no practice or records

1. your practice's invitation reaches you → A practice's invitation waits for you
2. join your practice
3. your Share on your practice goes through → Connected to your practice, sharing

## You review Duet's update for your doctor against your latest version, save it and send it

A proposal saves nothing by itself; one your edits overtook is reviewed again
first, and the practice gets the saved version.

From: You join your practice when its invitation reaches you

1. type in the composer
2. send your message
3. Duet receives your message → Duet is writing
4. Duet proposes an update for your doctor → An update proposed.An update to review
5. Duet finishes the reply
6. save For my doctor
7. your Save on For my doctor goes through → For my doctor saved; An update proposed.An update to review again
8. review the update again → An update proposed.An update to review
9. save the update for your doctor
10. your Save the update on For my doctor goes through → For my doctor saved
11. send For my doctor to your practice
12. your Send to my practice on For my doctor goes through → For my doctor saved; Connected to your practice, sharing

## Leaving your practice keeps your Duet

Leaving ends sharing; your account and history stay (Q16).

From: You join your practice when its invitation reaches you

1. leave your practice
2. your Leave on your practice goes through → Connection ended.You left your practice; Set up

## Your practice's discharge keeps your Duet, with a dated notice

From: You join your practice when its invitation reaches you

1. your practice discharges you → Connection ended.Your practice ended the connection, with a dated notice; Set up

## Duet's own work can be quiet, and a failed check is never shown as quiet

Quiet follows an actual review; a failure says so and offers Retry (Q3).

From: You set up Duet and talk to it, with no practice or records

1. Duet takes on its own work → Duet's own work running
2. Duet finishes its own work quietly → Quiet; Nothing Duet is working on
3. Duet takes on its own work → Duet's own work running
4. Duet's own work can't finish → Couldn't check, with Retry
5. retry the check → Duet's own work running
6. Duet finishes its own work with something to tell you → Nothing Duet is working on

## Supporting information shows what the reply cited, with a note when a newer version exists

Which version a reply cited, and whether a newer one exists, are records the
graph doesn't hold; that the cited version is what shows is a promise in
data-promises.md. This journey shows the note can appear when it loads.

From: You set up Duet and talk to it, with no practice or records

1. allow your phone's health data
2. clinical records arrive from your phone's health app → Records from your sources kept
3. type in the composer
4. send your message
5. Duet receives your message
6. Duet finishes the reply
7. open supporting information → Supporting information open
8. the supporting information loads → A note that a newer version exists

## Withdrawing AI permission ends the reply, and allowing it again lets Duet answer

From: You set up Duet and talk to it, with no practice or records

1. type in the composer
2. send your message
3. Duet receives your message → Duet is writing
4. withdraw AI permission
5. your Withdraw permission on your AI settings goes through → AI not allowed; Reply ended when AI permission was withdrawn
6. allow AI in settings
7. your Allow AI on your AI settings goes through → AI allowed
8. type in the composer
9. send your message
10. Duet receives your message → Duet is writing
11. Duet finishes the reply → Reply finished

## Duet tells you about its own work while you're away, and the notification brings you back

From: You set up Duet and talk to it, with no practice or records

1. turn on alerts for Duet's own work
2. your Turn on on your alerts goes through → Duet's own work alerts you
3. allow alerts → Alerts allowed on this device
4. leave Duet → Duet in the background
5. Duet takes on its own work
6. Duet finishes its own work with something to tell you → A notification from Duet waiting
7. open the notification → Duet on screen

## You export everything Duet keeps about you

From: You set up Duet and talk to it, with no practice or records

1. export your data
2. your Export on your account goes through → Preparing your export
3. your export is ready → Your export is ready to save

## You delete your account

From: You join your practice when its invitation reaches you

1. delete your account
2. your Delete account on your account goes through → Deleting your account
3. the deletion finishes → No account; No practice connection

## Deleting a Plan keeps the Reminder you set for it

The Reminder is its own commitment: it stays, serving no Plan (Q19).

From: You set up Duet and talk to it, with no practice or records

1. type in the composer
2. send your message
3. Duet receives your message → Duet is writing
4. Duet saves the Plan you asked for → Plan saved
5. Duet saves the Reminder you asked for → Reminder scheduled.Reminder active
6. Duet links the Reminder to your Plan as you asked → Reminder serves your Plan
7. Duet finishes the reply
8. delete the Plan
9. your Delete on the Plan goes through → No Plan; Reminder scheduled.Reminder active; Reminder serves no Plan

## You keep a note of your own, with no records connected

From: You set up Duet and talk to it, with no practice or records

1. add a note
2. your Add on the note goes through → Note saved; No records from your sources yet

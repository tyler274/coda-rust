# Invitation

How a practice invites a patient to connect through Duet, and how the patient opens the invitation and joins. [connected-care.md](../../../intent/connected-care.md#c11-invitation) links here.

## Words

- **Member:** a person signed in to the practice's Chords.
- **Invitation:** from one practice to one patient on its chart, mailed to one email address. Coda prefills the email address from chart demographics when the chart has one, and the member can edit it or type an email address before sending.
- **Open:** an invitation that is sent and is neither accepted nor cancelled.
- **Link and short code:** the two ways into an invitation. Coda mails both, and both lead to the same invitation.
- **Date-of-birth check:** the patient enters their date of birth, and it matches when it equals the one on the practice's chart for that patient.
- **Locked:** an invitation that takes no more date-of-birth answers, after five wrong ones.
- **Share:** the patient's one tap that grants consent and connects them to the practice.
- **Connection state:** Not invited, Invited, Connected or Ended. Chords and Duet read it from the one connection record.

## States and steps

The invitation, as the practice sees it on the patient's page:

| From | Step | To |
| --- | --- | --- |
| Not invited | a member invites the patient at an email address prefilled from chart demographics or entered by the member | Invited: Coda mails the link and the short code |
| Invited | the mail can't be sent | Invited, and Chords says the send failed |
| Invited, unlocked | a member resends | Invited: Coda mails the same invitation again |
| Invited | a member cancels | Not invited: the link and the short code are closed |
| Invited, locked | a member cancels | Not invited, and a member can invite the patient again |
| Invited | the patient taps Share | Connected |
| Connected | the connection ends | Ended |
| Ended | a member invites the patient again | Invited |

The patient, with the email:

| From | Step | To |
| --- | --- | --- |
| Has the email | taps the link, with Duet installed | Duet opens at the invitation |
| Has the email | taps the link, without Duet | The App Store or Play Store, then Duet opens at the invitation |
| Has the email | enters the short code in Duet | Duet opens at the invitation |
| At the invitation, signed out | signs in, or signs up, by email | At the invitation, signed in |
| At the invitation, signed in | enters a date of birth that matches | Sees the practice, their physician and what the connection shares |
| At the invitation, signed in | enters a date of birth that doesn't match | Asked again, or Locked after five wrong answers |
| Locked | enters a date of birth | Refused: stays Locked and takes no more answers |
| Sees what the connection shares | taps Share | Connected, in Duet and in Chords |
| Sees what the connection shares | leaves without tapping Share | The invitation stays open, and nothing is shared |
| Any | the invitation is no longer open | Duet says so, in plain words |

## Rules

1. A patient and a practice have at most one open invitation.
2. A patient connects to a practice only by tapping Share on an invitation that the practice holds open at that moment.
3. Only Share grants consent: sending, resending, opening an invitation and passing the date-of-birth check share nothing.
4. A resend applies only to an open unlocked invitation, never makes a second invitation, never closes the open one, and never unlocks a locked invitation, so every link and short code already mailed for it still works.
5. After a member cancels an invitation, whether unlocked or locked, none of its links or short codes connects, and a member may invite the patient again.
6. The patient sees the practice, the physician and what the connection shares only after the date of birth they entered matches.
7. Five wrong date-of-birth answers lock an open invitation, and a locked invitation takes no more date-of-birth answers.
8. A mail that can't be sent leaves the invitation open.
9. Share connects the account that is signed in when the patient taps it, and never makes a second account.
10. An invitation from one practice never changes the patient's connection with another practice.
11. Chords and Duet show the same connection state.

These rules rest on the code and its records, not on the order of steps, so the model doesn't check them:

- When inviting, Coda prefills the patient's email address from chart demographics when the chart has one, and the member can edit it or type an email address before sending.
- Any member of the patient's practice can invite, resend and cancel. A member of another practice can't see the invitation or act on it.
- An invitation has no reminder and no expiry.
- A person who already has a Duet account can be invited, and joins with that account.
- Duet has no web surface, so every invitation opens the app.
- The practice gets no notification when a patient joins.

## Outside this spec

Signing in follows [Duet sign-in](../../duet/sign-in/README.md). What the connection shares, the records that reach the practice after Share, sending For my doctor, leaving and discharge follow [connected-care.md](../../../intent/connected-care.md).

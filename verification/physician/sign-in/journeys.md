# Sign-in journeys

What a physician can do in [the sign-in chart](sign-in.mjs), each step taken from
[physician.md A1](../../../intent/physician.md#a1-sign-in-and-access). Replay them from the
repository root:

```bash
node verification/statechart.mjs check verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/journeys.md
```

## You sign in from the email and see Today

1. request a sign-in link → Check your email
2. open the email button → Signing you in
3. verification succeeds → Your workspace; Signed in; Today

## Your session expires mid-work and you sign back in where you were

From: You sign in from the email and see Today

1. the session expires → Sign-in prompt over your work; Session expired
2. request a sign-in link → Sign-in prompt over your work
3. you sign in in another tab → Your workspace; Signed in

## A patient link opened while signed out lands on that patient

1. open a patient link while signed out → Enter your work email; Holding a patient link
2. request a sign-in link → Check your email; Holding a patient link
3. you sign in in another tab → Your workspace; The linked patient; No patient link

## A used or expired link is rejected and you ask for a new one

1. request a sign-in link → Check your email
2. open the email button → Signing you in
3. the link was used or has expired → This link no longer works
4. request a sign-in link → Check your email

## A link that no longer grants access says so

1. request a sign-in link → Check your email
2. open the email button → Signing you in
3. the link no longer grants access → No access to this practice; Signed out
4. use another email → Enter your work email

## Another physician's link signs you out and opens Today for them

From: You sign in from the email and see Today

1. open another physician's sign-in link → Signing you in; Signed out
2. verification succeeds → Your workspace; Signed in; Today

## Your own link changes nothing

From: You sign in from the email and see Today

1. open your own sign-in link → Your workspace; Signed in

## Closing Coda while signed out asks you to sign in again

From: Your session expires mid-work and you sign back in where you were

1. the session expires → Sign-in prompt over your work
2. close and reopen Coda → Enter your work email; Today

## You sign out

From: You sign in from the email and see Today

1. sign out → Enter your work email; Signed out

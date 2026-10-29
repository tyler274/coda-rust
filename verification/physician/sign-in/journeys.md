# Sign-in journeys

What a physician can do in [the sign-in chart](sign-in.mjs), told as stories. Replay them from the
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

## A sign-in from the new link's tab moves a rejected tab on

1. request a sign-in link → Check your email
2. open the email button → Signing you in
3. the link was used or has expired → This link no longer works
4. you sign in in another tab → Your workspace; Signed in; Today

## A link that no longer grants access says so

1. request a sign-in link → Check your email
2. open the email button → Signing you in
3. the link no longer grants access → No access to this practice; Signed out
4. use another email → Enter your work email

## Another physician's link signs you out and opens Today for them

From: You sign in from the email and see Today

1. open another physician's sign-in link → Signing you in; Signed out
2. verification succeeds → Your workspace; Signed in; Today

## Another physician's link opens Today, even after a link held earlier

1. open a patient link while signed out → Enter your work email; Holding a patient link
2. you sign in in another tab → Your workspace; Signed in; The linked patient
3. open another physician's sign-in link → Signing you in; Signed out; Today
4. the link was used or has expired → This link no longer works
5. you sign in in another tab → Your workspace; Signed in; Today

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

## A new tab opens your workspace while your session lasts

From: You sign in from the email and see Today

1. close and reopen Coda → Your workspace; Signed in; Today

## Coda can't send the link, and you try again

1. request a sign-in link → Check your email
2. Coda can't reach sign-in → Coda couldn't reach sign-in; Signed out
3. request a sign-in link → Check your email

## A rejected link offers another email

1. request a sign-in link → Check your email
2. open the email button → Signing you in
3. the link was used or has expired → This link no longer works
4. use another email → Enter your work email

## The practice removes your access mid-work

From: You sign in from the email and see Today

1. the practice removes your access → No access to this practice; Signed out; Today

## Another physician signs in elsewhere, and this tab opens Today for them

1. open a patient link while signed out → Enter your work email; Holding a patient link
2. another physician signs in in another tab → Your workspace; Signed in; No patient link; Today

## Sign-out forgets the patient you were on

1. open a patient link while signed out → Enter your work email; Holding a patient link
2. request a sign-in link → Check your email; Holding a patient link
3. you sign in in another tab → Your workspace; The linked patient; No patient link
4. sign out → Enter your work email; Signed out; Today

// How a physician signs in to Coda Agents. This chart is the sign-in spec; README.md
// says it in words. Open it in Stately Sketch, or check it with:
//   node verification/statechart.mjs check verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/journeys.md
import { setup, stateIn, not, and } from 'xstate';

const tab = 'This tab shows';
const session = 'Your session';
const link = 'A patient link this tab holds';
const opens = 'Where the workspace opens';

export const machine = setup({
  guards: {
    'signing in': stateIn({ [tab]: 'Signing you in' }),
    'not signing in': not(stateIn({ [tab]: 'Signing you in' })),
    'in your workspace': stateIn({ [tab]: 'Your workspace' }),
    'signed in': stateIn({ [session]: 'Signed in' }),
    'not signed in': not(stateIn({ [session]: 'Signed in' })),
    'not signed out': not(stateIn({ [session]: 'Signed out' })),
    'signed in, in your workspace': and([stateIn({ [session]: 'Signed in' }), stateIn({ [tab]: 'Your workspace' })]),
    'not signed in, and not signing in or prompted': and([
      not(stateIn({ [session]: 'Signed in' })),
      not(stateIn({ [tab]: 'Signing you in' })),
      not(stateIn({ [tab]: 'Sign-in prompt over your work' })),
    ]),
    'not signed in, holding a patient link': and([
      not(stateIn({ [session]: 'Signed in' })),
      stateIn({ [link]: 'Holding a patient link' }),
    ]),
  },
}).createMachine({
  id: 'Physician sign-in',
  type: 'parallel',
  meta: {
    events: {
      'request a sign-in link': { by: 'you', intent: 'Coda always answers "Check your email", and an address with no account gets no email.' },
      'use another email': { by: 'you', intent: 'Back to the email field.' },
      'open the email button': { by: 'you', intent: 'The emailed link opens in this tab and signs you in, with no code screen.' },
      'verification succeeds': { by: "Coda's sign-in service", intent: 'This tab opens Today. The email never carries a patient.' },
      'the link was used or has expired': { by: "Coda's sign-in service", intent: 'The link is rejected.' },
      'the link no longer grants access': { by: "Coda's sign-in service", intent: 'Shows "No access to this practice".' },
      "Coda can't reach sign-in": { by: "Coda's sign-in service", intent: 'Coda couldn\'t send the link or confirm it, and says so with a way to try again.' },
      'you sign in in another tab': { by: 'you', intent: 'Your session is shared across tabs, so this tab goes on to where you were, or to the patient link it held.' },
      'another physician signs in in another tab': { by: 'another physician', intent: 'The session is shared across tabs, so this tab becomes theirs and opens Today, never a patient this tab held.' },
      'the session expires': { by: "Coda's sign-in service", intent: 'A prompt covers your work.' },
      'the practice removes your access': { by: 'the practice', intent: 'You are signed out and told "No access to this practice".' },
      'sign out': { by: 'you', intent: 'Signs out every tab and forgets where you were.' },
      'open a patient link while signed out': { by: 'you', intent: 'This tab holds the link while you sign in.' },
      'close and reopen Coda': { by: 'you', intent: 'Opening Coda in a new tab or after closing it opens your workspace while your session lasts, and asks you to sign in once it has ended.' },
      "open another physician's sign-in link": { by: 'you', intent: 'Signs you out first, then signs them in and opens Today.' },
      'open your own sign-in link': { by: 'you', intent: 'Changes nothing.' },
    },
    never: [
      { name: 'WorkspaceAfterSignOut', states: { [tab]: 'Your workspace', [session]: 'Signed out' }, sentence: 'Once you sign out, your workspace is gone from this tab.' },
      { name: 'WorkspaceAfterExpiry', states: { [tab]: 'Your workspace', [session]: 'Session expired' }, sentence: 'When your session expires mid-work, a sign-in prompt covers the screen.' },
      { name: 'AskedToSignInWhileSignedIn', states: { [tab]: ['Enter your work email', 'Check your email', 'This link no longer works', 'No access to this practice', "Coda couldn't reach sign-in", 'Sign-in prompt over your work'], [session]: 'Signed in' }, sentence: 'While you are signed in, Coda never asks you to sign in or tells you sign-in failed.' },
      { name: 'NewSignInOpensToday', states: { [tab]: 'Signing you in', [opens]: 'The linked patient' }, sentence: "Another physician's sign-in link signs you out and opens Today for them." },
      { name: 'HeldLinkLands', states: { [tab]: 'Your workspace', [link]: 'Holding a patient link' }, sentence: 'A patient link opened while signed out lands on that patient once you sign in.' },
    ],
  },
  states: {
    [tab]: {
      initial: 'Enter your work email',
      states: {
        'Enter your work email': {
          on: {
            'request a sign-in link': 'Check your email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'another physician signs in in another tab': 'Your workspace',
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'Check your email': {
          on: {
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            "Coda can't reach sign-in": "Coda couldn't reach sign-in",
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in, and not signing in or prompted' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'Signing you in': {
          on: {
            'verification succeeds': { target: 'Your workspace', guard: 'signing in' },
            'the link was used or has expired': { target: 'This link no longer works', guard: 'signing in' },
            'the link no longer grants access': { target: 'No access to this practice', guard: 'signing in' },
            "Coda can't reach sign-in": "Coda couldn't reach sign-in",
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'This link no longer works': {
          on: {
            'request a sign-in link': 'Check your email',
            'use another email': 'Enter your work email',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in, and not signing in or prompted' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'No access to this practice': {
          on: {
            'use another email': 'Enter your work email',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in, and not signing in or prompted' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        "Coda couldn't reach sign-in": {
          on: {
            'request a sign-in link': 'Check your email',
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in, and not signing in or prompted' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'Your workspace': {
          on: {
            "open another physician's sign-in link": { target: 'Signing you in', guard: 'signed in' },
            'open your own sign-in link': { target: 'Your workspace', guard: 'signed in' },
            'the session expires': { target: 'Sign-in prompt over your work', guard: 'signed in' },
            'the practice removes your access': { target: 'No access to this practice', guard: 'signed in' },
            'sign out': { target: 'Enter your work email', guard: 'not signed out' },
            'another physician signs in in another tab': 'Your workspace',
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'Sign-in prompt over your work': {
          on: {
            'request a sign-in link': 'Sign-in prompt over your work',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'another physician signs in in another tab': 'Your workspace',
            'sign out': { target: 'Enter your work email', guard: 'not signed out' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
      },
    },
    [session]: {
      initial: 'Signed out',
      states: {
        'Signed out': {
          on: {
            'verification succeeds': { target: 'Signed in', guard: 'signing in' },
            'you sign in in another tab': { target: 'Signed in', guard: 'not signed in' },
            'another physician signs in in another tab': { target: 'Signed in', guard: 'not signing in' },
          },
        },
        'Signed in': {
          on: {
            'the session expires': { target: 'Session expired', guard: 'in your workspace' },
            'the practice removes your access': { target: 'Signed out', guard: 'in your workspace' },
            'sign out': { target: 'Signed out', guard: 'not signed out' },
            "open another physician's sign-in link": { target: 'Signed out', guard: 'in your workspace' },
          },
        },
        'Session expired': {
          on: {
            'verification succeeds': { target: 'Signed in', guard: 'signing in' },
            'you sign in in another tab': { target: 'Signed in', guard: 'not signed in' },
            'another physician signs in in another tab': { target: 'Signed in', guard: 'not signing in' },
            'sign out': { target: 'Signed out', guard: 'not signed out' },
          },
        },
      },
    },
    [link]: {
      initial: 'No patient link',
      states: {
        'No patient link': {
          on: {
            'open a patient link while signed out': { target: 'Holding a patient link', guard: 'not signed in, and not signing in or prompted' },
          },
        },
        'Holding a patient link': {
          on: {
            'you sign in in another tab': { target: 'No patient link', guard: 'not signed in' },
            'another physician signs in in another tab': { target: 'No patient link', guard: 'not signing in' },
            'open the email button': 'No patient link',
            'close and reopen Coda': 'No patient link',
          },
        },
      },
    },
    [opens]: {
      initial: 'Today',
      states: {
        Today: {
          on: {
            'you sign in in another tab': { target: 'The linked patient', guard: 'not signed in, holding a patient link' },
          },
        },
        'The linked patient': {
          on: {
            "open another physician's sign-in link": { target: 'Today', guard: 'signed in, in your workspace' },
            'the practice removes your access': { target: 'Today', guard: 'signed in, in your workspace' },
            'another physician signs in in another tab': { target: 'Today', guard: 'not signing in' },
            'sign out': { target: 'Today', guard: 'not signed out' },
            'close and reopen Coda': 'Today',
          },
        },
      },
    },
  },
});

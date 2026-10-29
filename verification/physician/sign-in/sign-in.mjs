// How a physician signs in, from physician.md A1.1–A1.5. Open it in Stately Sketch, or
// check it with: node verification/statechart.mjs check verification/physician/sign-in/sign-in.mjs
import { setup, stateIn, not, and } from 'xstate';

const tab = 'This tab shows';
const session = 'Your session';
const link = 'A patient link this tab holds';
const opens = 'Where the workspace opens';

export const machine = setup({
  guards: {
    'signing in': stateIn({ [tab]: 'Signing you in' }),
    'signed in': stateIn({ [session]: 'Signed in' }),
    'not signed in': not(stateIn({ [session]: 'Signed in' })),
    'not signed out': not(stateIn({ [session]: 'Signed out' })),
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
      'request a sign-in link': { by: 'you', intent: 'Coda always answers "Check your email"; an address with no account gets no email (physician A1.2).' },
      'use another email': { by: 'you', intent: 'Back to the email field.' },
      'open the email button': { by: 'you', intent: 'The emailed link opens in this tab and signs you in with no code screen (physician A1.2).' },
      'verification succeeds': { by: "Coda's sign-in service", intent: 'This tab opens Today; the email carries no patient (physician A1.3).' },
      'the link was used or has expired': { by: "Coda's sign-in service", intent: 'The link is rejected (physician A1.5).' },
      'the link no longer grants access': { by: "Coda's sign-in service", intent: 'Shows "No access to this practice" (physician A1.2).' },
      'you sign in in another tab': { by: 'you', intent: 'The emailed link signs you in from the tab it opens; this tab then goes on to where you were, or to the patient link it held (physician A1.3).' },
      'the session expires': { by: "Coda's sign-in service", intent: 'A prompt covers your work (physician A1.3).' },
      'sign out': { by: 'you', intent: 'Sign-out works (physician A1.5).' },
      'open a patient link while signed out': { by: 'you', intent: 'This tab holds the link while you sign in (physician A1.3).' },
      'close and reopen Coda': { by: 'you', intent: 'Reopening without a session asks you to sign in (physician A1.3).' },
      "open another physician's sign-in link": { by: 'you', intent: 'Signs you out and opens Today for them (physician A1.4).' },
      'open your own sign-in link': { by: 'you', intent: 'Changes nothing (physician A1.4).' },
    },
    never: [
      { name: 'WorkspaceAfterSignOut', states: { [tab]: 'Your workspace', [session]: 'Signed out' }, sentence: 'Sign-out works. (physician A1.5)' },
      { name: 'WorkspaceAfterExpiry', states: { [tab]: 'Your workspace', [session]: 'Session expired' }, sentence: 'If the session expires mid-work, a sign-in prompt covers the screen. (physician A1.3)' },
      { name: 'AskedToSignInWhileSignedIn', states: { [tab]: ['Enter your work email', 'Check your email', 'This link no longer works', 'No access to this practice', 'Sign-in prompt over your work'], [session]: 'Signed in' }, sentence: 'After signing in the physician is exactly where they were. (physician A1.3)' },
      { name: 'HeldLinkLands', states: { [tab]: 'Your workspace', [link]: 'Holding a patient link' }, sentence: 'A patient link opened while signed out lands that tab on the patient after sign-in. (physician A1.3)' },
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
          },
        },
        'Check your email': {
          on: {
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in, and not signing in or prompted' },
          },
        },
        'Signing you in': {
          on: {
            'verification succeeds': { target: 'Your workspace', guard: 'signing in' },
            'the link was used or has expired': { target: 'This link no longer works', guard: 'signing in' },
            'the link no longer grants access': { target: 'No access to this practice', guard: 'signing in' },
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
          },
        },
        'This link no longer works': {
          on: {
            'request a sign-in link': 'Check your email',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in, and not signing in or prompted' },
          },
        },
        'No access to this practice': {
          on: {
            'use another email': 'Enter your work email',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in, and not signing in or prompted' },
          },
        },
        'Your workspace': {
          on: {
            "open another physician's sign-in link": { target: 'Signing you in', guard: 'signed in' },
            'open your own sign-in link': { target: 'Your workspace', guard: 'signed in' },
            'the session expires': { target: 'Sign-in prompt over your work', guard: 'signed in' },
            'sign out': { target: 'Enter your work email', guard: 'not signed out' },
          },
        },
        'Sign-in prompt over your work': {
          on: {
            'request a sign-in link': 'Sign-in prompt over your work',
            'you sign in in another tab': { target: 'Your workspace', guard: 'not signed in' },
            'sign out': { target: 'Enter your work email', guard: 'not signed out' },
            'close and reopen Coda': 'Enter your work email',
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
          },
        },
        'Signed in': {
          on: {
            'the session expires': { target: 'Session expired', guard: 'signed in' },
            'sign out': { target: 'Signed out', guard: 'not signed out' },
            "open another physician's sign-in link": { target: 'Signed out', guard: 'signed in' },
          },
        },
        'Session expired': {
          on: {
            'verification succeeds': { target: 'Signed in', guard: 'signing in' },
            'you sign in in another tab': { target: 'Signed in', guard: 'not signed in' },
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
            'open the email button': 'No patient link',
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
            'verification succeeds': { target: 'Today', guard: 'signing in' },
            'sign out': { target: 'Today', guard: 'not signed out' },
            'close and reopen Coda': 'Today',
          },
        },
      },
    },
  },
});

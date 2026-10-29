// What the physician app's sign-in code does, at commit d878a812, drawn in the agreed
// chart's words so the two can be compared:
//   node verification/statechart.mjs compare verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/code.mjs
// Each event's `code` names where the code does it, under crates/physician-app/src. Words
// here describe this tab: "Your session" is the session this tab holds in sessionStorage,
// and a patient link is the ?patient= the page loaded with, which the tab keeps until a
// different physician's sign-in replaces it (PhysicianRoot, generation 0).
import { setup, stateIn, not, and, or } from 'xstate';

const tab = 'This tab shows';
const session = 'Your session';
const link = 'A patient link this tab holds';
const opens = 'Where the workspace opens';

const signInScreen = or(['Enter your work email', 'Check your email', 'Signing you in', 'This link no longer works',
  'No access to this practice', "Coda couldn't send or confirm the link"].map((s) => stateIn({ [tab]: s })));

export const machine = setup({
  guards: {
    'on a sign-in screen': signInScreen,
    'on a sign-in screen or the prompt': or([signInScreen, stateIn({ [tab]: 'Sign-in prompt over your work' })]),
    'in the workspace or the prompt': or([stateIn({ [tab]: 'Your workspace' }), stateIn({ [tab]: 'Sign-in prompt over your work' })]),
    'in the workspace': stateIn({ [tab]: 'Your workspace' }),
    'signing in': stateIn({ [tab]: 'Signing you in' }),
    'signed in': stateIn({ [session]: 'Signed in' }),
    'not signed in': not(stateIn({ [session]: 'Signed in' })),
    'holding a patient link': stateIn({ [link]: 'Holding a patient link' }),
    'opened at the linked patient': stateIn({ [opens]: 'The linked patient' }),
    'in the workspace, at the linked patient': and([stateIn({ [tab]: 'Your workspace' }), stateIn({ [opens]: 'The linked patient' })]),
  },
}).createMachine({
  id: 'Physician sign-in, as the code does it',
  type: 'parallel',
  meta: {
    events: {
      'request a sign-in link': { code: 'lib.rs SignInScreen request_token: Mailed shows Check your email; the prompt mails inside itself' },
      'use another email': { code: 'lib.rs SignInScreen "Use a different email" on Check your email, the rejection and failure screens, and No access' },
      'open the email button': { code: 'main.rs sign_in_link: the link loads this tab, and PhysicianRoot redeems it whatever the tab showed; the page has no ?patient=' },
      'verification succeeds': { code: 'lib.rs SignInScreen verify_link Ok: stores the session, announces it, signs this tab in' },
      'the link was used or has expired': { code: 'lib.rs verify_link 401: SIGN_IN_INVALID; the stored session is left as it was' },
      'the link no longer grants access': { code: 'lib.rs verify_link 403: NoAccess; the stored session is left as it was' },
      'you sign in in another tab': { code: 'lib.rs SessionState::hear SignedIn: a tab not signed in takes the session; a covered tab of the same physician resumes' },
      'the session expires': { code: 'lib.rs SessionState::refused SessionEnded with the page shown: the prompt covers it' },
      'sign out': { code: 'lib.rs sign_out_everywhere, from the shell or the prompt; PhysicianRoot keeps the ?patient= it loaded with' },
      'open a patient link while signed out': { code: 'main.rs open_patient; PhysicianRoot reads /today, and a refusal signs the tab out onto the sign-in page' },
      'close and reopen Coda': { code: 'lib.rs read_stored_session: a new tab has no sessionStorage, so its first read is refused' },
      "open another physician's sign-in link": { code: 'main.rs sign_in_link: the link loads this tab like any other; the old session stays stored until the new one replaces it' },
      'open your own sign-in link': { code: 'main.rs sign_in_link: the link loads this tab and is redeemed again; the new page opens Today' },
      'another physician signs in in another tab': { code: 'lib.rs SessionState::hear SignedIn: a tab not signed in takes any physician\'s session; a signed-in tab of another physician signs out and starts at Today (generation + 1)' },
      'the practice removes your access': { code: 'lib.rs SessionState::refused AccessRemoved: sign_out onto the sign-in page' },
      "Coda can't reach the sign-in service": { code: 'lib.rs request_token and verify_link failures: Recovery with SIGN_IN_UNAVAILABLE or "couldn\'t confirm sign-in"' },
    },
  },
  states: {
    [tab]: {
      initial: 'Enter your work email',
      states: {
        'Enter your work email': {
          on: {
            'request a sign-in link': 'Check your email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            "Coda can't reach the sign-in service": "Coda couldn't send or confirm the link",
          },
        },
        'Check your email': {
          on: {
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': 'Enter your work email',
            "Coda can't reach the sign-in service": "Coda couldn't send or confirm the link",
          },
        },
        'Signing you in': {
          on: {
            'verification succeeds': 'Your workspace',
            'the link was used or has expired': 'This link no longer works',
            'the link no longer grants access': 'No access to this practice',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': 'Enter your work email',
            "Coda can't reach the sign-in service": "Coda couldn't send or confirm the link",
          },
        },
        'This link no longer works': {
          on: {
            'request a sign-in link': 'Check your email',
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': 'Enter your work email',
            "Coda can't reach the sign-in service": "Coda couldn't send or confirm the link",
          },
        },
        'No access to this practice': {
          on: {
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': 'Enter your work email',
          },
        },
        "Coda couldn't send or confirm the link": {
          on: {
            'request a sign-in link': 'Check your email',
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': 'Enter your work email',
          },
        },
        'Your workspace': {
          on: {
            "open another physician's sign-in link": 'Signing you in',
            'open your own sign-in link': 'Signing you in',
            'the session expires': { target: 'Sign-in prompt over your work', guard: 'signed in' },
            'sign out': 'Enter your work email',
            'close and reopen Coda': 'Enter your work email',
            'the practice removes your access': 'Enter your work email',
          },
        },
        'Sign-in prompt over your work': {
          on: {
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'sign out': 'Enter your work email',
            'close and reopen Coda': 'Enter your work email',
            'open the email button': 'Signing you in',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
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
            'you sign in in another tab': { target: 'Signed in', guard: 'on a sign-in screen' },
            'another physician signs in in another tab': { target: 'Signed in', guard: 'on a sign-in screen' },
          },
        },
        'Signed in': {
          on: {
            'the session expires': 'Session expired',
            'sign out': { target: 'Signed out', guard: 'in the workspace or the prompt' },
            'close and reopen Coda': 'Signed out',
            'the practice removes your access': { target: 'Signed out', guard: 'in the workspace' },
          },
        },
        'Session expired': {
          on: {
            'verification succeeds': { target: 'Signed in', guard: 'signing in' },
            'you sign in in another tab': { target: 'Signed in', guard: 'on a sign-in screen or the prompt' },
            'another physician signs in in another tab': { target: 'Signed in', guard: 'on a sign-in screen or the prompt' },
            'sign out': { target: 'Signed out', guard: 'in the workspace or the prompt' },
            'open a patient link while signed out': 'Signed out',
            'close and reopen Coda': 'Signed out',
          },
        },
      },
    },
    [link]: {
      initial: 'No patient link',
      states: {
        'No patient link': {
          on: {
            'open a patient link while signed out': { target: 'Holding a patient link', guard: 'not signed in' },
            'sign out': { target: 'Holding a patient link', guard: 'opened at the linked patient' },
            'the practice removes your access': { target: 'Holding a patient link', guard: 'in the workspace, at the linked patient' },
          },
        },
        'Holding a patient link': {
          on: {
            'open the email button': 'No patient link',
            'you sign in in another tab': { target: 'No patient link', guard: 'on a sign-in screen' },
            'another physician signs in in another tab': { target: 'No patient link', guard: 'on a sign-in screen' },
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
            'you sign in in another tab': { target: 'The linked patient', guard: 'holding a patient link' },
            'another physician signs in in another tab': { target: 'The linked patient', guard: 'holding a patient link' },
          },
        },
        'The linked patient': {
          on: {
            'open the email button': { target: 'Today', guard: 'on a sign-in screen or the prompt' },
            "open another physician's sign-in link": { target: 'Today', guard: 'in the workspace' },
            'open your own sign-in link': { target: 'Today', guard: 'in the workspace' },
            'sign out': 'Today',
            'close and reopen Coda': 'Today',
            'the practice removes your access': { target: 'Today', guard: 'in the workspace' },
            'open a patient link while signed out': { target: 'Today', guard: 'not signed in' },
            'another physician signs in in another tab': { target: 'Today', guard: 'in the workspace or the prompt' },
          },
        },
      },
    },
  },
});

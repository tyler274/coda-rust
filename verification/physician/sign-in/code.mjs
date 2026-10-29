// What the physician app's sign-in code does, drawn in the agreed chart's words so the two
// can be compared:
//   node verification/statechart.mjs compare verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/code.mjs
// Each event's `code` names where the code does it, under crates/physician-app/src. "Your
// session" is the session this browser holds in localStorage, and a patient link is the
// ?patient= the page loaded with, which SessionState keeps as held_link until someone signs in,
// sign-out forgets it, or another physician's sign-in replaces it.
import { setup, stateIn, not, and, or } from 'xstate';

const tab = 'This tab shows';
const session = 'Your session';
const link = 'A patient link this tab holds';
const opens = 'Where the workspace opens';

const signInScreen = or(['Enter your work email', 'Check your email', 'Signing you in', 'This link no longer works',
  'No access to this practice', "Coda couldn't reach sign-in"].map((s) => stateIn({ [tab]: s })));

export const machine = setup({
  guards: {
    'on a sign-in screen': signInScreen,
    'on a sign-in screen or the prompt': or([signInScreen, stateIn({ [tab]: 'Sign-in prompt over your work' })]),
    'in the workspace or the prompt': or([stateIn({ [tab]: 'Your workspace' }), stateIn({ [tab]: 'Sign-in prompt over your work' })]),
    'in the workspace': stateIn({ [tab]: 'Your workspace' }),
    'signing in': stateIn({ [tab]: 'Signing you in' }),
    'signed in': stateIn({ [session]: 'Signed in' }),
    'not signed in': not(stateIn({ [session]: 'Signed in' })),
    'signed in, in the workspace': and([stateIn({ [session]: 'Signed in' }), stateIn({ [tab]: 'Your workspace' })]),
    'holding a patient link': stateIn({ [link]: 'Holding a patient link' }),
  },
}).createMachine({
  id: 'Physician sign-in, as the code does it',
  type: 'parallel',
  meta: {
    events: {
      'request a sign-in link': { code: 'lib.rs SignInScreen request_token: Mailed shows Check your email; the prompt mails inside itself' },
      'use another email': { code: 'lib.rs SignInScreen "Use a different email" on Check your email, the rejection and failure screens, and No access' },
      'open the email button': { code: 'main.rs sign_in_link: the link loads this tab, and SignInScreen redeems it; the page has no ?patient=' },
      'verification succeeds': { code: 'lib.rs SignInScreen verify_link Ok: stores the session, announces it, signs this tab in' },
      'the link was used or has expired': { code: 'lib.rs verify_link 401: SIGN_IN_INVALID' },
      'the link no longer grants access': { code: 'lib.rs verify_link 403: NoAccess' },
      "Coda can't reach sign-in": { code: 'lib.rs request_token and verify_link failures: Recovery with SIGN_IN_UNAVAILABLE or "couldn\'t confirm sign-in"' },
      'you sign in in another tab': { code: 'lib.rs SessionState::hear SignedIn, at once and without touching the server or storage: a tab not signed in takes the session and opens what it held; a covered tab of the same physician resumes' },
      'another physician signs in in another tab': { code: 'lib.rs SessionState::hear SignedIn, at once: a signed-in tab of another physician takes the new session, forgets its place and starts at Today; a tab not signed in cannot tell whose sign-in it is and takes it like its own' },
      'the session expires': { code: 'lib.rs SessionState::refused SessionEnded with the page shown: the prompt covers it' },
      'the practice removes your access': { code: 'lib.rs SessionState::refused AccessRemoved: signs out and SignInScreen opens at NoAccess' },
      'sign out': { code: 'lib.rs SessionState::end, from the shell or the prompt: ends the session, tells every tab, forgets the held link and the address; the other tabs follow in SessionState::hear without touching the server or storage' },
      'open a patient link while signed out': { code: 'main.rs open_patient; PhysicianRoot reads /today, and a refusal signs the tab out onto the sign-in page holding the link' },
      'close and reopen Coda': { code: 'lib.rs read_stored_session: localStorage outlives the tab, so a new page opens the workspace while the session lasts' },
      "open another physician's sign-in link": { code: 'lib.rs SignInScreen link effect: whoami names someone else or cannot say, so SessionState::end runs before the link is redeemed' },
      'open your own sign-in link': { code: 'lib.rs SignInScreen link effect: whoami names the link\'s account, so the link is not redeemed; the loaded page opens Today' },
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
            "Coda can't reach sign-in": "Coda couldn't reach sign-in",
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'Check your email': {
          on: {
            'request a sign-in link': 'Check your email',
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            "Coda can't reach sign-in": "Coda couldn't reach sign-in",
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'Signing you in': {
          on: {
            'verification succeeds': 'Your workspace',
            'the link was used or has expired': 'This link no longer works',
            'the link no longer grants access': 'No access to this practice',
            "Coda can't reach sign-in": "Coda couldn't reach sign-in",
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'This link no longer works': {
          on: {
            'request a sign-in link': 'Check your email',
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            "Coda can't reach sign-in": "Coda couldn't reach sign-in",
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'No access to this practice': {
          on: {
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        "Coda couldn't reach sign-in": {
          on: {
            'request a sign-in link': 'Check your email',
            'use another email': 'Enter your work email',
            'open the email button': 'Signing you in',
            "Coda can't reach sign-in": "Coda couldn't reach sign-in",
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'Your workspace': {
          on: {
            "open another physician's sign-in link": { target: 'Signing you in', guard: 'signed in' },
            'open your own sign-in link': { target: 'Your workspace', guard: 'signed in' },
            'the session expires': { target: 'Sign-in prompt over your work', guard: 'signed in' },
            'the practice removes your access': { target: 'No access to this practice', guard: 'signed in' },
            'sign out': 'Enter your work email',
            'another physician signs in in another tab': 'Your workspace',
            'close and reopen Coda': [{ target: 'Your workspace', guard: 'signed in' }, { target: 'Enter your work email', guard: 'not signed in' }],
          },
        },
        'Sign-in prompt over your work': {
          on: {
            'request a sign-in link': 'Sign-in prompt over your work',
            'you sign in in another tab': 'Your workspace',
            'another physician signs in in another tab': 'Your workspace',
            'sign out': 'Enter your work email',
            'open the email button': 'Signing you in',
            'open a patient link while signed out': { target: 'Enter your work email', guard: 'not signed in' },
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
            'you sign in in another tab': { target: 'Signed in', guard: 'on a sign-in screen' },
            'another physician signs in in another tab': { target: 'Signed in', guard: 'on a sign-in screen' },
          },
        },
        'Signed in': {
          on: {
            'the session expires': { target: 'Session expired', guard: 'in the workspace' },
            'sign out': { target: 'Signed out', guard: 'in the workspace or the prompt' },
            'the practice removes your access': { target: 'Signed out', guard: 'in the workspace' },
            "open another physician's sign-in link": { target: 'Signed out', guard: 'in the workspace' },
          },
        },
        'Session expired': {
          on: {
            'verification succeeds': { target: 'Signed in', guard: 'signing in' },
            'you sign in in another tab': { target: 'Signed in', guard: 'on a sign-in screen or the prompt' },
            'another physician signs in in another tab': { target: 'Signed in', guard: 'on a sign-in screen or the prompt' },
            'sign out': { target: 'Signed out', guard: 'in the workspace or the prompt' },
            'open a patient link while signed out': 'Signed out',
            'open the email button': 'Signed out',
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
            "open another physician's sign-in link": { target: 'Today', guard: 'signed in, in the workspace' },
            'open your own sign-in link': { target: 'Today', guard: 'signed in, in the workspace' },
            'the practice removes your access': { target: 'Today', guard: 'signed in, in the workspace' },
            'another physician signs in in another tab': { target: 'Today', guard: 'in the workspace or the prompt' },
            'open the email button': { target: 'Today', guard: 'on a sign-in screen or the prompt' },
            'open a patient link while signed out': { target: 'Today', guard: 'not signed in' },
            'sign out': 'Today',
            'close and reopen Coda': 'Today',
          },
        },
      },
    },
  },
});

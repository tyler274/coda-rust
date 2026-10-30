// How a physician signs in to Coda Agents: the design, drawn from README.md and
// intent/physician.md A1. The rules are the agreed sentences, held in every tab and every state
// the model reaches; the outcomes are what the spec walks through, each a list of states the model
// must pass through. The code carries the design out, and each step's `code` note says where.
//   node verification/statechart.mjs check verification/physician/sign-in/sign-in.mjs
//   node verification/statechart.mjs verus verification/physician/sign-in/sign-in.mjs --module sign_in --check
//
// The session belongs to the browser: the session it stores, and its owner beside it, decide whether
// you are signed in, and the browser counts its sign-outs. Two tabs run the same design, so whatever
// one tab can do, the other can do too, in any order. A tab follows the browser a moment late: news
// from another tab only tells it to catch up, and it then takes whatever the browser stores.
//
// Each wait in the code is a step of its own: Coda answers reads, link checks and sign-ins whenever
// it does, and news reaches a tab whenever it does. Signing out is one step for the browser: the tab
// forgets the stored session, counts the sign-out, tells the other tabs and asks Coda to end the
// session without waiting; Coda ending it on arrival is taken as at once, since every tab still
// holding it has news on its way. A screen that goes away drops what it was still waiting on, as
// Dioxus drops a component's tasks; a page load drops everything. When a session ends, Coda ends
// the event stream of a workspace that holds it, and the workspace reads again. The browser holds
// up to three sessions in a run; asking for a link while Coda answers is one step.
import { model, is, not, all, any } from '../../model.mjs';

const SESSIONS = ['Session 1', 'Session 2', 'Session 3'];
const NONE = 'No session';

// Shared by both tabs.
const stores = 'This browser stores';
const access = 'Your access';
const GRANTED = 'Granted';
const REMOVED = 'Removed by the practice';
const UNUSED = 'Not signed in yet';
const YOURS_LIVE = 'Yours, live';
const YOURS_EXPIRED = 'Yours, expired';
const YOURS_OUT = 'Yours, signed out';
const THEIRS_LIVE = "Another physician's, live";
const THEIRS_EXPIRED = "Another physician's, expired";
const THEIRS_OUT = "Another physician's, signed out";
const YOURS = [YOURS_LIVE, YOURS_EXPIRED, YOURS_OUT];
const THEIRS = [THEIRS_LIVE, THEIRS_EXPIRED, THEIRS_OUT];
const signingOut = { [YOURS_LIVE]: YOURS_OUT, [YOURS_EXPIRED]: YOURS_OUT, [THEIRS_LIVE]: THEIRS_OUT, [THEIRS_EXPIRED]: THEIRS_OUT };

// Each tab.
const CLOSED = 'Closed';
const OPENING = 'Opening your workspace';
const EMAIL = 'Enter your work email';
const CHECK = 'Check your email';
const SIGNING = 'Signing you in';
const USED = 'This link no longer works';
const NO_ACCESS = 'No access to this practice';
const UNREACHED = "Coda couldn't reach sign-in";
const WORK = 'Your workspace';
const PROMPT = 'Sign-in prompt over your work';
const SIGN_IN_PAGES = [EMAIL, CHECK, SIGNING, USED, NO_ACCESS, UNREACHED];
const SHELL = [OPENING, WORK, PROMPT];
const NOBODYS = "Nobody's";
const YOURS_TAB = 'Yours';
const THEIRS_TAB = "Another physician's";
const NO_READ = 'No read out';
const READ_NONE = 'Reading with no session';
const reading = (s) => `Reading with ${s}`;
const NO_SIGN_IN = 'No sign-in out';
const checking = (s) => `Checking the link with ${s}`;
const VERIFYING = { yours: 'Verifying your link', theirs: "Verifying another physician's link" };
const ASKING = { yours: 'Asking Coda who signed in: you', theirs: 'Asking Coda who signed in: another physician' };
const NO_LINK = 'No patient link';
const HOLDING = 'Holding a patient link';
const TODAY = 'Today';
const PATIENT = 'The linked patient';
const NO_NEWS = 'No news';
const NEWS = 'The browser changed its session';
const SAW = 'Every sign-out';
const MISSED = 'Not the latest sign-out';
const NOTHING_OLD = 'Nothing from before a sign-out';
const OLD_WORK = 'Your work from before a sign-out';
// What a tab owes you: the patient whose link you opened there while signed out; a tab whose session
// Coda refuses, holding a link with no sign-out since, owes it too. Landing doesn't let it off, so
// the rule can see where it landed; only signing out, losing access, another physician taking the
// tab or loading another page there does.
const OWES_NOTHING = 'Nothing';
const OWES_PATIENT = 'The patient you opened while signed out';

const owners = { yours: { slot: YOURS, live: YOURS_LIVE, tab: YOURS_TAB }, theirs: { slot: THEIRS, live: THEIRS_LIVE, tab: THEIRS_TAB } };

const both = all;

const TABS = ['A', 'B'];
const tabOf = (t) => ({
  shows: `Tab ${t} shows`, presents: `Tab ${t} presents`, whose: `Tab ${t} is`, read: `Tab ${t} reads`,
  signIn: `Tab ${t} signs in`, link: `Tab ${t} holds`, opens: `Tab ${t} opens at`,
  news: `News for tab ${t}`, seen: `Tab ${t} has seen`, before: `Tab ${t} still shows`, owes: `Tab ${t} owes you`,
});
const other = (t) => (t === 'A' ? 'B' : 'A');

// Every combination of one case from each list, in order, with later lists' effects winning.
const combine = (...lists) => lists.reduce((acc, list) => acc.flatMap((a) => list.map((b) => ({
  when: both(a.when, b.when), set: { ...a.set, ...b.set },
}))), [{ set: {} }]).map((c) => (c.when ? c : { set: c.set }));
const unless = (cases) => [...cases, { set: {} }];

// A tab forgets where it was: the patient link it held and the place it opens.
const forgetPlace = (c) => ({ [c.link]: NO_LINK, [c.opens]: TODAY, [c.before]: NOTHING_OLD });
// The workspace closes, and with it the read it was waiting on.
const closeShell = (c) => ({ [c.read]: NO_READ });
// The sign-in page closes, and with it what it was waiting on.
const closeSignIn = (c) => ({ [c.signIn]: NO_SIGN_IN });
const signedOutHere = (c) => ({ ...closeShell(c), [c.shows]: EMAIL, [c.presents]: NONE, [c.whose]: NOBODYS });
// A fresh page: nothing it waited on survives, and it has read the sign-out count.
const freshPage = (c) => ({
  ...closeShell(c), ...forgetPlace(c), [c.signIn]: NO_SIGN_IN, [c.news]: NO_NEWS, [c.seen]: SAW, [c.owes]: OWES_NOTHING,
});
const holdsAPlace = (c) => any([is(c.shows, ...SHELL), is(c.link, HOLDING)]);

// Another tab hears that the browser changed.
const tell = (t) => ({ [tabOf(other(t)).news]: NEWS });
// A sign-out leaves what any tab shows or holds behind it, until that tab forgets its place.
const signOutMarks = TABS.map((t) => { const c = tabOf(t); return unless([{ when: holdsAPlace(c), set: { [c.before]: OLD_WORK } }]); });
// When a session stops being live, Coda ends the event stream of a workspace that holds it, and
// the workspace reads again.
const streamEnds = (s, onlyIf) => TABS.map((t) => {
  const c = tabOf(t);
  return unless([{ when: both(is(c.shows, WORK), is(c.presents, s), is(c.read, NO_READ), onlyIf), set: { [c.read]: reading(s) } }]);
});

// The browser signs out, from tab t: it forgets the session it stores, or the tab's own when it
// stores none, counts the sign-out and tells the other tab; Coda ends that session. Every tab's
// place from before is now work from before a sign-out.
const signsOut = (t, when, set) => {
  const c = tabOf(t), o = tabOf(other(t));
  const counted = { ...set, [stores]: NONE, [c.seen]: SAW, [o.seen]: MISSED, [o.news]: NEWS, [c.owes]: OWES_NOTHING, [o.owes]: OWES_NOTHING };
  return [
    ...SESSIONS.flatMap((s) => combine(
      ...signOutMarks, ...streamEnds(s, is(s, YOURS_LIVE, THEIRS_LIVE)),
      [{ when: both(when, any([is(stores, s), both(is(stores, NONE), is(c.presents, s))])), set: { ...counted, [s]: signingOut } }],
    )),
    ...combine(...signOutMarks, [{ when, set: counted }]),
  ];
};

// Catching up: the tab takes what the browser stores. A sign-out it missed makes it forget where it
// was, and only that. With nothing stored, a signed-in tab signs out. A signed-out tab, which a tab still opening
// with no session is too, takes the stored session and keeps the patient link it holds. Your own tab
// takes it where it was and reads with it; any other tab, or one that missed a sign-out, starts a
// new workspace at Today.
const catchUp = (t, when, set = {}) => {
  const c = tabOf(t);
  return [SAW, MISSED].flatMap((seenState) => {
    const missed = seenState === MISSED;
    const atSeen = is(c.seen, seenState);
    const forget = missed ? forgetPlace(c) : {};
    const base = { ...set, ...forget, [c.seen]: SAW };
    return [
      { when: both(when, atSeen, is(stores, NONE), is(c.shows, ...SHELL)), set: { ...base, ...signedOutHere(c) } },
      { when: both(when, atSeen, is(stores, NONE)), set: { ...base, [c.presents]: NONE } },
      ...SESSIONS.flatMap((s) => [
        ...(missed ? [] : [{ when: both(when, atSeen, is(stores, s), is(c.shows, ...SHELL), is(c.presents, s)), set: base }]),
        ...Object.values(owners).flatMap((o) => [
          { when: both(when, atSeen, is(stores, s), is(s, ...o.slot), is(c.shows, ...SHELL), is(c.whose, NOBODYS)),
            set: { ...base, [c.presents]: s, [c.read]: reading(s), [c.whose]: o.tab } },
          { when: both(when, atSeen, is(stores, s), is(s, ...o.slot), not(is(c.shows, ...SHELL))),
            set: { ...base, ...closeSignIn(c), [c.shows]: OPENING, [c.presents]: s, [c.read]: reading(s), [c.whose]: o.tab } },
          ...(missed ? [] : [
            { when: both(when, atSeen, is(stores, s), is(s, ...o.slot), is(c.whose, o.tab), is(c.shows, PROMPT)),
              set: { ...base, [c.shows]: WORK, [c.presents]: s, [c.read]: reading(s) } },
            { when: both(when, atSeen, is(stores, s), is(s, ...o.slot), is(c.whose, o.tab)),
              set: { ...base, [c.presents]: s, [c.read]: reading(s) } },
          ]),
          // A tab that was someone's becomes the new physician's at Today, owing nothing.
          { when: both(when, atSeen, is(stores, s), is(s, ...o.slot)),
            set: { ...base, ...closeShell(c), ...forgetPlace(c), [c.shows]: OPENING, [c.presents]: s, [c.read]: reading(s), [c.whose]: o.tab, [c.owes]: OWES_NOTHING } },
        ]),
      ]),
    ];
  });
};

// The first session not yet signed in, which Coda gives the next sign-in.
const next = (s) => {
  const i = SESSIONS.indexOf(s);
  return both(is(s, UNUSED), ...(i ? [not(is(SESSIONS[i - 1], UNUSED))] : []));
};
const ownerTab = (s, o) => both(is(s, ...o.slot));

const events = {};
for (const t of TABS) {
  const c = tabOf(t);
  const tab = `in tab ${t}`;
  const loads = (holding) => [
    ...SESSIONS.flatMap((s) => Object.values(owners).map((o) => ({
      when: both(is(stores, s), ownerTab(s, o)),
      set: { ...freshPage(c), [c.shows]: OPENING, [c.presents]: s, [c.read]: reading(s), [c.whose]: o.tab, [c.link]: holding },
    }))),
    { set: { ...freshPage(c), [c.shows]: OPENING, [c.presents]: NONE, [c.read]: READ_NONE, [c.whose]: NOBODYS, [c.link]: holding,
      [c.owes]: holding === HOLDING ? OWES_PATIENT : OWES_NOTHING } },
  ];
  events[`open Coda ${tab}`] = {
    by: 'you', intent: 'A new tab, a reload, or Coda reopened after closing it opens from what the browser stores; its first read decides.',
    code: 'lib.rs PhysicianRoot: the client takes the stored session; PhysicianShell: the first /today read decides',
    cases: loads(NO_LINK),
  };
  events[`open a patient link ${tab}`] = {
    by: 'you', intent: 'The tab holds the link: it opens that patient once someone is signed in.',
    code: 'main.rs ?patient=; lib.rs PhysicianRoot held_link',
    cases: loads(HOLDING),
  };
  for (const [who, o] of Object.entries(owners)) {
    events[who === 'yours' ? `open your sign-in link ${tab}` : `open another physician's sign-in link ${tab}`] = {
      by: who === 'yours' ? 'you' : 'another physician',
      intent: 'The emailed link opens a page that checks it against the session the browser stores: the same physician\'s session stands if Coda still knows it, another physician\'s is signed out first, and otherwise Coda redeems the link.',
      code: 'main.rs sign_in_link loads the page; lib.rs SignInScreen link effect compares the stored owner with the link',
      cases: [
        ...SESSIONS.map((s) => ({ when: both(is(stores, s), ownerTab(s, o)),
          set: { ...freshPage(c), [c.shows]: SIGNING, [c.presents]: s, [c.whose]: NOBODYS, [c.signIn]: checking(s) } })),
        ...signsOut(t, not(is(stores, NONE)), { ...freshPage(c), [c.shows]: SIGNING, [c.presents]: NONE, [c.whose]: NOBODYS, [c.signIn]: VERIFYING[who] }),
        { set: { ...freshPage(c), [c.shows]: SIGNING, [c.presents]: NONE, [c.whose]: NOBODYS, [c.signIn]: VERIFYING[who] } },
      ],
    };
  }
  events[`request a sign-in link ${tab}`] = {
    by: 'you', intent: 'Coda always answers "Check your email", and an address with no account gets no email.',
    code: 'lib.rs SignInScreen request_token: Mailed shows Check your email; the prompt mails inside itself',
    cases: [{ when: is(c.shows, EMAIL, CHECK, USED, UNREACHED), set: { [c.shows]: CHECK } }, { when: is(c.shows, PROMPT), set: { [c.shows]: PROMPT } }],
  };
  events[`use another email ${tab}`] = {
    by: 'you', intent: 'Back to the email field.',
    code: 'lib.rs SignInScreen "Use a different email"',
    cases: [{ when: is(c.shows, CHECK, USED, NO_ACCESS, UNREACHED), set: { [c.shows]: EMAIL } }],
  };
  events[`Coda checks the link ${tab}`] = {
    by: "Coda's sign-in service", intent: 'A session the browser still stores and Coda still knows for the same physician stands, and the tab opens the workspace; otherwise Coda redeems the link.',
    code: 'lib.rs SignInScreen link effect: whoami with the held session; on_signed_in while the browser still stores it, else verify_link',
    cases: SESSIONS.flatMap((s) => Object.entries(owners).flatMap(([who, o]) => [
      { when: both(is(c.signIn, checking(s)), is(stores, s), is(s, o.live), who === 'yours' ? is(access, GRANTED) : undefined),
        set: { ...closeSignIn(c), [c.shows]: OPENING, [c.read]: reading(s), [c.whose]: o.tab } },
      { when: both(is(c.signIn, checking(s)), is(s, ...o.slot)), set: { [c.signIn]: VERIFYING[who] } },
    ])),
  };
  events[`Coda signs in the link ${tab}`] = {
    by: "Coda's sign-in service", intent: 'Coda starts a new session; the browser stores it with its owner and tells the other tabs. The email never carries a patient.',
    code: 'lib.rs verify_link Ok: store_session with the link email, read the sign-out count, announce',
    cases: Object.entries(owners).flatMap(([who, o]) => SESSIONS.map((s) => ({
      when: both(is(c.signIn, VERIFYING[who]), next(s), who === 'yours' ? is(access, GRANTED) : undefined),
      set: { [s]: o.live, [stores]: s, [c.presents]: s, [c.signIn]: ASKING[who], [c.seen]: SAW, ...tell(t) },
    }))),
  };
  events[`Coda answers who signed in ${tab}`] = {
    by: "Coda's sign-in service", intent: 'The tab opens the workspace at Today.',
    code: 'lib.rs verify_link: on_signed_in(whoami)',
    cases: Object.entries(owners).flatMap(([who, o]) => [
      ...SESSIONS.map((s) => ({
        when: both(is(c.signIn, ASKING[who]), is(c.presents, s)),
        set: { ...closeSignIn(c), [c.shows]: OPENING, [c.read]: reading(s), [c.whose]: o.tab },
      })),
      { when: is(c.signIn, ASKING[who]), set: { ...closeSignIn(c), [c.shows]: OPENING, [c.read]: READ_NONE, [c.whose]: o.tab } },
    ]),
  };
  events[`the link was used or has expired ${tab}`] = {
    by: "Coda's sign-in service", intent: 'The link is rejected.',
    code: 'lib.rs verify_link 401: SIGN_IN_INVALID',
    cases: [{ when: is(c.signIn, ...Object.values(VERIFYING)), set: { [c.signIn]: NO_SIGN_IN, [c.shows]: USED } }],
  };
  events[`the link no longer grants access ${tab}`] = {
    by: "Coda's sign-in service", intent: 'Shows "No access to this practice".',
    code: 'lib.rs verify_link 403: NoAccess',
    cases: [{ when: both(is(c.signIn, VERIFYING.yours), is(access, REMOVED)), set: { [c.signIn]: NO_SIGN_IN, [c.shows]: NO_ACCESS } }],
  };
  events[`Coda can't reach sign-in ${tab}`] = {
    by: "Coda's sign-in service", intent: "Coda couldn't send the link or confirm it, and says so with a way to try again. A link check Coda can't answer keeps the session the browser still stores, or else redeems the link.",
    code: 'lib.rs request_token Err and verify_link failure: Recovery; the link effect on another whoami failure: on_signed_in(None)',
    cases: [
      { when: is(c.signIn, ...Object.values(VERIFYING)), set: { [c.signIn]: NO_SIGN_IN, [c.shows]: UNREACHED } },
      ...SESSIONS.flatMap((s) => Object.entries(owners).flatMap(([who, o]) => [
        { when: both(is(c.signIn, checking(s)), is(stores, s), ownerTab(s, o)),
          set: { ...closeSignIn(c), [c.shows]: OPENING, [c.read]: reading(s), [c.whose]: o.tab } },
        { when: both(is(c.signIn, checking(s)), ownerTab(s, o)), set: { [c.signIn]: VERIFYING[who] } },
      ])),
      { when: both(is(c.signIn, NO_SIGN_IN), is(c.shows, EMAIL, CHECK, USED, UNREACHED)), set: { [c.shows]: UNREACHED } },
    ],
  };
  events[`sign out ${tab}`] = {
    by: 'you', intent: 'Signs out every tab and forgets where you were.',
    code: "lib.rs SessionState::end: forgets the session the browser stores, or the tab's own, counts the sign-out, announces, and asks Coda to end it",
    cases: signsOut(t, is(c.shows, ...SHELL), { ...forgetPlace(c), ...signedOutHere(c) }),
  };
  events[`the workspace reads ${tab}`] = {
    by: 'the physician app', intent: 'The workspace reads with the session the tab holds.',
    code: 'lib.rs PhysicianShell reads capture the presented bearer',
    cases: SESSIONS.map((s) => ({ when: both(is(c.shows, WORK), is(c.read, NO_READ), is(c.presents, s)), set: { [c.read]: reading(s) } })),
  };
  events[`Coda answers the read ${tab}`] = {
    by: "Coda's sign-in service", intent: 'A live session reads, and an opening workspace shows, on the patient link it held. A refusal of a session the tab no longer holds changes nothing. An ended session covers a shown workspace with the prompt, and sends an opening one to the sign-in page with its patient link. Removed access signs the tab out and says so.',
    code: 'lib.rs PhysicianShell /today answer; SessionState::refused(refusal, shown, presented)',
    cases: [
      ...SESSIONS.flatMap((s) => {
        const read = is(c.read, reading(s));
        const answers = any([both(is(s, YOURS_LIVE), is(access, GRANTED)), is(s, THEIRS_LIVE)]);
        return [
          { when: both(read, answers, is(c.shows, OPENING), is(c.link, HOLDING)), set: { [c.read]: NO_READ, [c.shows]: WORK, [c.link]: NO_LINK, [c.opens]: PATIENT } },
          { when: both(read, answers, is(c.shows, OPENING)), set: { [c.read]: NO_READ, [c.shows]: WORK } },
          { when: both(read, answers), set: { [c.read]: NO_READ } },
          { when: both(read, not(is(c.presents, s))), set: { [c.read]: NO_READ } },
          { when: both(read, is(s, YOURS_LIVE), is(access, REMOVED)),
            set: { ...forgetPlace(c), ...closeShell(c), [c.shows]: NO_ACCESS, [c.presents]: NONE, [c.whose]: NOBODYS, [stores]: { [s]: NONE }, [c.owes]: OWES_NOTHING } },
          { when: both(read, is(c.shows, WORK, PROMPT)), set: { [c.read]: NO_READ, [c.shows]: PROMPT, [stores]: { [s]: NONE } } },
          { when: both(read, is(c.link, HOLDING), is(c.seen, SAW)), set: { ...signedOutHere(c), [stores]: { [s]: NONE }, [c.owes]: OWES_PATIENT } },
          { when: read, set: { ...signedOutHere(c), [stores]: { [s]: NONE } } },
        ];
      }),
      { when: both(is(c.read, READ_NONE), not(is(c.presents, NONE))), set: { [c.read]: NO_READ } },
      { when: both(is(c.read, READ_NONE), is(c.link, HOLDING), is(c.seen, SAW)), set: { ...signedOutHere(c), [c.owes]: OWES_PATIENT } },
      { when: is(c.read, READ_NONE), set: signedOutHere(c) },
    ],
  };
  events[`this tab hears the news ${tab}`] = {
    by: 'another tab', intent: 'The tab takes what the browser stores: a signed-out tab opens the patient link it holds, your own tab picks up where it was, any other tab starts at Today, and a browser that stores nothing signs the tab out.',
    code: 'lib.rs SessionState::catch_up on tab news: reads the stored session, its owner and the sign-outs counted',
    cases: catchUp(t, both(is(c.news, NEWS), not(is(c.shows, CLOSED))), { [c.news]: NO_NEWS }),
  };
}
for (const s of SESSIONS)
  events[`${s} expires`] = {
    by: "Coda's sign-in service", intent: 'The session reaches its end.',
    code: 'store.rs session: expires_at; the event stream ends',
    cases: combine([{ when: is(s, YOURS_LIVE, THEIRS_LIVE), set: { [s]: { [YOURS_LIVE]: YOURS_EXPIRED, [THEIRS_LIVE]: THEIRS_EXPIRED } } }], ...streamEnds(s)),
  };
events['the practice removes your access'] = {
  by: 'the practice', intent: 'Coda refuses your sessions and your links from now on.',
  code: 'auth.rs: refusal reason access_removed; the event stream ends',
  cases: combine([{ when: is(access, GRANTED), set: { [access]: REMOVED } }],
    ...TABS.map((t) => { const c = tabOf(t); return unless(SESSIONS.map((s) => ({ when: both(is(c.shows, WORK), is(c.presents, s), is(s, YOURS_LIVE), is(c.read, NO_READ)), set: { [c.read]: reading(s) } }))); })),
};

const regions = {
  ...Object.fromEntries(SESSIONS.map((s) => [s, [UNUSED, YOURS_LIVE, YOURS_EXPIRED, YOURS_OUT, THEIRS_LIVE, THEIRS_EXPIRED, THEIRS_OUT]])),
  [stores]: [NONE, ...SESSIONS],
  [access]: [GRANTED, REMOVED],
  ...Object.fromEntries(TABS.flatMap((t) => {
    const c = tabOf(t);
    return [
      [c.shows, [CLOSED, OPENING, EMAIL, CHECK, SIGNING, USED, NO_ACCESS, UNREACHED, WORK, PROMPT]],
      [c.presents, [NONE, ...SESSIONS]],
      [c.whose, [NOBODYS, YOURS_TAB, THEIRS_TAB]],
      [c.read, [NO_READ, READ_NONE, ...SESSIONS.map(reading)]],
      [c.signIn, [NO_SIGN_IN, ...SESSIONS.map(checking), ...Object.values(VERIFYING), ...Object.values(ASKING)]],
      [c.link, [NO_LINK, HOLDING]],
      [c.opens, [TODAY, PATIENT]],
      [c.news, [NO_NEWS, NEWS]],
      [c.seen, [SAW, MISSED]],
      [c.before, [NOTHING_OLD, OLD_WORK]],
      [c.owes, [OWES_NOTHING, OWES_PATIENT]],
    ];
  })),
};

// A tab has caught up when it waits on nothing and has heard every news.
const settled = (t) => { const c = tabOf(t); return { [c.read]: NO_READ, [c.signIn]: NO_SIGN_IN, [c.news]: NO_NEWS }; };
const ONCE_OUT = 'Once you sign out, your workspace is gone from this tab.';
const EXPIRY = 'When your session expires mid-work, a sign-in prompt covers the screen.';
const SIGNED_IN = 'While you are signed in, Coda never asks you to sign in or tells you sign-in failed.';
const TODAY_FIRST = 'An emailed sign-in link always opens Today.';
const LANDS = 'A patient link opened while signed out lands on that patient once you sign in.';
const never = TABS.flatMap((t) => {
  const c = tabOf(t);
  const here = { ...settled(t) };
  return [
    { name: `WorkspaceAfterSignOut${t}`, states: { [c.shows]: WORK, [c.before]: OLD_WORK, ...here }, sentence: ONCE_OUT },
    ...SESSIONS.map((s, i) => ({ name: `WorkspaceOnASignedOutSession${t}${i + 1}`, states: { [c.shows]: WORK, [c.presents]: s, [s]: [YOURS_OUT, THEIRS_OUT], ...here }, sentence: ONCE_OUT })),
    ...SESSIONS.map((s, i) => ({ name: `WorkspaceAfterExpiry${t}${i + 1}`, states: { [c.shows]: WORK, [c.presents]: s, [s]: [YOURS_EXPIRED, THEIRS_EXPIRED], ...here }, sentence: EXPIRY })),
    ...SESSIONS.flatMap((s, i) => [
      { name: `AskedToSignInWhileSignedIn${t}${i + 1}`, states: { [c.shows]: [...SIGN_IN_PAGES, PROMPT], [stores]: s, [s]: YOURS_LIVE, [access]: GRANTED, ...here }, sentence: SIGNED_IN },
      { name: `AskedToSignInWhileAnotherIsSignedIn${t}${i + 1}`, states: { [c.shows]: [...SIGN_IN_PAGES, PROMPT], [stores]: s, [s]: THEIRS_LIVE, ...here }, sentence: SIGNED_IN },
    ]),
    { name: `NewSignInOpensToday${t}`, states: { [c.shows]: SIGNING, [c.opens]: PATIENT }, sentence: TODAY_FIRST },
    { name: `HeldLinkLands${t}`, states: { [c.shows]: WORK, [c.owes]: OWES_PATIENT, [c.opens]: TODAY, ...here }, sentence: LANDS },
  ];
});

// Each rule shown to fail: a mistake the code could plausibly make, made to a copy of the events.
const mentions = (guard, region, state) => !!guard && (guard.in ? guard.in[0] === region && guard.in[1] === state
  : guard.not ? mentions(guard.not, region, state) : (guard.and ?? guard.or).some((g) => mentions(g, region, state)));
const eachCase = (events, prefix, edit) => TABS.forEach((t) => events[`${prefix} in tab ${t}`].cases.forEach((c) => edit(c, tabOf(t))));
const controls = [
  { sentence: ONCE_OUT, breaks: 'a tab that missed a sign-out keeps what it showed when it catches up',
    change: (events) => eachCase(events, 'this tab hears the news', (k, c) => { delete k.set[c.before]; }) },
  { sentence: EXPIRY, breaks: "an ended session's answer leaves the workspace showing",
    change: (events) => eachCase(events, 'Coda answers the read', (k, c) => { if (k.set[c.shows] === PROMPT) k.set[c.shows] = WORK; }) },
  { sentence: SIGNED_IN, breaks: "a tab on a sign-in page doesn't follow another tab's sign-in",
    change: (events) => eachCase(events, 'this tab hears the news', (k, c) => {
      if (k.set[c.shows] === OPENING && k.set[c.signIn]) k.set = { [c.news]: NO_NEWS, [c.seen]: SAW };
    }) },
  { sentence: TODAY_FIRST, breaks: 'a sign-in link page keeps the place the tab was at',
    change: (events) => ['open your sign-in link', "open another physician's sign-in link"].forEach((prefix) =>
      eachCase(events, prefix, (k, c) => { delete k.set[c.opens]; })) },
  { sentence: LANDS, breaks: 'a tab holding a patient link opens Today when its read answers',
    change: (events) => eachCase(events, 'Coda answers the read', (k, c) => { if (k.set[c.opens] === PATIENT) k.set[c.opens] = TODAY; }) },
  { sentence: LANDS, breaks: 'a tab still opening without a session forgets its patient link when another tab signs in',
    change: (events) => TABS.forEach((t) => {
      const c = tabOf(t);
      for (const k of events[`this tab hears the news in tab ${t}`].cases)
        if (mentions(k.when, c.whose, NOBODYS)) Object.assign(k.set, forgetPlace(c));
    }) },
];

// Why the rules hold: facts true in every state, each kept by every step. Verus proves them; the
// search confirms them in every state it reaches.
const implies = (a, b) => any([not(a), b]);
const readable = (s) => any([both(is(s, YOURS_LIVE), is(access, GRANTED)), is(s, THEIRS_LIVE)]);
const ENDED = [YOURS_EXPIRED, YOURS_OUT, THEIRS_EXPIRED, THEIRS_OUT];
const invariant = [
  ...SESSIONS.slice(1).map((s, i) => ({ name: `Sessions start in order ${i + 2}`, holds: implies(is(SESSIONS[i], UNUSED), is(s, UNUSED)) })),
  ...SESSIONS.map((s, i) => ({ name: `Nothing holds a session not yet signed in ${i + 1}`, holds: implies(is(s, UNUSED), all(
    not(is(stores, s)),
    ...TABS.flatMap((t) => { const c = tabOf(t); return [not(is(c.presents, s)), not(is(c.read, reading(s))), not(is(c.signIn, checking(s)))]; }),
  )) })),
  ...TABS.flatMap((t) => {
    const c = tabOf(t);
    return [
      ...SESSIONS.map((s, i) => ({ name: `Tab ${t} reads with the session it holds ${i + 1}`, holds: implies(is(c.read, reading(s)), all(is(c.presents, s), is(c.shows, ...SHELL))) })),
      { name: `Tab ${t} reads with no session only while opening with none`, holds: implies(is(c.read, READ_NONE), all(is(c.presents, NONE), is(c.shows, OPENING))) },
      { name: `Tab ${t} signs in only on a fresh link page`, holds: implies(not(is(c.signIn, NO_SIGN_IN)), all(is(c.shows, SIGNING), is(c.before, NOTHING_OLD), is(c.link, NO_LINK))) },
      ...SESSIONS.flatMap((s, i) => [
        { name: `Tab ${t} checks a link with the session it holds ${i + 1}`, holds: implies(is(c.signIn, checking(s)), is(c.presents, s, NONE)) },
        { name: `Tab ${t} holds the session it checks while the browser stores it ${i + 1}`, holds: implies(all(is(c.signIn, checking(s)), is(stores, s)), is(c.presents, s)) },
      ]),
      { name: `Tab ${t} opens Today from a link page`, holds: implies(is(c.shows, SIGNING), is(c.opens, TODAY)) },
      { name: `Tab ${t} holds a patient link only until it lands`, holds: implies(is(c.link, HOLDING), not(is(c.shows, WORK, PROMPT))) },
      { name: `Tab ${t} owes you a patient only while it holds the link or opens at the patient, and has seen every sign-out`,
        holds: implies(is(c.owes, OWES_PATIENT), all(any([is(c.link, HOLDING), is(c.opens, PATIENT)]), is(c.seen, SAW))) },
      { name: `Tab ${t} is nobody's workspace only while it opens with no session`,
        holds: implies(all(is(c.shows, ...SHELL), is(c.whose, NOBODYS)), all(is(c.shows, OPENING), is(c.presents, NONE))) },
      { name: `Tab ${t} has news coming for work from before a sign-out`, holds: implies(is(c.before, OLD_WORK), all(is(c.seen, MISSED), is(c.news, NEWS))) },
      ...SESSIONS.map((s, i) => ({ name: `Tab ${t} reads again on a session that ended ${i + 1}`, holds: implies(all(is(c.shows, WORK), is(c.presents, s), is(s, ...ENDED)), is(c.read, reading(s))) })),
      ...SESSIONS.map((s, i) => ({ name: `Tab ${t} covers a session Coda refuses ${i + 1}`, holds: implies(all(is(c.shows, PROMPT), is(c.presents, s)), not(readable(s))) })),
      { name: `Tab ${t} follows the browser once it has heard every news`, holds: implies(
        all(any([is(c.shows, ...SHELL), is(c.signIn, ...SESSIONS.map(checking), ...Object.values(ASKING))]), is(c.news, NO_NEWS)),
        any([is(stores, NONE), ...SESSIONS.map((s) => both(is(stores, s), is(c.presents, s)))])) },
      ...SESSIONS.map((s, i) => ({ name: `Tab ${t} leaves the sign-in page for a stored session it can read ${i + 1}`, holds: implies(
        all(is(c.shows, ...SIGN_IN_PAGES), is(c.news, NO_NEWS), is(stores, s), readable(s)),
        is(c.signIn, checking(s), ...Object.values(ASKING))) })),
    ];
  }),
];

// What the spec walks through, as states tab A passes through in order.
const A = tabOf('A');
const B = tabOf('B');
const atRest = settled('A');
const reach = [
  { name: 'You sign in from the email and see Today', through: [{ [A.shows]: WORK, [A.whose]: YOURS_TAB, [A.opens]: TODAY, ...atRest }] },
  { name: 'Your session expires mid-work and you sign back in where you were', through: [
    { [A.shows]: WORK, [A.opens]: PATIENT, ...atRest }, { [A.shows]: PROMPT, [A.opens]: PATIENT, ...atRest },
    { [A.shows]: WORK, [A.opens]: PATIENT, ...atRest }] },
  { name: 'A patient link opened while signed out lands on that patient', through: [
    { [A.shows]: EMAIL, [A.link]: HOLDING }, { [A.shows]: WORK, [A.opens]: PATIENT, ...atRest }] },
  { name: 'A used or expired link is rejected and you ask for a new one', through: [{ [A.shows]: USED }, { [A.shows]: CHECK }] },
  { name: "A sign-in from the new link's tab moves a rejected tab on", through: [{ [A.shows]: USED, ...atRest }, { [A.shows]: WORK, ...atRest }] },
  { name: 'A link that no longer grants access says so', through: [{ [A.shows]: NO_ACCESS, [access]: REMOVED, ...atRest }] },
  { name: "Another physician's link signs you out and opens Today for them", through: [
    { [A.shows]: WORK, [A.whose]: YOURS_TAB, ...atRest }, { [A.shows]: WORK, [A.whose]: THEIRS_TAB, [A.opens]: TODAY, [stores]: SESSIONS[1], [SESSIONS[0]]: YOURS_OUT, ...atRest }] },
  { name: "Another physician's link opens Today, even after a link held earlier", through: [
    { [A.link]: HOLDING }, { [A.shows]: WORK, [A.opens]: PATIENT, ...atRest }, { [A.shows]: WORK, [A.whose]: THEIRS_TAB, [A.opens]: TODAY, ...atRest }] },
  { name: 'Your own link keeps your session', through: [
    { [A.shows]: WORK, [stores]: SESSIONS[0], ...atRest }, { [B.shows]: WORK, [B.whose]: YOURS_TAB, [stores]: SESSIONS[0], [SESSIONS[1]]: UNUSED, ...settled('B') }] },
  { name: 'Closing Coda while signed out asks you to sign in again', through: [{ [A.shows]: PROMPT, ...atRest }, { [A.shows]: EMAIL, [stores]: NONE, ...atRest }] },
  { name: 'You sign out', through: [{ [A.shows]: WORK, ...atRest }, { [A.shows]: EMAIL, [stores]: NONE, [SESSIONS[0]]: YOURS_OUT, ...atRest }] },
  { name: 'A new tab opens your workspace while your session lasts', through: [{ [A.shows]: WORK, ...atRest, [B.shows]: CLOSED }, { [B.shows]: WORK, [stores]: SESSIONS[0], [SESSIONS[1]]: UNUSED, ...settled('B') }] },
  { name: "Coda can't send the link, and you try again", through: [{ [A.shows]: UNREACHED }, { [A.shows]: CHECK }] },
  { name: 'A rejected link offers another email', through: [{ [A.shows]: USED }, { [A.shows]: EMAIL }] },
  { name: 'The practice removes your access mid-work', through: [{ [A.shows]: WORK, ...atRest }, { [A.shows]: NO_ACCESS, ...atRest }] },
  { name: 'Another physician signs in elsewhere, and this tab opens Today for them', through: [
    { [A.shows]: WORK, [A.whose]: YOURS_TAB, [A.opens]: PATIENT, ...atRest }, { [A.shows]: WORK, [A.whose]: THEIRS_TAB, [A.opens]: TODAY, ...atRest }] },
  { name: 'A signed-out tab opens its patient link for whoever signs in', through: [
    { [A.shows]: EMAIL, [A.link]: HOLDING }, { [A.shows]: WORK, [A.whose]: THEIRS_TAB, [A.opens]: PATIENT, ...atRest }] },
  { name: 'Sign-out forgets the patient you were on', through: [
    { [A.shows]: WORK, [A.opens]: PATIENT, ...atRest }, { [A.shows]: EMAIL, ...atRest }, { [A.shows]: WORK, [A.opens]: TODAY, ...atRest }] },
];

export const machine = model({
  id: 'Physician sign-in',
  regions,
  events,
  never,
  invariant,
  reach,
  controls,
});

// Native CDP E2E verification for the browser-SQLite Chords product demo under /coda-rust/.
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {mkdtempSync, rmSync, mkdirSync, writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';

const [origin, rawBasePath = ''] = process.argv.slice(2);
const basePath = rawBasePath === '/' ? '' : rawBasePath.replace(/\/+$/, '');
const withBase = p => (p === '/' ? (basePath ? `${basePath}/` : '/') : `${basePath}${p}`);
const screenshots = process.env.DEMO_SCREENSHOTS || join(tmpdir(), 'coda-demo-screenshots');
mkdirSync(screenshots, {recursive: true});

const profile = mkdtempSync(join(tmpdir(), 'coda-demo-browser-'));
const env = {...process.env};
delete env.LD_LIBRARY_PATH;
const child = spawn(process.env.CHROMIUM || 'chromium', [
  '--headless=new', '--no-sandbox', '--disable-gpu', '--remote-debugging-port=0', `--user-data-dir=${profile}`, 'about:blank',
], {env, stdio: ['ignore', 'ignore', 'pipe']});

let socket;
const deadline = setTimeout(() => { console.error('Demo Chromium proof exceeded 120s'); child.kill(); process.exit(1); }, 120000);

try {
  const ws = await new Promise((resolve, reject) => {
    let stderr = '';
    child.on('error', reject);
    child.on('exit', c => reject(new Error(`Chromium exited ${c}: ${stderr}`)));
    child.stderr.on('data', chunk => { stderr += chunk; const m = stderr.match(/DevTools listening on (ws:\/\/\S+)/); if (m) resolve(m[1]); });
  });
  const targets = await (await fetch(`http://${new URL(ws).host}/json/list`)).json();
  socket = new WebSocket(targets.find(t => t.type === 'page').webSocketDebuggerUrl);
  await new Promise((resolve, reject) => { socket.onopen = resolve; socket.onerror = reject; });

  let next = 0;
  const pending = new Map(), errors = [], failures = [];
  let probedDemoSession = false;
  let allowMockBudget429 = false;
  socket.onmessage = ({data}) => {
    const msg = JSON.parse(data);
    if (msg.id && pending.has(msg.id)) {
      const item = pending.get(msg.id); pending.delete(msg.id);
      msg.error ? item.reject(new Error(JSON.stringify(msg.error))) : item.resolve(msg.result);
    }
    if (msg.method === 'Runtime.exceptionThrown') errors.push(msg.params.exceptionDetails);
    if (msg.method === 'Network.responseReceived' && msg.params.response.url.startsWith(origin) && msg.params.response.status >= 400) {
      const {url, status} = msg.params.response;
      if (status === 404 && url.endsWith('/auth/demo-session')) {
        probedDemoSession = true;
      } else if (allowMockBudget429 && status === 429 && url.endsWith('/auth/demo-session')) {
        // Intentional simulated live turn budget exhaustion check below.
      } else {
        failures.push({url, status});
      }
    }
  };
  const call = (method, params = {}) => new Promise((resolve, reject) => { const id = ++next; pending.set(id, {resolve, reject}); socket.send(JSON.stringify({id, method, params})); });
  const evaluate = async expression => { const r = await call('Runtime.evaluate', {expression, returnByValue: true, awaitPromise: true}); assert(!r.exceptionDetails, JSON.stringify(r)); return r.result.value; };
  const until = async (expr, desc = expr) => {
    for (let i = 0; i < 200; i++) { if (await evaluate(expr)) return; await delay(100); }
    const diag = await evaluate(`JSON.stringify({
      hasRecs: document.body?.innerText.includes('Undecided medication recommendations'),
      hasSql: document.body?.innerText.includes('From SQLite chart search across'),
      hasStop: !!document.querySelector('[data-follow-up-stop]'),
      hasResume: !!document.querySelector('[data-follow-up-resume]'),
      tail: document.body?.innerText.slice(-900) || ""
    })`);
    throw new Error(`Timed out waiting for: ${desc}\nDiag:\n${diag}`);
  };
  const snap = async name => writeFileSync(join(screenshots, `${name}.png`), Buffer.from((await call('Page.captureScreenshot', {format: 'png'})).data, 'base64'));
  const clickBtn = text => evaluate(`[...document.querySelectorAll('button')].find(b => b.innerText.includes(${JSON.stringify(text)})).click()`);
  const setInput = (sel, proto, val) => evaluate(`(() => { const el = document.querySelector(${JSON.stringify(sel)}); Object.getOwnPropertyDescriptor(${proto}.prototype, 'value').set.call(el, ${JSON.stringify(val)}); el.dispatchEvent(new Event('input', {bubbles: true})); })()`);
  const assertNoLoadErrors = async step => {
    const body = await evaluate('document.body?.innerText || ""');
    for (const bad of [
      'HTTP client requested on demo transport',
      'Coda couldn’t load',
      'Coda couldn’t refresh',
      'Coda couldn’t open',
      'Coda couldn’t confirm',
      'Coda couldn’t save',
      'Coda couldn’t start',
      'Coda couldn’t mark',
      'couldn’t load',
      "couldn't load",
    ]) {
      assert(!body.includes(bad), `${step}: unexpected load/transport error ${JSON.stringify(bad)} in:\n${body.slice(0, 800)}`);
    }
  };

  await call('Page.enable'); await call('Runtime.enable'); await call('Network.enable');

  // Mobile 390x844 dark-venue + one-handed demo entry checks on exported --physician-demo site.
  await call('Emulation.setDeviceMetricsOverride', {width: 390, height: 844, deviceScaleFactor: 1, mobile: true});
  await call('Emulation.setEmulatedMedia', {features: [{name: 'prefers-color-scheme', value: 'dark'}]});
  const navMobile = await call('Page.navigate', {url: origin + withBase('/')});
  assert(!navMobile.errorText, navMobile.errorText);
  await until(`document.readyState === 'complete' && document.body.innerText.includes('Try live demo') && getComputedStyle(document.querySelector('h1')).opacity === '1'`, 'mobile demo homepage CTA');
  const mobileMetrics = await evaluate(`(() => {
    const navCta = document.querySelector('.nav a.btn-sm');
    const navLinks = document.querySelector('.nav-links');
    const heroCta = document.querySelector('.hero-cta-row a.btn');
    const navRect = navCta.getBoundingClientRect();
    const linksRect = navLinks.getBoundingClientRect();
    const heroRect = heroCta.getBoundingClientRect();
    const links = [...document.querySelectorAll('.nav-links a')].map(a => ({text: a.textContent.trim(), href: a.getAttribute('href'), h: a.getBoundingClientRect().height}));
    return {
      overflow: document.documentElement.scrollWidth > window.innerWidth,
      bg: getComputedStyle(document.body).backgroundColor,
      ink: getComputedStyle(document.body).color,
      navHref: navCta.getAttribute('href'),
      navTop: navRect.top,
      navBottom: navRect.bottom,
      navHeight: navRect.height,
      linksTop: linksRect.top,
      heroHref: heroCta.getAttribute('href'),
      heroText: heroCta.textContent.trim(),
      heroTop: heroRect.top,
      heroBottom: heroRect.bottom,
      heroHeight: heroRect.height,
      links,
    };
  })()`);
  assert.equal(mobileMetrics.overflow, false, 'mobile demo homepage must not overflow horizontally');
  assert.equal(mobileMetrics.bg, 'rgb(11, 17, 14)', 'dark mode background must be rgb(11, 17, 14)');
  assert.equal(mobileMetrics.ink, 'rgb(242, 246, 243)', 'dark mode ink must be rgb(242, 246, 243)');
  assert.equal(mobileMetrics.navHref, withBase('/physician/'));
  assert(mobileMetrics.navTop >= 0 && mobileMetrics.navBottom <= 844 && mobileMetrics.navHeight >= 44 && mobileMetrics.navTop < mobileMetrics.linksTop, `mobile nav demo CTA must sit on row 1 above nav-links with >=44px target: ${JSON.stringify(mobileMetrics)}`);
  assert.equal(mobileMetrics.heroHref, withBase('/physician/'));
  assert(mobileMetrics.heroText.includes('Try live demo'), mobileMetrics.heroText);
  assert(mobileMetrics.heroTop >= 0 && mobileMetrics.heroBottom <= 844 && mobileMetrics.heroHeight >= 44, `mobile hero demo CTA must fit above fold with >=44px target: ${JSON.stringify(mobileMetrics)}`);
  assert(mobileMetrics.links.some(l => l.text === 'Book' && l.h >= 44), `mobile nav-links must show Book >=44px: ${JSON.stringify(mobileMetrics.links)}`);
  await snap('00-mobile-dark-demo-home');

  await call('Emulation.setDeviceMetricsOverride', {width: 1440, height: 960, deviceScaleFactor: 1, mobile: false});
  await call('Emulation.setEmulatedMedia', {features: [{name: 'prefers-color-scheme', value: 'light'}]});

  const navHome = await call('Page.navigate', {url: origin + withBase('/')});
  assert(!navHome.errorText, navHome.errorText);
  await until(`document.readyState === 'complete' && document.body.innerText.includes('Try live demo') && getComputedStyle(document.querySelector('h1')).opacity === '1'`, 'homepage CTA');
  assert.equal(await evaluate(`document.querySelector('.hero-cta-row a.btn')?.getAttribute('href') || ''`), withBase('/physician/'));
  await snap('01-public-home-demo-cta');

  await evaluate(`document.querySelector('.hero-cta-row a.btn').click()`);
  await until(`location.pathname.includes('/physician') && document.body.innerText.includes('Gary Kowalczyk') && document.querySelector('.demo-quick-bar')?.innerText.includes('Local chart preview') && document.body.innerText.includes('Reset demo') && !document.body.innerText.includes('No visits remaining today')`, 'demo Today + Schedule + Local chart preview pill');
  assert.equal(probedDemoSession, true, 'Demo startup must probe POST /auth/demo-session before falling back');
  await assertNoLoadErrors('initial Today');
  await snap('02-demo-today');

  await clickBtn('Find patient');
  await until(`!!document.querySelector('dialog input, input[type="search"], input')`, 'search dialog');
  await setInput('dialog input, input[type="search"], input', 'HTMLInputElement', 'Marchetti');
  await until(`document.body.innerText.includes('Kristen Marchetti')`, 'SQLite search match');
  await evaluate(`[...document.querySelectorAll('dialog button')].find(b => b.innerText.includes('Kristen Marchetti')).click()`);
  await until(`document.body.innerText.includes('Kristen Marchetti') && document.body.innerText.includes('Patient summary')`, 'Kristen Marchetti workspace');
  await assertNoLoadErrors('search Kristen Marchetti');
  await snap('03-demo-search-workspace');

  await clickBtn('Today');
  await until(`document.body.innerText.includes('Gary Kowalczyk') && document.body.innerText.includes('6 need you')`, 'back on Today');

  // Clear one Today item via Done (6 -> 5) keeping Gary Kowalczyk open to test item.open next.
  await evaluate(`(() => { const btns = [...document.querySelectorAll('[data-today-done]')]; btns[btns.length - 1].click(); })()`);
  await until(`document.body.innerText.includes('5 need you')`, 'Today Done decrements count 6 -> 5');
  await assertNoLoadErrors('Today Done');

  // Click Gary Kowalczyk's .today-row-open directly to test live open_today_update_item + evidence + workspace load.
  await evaluate(`document.querySelector('[data-today-item="today-unit-2013"] .today-row-open').click()`);
  await until(`document.body.innerText.includes('High-intensity statin review') && !!document.querySelector('[data-work-evidence]')`, 'open_today_update_item opens evidence and Gary Kowalczyk workspace');
  await assertNoLoadErrors('open_today_update_item');

  await evaluate(`document.querySelector('[data-decide-recommendation]').click()`);
  await until(`document.body.innerText.includes('Active plan') && !document.body.innerText.includes('Undecided medication recommendations')`, 'recommendation approved');

  // Exercise Follow-ups mutation (Stop -> Held) on Gary Kowalczyk's patient page.
  await until(`!!document.querySelector('[data-follow-up-stop]')`, 'Follow-up Stop button');
  await evaluate(`document.querySelector('[data-follow-up-stop]').click()`);
  await until(`document.body.innerText.includes('Held') && !!document.querySelector('[data-follow-up-resume]')`, 'Follow-up transitioned to Held');
  await assertNoLoadErrors('Follow-up stop');

  // Exercise Past work dialog on Gary Kowalczyk's patient page.
  await evaluate(`document.querySelector('button.past-work-button').click()`);
  await until(`!!document.querySelector('dialog.past-work-dialog') && document.querySelector('dialog.past-work-dialog').innerText.includes('Pre-visit chart brief')`, 'Past work dialog loaded');
  await assertNoLoadErrors('Past work');
  await evaluate(`document.querySelector('dialog.past-work-dialog button[aria-label="Close"]').click()`);
  await until(`!document.querySelector('dialog.past-work-dialog')`, 'Past work dialog closed');

  await setInput('textarea', 'HTMLTextAreaElement', 'What is his latest LDL and atorvastatin dose?');
  await evaluate(`[...document.querySelectorAll('button')].find(b => b.innerText.trim() === 'Send').click()`);
  await until(`document.body.innerText.includes('From SQLite chart search across') && document.querySelectorAll('.answer-words button.citation-chip').length > 0 && document.body.innerText.includes('Basis and limitations')`, 'SQLite chat answer + inline citation chip + Basis and limitations');
  await evaluate(`document.querySelector('.answer-words button.citation-chip').click()`);
  await until(`!!document.querySelector('aside.source-reading-pane') && (document.querySelector('aside.source-reading-pane .source-pane-content')?.innerText.trim().length || 0) > 20`, 'citation reader open via cited_source with non-empty source prose');
  const paneText = await evaluate(`document.querySelector('aside.source-reading-pane')?.innerText || ''`);
  assert(!paneText.includes("Couldn't load this") && !paneText.includes("Couldn’t load this"), `citation pane showed load error: ${paneText}`);
  await assertNoLoadErrors('chat cited_source');
  await snap('04-demo-chat-sql-citation');
  await evaluate(`document.querySelector('aside.source-reading-pane button.source-pane-close')?.click()`);
  await until(`!document.querySelector('aside.source-reading-pane')`, 'citation reader closed');

  // Open a Schedule visit briefing from the right rail.
  await evaluate(`document.querySelector('.schedule-visit').click()`);
  await until(`!!document.querySelector('nav.briefing-nav') && document.body.innerText.includes('Prepared')`, 'Schedule visit briefing loaded');
  await assertNoLoadErrors('Schedule visit briefing');

  // Open Updates and open one update item.
  await clickBtn('Updates');
  await until(`document.querySelectorAll('.tu-updates-group').length > 0 && document.querySelectorAll('.today-row-open').length > 0`, 'Updates groups loaded');
  await evaluate(`document.querySelector('.today-row-open').click()`);
  await until(`document.body.innerText.includes('Patient summary')`, 'Updates row opened patient work');
  await assertNoLoadErrors('Updates item open');

  // Return to Today and open the Spruce drafted reply row.
  await clickBtn('Today');
  await until(`!!document.querySelector('[data-today-review-reply]')`, 'Today Review reply row present');
  await evaluate(`document.querySelector('[data-today-review-reply]').click()`);
  await until(`!!document.querySelector('[data-spruce-reply-card]')`, 'Spruce reply card opened');
  await assertNoLoadErrors('Spruce reply card');

  await call('Page.reload', {});
  await until(`document.body.innerText.includes('Reset demo') && document.body.innerText.includes('Today')`, 'reloaded app shell');
  await clickBtn('Today');
  await until(`document.body.innerText.includes('Gary Kowalczyk') && document.body.innerText.includes('5 need you')`, 'reloaded Today preserved 5 items');
  await evaluate(`document.querySelector('[data-today-item="today-unit-2013"] .today-row-open').click()`);
  await until(`document.body.innerText.includes('From SQLite chart search across') && !document.body.innerText.includes('Undecided medication recommendations') && !!document.querySelector('[data-follow-up-resume]')`, 'IndexedDB persistence verified across Today Done, chat, plan, and Follow-ups');
  await assertNoLoadErrors('Reload persistence');

  await clickBtn('Reset demo');
  await until(`document.body.innerText.includes('Undecided medication recommendations') && !document.body.innerText.includes('What is his latest LDL and atorvastatin dose?') && !!document.querySelector('[data-follow-up-stop]')`, 'Reset restored workspace and follow-ups');
  await clickBtn('Today');
  await until(`document.body.innerText.includes('6 need you')`, 'Reset restored 6 Today items');
  await assertNoLoadErrors('After reset');
  await snap('05-demo-after-reset');

  // Verify /coda/physician/ resolves API calls to /coda/physician/api/*, transitions
  // to "Live harness" + Bearer Valkey SSE stream when POST /auth/demo-session answers 200,
  // hides Sign out / Invite colleague, and honestly falls back when turn limit is hit.
  let liveSessionCalls = 0;
  let sseAuthHeader = '';
  let budgetExhaustNextSession = false;
  const jsonHeaders = [
    {name: 'content-type', value: 'application/json'},
    {name: 'cache-control', value: 'no-store'},
  ];
  const fulfillJson = (requestId, status, obj) => call('Fetch.fulfillRequest', {
    requestId,
    responseCode: status,
    responseHeaders: jsonHeaders,
    body: Buffer.from(JSON.stringify(obj)).toString('base64'),
  });
  const handleFetchPaused = async ({requestId, request}) => {
    const u = new URL(request.url);
    const apiPrefix = '/coda/physician/api';
    if (!u.pathname.startsWith(apiPrefix)) {
      const targetPath = u.pathname.replace('/coda/physician', `${basePath}/physician`);
      await call('Fetch.continueRequest', {requestId, url: `${origin}${targetPath}${u.search}`});
      return;
    }
    const route = u.pathname.slice(apiPrefix.length);
    if (route === '/auth/demo-session' && request.method === 'POST') {
      liveSessionCalls += 1;
      if (budgetExhaustNextSession) {
        await fulfillJson(requestId, 429, {
          code: 'demo_turn_budget_exhausted',
          detail: 'Live demo turn budget reached for this visitor.',
        });
      } else {
        await fulfillJson(requestId, 200, {
          session: 'mock-live-demo-1',
          physician_id: 'phys-demo-hannah',
          practice_id: 'coda-demo-practice',
          ends_at: 1800000000,
        });
      }
    } else if (route === '/auth/whoami') {
      await fulfillJson(requestId, 200, {
        physician_id: 'phys-demo-hannah',
        practice_id: 'coda-demo-practice',
        email: 'hannah@coda-demo.example',
        display_name: 'Dr. Hannah Alvarez',
        admin: false,
        practice_accounts: [],
      });
    } else if (route === '/health') {
      await fulfillJson(requestId, 200, {status: 'ok', retrying: [], held_by_newer_release: []});
    } else if (route === '/today') {
      await fulfillJson(requestId, 200, {
        date: '2026-11-21',
        time_zone: 'America/New_York',
        rows: [],
        as_of: 0,
        connections: [],
        seen: [],
        also_reviewed: 0,
        practice_outages: [],
      });
    } else if (route === '/patient-workspaces') {
      await fulfillJson(requestId, 200, {patients: []});
    } else if (route === '/practice-day') {
      await fulfillJson(requestId, 200, {
        current_time: '2026-11-21T09:00:00-05:00',
        time_zone: 'America/New_York',
        source_state: {state: 'connected'},
        visits: [],
      });
    } else if (route === '/recent-conversations') {
      await fulfillJson(requestId, 200, {conversations: []});
    } else if (route === '/updates') {
      await fulfillJson(requestId, 200, {
        active_runs: [],
        rows: [],
        today_items: [],
        reviewed_nothing_for_you: [],
        earlier_days: [],
      });
    } else if (route === '/patient-workspaces/events') {
      sseAuthHeader = request.headers.authorization || request.headers.Authorization || '';
      await call('Fetch.fulfillRequest', {
        requestId,
        responseCode: 200,
        responseHeaders: [
          {name: 'content-type', value: 'text/event-stream'},
          {name: 'cache-control', value: 'no-store'},
        ],
        body: Buffer.from(': connected\n\n').toString('base64'),
      });
    } else {
      await fulfillJson(requestId, 200, {});
    }
  };
  socket.addEventListener('message', ({data}) => {
    const msg = JSON.parse(data);
    if (msg.method === 'Fetch.requestPaused') {
      handleFetchPaused(msg.params).catch(err => errors.push({message: String(err)}));
    }
  });
  await call('Fetch.enable', {patterns: [{urlPattern: `${origin}/coda/physician*`}]});
  const navLive = await call('Page.navigate', {url: `${origin}/coda/physician/`});
  assert(!navLive.errorText, navLive.errorText);
  await until(
    `document.querySelector('.demo-quick-bar')?.innerText.includes('Live harness') && document.body.innerText.includes('Dr. Hannah Alvarez')`,
    '/coda/physician/ switches to Live harness when POST /coda/physician/api/auth/demo-session succeeds',
  );
  for (let i = 0; i < 100 && sseAuthHeader !== 'Bearer mock-live-demo-1'; i++) await delay(100);
  assert.equal(liveSessionCalls, 1, 'Expected /coda/physician/api/auth/demo-session probe');
  assert.equal(sseAuthHeader, 'Bearer mock-live-demo-1', 'Expected SSE /coda/physician/api/patient-workspaces/events with live Bearer');
  const liveBody = await evaluate(`document.body.innerText`);
  assert(!liveBody.includes('Sign out'), 'Live demo must hide Sign out');
  assert(!liveBody.includes('Invite colleague'), 'Live demo must hide Invite colleague');
  await assertNoLoadErrors('Live harness subpath mock');

  // Verify honest budget-exhausted state + local SQLite fallback switchover button.
  allowMockBudget429 = true;
  budgetExhaustNextSession = true;
  await clickBtn('Reset demo');
  await until(
    `document.querySelector('.demo-quick-bar')?.innerText.includes('Live turn limit reached') && document.body.innerText.includes('Live demo turn budget reached for this visitor.') && document.body.innerText.includes('Use local chart preview')`,
    '429 demo_turn_budget_exhausted shows Live turn limit reached and Use local chart preview',
  );
  await clickBtn('Use local chart preview');
  await until(
    `document.querySelector('.demo-quick-bar')?.innerText.includes('Local chart preview') && document.body.innerText.includes('Gary Kowalczyk') && document.body.innerText.includes('6 need you')`,
    'Use local chart preview switches back to DemoStore cleanly',
  );
  await call('Fetch.disable');

  assert.deepEqual(errors, [], `Runtime exceptions: ${JSON.stringify(errors)}`);
  assert.deepEqual(failures, [], `HTTP >= 400: ${JSON.stringify(failures)}`);
  console.log('Browser SQLite Chords demo E2E passed with 0 errors and 0 HTTP failures.');
} finally {
  clearTimeout(deadline); socket?.close(); child.kill('SIGKILL'); await delay(150);
  try { rmSync(profile, {recursive: true, force: true}); } catch {}
}

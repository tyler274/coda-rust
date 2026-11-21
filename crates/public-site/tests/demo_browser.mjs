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
  socket.onmessage = ({data}) => {
    const msg = JSON.parse(data);
    if (msg.id && pending.has(msg.id)) {
      const item = pending.get(msg.id); pending.delete(msg.id);
      msg.error ? item.reject(new Error(JSON.stringify(msg.error))) : item.resolve(msg.result);
    }
    if (msg.method === 'Runtime.exceptionThrown') errors.push(msg.params.exceptionDetails);
    if (msg.method === 'Network.responseReceived' && msg.params.response.url.startsWith(origin) && msg.params.response.status >= 400) {
      failures.push({url: msg.params.response.url, status: msg.params.response.status});
    }
  };
  const call = (method, params = {}) => new Promise((resolve, reject) => { const id = ++next; pending.set(id, {resolve, reject}); socket.send(JSON.stringify({id, method, params})); });
  const evaluate = async expression => { const r = await call('Runtime.evaluate', {expression, returnByValue: true, awaitPromise: true}); assert(!r.exceptionDetails, JSON.stringify(r)); return r.result.value; };
  const until = async (expr, desc = expr) => {
    for (let i = 0; i < 200; i++) { if (await evaluate(expr)) return; await delay(100); }
    throw new Error(`Timed out waiting for: ${desc}\nBody:\n${await evaluate('document.body?.innerText.slice(0, 600) || ""')}`);
  };
  const snap = async name => writeFileSync(join(screenshots, `${name}.png`), Buffer.from((await call('Page.captureScreenshot', {format: 'png'})).data, 'base64'));
  const clickBtn = text => evaluate(`[...document.querySelectorAll('button')].find(b => b.innerText.includes(${JSON.stringify(text)})).click()`);
  const setInput = (sel, proto, val) => evaluate(`(() => { const el = document.querySelector(${JSON.stringify(sel)}); Object.getOwnPropertyDescriptor(${proto}.prototype, 'value').set.call(el, ${JSON.stringify(val)}); el.dispatchEvent(new Event('input', {bubbles: true})); })()`);

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
  await until(`location.pathname.includes('/physician') && document.body.innerText.includes('Gary Kowalczyk') && document.body.innerText.includes('Reset demo') && !document.body.innerText.includes('No visits remaining today')`, 'demo Today + Schedule');
  await snap('02-demo-today');

  await clickBtn('Find patient');
  await until(`!!document.querySelector('dialog input, input[type="search"], input')`, 'search dialog');
  await setInput('dialog input, input[type="search"], input', 'HTMLInputElement', 'Marchetti');
  await until(`document.body.innerText.includes('Kristen Marchetti')`, 'SQLite search match');
  await evaluate(`[...document.querySelectorAll('dialog button')].find(b => b.innerText.includes('Kristen Marchetti')).click()`);
  await until(`document.body.innerText.includes('Kristen Marchetti') && document.body.innerText.includes('Patient summary')`, 'Kristen Marchetti workspace');
  await snap('03-demo-search-workspace');

  await clickBtn('Today');
  await until(`document.body.innerText.includes('Gary Kowalczyk') && document.body.innerText.includes('6 need you')`, 'back on Today');
  await clickBtn('Gary Kowalczyk');
  await until(`document.body.innerText.includes('High-intensity statin review')`, 'Gary Kowalczyk workspace');

  await evaluate(`document.querySelector('[data-decide-recommendation]').click()`);
  await until(`document.body.innerText.includes('Active plan') && !document.body.innerText.includes('Undecided medication recommendations')`, 'recommendation approved');

  await setInput('textarea', 'HTMLTextAreaElement', 'What is his latest LDL and atorvastatin dose?');
  await evaluate(`[...document.querySelectorAll('button')].find(b => b.innerText.trim() === 'Send').click()`);
  await until(`document.body.innerText.includes('From SQLite chart search across') && document.querySelectorAll('.message-sources button').length > 0`, 'SQLite chat answer + citation');
  await evaluate(`document.querySelector('.message-sources button').click()`);
  await until(`!!document.querySelector('aside.source-reading-pane')`, 'citation reader open');
  await snap('04-demo-chat-sql-citation');
  await evaluate(`document.querySelector('aside.source-reading-pane button.source-pane-close')?.click()`);
  await until(`!document.querySelector('aside.source-reading-pane')`, 'citation reader closed');

  await call('Page.reload', {});
  await until(`document.body.innerText.includes('Gary Kowalczyk')`, 'reloaded Today');
  await clickBtn('Gary Kowalczyk');
  await until(`document.body.innerText.includes('From SQLite chart search across') && !document.body.innerText.includes('Undecided medication recommendations')`, 'IndexedDB persistence verified');

  await clickBtn('Reset demo');
  await until(`document.body.innerText.includes('Undecided medication recommendations') && !document.body.innerText.includes('From SQLite chart search across')`, 'Reset restored baseline');
  await snap('05-demo-after-reset');

  assert.deepEqual(errors, [], `Runtime exceptions: ${JSON.stringify(errors)}`);
  assert.deepEqual(failures, [], `HTTP >= 400: ${JSON.stringify(failures)}`);
  console.log('Browser SQLite Chords demo E2E passed with 0 errors and 0 HTTP failures.');
} finally {
  clearTimeout(deadline); socket?.close(); child.kill('SIGKILL'); await delay(150);
  try { rmSync(profile, {recursive: true, force: true}); } catch {}
}

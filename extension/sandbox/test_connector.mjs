// The shipped worker runs unchanged in a VM with simulated Chrome APIs. Auth,
// chart and bridge traffic use real local HTTP. This is not real Chrome proof.
import assert from 'node:assert/strict';
import {createServer} from 'node:http';
import {spawn} from 'node:child_process';
import {createContext, runInContext} from 'node:vm';
import {readFileSync, writeFileSync, mkdtempSync, existsSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join, dirname, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {webcrypto, createHash, randomUUID} from 'node:crypto';
import {createInterface} from 'node:readline';

const output = dirname(fileURLToPath(import.meta.url));
const root = output;
const identity = JSON.parse(readFileSync(join(root, 'extension-identity.json')));
const work = mkdtempSync(join(tmpdir(), 'coda-hint-independent-'));
const localFile = join(work, 'browser-local.json');
const checkpoint = join(work, 'checkpoint.json');
const source = readFileSync(process.env.HINT_WORKER_SOURCE || join(root, '../chrome/worker.js'), 'utf8');
const patient = 'sbx-pat-QItKCySDsdET';
const nativeFetch = globalThis.fetch;
let now = Date.now(), authCount = 0, reads = [], authMode = 'ok', readMode = 'ok';
let authSuccessStatus = 201;
let loseCompletion = false, refuseBridge = false, portalCount = 0, hangingRead = null;
// What the open Hint tab shows: 'ready', 'signed_out', 'other_practice' or 'sign_in_page'.
let hintPage = 'ready';
let cookies = 'fixture-browser-cookie', cookieAccepted = true, bridge, vm;
let expectedRefresh = null, exchanges = 0;
const CANARY = 'FIXTURE_REFRESH_SECRET_MUST_NOT_LEAVE_BROWSER';
const claims = () => ({iat: Math.floor(now / 1000), exp: Math.floor(now / 1000) + (authMode === 'short_lived' ? 60 : 900),
  tenantId: authMode === 'wrong_tenant' ? 'wrong-tenant' : 'fixture-tenant', sub: 'fixture-user', iss: 'fixture-idp', aud: 'fixture-client'});
const jwt = () => 'fixture.' + Buffer.from(JSON.stringify(claims())).toString('base64url') + '.fixture';
const report = {scope: 'VM Chrome API fixture + real loopback HTTP + real MCP stdio; no live Hint or real Chrome', cases: []};
report.tested_worker_sha256 = createHash('sha256').update(source).digest('hex');
report.test_sha256 = createHash('sha256').update(readFileSync(fileURLToPath(import.meta.url))).digest('hex');
report.sources = Object.fromEntries(['../chrome/manifest.json', '../chrome/bootstrap.js', '../chrome/worker.js',
  '../chrome/options.html', '../chrome/popup.js', 'bridge.py']
  .map(p => [p, createHash('sha256').update(readFileSync(join(root, p))).digest('hex')]));

const vendor = createServer(async (req, res) => {
  let text = ''; for await (const chunk of req) text += chunk;
  if (['/frontegg/oauth/authorize/silent', '/frontegg/oauth/token'].includes(req.url)) {
    authCount++;
    const exchange = req.url.endsWith('/token');
    if (exchange) {
      exchanges++;
      assert.equal(req.headers.cookie, undefined, 'Explicit refresh must not depend on the website cookie');
      assert.deepEqual(JSON.parse(text), {grant_type: 'refresh_token', refresh_token: expectedRefresh});
      assert.match(req.headers.authorization || '', /^Bearer fixture\./);
    } else {
      assert.equal(req.headers.cookie, cookies);
      assert.deepEqual(JSON.parse(text), {tenantId: 'fixture-tenant'});
    }
    if ((exchange && ['stale', 'switched', 'denied', 'forbidden'].includes(authMode)) ||
        (!exchange && (authMode === 'denied' || !cookieAccepted))) {
      res.writeHead(authMode === 'forbidden' ? 403 : 401, {'Content-Type': 'application/json'});
      if (authMode === 'opaque') {res.end(CANARY); return;}
      res.end(JSON.stringify({error: exchange ? 'invalid_grant' : 'login_required', detail: CANARY}));
      return;
    }
    res.statusCode = authSuccessStatus;
    res.setHeader('Content-Type', 'application/json');
    expectedRefresh = CANARY + '_' + authCount;
    if (!exchange && authMode === 'switched') authMode = 'wrong_tenant';
    res.end(JSON.stringify({id_token: jwt(), ...(authMode === 'missing_refresh' ? {} :
      {refresh_token: expectedRefresh}), expires_in: 900}));
    return;
  }
  reads.push({url: req.url, at: now});
  assert.match(req.headers.authorization || '', /^Bearer fixture\./);
  if (readMode === '401' || readMode === '401-once') {
    if (readMode === '401-once') readMode = 'ok';
    res.writeHead(401); res.end(CANARY); return;
  }
  if (readMode === '403') {res.writeHead(403); res.end(CANARY); return;}
  if (readMode === '429') {res.writeHead(429); res.end(CANARY); return;}
  if (readMode === 'hang') {res.writeHead(200, {'Content-Type': 'application/json'}); res.flushHeaders(); hangingRead?.(); return;}
  if (readMode === 'html') {res.writeHead(200); res.end('<html>Sign in</html>'); return;}
  let body = {total: 0, entities: []};
  if (req.url.startsWith('/api/provider/patients?')) body = [{id: patient, first_name: 'Fictional'}];
  else if (req.url === '/api/provider/patients/' + patient) body = {id: patient, first_name: 'Fictional'};
  else if (req.url.includes('/diagnoses')) body = {total: 1, entities: [{patientId: patient, description: 'FICTIONAL COND-101'}]};
  else if (req.url.startsWith('/notes/')) body = {total: 0, data: [], limit: 100, offset: 0};
  res.setHeader('Content-Type', 'application/json'); res.end(JSON.stringify(body));
});
await new Promise(resolve => vendor.listen(0, '127.0.0.1', resolve));
const vendorPort = vendor.address().port;

async function startBridge() {
  bridge = spawn('python3', [join(root, 'bridge.py'), '--serve', '--port', '0', '--checkpoint', checkpoint], {stdio: ['ignore', 'pipe', 'pipe']});
  const lines = createInterface({input: bridge.stdout});
  const first = await new Promise((resolve, reject) => {
    lines.once('line', resolve); bridge.once('error', reject); bridge.once('exit', code => reject(Error('Bridge exited ' + code)));
  });
  return JSON.parse(first).port;
}
let bridgePort = await startBridge();
async function stopBridge() {
  const proc = bridge;
  await new Promise(resolve => {proc.once('exit', resolve); proc.kill('SIGTERM');});
}
async function rpc(name, args = {}) {
  const response = await nativeFetch(`http://127.0.0.1:${bridgePort}/rpc`, {method: 'POST',
    headers: {'Content-Type': 'application/json'}, body: JSON.stringify({name, arguments: args})});
  assert.equal(response.status, 200); return response.json();
}
function area(durable, map = {}) {
  return {map, async get(key) {return key === null ? structuredClone(map) : {[key]: structuredClone(map[key])};},
    async set(value) {Object.assign(map, structuredClone(value)); if (durable) writeFileSync(localFile, JSON.stringify(map));},
    async remove(key) {delete map[key]; if (durable) writeFileSync(localFile, JSON.stringify(map));}};
}
const session = area(false);
async function createWorker(clearSession = false) {
  if (clearSession) for (const key of Object.keys(session.map)) delete session.map[key];
  const storage = {local: area(true, existsSync(localFile) ? JSON.parse(readFileSync(localFile)) : {}), session};
  const listeners = {}, alarms = new Map();
  const chrome = {storage, runtime: {id: identity.id, getManifest: () => JSON.parse(readFileSync(join(root, '../chrome/manifest.json'))), getURL: p => identity.origin + '/' + p,
    onStartup: {addListener: fn => {listeners.startup = fn;}}, onMessage: {addListener: fn => {listeners.message = fn;}}},
    alarms: {async get(name) {return alarms.get(name);}, async create(name, options) {alarms.set(name, options);},
      async clear(name) {return alarms.delete(name);}, onAlarm: {addListener: fn => {listeners.alarm = fn;}}},
    // The worker opens, closes and injects into no tab other than the one Connect reads once.
    tabs: {async query() {
      const url = hintPage === 'sign_in_page' ? 'https://accounts.hint.com/oauth/account/login' : 'https://app.hint.com/patients';
      return Array.from({length: portalCount}, (_, id) => ({id, url}));
    }},
    scripting: {async executeScript({world, files}) {
      assert.equal(world, 'MAIN');
      assert.equal(files[0], 'bootstrap.js');
      const script = readFileSync(join(root, '../chrome/bootstrap.js'), 'utf8');
      const signedIn = hintPage !== 'signed_out', sandbox = hintPage !== 'other_practice';
      const page = createContext({location: {origin: 'https://app.hint.com'},
        document: {body: {innerText: sandbox ? 'Currently viewing sandbox data.' : 'Another practice'},
          querySelector: () => (sandbox ? {} : null)},
        window: {hintSession: {getSnapshot: () => ({status: signedIn ? 'active' : 'expired', token: signedIn ? jwt() : null})},
          fronteggApp: {store: {getState: () => ({root: {context: {baseUrl: 'https://accounts.hint.com'}},
            auth: {isAuthenticated: signedIn, tenantsState: {activeTenant: {tenantId: 'fixture-tenant'}}}})}}},
        Date: Clock, atob});
      return [{result: runInContext(script, page)}];
    }}};
  class Clock extends Date {static now() {return now;}}
  const sandbox = createContext({chrome, Date: Clock, crypto: webcrypto, setTimeout, clearTimeout,
    AbortController, TextEncoder, TextDecoder, URL, atob, fetch: async (url, options) => {
      const parsed = new URL(url);
      if (parsed.origin === 'http://127.0.0.1:51383') {
        if (refuseBridge) return new Response('{"error": "Route or origin refused"}', {status: 403});
        const response = await nativeFetch(`http://127.0.0.1:${bridgePort}${parsed.pathname}`, {...options,
          headers: {...options.headers, Origin: identity.origin}});
        if (parsed.pathname === '/complete' && loseCompletion) {loseCompletion = false; throw Error('Fixture response loss');}
        return response;
      }
      assert(['accounts.hint.com', 'api.hint.com', 'api.one.hint.com'].includes(parsed.hostname));
      assert.equal(options.redirect, 'error');
      const headers = {...options.headers};
      if (options.credentials === 'include') headers.Cookie = cookies;
      return nativeFetch(`http://127.0.0.1:${vendorPort}${parsed.pathname}${parsed.search}`, {...options, headers});
    }});
  runInContext(source, sandbox, {filename: 'worker.js'});
  vm = {storage, listeners};
  return vm;
}
async function message(action) {
  return new Promise(resolve => {
    const kept = vm.listeners.message({action}, {id: identity.id, url: identity.origin + '/options.html'}, resolve);
    assert.equal(kept, true);
  });
}
async function tick() {vm.listeners.alarm({name: 'coda-hint-sandbox'}); return message('status');}
function evidence(name, detail = {}) {report.cases.push({name, passed: true, ...detail});}
async function resetConnection() {
  await message('stop');
  await vm.storage.local.set({context: {...vm.storage.local.map.context, prepared_at: now}});
  authMode = 'ok'; readMode = 'ok'; cookieAccepted = true;
}

try {
  await createWorker();
  portalCount = 1;
  const prepared = await message('prepare'); assert.equal(prepared.state.state, 'prepared');
  assert(!JSON.stringify(vm.storage.local.map.context).includes('fixture.'));
  portalCount = 0;
  assert.equal((await message('start')).state.state, 'connected');
  evidence('actual bootstrap script admits sandbox metadata without copying bearer token; connection metadata contains no credentials');
  const roster = await rpc('request_roster'); await tick();
  assert.equal((await rpc('read_result', {request_id: roster.request_id})).complete, true);
  const chart = await rpc('request_chart', {patient_id: patient});
  assert.equal((await rpc('request_chart', {patient_id: patient})).request_id, chart.request_id);
  for (let i = 0; i < 3; i++) await tick();
  const before = authCount;
  await createWorker(); await tick();
  assert.equal(authCount, before, 'Worker termination must preserve Chrome memory token without another refresh');
  await createWorker(true); await tick();
  assert.equal(authCount, before + 1, 'Browser restart must use normal cookie-backed renewal');
  for (let i = 0; i < 7; i++) await tick();
  const result = await rpc('read_result', {request_id: chart.request_id});
  assert.equal(result.complete, true); assert.equal(result.sections.length, 12);
  assert(result.sections.every(s => s.attempts === 1));
  assert.equal(reads.length, 13, 'Completed sections must not be re-fetched after restart');
  evidence('roster, twelve-section chart, deduplicated pending request and two restart types', {reads: reads.length});
  assert(JSON.parse(readFileSync(checkpoint)).events.some(e => e.auth_events?.some(a => a.http_status === 201)));
  evidence('successful HTTP201 silent renewal survives receipt delivery and chart acquisition');

  authSuccessStatus = 200;
  cookieAccepted = false;
  now += 14 * 60000;
  const later = await rpc('request_chart', {patient_id: patient});
  const renewedState = await tick(); assert.equal(authCount, before + 2);
  assert.equal(renewedState.state.state, 'connected', 'Explicit refresh must keep chart acquisition connected when the website cookie is no longer usable');
  const firstLater = (await rpc('read_result', {request_id: later.request_id})).sections[0];
  assert(firstLater.result.receipt.token_iat > result.sections[0].result.receipt.token_iat);
  assert(firstLater.result.receipt.started > result.sections[0].result.receipt.started);
  assert.equal(exchanges, 1, 'Renewal must use explicit refresh grant after website cookie expires');
  const rotated = expectedRefresh;
  now += 14 * 60000;
  await tick();
  assert.equal(exchanges, 2);
  assert.notEqual(expectedRefresh, rotated, 'Server rotates the refresh token and next exchange must consume that rotation');
  evidence('two explicit rotating refresh exchanges and chart reads without a usable website cookie');

  loseCompletion = true; await tick();
  assert(vm.storage.local.map.outbox);
  const readCount = reads.length;
  await stopBridge(); bridgePort = await startBridge();
  await createWorker(); await tick();
  assert.equal(vm.storage.local.map.outbox, undefined);
  assert.equal(reads.length, readCount + 1, 'Outbox delivery must not re-fetch its completed section');
  evidence('lost completion response, real bridge process restart, idempotent outbox recovery');

  loseCompletion = true; await tick();
  assert(vm.storage.local.map.outbox);
  await resetConnection();
  assert.equal((await message('start')).state.state, 'connected', 'Reconnecting after Disconnect must deliver the waiting read');
  assert.equal(vm.storage.local.map.outbox, undefined);
  evidence('a read still waiting for delivery at Disconnect is delivered on reconnect');

  await resetConnection(); await message('start');
  authMode = 'stale'; now += 14 * 60000;
  const recoveryRequest = await rpc('request_chart', {patient_id: patient});
  const beforeRecoveryReads = reads.length, beforeRecoveryAuth = authCount;
  assert.equal((await tick()).state.state, 'connected', 'A rejected rotating grant must use an available same-principal silent session');
  assert.equal(authCount, beforeRecoveryAuth + 2);
  assert.equal(reads.length, beforeRecoveryReads + 1);
  await tick();
  assert((await rpc('read_result', {request_id: recoveryRequest.request_id})).sections.some(s => s.result?.receipt.http_status === 200));
  evidence('rejected grant followed by one same-practice silent exchange and a fresh successful chart read');

  await resetConnection(); await message('start'); now += 60000;
  readMode = '401-once';
  const countBefore401 = reads.length, exchangesBefore401 = exchanges;
  const interrupted = await rpc('request_chart', {patient_id: patient});
  const queuedBefore = await rpc('read_result', {request_id: interrupted.request_id});
  const recovered = await tick();
  assert.equal(recovered.state.state, 'connected', 'An unexpired credential rejected by a chart GET must trigger one fresh exchange');
  assert.equal(exchanges, exchangesBefore401+1);
  assert.equal(reads.length, countBefore401+2);
  assert.equal(reads.at(-1).url, reads.at(-2).url);
  const after401 = await rpc('read_result', {request_id: interrupted.request_id});
  const newlyDone = after401.sections.filter((s, i) => s.state === 'done' && queuedBefore.sections[i].state !== 'done');
  assert.equal(newlyDone.length, 1);
  assert.equal(newlyDone[0].result.receipt.token_iat, Math.floor(now/1000));
  assert.equal(newlyDone[0].attempts, 1);
  assert.equal(recovered.state.token_iat, newlyDone[0].result.receipt.token_iat);
  evidence('chart 401 while token is unexpired reacquires once and completes the same GET under the new token');

  await resetConnection(); await message('start'); readMode = '401-once';
  const readsBeforeIdentical = reads.length, exchangesBeforeIdentical = exchanges;
  await rpc('request_chart', {patient_id: patient});
  assert.equal((await tick()).state.state, 'chart_unauthorized');
  assert.equal(exchanges, exchangesBeforeIdentical+1);
  assert.equal(reads.length, readsBeforeIdentical+1);
  evidence('a successful exchange that reissues the rejected bearer cannot replay the GET');

  await resetConnection(); await message('start'); now += 60000;
  readMode = '401'; cookieAccepted = false; authMode = 'denied';
  await rpc('request_chart', {patient_id: patient});
  const beforeDeniedChart = reads.length, authBeforeDeniedChart = authCount;
  assert.equal((await tick()).state.state, 'interaction_required');
  const denial = JSON.parse(readFileSync(checkpoint)).events.at(-1);
  assert.deepEqual(denial.auth_events.map(a=>[a.kind,a.http_status]), [
    ['chart_401',401],['refresh_token',401],['silent_reconnect',401]]);
  assert.equal(denial.auth_events[0].job_id, denial.job_id);
  assert.equal(denial.auth_events[2].category, 'login_required');
  await tick();
  assert.equal(reads.length, beforeDeniedChart+1);
  assert.equal(authCount, authBeforeDeniedChart+2);
  assert.equal(session.map.access, undefined);
  evidence('chart 401 plus invalid grant and explicit login_required stops without a second chart read');

  await resetConnection(); await message('start');
  authMode = 'switched'; now += 14 * 60000;
  const beforeSwitch = reads.length;
  assert.equal((await tick()).state.state, 'auth_identity_changed');
  assert.equal(reads.length, beforeSwitch); assert.equal(session.map.access, undefined);
  evidence('silent recovery for another tenant stops before any chart read');

  await resetConnection(); await message('start');
  authMode = 'forbidden'; now += 14 * 60000;
  const beforeForbidden = authCount;
  assert.equal((await tick()).state.state, 'access_denied'); assert.equal(authCount, beforeForbidden + 1);
  evidence('forbidden grant is terminal without another authentication attempt');

  await resetConnection(); await message('start');
  cookieAccepted = false; authMode = 'denied'; now += 14 * 60000;
  const priorExchanges = exchanges, priorAuth = authCount;
  assert.equal((await tick()).state.state, 'interaction_required');
  assert.equal(exchanges, priorExchanges + 1);
  assert.equal(authCount, priorAuth + 2, 'One silent authorization check after a 401');
  assert.equal(session.map.access, undefined, 'Terminal refresh rejection must clear both credentials');
  const deniedEvents = JSON.parse(readFileSync(checkpoint)).events.at(-1).auth_events;
  assert.deepEqual(deniedEvents.map(e => [e.kind, e.category]), [['refresh_token', 'invalid_grant'], ['silent_reconnect', 'login_required']]);
  await tick(); assert.equal(authCount, priorAuth + 2);
  evidence('provider login_required stops once, preserves sanitized causes and clears credentials');

  await resetConnection(); cookieAccepted = false; authMode = 'opaque';
  const opaque = await message('start');
  assert.equal(opaque.state.state, 'silent_auth_unavailable');
  const halt = JSON.parse(readFileSync(checkpoint)).events.at(-1);
  assert.equal(halt.auth_events[0].category,'unclassified');
  evidence('opaque silent 401 stays unattributed and stops without opening anything');

  await resetConnection(); authMode = 'missing_refresh';
  assert.equal((await message('start')).state.state, 'auth_response_invalid');
  assert.equal(session.map.access, undefined);
  evidence('missing refresh credential is an incomplete auth response, never a connected state');

  await resetConnection(); authMode = 'wrong_tenant';
  const wrong = await message('start'); assert.equal(wrong.state.state, 'auth_identity_changed');
  assert.equal(session.map.access, undefined);
  const stoppedCount = authCount; await tick(); assert.equal(authCount, stoppedCount);
  evidence('changed identity stops acquisition without repeated auth retries');

  await resetConnection(); authMode = 'short_lived';
  assert.equal((await message('start')).state.state, 'auth_response_invalid');
  assert.equal(session.map.access, undefined);
  evidence('a token too close to expiry is an invalid auth response, not a changed account');

  await resetConnection(); authMode = 'denied';
  assert.equal((await message('start')).state.state, 'interaction_required');
  assert.equal(session.map.access, undefined); await tick();
  evidence('signed-out renewal is auth_required, never generic request_failed');

  for (const [mode, expected] of [['401', 'chart_unauthorized'], ['403', 'access_denied'], ['429', 'rate_limited'], ['html', 'read_non_json'], ['hang', 'read_transport_failed']]) {
    await resetConnection(); await message('start'); now += 60000; readMode = mode;
    await rpc('request_chart', {patient_id: patient});
    const count = reads.length, authBefore = authCount; const outcome = await tick();
    assert.equal(outcome.state.state, expected);
    assert.equal(session.map.access, undefined);
    await tick(); assert.equal(reads.length, count + (mode === '401' ? 2 : 1));
    assert.equal(authCount, authBefore + (mode === '401' ? 1 : 0));
    evidence('chart ' + mode + ' stops once and clears access token');
  }
  await resetConnection(); await message('start'); readMode = 'hang';
  await rpc('request_chart', {patient_id: patient});
  const waiting = new Promise(resolve => {hangingRead = resolve;});
  vm.listeners.alarm({name: 'coda-hint-sandbox'}); await waiting; hangingRead = null;
  const stopResult = await message('stop');
  assert.equal(stopResult.state.state, 'stopped'); assert.equal(session.map.access, undefined);
  evidence('Stop aborts an in-flight response body and clears Chrome-memory token');

  await resetConnection(); refuseBridge = true;
  const workerRefused = await message('start');
  assert.equal(workerRefused.state.state, 'bridge_refused'); assert.equal(workerRefused.state.http_status, 403);
  assert.equal(workerRefused.state.cause, '{"error": "Route or origin refused"}');
  refuseBridge = false;
  evidence('bridge permission rejection is terminal, not an offline transport retry');
  await resetConnection(); await message('start');
  now += 241 * 60000; const count = reads.length; await tick();
  assert.equal(reads.length, count); assert.equal(session.map.access, undefined);
  evidence('deadline ends acquisition before another EHR request');

  const refused = await nativeFetch(`http://127.0.0.1:${bridgePort}/heartbeat`, {method: 'POST', headers: {'Origin': 'https://evil.invalid', 'Content-Type': 'application/json'}, body: '{}'});
  assert.equal(refused.status, 403);
  const invalid = await nativeFetch(`http://127.0.0.1:${bridgePort}/heartbeat`, {method: 'POST', headers: {'Origin': identity.origin, 'Content-Type': 'application/json'}, body: JSON.stringify({authorization: CANARY})});
  assert.equal(invalid.status, 400);
  for (const file of [checkpoint, localFile]) {
    const text = readFileSync(file, 'utf8');
    assert(!text.includes(CANARY)); assert(!text.includes('Bearer fixture.')); assert(!text.includes('id_token'));
  }
  evidence('strict bridge origin/schema and no fake credentials in persisted browser metadata or MCP checkpoint');

  const wire = async (path, body) => {
    const response = await nativeFetch(`http://127.0.0.1:${bridgePort}${path}`, {method: 'POST',
      headers: {Origin: identity.origin, 'Content-Type': 'application/json'}, body: JSON.stringify(body)});
    return {status: response.status, value: await response.json()};
  };
  const owner = {run_id: randomUUID(), worker: randomUUID()};
  assert.equal((await wire('/heartbeat', {...owner, until: Date.now() + 60000, portal_tab_count: 0, auth_events: []})).status, 200);
  const claimed = (await wire('/claim', owner)).value.job; assert(claimed);
  const text = JSON.stringify(claimed.section === 'patient' ? {id: patient} : {total: 0, entities: []});
  const payload = {...owner, job_id: claimed.id, lease: claimed.lease, result: {json_text: text,
    receipt: {started: now, ended: now, http_status: 200, bytes: Buffer.byteLength(text),
      sha256: '0'.repeat(64), token_iat: Math.floor(now / 1000), token_exp: Math.floor(now / 1000) + 900}}};
  assert.equal((await wire('/complete', payload)).status, 400, 'Bridge must independently reject a body/hash mismatch');
  payload.result.receipt.sha256 = createHash('sha256').update(text).digest('hex');
  assert.equal((await wire('/complete', {...payload, worker: randomUUID()})).value.outcome, 'stale');
  assert.equal((await wire('/complete', payload)).value.outcome, 'accepted');
  assert.equal((await wire('/complete', payload)).value.outcome, 'duplicate');
  evidence('bridge independently validates original JSON hash and fences stale/duplicate section completions');

  const mcp = spawn('python3', [join(root, 'bridge.py'), '--port', String(bridgePort)], {stdio: ['pipe', 'pipe', 'pipe']});
  const replies = []; const lines = createInterface({input: mcp.stdout});
  lines.on('line', line => replies.push(JSON.parse(line)));
  for (const request of [
    {id: 1, method: 'initialize', params: {protocolVersion: '2024-11-05'}},
    {id: 2, method: 'tools/list'},
    {id: 3, method: 'tools/call', params: {name: 'read_result', arguments: {request_id: chart.request_id}}}]) {
    mcp.stdin.write(JSON.stringify({jsonrpc: '2.0', ...request}) + '\n');
  }
  mcp.stdin.end(); await new Promise(resolve => mcp.once('exit', resolve));
  assert.equal(replies.length, 3); assert.equal(replies[1].result.tools.length, 4);
  assert.equal(replies[2].result.structuredContent.complete, true);
  assert(replies[2].result.structuredContent.sections[0].result.body.entities[0].description.includes('COND-101'));
  assert(!JSON.stringify(replies).includes(CANARY));
  evidence('real MCP subprocess initialize/list/call retrieves acquired chart JSON');

  // Each failure keeps its own JSON-RPC code and says what went wrong.
  const refusals = async (port, lines) => {
    const proxy = spawn('python3', [join(root, 'bridge.py'), '--port', String(port)], {stdio: ['pipe', 'pipe', 'pipe']});
    const heard = []; createInterface({input: proxy.stdout}).on('line', line => heard.push(JSON.parse(line)));
    for (const line of lines) proxy.stdin.write(line + '\n');
    proxy.stdin.end(); await new Promise(resolve => proxy.once('exit', resolve));
    return heard;
  };
  const [malformed, unknown, badParams] = await refusals(bridgePort, ['{not json',
    JSON.stringify({jsonrpc: '2.0', id: 4, method: 'resources/list'}),
    JSON.stringify({jsonrpc: '2.0', id: 5, method: 'tools/call', params: {}})]);
  assert.equal(malformed.error.code, -32700); assert.match(malformed.error.message, /^Parse error: /);
  assert.equal(unknown.error.code, -32601); assert.equal(unknown.error.message, 'Method not found: resources/list');
  assert.equal(badParams.error.code, -32602); assert.match(badParams.error.message, /^Invalid params: KeyError\('name'\)/);
  const [refusedByBridge] = await refusals(bridgePort, [JSON.stringify({jsonrpc: '2.0', id: 6, method: 'tools/call', params: {name: 'read_result', arguments: {}}})]);
  assert.equal(refusedByBridge.error.code, -32603); assert.match(refusedByBridge.error.message, /^The bridge refused the call with HTTP 400: .*"reason"/);
  const [unreachable] = await refusals(1, [JSON.stringify({jsonrpc: '2.0', id: 7, method: 'tools/call', params: {name: 'connection_status', arguments: {}}})]);
  assert.equal(unreachable.error.code, -32603); assert.match(unreachable.error.message, /^The bridge at 127\.0\.0\.1:1 isn't reachable: /);
  evidence('the MCP proxy keeps each failure distinct: bad JSON, unknown method, bad params, a bridge refusal, an unreachable bridge');

  await resetConnection(); portalCount = 1;
  assert.equal((await message('connect')).state.state, 'connected');
  await message('stop');
  assert.equal(session.map.access, undefined);
  evidence('one Connect action works with the Hint page open and Disconnect clears credentials');

  const kept = JSON.stringify(vm.storage.local.map);
  const where = async () => (await message('status')).readiness;
  portalCount = 0;
  assert.equal(await where(), 'hint_closed');
  portalCount = 1; hintPage = 'sign_in_page';
  assert.equal(await where(), 'signed_out');
  hintPage = 'signed_out';
  assert.equal(await where(), 'signed_out');
  hintPage = 'other_practice';
  assert.equal(await where(), 'other_practice');
  hintPage = 'ready';
  const ready = await message('status');
  assert.equal(ready.readiness, 'ready');
  assert.equal(ready.practice, 'Coda Health Sandbox Practice');
  portalCount = 2;
  assert.equal((await message('status')).readiness, undefined, 'Several Hint tabs leave readiness unknown');
  assert.equal(JSON.stringify(vm.storage.local.map), kept, 'Reporting where Hint stands stores nothing');
  portalCount = 0;
  evidence('status reports where Hint stands from the page check, read fresh and stored nowhere');
  report.passed = true;
} finally {
  report.auth_requests = authCount; report.chart_requests = reads.length;
  report.fixture_workdir = work;
  writeFileSync(join(output, process.env.HINT_WORKER_SOURCE ? 'before-local-proof.json' : 'local-proof.json'), JSON.stringify(report, null, 2) + '\n');
  await stopBridge(); vendor.closeAllConnections(); await new Promise(resolve => vendor.close(resolve));
}
console.log(JSON.stringify(report, null, 2));

// The worker owns renewal. A terminal silent denial can trigger one inactive
// visit to the normal app entry to distinguish a login challenge from a 401.
const BRIDGE = 'http://127.0.0.1:51383';
const AUTH = 'https://accounts.hint.com/frontegg/oauth/authorize/silent';
const TOKEN = 'https://accounts.hint.com/frontegg/oauth/token';
const VERSION = chrome.runtime.getManifest().version;
const RUN_MS = 4 * 60 * 60000;
const ALARM = 'coda-hint-sandbox';
const PATIENTS = new Set(['sbx-pat-QItKCySDsdET', 'sbx-pat-mCyJLEDTaMIJ',
  'sbx-pat-RqBReejavZju', 'sbx-pat-W2Ylt6lGTKrM']);
const SECTIONS = ['diagnoses', 'allergies', 'medications', 'freeform-medications',
  'surgeries', 'social-history', 'family-history', 'vaccines', 'general', 'patient', 'notes', 'tasks'];
let lane = Promise.resolve();
let controller = null;
function serial(action) {
  const next = lane.then(action);
  lane = next.catch(() => {});
  return next;
}
// A fault's cause is text diagnostics can hold. Only the bridge's faults carry one; the
// authentication handlers keep to fixed categories.
function fault(code, status = null, cause = null) {return Object.assign(new Error(code), {code, status, cause});}
async function stored(key) {return (await chrome.storage.local.get(key))[key];}
async function setState(state, extra = {}) {
  await chrome.storage.local.set({state: {state, at: Date.now(), ...extra}});
}
// Only fixed categories can leave the authentication response handler.
async function authError(response) {
  const unknown = 'unclassified';
  const reader = response.body?.getReader();
  if (!reader) return unknown;
  try {
    const bytes = [];
    for (;;) {
      const {done, value} = await reader.read();
      if (done) break;
      if (bytes.length + value.length > 8192) return unknown;
      bytes.push(...value);
    }
    const body = JSON.parse(new TextDecoder().decode(Uint8Array.from(bytes)));
    const fields = [body.error, body.errorCode, body.code, body.message, body.error_description,
      ...(Array.isArray(body.errors) ? body.errors : [])];
    const known = new Map([['Refresh token is not found', 'refresh_not_found'],
      ['ER-01095', 'session_inactive'], ['interaction_required', 'interaction_required'],
      ['login_required', 'login_required'], ['access_denied', 'access_denied'],
      ['invalid_grant', 'invalid_grant']]);
    for (const [literal, category] of known) if (fields.includes(literal)) return category;
    return unknown;
  } catch (_) {return unknown;}
  finally {void reader.cancel().catch(() => {});}
}
async function request(url, options) {
  controller = new AbortController();
  const timer = setTimeout(() => controller?.abort(), 15000);
  try {
    const response = await fetch(url, {...options, cache: 'no-store', redirect: 'error', signal: controller.signal});
    const auth = url === TOKEN || url === AUTH;
    // Keep cancellation/timeout active through body delivery, not just headers. An authentication
    // refusal's body is read only for its category.
    const text = response.ok || !auth ? await response.text() : '';
    const category = !response.ok && auth ? await authError(response) : null;
    return {ok: response.ok, status: response.status, category, text: async () => text,
      json: async () => JSON.parse(text)};
  }
  finally {clearTimeout(timer); controller = null;}
}
async function local(path, body) {
  let response;
  try {
    response = await request(BRIDGE + path, {method: 'POST', credentials: 'omit',
      headers: {'Content-Type': 'application/json'}, body: JSON.stringify(body)});
  } catch (error) {throw fault('bridge_offline', null, String(error));}
  // The bridge says why it refused in the body.
  if (!response.ok) throw fault('bridge_refused', response.status, await response.text());
  try {return await response.json();} catch (error) {throw fault('bridge_refused', response.status, String(error));}
}
function active(run) {
  if (!run || run.stopped || Date.now() >= run.until) throw fault('stopped');
}
function metadata(claims) {return {token_iat: claims.iat, token_exp: claims.exp};}
function decode(token) {
  try {return JSON.parse(atob(token.split('.')[1].replace(/-/g, '+').replace(/_/g, '/')));}
  catch (_) {throw fault('auth_response_invalid');}
}
async function recordAuth(event) {
  const pending = (await stored('auth_events')) || [];
  await chrome.storage.local.set({auth_events: [...pending, {at: Date.now(), ...event}]});
}
async function exchange(run, context, saved, kind) {
  active(run);
  const renewing = kind === 'refresh_token';
  let response;
  try {
    // The hosted SDK's __refreshToken uses this explicit grant. Cookies are
    // needed only for initial silent authorization, never for this exchange.
    response = await request(renewing ? TOKEN : AUTH, {method: 'POST',
      credentials: renewing ? 'omit' : 'include',
      headers: {'Content-Type': 'application/json', Accept: 'application/json',
        ...(renewing ? {Authorization: 'Bearer ' + saved.token} : {})},
      body: JSON.stringify(renewing ? {grant_type: 'refresh_token', refresh_token: saved.refresh_token} :
        {tenantId: context.tenant_id})});
  } catch (_) {throw fault('auth_transport_failed');}
  if (!response.ok) {
    await recordAuth({kind, http_status: response.status, category: response.category});
    let code = 'auth_failed';
    if (response.status === 429) code = 'rate_limited';
    else if (response.status === 403 || response.category === 'access_denied') code = 'access_denied';
    else if (renewing && (response.status === 401 ||
        (response.status === 400 && ['invalid_grant', 'refresh_not_found'].includes(response.category)))) {
      code = 'refresh_rejected';
    } else if (response.category === 'login_required' || response.category === 'interaction_required') {
      code = 'interaction_required';
    } else if (response.status === 401 || response.category === 'invalid_grant') code = 'silent_auth_unavailable';
    throw fault(code, response.status);
  }
  let data;
  try {data = await response.json();} catch (_) {throw fault('auth_response_invalid');}
  if (typeof data.id_token !== 'string' || typeof data.refresh_token !== 'string' ||
      !data.refresh_token) throw fault('auth_response_invalid');
  const claims = decode(data.id_token);
  if (claims.tenantId !== context.tenant_id || claims.sub !== context.subject ||
      claims.iss !== context.issuer || JSON.stringify(claims.aud) !== JSON.stringify(context.audience)) {
    throw fault('auth_identity_changed');
  }
  // The same account, but a token without its times or too close to expiry to read with.
  if (!Number.isFinite(claims.iat) || !Number.isFinite(claims.exp) || claims.exp * 1000 <= Date.now() + 90000) {
    throw fault('auth_response_invalid');
  }
  active(run);
  const value = {run_id: run.id, token: data.id_token, refresh_token: data.refresh_token,
    claims: {iat: claims.iat, exp: claims.exp}};
  // One memory-only credential owner, replaced atomically after rotation and
  // removed on Stop/terminal failure. No credential enters local storage or MCP.
  await chrome.storage.session.set({access: value});
  await recordAuth({kind, http_status: response.status, ...metadata(claims)});
  return value;
}
async function tokenFor(run, context, rejectedToken = null) {
  active(run);
  const saved = (await chrome.storage.session.get('access')).access;
  if (saved?.run_id !== run.id) return exchange(run, context, null, 'silent_renewal');
  if (saved.token !== rejectedToken && saved.claims.exp * 1000 - Date.now() > 90000) return saved;
  if (!saved.refresh_token) return exchange(run, context, null, 'silent_renewal');
  try {
    return await exchange(run, context, saved, 'refresh_token');
  } catch (error) {
    if (error.code !== 'refresh_rejected') throw error;
    // Ask the provider once whether the existing browser session is authorized.
    // The replacement must pass the same principal, practice and audience checks.
    return exchange(run, context, null, 'silent_reconnect');
  }
}
function endpoint(job) {
  if (job.kind === 'roster') return 'https://api.hint.com/api/provider/patients?sort=last_name&offset=0&limit=25';
  if (job.kind !== 'section' || !PATIENTS.has(job.patient_id) || !SECTIONS.includes(job.section)) {
    throw fault('job_refused');
  }
  const id = job.patient_id;
  if (job.section === 'patient') return `https://api.hint.com/api/provider/patients/${id}`;
  if (job.section === 'notes') return `https://api.one.hint.com/notes/v2/notes?patientId=${id}&offset=0&limit=100&excludeFormattedText=true`;
  if (job.section === 'tasks') return `https://api.one.hint.com/reminders/v1/tasks?limit=10&sort=dueOn&order=asc&status=TO_DO,IN_PROGRESS,BLOCKED&patientId=${id}`;
  return `https://api.one.hint.com/patients/v1/patients/${id}/${job.section}`;
}
async function acquire(job, access, run) {
  const url = endpoint(job);
  active(run);
  const started = Date.now();
  let response;
  try {response = await request(url, {method: 'GET', credentials: 'omit',
    headers: {Accept: 'application/json', Authorization: 'Bearer ' + access.token}});}
  catch (_) {throw fault('read_transport_failed');}
  if (response.status !== 200) throw fault(response.status === 401 ? 'chart_unauthorized' : response.status === 403 ? 'access_denied' :
    response.status === 429 ? 'rate_limited' : 'read_failed', response.status);
  const text = await response.text();
  if (new TextEncoder().encode(text).length > 2000000) throw fault('response_too_large');
  let body;
  try {body = JSON.parse(text);} catch (_) {throw fault('read_non_json');}
  if (job.kind === 'roster' && (!Array.isArray(body) || body.length >= 25 ||
      body.some(p => !PATIENTS.has(p.id)) || new Set(body.map(p => p.id)).size !== body.length)) {
    throw fault('roster_scope_changed');
  }
  if (!body || typeof body !== 'object') throw fault('read_schema_changed');
  if (job.section === 'patient' && body.id !== job.patient_id) throw fault('patient_mismatch');
  if (job.section && job.section !== 'patient' && (!Number.isFinite(body.total) ||
      !Array.isArray(job.section === 'notes' ? body.data : body.entities))) throw fault('read_schema_changed');
  if (job.section === 'diagnoses' && (!Array.isArray(body.entities) ||
      body.entities.some(d => d.patientId !== job.patient_id))) throw fault('patient_mismatch');
  const digest = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(text));
  return {json_text: text, receipt: {started, ended: Date.now(), http_status: response.status,
    bytes: new TextEncoder().encode(text).length,
    sha256: [...new Uint8Array(digest)].map(b => b.toString(16).padStart(2, '0')).join(''),
    ...metadata(access.claims)}};
}
async function flush() {
  const outbox = await stored('outbox');
  if (!outbox) return;
  const result = await local('/complete', outbox);
  if (!['accepted', 'duplicate', 'stale'].includes(result.outcome)) {
    throw fault('bridge_refused', null, `outcome ${JSON.stringify(result.outcome)}`);
  }
  await chrome.storage.local.remove('outbox');
}
async function tick() {
  const run = await stored('run');
  if (!run || run.stopped) return;
  let job = null;
  try {
    active(run);
    // Flush a completed read BEFORE requesting a lease after worker/bridge restart.
    await flush();
    const events = (await stored('auth_events')) || [];
    const portalTabs = await chrome.tabs.query({url: 'https://app.hint.com/*'});
    await local('/heartbeat', {run_id: run.id, worker: WORKER, until: run.until,
      portal_tab_count: portalTabs.length, auth_events: events});
    await chrome.storage.local.set({auth_events: []});
    const context = await stored('context');
    let access = await tokenFor(run, context);
    const claimed = await local('/claim', {run_id: run.id, worker: WORKER});
    job = claimed.job;
    if (job) {
      let result;
      try {
        result = await acquire(job, access, run);
      } catch (error) {
        if (error.code !== 'chart_unauthorized') throw error;
        // A chart server can reject an unexpired token. Reuse the same renewal
        // owner and replay only this leased GET, once, under a different token.
        await recordAuth({kind: 'chart_401', http_status: 401, job_id: job.id, ...metadata(access.claims)});
        const renewed = await tokenFor(run, context, access.token);
        if (renewed.token === access.token) throw error;
        access = renewed;
        result = await acquire(job, access, run);
      }
      await chrome.storage.local.set({outbox: {run_id: run.id, worker: WORKER,
        job_id: job.id, lease: job.lease, result}});
      await flush();
    }
    await setState('connected', {...metadata(access.claims), portal_tab_count: portalTabs.length});
  } catch (error) {
    const code = error.code || 'connector_failed';
    if (code === 'bridge_offline') {await setState(code, {cause: error.cause}); return;}
    await chrome.storage.session.remove('access');
    await chrome.storage.local.set({run: {...run, stopped: true}});
    await chrome.alarms.clear(ALARM);
    await setState(code, {http_status: error.status || null, cause: error.cause ?? null});
    // A fixed enum, no raw auth response/exception/body/headers enters diagnostics.
    try {
      await local('/halt', {run_id: run.id, worker: WORKER, reason: code,
        http_status: error.status || null, job_id: job?.id || null,
        auth_events: (await stored('auth_events')) || []});
      await chrome.storage.local.set({auth_events: []});
    } catch (_) {}
  }
}
const WORKER = crypto.randomUUID();
async function schedule() {
  const run = await stored('run');
  const alarm = await chrome.alarms.get(ALARM);
  if (run && !run.stopped && !alarm) {
    await chrome.alarms.create(ALARM, {periodInMinutes: 0.5});
  }
}
async function prepare() {
  const old = await stored('run');
  if (old && !old.stopped && Date.now() < old.until) throw fault('stop_before_prepare');
  const tabs = await chrome.tabs.query({url: 'https://app.hint.com/*'});
  if (tabs.length !== 1) throw fault('open_exactly_one_sandbox_tab');
  const replies = await chrome.scripting.executeScript({target: {tabId: tabs[0].id},
    world: 'MAIN', files: ['bootstrap.js']});
  const value = replies[0]?.result;
  if (!value?.ok) throw fault(value?.reason || 'context_unavailable');
  await chrome.storage.session.remove('access');
  await chrome.storage.local.set({context: value.context});
  await setState('prepared', {original_exp: value.context.original_exp});
}
async function start() {
  const context = await stored('context');
  const old = await stored('run');
  if (!context || context.practice !== 'Coda Health Sandbox Practice' ||
      Date.now() - context.prepared_at > 10 * 60000) throw fault('prepare_again');
  if (old && !old.stopped && Date.now() < old.until) throw fault('already_running');
  // No run/reset can discard an undelivered successful read: deliver it first, or refuse.
  try {await flush();} catch (error) {
    // Why the delivery failed: the bridge's fault and its cause.
    const cause = error.code ? [error.code, error.cause].filter(Boolean).join(': ') : String(error);
    throw fault('pending_delivery', error.status ?? null, cause);
  }
  const started = Date.now();
  const run = {id: crypto.randomUUID(), started, until: started + RUN_MS, stopped: false};
  await chrome.storage.local.set({run, auth_events: []});
  await chrome.storage.session.remove('access');
  await schedule();
  await tick();
}
async function connect() {
  await prepare();
  await start();
}
async function stop() {
  const run = await stored('run');
  if (run) await chrome.storage.local.set({run: {...run, stopped: true}});
  await chrome.storage.session.remove('access');
  await chrome.alarms.clear(ALARM);
  await setState('stopped');
  if (run) {
    try {await local('/halt', {run_id: run.id, worker: WORKER, reason: 'stopped', http_status: null, job_id: null});}
    catch (_) {}
  }
}
// Where the physician's Hint stands right now: open, signed in, and on the practice Coda reads.
// Read fresh from the Hint tab each time the popup asks, through the same page check Connect
// uses, and kept nowhere. Empty when it can't tell, such as with several Hint tabs open.
async function readiness() {
  const tabs = await chrome.tabs.query({url: ['https://app.hint.com/*', 'https://accounts.hint.com/*']});
  if (tabs.length === 0) return {readiness: 'hint_closed'};
  if (tabs.length > 1) return {};
  // Hint's sign-in page lives on its accounts host.
  if (!tabs[0].url?.startsWith('https://app.hint.com/')) return {readiness: 'signed_out'};
  try {
    const [page] = await chrome.scripting.executeScript({target: {tabId: tabs[0].id},
      world: 'MAIN', files: ['bootstrap.js']});
    const value = page?.result;
    if (value?.ok) return {practice: value.context.practice, readiness: 'ready'};
    if (value?.reason === 'sign_in_required') return {readiness: 'signed_out'};
    if (value?.reason === 'wrong_sandbox') return {readiness: 'other_practice'};
  } catch (_) { /* A page mid-navigation can't be read. */ }
  return {};
}
async function status() {
  const run = await stored('run');
  return {version: VERSION, state: await stored('state') || {state: 'not_prepared'},
    run: run ? {id: run.id, started: run.started, until: run.until, stopped: run.stopped} : null,
    pending_delivery: Boolean(await stored('outbox')),
    practice: (await stored('context'))?.practice ?? null, ...await readiness()};
}
chrome.alarms.onAlarm.addListener(alarm => {
  if (alarm.name === ALARM) void serial(tick);
});
chrome.runtime.onStartup.addListener(() => {void serial(async () => {await schedule(); await tick();});});
chrome.runtime.onMessage.addListener((message, sender, reply) => {
  if (sender.id !== chrome.runtime.id || sender.url !== chrome.runtime.getURL('options.html')) return false;
  const actions = {prepare, start, connect, stop, status};
  if (!message || Object.keys(message).length !== 1 || !Object.hasOwn(actions, message.action)) return false;
  if (message.action === 'stop') controller?.abort();
  serial(async () => {
    try {await actions[message.action](); reply({ok: true, ...await status()});}
    catch (error) {reply({ok: false, reason: error.code || 'connector_failed', cause: error.cause ?? null, ...await status()});}
  });
  return true;
});
// Recreate alarms on every worker start. No EHR request just because it wakes.
void serial(schedule);

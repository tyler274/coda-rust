// Runs the shipped connector and Hint reader unchanged in a VM with simulated
// Chrome APIs, plus a synthetic second EHR reader, against real local HTTP and
// a real Chords bridge + MCP stdio process.
import assert from 'node:assert/strict';
import {createServer} from 'node:http';
import {spawn} from 'node:child_process';
import {createContext, runInContext} from 'node:vm';
import {readFileSync, writeFileSync, mkdtempSync, existsSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join, dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {webcrypto, createHash, randomUUID} from 'node:crypto';
import {createInterface} from 'node:readline';

const output = dirname(fileURLToPath(import.meta.url));
const root = output;
const chromeDir = join(root, '../chrome');
const identity = JSON.parse(readFileSync(join(root, 'extension-identity.json'), 'utf8'));
const work = mkdtempSync(join(tmpdir(), 'coda-connector-test-'));
const localFile = join(work, 'browser-local.json');
const checkpoint = join(work, 'checkpoint.json');

const connectorFiles = [
  'manifest.json',
  'connector.js',
  'worker.js',
  'options.html',
  'popup.js',
];

const nativeFetch = globalThis.fetch;
let now = Date.now();
let authCount = 0;
let exchanges = 0;
let acmeAuthCount = 0;
let reads = [];
let acmeReads = [];
let authMode = 'ok';
let readMode = 'ok';
let pathOverrides = new Map();
let authSuccessStatus = 201;
let loseCompletion = false;
let refuseBridge = false;
let portalCount = 0;
let acmePortalCount = 0;
let hangingRead = null;
let hangingResponse = null;
let hintPage = 'ready';
let practiceName = 'Northwind Direct Primary Care';
let cookies = 'fixture-browser-cookie';
let cookieAccepted = true;
let expectedRefresh = null;
let allowPermissionPrompts = true;
const grantedOrigins = new Set();
const permissionRequests = [];
let tabMutations = 0;
let dynamicRosterIds = Array.from({length: 105}, (_, idx) => `pat-${String(idx + 1).padStart(3, '0')}`);

let bridge;
let vm;

const CANARY = 'FIXTURE_REFRESH_SECRET_MUST_NOT_LEAVE_BROWSER';
const claims = () => ({
  iat: Math.floor(now / 1000),
  exp: Math.floor(now / 1000) + (authMode === 'short_lived' ? 60 : 900),
  tenantId: authMode === 'wrong_tenant' ? 'wrong-tenant' : 'fixture-tenant',
  sub: 'fixture-user',
  iss: 'fixture-idp',
  aud: 'fixture-client',
});
const jwt = () => 'fixture.' + Buffer.from(JSON.stringify(claims())).toString('base64url') + '.fixture';

const report = {
  scope: 'VM Chrome API fixture + real loopback HTTP + real Chords bridge & MCP stdio; no live Hint or real Chrome',
  cases: [],
};
report.tested_connector_sha256 = createHash('sha256')
  .update(readFileSync(join(chromeDir, 'connector.js'), 'utf8'))
  .digest('hex');
report.test_sha256 = createHash('sha256')
  .update(readFileSync(fileURLToPath(import.meta.url)))
  .digest('hex');
report.sources = Object.fromEntries(
  [
    '../chrome/manifest.json',
    '../chrome/connector.js',
    '../chrome/worker.js',
    '../chrome/readers/index.js',
    '../chrome/readers/hint.js',
    '../chrome/readers/hint-page.js',
    '../chrome/options.html',
    '../chrome/popup.js',
    'bridge.py',
  ].map(p => [p, createHash('sha256').update(readFileSync(join(root, p))).digest('hex')]),
);

function evidence(name, detail = {}) {
  report.cases.push({name, passed: true, ...detail});
}

// 1. Audit connector files for source/host neutrality before starting servers.
const manifest = JSON.parse(readFileSync(join(chromeDir, 'manifest.json'), 'utf8'));
assert.equal(manifest.name, 'Coda', 'Extension name in manifest.json must be Coda');
assert.equal(manifest.host_permissions, undefined, 'Reader hosts must not be pre-granted in host_permissions');
assert(Array.isArray(manifest.optional_host_permissions) && manifest.optional_host_permissions.includes('https://*/*'));

const forbiddenConnectorPattern = new RegExp(
  '\\b(' +
    [
      'hint',
      'frontegg',
      'spruce',
      'epic',
      'athena',
      'cerner',
      'elation',
      'sbx-pat',
      'diagnoses',
      'diagnosis',
      'allergies',
      'allergy',
      'medications',
      'medication',
      'surgeries',
      'surgery',
      'vaccines',
      'vaccine',
      'vitals',
      'notes',
      'tasks',
      'appointments',
      'messages',
      'labs',
      'documents',
      'localhost',
    ].join('|') +
    ')\\b|127\\.0\\.0\\.1|\\.com\\b|\\.health\\b',
  'i',
);

for (const rel of connectorFiles) {
  const content = readFileSync(join(chromeDir, rel), 'utf8');
  const hit = content.match(forbiddenConnectorPattern);
  assert.equal(hit, null, `Connector file ${rel} matched forbidden token: ${hit?.[0]}`);
}
evidence('connector neutrality: connector files contain zero EHR, host, patient, or chart-part names and manifest is named Coda');

// Vendor HTTP server standing in for Hint + synthetic second EHR ("acme").
const vendor = createServer(async (req, res) => {
  let text = '';
  for await (const chunk of req) text += chunk;
  const urlObj = new URL(req.url, 'http://127.0.0.1');

  if (urlObj.pathname === '/acme/oauth/token') {
    acmeAuthCount += 1;
    res.writeHead(200, {'Content-Type': 'application/json'});
    res.end(
      JSON.stringify({
        access_token: 'acme-mem-token-' + acmeAuthCount + '-' + CANARY,
        refresh_token: 'acme-refresh-' + acmeAuthCount + '-' + CANARY,
        iat: Math.floor(now / 1000),
        exp: Math.floor(now / 1000) + 900,
      }),
    );
    return;
  }

  if (urlObj.pathname.startsWith('/acme/api/')) {
    acmeReads.push({url: req.url, at: now});
    assert.match(req.headers.authorization || '', /^Bearer acme-mem-token-/);
    res.writeHead(200, {'Content-Type': 'application/json'});
    if (urlObj.pathname === '/acme/api/patients') {
      res.end(JSON.stringify([{id: 'acme-p-1', display: 'Acme Patient One'}]));
    } else {
      res.end(
        JSON.stringify({
          patient_id: 'acme-p-1',
          summary: 'Acme Cardiology Summary',
          secret_billing_cents: 9900,
        }),
      );
    }
    return;
  }

  if (['/frontegg/oauth/authorize/silent', '/frontegg/oauth/token'].includes(urlObj.pathname)) {
    authCount += 1;
    const exchange = urlObj.pathname.endsWith('/token');
    if (exchange) {
      exchanges += 1;
      assert.equal(req.headers.cookie, undefined, 'Explicit refresh must not depend on website cookie');
      assert.deepEqual(JSON.parse(text), {grant_type: 'refresh_token', refresh_token: expectedRefresh});
      assert.match(req.headers.authorization || '', /^Bearer fixture\./);
    } else {
      assert.equal(req.headers.cookie, cookies);
      assert.deepEqual(JSON.parse(text), {tenantId: 'fixture-tenant'});
    }
    if (
      (exchange && ['stale', 'switched', 'denied', 'forbidden'].includes(authMode)) ||
      (!exchange && (authMode === 'denied' || !cookieAccepted))
    ) {
      res.writeHead(authMode === 'forbidden' ? 403 : 401, {'Content-Type': 'application/json'});
      if (authMode === 'opaque') {
        res.end(CANARY);
        return;
      }
      res.end(JSON.stringify({error: exchange ? 'invalid_grant' : 'login_required', detail: CANARY}));
      return;
    }
    res.statusCode = authSuccessStatus;
    res.setHeader('Content-Type', 'application/json');
    expectedRefresh = CANARY + '_' + authCount;
    if (!exchange && authMode === 'switched') authMode = 'wrong_tenant';
    res.end(
      JSON.stringify({
        id_token: jwt(),
        ...(authMode === 'missing_refresh' ? {} : {refresh_token: expectedRefresh}),
        expires_in: 900,
      }),
    );
    return;
  }

  reads.push({url: req.url, at: now});
  assert.match(req.headers.authorization || '', /^Bearer fixture\./);

  for (const [needle, handler] of pathOverrides.entries()) {
    if (req.url.includes(needle)) {
      const handled = handler(req, res, urlObj);
      if (handled) return;
    }
  }

  if (readMode === '401' || readMode === '401-once') {
    if (readMode === '401-once') readMode = 'ok';
    res.writeHead(401);
    res.end(CANARY);
    return;
  }
  if (readMode === '403') {
    res.writeHead(403);
    res.end(CANARY);
    return;
  }
  if (readMode === '429') {
    res.writeHead(429, {'Retry-After': '0'});
    res.end(CANARY);
    return;
  }
  if (readMode === 'hang') {
    hangingResponse = res;
    res.writeHead(200, {'Content-Type': 'application/json'});
    res.flushHeaders();
    hangingRead?.();
    return;
  }
  if (readMode === 'html') {
    res.writeHead(200);
    res.end('<html>Sign in</html>');
    return;
  }

  let body = {total: 0, entities: []};
  if (urlObj.pathname === '/api/provider/patients') {
    const offset = Number(urlObj.searchParams.get('offset') || '0');
    const limit = Number(urlObj.searchParams.get('limit') || '100');
    body = dynamicRosterIds.slice(offset, offset + limit).map(id => ({
      id,
      first_name: 'Fictional',
      last_name: id,
      balance: 15000,
      stripeCustomerId: 'cus_should_drop',
    }));
  } else if (urlObj.pathname.startsWith('/api/provider/patients/')) {
    const pid = decodeURIComponent(urlObj.pathname.split('/').pop());
    body = {
      id: pid,
      first_name: 'Elena',
      last_name: 'Vance',
      dob: '1984-06-15',
      balance: 4200,
      stripeCustomerId: 'cus_drop_me',
      paymentMethods: [{brand: 'visa'}],
      membershipFee: 19900,
    };
  } else if (urlObj.pathname.endsWith('/diagnoses')) {
    const pid = decodeURIComponent(urlObj.pathname.split('/')[4]);
    body = {
      total: 1,
      entities: [{id: 'dx-1', patientId: pid, description: 'FICTIONAL COND-101', onsetDate: '01/2020', status: 'active'}],
    };
  } else if (urlObj.pathname.endsWith('/medications')) {
    const pid = decodeURIComponent(urlObj.pathname.split('/')[4]);
    body = {
      total: 1,
      entities: [
        {
          id: 'med-rx-1',
          patientId: pid,
          name: 'Lisinopril 10 mg tablet',
          status: 'active',
          startDate: '01/2020',
          price: 18.5,
          unitPrice: 0.62,
          invoiceId: 'inv_999',
          dispensingFee: 5.0,
        },
      ],
    };
  } else if (urlObj.pathname.endsWith('/vitals')) {
    const pid = decodeURIComponent(urlObj.pathname.split('/')[4]);
    body = {
      total: 1,
      entities: [{id: 'vit-1', patientId: pid, name: 'Blood Pressure', systolic: 118, diastolic: 74, measuredAt: '2026-10-01T14:00:00Z'}],
    };
  } else if (urlObj.pathname === '/notes/v2/notes') {
    const pid = urlObj.searchParams.get('patientId');
    const offset = Number(urlObj.searchParams.get('offset') || '0');
    const limit = Number(urlObj.searchParams.get('limit') || '100');
    const allNotes = [
      {id: 'note-1', patientId: pid, title: 'Annual Visit', noteType: 'encounter', createdAt: '2026-10-01T15:00:00Z'},
      {id: 'note-2', patientId: pid, title: 'In-House Rx Dispense', noteType: 'in_house_rx', createdAt: '2026-10-02T10:00:00Z', invoiceNumber: 'INV-12', amountCents: 2400},
    ];
    body = {total: allNotes.length, data: allNotes.slice(offset, offset + limit), limit, offset};
  } else if (urlObj.pathname.startsWith('/notes/v2/notes/')) {
    const noteId = decodeURIComponent(urlObj.pathname.split('/').pop());
    const pid = urlObj.searchParams.get('patientId');
    body = {
      id: noteId,
      patientId: pid,
      title: noteId === 'note-1' ? 'Annual Visit' : 'In-House Rx Dispense',
      body: `Full chart note body for ${noteId}. Patient reports stable home BP since 01/2020.`,
      prescriptions: noteId === 'note-2' ? [{drugName: 'Metformin 500 mg', price: 12.0, invoiceId: 'inv_drop'}] : [],
    };
  } else if (urlObj.pathname === '/reminders/v1/tasks') {
    const pid = urlObj.searchParams.get('patientId');
    body = {
      total: 2,
      entities: [
        {id: 'task-open-1', patientId: pid, title: 'Follow up BMP', status: 'TO_DO', dueOn: '2026-11-20'},
        {id: 'task-done-2', patientId: pid, title: 'Prior records requested', status: 'COMPLETED', dueOn: '2026-09-01'},
      ],
    };
  } else if (urlObj.pathname === '/api/provider/appointments') {
    const pid = urlObj.searchParams.get('patient_id');
    body = [
      {id: 'appt-1', patient_id: pid, title: 'Follow-up Visit', status: 'confirmed', starts_at: '2026-11-15T16:00:00Z'},
    ];
  } else if (urlObj.pathname === '/communications/v1/messages') {
    const pid = urlObj.searchParams.get('patientId');
    body = {
      total: 1,
      data: [{id: 'msg-1', patientId: pid, subject: 'Lab follow-up', body: 'Your potassium is normal.', sentAt: '2026-10-03T12:00:00Z'}],
    };
  } else if (urlObj.pathname === '/files/v1/files') {
    const pid = urlObj.searchParams.get('patientId');
    body = {
      total: 1,
      data: [{id: 'file-1', patientId: pid, filename: 'outside-cardiology.pdf', uploadedAt: '2026-09-28T09:00:00Z'}],
    };
  }

  res.setHeader('Content-Type', 'application/json');
  res.end(JSON.stringify(body));
});

await new Promise(resolve => vendor.listen(0, '127.0.0.1', resolve));
const vendorPort = vendor.address().port;

async function startBridge() {
  bridge = spawn('python3', [join(root, 'bridge.py'), '--serve', '--port', '0', '--checkpoint', checkpoint], {
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  const lines = createInterface({input: bridge.stdout});
  const first = await new Promise((resolve, reject) => {
    lines.once('line', resolve);
    bridge.once('error', reject);
    bridge.once('exit', code => reject(Error('Bridge exited ' + code)));
  });
  return JSON.parse(first).port;
}
let bridgePort = await startBridge();
const chordsEndpoint = () => `http://127.0.0.1:${bridgePort}`;

async function stopBridge() {
  const proc = bridge;
  await new Promise(resolve => {
    proc.once('exit', resolve);
    proc.kill('SIGTERM');
  });
}

async function rpc(name, args = {}) {
  const response = await nativeFetch(`http://127.0.0.1:${bridgePort}/rpc`, {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({name, arguments: args}),
  });
  assert.equal(response.status, 200, `RPC ${name} failed`);
  return response.json();
}

function area(durable, map = {}) {
  return {
    map,
    async get(key) {
      return key === null ? structuredClone(map) : {[key]: structuredClone(map[key])};
    },
    async set(value) {
      Object.assign(map, structuredClone(value));
      if (durable) writeFileSync(localFile, JSON.stringify(map));
    },
    async remove(key) {
      delete map[key];
      if (durable) writeFileSync(localFile, JSON.stringify(map));
    },
  };
}

const session = area(false);
const syncArea = area(false);

// Synthetic second EHR reader ("acme") run through the unchanged connector loop.
function buildAcmeReader() {
  return {
    id: 'acme',
    label: 'Acme EHR',
    entryUrl: 'https://portal.acme-ehr.test/',
    hosts: ['https://portal.acme-ehr.test/*', 'https://api.acme-ehr.test/*'],
    appHosts: ['https://portal.acme-ehr.test/*'],
    pageHosts: ['https://portal.acme-ehr.test/*'],
    reads: {
      roster: {
        scope: 'practice',
        paged: false,
        buildUrl() {
          return 'https://api.acme-ehr.test/acme/api/patients';
        },
        validateShape(body) {
          if (!Array.isArray(body) || body.some(p => typeof p?.id !== 'string')) {
            return {status: 'changed', reason: 'read_schema_changed'};
          }
          return {status: 'complete', hasMore: false, nextOffset: 0};
        },
      },
      chart_summary: {
        scope: 'patient',
        paged: false,
        dropFields: new Set(['secret_billing_cents']),
        buildUrl(job) {
          return `https://api.acme-ehr.test/acme/api/patients/${encodeURIComponent(job.patient_id)}/summary`;
        },
        validateShape(body, job) {
          if (!body || typeof body !== 'object' || body.patient_id !== job.patient_id) {
            return {status: 'changed', reason: 'read_schema_changed'};
          }
          return {status: 'complete', hasMore: false, nextOffset: 0};
        },
      },
    },
    async prepareContext({tabsQuery, fault}) {
      const tabs = await tabsQuery({url: ['https://portal.acme-ehr.test/*']});
      if (tabs.length !== 1) throw fault('open_one_source_tab');
      return {practice: 'Acme Downtown Clinic', principal: 'dr-acme'};
    },
    async checkReadiness({tabsQuery}) {
      const tabs = await tabsQuery({url: ['https://portal.acme-ehr.test/*']});
      return tabs.length === 1 ? {readiness: 'ready', practice: 'Acme Downtown Clinic'} : {readiness: 'closed'};
    },
    async checkPassiveSession({tabsQuery}) {
      const tabs = await tabsQuery({url: ['https://portal.acme-ehr.test/*']});
      return {state: tabs.length === 1 ? 'ready' : 'waiting'};
    },
    async exchangeToken({kind, httpRequest, recordAuth, fault}) {
      const res = await httpRequest('https://api.acme-ehr.test/acme/oauth/token', {method: 'POST'});
      if (!res.ok) throw fault('auth_failed', res.status);
      const data = await res.json();
      await recordAuth({kind, http_status: res.status, token_iat: data.iat, token_exp: data.exp});
      return {
        token: data.access_token,
        refresh_token: data.refresh_token,
        claims: {iat: data.iat, exp: data.exp},
      };
    },
  };
}

async function createWorker({clearSession = false, includeAcme = false} = {}) {
  if (clearSession) {
    for (const key of Object.keys(session.map)) delete session.map[key];
  }
  const storage = {
    local: area(true, existsSync(localFile) ? JSON.parse(readFileSync(localFile, 'utf8')) : {}),
    session,
    sync: syncArea,
  };
  const listeners = {};
  const alarms = new Map();
  class Clock extends Date {
    static now() {
      return now;
    }
  }
  const chrome = {
    storage,
    permissions: {
      async contains({origins}) {
        return origins.every(o => grantedOrigins.has(o));
      },
      async request({origins}) {
        permissionRequests.push([...origins]);
        if (!allowPermissionPrompts) return false;
        for (const o of origins) grantedOrigins.add(o);
        return true;
      },
    },
    runtime: {
      id: identity.id,
      getManifest: () => JSON.parse(readFileSync(join(chromeDir, 'manifest.json'), 'utf8')),
      getURL: p => identity.origin + '/' + p,
      onStartup: {addListener: fn => { listeners.startup = fn; }},
      onMessage: {addListener: fn => { listeners.message = fn; }},
    },
    alarms: {
      async get(name) { return alarms.get(name); },
      async create(name, options) { alarms.set(name, options); },
      async clear(name) { return alarms.delete(name); },
      onAlarm: {addListener: fn => { listeners.alarm = fn; }},
    },
    tabs: {
      async query({url}) {
        const patterns = Array.isArray(url) ? url : [url];
        if (patterns.some(p => p?.includes('acme-ehr.test'))) {
          return Array.from({length: acmePortalCount}, (_, id) => ({id, url: 'https://portal.acme-ehr.test/home'}));
        }
        const tabUrl = hintPage === 'sign_in_page'
          ? 'https://accounts.hint.com/oauth/account/login'
          : 'https://app.hint.com/patients';
        return Array.from({length: portalCount}, (_, id) => ({id, url: tabUrl}));
      },
      async create() { tabMutations += 1; throw new Error('Connector must never open a tab'); },
      async update() { tabMutations += 1; throw new Error('Connector must never focus a tab'); },
    },
    windows: {
      async update() { tabMutations += 1; throw new Error('Connector must never focus a window'); },
    },
    scripting: {
      async executeScript({world, files}) {
        assert.equal(world, 'MAIN');
        assert.equal(files[0], 'readers/hint-page.js');
        const script = readFileSync(join(chromeDir, files[0]), 'utf8');
        const signedIn = hintPage !== 'signed_out';
        const onPractice = hintPage !== 'other_practice';
        const label = `Switch organization, currently ${practiceName}`;
        const page = createContext({
          location: {origin: 'https://app.hint.com'},
          document: {
            body: {innerText: onPractice ? practiceName : 'Unrecognized'},
            querySelector: () => (onPractice ? {getAttribute: () => label, ariaLabel: label} : null),
          },
          window: {
            hintSession: {getSnapshot: () => ({status: signedIn ? 'active' : 'expired', token: signedIn ? jwt() : null})},
            fronteggApp: {
              store: {
                getState: () => ({
                  root: {context: {baseUrl: 'https://accounts.hint.com'}},
                  auth: {
                    isAuthenticated: signedIn,
                    tenantsState: {activeTenant: {tenantId: 'fixture-tenant', name: onPractice ? practiceName : null}},
                  },
                }),
              },
            },
          },
          Date: Clock,
          atob,
        });
        return [{result: runInContext(script, page)}];
      },
    },
  };

  const sandbox = createContext({
    chrome,
    Date: Clock,
    crypto: webcrypto,
    setTimeout,
    clearTimeout,
    AbortController,
    TextEncoder,
    TextDecoder,
    URL,
    atob,
    fetch: async (url, options = {}) => {
      const parsed = new URL(url);
      if (parsed.hostname === '127.0.0.1') {
        if (refuseBridge) return new Response('{"error": "Route or origin refused"}', {status: 403});
        const response = await nativeFetch(`http://127.0.0.1:${bridgePort}${parsed.pathname}`, {
          ...options,
          headers: {...options.headers, Origin: identity.origin},
        });
        if (parsed.pathname === '/complete' && loseCompletion) {
          loseCompletion = false;
          throw Error('Fixture response loss');
        }
        return response;
      }
      assert(
        [
          'accounts.hint.com',
          'api.hint.com',
          'api.one.hint.com',
          'api.acme-ehr.test',
        ].includes(parsed.hostname),
        `Unexpected outbound hostname ${parsed.hostname}`,
      );
      assert.equal(options.redirect, 'error');
      const headers = {...options.headers};
      if (options.credentials === 'include') headers.Cookie = cookies;
      options.signal?.addEventListener('abort', () => hangingResponse?.destroy(), {once: true});
      return nativeFetch(`http://127.0.0.1:${vendorPort}${parsed.pathname}${parsed.search}`, {...options, headers});
    },
  });
  sandbox.self = sandbox;
  sandbox.importScripts = (...files) => {
    for (const file of files) {
      runInContext(readFileSync(join(chromeDir, file), 'utf8'), sandbox, {filename: file});
    }
  };
  if (includeAcme) {
    sandbox.importScripts('connector.js', 'readers/hint.js');
    sandbox.CodaConnector.startConnector({readers: [sandbox.CodaHintReader, buildAcmeReader()]});
  } else {
    runInContext(readFileSync(join(chromeDir, 'worker.js'), 'utf8'), sandbox, {filename: 'worker.js'});
  }
  vm = {storage, listeners, alarms};
  return vm;
}

async function sendMsg(payload) {
  const wire = typeof payload === 'string' ? {action: payload} : payload;
  return new Promise(resolve => {
    const kept = vm.listeners.message(
      wire,
      {id: identity.id, url: identity.origin + '/options.html'},
      resolve,
    );
    assert.equal(kept, true, `Worker rejected message ${JSON.stringify(wire)}`);
  });
}

async function tick() {
  vm.listeners.alarm({name: 'coda-connector-tick'});
  return sendMsg('status');
}

async function resetConnection() {
  await sendMsg('stop');
  if (vm.storage.local.map.pairing) {
    vm.storage.local.map.pairing.endpoint = chordsEndpoint();
  }
  if (vm.storage.local.map.context) {
    await vm.storage.local.set({context: {...vm.storage.local.map.context, prepared_at: now}});
  }
  authMode = 'ok';
  readMode = 'ok';
  pathOverrides.clear();
  cookieAccepted = true;
  hangingResponse = null;
  hintPage = 'ready';
}

try {
  await createWorker();

  // Optional host permission prompt checks on pair and connect.
  allowPermissionPrompts = false;
  const deniedPair = await sendMsg({action: 'pair', endpoint: chordsEndpoint()});
  assert.equal(deniedPair.ok, false);
  assert.equal(deniedPair.reason, 'permission_required');

  allowPermissionPrompts = true;
  const paired = await sendMsg({action: 'pair', endpoint: chordsEndpoint()});
  assert.equal(paired.ok, true);
  assert.equal(paired.paired, true);
  assert.equal(paired.permissions_granted, false, 'Hint origins must not be granted before connecting Hint');
  assert.equal(paired.readiness, undefined, 'Tab readiness stays unset before reader host permission is granted');

  portalCount = 1;
  allowPermissionPrompts = false;
  const deniedHintPerm = await sendMsg('prepare');
  assert.equal(deniedHintPerm.ok, false);
  assert.equal(deniedHintPerm.reason, 'permission_required');
  assert.deepEqual(permissionRequests.at(-1), [
    'https://app.hint.com/*',
    'https://accounts.hint.com/*',
    'https://api.hint.com/*',
    'https://api.one.hint.com/*',
  ]);

  allowPermissionPrompts = true;
  const prepared = await sendMsg('prepare');
  assert.equal(prepared.ok, true);
  assert.equal(prepared.state.state, 'prepared');
  assert.equal(prepared.practice, 'Northwind Direct Primary Care');
  assert(!JSON.stringify(vm.storage.local.map.context).includes('fixture.'));

  portalCount = 0;
  assert.equal((await sendMsg('start')).state.state, 'connected');
  evidence('host permissions & bootstrap: Chrome prompts for Chords and reader hosts on connect; page check copies no bearer token');

  // Multi-page 105-patient roster + dynamically added 106th patient + whole Hint chart + field redaction.
  const rosterReq = await rpc('request_roster');
  await tick();
  const rosterRes = await rpc('read_result', {request_id: rosterReq.request_id});
  assert.equal(rosterRes.complete, true);
  assert.equal(rosterRes.reads[0].result.body.length, 105);
  assert.equal(rosterRes.reads[0].result.receipt.pages, 2);
  assert.equal(rosterRes.reads[0].result.body[0].balance, undefined, 'Roster patient billing fields must be dropped');
  assert.equal(rosterRes.reads[0].result.body[0].stripeCustomerId, undefined);

  // Dynamically add a 106th patient to the signed-in practice and read their whole chart!
  const dynamicPatient = 'pat-dynamic-106';
  dynamicRosterIds.push(dynamicPatient);
  const rosterRefresh = await rpc('request_roster');
  await tick();
  const refreshedRoster = await rpc('read_result', {request_id: rosterRefresh.request_id});
  assert.equal(refreshedRoster.reads[0].result.body.length, 106);
  assert(refreshedRoster.reads[0].result.body.some(p => p.id === dynamicPatient));

  const chart = await rpc('request_chart', {patient_id: dynamicPatient});
  assert.equal((await rpc('request_chart', {patient_id: dynamicPatient})).request_id, chart.request_id);

  // Run 4 ticks, then verify service-worker termination & browser restart preservation!
  for (let i = 0; i < 4; i += 1) await tick();
  const authBeforeWorkerRestart = authCount;
  await createWorker();
  await tick();
  assert.equal(authCount, authBeforeWorkerRestart, 'Worker termination must preserve session memory token');
  await createWorker({clearSession: true});
  await tick();
  assert.equal(authCount, authBeforeWorkerRestart + 1, 'Browser restart uses cookie-backed silent renewal');

  // Drain remaining reads (16 base reads + 2 dynamically queued full-text `note` reads = 18 total reads!).
  for (let i = 0; i < 14; i += 1) await tick();
  const chartResult = await rpc('read_result', {request_id: chart.request_id});
  assert.equal(chartResult.complete, true);
  assert.equal(chartResult.reads.length, 18, '16 chart sections + 2 full-text note reads');
  assert(chartResult.reads.every(r => r.attempts === 1 && r.outcome === 'complete'));

  // Check field redactions on patient, medications, and full-text note Rx!
  const patientRead = chartResult.reads.find(r => r.read === 'patient').result.body;
  assert.equal(patientRead.first_name, 'Elena');
  assert.equal(patientRead.balance, undefined);
  assert.equal(patientRead.stripeCustomerId, undefined);
  assert.equal(patientRead.paymentMethods, undefined);
  assert.equal(patientRead.membershipFee, undefined);

  const medsRead = chartResult.reads.find(r => r.read === 'medications').result.body.entities[0];
  assert.equal(medsRead.name, 'Lisinopril 10 mg tablet');
  assert.equal(medsRead.startDate, '01/2020');
  assert.equal(medsRead.price, undefined);
  assert.equal(medsRead.unitPrice, undefined);
  assert.equal(medsRead.invoiceId, undefined);
  assert.equal(medsRead.dispensingFee, undefined);

  const note2Read = chartResult.reads.find(r => r.read === 'note' && r.item_id === 'note-2').result.body;
  assert(note2Read.body.includes('Full chart note body for note-2'));
  assert.equal(note2Read.prescriptions[0].drugName, 'Metformin 500 mg');
  assert.equal(note2Read.prescriptions[0].price, undefined);
  assert.equal(note2Read.prescriptions[0].invoiceId, undefined);

  const tasksRead = chartResult.reads.find(r => r.read === 'tasks').result.body.entities;
  assert.equal(tasksRead.length, 2, 'Both open and completed tasks must be read');
  evidence('practice roster & whole Hint chart: 106-patient paged roster, dynamically added patient, all 16 sections + full note bodies, billing & Rx price/invoice redaction');

  // Refuses undeclared read and any job carrying a URL with zero outbound fetches.
  const readsBeforeRefusal = reads.length;
  const authBeforeRefusal = authCount;
  const undeclaredJob = await rpc('enqueue_raw_job', {
    source: 'hint',
    wire: {source: 'hint', read: 'export_billing_ledger', patient_id: dynamicPatient},
  });
  await tick();
  assert.equal(reads.length, readsBeforeRefusal, 'Undeclared read must call nothing');
  assert.equal(authCount, authBeforeRefusal);
  const undeclaredRes = await rpc('read_result', {request_id: undeclaredJob.request_id});
  assert.equal(undeclaredRes.complete, false);
  assert.equal(undeclaredRes.reads[0].outcome, 'failed');
  assert.equal(undeclaredRes.reads[0].result.reason, 'job_refused');

  const urlFieldJob = await rpc('enqueue_raw_job', {
    source: 'hint',
    wire: {
      source: 'hint',
      read: 'patient',
      patient_id: dynamicPatient,
      url: 'https://api.hint.com/api/provider/patients/' + dynamicPatient,
    },
  });
  await tick();
  assert.equal(reads.length, readsBeforeRefusal, 'Job carrying a url field must call nothing');
  assert.equal((await rpc('read_result', {request_id: urlFieldJob.request_id})).reads[0].result.reason, 'job_refused');

  const urlInIdJob = await rpc('enqueue_raw_job', {
    source: 'hint',
    wire: {source: 'hint', read: 'patient', patient_id: 'https://evil.invalid/p'},
  });
  await tick();
  assert.equal(reads.length, readsBeforeRefusal, 'Job carrying a URL in patient_id must call nothing');
  assert.equal((await rpc('read_result', {request_id: urlInIdJob.request_id})).reads[0].result.reason, 'job_refused');
  evidence('declared reads only: undeclared read and URL-carrying jobs are refused before any fetch');

  // Response whose shape changed is reported as 'changed' and never delivered as 'complete', without stopping other reads.
  pathOverrides.set('/diagnoses', (_req, res) => {
    res.writeHead(200, {'Content-Type': 'application/json'});
    res.end(JSON.stringify({unexpected_envelope: [{code: 'E11.9'}]}));
    return true;
  });
  const changedJob = await rpc('enqueue_job', {source: 'hint', read: 'diagnoses', patient_id: dynamicPatient});
  const validFollowup = await rpc('enqueue_job', {source: 'hint', read: 'allergies', patient_id: dynamicPatient});
  await tick();
  await tick();
  const changedOutcome = await rpc('read_result', {request_id: changedJob.request_id});
  assert.equal(changedOutcome.complete, false, 'Changed response shape must never report complete');
  assert.equal(changedOutcome.reads[0].outcome, 'changed');
  assert.equal(changedOutcome.reads[0].result.reason, 'read_schema_changed');
  assert.equal((await rpc('read_result', {request_id: validFollowup.request_id})).complete, true);
  pathOverrides.clear();
  evidence('shape guard: response with changed shape is reported as changed, never complete, and subsequent reads continue');

  // Per-read failure isolation (500 + HTML), partial multi-page read, and 429 Retry-After retry.
  let rateLimitHits = 0;
  pathOverrides.set('/surgeries', (_req, res) => {
    res.writeHead(500, {'Content-Type': 'application/json'});
    res.end(JSON.stringify({error: 'upstream failure'}));
    return true;
  });
  pathOverrides.set('/allergies', (_req, res) => {
    if (rateLimitHits < 2) {
      rateLimitHits += 1;
      res.writeHead(429, {'Retry-After': '1'});
      res.end('Slow down');
      return true;
    }
    return false;
  });
  pathOverrides.set('/notes/v2/notes?', (req, res, urlObj) => {
    const offset = Number(urlObj.searchParams.get('offset') || '0');
    if (offset === 0) {
      const page1 = Array.from({length: 100}, (_, i) => ({id: `n-${i}`, patientId: dynamicPatient, title: `Note ${i}`}));
      res.writeHead(200, {'Content-Type': 'application/json'});
      res.end(JSON.stringify({total: 120, data: page1}));
      return true;
    }
    res.writeHead(503);
    res.end('Page 2 unavailable');
    return true;
  });

  const failedSingle = await rpc('enqueue_job', {source: 'hint', read: 'surgeries', patient_id: dynamicPatient});
  const rateLimitedSingle = await rpc('enqueue_job', {source: 'hint', read: 'allergies', patient_id: dynamicPatient});
  const partialPaged = await rpc('enqueue_job', {source: 'hint', read: 'notes', patient_id: dynamicPatient});
  const healthyAfterFaults = await rpc('enqueue_job', {source: 'hint', read: 'vaccines', patient_id: dynamicPatient});

  const retryBegan = performance.now();
  const stateAfter500 = await tick();
  assert.equal(stateAfter500.state.state, 'connected', 'Single 500 read must not stop the run');
  const stateAfter429 = await tick();
  const retryElapsedSec = (performance.now() - retryBegan) / 1000;
  assert.equal(stateAfter429.state.state, 'connected');
  assert.equal(rateLimitHits, 2);
  assert(retryElapsedSec >= 1.8, `429 Retry-After: 1 x2 should wait before retrying (waited ${retryElapsedSec}s)`);
  await tick();
  await tick();

  const failedRes = await rpc('read_result', {request_id: failedSingle.request_id});
  assert.equal(failedRes.complete, false);
  assert.equal(failedRes.reads[0].outcome, 'failed');
  assert.equal(failedRes.reads[0].result.reason, 'read_failed');
  assert.equal(failedRes.reads[0].result.http_status, 500);

  const rlRes = await rpc('read_result', {request_id: rateLimitedSingle.request_id});
  assert.equal(rlRes.complete, true);
  assert.equal(rlRes.reads[0].outcome, 'complete');

  const partialRes = await rpc('read_result', {request_id: partialPaged.request_id});
  assert.equal(partialRes.complete, false);
  assert.equal(partialRes.reads[0].outcome, 'partial');
  assert.equal(partialRes.reads[0].result.reason, 'read_failed');
  assert.equal(partialRes.reads[0].result.body.data.length, 100);

  assert.equal((await rpc('read_result', {request_id: healthyAfterFaults.request_id})).complete, true);
  pathOverrides.clear();
  evidence('fault isolation & backoff: single 500 failure and multi-page partial failure are recorded without stopping other reads; 429 Retry-After waits and retries');

  // Continuous operation past 4 hours (9 hours simulated) with rotating refresh tokens and bridge restart outbox recovery.
  authSuccessStatus = 200;
  cookieAccepted = false;
  const exchangesBeforeLongRun = exchanges;
  for (let hour = 1; hour <= 9; hour += 1) {
    now += 60 * 60000;
    const hourlyJob = await rpc('enqueue_job', {source: 'hint', read: 'vitals', patient_id: dynamicPatient});
    const hourlyState = await tick();
    assert.equal(hourlyState.state.state, 'connected', `Run must keep going at hour ${hour} without a 4-hour hard stop`);
    assert.equal((await rpc('read_result', {request_id: hourlyJob.request_id})).complete, true);
  }
  assert.equal(exchanges - exchangesBeforeLongRun, 9, 'Rotating refresh token renews continuously across 9 hours without cookies');

  // Lost completion + real bridge restart once-only delivery.
  const outboxJob = await rpc('enqueue_job', {source: 'hint', read: 'patient', patient_id: dynamicPatient});
  loseCompletion = true;
  await tick();
  assert(vm.storage.local.map.outbox, 'Undelivered read stays in outbox');
  const readsBeforeOutboxFlush = reads.length;
  await stopBridge();
  bridgePort = await startBridge();
  vm.storage.local.map.pairing.endpoint = chordsEndpoint();
  writeFileSync(localFile, JSON.stringify(vm.storage.local.map));
  await createWorker();
  await tick();
  assert.equal(vm.storage.local.map.outbox, undefined);
  assert.equal(reads.length, readsBeforeOutboxFlush, 'Outbox flush after bridge + worker restart must not re-fetch from EHR');
  assert.equal((await rpc('read_result', {request_id: outboxJob.request_id})).complete, true);
  evidence('continuous background run & receipts: 9-hour operation without 4-hour stop and once-only outbox delivery across bridge + worker restart');

  // Lost sign-in (401 + login_required) stops reading & clears memory token, then passively catches up when Hint signs back in without taking tab/focus.
  readMode = '401';
  cookieAccepted = false;
  authMode = 'denied';
  portalCount = 1;
  hintPage = 'signed_out';
  const catchupJob1 = await rpc('enqueue_job', {source: 'hint', read: 'vaccines', patient_id: dynamicPatient});
  const catchupJob2 = await rpc('enqueue_job', {source: 'hint', read: 'general', patient_id: dynamicPatient});

  const haltedOnSignout = await tick();
  assert.equal(haltedOnSignout.state.state, 'interaction_required');
  assert.equal(session.map.access, undefined, 'Lost sign-in must clear in-memory session token');

  const readsWhileSignedOut = reads.length;
  const authWhileSignedOut = authCount;
  await tick();
  assert.equal(reads.length, readsWhileSignedOut, 'No EHR GETs while signed out');
  assert.equal(authCount, authWhileSignedOut, 'No repeated auth calls while Hint tab remains signed out');

  // Physician signs back into the same Hint practice in their browser tab; connector catches up passively!
  hintPage = 'ready';
  cookieAccepted = true;
  authMode = 'ok';
  readMode = 'ok';
  const caughtUp1 = await tick();
  assert.equal(caughtUp1.state.state, 'connected', 'Connector passively resumes once Hint is signed back in');
  await tick();
  assert.equal((await rpc('read_result', {request_id: catchupJob1.request_id})).complete, true);
  assert.equal((await rpc('read_result', {request_id: catchupJob2.request_id})).complete, true);
  assert.equal(tabMutations, 0, 'Connector never opened or focused any tab or window');
  portalCount = 0;
  evidence('passive sign-in catch-up: unrenewable 401 halts reading and clears token; signing back into Hint passively catches up queued reads with zero tab/focus changes');

  // Chart 401 with valid refresh token replays the single GET once under the renewed token.
  now += 60000;
  readMode = '401-once';
  const readsBefore401Replay = reads.length;
  const replayJob = await rpc('enqueue_job', {source: 'hint', read: 'allergies', patient_id: dynamicPatient});
  const replayState = await tick();
  assert.equal(replayState.state.state, 'connected');
  assert.equal(reads.length, readsBefore401Replay + 2);
  assert.equal((await rpc('read_result', {request_id: replayJob.request_id})).complete, true);

  // Unrenewable 403 stops reading immediately and clears memory token.
  readMode = '403';
  await rpc('enqueue_job', {source: 'hint', read: 'allergies', patient_id: dynamicPatient});
  const denied403 = await tick();
  assert.equal(denied403.state.state, 'access_denied');
  assert.equal(session.map.access, undefined);

  // Changed identity stops before any chart read.
  await resetConnection();
  portalCount = 1;
  await sendMsg('connect');
  portalCount = 0;
  authMode = 'switched';
  now += 14 * 60000;
  const readsBeforeSwitch = reads.length;
  assert.equal((await tick()).state.state, 'auth_identity_changed');
  assert.equal(reads.length, readsBeforeSwitch);
  assert.equal(session.map.access, undefined);

  // Stop aborts an in-flight hanging body within <5s and clears memory token.
  await resetConnection();
  portalCount = 1;
  await sendMsg('connect');
  portalCount = 0;
  readMode = 'hang';
  await rpc('enqueue_job', {source: 'hint', read: 'patient', patient_id: dynamicPatient});
  const waiting = new Promise(resolve => { hangingRead = resolve; });
  vm.listeners.alarm({name: 'coda-connector-tick'});
  await waiting;
  hangingRead = null;
  const stopBegan = performance.now();
  const stopResult = await sendMsg('stop');
  const stopSeconds = (performance.now() - stopBegan) / 1000;
  assert.equal(stopResult.state.state, 'stopped');
  assert.equal(session.map.access, undefined);
  assert(stopSeconds < 5, `Stop aborted hanging body in ${stopSeconds}s`);
  readMode = 'ok';
  evidence('Auth replay, 403 access denial, tenant switch fence, and fast Stop abort verified');

  // Synthetic second reader ("acme") runs through the exact same connector loop with zero connector edits.
  await createWorker({includeAcme: true});
  acmePortalCount = 1;
  const acmeConnected = await sendMsg({action: 'connect', source: 'acme'});
  assert.equal(acmeConnected.ok, true);
  assert.equal(acmeConnected.source, 'acme');
  assert.equal(acmeConnected.practice, 'Acme Downtown Clinic');
  assert(grantedOrigins.has('https://api.acme-ehr.test/*'));
  acmePortalCount = 0;

  const acmeRoster = await rpc('request_roster', {source: 'acme'});
  await tick();
  assert.equal((await rpc('read_result', {request_id: acmeRoster.request_id})).complete, true);

  const acmeSummary = await rpc('enqueue_job', {
    source: 'acme',
    read: 'chart_summary',
    patient_id: 'acme-p-1',
  });
  await tick();
  const acmeSummaryRes = await rpc('read_result', {request_id: acmeSummary.request_id});
  assert.equal(acmeSummaryRes.complete, true);
  assert.equal(acmeSummaryRes.reads[0].result.body.summary, 'Acme Cardiology Summary');
  assert.equal(acmeSummaryRes.reads[0].result.body.secret_billing_cents, undefined, 'Second reader dropFields enforced');
  await sendMsg('stop');
  evidence('second source reader: synthetic reader (acme) runs end-to-end through unchanged connector loop');

  // Bridge origin/auth fencing + MCP stdio check.
  const evilOrigin = await nativeFetch(`http://127.0.0.1:${bridgePort}/heartbeat`, {
    method: 'POST',
    headers: {Origin: 'https://evil.invalid', 'Content-Type': 'application/json'},
    body: '{}',
  });
  assert.equal(evilOrigin.status, 403);

  const mcp = spawn('python3', [join(root, 'bridge.py'), '--port', String(bridgePort)], {
    stdio: ['pipe', 'pipe', 'pipe'],
  });
  const mcpReplies = [];
  createInterface({input: mcp.stdout}).on('line', line => mcpReplies.push(JSON.parse(line)));
  for (const request of [
    {id: 1, method: 'initialize', params: {protocolVersion: '2024-11-05'}},
    {id: 2, method: 'tools/list'},
    {id: 3, method: 'tools/call', params: {name: 'read_result', arguments: {request_id: acmeSummary.request_id}}},
  ]) {
    mcp.stdin.write(JSON.stringify({jsonrpc: '2.0', ...request}) + '\n');
  }
  mcp.stdin.end();
  await new Promise(resolve => mcp.once('exit', resolve));
  assert.equal(mcpReplies.length, 3);
  assert.equal(mcpReplies[1].result.tools.length, 4);
  assert.equal(mcpReplies[2].result.structuredContent.complete, true);
  assert(!JSON.stringify(mcpReplies).includes(CANARY));

  // Readiness checks on Hint without mutating storage.
  await createWorker();
  const snapshotBeforeReadiness = JSON.stringify(vm.storage.local.map);
  portalCount = 0;
  assert.equal((await sendMsg('status')).readiness, 'closed');
  portalCount = 1;
  hintPage = 'sign_in_page';
  assert.equal((await sendMsg('status')).readiness, 'signed_out');
  hintPage = 'signed_out';
  assert.equal((await sendMsg('status')).readiness, 'signed_out');
  hintPage = 'other_practice';
  assert.equal((await sendMsg('status')).readiness, 'other_practice');
  hintPage = 'ready';
  const readyReply = await sendMsg('status');
  assert.equal(readyReply.readiness, 'ready');
  assert.equal(readyReply.practice, 'Northwind Direct Primary Care');
  portalCount = 2;
  assert.equal((await sendMsg('status')).readiness, undefined);
  assert.equal(JSON.stringify(vm.storage.local.map), snapshotBeforeReadiness);
  portalCount = 0;

  // Storage inspection — zero sign-in tokens or secrets in local storage, sync storage, or bridge checkpoint.
  for (const text of [
    readFileSync(checkpoint, 'utf8'),
    readFileSync(localFile, 'utf8'),
    JSON.stringify(syncArea.map),
  ]) {
    assert(!text.includes(CANARY), 'Refresh secret leaked to durable storage');
    assert(!text.includes('Bearer fixture.'), 'Hint bearer token leaked to durable storage');
    assert(!text.includes('acme-mem-token-'), 'Acme bearer token leaked to durable storage');
    assert(!text.includes('acme-refresh-'), 'Acme refresh secret leaked to durable storage');
    assert(!text.includes('id_token'), 'id_token leaked to durable storage');
    assert(!text.includes('"refresh_token":'), 'refresh_token field leaked to durable storage');
  }
  evidence('memory-only credentials: zero sign-in tokens or refresh credentials in chrome.storage.local, chrome.storage.sync, or bridge checkpoint');

  report.passed = true;
} finally {
  report.auth_requests = authCount + acmeAuthCount;
  report.chart_requests = reads.length + acmeReads.length;
  report.fixture_workdir = work;
  writeFileSync(
    join(output, process.env.HINT_WORKER_SOURCE ? 'before-local-proof.json' : 'local-proof.json'),
    JSON.stringify(report, null, 2) + '\n',
  );
  await stopBridge();
  vendor.closeAllConnections();
  await new Promise(resolve => vendor.close(resolve));
}
console.log(JSON.stringify(report, null, 2));

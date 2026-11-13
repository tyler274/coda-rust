// Source-agnostic connector. Pairs with Chords, leases named jobs without URLs,
// runs them through a registered source reader, keeps session tokens in memory
// only, and delivers exact responses with SHA-256 receipts.
(() => {
  const ALARM = 'coda-connector-tick';
  const BACKOFF_ALARM = 'coda-connector-backoff';
  const ALLOWED_JOB_KEYS = new Set(['id', 'lease', 'source', 'read', 'patient_id', 'item_id']);
  const MAX_RESPONSE_BYTES = 2000000;
  const MAX_PAGES = 50;
  const MAX_RATE_LIMIT_RETRIES = 3;
  const MAX_INLINE_RETRY_MS = 5000;

  function fault(code, status = null, cause = null) {
    return Object.assign(new Error(code), {code, status, cause});
  }

  function safeIdentifier(value) {
    if (typeof value !== 'string' || value.length === 0 || value.length > 256) return false;
    if (value.includes('://') || value.startsWith('//') || value.startsWith('http:') || value.startsWith('https:')) {
      return false;
    }
    if (value.includes('/') || value.includes('\\') || value.includes('?') || value.includes('#') || value.includes('..')) {
      return false;
    }
    return true;
  }

  function parseRetryAfterMs(headerValue, nowMs) {
    if (!headerValue) return 1000;
    const trimmed = String(headerValue).trim();
    if (/^\d+$/.test(trimmed)) {
      return Math.min(30000, Math.max(0, Number(trimmed) * 1000));
    }
    const parsedDate = Date.parse(trimmed);
    if (Number.isFinite(parsedDate)) {
      return Math.min(30000, Math.max(0, parsedDate - nowMs));
    }
    return 1000;
  }

  function dropNamedFields(value, fieldsToDrop) {
    if (!fieldsToDrop || fieldsToDrop.size === 0 || value === null || typeof value !== 'object') {
      return value;
    }
    if (Array.isArray(value)) {
      return value.map(item => dropNamedFields(item, fieldsToDrop));
    }
    const cleaned = {};
    for (const [key, entry] of Object.entries(value)) {
      if (fieldsToDrop.has(key)) continue;
      cleaned[key] = dropNamedFields(entry, fieldsToDrop);
    }
    return cleaned;
  }

  async function sha256Hex(bytes) {
    const digest = await crypto.subtle.digest('SHA-256', bytes);
    return [...new Uint8Array(digest)].map(b => b.toString(16).padStart(2, '0')).join('');
  }

  function normalizeEndpoint(raw) {
    if (typeof raw !== 'string' || !raw.trim()) throw fault('invalid_endpoint');
    let parsed;
    try {
      parsed = new URL(raw.trim());
    } catch (_) {
      throw fault('invalid_endpoint');
    }
    if ((parsed.protocol !== 'https:' && parsed.protocol !== 'http:') || parsed.username || parsed.password || parsed.search || parsed.hash) {
      throw fault('invalid_endpoint');
    }
    const basePath = parsed.pathname.replace(/\/+$/, '');
    return {
      endpoint: parsed.origin + basePath,
      originPattern: parsed.origin + '/*',
    };
  }

  function validateJob(job, readerMap, expectedSource) {
    if (!job || typeof job !== 'object' || Array.isArray(job)) {
      return {valid: false, detail: 'invalid_job_envelope'};
    }
    for (const key of Object.keys(job)) {
      if (!ALLOWED_JOB_KEYS.has(key)) {
        return {valid: false, detail: 'forbidden_job_field'};
      }
    }
    if (!safeIdentifier(job.id) || !safeIdentifier(job.lease) || !safeIdentifier(job.source) || !safeIdentifier(job.read)) {
      return {valid: false, detail: 'invalid_job_identifier'};
    }
    if (expectedSource && job.source !== expectedSource) {
      return {valid: false, detail: 'source_mismatch'};
    }
    const reader = readerMap.get(job.source);
    if (!reader) {
      return {valid: false, detail: 'unknown_source'};
    }
    if (!Object.hasOwn(reader.reads, job.read)) {
      return {valid: false, detail: 'undeclared_read'};
    }
    const spec = reader.reads[job.read];
    if (spec.scope === 'practice') {
      if (job.patient_id !== null && job.patient_id !== undefined) {
        return {valid: false, detail: 'unexpected_patient_id'};
      }
    } else if (!safeIdentifier(job.patient_id)) {
      return {valid: false, detail: 'invalid_patient_id'};
    }
    if (spec.requiresItemId) {
      if (!safeIdentifier(job.item_id)) {
        return {valid: false, detail: 'invalid_item_id'};
      }
    } else if (job.item_id !== null && job.item_id !== undefined) {
      return {valid: false, detail: 'unexpected_item_id'};
    }
    return {valid: true, reader, spec};
  }

  function createConnector({readers}) {
    if (!Array.isArray(readers) || readers.length === 0) {
      throw new Error('At least one source reader is required');
    }
    const readerMap = new Map(readers.map(r => [r.id, r]));
    const defaultReader = readers[0];
    const version = chrome.runtime.getManifest().version;
    const workerId = crypto.randomUUID();
    let lane = Promise.resolve();
    let activeAbort = null;

    function serial(action) {
      const next = lane.then(action);
      lane = next.catch(() => {});
      return next;
    }

    async function stored(key) {
      return (await chrome.storage.local.get(key))[key];
    }

    async function setState(state, extra = {}) {
      await chrome.storage.local.set({state: {state, at: Date.now(), ...extra}});
    }

    async function hasOrigins(origins) {
      if (!chrome.permissions?.contains) return true;
      return Boolean(await chrome.permissions.contains({origins}));
    }

    async function ensureOrigins(origins) {
      if (await hasOrigins(origins)) return true;
      if (!chrome.permissions?.request) return false;
      try {
        return Boolean(await chrome.permissions.request({origins}));
      } catch (_) {
        return false;
      }
    }

    function sleepMs(ms) {
      if (ms <= 0) return Promise.resolve();
      return new Promise((resolve, reject) => {
        activeAbort = new AbortController();
        const signal = activeAbort.signal;
        const timer = setTimeout(() => {
          activeAbort = null;
          resolve();
        }, ms);
        signal.addEventListener('abort', () => {
          clearTimeout(timer);
          activeAbort = null;
          reject(fault('stopped'));
        }, {once: true});
      });
    }

    async function httpRequest(url, options = {}, {classifyAuthError = null} = {}) {
      activeAbort = new AbortController();
      const timer = setTimeout(() => activeAbort?.abort(), 15000);
      try {
        const response = await fetch(url, {
          ...options,
          cache: 'no-store',
          redirect: 'error',
          signal: activeAbort.signal,
        });
        const isAuthError = !response.ok && typeof classifyAuthError === 'function';
        const text = response.ok || !isAuthError ? await response.text() : '';
        const category = isAuthError ? await classifyAuthError(response) : null;
        const retryAfter = response.headers?.get ? response.headers.get('retry-after') : null;
        return {
          ok: response.ok,
          status: response.status,
          category,
          retryAfter,
          text: async () => text,
          json: async () => JSON.parse(text),
        };
      } finally {
        clearTimeout(timer);
        activeAbort = null;
      }
    }

    async function chordsCall(path, body, explicitPairing = null) {
      const pairing = explicitPairing || await stored('pairing');
      if (!pairing?.endpoint) throw fault('unpaired');
      let response;
      try {
        const headers = {'Content-Type': 'application/json'};
        if (pairing.pair_id && pairing.pair_key) {
          headers['X-Coda-Pair'] = `${pairing.pair_id}:${pairing.pair_key}`;
        }
        response = await httpRequest(pairing.endpoint + path, {
          method: 'POST',
          credentials: 'omit',
          headers,
          body: JSON.stringify(pairing.pair_id ? {pair_id: pairing.pair_id, ...body} : body),
        });
      } catch (error) {
        if (error?.code === 'stopped') throw error;
        throw fault('bridge_offline', null, String(error));
      }
      if (!response.ok) {
        throw fault(path === '/pair' ? 'pair_refused' : 'bridge_refused', response.status, await response.text());
      }
      try {
        return await response.json();
      } catch (error) {
        throw fault('bridge_refused', response.status, String(error));
      }
    }

    function ensureActive(run) {
      if (!run || run.stopped) throw fault('stopped');
    }

    function tokenClaimsMeta(claims) {
      return {token_iat: claims.iat, token_exp: claims.exp};
    }

    async function recordAuth(event) {
      const pending = (await stored('auth_events')) || [];
      await chrome.storage.local.set({auth_events: [...pending, {at: Date.now(), ...event}]});
    }

    async function sessionAccess(sourceId) {
      const all = (await chrome.storage.session.get('access')).access || {};
      return all[sourceId] || null;
    }

    async function saveSessionAccess(sourceId, value) {
      const all = (await chrome.storage.session.get('access')).access || {};
      await chrome.storage.session.set({access: {...all, [sourceId]: value}});
    }

    async function clearSessionAccess(sourceId = null) {
      if (!sourceId) {
        await chrome.storage.session.remove('access');
        return;
      }
      const all = (await chrome.storage.session.get('access')).access;
      if (!all || !Object.hasOwn(all, sourceId)) return;
      const next = {...all};
      delete next[sourceId];
      if (Object.keys(next).length === 0) {
        await chrome.storage.session.remove('access');
      } else {
        await chrome.storage.session.set({access: next});
      }
    }

    async function exchangeForReader(reader, run, context, saved, kind) {
      ensureActive(run);
      const exchanged = await reader.exchangeToken({
        context,
        saved,
        kind,
        now: Date.now(),
        httpRequest,
        recordAuth,
        fault,
      });
      ensureActive(run);
      const value = {
        run_id: run.id,
        source: reader.id,
        token: exchanged.token,
        refresh_token: exchanged.refresh_token,
        claims: {iat: exchanged.claims.iat, exp: exchanged.claims.exp},
      };
      await saveSessionAccess(reader.id, value);
      return value;
    }

    async function tokenFor(reader, run, context, rejectedToken = null) {
      ensureActive(run);
      const saved = await sessionAccess(reader.id);
      if (saved?.run_id !== run.id) {
        return exchangeForReader(reader, run, context, null, 'silent_renewal');
      }
      if (saved.token !== rejectedToken && saved.claims.exp * 1000 - Date.now() > 90000) {
        return saved;
      }
      if (!saved.refresh_token) {
        return exchangeForReader(reader, run, context, null, 'silent_renewal');
      }
      try {
        return await exchangeForReader(reader, run, context, saved, 'refresh_token');
      } catch (error) {
        if (error.code !== 'refresh_rejected') throw error;
        return exchangeForReader(reader, run, context, null, 'silent_reconnect');
      }
    }

    async function fetchSourceJson(url, access, run) {
      let attempt = 0;
      for (;;) {
        ensureActive(run);
        attempt += 1;
        let response;
        try {
          response = await httpRequest(url, {
            method: 'GET',
            credentials: 'omit',
            headers: {
              Accept: 'application/json',
              Authorization: 'Bearer ' + access.token,
            },
          });
        } catch (error) {
          if (error?.code === 'stopped') throw error;
          throw fault('read_transport_failed');
        }
        if (response.status === 429) {
          if (attempt <= MAX_RATE_LIMIT_RETRIES) {
            const waitMs = parseRetryAfterMs(response.retryAfter, Date.now());
            if (waitMs <= MAX_INLINE_RETRY_MS) {
              await sleepMs(waitMs);
              continue;
            }
            const resumeAt = Date.now() + waitMs;
            await chrome.alarms.create(BACKOFF_ALARM, {when: resumeAt});
            throw fault('rate_limit_backoff', 429, String(waitMs));
          }
          throw fault('rate_limited', 429);
        }
        if (response.status === 401) throw fault('unauthorized', 401);
        if (response.status === 403) throw fault('access_denied', 403);
        if (response.status !== 200) throw fault('read_failed', response.status);
        const text = await response.text();
        const bytes = new TextEncoder().encode(text);
        if (bytes.length > MAX_RESPONSE_BYTES) throw fault('response_too_large', 200);
        let parsed;
        try {
          parsed = JSON.parse(text);
        } catch (_) {
          throw fault('read_non_json', 200);
        }
        return {text, bytes, parsed, status: response.status, attempts: attempt};
      }
    }

    async function executeRead(reader, spec, job, access, run) {
      const started = Date.now();
      const pages = [];
      let offset = 0;
      let totalAttempts = 0;
      const pageSize = spec.pageSize || 100;

      for (let pageIndex = 0; pageIndex < MAX_PAGES; pageIndex += 1) {
        const url = spec.buildUrl(job, {offset, limit: pageSize});
        let fetched;
        try {
          fetched = await fetchSourceJson(url, access, run);
        } catch (error) {
          if (['stopped', 'unauthorized', 'access_denied', 'rate_limit_backoff'].includes(error.code)) {
            throw error;
          }
          if (pages.length > 0 && spec.combinePages) {
            const partialRaw = spec.combinePages(pages, {partial: true});
            const sanitizedPartial = dropNamedFields(partialRaw, spec.dropFields);
            const jsonText = JSON.stringify(sanitizedPartial);
            const rawBytes = new TextEncoder().encode(jsonText);
            const digest = await sha256Hex(rawBytes);
            return {
              outcome: 'partial',
              reason: error.code || 'read_failed',
              json_text: jsonText,
              sha256: digest,
              receipt: {
                started,
                ended: Date.now(),
                http_status: error.status || 200,
                bytes: rawBytes.length,
                sha256: digest,
                pages: pages.length,
                ...tokenClaimsMeta(access.claims),
              },
            };
          }
          return {
            outcome: 'failed',
            reason: error.code || 'read_failed',
            http_status: error.status || null,
            receipt: {
              started,
              ended: Date.now(),
              http_status: error.status || null,
              attempts: totalAttempts + 1,
            },
          };
        }

        totalAttempts += fetched.attempts;
        const check = spec.validateShape(fetched.parsed, job, {offset, limit: pageSize});
        if (check.status === 'changed') {
          const digest = await sha256Hex(fetched.bytes);
          return {
            outcome: 'changed',
            reason: check.reason || 'read_schema_changed',
            sha256: digest,
            receipt: {
              started,
              ended: Date.now(),
              http_status: fetched.status,
              bytes: fetched.bytes.length,
              sha256: digest,
              pages: pages.length + 1,
              ...tokenClaimsMeta(access.claims),
            },
          };
        }
        if (check.status === 'failed') {
          return {
            outcome: 'failed',
            reason: check.reason || 'read_failed',
            http_status: fetched.status,
            receipt: {
              started,
              ended: Date.now(),
              http_status: fetched.status,
              attempts: totalAttempts,
            },
          };
        }

        pages.push(fetched.parsed);
        if (!spec.paged || !check.hasMore) {
          const combined = spec.combinePages ? spec.combinePages(pages, {partial: check.status === 'partial'}) : pages[0];
          const sanitized = dropNamedFields(combined, spec.dropFields);
          const jsonText = JSON.stringify(sanitized);
          const outBytes = new TextEncoder().encode(jsonText);
          const digest = await sha256Hex(outBytes);
          const finalOutcome = check.status === 'partial' ? 'partial' : 'complete';
          return {
            outcome: finalOutcome,
            ...(finalOutcome === 'partial' ? {reason: check.reason || 'incomplete_page'} : {}),
            json_text: jsonText,
            sha256: digest,
            receipt: {
              started,
              ended: Date.now(),
              http_status: fetched.status,
              bytes: outBytes.length,
              sha256: digest,
              pages: pages.length,
              ...tokenClaimsMeta(access.claims),
            },
          };
        }
        offset = check.nextOffset;
      }

      const partialCombined = spec.combinePages ? spec.combinePages(pages, {partial: true}) : pages[0];
      const sanitized = dropNamedFields(partialCombined, spec.dropFields);
      const jsonText = JSON.stringify(sanitized);
      const outBytes = new TextEncoder().encode(jsonText);
      const digest = await sha256Hex(outBytes);
      return {
        outcome: 'partial',
        reason: 'page_limit_exceeded',
        json_text: jsonText,
        sha256: digest,
        receipt: {
          started,
          ended: Date.now(),
          http_status: 200,
          bytes: outBytes.length,
          sha256: digest,
          pages: pages.length,
          ...tokenClaimsMeta(access.claims),
        },
      };
    }

    async function flush() {
      const outbox = await stored('outbox');
      if (!outbox) return;
      const result = await chordsCall('/complete', outbox);
      if (!['accepted', 'duplicate', 'stale'].includes(result.outcome)) {
        throw fault('bridge_refused', null, `outcome ${JSON.stringify(result.outcome)}`);
      }
      await chrome.storage.local.remove('outbox');
    }

    async function stageAndFlush(run, job, result) {
      await chrome.storage.local.set({
        outbox: {
          run_id: run.id,
          worker: workerId,
          job_id: job.id,
          lease: job.lease,
          result,
        },
      });
      await flush();
    }

    function activeReader(context) {
      const sourceId = context?.source || defaultReader.id;
      return readerMap.get(sourceId) || defaultReader;
    }

    async function tryPassiveCatchUp(reader, run, context) {
      if (!(await hasOrigins(reader.hosts))) return false;
      const check = await reader.checkPassiveSession({
        context,
        tabsQuery: query => chrome.tabs.query(query),
        executeScript: details => chrome.scripting.executeScript(details),
      });
      if (check.state === 'identity_changed') {
        throw fault('auth_identity_changed');
      }
      return check.state === 'ready';
    }

    async function tick() {
      let run = await stored('run');
      if (!run || run.stopped) return;
      const context = await stored('context');
      const reader = activeReader(context);
      let job = null;

      try {
        ensureActive(run);
        if (!(await hasOrigins(reader.hosts))) {
          throw fault('permission_required');
        }
        if (run.awaiting_auth) {
          const restored = await tryPassiveCatchUp(reader, run, context);
          if (!restored) return;
          run = {...run, awaiting_auth: false};
          await chrome.storage.local.set({run});
        }

        await flush();
        const events = (await stored('auth_events')) || [];
        const openTabs = await chrome.tabs.query({url: reader.appHosts});
        await chordsCall('/heartbeat', {
          run_id: run.id,
          source: reader.id,
          practice: context?.practice || null,
          worker: workerId,
          portal_tab_count: openTabs.length,
          auth_events: events,
        });
        await chrome.storage.local.set({auth_events: []});

        let access = await tokenFor(reader, run, context);
        const claimed = await chordsCall('/claim', {
          run_id: run.id,
          source: reader.id,
          worker: workerId,
        });
        job = claimed.job;

        if (job) {
          const admission = validateJob(job, readerMap, reader.id);
          if (!admission.valid) {
            if (!safeIdentifier(job?.id) || !safeIdentifier(job?.lease)) {
              throw fault('bridge_refused', null, admission.detail);
            }
            const nowMs = Date.now();
            await stageAndFlush(run, job, {
              outcome: 'failed',
              reason: 'job_refused',
              detail: admission.detail,
              http_status: null,
              receipt: {started: nowMs, ended: nowMs, http_status: null, attempts: 0},
            });
          } else {
            let result;
            try {
              result = await executeRead(admission.reader, admission.spec, job, access, run);
            } catch (error) {
              if (error.code !== 'unauthorized') throw error;
              await recordAuth({
                kind: 'read_401',
                http_status: 401,
                job_id: job.id,
                ...tokenClaimsMeta(access.claims),
              });
              const renewed = await tokenFor(admission.reader, run, context, access.token);
              if (renewed.token === access.token) throw error;
              access = renewed;
              result = await executeRead(admission.reader, admission.spec, job, access, run);
            }
            await stageAndFlush(run, job, result);
          }
        }

        await setState('connected', {
          ...tokenClaimsMeta(access.claims),
          portal_tab_count: openTabs.length,
        });
      } catch (error) {
        const code = error.code || 'connector_failed';
        if (code === 'bridge_offline') {
          await setState('bridge_offline', {cause: error.cause});
          return;
        }
        if (code === 'rate_limit_backoff') {
          return;
        }
        await clearSessionAccess(reader.id);
        const recoverableSignOut = ['silent_auth_unavailable', 'interaction_required', 'unauthorized'].includes(code);
        if (recoverableSignOut) {
          await chrome.storage.local.set({run: {...run, awaiting_auth: true, stopped: false}});
        } else {
          await chrome.storage.local.set({run: {...run, awaiting_auth: false, stopped: true}});
          await chrome.alarms.clear(ALARM);
        }
        await setState(code, {http_status: error.status || null, cause: error.cause ?? null});
        try {
          await chordsCall('/halt', {
            run_id: run.id,
            worker: workerId,
            reason: code,
            http_status: error.status || null,
            job_id: safeIdentifier(job?.id) ? job.id : null,
            auth_events: (await stored('auth_events')) || [],
          });
          await chrome.storage.local.set({auth_events: []});
        } catch (_) {}
      }
    }

    async function schedule() {
      const run = await stored('run');
      const alarm = await chrome.alarms.get(ALARM);
      if (run && !run.stopped && !alarm) {
        await chrome.alarms.create(ALARM, {periodInMinutes: 0.5});
      }
    }

    async function pair({endpoint, code = null} = {}) {
      const old = await stored('run');
      if (old && !old.stopped && !old.awaiting_auth) throw fault('stop_before_prepare');
      const {endpoint: normalized, originPattern} = normalizeEndpoint(endpoint);
      if (!(await ensureOrigins([originPattern]))) {
        throw fault('permission_required');
      }
      const reply = await chordsCall(
        '/pair',
        {worker: workerId, version, code: code || null},
        {endpoint: normalized},
      );
      if (typeof reply?.pair_id !== 'string' || !reply.pair_id || typeof reply?.pair_key !== 'string' || !reply.pair_key) {
        throw fault('pair_refused', null, 'invalid_pair_response');
      }
      await chrome.storage.local.set({
        pairing: {
          endpoint: normalized,
          pair_id: reply.pair_id,
          pair_key: reply.pair_key,
          paired_at: Date.now(),
        },
      });
    }

    async function unpair() {
      const old = await stored('run');
      if (old && !old.stopped && !old.awaiting_auth) throw fault('stop_before_prepare');
      try {
        await flush();
      } catch (error) {
        const cause = error.code ? [error.code, error.cause].filter(Boolean).join(': ') : String(error);
        throw fault('pending_delivery', error.status ?? null, cause);
      }
      await clearSessionAccess();
      await chrome.alarms.clear(ALARM);
      await chrome.alarms.clear(BACKOFF_ALARM);
      await chrome.storage.local.remove('pairing');
      await chrome.storage.local.remove('context');
      await chrome.storage.local.remove('run');
      await chrome.storage.local.set({auth_events: []});
      await setState('not_prepared');
    }

    async function prepare({source = null} = {}) {
      const old = await stored('run');
      if (old && !old.stopped && !old.awaiting_auth) throw fault('stop_before_prepare');
      const reader = source ? readerMap.get(source) : defaultReader;
      if (!reader) throw fault('unknown_source');
      if (!(await ensureOrigins(reader.hosts))) {
        throw fault('permission_required');
      }
      const context = await reader.prepareContext({
        tabsQuery: query => chrome.tabs.query(query),
        executeScript: details => chrome.scripting.executeScript(details),
        fault,
      });
      await clearSessionAccess(reader.id);
      await chrome.storage.local.set({context: {...context, source: reader.id, prepared_at: Date.now()}});
      await setState('prepared', {original_exp: context.original_exp ?? null});
    }

    async function start() {
      const pairing = await stored('pairing');
      if (!pairing?.endpoint) throw fault('unpaired');
      const context = await stored('context');
      const old = await stored('run');
      if (!context || !context.practice || Date.now() - context.prepared_at > 10 * 60000) {
        throw fault('prepare_again');
      }
      if (old && !old.stopped && !old.awaiting_auth) throw fault('already_running');
      const reader = activeReader(context);
      if (!(await ensureOrigins(reader.hosts))) {
        throw fault('permission_required');
      }
      try {
        await flush();
      } catch (error) {
        const cause = error.code ? [error.code, error.cause].filter(Boolean).join(': ') : String(error);
        throw fault('pending_delivery', error.status ?? null, cause);
      }
      const run = {
        id: crypto.randomUUID(),
        source: reader.id,
        started: Date.now(),
        stopped: false,
        awaiting_auth: false,
      };
      await chrome.storage.local.set({run, auth_events: []});
      await clearSessionAccess(reader.id);
      await schedule();
      await tick();
    }

    async function connect({source = null} = {}) {
      const pairing = await stored('pairing');
      if (!pairing?.endpoint) throw fault('unpaired');
      await prepare({source});
      await start();
    }

    async function stop() {
      const run = await stored('run');
      if (run) {
        await chrome.storage.local.set({run: {...run, stopped: true, awaiting_auth: false}});
      }
      await clearSessionAccess();
      await chrome.alarms.clear(ALARM);
      await chrome.alarms.clear(BACKOFF_ALARM);
      await setState('stopped');
      if (run) {
        try {
          await chordsCall('/halt', {
            run_id: run.id,
            worker: workerId,
            reason: 'stopped',
            http_status: null,
            job_id: null,
          });
        } catch (_) {}
      }
    }

    async function readiness(reader) {
      if (!(await hasOrigins(reader.hosts))) {
        return {permissions_granted: false};
      }
      const checked = await reader.checkReadiness({
        tabsQuery: query => chrome.tabs.query(query),
        executeScript: details => chrome.scripting.executeScript(details),
      });
      return {permissions_granted: true, ...checked};
    }

    async function status() {
      const pairing = await stored('pairing');
      const run = await stored('run');
      const context = await stored('context');
      const reader = activeReader(context);
      const ready = await readiness(reader);
      return {
        version,
        paired: Boolean(pairing?.endpoint),
        endpoint: pairing?.endpoint ?? null,
        source: reader.id,
        source_label: reader.label,
        entry_url: reader.entryUrl,
        tab_pattern: reader.appHosts[0] || null,
        hosts: reader.hosts,
        state: (await stored('state')) || {state: 'not_prepared'},
        run: run ? {
          id: run.id,
          source: run.source,
          started: run.started,
          stopped: run.stopped,
          awaiting_auth: Boolean(run.awaiting_auth),
        } : null,
        pending_delivery: Boolean(await stored('outbox')),
        practice: ready.practice ?? context?.practice ?? null,
        permissions_granted: ready.permissions_granted,
        ...(ready.readiness ? {readiness: ready.readiness} : {}),
      };
    }

    function bind() {
      chrome.alarms.onAlarm.addListener(alarm => {
        if (alarm.name === ALARM || alarm.name === BACKOFF_ALARM) {
          void serial(tick);
        }
      });
      chrome.runtime.onStartup.addListener(() => {
        void serial(async () => {
          await schedule();
          await tick();
        });
      });
      chrome.runtime.onMessage.addListener((message, sender, reply) => {
        if (sender.id !== chrome.runtime.id || sender.url !== chrome.runtime.getURL('options.html')) {
          return false;
        }
        const actions = {pair, unpair, prepare, start, connect, stop, status};
        if (!message || typeof message !== 'object' || !Object.hasOwn(actions, message.action)) {
          return false;
        }
        const allowedKeys = {
          pair: ['action', 'endpoint', 'code'],
          unpair: ['action'],
          prepare: ['action', 'source'],
          start: ['action'],
          connect: ['action', 'source'],
          stop: ['action'],
          status: ['action'],
        }[message.action];
        if (Object.keys(message).some(k => !allowedKeys.includes(k))) {
          return false;
        }
        if (message.action === 'stop') activeAbort?.abort();
        serial(async () => {
          try {
            await actions[message.action](message);
            reply({ok: true, ...(await status())});
          } catch (error) {
            reply({
              ok: false,
              reason: error.code || 'connector_failed',
              cause: error.cause ?? null,
              ...(await status()),
            });
          }
        });
        return true;
      });
      void serial(schedule);
    }

    return {bind};
  }

  function startConnector(options) {
    const connector = createConnector(options);
    connector.bind();
    return connector;
  }

  self.CodaConnector = {createConnector, startConnector};
})();

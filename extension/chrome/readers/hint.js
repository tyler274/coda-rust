// Hint reader: declares Hint's hosts, browser session and Frontegg token
// exchange, complete named chart reads, response shape validators, and billing /
// Rx price-and-invoice field redaction.
(() => {
  const APP_ORIGIN = 'https://app.hint.com';
  const AUTH_SILENT_URL = 'https://accounts.hint.com/frontegg/oauth/authorize/silent';
  const AUTH_TOKEN_URL = 'https://accounts.hint.com/frontegg/oauth/token';
  const PROVIDER_API = 'https://api.hint.com/api/provider';
  const ONE_API = 'https://api.one.hint.com';
  const PAGE_SCRIPT = 'readers/hint-page.js';

  const HOSTS = [
    'https://app.hint.com/*',
    'https://accounts.hint.com/*',
    'https://api.hint.com/*',
    'https://api.one.hint.com/*',
  ];
  const APP_HOSTS = ['https://app.hint.com/*'];
  const PAGE_HOSTS = ['https://app.hint.com/*', 'https://accounts.hint.com/*'];

  const PATIENT_BILLING_FIELDS = new Set([
    'billing',
    'balance',
    'balances',
    'account_balance',
    'accountBalance',
    'credit_balance',
    'creditBalance',
    'payment_method',
    'payment_methods',
    'paymentMethod',
    'paymentMethods',
    'stripe_customer_id',
    'stripeCustomerId',
    'invoices',
    'billing_address',
    'billingAddress',
    'membership_fee',
    'membershipFee',
    'plan_price',
    'planPrice',
  ]);

  const RX_PRICE_INVOICE_FIELDS = new Set([
    'price',
    'unit_price',
    'unitPrice',
    'total_price',
    'totalPrice',
    'cost',
    'wholesale_price',
    'wholesalePrice',
    'retail_price',
    'retailPrice',
    'dispensing_fee',
    'dispensingFee',
    'invoice',
    'invoice_id',
    'invoiceId',
    'invoice_number',
    'invoiceNumber',
    'invoice_status',
    'invoiceStatus',
    'charge',
    'charge_id',
    'chargeId',
    'line_item_id',
    'lineItemId',
    'amount_cents',
    'amountCents',
  ]);

  async function classifyAuthError(response) {
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
      const fields = [
        body.error,
        body.errorCode,
        body.code,
        body.message,
        body.error_description,
        ...(Array.isArray(body.errors) ? body.errors : []),
      ];
      const known = new Map([
        ['Refresh token is not found', 'refresh_not_found'],
        ['ER-01095', 'session_inactive'],
        ['interaction_required', 'interaction_required'],
        ['login_required', 'login_required'],
        ['access_denied', 'access_denied'],
        ['invalid_grant', 'invalid_grant'],
      ]);
      for (const [literal, category] of known) {
        if (fields.includes(literal)) return category;
      }
      return unknown;
    } catch (_) {
      return unknown;
    } finally {
      void reader.cancel().catch(() => {});
    }
  }

  function decodeJwtClaims(token, fault) {
    try {
      return JSON.parse(atob(token.split('.')[1].replace(/-/g, '+').replace(/_/g, '/')));
    } catch (_) {
      throw fault('auth_response_invalid');
    }
  }

  function validatePatientBinding(items, patientId) {
    for (const item of items) {
      if (!item || typeof item !== 'object' || Array.isArray(item)) {
        return {status: 'changed', reason: 'read_schema_changed'};
      }
      const boundId = item.patientId ?? item.patient_id;
      if (typeof boundId === 'string' && boundId !== patientId) {
        return {status: 'failed', reason: 'patient_mismatch'};
      }
    }
    return null;
  }

  function pagedEntityCollection({pathBuilder, listKey = 'entities', dropFields = null}) {
    return {
      scope: 'patient',
      paged: true,
      pageSize: 100,
      dropFields,
      buildUrl(job, {offset, limit}) {
        return pathBuilder(job.patient_id, {offset, limit});
      },
      validateShape(body, job, {offset}) {
        if (!body || typeof body !== 'object' || Array.isArray(body) ||
            !Number.isFinite(body.total) || body.total < 0 || !Array.isArray(body[listKey])) {
          return {status: 'changed', reason: 'read_schema_changed'};
        }
        const items = body[listKey];
        const mismatch = validatePatientBinding(items, job.patient_id);
        if (mismatch) return mismatch;
        const nextOffset = offset + items.length;
        return {
          status: 'complete',
          hasMore: items.length > 0 && nextOffset < body.total,
          nextOffset,
        };
      },
      combinePages(pages, {partial}) {
        if (pages.length === 1 && !partial) return pages[0];
        const merged = [];
        let total = 0;
        for (const page of pages) {
          total = Math.max(total, page.total);
          merged.push(...page[listKey]);
        }
        return {
          ...pages[0],
          total,
          [listKey]: merged,
          ...(partial ? {partial: true} : {}),
        };
      },
    };
  }

  function healthSection(section, dropFields = null) {
    return pagedEntityCollection({
      listKey: 'entities',
      dropFields,
      pathBuilder(patientId, {offset, limit}) {
        const paging = offset > 0 ? `?offset=${offset}&limit=${limit}` : '';
        return `${ONE_API}/patients/v1/patients/${encodeURIComponent(patientId)}/${section}${paging}`;
      },
    });
  }

  const READS = {
    roster: {
      scope: 'practice',
      paged: true,
      pageSize: 100,
      dropFields: PATIENT_BILLING_FIELDS,
      buildUrl(_job, {offset, limit}) {
        return `${PROVIDER_API}/patients?sort=last_name&offset=${offset}&limit=${limit}`;
      },
      validateShape(body, _job, {offset, limit}) {
        if (!Array.isArray(body)) {
          return {status: 'changed', reason: 'read_schema_changed'};
        }
        const seen = new Set();
        for (const item of body) {
          if (!item || typeof item !== 'object' || Array.isArray(item) || typeof item.id !== 'string' || !item.id) {
            return {status: 'changed', reason: 'read_schema_changed'};
          }
          if (seen.has(item.id)) {
            return {status: 'changed', reason: 'read_schema_changed'};
          }
          seen.add(item.id);
        }
        return {
          status: 'complete',
          hasMore: body.length === limit,
          nextOffset: offset + body.length,
        };
      },
      combinePages(pages) {
        const merged = [];
        const seen = new Set();
        for (const page of pages) {
          for (const item of page) {
            if (!seen.has(item.id)) {
              seen.add(item.id);
              merged.push(item);
            }
          }
        }
        return merged;
      },
    },

    patient: {
      scope: 'patient',
      paged: false,
      dropFields: PATIENT_BILLING_FIELDS,
      buildUrl(job) {
        return `${PROVIDER_API}/patients/${encodeURIComponent(job.patient_id)}`;
      },
      validateShape(body, job) {
        if (!body || typeof body !== 'object' || Array.isArray(body) || typeof body.id !== 'string') {
          return {status: 'changed', reason: 'read_schema_changed'};
        }
        if (body.id !== job.patient_id) {
          return {status: 'failed', reason: 'patient_mismatch'};
        }
        return {status: 'complete', hasMore: false, nextOffset: 0};
      },
    },

    diagnoses: healthSection('diagnoses'),
    allergies: healthSection('allergies'),
    medications: healthSection('medications', RX_PRICE_INVOICE_FIELDS),
    'freeform-medications': healthSection('freeform-medications', RX_PRICE_INVOICE_FIELDS),
    surgeries: healthSection('surgeries'),
    'social-history': healthSection('social-history'),
    'family-history': healthSection('family-history'),
    vaccines: healthSection('vaccines'),
    general: healthSection('general'),
    vitals: healthSection('vitals'),

    notes: pagedEntityCollection({
      listKey: 'data',
      dropFields: RX_PRICE_INVOICE_FIELDS,
      pathBuilder(patientId, {offset, limit}) {
        return `${ONE_API}/notes/v2/notes?patientId=${encodeURIComponent(patientId)}&offset=${offset}&limit=${limit}&excludeFormattedText=true`;
      },
    }),

    note: {
      scope: 'patient',
      requiresItemId: true,
      paged: false,
      dropFields: RX_PRICE_INVOICE_FIELDS,
      buildUrl(job) {
        return `${ONE_API}/notes/v2/notes/${encodeURIComponent(job.item_id)}?patientId=${encodeURIComponent(job.patient_id)}`;
      },
      validateShape(body, job) {
        if (!body || typeof body !== 'object' || Array.isArray(body) || typeof body.id !== 'string') {
          return {status: 'changed', reason: 'read_schema_changed'};
        }
        if (body.id !== job.item_id) {
          return {status: 'failed', reason: 'patient_mismatch'};
        }
        const boundPatient = body.patientId ?? body.patient_id;
        if (typeof boundPatient === 'string' && boundPatient !== job.patient_id) {
          return {status: 'failed', reason: 'patient_mismatch'};
        }
        return {status: 'complete', hasMore: false, nextOffset: 0};
      },
    },

    tasks: pagedEntityCollection({
      listKey: 'entities',
      pathBuilder(patientId, {offset, limit}) {
        return `${ONE_API}/reminders/v1/tasks?patientId=${encodeURIComponent(patientId)}&offset=${offset}&limit=${limit}&sort=dueOn&order=asc`;
      },
    }),

    appointments: {
      scope: 'patient',
      paged: true,
      pageSize: 100,
      buildUrl(job, {offset, limit}) {
        return `${PROVIDER_API}/appointments?patient_id=${encodeURIComponent(job.patient_id)}&offset=${offset}&limit=${limit}`;
      },
      validateShape(body, job, {offset, limit}) {
        if (Array.isArray(body)) {
          for (const item of body) {
            if (!item || typeof item !== 'object' || Array.isArray(item) || typeof item.id !== 'string') {
              return {status: 'changed', reason: 'read_schema_changed'};
            }
          }
          const mismatch = validatePatientBinding(body, job.patient_id);
          if (mismatch) return mismatch;
          return {
            status: 'complete',
            hasMore: body.length === limit,
            nextOffset: offset + body.length,
          };
        }
        if (body && typeof body === 'object' && Number.isFinite(body.total) && body.total >= 0) {
          const items = Array.isArray(body.entities) ? body.entities : (Array.isArray(body.data) ? body.data : null);
          if (!items) return {status: 'changed', reason: 'read_schema_changed'};
          const mismatch = validatePatientBinding(items, job.patient_id);
          if (mismatch) return mismatch;
          const nextOffset = offset + items.length;
          return {
            status: 'complete',
            hasMore: items.length > 0 && nextOffset < body.total,
            nextOffset,
          };
        }
        return {status: 'changed', reason: 'read_schema_changed'};
      },
      combinePages(pages, {partial}) {
        if (Array.isArray(pages[0])) {
          const merged = [];
          for (const page of pages) merged.push(...page);
          return partial ? {partial: true, data: merged} : merged;
        }
        const key = Array.isArray(pages[0].entities) ? 'entities' : 'data';
        const merged = [];
        let total = 0;
        for (const page of pages) {
          total = Math.max(total, page.total);
          merged.push(...page[key]);
        }
        return {...pages[0], total, [key]: merged, ...(partial ? {partial: true} : {})};
      },
    },

    messages: pagedEntityCollection({
      listKey: 'data',
      pathBuilder(patientId, {offset, limit}) {
        return `${ONE_API}/communications/v1/messages?patientId=${encodeURIComponent(patientId)}&offset=${offset}&limit=${limit}`;
      },
    }),

    'patient-files': pagedEntityCollection({
      listKey: 'data',
      pathBuilder(patientId, {offset, limit}) {
        return `${ONE_API}/files/v1/files?patientId=${encodeURIComponent(patientId)}&offset=${offset}&limit=${limit}`;
      },
    }),
  };

  async function inspectOpenTab({tabsQuery, executeScript}) {
    const tabs = await tabsQuery({url: PAGE_HOSTS});
    if (tabs.length === 0) return {status: 'closed'};
    if (tabs.length > 1) return {status: 'ambiguous'};
    if (!tabs[0].url?.startsWith(APP_ORIGIN + '/')) return {status: 'signed_out'};
    try {
      const [page] = await executeScript({
        target: {tabId: tabs[0].id},
        world: 'MAIN',
        files: [PAGE_SCRIPT],
      });
      const value = page?.result;
      if (value?.ok) return {status: 'ready', context: value.context};
      if (value?.reason === 'sign_in_required') return {status: 'signed_out'};
      if (value?.reason === 'other_practice') return {status: 'other_practice'};
      return {status: 'failed', reason: value?.reason || 'context_unavailable'};
    } catch (_) {
      return {status: 'ambiguous'};
    }
  }

  const hintReader = {
    id: 'hint',
    label: 'Hint',
    entryUrl: APP_ORIGIN + '/',
    hosts: HOSTS,
    appHosts: APP_HOSTS,
    pageHosts: PAGE_HOSTS,
    reads: READS,

    async prepareContext({tabsQuery, executeScript, fault}) {
      const tabs = await tabsQuery({url: APP_HOSTS});
      if (tabs.length !== 1) throw fault('open_one_source_tab');
      const replies = await executeScript({
        target: {tabId: tabs[0].id},
        world: 'MAIN',
        files: [PAGE_SCRIPT],
      });
      const value = replies[0]?.result;
      if (!value?.ok) throw fault(value?.reason || 'context_unavailable');
      return value.context;
    },

    async checkReadiness({tabsQuery, executeScript}) {
      const inspected = await inspectOpenTab({tabsQuery, executeScript});
      if (inspected.status === 'closed') return {readiness: 'closed'};
      if (inspected.status === 'signed_out') return {readiness: 'signed_out'};
      if (inspected.status === 'other_practice') return {readiness: 'other_practice'};
      if (inspected.status === 'ready') {
        return {readiness: 'ready', practice: inspected.context.practice};
      }
      return {};
    },

    async checkPassiveSession({context, tabsQuery, executeScript}) {
      if (!context) return {state: 'waiting'};
      const tabs = await tabsQuery({url: APP_HOSTS});
      if (tabs.length !== 1) return {state: 'waiting'};
      try {
        const [page] = await executeScript({
          target: {tabId: tabs[0].id},
          world: 'MAIN',
          files: [PAGE_SCRIPT],
        });
        const value = page?.result;
        if (!value?.ok) return {state: 'waiting'};
        const live = value.context;
        if (live.tenant_id !== context.tenant_id || live.subject !== context.subject ||
            live.issuer !== context.issuer || JSON.stringify(live.audience) !== JSON.stringify(context.audience)) {
          return {state: 'identity_changed'};
        }
        return {state: 'ready'};
      } catch (_) {
        return {state: 'waiting'};
      }
    },

    async exchangeToken({context, saved, kind, now, httpRequest, recordAuth, fault}) {
      const renewing = kind === 'refresh_token';
      let response;
      try {
        response = await httpRequest(
          renewing ? AUTH_TOKEN_URL : AUTH_SILENT_URL,
          {
            method: 'POST',
            credentials: renewing ? 'omit' : 'include',
            headers: {
              'Content-Type': 'application/json',
              Accept: 'application/json',
              ...(renewing ? {Authorization: 'Bearer ' + saved.token} : {}),
            },
            body: JSON.stringify(
              renewing
                ? {grant_type: 'refresh_token', refresh_token: saved.refresh_token}
                : {tenantId: context.tenant_id},
            ),
          },
          {classifyAuthError},
        );
      } catch (_) {
        throw fault('auth_transport_failed');
      }

      if (!response.ok) {
        await recordAuth({kind, http_status: response.status, category: response.category});
        let code = 'auth_failed';
        if (response.status === 429) code = 'rate_limited';
        else if (response.status === 403 || response.category === 'access_denied') code = 'access_denied';
        else if (
          renewing &&
          (response.status === 401 ||
            (response.status === 400 && ['invalid_grant', 'refresh_not_found'].includes(response.category)))
        ) {
          code = 'refresh_rejected';
        } else if (response.category === 'login_required' || response.category === 'interaction_required') {
          code = 'interaction_required';
        } else if (response.status === 401 || response.category === 'invalid_grant') {
          code = 'silent_auth_unavailable';
        }
        throw fault(code, response.status);
      }

      let data;
      try {
        data = await response.json();
      } catch (_) {
        throw fault('auth_response_invalid');
      }
      if (typeof data.id_token !== 'string' || typeof data.refresh_token !== 'string' || !data.refresh_token) {
        throw fault('auth_response_invalid');
      }
      const claims = decodeJwtClaims(data.id_token, fault);
      if (claims.tenantId !== context.tenant_id ||
          claims.sub !== context.subject ||
          claims.iss !== context.issuer ||
          JSON.stringify(claims.aud) !== JSON.stringify(context.audience)) {
        throw fault('auth_identity_changed');
      }
      if (!Number.isFinite(claims.iat) || !Number.isFinite(claims.exp) || claims.exp * 1000 <= now + 90000) {
        throw fault('auth_response_invalid');
      }
      await recordAuth({kind, http_status: response.status, token_iat: claims.iat, token_exp: claims.exp});
      return {
        token: data.id_token,
        refresh_token: data.refresh_token,
        claims,
      };
    },
  };

  self.CodaHintReader = hintReader;
})();

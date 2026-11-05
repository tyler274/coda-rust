(() => {
  const send = window.fetch.bind(window);
  const coda = window.__coda = { held: [], news: [], sent: new Set(), signingOut: 0, streams: [], log: [] };
  addEventListener('error', (e) => coda.log.push('error ' + e.message));
  addEventListener('unhandledrejection', (e) => coda.log.push('rejection ' + e.reason));
  const bearerOf = (input, init) => {
    const headers = new Headers((init && init.headers) || (input instanceof Request ? input.headers : undefined));
    const value = headers.get('authorization');
    return value ? value.replace(/^Bearer\s+/i, '') : null;
  };
  const kindOf = (url) => {
    const path = new URL(url, location.href).pathname;
    if (path === '/auth/verify') return 'verify';
    if (path === '/auth/sign-in') return 'request';
    if (path === '/auth/sign-out') return 'sign-out';
    if (path === '/auth/whoami') return 'whoami';
    if (path === '/patient-workspaces/events') return 'stream';
    if (path === '/today' || /^\/patient-workspaces\/[^/]+$/.test(path)) return 'read';
    return null;
  };
  window.fetch = (input, init) => {
    const url = String(input instanceof Request ? input.url : input);
    const kind = kindOf(url);
    const bearer = bearerOf(input, init);
    if (kind === 'sign-out') {
      coda.signingOut++;
      return send(input, init).finally(() => { coda.signingOut--; });
    }
    if (kind === 'stream') return send(input, init).then((response) => {
      if (!response.ok || !response.body) return response;
      const reader = response.body.getReader();
      let end = () => {};
      const body = new ReadableStream({
        start(controller) { end = () => { try { controller.close(); } catch (_) {} reader.cancel().catch(() => {}); }; },
        pull(controller) {
          return reader.read().then(({ done, value }) => done ? controller.close() : controller.enqueue(value),
            (error) => controller.error(error));
        },
        cancel() { return reader.cancel(); },
      });
      coda.streams.push({ bearer, end });
      return new Response(body, { status: response.status, headers: response.headers });
    });
    if (!kind) return send(input, init);
    return new Promise((resolve, reject) => {
      const held = { kind, bearer, input, init, resolve, reject };
      coda.log.push(`hold ${kind} ${bearer}`);
      coda.held.push(held);
      // A request the page drops is no longer waited on.
      const signal = (init && init.signal) || (input instanceof Request ? input.signal : null);
      if (signal) signal.addEventListener('abort', () => {
        coda.log.push(`abort ${kind}`);
        coda.held = coda.held.filter((h) => h !== held);
        reject(new DOMException('aborted', 'AbortError'));
      });
    });
  };
  coda.release = (kind, bearer, how) => {
    // A link request or verification is answered whatever bearer it carries.
    const now = coda.held.filter((h) => h.kind === kind
      && (kind === 'request' || kind === 'verify' || h.bearer === bearer));
    coda.held = coda.held.filter((h) => !now.includes(h));
    for (const h of now) {
      coda.log.push(`release ${kind} ${bearer} ${JSON.stringify(how).slice(0, 60)}`);
      if (how.fail) h.reject(new TypeError('Failed to fetch'));
      else if (how.status) {
        const response = new Response(JSON.stringify(how.body),
          { status: how.status, headers: { 'content-type': 'application/json' } });
        // reqwest reads the answer's URL.
        Object.defineProperty(response, 'url', { value: new URL(h.input instanceof Request ? h.input.url : String(h.input), location.href).href });
        h.resolve(response);
      }
      else h.resolve(send(h.input, h.init));
    }
    return now.length;
  };
  coda.endStreams = (bearer) => {
    coda.streams.filter((s) => s.bearer === bearer).forEach((s) => s.end());
    coda.streams = coda.streams.filter((s) => s.bearer !== bearer);
  };
  const Channel = window.BroadcastChannel;
  window.BroadcastChannel = class extends Channel {
    postMessage(data) { coda.sent.add(String(data)); return super.postMessage(data); }
    set onmessage(handler) {
      this.handler = handler;
      super.onmessage = handler ? (event) => {
        if (!coda.sent.has(String(event.data))) coda.news.push(() => this.handler && this.handler(event));
      } : null;
    }
    get onmessage() { return this.handler; }
  };
  coda.hear = () => { const now = coda.news; coda.news = []; now.forEach((deliver) => deliver()); return now.length; };
})()

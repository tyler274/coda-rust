// Scripts the answers Coda gives a sign-in page, for the stages the sample server can't reach on
// its own. Each step in `sessionStorage['sign-in-script']`, or pushed to `window.__script`, answers
// one request whose address holds `path`: with `status` and `body`, or with Coda's own answer when
// it has no `status`. With `hold`, the answer waits for `window.__release()`; with `keep`, the step
// answers every such request.
(() => {
  const real = window.fetch.bind(window);
  const stored = sessionStorage.getItem('sign-in-script');
  sessionStorage.removeItem('sign-in-script');
  window.__script = stored ? JSON.parse(stored) : [];
  window.__held = [];
  window.fetch = (input, init) => {
    const url = input instanceof Request ? input.url : String(input);
    const at = window.__script.findIndex((step) => url.includes(step.path));
    if (at < 0) return real(input, init);
    const step = window.__script[at];
    if (!step.keep) window.__script.splice(at, 1);
    const scripted = () => {
      const response = new Response(JSON.stringify(step.body), {
        status: step.status,
        headers: { 'content-type': 'application/json' },
      });
      // The client reads the address it was answered from; a made-up response has none.
      Object.defineProperty(response, 'url', { value: new URL(url, location.href).href });
      return Promise.resolve(response);
    };
    const answer = () => step.status ? scripted() : real(input, init);
    if (!step.hold) return answer();
    return new Promise((resolve) => window.__held.push(() => answer().then(resolve)));
  };
  window.__release = () => {
    window.__held.splice(0).forEach((release) => release());
    return 'released';
  };
})();

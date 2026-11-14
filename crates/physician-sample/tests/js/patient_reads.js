// Stands in for a slower server and keeps a log, for the patient preload proofs. Before the page
// loads, the test puts `{ delay, answers }` in `sessionStorage['patient-reads']`. The page's two
// patient reads, `GET /patient-workspaces/{id}` and `GET /today?patient_workspace_id=`, each answer
// `delay` milliseconds late. A request whose address holds an answer's `path` gets that answer's
// `status` and `body` instead of the server's. Every request the page sends is logged in
// `window.__requests`, and the moment the physician clicks or presses Enter in `window.__opened`.
(() => {
  const real = window.fetch.bind(window);
  const setup = JSON.parse(sessionStorage.getItem('patient-reads') || '{}');
  const delay = setup.delay || 0;
  const answers = setup.answers || [];
  window.__requests = [];
  window.__opened = null;
  const patientRead = (address) =>
    /^\/patient-workspaces\/pw-[^/]+$/.test(address.pathname) ||
    (address.pathname === '/today' && address.searchParams.has('patient_workspace_id'));
  const scripted = (step, url) => {
    const response = new Response(JSON.stringify(step.body), {
      status: step.status,
      headers: { 'content-type': 'application/json' },
    });
    // The client reads the address it was answered from; a made-up response has none.
    Object.defineProperty(response, 'url', { value: url });
    return response;
  };
  window.fetch = (input, init) => {
    const url = new URL(input instanceof Request ? input.url : String(input), location.href);
    const method = (init && init.method) || (input instanceof Request ? input.method : 'GET');
    const read = patientRead(url);
    window.__requests.push({
      method,
      path: url.pathname + url.search,
      read,
      at: performance.now(),
    });
    const step = answers.find((candidate) => url.href.includes(candidate.path));
    const answer = step ? Promise.resolve(scripted(step, url.href)) : real(input, init);
    if (!read || delay <= 0) return answer;
    return new Promise((resolve) => setTimeout(resolve, delay)).then(() => answer);
  };
  const opened = (event) => {
    if (window.__opened === null) window.__opened = performance.now();
  };
  addEventListener('click', opened, true);
  addEventListener('keydown', (event) => event.key === 'Enter' && opened(event), true);
})();

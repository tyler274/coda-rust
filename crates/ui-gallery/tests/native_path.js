// Runs the UI library's native scripts as Dioxus's native channel does: each as an async function of
// `dioxus`, Rust's messages arriving later and refused once it has returned, and its reports still
// heard after that. Leaves what
// each case saw in `window.nativeSeen`, which is complete once `done` is true.
(async () => {
  const AsyncFunction = async function () {}.constructor;
  const seen = {};
  seen.sendErrors = [];
  const channel = () => {
    const reports = [];
    const queue = [];
    const waiting = [];
    let closed = false;
    return {
      reports,
      // Rust reaches a script through a lookup its return clears, so a late message throws.
      rustSend(value) {
        if (closed) throw new TypeError("Cannot read properties of null (reading 'rustSend')");
        const next = waiting.shift();
        next ? next(value) : queue.push(value);
      },
      recv: () => new Promise((resolve) => { queue.length ? resolve(queue.shift()) : waiting.push(resolve); }),
      send: (value) => { reports.push(value); },
      close() { closed = true; },
    };
  };
  // Rust's messages arrive after the script has started, each in a task of its own.
  const run = (script, toScript) => {
    const dioxus = channel();
    const finished = new AsyncFunction('dioxus', script)(dioxus).then(
      () => { dioxus.close(); return 'returned'; },
      (error) => { dioxus.close(); return `rejected: ${error}`; },
    );
    const sent = new Promise((done) => setTimeout(() => {
      try { toScript.forEach((message) => dioxus.rustSend(message)); } catch (error) { seen.sendErrors.push(String(error)); }
      done();
    }));
    return { dioxus, finished: sent.then(() => finished) };
  };
  const heard = async (dioxus) => {
    for (let tries = 0; tries < 200 && dioxus.reports.length === 0; tries += 1) await new Promise((go) => setTimeout(go, 10));
    return dioxus.reports.map((report) => JSON.parse(report));
  };
  const behaviour = (kind, name, args) => run(NATIVE, [[kind, name, JSON.stringify(args)]]);

  // Nothing loaded: a caller hears why.
  delete window.codaUi;
  seen.notLoaded = await heard(behaviour('ask', 'reducedMotion', []).dioxus);

  // A module that won't load: the load script rejects, and a caller hears why.
  delete window.codaUi;
  seen.badLoad = await run(LOAD, ['this is not javascript (']).finished;
  seen.afterBadLoad = await heard(behaviour('ask', 'reducedMotion', []).dioxus);

  // The module loads once, and a second load keeps it.
  delete window.codaUi;
  seen.load = await run(LOAD, [BEHAVIOUR]).finished;
  const module = window.codaUi;
  seen.secondLoad = await run(LOAD, [BEHAVIOUR]).finished;
  seen.sameModule = module === window.codaUi;

  // Each kind of behaviour.
  seen.ask = await heard(behaviour('ask', 'reducedMotion', []).dioxus);
  const field = document.body.appendChild(Object.assign(document.createElement('input'), { id: 'native-field' }));
  await behaviour('fire', 'focusElement', ['native-field']).finished;
  seen.fired = document.activeElement === field;
  field.remove();

  // An answer that comes after its script has returned still arrives, as a dialog's close does.
  const dialog = document.body.appendChild(Object.assign(document.createElement('dialog'), { id: 'native-dialog' }));
  const modal = behaviour('ask', 'modal', ['native-dialog', 'button']);
  seen.modalFinishedEarly = await modal.finished;
  seen.modalWaits = modal.dioxus.reports.length === 0;
  dialog.close();
  seen.modal = await heard(modal.dioxus);
  dialog.remove();

  // A listener reports until it is stopped, and then it returns.
  const page = behaviour('listen', 'watchPage', ['no-citation', 'no-preview']);
  seen.listen = (await heard(page.dioxus)).length;
  page.dioxus.rustSend(null);
  seen.stopped = await page.finished;
  const before = page.dioxus.reports.length;
  dispatchEvent(new Event('resize'));
  seen.quietAfterStop = page.dioxus.reports.length === before;

  // A behaviour that throws: a caller hears why.
  seen.threw = await heard(behaviour('ask', 'noSuchBehaviour', []).dioxus);

  window.nativeSeen = seen;
})();

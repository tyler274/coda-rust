// What the patient preload proofs do inside the page. The test calls this function with a helper's
// name, so every value arrives as an argument, never as script text.
function (name, ...args) {
  const helpers = {
    // What `patient_reads.js` does on the next page this tab loads.
    setUp(setup) {
      sessionStorage.setItem('patient-reads', JSON.stringify(setup));
    },

    // The requests logged since the last call, and forgets them.
    takeRequests() {
      return window.__requests.splice(0);
    },

    // The middle of the first element `selector` finds whose name holds `words`, in viewport
    // pixels, once it is laid out; null before then.
    middle(selector, words) {
      const found = [...document.querySelectorAll(selector)].find((element) =>
        (element.getAttribute('aria-label') || element.innerText).includes(words));
      if (!found) return null;
      const box = found.getBoundingClientRect();
      if (box.width === 0 || box.height === 0) return null;
      return [box.left + box.width / 2, box.top + box.height / 2];
    },

    // Starts timing an open: from the click or Enter that `patient_reads.js` notes, until the
    // patient page names `patient` beside "Records as of" on screen.
    timeOpen(patient) {
      window.__opened = null;
      window.__usable = null;
      const look = () => {
        if (window.__opened !== null) {
          const text = document.body.innerText;
          if (text.includes('Records as of') && text.includes(patient)) {
            window.__usable = performance.now();
            return;
          }
        }
        requestAnimationFrame(look);
      };
      requestAnimationFrame(look);
    },

    // When the open started and when its page was usable, in the log's clock.
    openTimes() {
      return { opened: window.__opened, usable: window.__usable };
    },

    // Keeps a recovery record for `workspace`, as the composer keeps unsent text, under this
    // page's backend and the signed-in physician.
    keepDraft(practice, physician, workspace) {
      const parts = JSON.stringify([location.origin, practice, physician, workspace, 'chat-kept']);
      sessionStorage.setItem(`coda.physician.draft.v1|${parts}|proof`, '{}');
    },

    draftKeys() {
      return Object.keys(sessionStorage).filter((key) => key.startsWith('coda.physician.draft.'));
    },

    text() {
      return document.body ? document.body.innerText : '';
    },

    exists(selector) {
      return Boolean(document.querySelector(selector));
    },
  };
  return helpers[name](...args);
}

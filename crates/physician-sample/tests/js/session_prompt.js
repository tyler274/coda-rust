// What the session-end tests in `session_end_privacy.rs` and `auth_browser.rs` read and do in the
// page. They call it through `Runtime.callFunctionOn`, so each step's arguments arrive as values,
// never as script text.
function (step, ...args) {
  const text = () => (document.body ? document.body.innerText : "");
  const steps = {
    text,
    // The prompt stands in place of the workspace, which is gone from the page with its patient.
    promptReplacesWorkspace() {
      const prompt = document.querySelector(".session-prompt[open]");
      const top = document.elementFromPoint(innerWidth / 2, innerHeight / 2);
      return !!prompt && prompt.contains(top) && !document.querySelector(".glass-app")
        && !text().includes("Records as of");
    },
    // The prompt is gone and the workspace shows Today, not a patient.
    backAtToday(todayWords) {
      return !document.querySelector(".session-prompt") && text().includes(todayWords)
        && !text().includes("Records as of");
    },
    // Stored keys that name a physician or hold patient text, across both storages.
    identifyingKeys(physicianId) {
      return [...Object.keys(localStorage), ...Object.keys(sessionStorage)].filter(
        (key) => key.startsWith("coda.physician.draft.") || key.includes(physicianId),
      );
    },
    storedSession() {
      return localStorage.getItem("coda.physician.session");
    },
    endSession() {
      const session = localStorage.getItem("coda.physician.session");
      return fetch("/auth/sign-out", { method: "POST", headers: { Authorization: `Bearer ${session}` } })
        .then((response) => response.status);
    },
  };
  return steps[step](...args);
}

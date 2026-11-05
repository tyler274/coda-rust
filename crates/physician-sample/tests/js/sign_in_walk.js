// What the sign-in walk does inside the page. The test calls this function with a helper's name, so
// every value arrives as an argument, never as script text.
function (name, ...args) {
  // The prompt covers the page when it is up, and its buttons are the ones to press.
  const root = () => document.querySelector('.session-prompt') || document;
  const button = (words) => [...root().querySelectorAll('button')]
    .find((candidate) => candidate.innerText.trim() === words);
  const helpers = {
    theme(theme) {
      document.documentElement.dataset.theme = theme;
    },

    // The next answer to a request, as `sign_in_states.js` describes it.
    answer(step) {
      window.__script.push(step);
    },

    // Answers for the next page this tab loads.
    answerNextLoad(steps) {
      sessionStorage.setItem('sign-in-script', JSON.stringify(steps));
    },

    forgetAnswers() {
      window.__script = [];
    },

    release() {
      return window.__release();
    },

    // Presses the enabled button with these words; false when there is none.
    press(words) {
      const found = button(words);
      if (!found || found.disabled) return false;
      found.click();
      return true;
    },

    disabled(words) {
      return button(words).disabled;
    },

    field(selector) {
      const { value, autocomplete } = document.querySelector(selector);
      return { value, autocomplete };
    },

    focus(selector) {
      document.querySelector(selector).focus();
    },

    alert() {
      const found = root().querySelector('[role=alert]');
      return found ? found.innerText : null;
    },

    promptUp() {
      return Boolean(document.querySelector('.session-prompt'));
    },

    // How far the page, or the prompt that scrolls on its own, runs past its width.
    sidewaysScroll() {
      const page = document.documentElement;
      const prompt = document.querySelector('.session-prompt');
      return Math.max(
        page.scrollWidth - page.clientWidth,
        prompt ? prompt.scrollWidth - prompt.clientWidth : 0,
      );
    },
  };
  return helpers[name](...args) ?? null;
}

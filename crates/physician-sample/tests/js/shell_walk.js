// What the shell walk does inside the page. The test calls this function with a helper's name, so
// every value arrives as an argument, never as script text.
function (name, ...args) {
  // An open dialog stands over the page, and its controls are the ones to press.
  const root = () => [...document.querySelectorAll('dialog[open]')].pop() || document;
  const button = (words) => [...root().querySelectorAll('button')].find((candidate) =>
    candidate.getAttribute('aria-label') === words || candidate.innerText.trim() === words);
  const helpers = {
    theme(theme) {
      document.documentElement.dataset.theme = theme;
    },

    // The next answer to a request, as `sign_in_states.js` describes it.
    answer(step) {
      window.__script.push(step);
    },

    forgetAnswers() {
      window.__script = [];
    },

    // Answers for the next page this tab loads.
    answerNextLoad(steps) {
      sessionStorage.setItem('sign-in-script', JSON.stringify(steps));
    },

    release() {
      return window.__release();
    },

    // Presses the enabled button with these words or this name; false when there is none.
    press(words) {
      const found = button(words);
      if (!found || found.disabled) return false;
      found.click();
      return true;
    },

    // Presses the first enabled button this selector finds; false when there is none.
    pressFirst(selector) {
      const found = root().querySelector(selector);
      if (!found || found.disabled) return false;
      found.click();
      return true;
    },

    exists(selector) {
      return Boolean(document.querySelector(selector));
    },

    reveal(selector) {
      document.querySelector(selector).scrollIntoView({ block: 'start' });
    },

    attribute(selector, attribute) {
      const found = document.querySelector(selector);
      return found ? found.getAttribute(attribute) : null;
    },

    // Chooses the picker's option with these words, as a person does.
    choose(words) {
      const select = document.querySelector('select');
      const option = [...select.options].find((candidate) => candidate.text.includes(words));
      select.value = option.value;
      select.dispatchEvent(new Event('change', { bubbles: true }));
    },

    // The words the picker shows.
    chosen() {
      const select = document.querySelector('select');
      return select.options[select.selectedIndex].text;
    },

    sidewaysScroll() {
      const scroller = document.scrollingElement;
      return scroller.scrollWidth - scroller.clientWidth;
    },
  };
  return helpers[name](...args);
}

// What the browser tests do inside the page. `Page::run` calls this function through
// `Runtime.callFunctionOn` with a helper's name, so every value arrives as an argument, never as
// script text.
function (name, ...args) {
  const helpers = {
    // Clicks the first element matching `selector`.
    click(selector) {
      const node = document.querySelector(selector);
      if (!node) return 'missing';
      node.click();
      return 'clicked';
    },

    // Types `text` into the first input or textarea matching `selector` and fires the Dioxus
    // `input` event so signal bindings update.
    typeInto(selector, text) {
      const node = document.querySelector(selector);
      if (!node) return 'missing';
      const field = node instanceof window.HTMLInputElement
        ? window.HTMLInputElement : window.HTMLTextAreaElement;
      const setter = Object.getOwnPropertyDescriptor(field.prototype, 'value').set;
      setter.call(node, text);
      node.dispatchEvent(new Event('input', { bubbles: true }));
      return 'typed';
    },

    // Whether anything matches `selector`.
    has(selector) {
      return Boolean(document.querySelector(selector));
    },

    // Clicks the button whose trimmed text is `text`; false when there is none.
    clickButton(text) {
      const button = [...document.querySelectorAll('button')].find(b => b.textContent.trim() === text);
      if (!button) return false;
      button.click();
      return true;
    },
  };
  return helpers[name](...args);
}

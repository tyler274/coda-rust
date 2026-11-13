// Browser interaction helpers for `site_source_setup.rs`.
// Every value arrives as a typed JSON argument from Rust (`call_function`), never spliced into JS text.
function (action, ...args) {
  const root = () => [...document.querySelectorAll('dialog[open]')].pop() || document;

  const helpers = {
    press(words) {
      const found = [...root().querySelectorAll('button')].find((candidate) => {
        const aria = candidate.getAttribute('aria-label') || '';
        const text = (candidate.innerText || '').trim();
        return aria === words || text === words;
      });
      if (!found || found.disabled) return false;
      found.click();
      return true;
    },

    pressSourceTab(sourceLabel) {
      const prefix = `${sourceLabel} ·`;
      const found = [...root().querySelectorAll('button')].find((candidate) => {
        const text = (candidate.innerText || '').trim();
        return text === sourceLabel || text.startsWith(prefix);
      });
      if (!found || found.disabled) return false;
      found.click();
      return true;
    },

    chooseRadio(labelWords) {
      const row = [...root().querySelectorAll('.radio-group .selection-row')].find((candidate) => {
        const strong = candidate.querySelector('strong');
        return strong && strong.innerText.trim() === labelWords;
      });
      if (!row) return false;
      const input = row.querySelector('input[type="radio"]');
      if (!input || input.disabled) return false;
      if (!input.checked) {
        input.click();
      }
      return true;
    },

    setCheckbox(labelPrefix, wantChecked) {
      const row = [...root().querySelectorAll('.selection-row')].find((candidate) => {
        const strong = candidate.querySelector('strong');
        return strong && strong.innerText.trim().startsWith(labelPrefix);
      });
      if (!row) return false;
      const input = row.querySelector('input[type="checkbox"]');
      if (!input || input.disabled) return false;
      if (Boolean(input.checked) !== Boolean(wantChecked)) {
        input.click();
      }
      return true;
    },

    fillField(fieldLabel, value) {
      const frame = [...root().querySelectorAll('.text-field')].find((candidate) => {
        const label = candidate.querySelector('.text-field-label');
        return label && label.innerText.trim() === fieldLabel;
      });
      if (!frame) return false;
      const control = frame.querySelector('input, textarea');
      if (!control || control.disabled) return false;
      const proto = control.tagName === 'TEXTAREA'
        ? window.HTMLTextAreaElement.prototype
        : window.HTMLInputElement.prototype;
      const descriptor = Object.getOwnPropertyDescriptor(proto, 'value');
      if (descriptor && descriptor.set) {
        descriptor.set.call(control, value);
      } else {
        control.value = value;
      }
      control.dispatchEvent(new Event('input', { bubbles: true }));
      control.dispatchEvent(new Event('change', { bubbles: true }));
      return true;
    },

    dialogOpen() {
      return Boolean(document.querySelector('dialog[open]'));
    },

    dialogText() {
      const dlg = [...document.querySelectorAll('dialog[open]')].pop();
      return dlg ? dlg.innerText : '';
    },

    pageText() {
      return document.body ? document.body.innerText : '';
    },

    activeButtons() {
      return [...root().querySelectorAll('button')].map((btn) => (btn.innerText || '').trim());
    },

    exists(selector) {
      return Boolean(document.querySelector(selector));
    },

    pressFirst(selector) {
      const found = document.querySelector(selector);
      if (!found || found.disabled) return false;
      found.click();
      return true;
    },

    sessionToken() {
      return window.localStorage.getItem('coda.physician.session') || '';
    },

    sidewaysScroll() {
      const scroller = document.scrollingElement;
      const pageOverflow = scroller ? scroller.scrollWidth - scroller.clientWidth : 0;
      const dlgBody = document.querySelector('dialog[open] .dialog-body');
      const dlgOverflow = dlgBody ? dlgBody.scrollWidth - dlgBody.clientWidth : 0;
      return Math.max(pageOverflow, dlgOverflow);
    },
  };

  return helpers[action](...args);
}

// What the reply journeys in `site.rs` do and read in the page. `CdpBrowser::journey` calls this
// through `Runtime.callFunctionOn`, so what each step is given arrives as arguments, never as
// script text.
function (step, ...args) {
  const one = (selector) => {
    const found = document.querySelector(selector);
    if (!found) throw new Error(`nothing matches ${selector}`);
    return found;
  };
  const steps = {
    click(selector) {
      one(selector).click();
    },
    // The first `selector` whose words are exactly `words`.
    clickNamed(selector, words) {
      const found = [...document.querySelectorAll(selector)].find(
        (element) => element.textContent.trim() === words,
      );
      if (!found) throw new Error(`no ${selector} says ${words}`);
      found.click();
    },
    // A message or draft as read: its text, what it holds and how its words are set.
    readReply(selector) {
      const host = one(selector);
      return {
        text: host.innerText,
        prose: !!host.querySelector(".prose"),
        code: !!host.querySelector(".code-block-body"),
        table: !!host.querySelector(".prose-table-scroll"),
        list: !!host.querySelector("ul li"),
        font: getComputedStyle(host.querySelector("p:has(strong)")).font,
      };
    },
    readText(selector) {
      return one(selector).innerText;
    },
    // How many lines the table header saying `words` takes.
    headerLines(selector, words) {
      const header = [...document.querySelectorAll(`${selector} th`)].find(
        (cell) => cell.textContent === words,
      );
      if (!header) throw new Error(`no ${selector} header says ${words}`);
      const range = document.createRange();
      range.selectNodeContents(header);
      return range.getClientRects().length;
    },
    hasFocus(selector) {
      return document.activeElement === one(selector);
    },
    scrollLeft(selector) {
      return one(selector).scrollLeft;
    },
    // Copies through a stand-in clipboard, and answers with what it was given.
    copied(selector) {
      return new Promise((resolve) => {
        Object.defineProperty(navigator, "clipboard", {
          value: { writeText: async (text) => resolve(text) },
        });
        one(selector).click();
      });
    },
    // A compact field as laid out and named.
    field(selector) {
      const input = one(selector);
      const host = input.parentElement;
      const label = host.querySelector(".sr-only");
      return {
        labelContained: !label || label.offsetParent === host,
        hostHeight: host.offsetHeight,
        inputHeight: input.offsetHeight,
        name: input.labels?.[0]?.textContent ?? input.getAttribute("aria-label"),
        placeholder: input.placeholder,
      };
    },
    // A field's value once a moment has passed.
    valueSoon(selector) {
      return new Promise((resolve) => {
        setTimeout(() => resolve(one(selector).value), 300);
      });
    },
    setTheme(theme) {
      document.documentElement.dataset.theme = theme;
    },
    scrollToStart(selector) {
      document.querySelector(selector)?.scrollIntoView({ block: "start" });
    },
    collapseSidebar() {
      document
        .querySelector(".glass-app:not(.has-collapsed-sidebar) button.sidebar-collapse")
        ?.click();
    },
    expandSidebar() {
      document
        .querySelector(".glass-app.has-collapsed-sidebar button.sidebar-expand")
        ?.click();
    },
  };
  return steps[step](...args);
}

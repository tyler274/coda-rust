// What the site tests in `site.rs` do and read in the page. `Tab::step` calls this through
// `Runtime.callFunctionOn`, so what each step is given arrives as arguments, never as script text.
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
    // The composer's text (null when there is no composer) and the mark beside it.
    composer() {
      return [
        document.querySelector("textarea.composer-input")?.value ?? null,
        document.querySelector(".composer-context [role=status]")?.textContent ?? "",
      ];
    },
    async savedDraft() {
      const url = new URL(location.href);
      const patient = url.searchParams.get("patient");
      const chat = url.searchParams.get("chat");
      const session = localStorage.getItem("coda.physician.session");
      const response = await fetch(`/physician-state/drafts/${patient}/${chat}`, {
        headers: session ? { Authorization: `Bearer ${session}` } : {},
      });
      if (!response.ok) throw new Error(`draft read returned ${response.status}`);
      return response.json();
    },
    // This tab's same-tab draft recovery keys.
    draftRecoveryKeys() {
      return Object.keys(sessionStorage).filter((key) => key.startsWith("coda.physician.draft."));
    },
    // The open dialog's words, or nothing when no dialog is open.
    dialogText() {
      return document.querySelector("dialog.dialog")?.innerText ?? "";
    },
    // The text of each line in the conversation.
    conversationLines() {
      return [...document.querySelectorAll('[aria-label="Conversation"] .message')].map(
        (line) => line.textContent,
      );
    },
    // Each response's feedback controls as shown: the pressed thumb, whether the thumbs can be
    // pressed, what the details button says and the notice beside them.
    ratings() {
      return [...document.querySelectorAll('.message-actions[aria-label="Response actions"]')].map(
        (row) => ({
          pressed: row.querySelector('.feedback-rating [aria-pressed="true"]')?.getAttribute("aria-label") ?? null,
          enabled: [...row.querySelectorAll(".feedback-rating button")].every((button) => !button.disabled),
          details: row.querySelector(".feedback-details-open")?.textContent.trim() ?? null,
          notice: row.querySelector(".feedback-notice")?.textContent ?? null,
        }),
      );
    },
    // Clicks the control named `label` in the `index`th response's feedback controls.
    clickInRating(index, label) {
      const row = document.querySelectorAll('.message-actions[aria-label="Response actions"]')[index];
      if (!row) throw new Error(`no response ${index}`);
      const control = [...row.querySelectorAll("button")].find(
        (button) => button.getAttribute("aria-label") === label || button.textContent.trim() === label,
      );
      if (!control) throw new Error(`response ${index} has no ${label}`);
      control.click();
    },
    // The conflict the composer shows: the newer revision and its text, or null.
    conflict() {
      const conflict = document.querySelector(".draft-conflict");
      if (!conflict) return null;
      return [Number(conflict.dataset.revision), conflict.querySelector("pre")?.textContent ?? ""];
    },
    // Puts the caret at the end of a field, so the next key types there.
    focusEnd(selector) {
      const field = one(selector);
      field.focus();
      field.setSelectionRange(field.value.length, field.value.length);
    },
    // Presses `key` on `selector` as a keydown that bubbles to the page.
    keyOn(selector, key) {
      one(selector).dispatchEvent(new KeyboardEvent("keydown", { key, bubbles: true }));
    },
    // How many open dialogs say `words`.
    dialogsSaying(words) {
      return [...document.querySelectorAll("dialog")].filter((dialog) => dialog.innerText.includes(words)).length;
    },
    // Sends from the keyboard: Ctrl+Enter in the composer.
    sendWithKeyboard() {
      const box = one("textarea.composer-input");
      box.focus();
      box.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", ctrlKey: true, bubbles: true }));
    },
    // Clicks New conversation and checks every render until its empty state shows: `seen` is
    // whether any of them still listed a line containing `old`. The click itself changes no DOM,
    // so the page before it is not counted.
    newConversationShowing(old) {
      return new Promise((resolve) => {
        const region = () => document.querySelector('[aria-label="Conversation"]');
        const lines = () => [...(region()?.querySelectorAll(".message") ?? [])].map((m) => m.textContent);
        let seen = false;
        const check = () => {
          if (lines().some((text) => text.includes(old))) seen = true;
          if (region()?.querySelector(".conversation-empty")) {
            observer.disconnect();
            resolve({ seen, done: true });
          }
        };
        const observer = new MutationObserver(check);
        observer.observe(document.body, { subtree: true, childList: true, characterData: true });
        one("button.sidebar-new-conversation").click();
        setTimeout(() => {
          observer.disconnect();
          resolve({ seen, done: false });
        }, 10000);
      });
    },
    // Whether the logo and favicon load from under the `/physician/` prefix.
    async prefixAssetsLoad() {
      const logo = one("img.coda-logo");
      await logo.decode();
      const icon = one('link[rel="icon"]');
      const response = await fetch(icon.href);
      return logo.naturalWidth > 0 && new URL(logo.src).pathname.startsWith("/physician/assets/")
        && new URL(icon.href).pathname === "/physician/favicon.svg" && response.ok
        && (await response.text()).includes("<svg");
    },
    // The sidebar and the page as they stand.
    readShell() {
      const sidebar = one(".glass-sidebar");
      const stage = one(".glass-stage");
      const shown = (selector) => {
        const style = getComputedStyle(one(selector));
        return style.display !== "none" && style.visibility !== "hidden";
      };
      return {
        sidebar: shown(".glass-sidebar"),
        sidebarLeft: sidebar.getBoundingClientRect().left,
        sidebarWidth: sidebar.getBoundingClientRect().width,
        stageLeft: stage.getBoundingClientRect().left,
        stageWidth: stage.getBoundingClientRect().width,
        stageInert: stage.inert,
        backdrop: shown(".glass-sidebar-backdrop"),
        menu: shown(".stage-menu"),
        close: shown(".sidebar-close"),
        collapse: shown(".sidebar-collapse"),
        expand: shown("button.sidebar-expand"),
        labels: shown(".sidebar-nav-item > .item-text"),
        focus: document.activeElement?.getAttribute("aria-label") ?? null,
        pageWidth: document.documentElement.scrollWidth,
      };
    },
  };
  return steps[step](...args);
}

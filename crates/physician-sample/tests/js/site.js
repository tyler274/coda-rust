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
    // Each queued message as shown: its words, and the actions beside it.
    queuedLines() {
      return [...document.querySelectorAll('[aria-label="Conversation"] .message.is-queued')].map(
        (line) => ({
          text: line.querySelector("p, pre")?.textContent ?? line.textContent,
          status: line.parentElement.querySelector(".message-status span")?.textContent ?? "",
          actions: [...line.parentElement.querySelectorAll(".message-status button")].map(
            (button) => button.textContent.trim(),
          ),
        }),
      );
    },
    // Clicks `words` beside the queued message that says `text`.
    clickQueued(text, words) {
      const line = [...document.querySelectorAll('[aria-label="Conversation"] .message.is-queued')].find(
        (line) => line.textContent.includes(text),
      );
      if (!line) throw new Error(`no queued message says ${text}`);
      const button = [...line.parentElement.querySelectorAll(".message-status button")].find(
        (button) => button.textContent.trim() === words,
      );
      if (!button) throw new Error(`no ${words} beside ${text}`);
      button.click();
    },
    // Where the open chat stands with Archive: the sidebar's active and archived titles, whether
    // the page offers Archive (and whether it can be pressed), the archived line, and the toast.
    archiveState() {
      const titles = (selector) =>
        [...document.querySelectorAll(selector)].map((row) => row.textContent.trim());
      const archive = document.querySelector("button.conversation-archive");
      return {
        active: titles(".sidebar-conversation-link:not(.sidebar-archived-link)"),
        archived: titles(".sidebar-archived-link"),
        archive: archive ? (archive.disabled ? "disabled" : "enabled") : "absent",
        archivedLine: !!document.querySelector(".conversation-archived"),
        toast: document.querySelector(".toast:not(.is-leaving)")?.textContent.trim() ?? null,
        notice: [...document.querySelectorAll('[aria-label="Conversation"] [role="status"], [aria-label="Conversation"] [role="alert"]')]
          .map((line) => line.textContent.trim())
          .filter((text) => text.includes("archive")),
      };
    },
    // The words on the conversation's mechanical line buttons, such as Stop or Try again.
    mechanicalActions() {
      return [...document.querySelectorAll(".conversation-mechanical button")].map((button) =>
        button.textContent.trim(),
      );
    },
    // The composer's primary action words.
    sendWords() {
      return one(".composer-dock button[type=submit]").textContent.trim();
    },
    // The conflict the composer shows: the newer revision and its text, or null.
    conflict() {
      const conflict = document.querySelector(".draft-conflict");
      if (!conflict) return null;
      return [Number(conflict.dataset.revision), conflict.querySelector("pre")?.textContent ?? ""];
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

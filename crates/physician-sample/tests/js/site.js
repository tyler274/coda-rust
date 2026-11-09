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
    citationLayout(scroll = false) {
      const pane = one(".source-reading-pane");
      const content = one(".source-reading-pane .source-pane-body");
      content.scrollTop = scroll ? content.scrollHeight : 0;
      const rect = element => {
        const r = element.getBoundingClientRect();
        return { x:r.x, y:r.y, width:r.width, height:r.height, bottom:r.bottom, right:r.right };
      };
      const way = [...pane.querySelectorAll(".source-pane-close, .source-pane-back")]
        .find((control) => control.checkVisibility());
      const d = rect(pane), close = way ? rect(way) : { x:-1, y:-1, bottom:0, right:0, width:0 };
      const blocks = [...pane.querySelectorAll(".clinical-source-reader > *")];
      return {
        opacity: getComputedStyle(pane).opacity,
        animating: pane.getAnimations({subtree:true}).some(a => a.playState === "running"),
        geometry: [d, ...blocks.map(rect)],
        fits: d.x >= 0 && d.right <= innerWidth + 1 && d.y >= 0 && d.bottom <= innerHeight + 1
          && content.scrollWidth <= content.clientWidth + 1
          && blocks.every(b => b.scrollWidth <= b.clientWidth + 1),
        separated: rect(pane.querySelector(".source-pane-heading")).bottom <= rect(blocks[0]).y + 1
          && blocks.every((b, i) => !i || rect(blocks[i-1]).bottom <= rect(b).y + 1),
        closeVisible: close.width > 0 && close.y >= 0 && close.bottom <= innerHeight
          && close.x >= 0 && close.right <= innerWidth,
        scrollable: content.scrollHeight > content.clientHeight + 1,
        scrolled: content.scrollTop > 0,
      };
    },
    // Where a mouse rests on the conversation's citation, in viewport pixels.
    citationCenter() {
      const chip = one("section[aria-label='Conversation'] .citation-chip");
      chip.scrollIntoView({ block: "center" });
      const box = chip.getBoundingClientRect();
      return [box.x + box.width / 2, box.y + box.height / 2];
    },
    citationPreview() {
      const chip = one("section[aria-label='Conversation'] .citation-chip");
      const preview = document.getElementById(chip.getAttribute("aria-controls"));
      const body = preview?.querySelector(".citation-preview-body");
      if (!body) return null;
      const original = preview.querySelector(".citation-preview-actions a");
      return { text: body.innerText, original: original?.getAttribute("href") ?? null,
        originalLabel: original?.innerText ?? null };
    },
    citationClosed() { return !document.querySelector(".source-reading-pane"); },
    citationReading() {
      const pane = one(".source-reading-pane");
      const technical = pane.querySelector(".source-technical-details");
      return {
        title: pane.querySelector("h2").textContent,
        kind: pane.querySelector(".source-record-kind")?.textContent ?? "",
        sections: [...pane.querySelectorAll(".source-record-section")].map(s => s.textContent).join("\n"),
        visible: pane.querySelector(".clinical-source-reader").innerText,
        technicalOpen: technical.open,
        technical: technical.textContent,
        original: pane.querySelector(".source-original-data").textContent,
        focused: document.activeElement === pane.querySelector("h2"),
        order: [...pane.querySelectorAll(
          ".clinical-source-reader > .citation-open, .source-coda-summary, .source-record-kind",
        )].map((node) => node.className),
        link: pane.querySelector(".clinical-source-reader > .citation-open")?.getAttribute("href") ?? null,
        linkTitle: pane.querySelector(".clinical-source-reader > .citation-open")?.getAttribute("title") ?? null,
        summary: pane.querySelector(".source-coda-summary")?.innerText ?? null,
        statuses: [...pane.querySelectorAll(".source-record-status")].map((node) => node.innerText),
        // The reader's blocks in reading order, by their first class.
        blocks: [...pane.querySelector(".clinical-source-reader").children]
          .map((node) => node.classList[0] ?? node.tagName.toLowerCase()),
        // Each readable field's source path with the text shown for it, outside Technical details.
        fields: [...pane.querySelectorAll("[data-source-path]")]
          .filter((node) => !node.closest(".source-technical-details"))
          .map((node) => ({ path: node.dataset.sourcePath, title: node.title, text: node.innerText })),
      };
    },
    // Presses the pane's way out a person can see: Close beside a wide reader, Back over a narrow
    // one. A control hidden at this width is never clicked.
    citationDismiss() {
      const shown = [...document.querySelectorAll(".source-pane-close, .source-pane-back")]
        .filter((control) => control.checkVisibility() && control.getBoundingClientRect().width > 0);
      if (shown.length !== 1) throw new Error(`expected one visible way out, saw ${shown.length}`);
      shown[0].click();
      return shown[0].classList.contains("source-pane-back") ? "back" : "close";
    },
    citationFocusReturned() {
      return document.activeElement?.classList.contains("citation-chip") ?? false;
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

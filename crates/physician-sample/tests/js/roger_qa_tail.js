// Browser inspection and interaction steps for `site_roger_qa_tail.rs`.
// Invoked via `Runtime.callFunctionOn` so parameters travel as structured JSON arguments.
function (step, ...args) {
  const one = (selector) => {
    const found = document.querySelector(selector);
    if (!found) throw new Error(`nothing matches ${selector}`);
    return found;
  };
  const steps = {
    visibleSnapshot() {
      return {
        text: document.body?.innerText ?? "",
        html: document.documentElement?.outerHTML ?? "",
      };
    },
    overflowMetrics() {
      return {
        scrollWidth: document.documentElement.scrollWidth,
        clientWidth: document.documentElement.clientWidth,
      };
    },
    clickNamed(selector, words) {
      const found = [...document.querySelectorAll(selector)].find(
        (element) => element.textContent.trim() === words,
      );
      if (!found) throw new Error(`no ${selector} says ${words}`);
      found.click();
    },
    clickContaining(selector, words) {
      const found = [...document.querySelectorAll(selector)].find(
        (element) => element.textContent.includes(words),
      );
      if (!found) throw new Error(`no ${selector} contains ${words}`);
      found.click();
    },
    searchInputValue() {
      const input = document.querySelector(".search-field input");
      return input ? input.value : null;
    },
    dialogRowTitles() {
      return [...document.querySelectorAll(".dialog-results .list-row-text strong")].map(
        (el) => el.textContent.trim(),
      );
    },
    dialogRowDetails() {
      return [...document.querySelectorAll(".dialog-results .list-row-text small")].map(
        (el) => el.textContent.trim(),
      );
    },
    scrollToBottom(selector) {
      const el = one(selector);
      el.scrollTop = el.scrollHeight;
      el.dispatchEvent(new Event("scroll", { bubbles: true }));
      return {
        scrollTop: el.scrollTop,
        scrollHeight: el.scrollHeight,
        clientHeight: el.clientHeight,
      };
    },
    conversationTitles() {
      return [...document.querySelectorAll(".glass-sidebar button.sidebar-conversation-link .sidebar-recent-title")].map(
        (el) => el.textContent.trim(),
      );
    },
    pageConversationTitles() {
      return [...document.querySelectorAll('section[aria-label="Conversation"] button.sidebar-conversation-link .sidebar-recent-title')].map(
        (el) => el.textContent.trim(),
      );
    },
    conversationRowTexts() {
      return [...document.querySelectorAll(".glass-sidebar button.sidebar-conversation-link")].map(
        (el) => el.innerText.trim(),
      );
    },
  };
  return steps[step](...args);
}

(action, ...args) => {
  switch (action) {
    case "pageText":
      return document.body ? document.body.innerText : "";
    case "sessionToken":
      return window.localStorage.getItem("coda.physician.session") || "";
    case "sidewaysScroll":
      return Math.max(0, document.documentElement.scrollWidth - window.innerWidth);
    case "exists": {
      const [selector] = args;
      return Boolean(document.querySelector(selector));
    }
    case "textOf": {
      const [selector] = args;
      const el = document.querySelector(selector);
      return el ? el.innerText : "";
    }
    case "click": {
      const [selector] = args;
      const el = document.querySelector(selector);
      if (!el) return false;
      el.scrollIntoView({ block: "center" });
      el.click();
      return true;
    }
    case "press": {
      const [words] = args;
      const target = Array.from(document.querySelectorAll("button, a")).find(
        (el) => el.innerText && el.innerText.trim().includes(words)
      );
      if (!target) return false;
      target.click();
      return true;
    }
    case "fillTextarea": {
      const [selector, value] = args;
      const el =
        document.querySelector(`${selector} textarea`) ||
        document.querySelector(selector);
      if (!el) return false;
      el.focus();
      const setter = Object.getOwnPropertyDescriptor(
        window.HTMLTextAreaElement.prototype,
        "value"
      ).set;
      setter.call(el, value);
      el.dispatchEvent(new Event("input", { bubbles: true }));
      el.dispatchEvent(new Event("change", { bubbles: true }));
      return true;
    }
    case "scrollTo": {
      const [selector] = args;
      const el = document.querySelector(selector);
      if (!el) return false;
      el.scrollIntoView({ block: "start" });
      return true;
    }
    default:
      throw new Error(`unknown action: ${action}`);
  }
}

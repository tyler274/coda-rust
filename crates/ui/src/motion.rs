//! The crate's look and the motion its stylesheet cannot express alone, after the React app's
//! `motion.ts`: script-driven motion runs on the stylesheet's own roles, and none of it runs under
//! reduced motion or keyboard-driven interaction, where the same end state simply arrives (Dev,
//! 2026-09-29/30). A list's selection is one fill that slides to the item chosen and otherwise stays
//! anchored to it.

use dioxus::prelude::*;

use crate::STYLESHEET;

/// Installs, once per page: which way the person is driving it (`data-interaction` on the root,
/// pointer or keyboard), `window.codaMotion` for motion measured before and after, and the slide of
/// every selection host's fill when its current item changes.
const MOTION: &str = r#"
if (!window.codaMotion) {
  const root = document.documentElement;
  const pointer = () => { if (root.dataset.interaction !== "pointer") root.dataset.interaction = "pointer"; };
  document.addEventListener("pointerdown", pointer, true);
  document.addEventListener("pointermove", pointer, { capture: true, passive: true });
  document.addEventListener("keydown", () => { root.dataset.interaction = "keyboard"; }, true);
  const reduced = matchMedia("(prefers-reduced-motion: reduce)");
  const immediate = () => reduced.matches || root.dataset.interaction === "keyboard";
  const token = (name) => getComputedStyle(root).getPropertyValue(name).trim();
  const milliseconds = (value) => parseFloat(value) * (value.endsWith("ms") ? 1 : 1000);
  // An element animated on a motion role, or not at all where motion is immediate.
  const animate = (element, keyframes, role, easing, pseudoElement) => immediate() ? undefined
    : element.animate(keyframes, { duration: milliseconds(token(`--motion-${role}`)), easing: token(`--ease-${easing}`),
      ...(pseudoElement === undefined ? {} : { pseudoElement }) });
  // The item a selection host's fill is anchored to (components.css).
  const currentItem = (host) => host.querySelector('[aria-current]:not([aria-current="false"])');
  // Only when the selection itself moves does the fill travel, from where the item it left still stands.
  const anchored = CSS.supports("anchor-name: --selection");
  const shown = new WeakMap();
  const follow = () => {
    for (const host of document.querySelectorAll(".selection-host")) {
      const next = currentItem(host);
      const known = shown.has(host);
      const from = shown.get(host);
      shown.set(host, next);
      if (!known || from === next || !anchored || !from || !next || !from.isConnected || !host.contains(from)) continue;
      const was = from.getBoundingClientRect();
      const is = next.getBoundingClientRect();
      animate(host, [{ translate: `${was.left - is.left}px ${was.top - is.top}px`, width: `${was.width}px`,
        height: `${was.height}px` }, {}], "step", "emphasized", "::after");
    }
  };
  new MutationObserver(follow).observe(root, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-current"] });
  follow();
  window.codaMotion = { animate, currentItem };
}
"#;

/// The crate's stylesheet with an app's own styles after it, and the crate's motion: each app's
/// root draws it once.
#[component]
pub fn Stylesheet(#[props(default)] app: &'static str) -> Element {
    use_hook(|| {
        document::eval(MOTION);
    });
    rsx! { style { "{STYLESHEET}{app}" } }
}

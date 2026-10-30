//! Overlays: surfaces that stand over the page beside what opened them. Each is the browser's own
//! popover in the top layer, so nothing beneath covers or clips it, and the stylesheet places it
//! against the control that opens it.

use dioxus::prelude::*;

use crate::{Icon, IconName, KeyHint};

/// A surface of the host's content beside the control that opens it, a `Button` whose `opens`
/// names this `id`. It opens under the control from its leading edge, flipping above or across
/// where there isn't room, and follows the control as the page scrolls. Escape, a press elsewhere or
/// the control again closes it, and focus returns to the control. `label` names it; content marked
/// `autofocus` takes focus as it opens.
#[component]
pub fn Popover(id: String, label: String, children: Element) -> Element {
    rsx! {
        div { id, class: "popover", "popover": "auto", role: "dialog", aria_label: label, {children} }
    }
}

/// A menu of the host's choices beside the control that opens it, a `Button` whose `opens` names
/// this `id` and whose `aria_haspopup` is `menu`. It opens and closes as a `Popover` does, with
/// focus on its first choice. The arrow keys, Home and End move through the choices, a letter
/// moves to the next that begins with it, and pointing at one moves there too. Choosing one, or
/// Tab, closes it. Its choices are `MenuItem`s, grouped by `MenuSection`s, with a `MenuPalette`
/// leading where a row of icons serves.
#[component]
pub fn Menu(id: String, label: String, children: Element) -> Element {
    rsx! {
        div {
            id,
            class: "popover menu",
            "popover": "auto",
            role: "menu",
            aria_label: label,
            onmounted: |_| {
                document::eval(MENU_KEYS);
            },
            {children}
        }
    }
}

/// Related choices in a menu, set off from the others by a hairline, under `label` when the group
/// needs naming.
#[component]
pub fn MenuSection(label: Option<String>, children: Element) -> Element {
    rsx! {
        div { class: "menu-section", role: "group", aria_label: label.clone(),
            if let Some(label) = &label {
                div { class: "menu-section-label", aria_hidden: "true", "{label}" }
            }
            {children}
        }
    }
}

/// One choice in a menu: its icon, its words with an optional `detail` beneath, and the key that
/// does the same where there is a keyboard. A choice that is on or off carries `checked`, its mark
/// at the trailing edge; `radio` makes it one of a section's choices of which only one is on.
/// `danger` is red, for what removes what the person has. A `disabled` choice stays where it is,
/// reachable but inert.
#[component]
pub fn MenuItem(
    icon: Option<IconName>,
    detail: Option<String>,
    shortcut: Option<String>,
    checked: Option<bool>,
    #[props(default)] radio: bool,
    #[props(default)] danger: bool,
    #[props(default)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let role = match (checked, radio) {
        (None, _) => "menuitem",
        (Some(_), false) => "menuitemcheckbox",
        (Some(_), true) => "menuitemradio",
    };
    rsx! {
        button {
            class: if danger { "menu-item is-danger" } else { "menu-item" },
            r#type: "button",
            role,
            tabindex: "-1",
            aria_checked: checked.map(|on| if on { "true" } else { "false" }),
            aria_disabled: disabled.then_some("true"),
            onclick: move |event| {
                if !disabled {
                    onclick.call(event);
                }
            },
            if let Some(name) = icon {
                Icon { name }
            }
            span { class: "menu-item-label",
                span { {children} }
                if let Some(detail) = detail {
                    small { "{detail}" }
                }
            }
            if let Some(key) = shortcut {
                KeyHint { key_label: key }
            }
            if checked == Some(true) {
                span { class: "menu-item-check", Icon { name: IconName::Check } }
            }
        }
    }
}

/// A row of icon choices leading a menu, such as a reply's thumbs, named together by `label`.
#[component]
pub fn MenuPalette(label: String, children: Element) -> Element {
    rsx! {
        div { class: "menu-palette", role: "group", aria_label: label, {children} }
    }
}

/// One icon in a menu's palette, named by `label`; `chosen` draws it in the accent.
#[component]
pub fn MenuChoice(
    label: String,
    icon: IconName,
    #[props(default)] chosen: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "menu-choice",
            r#type: "button",
            role: "menuitemcheckbox",
            tabindex: "-1",
            aria_label: label,
            aria_checked: if chosen { "true" } else { "false" },
            onclick: move |event| onclick.call(event),
            Icon { name: icon }
        }
    }
}

/// Installs, once per page, every menu's keys, its focus on opening and its closing on a choice.
/// The listeners sit on the document, so a menu added later needs nothing more.
const MENU_KEYS: &str = r#"
if (!window.codaMenuKeys) {
  window.codaMenuKeys = true;
  const choices = (menu) => [...menu.querySelectorAll('[role^="menuitem"]')];
  const menuOf = (element) => element?.closest?.(".menu:popover-open");
  document.addEventListener("toggle", (event) => {
    if (event.target.classList?.contains("menu") && event.newState === "open") {
      choices(event.target)[0]?.focus({ preventScroll: true });
    }
  }, true);
  document.addEventListener("keydown", (event) => {
    const menu = menuOf(event.target);
    if (!menu || event.altKey || event.ctrlKey || event.metaKey) return;
    if (event.key === "Tab") { menu.hidePopover(); return; }
    const all = choices(menu);
    const now = all.indexOf(document.activeElement);
    const inPalette = !!document.activeElement?.closest(".menu-palette");
    let next = { ArrowDown: now + 1, ArrowUp: now < 0 ? all.length - 1 : now - 1, Home: 0, End: all.length - 1 }[event.key];
    if (inPalette && (event.key === "ArrowRight" || event.key === "ArrowLeft")) next = now + (event.key === "ArrowRight" ? 1 : -1);
    // Down from the palette goes to the first row beneath it.
    const below = all.findIndex((choice) => !choice.closest(".menu-palette"));
    if (inPalette && event.key === "ArrowDown" && below >= 0) next = below;
    if (next === undefined && event.key.length === 1 && event.key !== " ") {
      const letter = event.key.toLocaleLowerCase();
      const after = [...all.slice(now + 1), ...all.slice(0, now + 1)];
      const found = after.find((choice) => (choice.getAttribute("aria-label") ?? choice.textContent).trim().toLocaleLowerCase().startsWith(letter));
      if (found) next = all.indexOf(found);
    }
    if (next === undefined) return;
    event.preventDefault();
    all[(next + all.length) % all.length]?.focus({ preventScroll: false });
  });
  document.addEventListener("pointermove", (event) => {
    if (event.pointerType !== "mouse" || !menuOf(event.target)) return;
    const choice = event.target.closest('[role^="menuitem"]');
    if (choice && document.activeElement !== choice) choice.focus({ preventScroll: true });
  }, { passive: true });
  document.addEventListener("click", (event) => {
    const choice = event.target.closest?.('[role^="menuitem"]');
    const menu = menuOf(choice);
    if (menu && choice.getAttribute("aria-disabled") !== "true") menu.hidePopover();
  });
}
"#;

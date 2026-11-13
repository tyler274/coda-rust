//! Overlays: surfaces that stand over the page in the top layer, so nothing beneath covers or clips
//! them. A popover, menu or tooltip is the browser's own popover, which the stylesheet places
//! against what opens it.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Icon, IconName, KeyHint, host_attributes, script};

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
/// focus on its first choice when opened from the keys. The arrow keys, Home and End move through the choices, a letter
/// moves to the next that begins with it, and pointing at one moves there too. Choosing one, or
/// Tab, closes it. Its choices are `MenuItem`s, grouped by `MenuSection`s.
#[component]
pub fn Menu(id: String, label: String, children: Element) -> Element {
    rsx! {
        div {
            id,
            class: "popover menu",
            "popover": "auto",
            role: "menu",
            tabindex: "-1",
            aria_label: label,
            onmounted: |_| {
                script::menu_keys();
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

/// Content that opens the `Menu` named by `menu` where the person asks for it. A right-click opens
/// it at the pointer, and the menu key or Shift-F10 under what has focus inside. A press held half
/// a second on touch lifts the content's first element over a blurred page, no taller than about
/// half the window and fading at its foot when longer, with the menu beneath it at whichever edge
/// the content stands nearer; the page itself never moves. Whatever closes the menu settles the
/// content back where it stood. The same menu can also have its own `Button`.
#[component]
pub fn ContextMenu(
    menu: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("context-{}", HOSTS.fetch_add(1, Ordering::Relaxed)));
    let host = id.clone();
    let dropped = id.clone();
    use_drop(move || {
        script::dispose_context_host(&dropped);
    });
    let mut class = String::from("context-menu-host");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        div {
            id: "{id}",
            class: "{class}",
            "data-menu": "{menu}",
            onmounted: move |_| {
                script::context_host(&host);
            },
            ..rest,
            {children}
        }
    }
}

static HOSTS: AtomicUsize = AtomicUsize::new(0);

/// What a control does, named over it once the pointer rests on it, with the key that does the
/// same where there is a keyboard. Keyboard focus shows it at once, and once one has shown, the
/// next shows at once too; touch never shows it. It stands above the control, below where there
/// isn't room, and leaves on Escape, a press, or the pointer or focus leaving. Its words echo the
/// control's own accessible name, which is what assistive technology hears, so it stays hidden
/// from it.
#[component]
pub fn Tooltip(text: String, shortcut: Option<String>, children: Element) -> Element {
    let anchor = use_hook(|| format!("--tip-{}", TIPS.fetch_add(1, Ordering::Relaxed)));
    rsx! {
        span {
            class: "tooltip-host",
            style: "anchor-name: {anchor}",
            onmounted: |_| {
                script::tooltips();
            },
            {children}
            span { class: "tooltip", "popover": "manual", aria_hidden: "true", style: "position-anchor: {anchor}",
                "{text}"
                if let Some(key) = shortcut {
                    kbd { "{key}" }
                }
            }
        }
    }
}

static TIPS: AtomicUsize = AtomicUsize::new(0);

/// One row of a list found by typing: its icon, its words with a quieter `detail` beside them, and
/// whatever the host sets at its end. The row under the keys or the pointer is `active`; pointing
/// at one moves there. One that is `disabled` is shown, but can't be reached or chosen.
#[component]
pub(crate) fn OptionRow(
    id: String,
    label: String,
    detail: Option<String>,
    icon: Option<IconName>,
    active: bool,
    #[props(default)] disabled: bool,
    on_point: EventHandler,
    on_choose: EventHandler,
    children: Element,
) -> Element {
    rsx! {
        div {
            id,
            class: "option-row",
            role: "option",
            aria_selected: if active { "true" } else { "false" },
            aria_disabled: disabled.then_some("true"),
            onpointermove: move |_| {
                if !active && !disabled {
                    on_point.call(());
                }
            },
            onclick: move |_| {
                if !disabled {
                    on_choose.call(());
                }
            },
            if let Some(name) = icon {
                Icon { name }
            }
            span { class: "option-row-label",
                span { "{label}" }
                if let Some(detail) = detail {
                    small { "{detail}" }
                }
            }
            {children}
        }
    }
}

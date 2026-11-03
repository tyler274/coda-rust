//! Overlays: surfaces that stand over the page in the top layer, so nothing beneath covers or clips
//! them. A popover, menu or tooltip is the browser's own popover, which the stylesheet places
//! against what opens it; the command palette is a modal dialog.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Icon, IconName, KeyHint, host_attributes, script, use_modal};

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

/// One command in a `CommandPalette`.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Command {
    /// The heading it is listed under; a group stands where its first command does, and an empty
    /// one has no heading.
    pub group: String,
    /// What it does, in the words matched against what is typed.
    pub label: String,
    /// Its icon.
    pub icon: Option<IconName>,
    /// A quieter line after its words, matched too.
    pub detail: Option<String>,
    /// The key that does the same, shown where there is a keyboard.
    pub shortcut: Option<String>,
}

/// The commands, found by typing: a modal over the page, toward its top, its field taking focus
/// and every command listed under its group. Each word typed narrows the list to the commands
/// whose words or detail contain it, and the first one left is the active one. The arrow
/// keys or the pointer move the active command, and Enter or a press chooses it: the host hears
/// its index in `commands`, and the palette closes. Escape or a press beside it closes it too; it
/// settles out before the host hears `on_close`. `empty` is said when nothing matches.
#[component]
pub fn CommandPalette(
    /// Its name and its field's.
    label: String,
    #[props(default)] placeholder: String,
    commands: Vec<Command>,
    empty: String,
    on_choose: EventHandler<usize>,
    on_close: EventHandler,
) -> Element {
    let id = use_hook(|| format!("palette-{}", PALETTES.fetch_add(1, Ordering::Relaxed)));
    let mut query = use_signal(String::new);
    let mut active = use_signal(|| 0usize);
    let mut closing = use_signal(|| false);
    use_modal(id.clone(), "input", on_close);
    let mut dismiss = move || closing.set(true);
    let rows = matching(&commands, &query());
    let current = rows
        .get(active().min(rows.len().saturating_sub(1)))
        .copied();
    let mut choose = move |index: usize| {
        if !closing() {
            on_choose.call(index);
            dismiss();
        }
    };
    let list = format!("{id}-list");
    // Each group's commands, with each one's place in the list the keys move through.
    let mut groups: Vec<(&str, Vec<(usize, usize)>)> = Vec::new();
    for (at, &index) in rows.iter().enumerate() {
        let group = commands[index].group.as_str();
        match groups.last_mut() {
            Some((last, members)) if *last == group => members.push((index, at)),
            _ => groups.push((group, vec![(index, at)])),
        }
    }
    let moving = id.clone();
    let keyed = rows.clone();
    rsx! {
        dialog {
            id: "{id}",
            class: if closing() { "dialog command-palette is-closing" } else { "dialog command-palette" },
            aria_label: "{label}",
            onkeydown: move |event| {
                if event.key() == Key::Escape {
                    event.stop_propagation();
                }
            },
            oncancel: move |event| {
                event.prevent_default();
                dismiss();
            },
            onclick: move |_| dismiss(),
            div { class: "command-palette-body", onclick: move |event| event.stop_propagation(),
                div { class: "search-field field command-palette-search",
                    Icon { name: IconName::Search }
                    input {
                        r#type: "text",
                        role: "combobox",
                        aria_label: "{label}",
                        aria_expanded: "true",
                        aria_controls: "{list}",
                        aria_autocomplete: "list",
                        aria_activedescendant: current.map(|index| format!("{id}-{index}")),
                        autocomplete: "off",
                        spellcheck: "false",
                        placeholder: "{placeholder}",
                        value: "{query}",
                        oninput: move |event| {
                            query.set(event.value());
                            active.set(0);
                        },
                        onkeydown: move |event| {
                            // Keys that confirm composed text are the text's, not the palette's.
                            if event.is_composing() {
                                return;
                            }
                            let rows = &keyed;
                            let count = rows.len();
                            let now = active().min(count.saturating_sub(1));
                            let next = match event.key() {
                                Key::ArrowDown if count > 0 => (now + 1) % count,
                                Key::ArrowUp if count > 0 => (now + count - 1) % count,
                                Key::Enter => {
                                    event.prevent_default();
                                    if let Some(&index) = rows.get(now) {
                                        choose(index);
                                    }
                                    return;
                                }
                                _ => return,
                            };
                            event.prevent_default();
                            active.set(next);
                            script::scroll_into_view(&format!("{moving}-{}", rows[next]));
                        },
                    }
                }
                div { id: "{list}", class: "command-palette-list", role: "listbox", aria_label: "{label}",
                    for (number, (group, members)) in groups.into_iter().enumerate() {
                        div {
                            key: "{group}-{number}",
                            role: "group",
                            aria_labelledby: (!group.is_empty()).then(|| format!("{id}-group-{number}")),
                            if !group.is_empty() {
                                div { id: "{id}-group-{number}", class: "command-group-label", "{group}" }
                            }
                            for (index, at) in members {
                                OptionRow {
                                    key: "{index}",
                                    id: "{id}-{index}",
                                    label: commands[index].label.clone(),
                                    detail: commands[index].detail.clone(),
                                    icon: commands[index].icon,
                                    active: current == Some(index),
                                    on_point: move |_| active.set(at),
                                    on_choose: move |_| choose(index),
                                    if let Some(key) = commands[index].shortcut.clone() {
                                        KeyHint { key_label: key }
                                    }
                                }
                            }
                        }
                    }
                }
                if rows.is_empty() {
                    p { class: "command-palette-empty", role: "status", "{empty}" }
                }
            }
        }
    }
}

static PALETTES: AtomicUsize = AtomicUsize::new(0);

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

/// The commands whose words or detail hold every word of `query`, ignoring case, each group
/// together where its first match stands.
fn matching(commands: &[Command], query: &str) -> Vec<usize> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    let found: Vec<usize> = (0..commands.len())
        .filter(|&index| {
            let command = &commands[index];
            let text = format!(
                "{} {}",
                command.label,
                command.detail.as_deref().unwrap_or_default()
            )
            .to_lowercase();
            words.iter().all(|word| text.contains(word.as_str()))
        })
        .collect();
    let mut groups: Vec<&str> = Vec::new();
    for &index in &found {
        if !groups.contains(&commands[index].group.as_str()) {
            groups.push(&commands[index].group);
        }
    }
    groups
        .into_iter()
        .flat_map(|group| {
            found
                .iter()
                .copied()
                .filter(move |&index| commands[index].group == group)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_word_narrows_the_commands_and_groups_stay_together() {
        let command = |group: &str, label: &str, detail: Option<&str>| Command {
            group: group.into(),
            label: label.into(),
            detail: detail.map(Into::into),
            ..Command::default()
        };
        let commands = [
            command("Conversation", "Copy last reply", None),
            command("Settings", "Theme", Some("Switch to dark")),
            command("Conversation", "Share", None),
            command("Settings", "Open settings", None),
        ];
        assert_eq!(matching(&commands, ""), [0, 2, 1, 3]);
        assert_eq!(matching(&commands, "  DARK "), [1]);
        assert_eq!(matching(&commands, "settings"), [3]);
        assert_eq!(matching(&commands, "co re"), [0]);
        assert!(matching(&commands, "copy dark").is_empty());
    }
}

//! Overlays: surfaces that stand over the page in the top layer, so nothing beneath covers or clips
//! them. A popover, menu or tooltip is the browser's own popover, which the stylesheet places
//! against what opens it; the command palette is a modal dialog.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Icon, IconName, KeyHint, host_attributes, use_modal};

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
            tabindex: "-1",
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
    let script = CONTEXT.replace("{host}", &id);
    let dropped = id.clone();
    use_drop(move || {
        document::eval(&format!(
            "window.codaContextHosts?.['{dropped}']?.dispose()"
        ));
    });
    let mut class = String::from("context-menu-host");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        div {
            id: "{id}",
            class: "{class}",
            "data-menu": "{menu}",
            onmounted: move |_| {
                document::eval(&script);
            },
            ..rest,
            {children}
        }
    }
}

static HOSTS: AtomicUsize = AtomicUsize::new(0);

/// One context menu host's ways in. `{host}` is its id; its menu's is its `data-menu`, read each
/// time it opens. A menu opened at a point keeps that place until it next opens from its own
/// control. Lifting uses one layer per page, `window.codaLift`, which blurs the page beneath; its
/// motion runs on the springs and simply arrives where motion is immediate. The host's
/// `window.codaContextHosts[id].dispose()` ends it: a press or opening still pending never lands,
/// and its lifted copy, and a menu it opened, close.
const CONTEXT: &str = r#"
const host = document.getElementById("{host}");
if (host && !host.dataset.contextMenu) {
  host.dataset.contextMenu = "";
  const root = document.documentElement;
  let gone = false;
  // Whether a button or finger is down anywhere on the page, kept once for every host.
  if (!window.codaPressed) {
    window.codaPressed = { down: false };
    addEventListener("pointerdown", () => { window.codaPressed.down = true; }, { capture: true, passive: true });
    for (const ending of ["pointerup", "pointercancel"]) addEventListener(ending, () => { window.codaPressed.down = false; }, { capture: true, passive: true });
  }
  const margin = 12, gap = 8;
  const immediate = () => matchMedia("(prefers-reduced-motion: reduce)").matches || root.dataset.interaction === "keyboard";
  const timing = (name) => {
    const [length, ...curve] = getComputedStyle(root).getPropertyValue(name).trim().split(" ");
    return { duration: parseFloat(length) * (length.endsWith("ms") ? 1 : 1000), easing: curve.join(" ") || "ease-out" };
  };
  // A press released outside a popover closes every open one, so a menu asked for while a button or finger is still down
  // opens once it is let go.
  const released = (then) => {
    if (!window.codaPressed.down) { then(); return; }
    const go = () => { removeEventListener("pointerup", go, true); removeEventListener("pointercancel", go, true); setTimeout(() => { if (!gone) then(); }); };
    addEventListener("pointerup", go, true);
    addEventListener("pointercancel", go, true);
  };
  const run = (element, keyframes, how) => immediate() ? undefined : element.animate(keyframes, how);
  // Shows the menu at a point, its own control's placement given back the next time that control opens it.
  const show = () => {
    const menu = document.getElementById(host.dataset.menu);
    if (!menu) return;
    if (menu.matches(":popover-open")) menu.hidePopover();
    if (menu.dataset.placesItself === undefined) {
      menu.dataset.placesItself = "";
      menu.addEventListener("beforetoggle", (event) => {
        if (event.newState !== "open" || menu.dataset.atPoint !== undefined) return;
        delete menu.dataset.openedBy;
        menu.classList.remove("is-at-point");
        for (const property of ["left", "top", "transform-origin"]) menu.style.removeProperty(property);
      });
    }
    menu.classList.add("is-at-point");
    menu.dataset.openedBy = host.id;
    menu.dataset.atPoint = "";
    menu.showPopover();
    delete menu.dataset.atPoint;
    return menu;
  };
  const place = (menu, x, y, leading) => {
    const width = menu.offsetWidth, height = menu.offsetHeight;
    const left = Math.max(margin, Math.min(leading ? x : x - width, innerWidth - margin - width));
    const below = y + height + margin <= innerHeight;
    Object.assign(menu.style, { left: `${left}px`, top: `${below ? y : Math.max(margin, y - height)}px`,
      transformOrigin: `${leading ? "left" : "right"} ${below ? "top" : "bottom"}` });
  };
  const openAt = (x, y) => released(() => { const menu = show(); if (menu) place(menu, x, y, true); });
  // Lifting: the copy rises in place while the finger holds, then, once it is let go, moves only as far as the menu
  // beneath it needs.
  const lift = () => {
    getSelection()?.removeAllRanges();
    const content = host.firstElementChild ?? host;
    // Its own box, without the shrink of the press it may still be settling from.
    const box = content.getBoundingClientRect();
    const at = { width: content.offsetWidth, height: content.offsetHeight };
    at.left = box.left + (box.width - at.width) / 2;
    at.top = box.top + (box.height - at.height) / 2;
    at.right = at.left + at.width;
    const copy = content.cloneNode(true);
    for (const element of [copy, ...copy.querySelectorAll("[id]")]) element.removeAttribute("id");
    copy.inert = true;
    // The copy keeps the box the content was given, never one it would take from the layer.
    Object.assign(copy.style, { boxSizing: "border-box", width: `${at.width}px`, maxWidth: "none", margin: "0" });
    const layer = window.codaLift ??= Object.assign(document.createElement("div"), { className: "context-lift", popover: "manual" });
    layer.dataset.host = host.id;
    if (!layer.isConnected) document.body.append(layer);
    const preview = Object.assign(document.createElement("div"), { className: "context-lift-preview" });
    preview.append(copy);
    layer.replaceChildren(preview);
    layer.classList.remove("is-settling");
    const height = Math.min(at.height, innerHeight * 0.45);
    preview.classList.toggle("is-clipped", at.height > height);
    Object.assign(preview.style, { left: `${at.left}px`, top: `${at.top}px`, width: `${at.width}px`, height: `${height}px` });
    layer.showPopover();
    host.dataset.lifted = "";
    run(preview, [{ scale: 0.97 }, { scale: 1 }], timing("--spring-step"));
    const settle = (to) => {
      layer.classList.add("is-settling");
      const back = run(preview, [{ translate: "0 0" }, { translate: to }], { duration: timing("--motion-control").duration, easing: "ease-in", fill: "forwards" });
      const done = () => { if (layer.contains(preview)) { layer.hidePopover(); layer.replaceChildren(); } delete host.dataset.lifted; };
      back ? back.finished.then(done, done) : done();
    };
    released(() => {
      const menu = show();
      if (!menu) { settle("0 0"); return; }
      const top = Math.max(margin, Math.min(at.top, innerHeight - margin - menu.offsetHeight - gap - height));
      preview.style.top = `${top}px`;
      const leading = at.left + at.width / 2 <= innerWidth / 2;
      place(menu, leading ? at.left : at.right, top + height + gap, leading);
      const away = `0 ${at.top - top}px`;
      run(preview, [{ translate: away }, { translate: "0 0" }], timing("--spring-step"));
      const closed = (event) => {
        if (event.newState !== "closed") return;
        menu.removeEventListener("toggle", closed);
        settle(away);
      };
      menu.addEventListener("toggle", closed);
    });
  };
  // A held press on touch or pen: the content shrinks a touch while held and, unmoved for half a second, lifts; the tap
  // that ends it does nothing else. Movement, lifting the finger or a scroll lets go. Android's own long-press menu
  // request lifts it at once.
  let timer, start, fired = false, touch = false;
  const release = () => { clearTimeout(timer); timer = undefined; delete host.dataset.pressing; removeEventListener("scroll", release, true); };
  const fire = () => { release(); fired = true; lift(); };
  host.addEventListener("pointerdown", (event) => {
    touch = event.pointerType !== "mouse";
    if (!touch || event.button !== 0) return;
    start = [event.clientX, event.clientY];
    fired = false;
    host.dataset.pressing = "";
    addEventListener("scroll", release, { capture: true, passive: true });
    timer = setTimeout(fire, 500);
  });
  host.addEventListener("pointermove", (event) => {
    if (timer !== undefined && Math.hypot(event.clientX - start[0], event.clientY - start[1]) > 10) release();
  });
  for (const ending of ["pointerup", "pointercancel", "pointerleave"]) host.addEventListener(ending, release);
  host.addEventListener("click", (event) => {
    if (fired) { fired = false; event.preventDefault(); event.stopPropagation(); }
  }, true);
  host.addEventListener("contextmenu", (event) => {
    event.preventDefault();
    if (touch) { if (!fired) fire(); return; }
    openAt(event.clientX, event.clientY);
  });
  host.addEventListener("keydown", (event) => {
    if (event.key !== "ContextMenu" && !(event.key === "F10" && event.shiftKey)) return;
    event.preventDefault();
    const at = (host.contains(document.activeElement) ? document.activeElement : host).getBoundingClientRect();
    openAt(at.left, at.bottom + 4);
  });
  window.codaContextHosts ??= {};
  window.codaContextHosts[host.id] = {
    dispose() {
      gone = true;
      release();
      const layer = window.codaLift;
      if (layer?.dataset.host === host.id) { if (layer.matches(":popover-open")) layer.hidePopover(); layer.replaceChildren(); }
      const menu = document.getElementById(host.dataset.menu);
      if (menu?.dataset.openedBy === host.id && menu.matches(":popover-open")) menu.hidePopover();
      delete window.codaContextHosts[host.id];
    },
  };
}
"#;

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
                document::eval(TOOLTIPS);
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

/// Installs, once per page, every tooltip's showing and leaving. A rest of half a second shows one;
/// within a moment of another leaving, the next shows at once. The pointer can cross to the tooltip
/// and rest on it: leaving the control waits a moment for it.
const TOOLTIPS: &str = r#"
if (!window.codaTooltips) {
  window.codaTooltips = true;
  let timer, leaving, shown, warmUntil = 0;
  const tipOf = (host) => host?.querySelector(":scope > .tooltip");
  const hide = () => {
    clearTimeout(timer);
    clearTimeout(leaving);
    if (shown?.matches(":popover-open")) { shown.hidePopover(); warmUntil = performance.now() + 400; }
    shown = undefined;
  };
  const show = (host, now) => {
    clearTimeout(timer);
    clearTimeout(leaving);
    const tip = tipOf(host);
    if (!tip || tip === shown) return;
    // It opens on a task of its own, never inside a focus change another popover is making.
    const open = () => { hide(); if (tip.isConnected) { tip.showPopover(); shown = tip; } };
    timer = setTimeout(open, now || shown || performance.now() < warmUntil ? 0 : 500);
  };
  document.addEventListener("pointerover", (event) => {
    if (event.pointerType !== "mouse") return;
    const host = event.target.closest?.(".tooltip-host");
    if (host && !host.contains(event.relatedTarget)) show(host, false);
  });
  document.addEventListener("pointerout", (event) => {
    const host = event.target.closest?.(".tooltip-host");
    if (!host || host.contains(event.relatedTarget)) return;
    clearTimeout(timer);
    clearTimeout(leaving);
    leaving = setTimeout(hide, 120);
  });
  document.addEventListener("focusin", (event) => {
    const host = event.target.closest?.(".tooltip-host");
    if (host && event.target.matches(":focus-visible")) show(host, true);
  });
  document.addEventListener("focusout", hide);
  document.addEventListener("pointerdown", hide, true);
  document.addEventListener("keydown", (event) => { if (event.key === "Escape") hide(); }, true);
}
"#;

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
                            document::eval(&format!(
                                "document.getElementById('{moving}-{}')?.scrollIntoView({{ block: 'nearest' }})",
                                rows[next]
                            ));
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
                                CommandRow {
                                    key: "{index}",
                                    id: "{id}-{index}",
                                    command: commands[index].clone(),
                                    active: current == Some(index),
                                    on_point: move |_| active.set(at),
                                    on_choose: move |_| choose(index),
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

#[component]
fn CommandRow(
    id: String,
    command: Command,
    active: bool,
    on_point: EventHandler,
    on_choose: EventHandler,
) -> Element {
    rsx! {
        div {
            id,
            class: "command-row",
            role: "option",
            aria_selected: if active { "true" } else { "false" },
            onpointermove: move |_| {
                if !active {
                    on_point.call(());
                }
            },
            onclick: move |_| on_choose.call(()),
            if let Some(name) = command.icon {
                Icon { name }
            }
            span { class: "command-row-label",
                span { "{command.label}" }
                if let Some(detail) = command.detail {
                    small { "{detail}" }
                }
            }
            if let Some(key) = command.shortcut {
                KeyHint { key_label: key }
            }
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

/// Installs, once per page, every menu's keys, its focus on opening and its closing on a choice.
/// The listeners sit on the document, so a menu added later needs nothing more.
const MENU_KEYS: &str = r#"
if (!window.codaMenuKeys) {
  window.codaMenuKeys = true;
  const choices = (menu) => [...menu.querySelectorAll('[role^="menuitem"]')];
  const menuOf = (element) => element?.closest?.(".menu:popover-open");
  document.addEventListener("toggle", (event) => {
    // Opened from the keys, focus is on the first choice; otherwise on the menu, where the keys still start.
    if (event.target.classList?.contains("menu") && event.newState === "open") {
      const keyboard = document.documentElement.dataset.interaction === "keyboard";
      (keyboard ? choices(event.target)[0] : event.target)?.focus({ preventScroll: true });
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

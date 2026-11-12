// The crate's browser behaviour, each piece a function called by name, as `src/script.rs` declares
// it. A browser build imports this file as a module, so a page that refuses `eval`, such as a Chrome
// extension's, runs it; a native webview imports it once per page. A function either only acts; or
// answers once, by returning its answer or a promise of it; or keeps listening, reporting through
// `send` and returning how to stop.

// Installs, once per page: which way the person is driving it (`data-interaction` on the root,
// pointer or keyboard), `window.codaMotion` for motion measured before and after, and the slide of
// every selection host's fill when its current item changes.
function motion() {
  if (window.codaMotion) return;
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

// Installs, once per page, every menu's keys, its focus on opening and its closing on a choice.
// The listeners sit on the document, so a menu added later needs nothing more.
function menuKeys() {
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
      let next = { ArrowDown: now + 1, ArrowUp: now < 0 ? all.length - 1 : now - 1, Home: 0, End: all.length - 1 }[event.key];
      if (next === undefined && event.key.length === 1 && event.key !== " ") {
        const letter = event.key.toLocaleLowerCase();
        const after = [...all.slice(now + 1), ...all.slice(0, now + 1)];
        const found = after.find((choice) => choice.textContent.trim().toLocaleLowerCase().startsWith(letter));
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
}

// Installs, once per page, every tooltip's showing and leaving. A rest of half a second shows one;
// within a moment of another leaving, the next shows at once. The pointer can cross to the tooltip
// and rest on it: leaving the control waits a moment for it.
function tooltips() {
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
}

// One context menu host's ways in. `hostId` is its id; its menu's is its `data-menu`, read each
// time it opens. A menu opened at a point keeps that place until it next opens from its own
// control. Lifting uses one layer per page, `window.codaLift`, which blurs the page beneath; its
// motion runs on the springs and simply arrives where motion is immediate.
function contextHost(hostId) {
  const host = document.getElementById(hostId);
  if (!host || host.dataset.contextMenu) return;
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

// Ends context menu host `hostId`: a press or opening still pending never lands, and its lifted
// copy, and a menu it opened, close.
function disposeContextHost(hostId) {
  window.codaContextHosts?.[hostId]?.dispose();
}

// A list the server reorders: each row it keeps slides from where it stood to where it now stands,
// its current item carrying the selection fill. A row that rises past others is lifted while it
// moves, so it passes over them rather than through them. Rows that only join or leave move nothing
// here; their own frames open and close. The frames of list `id` are its children, in the order of
// `keys`.
function reorder(id, keys) {
  const list = document.getElementById(id);
  if (list !== null) {
    const frames = [...list.children].filter((child) => child.classList.contains("row-frame"));
    const tops = new Map(keys.map((key, index) => [key, frames[index]?.offsetTop ?? 0]));
    const before = list.codaShownRows ?? { keys: [], tops: new Map() };
    list.codaShownRows = { keys, tops };
    const motion = window.codaMotion;
    const kept = keys.filter((key) => before.tops.has(key));
    if (motion !== undefined && kept.join("\n") !== before.keys.filter((key) => tops.has(key)).join("\n")) {
      const selected = motion.currentItem(list);
      keys.forEach((key, index) => {
        const from = before.tops.get(key);
        const to = tops.get(key);
        const frame = frames[index];
        if (from === undefined || frame === undefined || from === to) return;
        const shift = [{ translate: `0 ${from - to}px` }, {}];
        const moving = motion.animate(frame, shift, "step", "emphasized");
        if (moving !== undefined && from > to) {
          frame.classList.add("is-lifted");
          moving.finished.catch(() => undefined).then(() => frame.classList.remove("is-lifted"));
        }
        // The current item carries the fill itself while it moves, so the two never part.
        if (moving !== undefined && selected !== null && frame.contains(selected)) {
          selected.classList.add("is-carrying-selection");
          moving.finished.catch(() => undefined).then(() => selected.classList.remove("is-carrying-selection"));
        }
      });
    }
  }
}

// Answers true once frame `id`'s collapse has finished, or at once with none to run: under reduced
// motion. A collapse cancelled by the row coming back answers false; one cancelled while the row
// still leaves is watched again.
function watchExit(id) {
  return new Promise((report) => watchFrameExit(id, report));
}

function watchFrameExit(id, report) {
  const watch = () => requestAnimationFrame(() => {
    const frame = document.getElementById(id);
    const collapsing = frame ? frame.getAnimations().filter((animation) => animation.transitionProperty === "grid-template-rows") : [];
    if (collapsing.length === 0) { report(true); return; }
    Promise.allSettled(collapsing.map((animation) => animation.finished)).then((settled) => {
      if (settled.every((result) => result.status === "fulfilled")) report(true);
      // A collapse cancelled while the row still leaves (motion became immediate) is watched again.
      else if (frame.classList.contains("is-exiting")) watch();
      else report(false);
    });
  });
  watch();
}

// Opens a reading surface's own scrolling body, `bodyId`, at its top, or at the passage or field
// marked in it when the mark would be out of sight there, once the surface has its size: only that
// body moves, never the page beneath. What stands above the record stays in view when it can.
function scrollToPassage(bodyId) {
  requestAnimationFrame(() => requestAnimationFrame(() => {
    const body = document.getElementById(bodyId);
    if (!body) return;
    body.scrollTop = 0;
    const passage = body.querySelector(".source-text mark, .record-fields mark");
    if (!passage) return;
    const shown = body.getBoundingClientRect();
    const at = passage.getBoundingClientRect();
    if (at.bottom > shown.bottom) body.scrollTop += at.top - shown.top;
  }));
}

// Holds the reader's place in conversation `scrollerId` while it is watched, in place of the browser's own anchoring.
// At the end (within 48px of it), the reader stays at the end; scrolled back, the first turn in sight stays where it
// stood, whatever changes the size of the region or of its content. The turns are the children of its one child.
// Content that appears at once at the end, such as an answer or an ending, is not followed in one frame: the region
// eases to the new end, closing the same share of the distance each frame so most of it is covered within
// `--motion-arrive`, and what was in sight moves with the new content's own entry instead of jumping. While motion is
// immediate (reduced motion, or driving by keyboard) it sets the end at once. The positions it sets itself are not
// read as the reader's scrolling; a wheel, touch or key that moves back stops the easing at once.
// Reports whether the reader is at the end, now and on each change. Two frames after it starts the region is marked
// settled, so what stood there from the start never runs an entry. Answers how to stop.
function watchReading(scrollerId, send) {
  const scroller = document.getElementById(scrollerId);
  const content = scroller?.firstElementChild;
  if (!scroller || !content) throw new Error(`no conversation ${scrollerId}`);
  let place = { end: true };
  let reported;
  // The position this watch set last, so its own scrolling isn't read as the reader's.
  let ours = null;
  let easing = 0;
  const pace = parseFloat(getComputedStyle(scroller).getPropertyValue('--motion-arrive')) / 4;
  const edge = () => scroller.getBoundingClientRect().top;
  const end = () => scroller.scrollHeight - scroller.clientHeight;
  const set = (top) => { scroller.scrollTop = top; ours = scroller.scrollTop; };
  const immediate = () => !('settled' in scroller.dataset) || !(pace > 0) || reducedMotion()
    || document.documentElement.dataset.interaction === 'keyboard';
  const stopEasing = () => { cancelAnimationFrame(easing); easing = 0; };
  const report = () => { if (place.end !== reported) { reported = place.end; send(place.end); } };
  const capture = () => {
    if (ours !== null && Math.abs(scroller.scrollTop - ours) <= 1) return;
    ours = null;
    stopEasing();
    if (end() - scroller.scrollTop <= 48) place = { end: true };
    else {
      const top = edge();
      const turn = [...content.children].find((child) => child.getBoundingClientRect().bottom > top);
      place = { end: false, turn, offset: turn ? turn.getBoundingClientRect().top - top : 0 };
    }
    report();
  };
  const ease = (then) => {
    easing = requestAnimationFrame((now) => {
      const gap = end() - scroller.scrollTop;
      if (gap <= 0.5) { easing = 0; if (gap > 0) set(end()); return; }
      // At least a whole pixel a frame: a screen that places scrolling on whole pixels drops a smaller step.
      set(Math.min(end(), scroller.scrollTop + Math.max(1, gap * (1 - Math.exp(-(now - then) / pace)))));
      ease(now);
    });
  };
  const restore = () => {
    if (!place.end) {
      if (place.turn?.isConnected) set(scroller.scrollTop + place.turn.getBoundingClientRect().top - edge() - place.offset);
    } else if (immediate()) {
      stopEasing();
      set(end());
    } else if (!easing && end() - scroller.scrollTop > 0.5) {
      ease(performance.now());
    }
  };
  // Moving back by wheel, touch or key is the reader's, before any frame of it lands.
  const back = (event) => {
    if (event.type === 'wheel' ? event.deltaY >= 0 : event.type === 'keydown' && !['ArrowUp', 'PageUp', 'Home'].includes(event.key)) return;
    ours = null;
    stopEasing();
  };
  const sizes = new ResizeObserver(restore);
  sizes.observe(scroller);
  sizes.observe(content);
  scroller.addEventListener('scroll', capture, { passive: true });
  for (const kind of ['wheel', 'touchmove', 'keydown']) scroller.addEventListener(kind, back, { passive: true });
  scroller.codaReading = {
    toEnd() { place = { end: true }; restore(); report(); },
    jump(smoothly) {
      const replies = scroller.querySelectorAll('.message-assistant');
      const reply = replies[replies.length - 1];
      if (!reply) return;
      stopEasing();
      ours = null;
      const turn = [...content.children].find((child) => child.contains(reply));
      place = { end: false, turn, offset: turn ? turn.getBoundingClientRect().top - reply.getBoundingClientRect().top : 0 };
      scroller.scrollTo({ top: scroller.scrollTop + reply.getBoundingClientRect().top - edge(), behavior: smoothly ? 'smooth' : 'instant' });
      if (!reply.hasAttribute('tabindex')) reply.setAttribute('tabindex', '-1');
      reply.focus({ preventScroll: true });
    },
  };
  restore();
  report();
  let settling = requestAnimationFrame(() => { settling = requestAnimationFrame(() => { scroller.dataset.settled = ''; }); });
  return () => {
    cancelAnimationFrame(settling);
    stopEasing();
    sizes.disconnect();
    scroller.removeEventListener('scroll', capture);
    for (const kind of ['wheel', 'touchmove', 'keydown']) scroller.removeEventListener(kind, back);
    delete scroller.codaReading;
  };
}

// Brings the reader of conversation `scrollerId` to its end.
function readToEnd(scrollerId) {
  document.getElementById(scrollerId)?.codaReading?.toEnd();
}

// Takes the reader of conversation `scrollerId` to the top of its newest reply, `smoothly` or at once, and focuses it.
function jumpToLatest(scrollerId, smoothly) {
  document.getElementById(scrollerId)?.codaReading?.jump(smoothly);
}

// What the open preview `previewId` hears from the page while it is open, until stopped: the
// window's size now and on every scroll or resize; Escape, taken first so a surrounding dialog keeps
// it, with whether focus was inside the preview; and citation `citationId` going out of sight,
// scroll areas included. Every Dioxus renderer runs it the same way, so the preview depends on no
// one engine's CSS.
function watchPage(citationId, previewId, send) {
  const root = document.documentElement;
  const citation = document.getElementById(citationId);
  const report = () => send({ kind: 'window', width: root.clientWidth, height: root.clientHeight });
  const escape = (event) => {
    if (event.key !== 'Escape') return;
    event.preventDefault();
    event.stopPropagation();
    // Looked up now: this script starts before the preview is in the page.
    const preview = document.getElementById(previewId);
    send({ kind: 'escape', inside: preview !== null && preview.contains(document.activeElement) });
  };
  const sight = new IntersectionObserver(([entry]) => { if (entry !== undefined && !entry.isIntersecting) send({ kind: 'away' }); });
  if (citation !== null) sight.observe(citation);
  addEventListener('scroll', report, true);
  addEventListener('resize', report);
  document.addEventListener('keydown', escape, true);
  report();
  return () => {
    sight.disconnect();
    removeEventListener('scroll', report, true);
    removeEventListener('resize', report);
    document.removeEventListener('keydown', escape, true);
  };
}

// What popover `popoverId`, which control `controlId` opens, needs to be placed, until stopped: as it
// opens, and on every scroll or resize while it is open, the control's box, the popover's own width
// and the window's size; and that it closed. The browser opens and closes it; the control going out
// of sight, scroll areas included, closes it too.
function watchPopover(controlId, popoverId, send) {
  const root = document.documentElement;
  const control = document.getElementById(controlId);
  const popover = document.getElementById(popoverId);
  if (control === null || popover === null) throw new Error(`no popover ${popoverId} opened by ${controlId}`);
  const report = (kind) => {
    const box = control.getBoundingClientRect();
    send({ kind, control: { x: box.left, y: box.top, width: box.width, height: box.height },
      width: popover.offsetWidth, window: [root.clientWidth, root.clientHeight] });
  };
  const moved = () => report('moved');
  const sight = new IntersectionObserver(([entry]) => {
    if (entry !== undefined && !entry.isIntersecting && popover.matches(':popover-open')) popover.hidePopover();
  });
  const unfollow = () => {
    sight.disconnect();
    removeEventListener('scroll', moved, true);
    removeEventListener('resize', moved);
  };
  const toggled = (event) => {
    if (event.newState === 'open') {
      report('opened');
      sight.observe(control);
      addEventListener('scroll', moved, true);
      addEventListener('resize', moved);
    } else {
      unfollow();
      send({ kind: 'closed' });
    }
  };
  popover.addEventListener('toggle', toggled);
  return () => {
    popover.removeEventListener('toggle', toggled);
    unfollow();
  };
}

// Takes only drags that carry files over composer `id`, which only the browser's own list of what
// a drag holds can tell while it moves. Counted, because entering a child leaves its parent.
function fileDrags(id) {
  const form = document.getElementById(id);
  if (form && !form.dataset.takesFiles) {
    form.dataset.takesFiles = 'true';
    let over = 0;
    const files = (event) => form.querySelector(':scope > .composer-drop') && [...(event.dataTransfer?.types ?? [])].includes('Files');
    const settle = () => { over = 0; delete form.dataset.dropping; };
    form.addEventListener('dragenter', (event) => { if (files(event)) { event.preventDefault(); over += 1; form.dataset.dropping = 'true'; } });
    form.addEventListener('dragover', (event) => { if (files(event)) event.preventDefault(); });
    form.addEventListener('dragleave', (event) => { if (files(event) && --over <= 0) settle(); });
    form.addEventListener('drop', (event) => { if (files(event)) { event.preventDefault(); settle(); } });
  }
}

// Sheet `dialog` follows the finger or pointer down from its handle, its header, or its content
// once that is scrolled to the top. Pulled up, it resists, rubber-banding as iOS does. Let go, it is
// projected where the flick would carry it, decelerating as a native scroll view does: past half
// its height it goes, otherwise it springs back. Either way the release carries the flick's speed
// into the step spring (`--spring-step`: a smooth spring, response 0.35, critically damped).
// Without motion it lands at once. A drag that began on a control doesn't also press it.
function sheetDrag(dialog) {
  const sheet = document.getElementById(dialog);
  const body = sheet && sheet.querySelector('.dialog-body');
  if (sheet && body) {
    const stiffness = (2 * Math.PI / 0.35) ** 2, damping = 4 * Math.PI / 0.35;
    // Where a flick at this speed (px/s) would come to rest, at UIScrollView's normal deceleration.
    const projection = 0.998 / (1 - 0.998) / 1000;
    const still = () => matchMedia('(prefers-reduced-motion: reduce)').matches;
    let drag = null, settling = 0;
    const show = (y) => {
      const height = sheet.offsetHeight || 1;
      sheet.style.transform = `translateY(${y}px)`;
      sheet.style.setProperty('--sheet-shown', String(Math.min(1, Math.max(0, 1 - y / height))));
    };
    const rest = () => { sheet.style.removeProperty('transform'); sheet.style.removeProperty('--sheet-shown'); delete sheet.dataset.dragging; };
    const band = (y, height) => y >= 0 ? y : -(1 - 1 / (-y * 0.55 / height + 1)) * height;
    // Where the sheet is drawn now, whether its entry or a spring is moving it.
    const drawn = () => { const t = getComputedStyle(sheet).transform; return t === 'none' ? 0 : new DOMMatrix(t).m42; };
    // Where the hand would be for a sheet already shown at `y`, so taking hold of it never moves it.
    const unband = (y, height) => y >= 0 ? y : -height * (1 / (1 + y / height) - 1) / 0.55;
    const settle = (from, to, speed, done) => {
      cancelAnimationFrame(settling);
      if (still()) { show(to); done(); return; }
      let x = from, v = speed, last = performance.now();
      const step = (now) => {
        let dt = Math.min(0.032, (now - last) / 1000); last = now;
        while (dt > 0) { const h = Math.min(dt, 1 / 240); v += (-stiffness * (x - to) - damping * v) * h; x += v * h; dt -= h; }
        show(x);
        // The spring is critically damped, so it nears its end without passing it.
        const arrived = to > 0 ? x >= to - 0.5 : Math.abs(x - to) < 0.5 && Math.abs(v) < 5;
        if (arrived) { show(to); done(); } else { settling = requestAnimationFrame(step); }
      };
      settling = requestAnimationFrame(step);
    };
    const typing = (node) => node.closest('input, textarea, select, [contenteditable]') !== null;
    sheet.addEventListener('pointerdown', (event) => {
      if (event.button !== 0 || !sheet.open || sheet.classList.contains('is-closing') || typing(event.target)) return;
      const fromContent = body.contains(event.target) && !event.target.closest('.dialog-header');
      drag = { id: event.pointerId, x: event.clientX, y: event.clientY, fromContent, active: false, from: 0, offset: 0, samples: [[event.timeStamp, event.clientY]] };
    });
    sheet.addEventListener('pointermove', (event) => {
      if (!drag || event.pointerId !== drag.id) return;
      const dx = event.clientX - drag.x, dy = event.clientY - drag.y;
      if (!drag.active) {
        if (Math.abs(dy) < 6 || Math.abs(dy) < Math.abs(dx)) return;
        if (drag.fromContent && (body.scrollTop > 0 || dy < 0)) { drag = null; return; }
        drag.active = true;
        // Taken mid-motion, it is held where it is; the movement that started the drag still counts.
        cancelAnimationFrame(settling);
        drag.from = unband(drawn(), sheet.offsetHeight || 1);
        sheet.dataset.dragging = '';
        sheet.setPointerCapture(event.pointerId);
      }
      const height = sheet.offsetHeight || 1;
      drag.offset = band(drag.from + event.clientY - drag.y, height);
      drag.samples.push([event.timeStamp, event.clientY]);
      while (drag.samples.length > 2 && event.timeStamp - drag.samples[0][0] > 100) drag.samples.shift();
      show(drag.offset);
    });
    // A drag is the sheet's, not the page's: on touch the browser would otherwise scroll it away.
    sheet.addEventListener('touchmove', (event) => { if (drag && drag.active) event.preventDefault(); }, { passive: false });
    const release = (event) => {
      if (!drag || event.pointerId !== drag.id) return;
      const ended = drag; drag = null;
      if (!ended.active) return;
      // The click this release would make belongs to the drag; it arrives before any timer runs.
      const swallow = (click) => { click.preventDefault(); click.stopPropagation(); };
      sheet.addEventListener('click', swallow, { capture: true, once: true });
      setTimeout(() => sheet.removeEventListener('click', swallow, { capture: true }), 0);
      // The speed at release, over its last 100ms: a hand held still before letting go has none.
      const recent = [...ended.samples, [event.timeStamp, event.clientY]].filter(([t]) => event.timeStamp - t <= 100);
      const [first, last] = [recent[0], recent[recent.length - 1]];
      const speed = recent.length > 1 && last[0] > first[0] ? (last[1] - first[1]) / ((last[0] - first[0]) / 1000) : 0;
      const height = sheet.offsetHeight || 1;
      const goes = event.type === 'pointerup' && ended.offset > 0 && ended.offset + speed * projection > height / 2;
      if (goes) {
        settle(ended.offset, height, Math.max(speed, 0), () => sheet.close());
      } else {
        settle(ended.offset, 0, ended.offset < 0 ? 0 : speed, rest);
      }
    };
    sheet.addEventListener('pointerup', release);
    sheet.addEventListener('pointercancel', release);
  }
}

// Moves focus off an arrow of switch `id` that has just been disabled.
function handOff(id) {
  const previous = document.getElementById(`${id}-previous`), next = document.getElementById(`${id}-next`);
  const held = document.activeElement;
  if (held === previous && previous.disabled) next.focus();
  if (held === next && next.disabled) previous.focus();
}

// Opens dialog `id` modally once, focusing its `[autofocus]` or else `focus`; its exit motion
// closes it. Answers true once whatever closes it has, or false if it couldn't open.
function modal(id, focus) {
  const d = document.getElementById(id);
  if (!d || d.open) return false;
  return new Promise((report) => {
    d.addEventListener('animationend', (e) => {
      if (e.target === d && e.animationName === 'dialog-exit') d.close();
    });
    d.addEventListener('close', () => report(true), { once: true });
    d.showModal();
    (d.querySelector('[autofocus]') ?? d.querySelector(focus))?.focus({ preventScroll: true });
  });
}

// Writes `text` to the clipboard; a refusal rejects.
function copy(text) {
  return navigator.clipboard.writeText(text);
}

// Answers whether the person asks for reduced motion.
function reducedMotion() {
  return matchMedia('(prefers-reduced-motion: reduce)').matches;
}

// Sets checkbox `id`'s mixed state, which only a property holds.
function indeterminate(id, mixed) {
  const box = document.getElementById(id);
  if (box) box.indeterminate = mixed;
}

// Shows or hides popover `id`.
function togglePopover(id, show) {
  document.getElementById(id)?.togglePopover(show);
}

// Brings the first citation numbered `number` in answer `answerId` into view, then focuses and
// chooses it, so its source opens as that citation opens it and focus returns to it after.
function openCitation(answerId, number) {
  const citation = document.getElementById(answerId)?.querySelector(`button[data-source-number="${number}"]`);
  if (!citation) return;
  citation.scrollIntoView({ block: 'nearest' });
  citation.focus({ preventScroll: true });
  citation.click();
}

// Scrolls element `id` just into view.
function scrollIntoView(id) {
  document.getElementById(id)?.scrollIntoView({ block: 'nearest' });
}

// Focuses element `id`.
function focusElement(id) {
  document.getElementById(id)?.focus();
}

// Brings focus inside region `id` back to its line, `id-line`.
function refocusLine(id) {
  const region = document.getElementById(id);
  if (region && region.contains(document.activeElement)) document.getElementById(`${id}-line`)?.focus();
}

// Puts the caret at the end of the focused field.
function caretToEnd() {
  const field = document.activeElement;
  if (field && field.setSelectionRange) field.setSelectionRange(field.value.length, field.value.length);
}

const fires = {
  motion, menuKeys, tooltips, contextHost, disposeContextHost, reorder, scrollToPassage, fileDrags,
  sheetDrag, handOff, indeterminate, togglePopover, openCitation, scrollIntoView, focusElement,
  refocusLine, caretToEnd, readToEnd, jumpToLatest,
};
const asks = { watchExit, modal, copy, reducedMotion };
const listens = { watchPage, watchPopover, watchReading };

const running = new Map();
let calls = 0;

const ok = (value) => JSON.stringify({ ok: value ?? null });
const err = (error) => JSON.stringify({ err: `${error?.name ?? 'Error'}: ${error?.message ?? error}` });
// What reaches `report` once Rust has let go of it is nothing.
const telling = (report) => (words) => { try { report(words); } catch { /* Rust let go. */ } };

// Runs behaviour `name` with `args`, a JSON array, for what it does alone.
export function fire(name, args) {
  try {
    fires[name](...JSON.parse(args));
  } catch (error) {
    console.error(`coda ui: ${name}`, error);
  }
}

// Runs behaviour `name` with `args`, a JSON array; `report` hears its answer, or why it threw or
// rejected, exactly once.
export function ask(name, args, report) {
  const tell = telling(report);
  let answer;
  try {
    answer = asks[name](...JSON.parse(args));
  } catch (error) {
    answer = Promise.reject(error);
  }
  Promise.resolve(answer).then((value) => tell(ok(value)), (error) => tell(err(error)));
}

// Runs behaviour `name` with `args`, a JSON array; `report` hears each thing it says, or why it
// threw. Answers the handle `stop` takes.
export function listen(name, args, report) {
  const tell = telling(report);
  try {
    const stops = listens[name](...JSON.parse(args), (value) => tell(ok(value)));
    calls += 1;
    running.set(calls, stops);
    return calls;
  } catch (error) {
    tell(err(error));
    return 0;
  }
}

// Stops the behaviour that call `handle` started.
export function stop(handle) {
  running.get(handle)?.();
  running.delete(handle);
}

//! Progressive enhancement only. The server's HTML and paintings stand alone.
//! A single page-lifetime clock pauses offscreen/hidden stages. No renderer,
//! network requests, application state, or Dioxus is linked into this island.
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

fn all(root: &Element, selector: &str) -> Vec<Element> {
    root.query_selector_all(selector)
        .ok()
        .map(|nodes| {
            (0..nodes.length())
                .filter_map(|i| nodes.item(i)?.dyn_into().ok())
                .collect()
        })
        .unwrap_or_default()
}
fn one(root: &Element, selector: &str) -> Option<Element> {
    root.query_selector(selector).ok().flatten()
}
fn class(el: &Element, name: &str, on: bool) {
    let _ = el.class_list().toggle_with_force(name, on);
}
fn classes(root: &Element, selector: &str, name: &str, on: bool) {
    for el in all(root, selector) {
        class(&el, name, on);
    }
}
fn style(el: &Element, name: &str, value: &str) {
    if let Some(el) = el.dyn_ref::<HtmlElement>() {
        let _ = el.style().set_property(name, value);
    }
}
fn text(root: &Element, selector: &str, value: &str) {
    if let Some(el) = one(root, selector)
        && el.text_content().as_deref() != Some(value)
    {
        el.set_text_content(Some(value));
    }
}
fn number(el: &Element, name: &str, default: f64) -> f64 {
    el.get_attribute(name)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}
fn visible(el: &Element, height: f64) -> bool {
    let r = el.get_bounding_client_rect();
    r.bottom() > 0.0 && r.top() < height && r.height() > 0.0
}
fn reduced() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches())
}
fn fit(root: &Element) {
    for wrap in all(root, ".stage-wrap") {
        let width = f64::from(wrap.client_width());
        let compact = width < 620.0 && wrap.has_attribute("data-cw");
        let w = number(&wrap, if compact { "data-cw" } else { "data-w" }, 1280.0);
        let h = number(&wrap, if compact { "data-ch" } else { "data-h" }, 720.0);
        class(&wrap, "compact", compact);
        style(&wrap, "aspect-ratio", &format!("{w} / {h}"));
        style(&wrap, "--s", &(width / w).to_string());
    }
    for feats in all(root, ".feats") {
        if let Some(frame) = one(&feats, ".fframe") {
            style(&feats, "--fh", &format!("{}px", frame.client_height()));
        }
    }
}
fn scale(stage: &Element) -> f64 {
    let width = stage
        .dyn_ref::<HtmlElement>()
        .map(|s| s.offset_width())
        .unwrap_or(0);
    if width > 0 {
        stage.get_bounding_client_rect().width() / f64::from(width)
    } else {
        1.0
    }
}
fn anchor(stage: &Element, popup: &Element, cite: &Element) {
    let s = scale(stage).max(0.001);
    let sr = stage.get_bounding_client_rect();
    let cr = cite.get_bounding_client_rect();
    let max = (sr.width() - popup.get_bounding_client_rect().width()) / s - 12.0;
    let x = ((cr.left() - sr.left()) / s - 20.0)
        .max(12.0)
        .min(max.max(12.0));
    style(popup, "left", &format!("{x}px"));
    style(
        popup,
        "top",
        &format!("{}px", (cr.bottom() - sr.top()) / s + 8.0),
    );
}
fn point(stage: &Element, target: Option<Element>) {
    if let Some(target) = target {
        let s = scale(stage).max(0.001);
        let sr = stage.get_bounding_client_rect();
        let r = target.get_bounding_client_rect();
        if let Some(x) = one(stage, ".ptr-x") {
            style(
                &x,
                "transform",
                &format!(
                    "translateX({}px)",
                    (r.left() - sr.left() + r.width() / 2.0) / s
                ),
            );
        }
        if let Some(y) = one(stage, ".ptr-y") {
            style(
                &y,
                "transform",
                &format!(
                    "translateY({}px)",
                    (r.top() - sr.top() + r.height() / 2.0) / s
                ),
            );
        }
    }
}
fn conversation(root: &Element, t: f64, static_view: bool) {
    classes(root, ".a-msg.me", "on", static_view || t >= 0.0);
    classes(
        root,
        ".a-think",
        "on",
        !static_view && (800.0..3400.0).contains(&t),
    );
    classes(root, ".a-msg.coda", "on", static_view || t >= 3400.0);
    for (i, cite) in all(root, ".a-cite").iter().enumerate() {
        class(
            cite,
            "hot",
            !static_view && t >= 4100.0 + i as f64 * 550.0 && t < 4650.0 + i as f64 * 550.0,
        );
    }
}
struct Hero {
    root: Element,
    question: String,
}
impl Hero {
    fn new(root: Element) -> Self {
        let question = one(&root, ".hero-disc .a-msg.me")
            .and_then(|e| e.text_content())
            .unwrap_or_default();
        Self { root, question }
    }
    fn draw(&self, t: f64, static_view: bool) {
        let r = &self.root;
        for (i, row) in all(r, ".a-pre .a-row").iter().enumerate() {
            class(row, "in", static_view || t >= 400.0 + i as f64 * 140.0);
        }
        classes(r, "[data-k='maya']", "sel", static_view || t >= 2960.0);
        class(r, "s-read", static_view || t >= 3310.0);
        class(r, "drawn", static_view || t >= 3910.0);
        class(r, "s-disc", !static_view && t >= 10710.0);
        let citation = if static_view || (7910.0..9410.0).contains(&t) {
            Some(2)
        } else if (5510.0..7010.0).contains(&t) {
            Some(0)
        } else {
            None
        };
        let cites = all(r, ".hero-reader .a-cite");
        for (i, cite) in cites.iter().enumerate() {
            class(cite, "hot", citation == Some(i));
        }
        for (sel, index) in [(".sat-s1", 0), (".sat-s3", 2)] {
            if let Some(pop) = one(r, sel) {
                class(&pop, "on", citation == Some(index));
                if let Some(cite) = cites.get(index) {
                    anchor(r, &pop, cite);
                }
            }
        }
        classes(
            r,
            "[data-k='discuss']",
            "hot",
            !static_view && (10410.0..10710.0).contains(&t),
        );
        classes(
            r,
            ".pointer",
            "show",
            !static_view && (1860.0..12460.0).contains(&t),
        );
        classes(
            r,
            ".pointer",
            "click",
            !static_view
                && ((2960.0..3310.0).contains(&t)
                    || (10410.0..10710.0).contains(&t)
                    || (12210.0..12460.0).contains(&t)),
        );
        let target = if t < 4610.0 {
            one(r, "[data-k='maya'] p")
        } else if t < 7010.0 {
            cites.first().cloned()
        } else if t < 9410.0 {
            cites.get(2).cloned()
        } else if t < 11610.0 {
            one(r, "[data-k='discuss']")
        } else {
            one(r, ".a-typed")
        };
        if !static_view {
            point(r, target);
        }
        let duration = typing_duration(&self.question);
        let send_at = 12460.0 + duration + 450.0;
        if let Some(typed) = one(r, ".a-typed") {
            let typing = !static_view && (12460.0..send_at).contains(&t);
            class(&typed, "typing", typing);
            let value = if typing {
                typed_prefix(&self.question, t - 12460.0)
            } else {
                typed.get_attribute("data-ph").unwrap_or_default()
            };
            if typed.text_content().as_deref() != Some(&value) {
                typed.set_text_content(Some(&value));
            }
        }
        classes(
            r,
            ".a-send",
            "ready",
            !static_view && t >= send_at - 450.0 && t < send_at,
        );
        if let Some(disc) = one(r, ".hero-disc") {
            conversation(&disc, if static_view { -1.0 } else { t - send_at }, false);
        }
    }
    fn duration(&self) -> f64 {
        12460.0 + typing_duration(&self.question) + 10000.0
    }
}
fn typing_duration(s: &str) -> f64 {
    s.chars().map(|c| if c == ' ' { 74.0 } else { 34.0 }).sum()
}
fn typed_prefix(s: &str, t: f64) -> String {
    let mut elapsed = 0.0;
    s.chars()
        .take_while(|c| {
            let show = elapsed <= t;
            elapsed += if *c == ' ' { 74.0 } else { 34.0 };
            show
        })
        .collect()
}

// Proleptic Gregorian civil date -> days since 1970. Avoids browser timezone
// dependence and date parsing; HTML contains ISO civil dates, not timestamps.
fn day(s: &str) -> Option<f64> {
    let mut parts = s.split('-');
    let y: i32 = parts.next()?.parse().ok()?;
    let m: i32 = parts.next()?.parse().ok()?;
    let d: i32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    let y = y - i32::from(m <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = m + if m > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    Some(f64::from(
        era * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719468,
    ))
}
fn attr_day(el: &Element, name: &str) -> Option<f64> {
    day(&el.get_attribute(name)?)
}
fn percent(d: f64, a: f64, b: f64) -> f64 {
    ((d - a) / (b - a) * 100.0).clamp(0.0, 100.0)
}
fn ease(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}
struct MemoryRow {
    el: Element,
    value: Element,
    icon: Element,
    now: String,
    source: String,
    baseline: String,
    base_icon: String,
    at: f64,
    event: f64,
    state: u8,
}
struct Memory {
    root: Element,
    rows: Vec<MemoryRow>,
    h0: f64,
    f0: f64,
    f1: f64,
    events: Vec<f64>,
}
impl Memory {
    fn new(root: Element) -> Self {
        let h0 = day("2019-01-01").unwrap();
        let f0 = day("2026-08-01").unwrap();
        let f1 = day("2026-09-15").unwrap();
        for (lane, a, b) in [(".mt-h", h0, f0), (".mt-l", f0, f1)] {
            for el in all(&root, &format!("{lane} .mt-in > [data-d]:not(.mt-cap)")) {
                if let Some(d) = attr_day(&el, "data-d") {
                    style(&el, "--x", &format!("{}%", percent(d, a, b)));
                }
            }
            for el in all(&root, &format!("{lane} .mt-bar, {lane} .mt-span")) {
                let x = percent(attr_day(&el, "data-from").unwrap_or(a), a, b);
                let y = percent(attr_day(&el, "data-to").unwrap_or(b), a, b);
                style(&el, "--x", &format!("{x}%"));
                style(&el, "--w", &format!("{}%", (y - x).max(0.0)));
            }
        }
        let mut rows = Vec::new();
        for el in all(&root, ".mt-row") {
            let (Some(value), Some(icon)) = (one(&el, ".mt-v"), one(&el, ".mt-s")) else {
                continue;
            };
            let event = all(&el, ".mt-l [data-d], .mt-l [data-from]")
                .iter()
                .filter_map(|e| attr_day(e, "data-d").or_else(|| attr_day(e, "data-from")))
                .fold(f0, f64::max);
            let kind = el.get_attribute("data-bic").unwrap_or_default();
            let base_icon = if kind == "clock" {
                "<svg viewBox=\"0 0 24 24\"><circle cx=\"12\" cy=\"12\" r=\"8.5\"/><path d=\"M12 7.5V12l3 2\"/></svg>".into()
            } else {
                all(&root, ".mt-row")
                    .into_iter()
                    .find(|r| r.get_attribute("data-ic").as_deref() == Some(&kind))
                    .and_then(|r| one(&r, ".mt-s"))
                    .map(|i| i.inner_html())
                    .unwrap_or_default()
            };
            rows.push(MemoryRow {
                at: attr_day(&el, "data-bat").unwrap_or(h0),
                baseline: el.get_attribute("data-base").unwrap_or_default(),
                now: value.inner_html(),
                source: icon.inner_html(),
                el,
                value,
                icon,
                base_icon,
                event,
                state: 255,
            });
        }
        let mut events: Vec<_> = rows.iter().map(|r| r.event).collect();
        events.sort_by(f64::total_cmp);
        events.dedup();
        Self {
            root,
            rows,
            h0,
            f0,
            f1,
            events,
        }
    }
    fn draw(&mut self, t: f64, static_view: bool) {
        let hd = if static_view {
            self.f0
        } else {
            self.h0 + (self.f0 - self.h0) * ease((t - 500.0) / 2600.0)
        };
        let mut ld = self.f0;
        let mut cursor = 3500.0;
        let mut reach_phase = None;
        for &event in &self.events {
            let duration = ((event - ld) * 42.0).max(650.0);
            if t < cursor + duration {
                ld += (event - ld) * ease((t - cursor) / duration);
                break;
            }
            ld = event;
            cursor += duration;
            let reach = self
                .rows
                .iter()
                .any(|r| r.event == event && r.el.has_attribute("data-reach"));
            let hold = if reach { 2300.0 } else { 900.0 };
            if reach && t < cursor + hold {
                reach_phase = Some(t - cursor);
            }
            if t < cursor + hold {
                break;
            }
            cursor += hold;
        }
        if ld == *self.events.last().unwrap_or(&self.f0) && t >= cursor {
            ld += (self.f1 - ld) * ease((t - cursor) / 700.0);
        }
        if static_view {
            ld = self.f1;
        }
        self.sweep(".mt-h", hd, self.h0, self.f0, "--hp");
        self.sweep(".mt-l", ld, self.f0, self.f1, "--p");
        if let Some(card) = one(&self.root, ".mt-card") {
            class(
                &card,
                "h-live",
                !static_view && (500.0..3100.0).contains(&t),
            );
            class(&card, "l-live", !static_view && t >= 3500.0 && ld < self.f1);
        }
        let year = 2019 + ((hd - self.h0) / 365.25).floor() as i32;
        text(&self.root, ".mt-hpill", &year.min(2026).to_string());
        let recent = if ld >= self.f1 - 0.5 {
            "Today".into()
        } else if ld - self.f0 < 31.0 {
            format!("Aug {}", (ld - self.f0).round() as i32 + 1)
        } else {
            format!("Sep {}", (ld - self.f0 - 31.0).round() as i32 + 1)
        };
        text(&self.root, ".mt-lpill", &recent);
        text(
            &self.root,
            "[data-w]",
            if static_view || ld >= self.f1 {
                "Sep 15"
            } else if t >= 3500.0 {
                &recent
            } else {
                ""
            },
        );
        for row in &mut self.rows {
            let reached = ld >= row.event
                && (!row.el.has_attribute("data-reach")
                    || static_view
                    || reach_phase.is_none_or(|p| p >= 1400.0));
            let state = if reached {
                2
            } else if hd >= row.at {
                1
            } else {
                0
            };
            if state != row.state {
                class(&row.value, "sk", state == 0);
                class(&row.icon, "off", state == 0);
                if state == 2 {
                    row.value.set_inner_html(&row.now);
                    row.icon.set_inner_html(&row.source);
                } else if state == 1 {
                    row.value.set_text_content(Some(&row.baseline));
                    row.icon.set_inner_html(&row.base_icon);
                }
                row.state = state;
            }
            class(
                &row.el,
                "upd",
                !static_view && state == 2 && (ld - row.event).abs() < 0.05,
            );
        }
        classes(
            &self.root,
            ".k24",
            "glow",
            static_view
                || self
                    .rows
                    .iter()
                    .any(|r| r.el.has_attribute("data-reach") && r.state == 2),
        );
        if let (Some(reach), Some(a), Some(b), Some(card)) = (
            one(&self.root, ".mt-reach"),
            one(&self.root, ".k24"),
            one(&self.root, "[data-reach] .mt-l .mt-dot"),
            one(&self.root, ".mt-card"),
        ) {
            let s = scale(&self.root).max(0.001);
            let ar = a.get_bounding_client_rect();
            let br = b.get_bounding_client_rect();
            let cr = card.get_bounding_client_rect();
            style(
                &reach,
                "left",
                &format!("{}px", (ar.left() + ar.width() / 2.0 - cr.left()) / s),
            );
            style(
                &reach,
                "top",
                &format!("{}px", (ar.top() + ar.height() / 2.0 - cr.top()) / s),
            );
            style(
                &reach,
                "width",
                &format!(
                    "{}px",
                    ((br.left() + br.width() / 2.0 - ar.left() - ar.width() / 2.0) / s).max(0.0)
                ),
            );
            let p = reach_phase.unwrap_or(-1.0);
            style(
                &reach,
                "opacity",
                if (600.0..2000.0).contains(&p) {
                    "1"
                } else {
                    "0"
                },
            );
            style(
                &reach,
                "transform",
                &format!("scaleX({})", ease((p - 600.0) / 800.0)),
            );
        }
    }
    fn sweep(&self, lane: &str, d: f64, a: f64, b: f64, property: &str) {
        let pct = percent(d, a, b);
        if let Some(card) = one(&self.root, ".mt-card") {
            style(&card, property, &pct.to_string());
        }
        for el in all(&self.root, &format!(".mt-row {lane} [data-d]")) {
            class(
                &el,
                "on",
                attr_day(&el, "data-d").is_some_and(|v| v <= d + 0.5),
            );
        }
        for clip in all(&self.root, &format!(".mt-row {lane} .mt-clip")) {
            style(
                &clip,
                "clip-path",
                &format!("inset(0 {}% 0 0)", 100.0 - pct),
            );
        }
        if lane == ".mt-l" {
            for tick in all(&self.root, ".mt-tk") {
                let at = attr_day(&tick, "data-d")
                    .map(|v| percent(v, a, b))
                    .unwrap_or(100.0);
                style(
                    &tick,
                    "opacity",
                    if pct > 0.0 && pct < 100.0 && (pct - at).abs() < 11.0 {
                        "0"
                    } else {
                        "1"
                    },
                );
            }
        }
    }
}

struct Clock {
    elapsed: f64,
    started: bool,
}
impl Clock {
    fn new() -> Self {
        Self {
            elapsed: 0.0,
            started: false,
        }
    }
    fn advance(&mut self, el: &Element, dt: f64, height: f64, duration: f64) -> bool {
        if !visible(el, height) {
            return false;
        }
        let rect = el.get_bounding_client_rect();
        let fraction = (rect.bottom().min(height) - rect.top().max(0.0)) / rect.height();
        let first_screen = web_sys::window()
            .and_then(|w| w.scroll_y().ok())
            .unwrap_or(0.0)
            + rect.top()
            < height;
        if !self.started && fraction < 0.35 && !first_screen {
            return false;
        }
        self.started = true;
        self.elapsed = (self.elapsed + dt) % duration;
        true
    }
}
fn choose(root: &Element, index: usize) {
    let items = all(root, ".fitem");
    let changed = items
        .get(index)
        .is_some_and(|item| !item.class_list().contains("on"));
    for (i, item) in items.iter().enumerate() {
        class(item, "on", i == index);
        let _ = item.set_attribute("aria-pressed", if i == index { "true" } else { "false" });
    }
    for (i, stage) in all(root, ".fstage").iter().enumerate() {
        class(stage, "on", i == index);
    }
    if changed {
        fit(root);
    }
}
fn features(root: &Element, t: f64, static_view: bool) {
    if root.matches(".f-today").unwrap_or(false) {
        classes(root, "[data-k='mary']", "gone", !static_view && t >= 2780.0);
        classes(root, "[data-k='done']", "hot", !static_view && t >= 1800.0);
        classes(
            root,
            "[data-k='done']",
            "press",
            !static_view && (2300.0..2480.0).contains(&t),
        );
        text(
            root,
            ".a-n",
            if !static_view && t >= 2780.0 {
                "2"
            } else {
                "3"
            },
        );
        classes(
            root,
            ".a-also, .a-updates",
            "on",
            static_view || t >= 3880.0,
        );
    } else if root.matches(".f-brief").unwrap_or(false) {
        for (i, li) in all(root, ".a-lines li").iter().enumerate() {
            class(li, "on", static_view || t >= 600.0 + i as f64 * 550.0);
        }
        let cite = all(root, ".a-cite").get(1).cloned();
        if let (Some(cite), Some(pop)) = (cite, one(root, ".a-pop")) {
            class(&cite, "hot", static_view || t >= 3000.0);
            anchor(root, &pop, &cite);
            class(&pop, "on", static_view || t >= 3350.0);
        }
    } else if root.matches(".f-ask").unwrap_or(false) {
        conversation(root, t - 500.0, static_view);
    } else if root.matches(".f-follow").unwrap_or(false) {
        classes(root, ".a-arrive", "on", static_view || t >= 5400.0);
    } else if root.matches(".t-src").unwrap_or(false) {
        if let (Some(cite), Some(pop)) = (one(root, ".a-cite"), one(root, ".a-pop")) {
            class(&cite, "hot", static_view || t >= 1400.0);
            anchor(root, &pop, &cite);
            class(&pop, "on", static_view || t >= 1700.0);
        }
    } else if root.matches(".t-you").unwrap_or(false) {
        let buttons = all(root, ".a-btn");
        for (i, button) in buttons.iter().enumerate() {
            class(
                button,
                "hot",
                !static_view && i == (t / 1600.0) as usize % buttons.len(),
            );
        }
    } else if root.matches(".duet-stage").unwrap_or(false) {
        classes(root, ".ph-stat", "on", static_view || t >= 500.0);
        for (i, msg) in all(root, ".cm").iter().enumerate() {
            class(msg, "on", static_view || t >= 500.0 + i as f64 * 1100.0);
        }
        for (i, ps) in all(root, ".ps").iter().enumerate() {
            class(
                ps,
                "on",
                static_view || t >= if i >= 2 { 500.0 } else { 2700.0 },
            );
        }
        classes(
            root,
            ".ph-attach",
            "on",
            static_view || t >= 500.0 + all(root, ".cm").len() as f64 * 1100.0,
        );
    }
}

thread_local! {static STARTED:std::cell::Cell<bool> = const {std::cell::Cell::new(false)};}
/// Called by the deferred bootstrap after wasm initialization. Safe to call twice.
#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    if STARTED.with(|s| s.replace(true)) {
        return Ok(());
    }
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window missing"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document missing"))?;
    let root = document
        .document_element()
        .ok_or_else(|| JsValue::from_str("document root missing"))?;
    fit(&root);
    let hero = one(&root, "#heroStage").map(Hero::new);
    let mut hc = Clock::new();
    let mut memory = one(&root, "#memStage").map(Memory::new);
    let mut mc = Clock::new();
    let mut stages: Vec<_> = all(
        &root,
        ".f-today, .f-brief, .f-ask, .f-follow, .t-src, .t-you, .duet-stage",
    )
    .into_iter()
    .map(|r| (r, Clock::new()))
    .collect();
    let lists = all(&root, "#physicians .feats");
    for list in &lists {
        choose(list, 0);
        for (i, item) in all(list, ".fitem").iter().enumerate() {
            let list = list.clone();
            let item = item.clone();
            let target = item.clone();
            let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                let _ = list.set_attribute("data-picked", "true");
                choose(&list, i);
                if let Some(w) = web_sys::window()
                    && w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0) > 900.0
                {
                    let rect = item.get_bounding_client_rect();
                    let h = w
                        .inner_height()
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    w.scroll_to_with_x_and_y(
                        0.0,
                        w.scroll_y().unwrap_or(0.0) + rect.top() + rect.height() / 2.0 - h / 2.0,
                    );
                }
            });
            target.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;
            click.forget();
        }
    }
    let ready_root = root.clone();
    let mut last = window.performance().map(|p| p.now()).unwrap_or(0.0);
    let mut old_width = -1.0;
    let mut total = 0.0;
    let mut previous_reduced = None;
    let mut draw = move || {
        let Some(w) = web_sys::window() else { return };
        let now = w.performance().map(|p| p.now()).unwrap_or(last + 50.0);
        let dt = (now - last).clamp(0.0, 100.0);
        last = now;
        // Initialization must draw even in a background document. Later ticks
        // still pause while hidden, including pending preference changes.
        if document.hidden() && previous_reduced.is_some() {
            return;
        }
        let height = w
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(800.0);
        let width = w
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1280.0);
        if width != old_width {
            fit(&root);
            old_width = width;
        }
        let static_view = reduced();
        let changed = previous_reduced != Some(static_view);
        previous_reduced = Some(static_view);
        total += dt;
        for rv in all(&root, ".rv:not(.in)") {
            if visible(&rv, height) {
                class(&rv, "in", true);
            }
        }
        for list in &lists {
            if !visible(list, height) {
                continue;
            }
            let items = all(list, ".fitem");
            if items.is_empty() {
                continue;
            }
            if width > 900.0 {
                let selected = items
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        let a = a.get_bounding_client_rect();
                        let b = b.get_bounding_client_rect();
                        (a.top() + a.height() / 2.0 - height / 2.0)
                            .abs()
                            .total_cmp(&(b.top() + b.height() / 2.0 - height / 2.0).abs())
                    })
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                choose(list, selected);
            } else if !static_view && !list.has_attribute("data-picked") {
                choose(list, (total / 7000.0) as usize % items.len());
            }
        }
        if let Some(hero) = &hero
            && (changed
                || (!static_view && hc.advance(&hero.root, dt, height, hero.duration() + 1400.0)))
        {
            hero.draw(hc.elapsed.min(hero.duration()), static_view);
            class(
                &hero.root,
                "motion-reset",
                !static_view && hc.elapsed > hero.duration(),
            );
        }
        if let Some(memory) = &mut memory
            && (changed || (!static_view && mc.advance(&memory.root, dt, height, 21000.0)))
        {
            memory.draw(mc.elapsed.min(19600.0), static_view);
            class(
                &memory.root,
                "motion-reset",
                !static_view && mc.elapsed > 19600.0,
            );
        }
        for (stage, clock) in &mut stages {
            let active = stage
                .closest(".fstage")
                .ok()
                .flatten()
                .is_none_or(|s| s.class_list().contains("on"));
            if changed || (!static_view && active && clock.advance(stage, dt, height, 14000.0)) {
                features(stage, clock.elapsed.min(12600.0), static_view);
                class(
                    stage,
                    "motion-reset",
                    !static_view && clock.elapsed > 12600.0,
                );
            }
        }
        if !static_view {
            for (i, line) in all(&root, "[data-steps]").iter().enumerate() {
                if visible(line, height) {
                    let steps = line.get_attribute("data-steps").unwrap_or_default();
                    let steps: Vec<_> = steps.split('|').collect();
                    let index = ((total + i as f64 * 900.0) / (2600.0 + (i % 3) as f64 * 400.0))
                        as usize
                        % steps.len();
                    if line.text_content().as_deref() != Some(steps[index]) {
                        line.set_text_content(Some(steps[index]));
                    }
                }
            }
        }
    };
    // Publish readiness only after every stage has its initial frame. Sharing
    // this draw with the timer also keeps preference changes on the same path.
    draw();
    let callback = Closure::<dyn FnMut()>::new(draw);
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        50,
    )?;
    class(&ready_root, "motion-ready", true);
    callback.forget();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dates_are_timezone_independent() {
        assert_eq!(day("1970-01-01"), Some(0.0));
        assert_eq!(
            day("2026-09-15").unwrap() - day("2026-08-01").unwrap(),
            45.0
        );
        assert_eq!(day("2024-03-01").unwrap() - day("2024-02-28").unwrap(), 2.0);
        assert_eq!(day("bad"), None);
    }
    #[test]
    fn typing_preserves_unicode_boundaries() {
        assert_eq!(typed_prefix("Hi é", 0.0), "H");
        assert_eq!(typed_prefix("Hi é", 68.0), "Hi ");
        assert_eq!(typed_prefix("Hi é", 142.0), "Hi é");
        assert_eq!(typing_duration("Hi é"), 176.0);
    }
    #[test]
    fn sweep_clamps_and_eases() {
        assert_eq!(percent(-1.0, 0.0, 10.0), 0.0);
        assert_eq!(percent(11.0, 0.0, 10.0), 100.0);
        assert_eq!(ease(0.5), 0.5);
        assert_eq!(ease(2.0), 1.0);
    }
}

//! Measurements and their charts: a value with its units, one
//! observation against its comparison, a series' recent shape, and a series in full. Charts draw
//! in the data colour, never the action's, and a colour never says whether a value is normal. Every
//! chart carries its values in words for assistive technology; the drawing is hidden from it.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName};

/// A measurement's figure, each number with its unit beside it in body text: `6 hr 18 min`.
/// Large on a measurement's own page; medium in a tile or a card.
#[component]
pub fn Value(parts: Vec<(String, String)>, #[props(default)] large: bool) -> Element {
    rsx! {
        span { class: if large { "value value-lg" } else { "value" },
            for (index, (figure, unit)) in parts.into_iter().enumerate() {
                span { key: "{index}", class: "value-part",
                    "{figure}"
                    if !unit.is_empty() {
                        span { class: "value-unit", "{unit}" }
                    }
                }
            }
        }
    }
}

/// One row of a comparison: what it measures, its value in words, and its share of the longest bar.
#[derive(Clone, Debug, PartialEq)]
pub struct Comparison {
    /// What the row measures: "Last night", "Previous 7 nights".
    pub label: String,
    /// The value in words: "6 hr 18 min".
    pub value: String,
    /// The bar's length as a share of the longest, 0 to 1.
    pub fraction: f64,
}

/// One observation against what it is compared with, as bars: the first row is the observation,
/// in the data colour; the rest are its comparisons, in the data track.
#[component]
pub fn ComparisonBars(rows: Vec<Comparison>) -> Element {
    rsx! {
        div { class: "comparison",
            for (index, row) in rows.into_iter().enumerate() {
                div { key: "{index}", class: if index == 0 { "comparison-row is-observed" } else { "comparison-row" },
                    p { class: "chart-caption",
                        span { "{row.label}" }
                        span { "{row.value}" }
                    }
                    span { class: "comparison-track", aria_hidden: "true",
                        span { class: "comparison-bar", style: "inline-size: {share(row.fraction)}%" }
                    }
                }
            }
        }
    }
}

/// A series' recent shape at a glance, ending on its latest point. `label` says the series in words.
#[component]
pub fn Sparkline(points: Vec<f64>, label: String) -> Element {
    let at = plot(&points, 100.0, 100.0, 10.0);
    let line = path(&at);
    rsx! {
        span { class: "sparkline", role: "img", aria_label: "{label}",
            svg { view_box: "0 0 100 100", preserve_aspect_ratio: "none", "aria-hidden": "true",
                path { class: "chart-line", d: "{line}" }
            }
            if let Some((x, y)) = at.last() {
                span { class: "chart-point", aria_hidden: "true", style: "left: {x}%; top: {y}%" }
            }
        }
    }
}

/// How a series is drawn: amounts as bars from zero,
/// levels as a line on their own range.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SeriesShape {
    /// An amount per bucket, a bar from zero.
    #[default]
    Bars,
    /// A level per bucket, a line on its own range.
    Line,
}

/// One bucket of a series (a day, an hour, a month): its value, or none where nothing was
/// recorded, where a line breaks rather than imply one; its low and high where the bucket holds a
/// spread; its word on the axis, if any; its value in words; and whether it is still filling.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct SeriesPoint {
    /// The bucket's value; none where nothing was recorded.
    pub value: Option<f64>,
    /// The lowest reading in the bucket, where it holds a spread.
    pub low: Option<f64>,
    /// The highest reading in the bucket, where it holds a spread.
    pub high: Option<f64>,
    /// The bucket's word under the plot; empty for none.
    pub axis: String,
    /// The bucket and its value in words, as assistive technology and a pinned mark say it.
    pub words: String,
    /// Still filling (today, this week): drawn between quiet and full.
    pub partial: bool,
}

/// A series across its buckets. The
/// emphasized buckets (the latest, unless given) are drawn at full strength and the rest quieter,
/// a filling bucket between the two; a dashed reference marks what the series is compared with,
/// named by a short word; the value axis stands on the trailing edge in round steps. With
/// `on_select`, a mark is pinned by a tap and cleared by tapping it again or empty plot, and the
/// arrow keys move it: the pinned bucket is drawn full, the rest quiet, under a scrub rule. Each
/// recorded bucket's words reach assistive technology, through its mark or, without `on_select`, a
/// list. The host owns what is selected and every word; `label` says the series in words.
#[component]
pub fn SeriesChart(
    #[props(default)] shape: SeriesShape,
    points: Vec<SeriesPoint>,
    label: String,
    #[props(default)] emphasized: Option<(usize, usize)>,
    #[props(default)] reference: Option<f64>,
    #[props(default)] reference_label: String,
    #[props(default)] tick_words: Option<Callback<f64, String>>,
    #[props(default)] selected: Option<usize>,
    on_select: Option<EventHandler<Option<usize>>>,
    #[props(default = true)] axes: bool,
) -> Element {
    let id = use_hook(|| format!("series-{}", SERIES.fetch_add(1, Ordering::Relaxed)));
    let count = points.len();
    let domain = series_domain(shape, &points, reference);
    let words = |value: f64| tick_words.map_or_else(|| compact(value), |words| words.call(value));
    let ticks = if axes {
        axis_ticks(domain, &words)
    } else {
        Vec::new()
    };
    let height = |value: f64| share((value - domain.0) / (domain.1 - domain.0));
    let centre = |index: usize| round((index as f64 + 0.5) / count.max(1) as f64 * 100.0);
    let latest = points.iter().rposition(|point| point.value.is_some());
    let emphasis = |index: usize| match emphasized {
        Some((first, last)) => (first..=last).contains(&index),
        None => Some(index) == latest,
    };
    let selected =
        selected.filter(|&index| points.get(index).is_some_and(|point| point.value.is_some()));
    let every_emphasized = (0..count).all(emphasis);
    let weight = |index: usize, point: &SeriesPoint| match selected {
        Some(chosen) if chosen == index => "is-full",
        Some(_) => "is-quiet",
        None if !emphasis(index) => "is-quiet",
        None if point.partial => "is-partial",
        None => "is-full",
    };
    let narrow = count <= 7 && axes;
    let segments = runs(&points, |point| point.value.is_some());
    let spreads = runs(&points, |point| point.low.is_some() && point.high.is_some());
    let mut class = String::from("series-chart");
    class.push_str(match shape {
        SeriesShape::Bars => " is-bars",
        SeriesShape::Line => " is-line",
    });
    if narrow {
        class.push_str(" is-narrow");
    }
    if !axes {
        class.push_str(" is-bare");
    }
    let interactive = on_select.is_some();
    // One stop for the keyboard: the pinned bucket, else the latest.
    let stop = selected.or(latest);
    let choose = move |next: Option<usize>| {
        if let Some(on_select) = on_select {
            on_select.call(next);
        }
    };
    let neighbour = {
        let values: Vec<bool> = points.iter().map(|point| point.value.is_some()).collect();
        move |from: usize, forward: bool| {
            if forward {
                (from + 1..values.len()).find(|&index| values[index])
            } else {
                (0..from).rev().find(|&index| values[index])
            }
        }
    };
    let focus_id = id.clone();
    rsx! {
        figure { class: "{class}", style: "--series-count: {count}",
            div {
                class: "series-plot",
                role: if interactive { "group" } else { "img" },
                aria_label: "{label}",
                onclick: move |_| {
                    if selected.is_some() {
                        choose(None);
                    }
                },
                for tick in ticks.iter() {
                    span { key: "grid-{tick}", class: "series-gridline", aria_hidden: "true", style: "bottom: {height(*tick)}%" }
                }
                if let Some(value) = reference {
                    span { class: "series-reference", aria_hidden: "true", style: "bottom: {height(value)}%",
                        if !reference_label.is_empty() {
                            span { class: "series-reference-word", "{reference_label}" }
                        }
                    }
                }
                match shape {
                    SeriesShape::Bars => rsx! {
                        for (index, point) in points.iter().enumerate() {
                            if let Some(value) = point.value {
                                span {
                                    key: "bar-{index}",
                                    class: "series-bar {weight(index, point)}",
                                    aria_hidden: "true",
                                    style: "left: {centre(index)}%; height: {height(value)}%",
                                }
                            }
                        }
                    },
                    SeriesShape::Line => rsx! {
                        svg {
                            class: if every_emphasized || selected.is_some() { "series-line" } else { "series-line is-quiet" },
                            view_box: "0 0 100 100",
                            preserve_aspect_ratio: "none",
                            "aria-hidden": "true",
                            for (index, spread) in spreads.iter().enumerate() {
                                if spread.len() > 1 {
                                    path { key: "band-{index}", class: "series-band", d: "{band_path(&points, spread, &centre, &height)}" }
                                }
                            }
                            for (index, segment) in segments.iter().enumerate() {
                                path {
                                    key: "line-{index}",
                                    class: "series-stroke",
                                    d: "{line_path(&points, segment, &centre, &height)}",
                                }
                            }
                        }
                        for (index, point) in points.iter().enumerate() {
                            if let Some(value) = point.value {
                                span {
                                    key: "dot-{index}",
                                    class: "series-dot {weight(index, point)}",
                                    aria_hidden: "true",
                                    style: "left: {centre(index)}%; top: {round(100.0 - height(value))}%",
                                }
                            }
                        }
                    },
                }
                if let Some(index) = selected {
                    span { class: "series-scrub", aria_hidden: "true", style: "left: {centre(index)}%" }
                    if shape == SeriesShape::Line {
                        if let Some(value) = points[index].value {
                            span {
                                class: "series-dot is-chosen",
                                aria_hidden: "true",
                                style: "left: {centre(index)}%; top: {round(100.0 - height(value))}%",
                            }
                        }
                    }
                }
                if interactive {
                    for (index, point) in points.iter().enumerate() {
                        if let Some(value) = point.value {
                            button {
                                key: "target-{index}",
                                id: "{id}-{index}",
                                class: "series-target",
                                r#type: "button",
                                tabindex: if stop == Some(index) { "0" } else { "-1" },
                                aria_label: "{point.words}",
                                aria_pressed: if selected == Some(index) { "true" } else { "false" },
                                style: match shape {
                                    SeriesShape::Bars => format!("left: {}%; height: calc({}% + 8px)", centre(index), height(value)),
                                    SeriesShape::Line => format!("left: {}%; top: {}%", centre(index), round(100.0 - height(value))),
                                },
                                onclick: move |event: MouseEvent| {
                                    event.stop_propagation();
                                    choose(if selected == Some(index) { None } else { Some(index) });
                                },
                                onkeydown: {
                                    let neighbour = neighbour.clone();
                                    let focus_id = focus_id.clone();
                                    move |event: KeyboardEvent| {
                                        let next = match event.key() {
                                            Key::ArrowRight => neighbour(index, true),
                                            Key::ArrowLeft => neighbour(index, false),
                                            _ => return,
                                        };
                                        event.prevent_default();
                                        if let Some(next) = next {
                                            choose(Some(next));
                                            document::eval(&format!("document.getElementById('{focus_id}-{next}')?.focus()"));
                                        }
                                    }
                                },
                                if shape == SeriesShape::Line {
                                    span { class: "series-target-reach", aria_hidden: "true" }
                                }
                            }
                        }
                    }
                }
            }
            // Without its marks to step through, the series says each recorded bucket in a list
            // only assistive technology reads.
            if !interactive {
                ul { class: "sr-only",
                    for (index, point) in points.iter().enumerate() {
                        if point.value.is_some() {
                            li { key: "words-{index}", "{point.words}" }
                        }
                    }
                }
            }
            if axes {
                ValueTicks { ticks: ticks.iter().map(|&tick| (height(tick), words(tick))).collect::<Vec<_>>() }
                div { class: "series-axis", aria_hidden: "true",
                    for (index, point) in points.iter().enumerate() {
                        span { key: "axis-{index}", "{point.axis}" }
                    }
                }
            }
        }
    }
}

static SERIES: AtomicUsize = AtomicUsize::new(0);

/// A running total across a span (a day) so far against its usual pace:
/// the total in the data colour, ending in a dot (named by `end_word`, if
/// any) where the span so far ends; the usual total behind it as a quieter dashed curve over a
/// soft wash, across the whole span; both smooth curves that never double back. Each point is
/// `(place, total)`, its place across the span from 0 at the start to 1 at the end; `marks` stand
/// across the span at their places with their words beneath. The host owns the places, the marks
/// and every word; `label` says the chart in words. Without `axes`, a tile's two curves alone.
#[component]
pub fn RunningChart(
    total: Vec<(f64, f64)>,
    #[props(default)] usual: Vec<(f64, f64)>,
    #[props(default)] marks: Vec<(f64, String)>,
    #[props(default)] end_word: String,
    label: String,
    #[props(default)] tick_words: Option<Callback<f64, String>>,
    #[props(default = true)] axes: bool,
) -> Element {
    let highest = total
        .iter()
        .chain(&usual)
        .map(|&(_, value)| value)
        .filter(|value| value.is_finite())
        .fold(0.0, f64::max);
    let domain = amount_domain(highest);
    let words = |value: f64| tick_words.map_or_else(|| compact(value), |words| words.call(value));
    let ticks = if axes {
        axis_ticks(domain, &words)
    } else {
        Vec::new()
    };
    let height = |value: f64| share(value / domain.1);
    let across = |place: f64| round(share(place));
    let at = |points: &[(f64, f64)]| -> Vec<(f64, f64)> {
        points
            .iter()
            .filter(|(place, value)| place.is_finite() && value.is_finite())
            .map(|&(place, value)| (across(place), round(100.0 - height(value))))
            .collect()
    };
    let total_at = at(&total);
    let usual_at = at(&usual);
    let usual_curve = monotone_path(&usual_at);
    let wash = match (usual_at.first(), usual_at.last()) {
        (Some(first), Some(last)) if usual_at.len() > 1 => {
            format!("{usual_curve}L{} 100L{} 100Z", last.0, first.0)
        }
        _ => String::new(),
    };
    let end = total_at.last().copied();
    rsx! {
        figure { class: if axes { "series-chart is-running" } else { "series-chart is-running is-bare" },
            div { class: "series-plot", role: "img", aria_label: "{label}",
                for tick in ticks.iter() {
                    span { key: "grid-{tick}", class: "series-gridline", aria_hidden: "true", style: "bottom: {height(*tick)}%" }
                }
                if axes {
                    for (place, _) in marks.iter() {
                        span { key: "mark-{place}", class: "series-mark", aria_hidden: "true", style: "left: {across(*place)}%" }
                    }
                }
                svg { class: "series-line", view_box: "0 0 100 100", preserve_aspect_ratio: "none", "aria-hidden": "true",
                    path { class: "series-wash", d: "{wash}" }
                    path { class: "series-usual", d: "{usual_curve}" }
                    path { class: "series-stroke", d: "{monotone_path(&total_at)}" }
                }
                if let Some((x, y)) = end {
                    span { class: "series-dot", aria_hidden: "true", style: "left: {x}%; top: {y}%; --place: {x / 100.0}",
                        if !end_word.is_empty() {
                            span { class: "series-end-word", "{end_word}" }
                        }
                    }
                }
            }
            if axes {
                ValueTicks { ticks: ticks.iter().map(|&tick| (height(tick), words(tick))).collect::<Vec<_>>() }
                div { class: "series-axis", aria_hidden: "true",
                    for (place, word) in marks.iter() {
                        if !word.is_empty() {
                            span { key: "axis-{place}", class: "series-axis-word", style: "left: {across(*place)}%", "{word}" }
                        }
                    }
                }
            }
        }
    }
}

/// A chart's value axis on its trailing edge: each tick's words at its height (a percentage from
/// the bottom), held open by the widest so no label reaches back into the plot.
#[component]
fn ValueTicks(ticks: Vec<(f64, String)>) -> Element {
    let widest = ticks
        .iter()
        .map(|(_, words)| words.as_str())
        .max_by_key(|words| words.chars().count())
        .unwrap_or_default()
        .to_string();
    rsx! {
        div { class: "series-ticks", aria_hidden: "true",
            span { class: "series-ticks-width", "{widest}" }
            for (height, words) in ticks.iter() {
                span { key: "tick-{words}", class: "series-tick", style: "bottom: {height}%", "{words}" }
            }
        }
    }
}

/// The ticks a value axis carries across `domain`: round steps, or one round tick for a flat span.
fn axis_ticks(domain: (f64, f64), words: &impl Fn(f64) -> String) -> Vec<f64> {
    value_ticks(domain, 3, words).unwrap_or_else(|| lone_tick(domain).into_iter().collect())
}

/// The values an amount spans: from zero to a tenth above the highest.
fn amount_domain(highest: f64) -> (f64, f64) {
    (0.0, (highest * 1.1).max(1.0))
}

/// The values a series' plot spans: bars from zero to a tenth above the highest; a line around
/// its lowest and highest, with room above and below. The reference is always inside.
fn series_domain(shape: SeriesShape, points: &[SeriesPoint], reference: Option<f64>) -> (f64, f64) {
    let values: Vec<f64> = points
        .iter()
        .flat_map(|point| [point.value, point.low, point.high])
        .flatten()
        .chain(reference)
        .filter(|value| value.is_finite())
        .collect();
    let low = values.iter().copied().fold(f64::INFINITY, f64::min);
    let high = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if values.is_empty() {
        return (0.0, 1.0);
    }
    match shape {
        SeriesShape::Bars => amount_domain(high),
        SeriesShape::Line => {
            let padding = ((high - low) * 0.25).max(high.abs() * 0.02).max(0.5);
            (low - padding, high + padding)
        }
    }
}

/// Value-axis ticks at about `desired` round steps across `domain`:
/// the smallest of 1, 2 or 5 times a power of ten at least the span over
/// `desired`, widened until every tick reads differently. None when no step leaves two ticks.
fn value_ticks(
    domain: (f64, f64),
    desired: usize,
    words: &impl Fn(f64) -> String,
) -> Option<Vec<f64>> {
    let span = domain.1 - domain.0;
    if !(span > 0.0 && span.is_finite()) || desired == 0 {
        return None;
    }
    let rough = span / desired as f64;
    let mut magnitude = 10f64.powf(rough.log10().floor());
    let mut multipliers: Vec<f64> = [1.0, 2.0, 5.0]
        .into_iter()
        .filter(|m| m * magnitude >= rough)
        .collect();
    for _ in 0..12 {
        for multiplier in &multipliers {
            let step = multiplier * magnitude;
            let mut ticks = Vec::new();
            let mut index = (domain.0 / step).ceil();
            while index * step <= domain.1 {
                // A step's multiple, rounded clear of floating-point drift.
                ticks.push((index * step * 1e9).round() / 1e9);
                index += 1.0;
            }
            if ticks.len() < 2 {
                return None;
            }
            let labels: Vec<String> = ticks.iter().map(|&tick| words(tick)).collect();
            if labels
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
                == labels.len()
            {
                return Some(ticks);
            }
        }
        magnitude *= 10.0;
        multipliers = vec![1.0, 2.0, 5.0];
    }
    None
}

/// One round tick inside `domain` when no step leaves two that read differently, as a flat
/// series has: the domain's middle at the coarsest power of ten that stays inside it.
fn lone_tick(domain: (f64, f64)) -> Option<f64> {
    let span = domain.1 - domain.0;
    if !(span > 0.0 && span.is_finite()) {
        return None;
    }
    let middle = (domain.0 + domain.1) / 2.0;
    let mut step = 10f64.powf(span.log10().floor());
    for _ in 0..12 {
        let tick = (middle / step).round() * step;
        if (domain.0..=domain.1).contains(&tick) {
            return Some((tick * 1e9).round() / 1e9);
        }
        step /= 10.0;
    }
    None
}

/// A number as an axis says it: whole below a thousand, then in thousands or millions with at
/// most one decimal, `10K`, `7.5K`, `1.2M`.
fn compact(value: f64) -> String {
    let (scaled, suffix) = match value.abs() {
        size if size >= 1e6 => (value / 1e6, "M"),
        size if size >= 1e3 => (value / 1e3, "K"),
        _ => (value, ""),
    };
    let tenths = (scaled * 10.0).round() / 10.0;
    if tenths.fract() == 0.0 {
        format!("{}{suffix}", tenths as i64)
    } else {
        format!("{tenths}{suffix}")
    }
}

/// The runs of consecutive buckets that `keep` holds: what a line connects, breaking wherever a
/// bucket recorded nothing, and what a spread spans, breaking wherever a bucket lacks either bound.
fn runs(points: &[SeriesPoint], keep: impl Fn(&SeriesPoint) -> bool) -> Vec<Vec<usize>> {
    let mut segments: Vec<Vec<usize>> = Vec::new();
    let mut current = Vec::new();
    for (index, point) in points.iter().enumerate() {
        if keep(point) {
            current.push(index);
        } else if !current.is_empty() {
            segments.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }
    segments
}

fn line_path(
    points: &[SeriesPoint],
    segment: &[usize],
    x: &impl Fn(usize) -> f64,
    y: &impl Fn(f64) -> f64,
) -> String {
    let at: Vec<(f64, f64)> = segment
        .iter()
        .filter_map(|&index| {
            points[index]
                .value
                .map(|value| (x(index), round(100.0 - y(value))))
        })
        .collect();
    if at.len() == 1 {
        // A lone reading is its dot; the line has nothing to join.
        return String::new();
    }
    path(&at)
}

/// The spread behind a line, low to high across a run of buckets that carry both.
fn band_path(
    points: &[SeriesPoint],
    spread: &[usize],
    x: &impl Fn(usize) -> f64,
    y: &impl Fn(f64) -> f64,
) -> String {
    let edge =
        |index: usize, bound: Option<f64>| (x(index), round(100.0 - y(bound.unwrap_or_default())));
    let mut at: Vec<(f64, f64)> = spread
        .iter()
        .map(|&index| edge(index, points[index].high))
        .collect();
    at.extend(
        spread
            .iter()
            .rev()
            .map(|&index| edge(index, points[index].low)),
    );
    format!("{}Z", path(&at))
}

/// A series' readout over its chart: what the figure is, the
/// figure, and the span or bucket it covers, each on one line so the chart beneath never moves as
/// the readout changes. While a mark is pinned (`pinned`) its words take the accent. With paging
/// words, back and forward stand at the trailing edge, each quiet where there is nowhere to go.
#[component]
pub fn ChartHeader(
    label: String,
    value: Vec<(String, String)>,
    caption: String,
    #[props(default)] pinned: bool,
    #[props(default)] previous: String,
    #[props(default)] next: String,
    on_previous: Option<EventHandler<MouseEvent>>,
    on_next: Option<EventHandler<MouseEvent>>,
) -> Element {
    let page = |words: &str, handler: Option<EventHandler<MouseEvent>>, icon: IconName| {
        (!words.trim().is_empty()).then(|| {
            rsx! {
                Button {
                    variant: ButtonVariant::Quiet,
                    icon_only: true,
                    aria_label: "{words}",
                    disabled: handler.is_none(),
                    onclick: move |event| {
                        if let Some(handler) = handler {
                            handler.call(event);
                        }
                    },
                    Icon { name: icon }
                }
            }
        })
    };
    let back = page(&previous, on_previous, IconName::ChevronLeft);
    let forward = page(&next, on_next, IconName::ChevronRight);
    rsx! {
        div { class: "chart-header",
            div { class: "chart-readout",
                span { class: if pinned { "chart-header-label is-pinned" } else { "chart-header-label" }, "{label}" }
                Value { parts: value }
                span { class: "chart-header-caption", "{caption}" }
            }
            if back.is_some() || forward.is_some() {
                div { class: "chart-paging", {back} {forward} }
            }
        }
    }
}

/// A measurement in a tile: what it is, its value, where and when it was read, and its recent
/// shape. A tile that opens the measurement is one button.
#[component]
pub fn MeasurementTile(
    icon: IconName,
    label: String,
    value: Vec<(String, String)>,
    detail: String,
    #[props(default)] points: Vec<f64>,
    #[props(default)] trend: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let body = rsx! {
        span { class: "measurement-tile-label", Icon { name: icon } "{label}" }
        Value { parts: value }
        span { class: "measurement-tile-detail", "{detail}" }
        if !points.is_empty() {
            Sparkline { points, label: trend }
        }
    };
    match onclick {
        Some(onclick) => rsx! {
            button { class: "measurement-tile navigation-row", r#type: "button", onclick: move |event| onclick.call(event), {body} }
        },
        None => rsx! { div { class: "measurement-tile", {body} } },
    }
}

/// A bar's share of the track, in percent, held to the track.
fn share(fraction: f64) -> f64 {
    if fraction.is_finite() {
        (fraction.clamp(0.0, 1.0) * 1000.0).round() / 10.0
    } else {
        0.0
    }
}

/// Where a series' points fall in a `width` by `height` box: first to last across the width, the
/// highest `inset` from the top and the lowest `inset` from the bottom. A flat series runs through
/// the middle, and a single point stands at the end, where the latest point stands.
fn plot(points: &[f64], width: f64, height: f64, inset: f64) -> Vec<(f64, f64)> {
    let (low, high) = points
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &point| {
            (low.min(point), high.max(point))
        });
    let span = high - low;
    let last = points.len().saturating_sub(1);
    points
        .iter()
        .enumerate()
        .map(|(index, &point)| {
            let x = if last == 0 {
                width
            } else {
                width * index as f64 / last as f64
            };
            let y = if span > 0.0 {
                inset + (high - point) / span * (height - 2.0 * inset)
            } else {
                height / 2.0
            };
            (round(x), round(y))
        })
        .collect()
}

fn round(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

fn path(at: &[(f64, f64)]) -> String {
    at.iter()
        .enumerate()
        .map(|(index, (x, y))| format!("{}{x} {y}", if index == 0 { "M" } else { "L" }))
        .collect()
}

/// A smooth curve through points in order across, which never rises or falls between two points
/// beyond them (monotone cubic interpolation): each point's slope is the
/// gentler of its two sides', flat where they disagree or one is flat, drawn as cubic segments.
fn monotone_path(at: &[(f64, f64)]) -> String {
    if at.len() < 3 {
        return if at.len() == 2 {
            path(at)
        } else {
            String::new()
        };
    }
    let secant = |index: usize| {
        let ((x0, y0), (x1, y1)) = (at[index], at[index + 1]);
        if x1 > x0 { (y1 - y0) / (x1 - x0) } else { 0.0 }
    };
    let last = at.len() - 1;
    let mut slopes = vec![0.0; at.len()];
    for index in 1..last {
        let (before, after) = (secant(index - 1), secant(index));
        let (left, right) = (at[index].0 - at[index - 1].0, at[index + 1].0 - at[index].0);
        let weighted = if left + right > 0.0 {
            (before * right + after * left) / (left + right)
        } else {
            0.0
        };
        if before * after > 0.0 {
            slopes[index] =
                before.signum() * 2.0 * before.abs().min(after.abs()).min(0.5 * weighted.abs());
        }
    }
    slopes[0] = (3.0 * secant(0) - slopes[1]) / 2.0;
    slopes[last] = (3.0 * secant(last - 1) - slopes[last - 1]) / 2.0;
    let mut curve = format!("M{} {}", at[0].0, at[0].1);
    for index in 0..last {
        let ((x0, y0), (x1, y1)) = (at[index], at[index + 1]);
        let third = (x1 - x0) / 3.0;
        curve.push_str(&format!(
            "C{} {} {} {} {x1} {y1}",
            round(x0 + third),
            round(y0 + third * slopes[index]),
            round(x1 - third),
            round(y1 - third * slopes[index + 1]),
        ));
    }
    curve
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_series_without_a_pin_still_says_each_recorded_value() {
        fn app() -> Element {
            let point = |value: Option<f64>, words: &str| SeriesPoint {
                value,
                words: words.into(),
                ..SeriesPoint::default()
            };
            rsx! {
                SeriesChart {
                    points: vec![point(Some(6.0), "Monday, 6,000 steps"), point(None, "Tuesday, none"), point(Some(8.0), "Wednesday, 8,000 steps")],
                    label: "Steps each day",
                }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(
            html.contains("Monday, 6,000 steps") && html.contains("Wednesday, 8,000 steps"),
            "{html}"
        );
        assert!(!html.contains("Tuesday, none"), "{html}");
    }

    #[test]
    fn a_series_spans_its_box_and_ends_on_the_latest_point() {
        assert_eq!(
            plot(&[7.0, 6.0, 8.0], 100.0, 100.0, 10.0),
            vec![(0.0, 50.0), (50.0, 90.0), (100.0, 10.0)]
        );
    }

    #[test]
    fn a_flat_or_single_series_still_draws() {
        assert_eq!(
            plot(&[5.0, 5.0], 100.0, 100.0, 10.0),
            vec![(0.0, 50.0), (100.0, 50.0)]
        );
        assert_eq!(plot(&[5.0], 100.0, 100.0, 10.0), vec![(100.0, 50.0)]);
        assert_eq!(path(&plot(&[], 100.0, 100.0, 10.0)), "");
    }

    #[test]
    fn a_bar_stays_on_its_track() {
        assert_eq!(share(0.9), 90.0);
        assert_eq!(share(1.4), 100.0);
        assert_eq!(share(-1.0), 0.0);
        assert_eq!(share(f64::NAN), 0.0);
    }

    #[test]
    fn the_value_axis_takes_round_steps_that_read_differently() {
        let words = |value: f64| compact(value);
        assert_eq!(
            value_ticks((0.0, 11220.0), 3, &words),
            Some(vec![0.0, 5000.0, 10000.0])
        );
        assert_eq!(value_ticks((58.5, 67.5), 3, &words), Some(vec![60.0, 65.0]));
        // Steps that would repeat a label widen until each is different.
        assert_eq!(
            value_ticks((0.0, 1.2), 4, &|value: f64| format!("{}", value.round())),
            Some(vec![0.0, 1.0])
        );
        assert_eq!(value_ticks((5.0, 5.0), 3, &words), None);
    }

    #[test]
    fn an_axis_says_large_numbers_briefly() {
        assert_eq!(compact(0.0), "0");
        assert_eq!(compact(950.0), "950");
        assert_eq!(compact(10000.0), "10K");
        assert_eq!(compact(7500.0), "7.5K");
        assert_eq!(compact(1_240_000.0), "1.2M");
        assert_eq!(compact(62.5), "62.5");
    }

    #[test]
    fn bars_stand_on_zero_and_a_line_keeps_room_around_its_range() {
        let point = |value: f64| SeriesPoint {
            value: Some(value),
            ..SeriesPoint::default()
        };
        let steps = [6200.0, 10200.0].map(point);
        assert_eq!(
            series_domain(SeriesShape::Bars, &steps, Some(7343.0)),
            (0.0, 10200.0 * 1.1)
        );
        let resting = [60.0, 66.0].map(point);
        assert_eq!(
            series_domain(SeriesShape::Line, &resting, None),
            (58.5, 67.5)
        );
    }

    #[test]
    fn a_line_breaks_where_nothing_was_recorded() {
        let points: Vec<SeriesPoint> = [Some(1.0), Some(2.0), None, Some(3.0), None]
            .into_iter()
            .map(|value| SeriesPoint {
                value,
                ..SeriesPoint::default()
            })
            .collect();
        assert_eq!(
            runs(&points, |point| point.value.is_some()),
            vec![vec![0, 1], vec![3]]
        );
    }

    #[test]
    fn a_spread_spans_only_buckets_with_both_bounds() {
        let point = |value: f64, low: Option<f64>, high: Option<f64>| SeriesPoint {
            value: Some(value),
            low,
            high,
            ..SeriesPoint::default()
        };
        let points = [
            point(5.0, Some(4.0), Some(6.0)),
            point(5.0, Some(4.0), Some(7.0)),
            point(5.0, None, None),
            point(5.0, Some(4.0), None),
        ];
        assert_eq!(
            runs(&points, |point| point.low.is_some() && point.high.is_some()),
            vec![vec![0, 1]]
        );
    }

    #[test]
    fn a_flat_series_keeps_one_round_tick() {
        let hours = |value: f64| format!("{}", value.round());
        assert_eq!(value_ticks((5.5, 6.5), 3, &hours), None);
        assert_eq!(lone_tick((5.5, 6.5)), Some(6.0));
        assert_eq!(lone_tick((58.5, 67.5)), Some(63.0));
        assert_eq!(lone_tick((1.0, 1.0)), None);
    }

    #[test]
    fn a_running_curve_never_dips_below_a_flat_start_or_rises_past_its_points() {
        // Flat overnight, a climb, then level again: every control point stays within its segment.
        let at = [
            (0.0, 100.0),
            (25.0, 100.0),
            (40.0, 60.0),
            (60.0, 20.0),
            (75.0, 5.0),
            (100.0, 5.0),
        ];
        let curve = monotone_path(&at);
        let numbers: Vec<f64> = curve
            .split(|c: char| c.is_ascii_alphabetic())
            .filter(|part| !part.is_empty())
            .flat_map(|part| {
                part.split(' ')
                    .map(|n| n.parse::<f64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let segments = numbers[2..].chunks(6);
        assert_eq!(segments.len(), at.len() - 1);
        for (index, segment) in segments.enumerate() {
            let (low, high) = (
                at[index].1.min(at[index + 1].1),
                at[index].1.max(at[index + 1].1),
            );
            for y in [segment[1], segment[3], segment[5]] {
                assert!(
                    (low..=high).contains(&y),
                    "segment {index}: {y} outside {low}..{high}"
                );
            }
        }
        assert_eq!(monotone_path(&at[..2]), "M0 100L25 100");
        assert_eq!(monotone_path(&at[..1]), "");
    }
}

//! Measurements and their charts, after the October 21 Duet mockups: a value with its units, one
//! observation against its comparison, a series' recent shape, and a series in full. Charts draw
//! in the data colour, never the action's, and a colour never says whether a value is normal. Every
//! chart carries its values in words for assistive technology; the drawing is hidden from it.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Icon, IconName};

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

/// How a series is drawn, after SwiftUI Duet's days and detail charts: amounts as bars from zero,
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

/// A series across its buckets, after SwiftUI Duet's `HealthStatDaysChart` and detail chart. The
/// emphasized buckets (the latest, unless given) are drawn at full strength and the rest quieter,
/// a filling bucket between the two; a dashed reference marks what the series is compared with,
/// named by a short word; the value axis stands on the trailing edge in round steps. With
/// `on_select`, a mark is pinned by a tap and cleared by tapping it again or empty plot, and the
/// arrow keys move it: the pinned bucket is drawn full, the rest quiet, under a scrub rule. The host
/// owns what is selected and every word; `label` says the series in words.
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
        value_ticks(domain, 3, &words).unwrap_or_else(|| lone_tick(domain).into_iter().collect())
    } else {
        Vec::new()
    };
    // The widest tick's words hold the axis open, so no label reaches back into the plot.
    let widest = ticks
        .iter()
        .map(|&tick| words(tick))
        .max_by_key(|text| text.chars().count())
        .unwrap_or_default();
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
            if axes {
                div { class: "series-ticks", aria_hidden: "true",
                    span { class: "series-ticks-width", "{widest}" }
                    for tick in ticks.iter() {
                        span { key: "tick-{tick}", class: "series-tick", style: "bottom: {height(*tick)}%", "{words(*tick)}" }
                    }
                }
            }
            if axes {
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
        SeriesShape::Bars => (0.0, (high * 1.1).max(1.0)),
        SeriesShape::Line => {
            let padding = ((high - low) * 0.25).max(high.abs() * 0.02).max(0.5);
            (low - padding, high + padding)
        }
    }
}

/// Value-axis ticks at about `desired` round steps across `domain`, after SwiftUI Duet's
/// `HealthStatAxisMath`: the smallest of 1, 2 or 5 times a power of ten at least the span over
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

#[cfg(test)]
mod tests {
    use super::*;

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
}

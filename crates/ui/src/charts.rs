//! Measurements and their charts, after the October 21 Duet mockups: a value with its units, one
//! observation against its comparison, a series' recent shape, and a series in full. Charts draw
//! in the data colour, never the action's, and a colour never says whether a value is normal. Every
//! chart carries its values in words for assistive technology; the drawing is hidden from it.

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

/// A series in full on a measurement's page: gridlines, the line and its latest point, with the
/// period's labels beneath, first to last. `label` says the series in words.
#[component]
pub fn TrendChart(
    points: Vec<f64>,
    label: String,
    #[props(default)] labels: Vec<String>,
) -> Element {
    const WIDTH: f64 = 320.0;
    const HEIGHT: f64 = 130.0;
    const INSET: f64 = 16.0;
    let at = plot(&points, WIDTH, HEIGHT, INSET);
    let line = path(&at);
    let grid = format!(
        "M0 {INSET}H{WIDTH}M0 {middle}H{WIDTH}M0 {bottom}H{WIDTH}",
        middle = HEIGHT / 2.0,
        bottom = HEIGHT - INSET
    );
    rsx! {
        figure { class: "trend-chart", role: "img", aria_label: "{label}",
            svg { view_box: "0 0 {WIDTH} {HEIGHT}", "aria-hidden": "true",
                path { class: "chart-grid", d: "{grid}" }
                path { class: "chart-line", d: "{line}" }
                if let Some((x, y)) = at.last() {
                    circle { class: "chart-dot", cx: "{x}", cy: "{y}", r: "4" }
                }
            }
            if !labels.is_empty() {
                figcaption { class: "chart-caption", aria_hidden: "true",
                    for (index, text) in labels.into_iter().enumerate() {
                        span { key: "{index}", "{text}" }
                    }
                }
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
}

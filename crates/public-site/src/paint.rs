//! Startup-only port of kytos' Mulberry32 landscape painter.
//! Insert `assets()` into the server asset map, then serve `stylesheet()` with
//! the asset map's hashed URL resolver. No random input, filesystem or browser
//! painting is involved; all eight source frames share immutable SVG assets.
use std::fmt::Write;

pub const LANDSCAPES: &[(&str, u32)] = &[
    ("mist", 7),
    ("sage", 21),
    ("sage", 31),
    ("sage", 32),
    ("sage", 33),
    ("dawn", 5),
    ("deep", 18),
    ("deep", 3),
];

pub struct PaintAsset {
    /// Logical path; the server may rewrite it to a content-addressed URL.
    pub path: String,
    pub content_type: &'static str,
    pub bytes: Vec<u8>,
}

pub fn assets() -> Vec<PaintAsset> {
    LANDSCAPES
        .iter()
        .map(|&(palette, seed)| PaintAsset {
            path: format!("/paint/{palette}-{seed}.svg"),
            content_type: "image/svg+xml",
            bytes: paint(palette, seed).into_bytes(),
        })
        .collect()
}

/// Load after site.css. `url` receives a logical asset path and returns its
/// trusted server-generated (optionally hashed) URL. Backgrounds work without JS.
pub fn stylesheet(mut url: impl FnMut(&str) -> String) -> String {
    let mut css = String::new();
    for &(name, seed) in LANDSCAPES {
        let path = format!("/paint/{name}-{seed}.svg");
        writeln!(
            css,
            "[data-art=\"{name}\"][data-seed=\"{seed}\"]{{--paint:url(\"{}\")}}",
            url(&path)
        )
        .unwrap();
    }
    css
}

struct Rng(u32);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6d2b79f5);
        let a = self.0;
        let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t));
        f64::from(t ^ (t >> 14)) / 4294967296.0
    }
}

struct Palette {
    sky: [&'static str; 4],
    cloud: &'static str,
    ridges: [&'static str; 4],
    water: [&'static str; 2],
    grain: f64,
}
fn palette(name: &str) -> Palette {
    match name {
        "sage" => Palette {
            sky: ["#C3CEC6", "#D8DFD7", "#E9EAE1", "#F1EEE4"],
            cloud: "#FBFBF6",
            ridges: ["#AEBBB3", "#93A69A", "#73897B", "#56705F"],
            water: ["#DCE2DA", "#BFCAC2"],
            grain: 0.22,
        },
        "dawn" => Palette {
            sky: ["#D6D6CB", "#E8E3D5", "#F2E9D9", "#F5EEDF"],
            cloud: "#FFFDF6",
            ridges: ["#C7C6B8", "#A9B2A4", "#889A8B", "#647C6B"],
            water: ["#EDE8DC", "#D3D3C6"],
            grain: 0.20,
        },
        "deep" => Palette {
            sky: ["#4D6457", "#6F8476", "#97A598", "#B4BCAE"],
            cloud: "#C9CFC2",
            ridges: ["#7E9184", "#62786A", "#4A6152", "#33493B"],
            water: ["#8E9E92", "#5D7264"],
            grain: 0.26,
        },
        _ => Palette {
            sky: ["#C9D3CC", "#DEE3DC", "#EEEDE4", "#F3EFE5"],
            cloud: "#FFFFFF",
            ridges: ["#B7C2BC", "#9DADA4", "#7E9387", "#5E7768"],
            water: ["#E3E6DE", "#C7D0CA"],
            grain: 0.20,
        },
    }
}
fn ridge(r: &mut Rng, base: f64, amp: f64, peaks: usize) -> String {
    let points: Vec<_> = (0..peaks)
        .map(|_| {
            (
                r.next() * 1920.0 - 160.0,
                (0.35 + r.next() * 0.65) * amp,
                (0.07 + r.next() * 0.16) * 1600.0,
            )
        })
        .collect();
    let mut d = String::from("M0,1000 ");
    for x in (0..=1600).step_by(8) {
        let xf = f64::from(x);
        let mut y: f64 = 0.0;
        for &(px, h, w) in &points {
            let u = (xf - px) / w;
            y = y.max(h * (-u * u).exp());
        }
        y += ((xf * 0.031 + peaks as f64).sin() * 0.5 + (xf * 0.0123 + amp).sin() * 0.5) * 2.0
            + (r.next() - 0.5);
        write!(d, "L{x},{:.1} ", base - y).unwrap();
    }
    d.push_str("L1600,1000 Z");
    d
}

/// Generate one soft-focus painting. Unknown palettes and seed zero use the
/// source defaults. All arithmetic in the PRNG deliberately wraps at 32 bits.
pub fn paint(name: &str, seed: u32) -> String {
    let p = palette(name);
    let mut r = Rng(if seed == 0 { 1 } else { seed });
    let mut s = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1600 1000\" preserveAspectRatio=\"xMidYMid slice\"><defs><linearGradient id=\"sk\" gradientUnits=\"userSpaceOnUse\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1000\">",
    );
    for (offset, color) in [
        ("0", p.sky[0]),
        (".42", p.sky[1]),
        (".58", p.sky[2]),
        (".64", p.sky[3]),
        (".7", p.water[0]),
        ("1", p.water[1]),
    ] {
        write!(s, "<stop offset=\"{offset}\" stop-color=\"{color}\"/>").unwrap();
    }
    s.push_str("</linearGradient>");
    for (blur, extent, size) in [(40, 20, 140), (14, 10, 120), (7, 10, 120), (24, 10, 120)] {
        write!(s, "<filter id=\"f{blur}\" x=\"-{extent}%\" y=\"-{extent}%\" width=\"{size}%\" height=\"{size}%\"><feGaussianBlur stdDeviation=\"{blur}\"/></filter>").unwrap();
    }
    write!(s, "<filter id=\"grain\"><feTurbulence type=\"fractalNoise\" baseFrequency=\".9\" numOctaves=\"2\" seed=\"{}\" stitchTiles=\"stitch\"/><feColorMatrix values=\"0 0 0 0 .08  0 0 0 0 .12  0 0 0 0 .1  0 0 0 1.2 -.5\"/></filter></defs><rect width=\"1600\" height=\"1000\" fill=\"url(#sk)\"/><g filter=\"url(#f40)\">", seed%31).unwrap();
    for _ in 0..7 {
        write!(s, "<ellipse cx=\"{:.0}\" cy=\"{:.0}\" rx=\"{:.0}\" ry=\"{:.0}\" fill=\"{}\" opacity=\"{:.2}\"/>", r.next()*1600.0, 80.0+r.next()*300.0, 220.0+r.next()*380.0, 50.0+r.next()*80.0, p.cloud, 0.3+r.next()*0.35).unwrap();
    }
    s.push_str("</g>");
    for (i, (base, amp, peaks, blur, opacity)) in [
        (580.0, 300.0, 5, 14, "0.55"),
        (604.0, 210.0, 6, 7, "0.7"),
        (622.0, 110.0, 7, 7, "0.85"),
    ]
    .into_iter()
    .enumerate()
    {
        write!(s, "<path d=\"{}\" fill=\"{}\" opacity=\"{opacity}\" filter=\"url(#f{blur})\"/><rect x=\"-100\" y=\"{}\" width=\"1800\" height=\"90\" fill=\"{}\" opacity=\".35\" filter=\"url(#f24)\"/>", ridge(&mut r,base,amp,peaks),p.ridges[i],base-50.0,p.sky[3]).unwrap();
    }
    write!(s, "<rect x=\"-100\" y=\"620\" width=\"1800\" height=\"400\" fill=\"url(#sk)\" opacity=\".9\" filter=\"url(#f14)\"/><g opacity=\".18\" filter=\"url(#f14)\" transform=\"translate(0,1248) scale(1,-1)\"><path d=\"{}\" fill=\"{}\"/></g><rect x=\"-100\" y=\"594\" width=\"1800\" height=\"60\" fill=\"{}\" opacity=\".55\" filter=\"url(#f14)\"/>",ridge(&mut Rng(seed.wrapping_add(7)),624.0,150.0,6),p.ridges[2],p.sky[3]).unwrap();
    let sx = if r.next() > 0.5 { 0 } else { 1600 };
    write!(s, "<ellipse cx=\"{sx}\" cy=\"1040\" rx=\"{:.0}\" ry=\"{:.0}\" fill=\"{}\" opacity=\".55\" filter=\"url(#f24)\"/><rect width=\"1600\" height=\"1000\" filter=\"url(#grain)\" opacity=\"{:.2}\"/></svg>",560.0+r.next()*200.0,230.0+r.next()*60.0,p.ridges[3],p.grain*0.7).unwrap();
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mulberry32_matches_source_vectors() {
        let mut rng = Rng(7);
        assert_eq!(rng.next(), 0.011704753153026104);
        assert_eq!(rng.next(), 0.06195825757458806);
    }
    #[test]
    fn landscapes_are_repeatable_and_distinct() {
        assert_eq!(paint("mist", 7), paint("mist", 7));
        assert_ne!(paint("sage", 31), paint("sage", 32));
        assert_eq!(paint("invalid", 7), paint("mist", 7));
        assert!(paint("deep", 18).ends_with("</svg>"));
        assert_eq!(assets().len(), 8);
    }
    #[test]
    fn css_uses_resolved_assets_and_original_selectors() {
        let css = stylesheet(|path| format!("/a/hash{path}"));
        assert!(css.contains("[data-art=\"mist\"][data-seed=\"7\"]"));
        assert!(css.contains("/a/hash/paint/mist-7.svg"));
    }
}

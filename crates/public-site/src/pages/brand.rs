//! Coda Health's products as the site presents them: each one's name, mark and wordmark.
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Product {
    /// The physician product.
    Chords,
    /// The patient app.
    Duet,
}

impl Product {
    fn name(self) -> &'static str {
        match self {
            Self::Chords => "Chords",
            Self::Duet => "Duet",
        }
    }

    /// The mark and its intrinsic size.
    fn mark(self) -> (&'static str, u32) {
        match self {
            Self::Chords => ("/brand/chords/mark.svg", 940),
            Self::Duet => ("/brand/duet/mark.svg", 400),
        }
    }

    /// The wordmark and the size it renders at. chrome.css's `product-wordmark`
    /// sizes Duet's; Chords' is wider at the same letter height, so it carries its
    /// own size.
    fn wordmark(self, hero: bool) -> Wordmark {
        match self {
            Self::Chords => Wordmark::Sized {
                src: "/brand/chords/wordmark.svg",
                width: if hero { 158 } else { 113 },
                height: if hero { 41 } else { 29 },
            },
            Self::Duet => Wordmark::Styled {
                src: "/brand/duet/wordmark.svg",
                width: 440,
                height: 170,
            },
        }
    }
}

enum Wordmark {
    /// Rendered at this size by its attributes.
    Sized {
        src: &'static str,
        width: u32,
        height: u32,
    },
    /// Rendered by the `product-wordmark` styles; the attributes are its intrinsic size.
    Styled {
        src: &'static str,
        width: u32,
        height: u32,
    },
}

/// A product's mark and wordmark, with the audience it serves, in the `product`
/// lockup styles. `hero` is the larger lockup that opens a product's own page.
#[component]
pub fn ProductLockup(
    product: Product,
    audience: Option<&'static str>,
    #[props(default)] hero: bool,
) -> Element {
    let (mark, mark_size) = product.mark();
    let class = if hero {
        "product product-hero rv"
    } else {
        "product"
    };
    rsx! {
        div { class,
            span { class: "product-lockup",
                img {
                    alt: "",
                    class: "product-mark",
                    height: "{mark_size}",
                    src: mark,
                    width: "{mark_size}",
                }
                match product.wordmark(hero) {
                    Wordmark::Sized { src, width, height } => rsx! {
                        img {
                            alt: product.name(),
                            class: "product-wordmark-sized",
                            height: "{height}",
                            src,
                            width: "{width}",
                        }
                    },
                    Wordmark::Styled { src, width, height } => rsx! {
                        img {
                            alt: product.name(),
                            class: "product-wordmark",
                            height: "{height}",
                            src,
                            width: "{width}",
                        }
                    },
                }
            }
            if let Some(audience) = audience {
                span { class: "product-label", "{audience}" }
            }
        }
    }
}

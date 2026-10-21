use crate::{Config, PageMeta, Route, site};
use axum::body::Bytes;
use dioxus::prelude::*;
use dioxus_history::{History, MemoryHistory};
use std::{collections::HashMap, rc::Rc};

/// Immutable rendered bytes. The server rewrites asset URLs and prepares all
/// compressed representations/cache validators once, before accepting requests.
#[derive(Clone, Debug)]
pub struct Page {
    pub html: Bytes,
    pub meta: PageMeta,
}

#[derive(Clone, Debug)]
pub struct PageTable {
    pub pages: HashMap<String, Page>,
    pub not_found: Page,
}

/// All Dioxus work happens here, before accepting any HTTP requests.
pub fn prerender(config: &Config) -> PageTable {
    let pages = Route::pages()
        .into_iter()
        .map(|route| (route.to_string(), render_page(&route, config)))
        .collect();
    let not_found = render_page(
        &Route::NotFound {
            segments: vec!["404".into()],
        },
        config,
    );
    PageTable { pages, not_found }
}

/// Render with an explicit per-document memory history, so Router/Outlet select
/// the requested route rather than silently falling back to the homepage.
pub fn render_page(route: &Route, config: &Config) -> Page {
    let meta = route.meta();
    let mut dom = VirtualDom::new_with_props(Document, DocumentProps { meta: meta.clone() });
    let history: Rc<dyn History> = Rc::new(MemoryHistory::with_initial_path(route));
    dom.insert_any_root_context(Box::new(history));
    dom.insert_any_root_context(Box::new(config.clone()));
    dom.rebuild_in_place();
    // Dioxus 0.7's HTML element set omits the outer html element.
    let html = format!(
        "<!DOCTYPE html><html lang=\"en\">{}</html>",
        dioxus_ssr::render(&dom)
    );
    Page {
        html: Bytes::from(html),
        meta,
    }
}

#[component]
fn Document(meta: PageMeta) -> Element {
    rsx! {
            head {
                meta { charset: "utf-8" }
                meta { name: "viewport", content: "width=device-width, initial-scale=1" }
                title { "{meta.title}" }
                meta { name: "description", content: meta.description.clone() }
                if !meta.canonical_url.is_empty() {
                link { rel: "canonical", href: meta.canonical_url.clone() }
                meta { property: "og:title", content: meta.title.clone() }
                meta { property: "og:description", content: meta.description.clone() }
                meta { property: "og:url", content: meta.canonical_url.clone() }
                meta { property: "og:site_name", content: site::SITE_NAME }
                meta { property: "og:type", content: "website" }
                meta { property: "og:image", content: meta.image_url.clone() }
                meta { property: "og:image:width", content: "1200" }
                meta { property: "og:image:height", content: "630" }
                meta { name: "twitter:card", content: "summary_large_image" }
                meta { name: "twitter:title", content: meta.title.clone() }
                meta { name: "twitter:description", content: meta.description.clone() }
                meta { name: "twitter:image", content: meta.image_url.clone() }
                meta { property: "og:image:alt", content: meta.image_alt.clone() }
                meta { name: "twitter:image:alt", content: meta.image_alt.clone() }
                }
                if meta.noindex {
                    meta { name: "robots", content: "noindex, nofollow" }
                }
                link { rel: "icon", href: "/favicon.ico" }
                link { rel: "stylesheet", href: "/css/chrome.css" }
                link { rel: "stylesheet", href: "/css/site.css" }
                link { rel: "stylesheet", href: "/css/utility.css" }
                link { rel: "stylesheet", href: "/css/paint.css" }
                link { rel: "stylesheet", href: "/motion.css" }
            }
            body {
                Router::<Route> {}
                script { r#type: "module", src: "/motion/bootstrap.js" }
            }
    }
}

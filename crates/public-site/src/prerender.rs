use crate::{
    Config, PageMeta, Route,
    legal::{Legal, LegalError},
    route::Shell,
    site,
};
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
    pub invite: Page,
    pub not_found: Page,
}

/// All Dioxus work happens here, before accepting any HTTP requests. The legal records
/// load first, so a malformed one stops startup before anything renders.
pub fn prerender(config: &Config) -> Result<PageTable, LegalError> {
    prerender_with_target(
        config,
        &site::SiteOrigin::default(),
        &site::BasePath::root(),
    )
}

/// Renders all pages resolved against an explicit origin and base path.
pub fn prerender_with_target(
    config: &Config,
    origin: &site::SiteOrigin,
    base_path: &site::BasePath,
) -> Result<PageTable, LegalError> {
    let legal = Rc::new(Legal::load()?);
    let pages = Route::pages(&legal)
        .into_iter()
        .map(|route| {
            (
                route.to_string(),
                render_page_with_target(&route, config, &legal, origin, base_path),
            )
        })
        .collect();
    let invite = render_page_with_target(&Route::Invite {}, config, &legal, origin, base_path);
    let not_found = render_page_with_target(
        &Route::NotFound {
            segments: vec!["404".into()],
        },
        config,
        &legal,
        origin,
        base_path,
    );
    Ok(PageTable {
        pages,
        invite,
        not_found,
    })
}

/// Render with an explicit per-document memory history, so Router/Outlet select
/// the requested route rather than silently falling back to the homepage.
pub fn render_page(route: &Route, config: &Config, legal: &Rc<Legal>) -> Page {
    render_page_with_target(
        route,
        config,
        legal,
        &site::SiteOrigin::default(),
        &site::BasePath::root(),
    )
}

/// Variant of [`render_page`] resolving metadata against an explicit origin and base path.
pub fn render_page_with_target(
    route: &Route,
    config: &Config,
    legal: &Rc<Legal>,
    origin: &site::SiteOrigin,
    base_path: &site::BasePath,
) -> Page {
    let meta = route.meta_for(legal, origin, base_path);
    let mut dom = VirtualDom::new_with_props(
        Document,
        DocumentProps {
            meta: meta.clone(),
            shell: route.shell(),
        },
    );
    let history: Rc<dyn History> = Rc::new(MemoryHistory::with_initial_path(route));
    dom.insert_any_root_context(Box::new(history));
    dom.insert_any_root_context(Box::new(config.clone()));
    dom.insert_any_root_context(Box::new(legal.clone()));
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

/// The document head: page metadata, site icons, and each shell's stylesheets.
#[component]
fn Document(meta: PageMeta, shell: Shell) -> Element {
    rsx! {
        head {
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            title { "{meta.title}" }
            meta { name: "description", content: meta.description.clone() }
            if meta.noindex {
                meta { name: "robots", content: "noindex, nofollow" }
            }
            if !meta.canonical_url.is_empty() {
                link { rel: "canonical", href: meta.canonical_url.clone() }
                meta { property: "og:title", content: meta.title.clone() }
                meta { property: "og:description", content: meta.description.clone() }
                meta { property: "og:url", content: meta.canonical_url.clone() }
                meta { property: "og:site_name", content: site::SITE_NAME }
                meta { property: "og:image", content: meta.image_url.clone() }
                meta { property: "og:image:width", content: "1200" }
                meta { property: "og:image:height", content: "630" }
                meta { property: "og:image:alt", content: meta.image_alt.clone() }
                meta { property: "og:type", content: "website" }
                meta { name: "twitter:card", content: "summary_large_image" }
                meta { name: "twitter:title", content: meta.title.clone() }
                meta { name: "twitter:description", content: meta.description.clone() }
                meta { name: "twitter:image", content: meta.image_url.clone() }
                meta { name: "twitter:image:width", content: "1200" }
                meta { name: "twitter:image:height", content: "630" }
                meta { name: "twitter:image:alt", content: meta.image_alt.clone() }
            }
            link { rel: "icon", href: "/favicon.ico", sizes: "any" }
            link { rel: "icon", href: "/favicon-32x32.png", r#type: "image/png", sizes: "32x32" }
            link { rel: "apple-touch-icon", href: "/apple-touch-icon.png" }
            match shell {
                Shell::Homepage => rsx! {
                    link { rel: "stylesheet", href: "/css/chrome.css" }
                    link { rel: "stylesheet", href: "/css/site.css" }
                    link { rel: "stylesheet", href: "/css/stage.css" }
                    link { rel: "stylesheet", href: "/css/duet.css" }
                    noscript { link { rel: "stylesheet", href: "/css/noscript.css" } }
                },
                Shell::Document => rsx! {
                    link { rel: "stylesheet", href: "/css/chrome.css" }
                    link { rel: "stylesheet", href: "/css/document.css" }
                },
            }
        }
        body {
            Router::<Route> {}
            script { r#type: "module", src: "/chrome.js" }
        }
    }
}

//! Complete embedded asset tree plus startup-generated assets.
use crate::{
    representation::{Representation, digest},
    site::BasePath,
};
use axum::body::Bytes;
use std::{collections::HashMap, sync::Arc};
include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"));

#[derive(Clone, Debug)]
pub struct Asset {
    pub representation: Arc<Representation>,
    pub immutable: bool,
}

#[derive(Clone, Debug, Default)]
pub struct AssetTable {
    pub assets: HashMap<String, Asset>,
    pub aliases: HashMap<String, String>,
}
impl AssetTable {
    /// Add deterministic generated assets before page rewriting. Duplicate URLs
    /// fail startup rather than silently shadowing files.
    pub fn embedded_with(extra: impl IntoIterator<Item = (String, Bytes)>) -> Self {
        Self::embedded_with_base(extra, &BasePath::root())
    }

    /// Variant of [`Self::embedded_with`] whose aliases resolve beneath `base_path`.
    pub fn embedded_with_base(
        extra: impl IntoIterator<Item = (String, Bytes)>,
        base_path: &BasePath,
    ) -> Self {
        let mut raw = HashMap::new();
        for (path, bytes) in EMBEDDED_ASSETS {
            raw.insert((*path).to_owned(), Bytes::from_static(bytes));
        }
        for (path, bytes) in extra {
            assert!(
                path.starts_with('/') && !path.contains(['?', '#']) && !path.starts_with("/a/"),
                "invalid generated asset path"
            );
            assert!(
                raw.insert(path, bytes).is_none(),
                "duplicate generated asset URL"
            );
        }
        let mut table = Self::default();
        // Build referenced images/fonts before stylesheets. Absolute root-relative
        // CSS URLs are rewritten to the same hashed references used by documents.
        let mut paths: Vec<_> = raw.keys().cloned().collect();
        paths.sort_by_key(|path| (path.ends_with(".css"), path.clone()));
        for path in paths {
            let mut bytes = raw.remove(&path).unwrap();
            if path.ends_with(".css") {
                let css = std::str::from_utf8(&bytes).expect("UTF-8 CSS asset");
                bytes = Bytes::from(table.rewrite(css));
            }
            let hashed = format!("/a/{}{path}", digest(&bytes));
            let content_type = mime_guess::from_path(&path)
                .first_or_octet_stream()
                .to_string();
            let representation = Arc::new(Representation::new(bytes, content_type));
            table.assets.insert(
                hashed.clone(),
                Asset {
                    representation: representation.clone(),
                    immutable: true,
                },
            );
            table.assets.insert(
                path.clone(),
                Asset {
                    representation,
                    immutable: false,
                },
            );
            table.aliases.insert(path, base_path.join(&hashed));
        }
        // The social image's public URL is extensionless; serve the embedded PNG there
        // with its correct image content type.
        if let Some(image) = table
            .assets
            .get("/scheduling/dev/opengraph-image.png")
            .cloned()
        {
            table
                .assets
                .insert("/scheduling/dev/opengraph-image".into(), image);
        }
        table
    }

    /// Rewrites root-relative asset URLs and, when mounted under a non-root
    /// `base_path`, non-asset `href="/..."` navigation targets once.
    pub fn rewrite_page(&self, html: &str, base_path: &BasePath) -> String {
        if base_path.is_root() {
            return self.rewrite(html);
        }
        let with_routes = self.rewrite_route_hrefs(html, base_path);
        self.rewrite(&with_routes)
    }

    fn rewrite_route_hrefs(&self, html: &str, base_path: &BasePath) -> String {
        let mut out = String::with_capacity(html.len());
        let mut cursor = 0;
        while let Some(rel) = html[cursor..].find("href=\"/") {
            let prefix_end = cursor + rel + "href=\"".len();
            let Some(quote_rel) = html[prefix_end..].find('"') else {
                break;
            };
            let target_end = prefix_end + quote_rel;
            let target = &html[prefix_end..target_end];
            let path_only = target
                .split(['?', '#'])
                .next()
                .expect("split always yields at least one slice");
            if target.starts_with("//") || self.aliases.contains_key(path_only) {
                out.push_str(&html[cursor..target_end]);
                cursor = target_end;
                continue;
            }
            out.push_str(&html[cursor..prefix_end]);
            out.push_str(&base_path.join(target));
            cursor = target_end;
        }
        out.push_str(&html[cursor..]);
        out
    }

    /// Rewrite only root-relative URL tokens, not substrings of absolute URLs or
    /// similarly-prefixed names. Stable absolute social/brand URLs remain aliases.
    pub fn rewrite(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut start = 0;
        for (index, character) in text.char_indices() {
            if character != '/' {
                continue;
            }
            let before = text[..index].chars().next_back();
            if !matches!(
                before,
                None | Some('"' | '\'' | '(' | '=' | ' ' | '\n' | '\t')
            ) {
                continue;
            }
            let end = text[index..]
                .find(['"', '\'', ')', ' ', '\n', '\t', '?', '#', '<', '>'])
                .map_or(text.len(), |end| index + end);
            if let Some(hashed) = self.aliases.get(&text[index..end]) {
                if index < start {
                    continue;
                }
                result.push_str(&text[start..index]);
                result.push_str(hashed);
                start = end;
            }
        }
        result.push_str(&text[start..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_embedded_asset_has_alias_and_content_address() {
        let table = AssetTable::embedded_with([]);
        assert!(!EMBEDDED_ASSETS.is_empty());
        assert_eq!(table.aliases.len(), EMBEDDED_ASSETS.len());
        for (path, _) in EMBEDDED_ASSETS {
            assert!(!table.assets[*path].immutable);
            assert!(table.assets[&table.aliases[*path]].immutable);
            assert!(Arc::ptr_eq(
                &table.assets[*path].representation,
                &table.assets[&table.aliases[*path]].representation
            ));
        }
    }
    #[test]
    fn rewrites_only_complete_root_relative_urls() {
        let mut table = AssetTable::default();
        table
            .aliases
            .insert("/image.png".into(), "/a/hash/image.png".into());
        assert_eq!(
            table.rewrite(
                "src=\"/image.png?x=1\" url(/image.png) https://example.com/image.png /image.png2"
            ),
            "src=\"/a/hash/image.png?x=1\" url(/a/hash/image.png) https://example.com/image.png /image.png2"
        );
    }

    #[test]
    fn rewrites_page_routes_and_assets_under_subpath_without_touching_text_slashes() {
        let base = BasePath::parse("/coda-rust").unwrap();
        let table = AssetTable::embedded_with_base([], &base);
        let input = r#"<link rel="stylesheet" href="/css/chrome.css"><a href="/">Home</a><a href="/#physicians">Physicians</a><a href="/duet">Duet</a><p>404 / Page not found</p><meta property="og:image" content="https://tyler274.github.io/coda-rust/brand/coda-health-social-preview-v4.png">"#;
        let rewritten = table.rewrite_page(input, &base);
        assert!(rewritten.contains("href=\"/coda-rust/a/"), "{rewritten}");
        assert!(rewritten.contains("/css/chrome.css\""), "{rewritten}");
        assert!(rewritten.contains("href=\"/coda-rust/\""), "{rewritten}");
        assert!(
            rewritten.contains("href=\"/coda-rust/#physicians\""),
            "{rewritten}"
        );
        assert!(
            rewritten.contains("href=\"/coda-rust/duet\""),
            "{rewritten}"
        );
        assert!(
            rewritten.contains("<p>404 / Page not found</p>"),
            "{rewritten}"
        );
        assert!(
            rewritten.contains("content=\"https://tyler274.github.io/coda-rust/brand/coda-health-social-preview-v4.png\""),
            "{rewritten}"
        );
        assert!(!rewritten.contains("/coda-rust/coda-rust"), "{rewritten}");
    }
}

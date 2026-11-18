//! Static site exporter for subpath-hosted deployments (such as GitHub Pages).

use crate::{
    Config, Route,
    assets::AssetTable,
    config::ExportConfig,
    legal::{Legal, LegalError},
    prerender::prerender_with_target,
    site::{robots_txt, sitemap_xml},
};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportReport {
    pub pages_written: usize,
    pub assets_written: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("failed to load legal records for static export")]
    Legal(#[from] LegalError),
    #[error("invalid rendered UTF-8 HTML for route '{route}'")]
    InvalidHtmlUtf8 {
        route: String,
        #[source]
        source: std::str::Utf8Error,
    },
    #[error("failed to write static export file '{path}'")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

/// Renders all public routes, error page, metadata files, and embedded/hashed
/// assets to `config.output_dir`, resolved under `config.base_path` and `config.origin`.
pub fn export_site(config: &ExportConfig) -> Result<ExportReport, ExportError> {
    let table = prerender_with_target(&Config::default(), &config.origin, &config.base_path)?;
    let legal = Legal::load()?;
    let assets = AssetTable::embedded_with_base([], &config.base_path);

    let mut pages_written = 0usize;
    for route in Route::pages(&legal) {
        let route_key = route.to_string();
        let page = &table.pages[&route_key];
        let html = rewrite_utf8(&route_key, &page.html, &assets, config)?;
        if route_key == "/" {
            write_file(&config.output_dir.join("index.html"), html.as_bytes())?;
            pages_written += 1;
        } else {
            let trimmed = route_key.trim_start_matches('/');
            write_file(
                &config.output_dir.join(format!("{trimmed}.html")),
                html.as_bytes(),
            )?;
            write_file(
                &config.output_dir.join(trimmed).join("index.html"),
                html.as_bytes(),
            )?;
            pages_written += 2;
        }
    }

    let not_found_html = rewrite_utf8("/404", &table.not_found.html, &assets, config)?;
    write_file(
        &config.output_dir.join("404.html"),
        not_found_html.as_bytes(),
    )?;
    write_file(
        &config.output_dir.join("sitemap.xml"),
        sitemap_xml(&config.origin, &config.base_path).as_bytes(),
    )?;
    write_file(
        &config.output_dir.join("robots.txt"),
        robots_txt(&config.origin, &config.base_path, false).as_bytes(),
    )?;
    write_file(&config.output_dir.join(".nojekyll"), b"")?;
    pages_written += 4;

    let mut asset_paths: Vec<_> = assets.assets.keys().cloned().collect();
    asset_paths.sort();
    for path in &asset_paths {
        let rel = path.trim_start_matches('/');
        write_file(
            &config.output_dir.join(rel),
            assets.assets[path].representation.identity_bytes(),
        )?;
    }

    Ok(ExportReport {
        pages_written,
        assets_written: asset_paths.len(),
    })
}

fn rewrite_utf8(
    route: &str,
    raw: &[u8],
    assets: &AssetTable,
    config: &ExportConfig,
) -> Result<String, ExportError> {
    let text = std::str::from_utf8(raw).map_err(|source| ExportError::InvalidHtmlUtf8 {
        route: route.to_owned(),
        source,
    })?;
    Ok(assets.rewrite_page(text, &config.base_path))
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<(), ExportError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| ExportError::WriteFile {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, bytes).map_err(|source| ExportError::WriteFile {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::site::{BasePath, SiteOrigin};

    struct TempExportDir(PathBuf);

    impl TempExportDir {
        fn new(label: &str) -> Self {
            let dir = std::env::temp_dir()
                .join(format!("coda-public-site-{label}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }

    impl Drop for TempExportDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn extract_attr_urls(html: &str, attr: &str) -> Vec<String> {
        let needle = format!("{attr}=\"");
        let mut out = Vec::new();
        let mut cursor = 0;
        while let Some(rel) = html[cursor..].find(&needle) {
            let start = cursor + rel + needle.len();
            let end = start + html[start..].find('"').unwrap();
            out.push(html[start..end].to_owned());
            cursor = end + 1;
        }
        out
    }

    fn exported_target_exists(root: &Path, base_prefix: &str, url: &str) -> bool {
        let Some(after_prefix) = url.strip_prefix(base_prefix) else {
            return false;
        };
        let without_fragment = after_prefix.split(['?', '#']).next().unwrap();
        let rel = without_fragment.trim_start_matches('/');
        if rel.is_empty() {
            return root.join("index.html").is_file();
        }
        root.join(rel).is_file()
            || root.join(format!("{rel}.html")).is_file()
            || root.join(rel).join("index.html").is_file()
    }

    #[test]
    fn static_export_under_subpath_resolves_every_page_asset_and_metadata_file() {
        let dir = TempExportDir::new("subpath");
        let base_path = BasePath::parse("/coda-rust").unwrap();
        let origin = SiteOrigin::parse("https://tyler274.github.io/coda-rust", &base_path).unwrap();
        let report = export_site(&ExportConfig {
            output_dir: dir.0.clone(),
            base_path,
            origin,
        })
        .unwrap();
        assert!(report.pages_written >= 20);
        assert!(report.assets_written >= 20);

        for file in [
            "index.html",
            "404.html",
            ".nojekyll",
            "sitemap.xml",
            "robots.txt",
            "duet.html",
            "duet/index.html",
            "duet/privacy.html",
            "duet/privacy/index.html",
            "duet/privacy/2026-09-09.html",
            "duet/privacy/2026-09-09/index.html",
            "clinical-transparency.html",
            "favicon.ico",
            "css/chrome.css",
            "homepage/site.js",
            "brand/coda-health-logo-transparent.webp",
        ] {
            assert!(dir.0.join(file).is_file(), "missing exported file {file}");
        }
        assert!(!dir.0.join("scheduling/dev.html").exists());

        let sitemap = fs::read_to_string(dir.0.join("sitemap.xml")).unwrap();
        assert!(sitemap.contains("<loc>https://tyler274.github.io/coda-rust/</loc>"));
        assert!(sitemap.contains("<loc>https://tyler274.github.io/coda-rust/duet</loc>"));
        assert!(!sitemap.contains("/coda-rust/coda-rust"));

        let robots = fs::read_to_string(dir.0.join("robots.txt")).unwrap();
        assert_eq!(
            robots,
            "User-Agent: *\nAllow: /\n\nHost: https://tyler274.github.io\nSitemap: https://tyler274.github.io/coda-rust/sitemap.xml\n"
        );

        let mut stack = vec![dir.0.clone()];
        let mut html_count = 0usize;
        while let Some(current) = stack.pop() {
            for entry in fs::read_dir(&current).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().and_then(|e| e.to_str()) != Some("html") {
                    continue;
                }
                html_count += 1;
                let html = fs::read_to_string(&path).unwrap();
                assert!(!html.contains("/coda-rust/coda-rust"), "{}", path.display());
                assert!(
                    html.contains("href=\"/coda-rust/book/\""),
                    "missing architecture book footer link in {}",
                    path.display()
                );
                for url in extract_attr_urls(&html, "href")
                    .into_iter()
                    .chain(extract_attr_urls(&html, "src"))
                {
                    if url.starts_with('/') {
                        assert!(
                            url.starts_with("/coda-rust/"),
                            "unprefixed root-relative URL {url} in {}",
                            path.display()
                        );
                        if url == "/coda-rust/book/" {
                            continue;
                        }
                        assert!(
                            exported_target_exists(&dir.0, "/coda-rust", &url),
                            "dangling local target {url} in {}",
                            path.display()
                        );
                    }
                }
            }
        }
        assert!(html_count >= 20);
        let not_found = fs::read_to_string(dir.0.join("404.html")).unwrap();
        assert!(not_found.contains("404 / Page not found"));
    }
}

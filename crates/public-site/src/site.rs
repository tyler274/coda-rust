//! Site-wide facts, each stated once.

use crate::Route;

/// The canonical origin every canonical URL and share image resolves against.
pub const SITE_ORIGIN: &str = "https://coda.dabney.moe";

/// The site name used in titles and `og:site_name`.
pub const SITE_NAME: &str = "Coda Health";

/// The partnership and contact address (nav, footer, 404).
pub const CONTACT_EMAIL: &str = "hello@dabney.moe";

/// The homepage document title; every other page uses `"{title} | Coda Health"`.
pub const HOME_TITLE: &str = "Coda Health | The physician-directed AI care team";

/// The HSTS policy production sends on every response: two years, for this host only.
/// Browsers ignore it over plain HTTP, so local runs are unaffected.
pub const STRICT_TRANSPORT_SECURITY: &str = "max-age=63072000";

/// Default listen address for the `public-site` binary.
pub const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:8791";

/// Validated URL configuration error for site origins and base paths.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SiteUrlError {
    #[error("base path must start with '/': {0}")]
    InvalidBasePath(String),
    #[error("site origin '{origin}' is not a valid http(s) URL")]
    InvalidOrigin {
        origin: String,
        #[source]
        source: url::ParseError,
    },
    #[error("site origin must use http or https and omit query/fragment: {0}")]
    UnsupportedOrigin(String),
    #[error("site origin path '{origin_path}' conflicts with base path '{base_path}'")]
    OriginBasePathMismatch {
        origin_path: String,
        base_path: String,
    },
}

/// Normalized path prefix where the public site is mounted (`""` for `/` or `"/coda-rust"`).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BasePath(String);

impl BasePath {
    /// Root mount (`/`).
    pub fn root() -> Self {
        Self(String::new())
    }

    /// Parses and normalizes a site base path (`"/"` -> root, `"/coda-rust/"` -> `"/coda-rust"`).
    pub fn parse(raw: &str) -> Result<Self, SiteUrlError> {
        if raw.is_empty()
            || raw.chars().any(char::is_whitespace)
            || !raw.starts_with('/')
            || raw.contains(['?', '#', '\\'])
            || raw.contains("://")
            || raw.contains("//")
        {
            return Err(SiteUrlError::InvalidBasePath(raw.to_owned()));
        }
        let trimmed = raw.trim_end_matches('/');
        if trimmed.is_empty() {
            return Ok(Self::root());
        }
        if trimmed
            .split('/')
            .skip(1)
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
        {
            return Err(SiteUrlError::InvalidBasePath(raw.to_owned()));
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    /// Empty string for root, otherwise leading-slash path without trailing slash (e.g. `"/coda-rust"`).
    pub fn prefix(&self) -> &str {
        &self.0
    }

    /// Resolves a root-relative site or asset path under this base path.
    pub fn join(&self, path: &str) -> String {
        debug_assert!(
            path.starts_with('/'),
            "site path must start with '/': {path}"
        );
        format!("{}{path}", self.0)
    }
}

/// Validated canonical origin (scheme + host + optional port) paired with [`BasePath`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SiteOrigin {
    host_origin: String,
}

impl Default for SiteOrigin {
    fn default() -> Self {
        Self {
            host_origin: SITE_ORIGIN.to_owned(),
        }
    }
}

impl SiteOrigin {
    /// Parses an origin URL (`https://tyler274.github.io` or `https://tyler274.github.io/coda-rust`)
    /// and verifies its path (if non-root) matches `base_path`.
    pub fn parse(raw: &str, base_path: &BasePath) -> Result<Self, SiteUrlError> {
        let parsed = url::Url::parse(raw).map_err(|source| SiteUrlError::InvalidOrigin {
            origin: raw.to_owned(),
            source,
        })?;
        if !matches!(parsed.scheme(), "http" | "https")
            || parsed.host_str().is_none()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(SiteUrlError::UnsupportedOrigin(raw.to_owned()));
        }
        let origin_path = parsed.path().trim_end_matches('/');
        if !origin_path.is_empty() && origin_path != base_path.prefix() {
            return Err(SiteUrlError::OriginBasePathMismatch {
                origin_path: origin_path.to_owned(),
                base_path: if base_path.is_root() {
                    "/".to_owned()
                } else {
                    base_path.prefix().to_owned()
                },
            });
        }
        Ok(Self {
            host_origin: parsed.origin().ascii_serialization(),
        })
    }

    /// Scheme + host (+ port), without any base path.
    pub fn host_origin(&self) -> &str {
        &self.host_origin
    }

    /// Canonical absolute URL for a route or asset path under `base_path`.
    /// When mounted at `/`, the homepage resolves to the bare origin (`https://coda.dabney.moe`).
    pub fn absolute_url(&self, base_path: &BasePath, path: &str) -> String {
        if base_path.is_root() && path == "/" {
            self.host_origin.clone()
        } else {
            format!("{}{}", self.host_origin, base_path.join(path))
        }
    }

    /// Sitemap location URL for a route (`/` retains its trailing slash).
    pub fn sitemap_url(&self, base_path: &BasePath, path: &str) -> String {
        format!("{}{}", self.host_origin, base_path.join(path))
    }
}

/// The `mailto:` link for [`CONTACT_EMAIL`].
pub fn contact_mailto() -> String {
    format!("mailto:{CONTACT_EMAIL}")
}

/// Resolves a site path against [`SITE_ORIGIN`]. The homepage resolves to the
/// bare origin (`https://coda.dabney.moe`, no slash).
pub fn absolute_url(path: &str) -> String {
    SiteOrigin::default().absolute_url(&BasePath::root(), path)
}

/// Renders `sitemap.xml` once for both the live server and static exports.
pub fn sitemap_xml(origin: &SiteOrigin, base_path: &BasePath) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">{}</urlset>",
        Route::sitemap()
            .iter()
            .map(|route| format!(
                "<url><loc>{}</loc></url>",
                xml_escape(&origin.sitemap_url(base_path, &route.to_string()))
            ))
            .collect::<String>()
    )
}

/// Renders `robots.txt` once for both the live server and static exports.
pub fn robots_txt(origin: &SiteOrigin, base_path: &BasePath, staging: bool) -> String {
    if staging {
        "User-agent: *\nDisallow: /\n".to_owned()
    } else {
        format!(
            "User-Agent: *\nAllow: /\n\nHost: {}\nSitemap: {}\n",
            origin.host_origin(),
            origin.sitemap_url(base_path, "/sitemap.xml")
        )
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_path_normalizes_and_rejects_invalid_inputs() {
        assert!(BasePath::parse("/").unwrap().is_root());
        assert_eq!(BasePath::parse("/").unwrap().join("/"), "/");
        assert_eq!(BasePath::parse("/").unwrap().join("/duet"), "/duet");

        let sub = BasePath::parse("/coda-rust/").unwrap();
        assert!(!sub.is_root());
        assert_eq!(sub.prefix(), "/coda-rust");
        assert_eq!(sub.join("/"), "/coda-rust/");
        assert_eq!(sub.join("/#physicians"), "/coda-rust/#physicians");
        assert_eq!(sub.join("/duet"), "/coda-rust/duet");

        for bad in [
            "",
            "coda-rust",
            "//coda-rust",
            "/coda-rust//pages",
            "/coda-rust/../etc",
            "/coda-rust?x=1",
            "/coda-rust#frag",
            "https://example.com/coda-rust",
            " /coda-rust",
        ] {
            assert!(BasePath::parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn origin_supports_host_only_or_matching_base_path_without_double_prefix() {
        let sub = BasePath::parse("/coda-rust").unwrap();
        let from_full = SiteOrigin::parse("https://tyler274.github.io/coda-rust", &sub).unwrap();
        let from_host = SiteOrigin::parse("https://tyler274.github.io", &sub).unwrap();
        assert_eq!(from_full, from_host);
        assert_eq!(from_full.host_origin(), "https://tyler274.github.io");
        assert_eq!(
            from_full.absolute_url(&sub, "/"),
            "https://tyler274.github.io/coda-rust/"
        );
        assert_eq!(
            from_full.absolute_url(&sub, "/duet"),
            "https://tyler274.github.io/coda-rust/duet"
        );
        assert_eq!(
            from_full.sitemap_url(&sub, "/sitemap.xml"),
            "https://tyler274.github.io/coda-rust/sitemap.xml"
        );
        assert!(SiteOrigin::parse("https://tyler274.github.io/other", &sub).is_err());
        assert!(SiteOrigin::parse("ftp://tyler274.github.io", &sub).is_err());
        assert!(SiteOrigin::parse("https://tyler274.github.io?q=1", &sub).is_err());
    }
}

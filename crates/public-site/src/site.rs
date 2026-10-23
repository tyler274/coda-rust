//! Site-wide facts, each stated once. Ported from kytos `lib/site.ts` and
//! `lib/metadata.ts`.

/// The canonical origin every canonical URL and share image resolves against.
pub const SITE_ORIGIN: &str = "https://codahealth.ai";

/// The site name used in titles and `og:site_name`.
pub const SITE_NAME: &str = "Coda Health";

/// The partnership and contact address (nav, footer, 404).
pub const CONTACT_EMAIL: &str = "hello@codahealth.ai";

/// The homepage document title; every other page uses `"{title} | Coda Health"`.
pub const HOME_TITLE: &str = "Coda Health | The physician-directed AI care team";

/// Default local upstream origin for the transport-only `/physician/` proxy.
pub const DEFAULT_PHYSICIAN_APP_URL: &str = "http://127.0.0.1:8790";

/// Default listen address for the `public-site` binary.
pub const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:8791";

/// The `mailto:` link for [`CONTACT_EMAIL`].
pub fn contact_mailto() -> String {
    format!("mailto:{CONTACT_EMAIL}")
}

/// Resolves a site path against [`SITE_ORIGIN`]. The homepage resolves to the
/// bare origin, as Next.js printed it (`https://codahealth.ai`, no slash).
pub fn absolute_url(path: &str) -> String {
    if path == "/" {
        SITE_ORIGIN.to_string()
    } else {
        format!("{SITE_ORIGIN}{path}")
    }
}

use crate::site::{DEFAULT_LISTEN_ADDR, DEFAULT_PHYSICIAN_APP_URL};
use std::net::SocketAddr;

/// Startup-only configuration; no physician credentials or database connection.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub physician_app_url: String,
    pub listen_addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            physician_app_url: DEFAULT_PHYSICIAN_APP_URL.into(),
            listen_addr: DEFAULT_LISTEN_ADDR
                .parse()
                .expect("constant socket address"),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();
        if let Ok(value) = std::env::var("PHYSICIAN_APP_URL") {
            // Prevent executable schemes from reaching a rendered navigation link.
            let uri: axum::http::Uri = value.parse()?;
            if !matches!(uri.scheme_str(), Some("http" | "https")) || uri.host().is_none() {
                return Err("PHYSICIAN_APP_URL must be an absolute HTTP(S) URL".into());
            }
            config.physician_app_url = value;
        }
        if let Ok(value) = std::env::var("PUBLIC_SITE_LISTEN_ADDR") {
            config.listen_addr = value.parse()?;
        }
        Ok(config)
    }
}

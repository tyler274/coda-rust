use crate::site::DEFAULT_LISTEN_ADDR;
use std::net::SocketAddr;

/// Startup-only rendering/listener configuration. Upstream transport settings
/// belong to RuntimeConfig, never to page content or navigation destinations.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub listen_addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: DEFAULT_LISTEN_ADDR
                .parse()
                .expect("constant socket address"),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();
        if let Ok(value) = std::env::var("PUBLIC_SITE_LISTEN_ADDR") {
            config.listen_addr = value.parse()?;
        }
        Ok(config)
    }
}

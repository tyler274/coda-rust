use crate::site::{BasePath, DEFAULT_LISTEN_ADDR, SiteOrigin, SiteUrlError};
use std::{ffi::OsString, net::SocketAddr, path::PathBuf};

/// How the public website links to `/physician/`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PhysicianLink {
    #[default]
    None,
    SignIn,
    LiveDemo,
}

/// Startup-only rendering/listener configuration. Upstream transport settings
/// belong to RuntimeConfig, never to page content or navigation destinations.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub listen_addr: SocketAddr,
    /// Whether the nav offers "Sign in". The binary sets it from
    /// `RuntimeConfig::physician_proxy`, so the link exists exactly when `/physician/` does.
    pub physician_sign_in: bool,
    /// Whether static export links "Try live demo" to the browser-SQLite `/physician/` bundle.
    pub physician_demo: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: DEFAULT_LISTEN_ADDR
                .parse()
                .expect("constant socket address"),
            physician_sign_in: false,
            physician_demo: false,
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

    pub fn physician_link(&self) -> PhysicianLink {
        if self.physician_sign_in {
            PhysicianLink::SignIn
        } else if self.physician_demo {
            PhysicianLink::LiveDemo
        } else {
            PhysicianLink::None
        }
    }
}

/// Static export configuration for writing prerendered HTML and assets to disk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportConfig {
    pub output_dir: PathBuf,
    pub base_path: BasePath,
    pub origin: SiteOrigin,
    pub physician_demo: bool,
}

/// Parsed binary invocation mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Serve,
    Export(ExportConfig),
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("missing value for CLI flag {0}")]
    MissingValue(&'static str),
    #[error("non-UTF-8 CLI argument")]
    InvalidEncoding,
    #[error("unknown CLI argument: {0}")]
    UnknownArgument(String),
    #[error("--base-path and --origin require --export-dir")]
    MissingExportDir,
    #[error("invalid static export URL setting")]
    InvalidUrl(#[from] SiteUrlError),
}

impl Command {
    /// Parses CLI arguments into either HTTP serving or one-shot static export.
    pub fn from_args(args: impl IntoIterator<Item = OsString>) -> Result<Self, CliError> {
        let mut iter = args.into_iter().skip(1);
        let mut export_dir: Option<PathBuf> = None;
        let mut base_path: Option<String> = None;
        let mut origin: Option<String> = None;
        let mut physician_demo = false;

        while let Some(raw) = iter.next() {
            let arg = raw.into_string().map_err(|_| CliError::InvalidEncoding)?;
            if arg == "--export-dir" {
                export_dir = Some(PathBuf::from(next_flag_value("--export-dir", &mut iter)?));
            } else if let Some(val) = arg.strip_prefix("--export-dir=") {
                export_dir = Some(PathBuf::from(val));
            } else if arg == "--base-path" {
                base_path = Some(next_flag_value("--base-path", &mut iter)?);
            } else if let Some(val) = arg.strip_prefix("--base-path=") {
                base_path = Some(val.to_owned());
            } else if arg == "--origin" {
                origin = Some(next_flag_value("--origin", &mut iter)?);
            } else if let Some(val) = arg.strip_prefix("--origin=") {
                origin = Some(val.to_owned());
            } else if arg == "--physician-demo" {
                physician_demo = true;
            } else {
                return Err(CliError::UnknownArgument(arg));
            }
        }

        let Some(output_dir) = export_dir else {
            if base_path.is_some() || origin.is_some() || physician_demo {
                return Err(CliError::MissingExportDir);
            }
            return Ok(Self::Serve);
        };
        if output_dir.as_os_str().is_empty() {
            return Err(CliError::MissingValue("--export-dir"));
        }

        let base_path = match base_path {
            Some(raw) => BasePath::parse(&raw)?,
            None => BasePath::root(),
        };
        let origin = match origin {
            Some(raw) => SiteOrigin::parse(&raw, &base_path)?,
            None => SiteOrigin::default(),
        };
        Ok(Self::Export(ExportConfig {
            output_dir,
            base_path,
            origin,
            physician_demo,
        }))
    }
}

fn next_flag_value(
    flag: &'static str,
    rest: &mut impl Iterator<Item = OsString>,
) -> Result<String, CliError> {
    rest.next()
        .ok_or(CliError::MissingValue(flag))?
        .into_string()
        .map_err(|_| CliError::InvalidEncoding)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn os(items: &[&str]) -> Vec<OsString> {
        items.iter().map(OsString::from).collect()
    }

    #[test]
    fn cli_defaults_to_serve_and_parses_static_export_flags() {
        assert_eq!(
            Command::from_args(os(&["public-site"])).unwrap(),
            Command::Serve
        );
        let cmd = Command::from_args(os(&[
            "public-site",
            "--export-dir",
            "_site",
            "--base-path=/coda-rust",
            "--origin",
            "https://tyler274.github.io/coda-rust",
            "--physician-demo",
        ]))
        .unwrap();
        assert_eq!(
            cmd,
            Command::Export(ExportConfig {
                output_dir: PathBuf::from("_site"),
                base_path: BasePath::parse("/coda-rust").unwrap(),
                origin: SiteOrigin::parse(
                    "https://tyler274.github.io/coda-rust",
                    &BasePath::parse("/coda-rust").unwrap()
                )
                .unwrap(),
                physician_demo: true,
            })
        );
        assert!(Command::from_args(os(&["public-site", "--base-path", "/coda-rust"])).is_err());
        assert!(Command::from_args(os(&["public-site", "--export-dir"])).is_err());
        assert!(Command::from_args(os(&["public-site", "--unknown"])).is_err());
    }
}

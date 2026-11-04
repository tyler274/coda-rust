//! Startup-rendered public website, isolated from the physician runtime.
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]
pub mod access;
pub mod assets;
pub mod chrome;
pub mod config;
pub mod legal;
pub mod pages;
pub mod prerender;
pub mod proxy;
pub mod representation;
pub mod route;
pub mod runtime;
pub mod server;
pub mod site;

pub use config::Config;
pub use prerender::{Page, PageTable, prerender};
pub use route::{PageMeta, Route};

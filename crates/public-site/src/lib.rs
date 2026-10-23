//! Startup-rendered public website, isolated from the physician runtime.
pub mod access;
pub mod assets;
pub mod chrome;
pub mod config;
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
pub use server::router;

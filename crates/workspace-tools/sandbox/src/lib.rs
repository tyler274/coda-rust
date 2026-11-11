//! The Coda agent sandbox: its config, the names it passes around, and the
//! read-only checks behind `doctor` and `status`.

pub mod config;
pub mod doctor;
pub mod editor_settings;
pub mod error;
pub mod ids;
pub mod probe;
pub mod status;

#[cfg(test)]
mod tests;

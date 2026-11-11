//! The Coda agent sandbox: its config, the names it passes around, the
//! read-only checks behind `doctor` and `status`, the commands that
//! replace the sandbox scripts (`worktree`, `ci run` and `carry`), and the
//! CI controller.

pub mod carry;
pub mod ci;
pub mod ci_controller;
pub mod config;
pub mod doctor;
pub mod editor_settings;
pub mod error;
pub mod git;
pub mod ids;
pub mod porcelain;
pub mod probe;
pub mod status;
pub mod worktree;

#[cfg(test)]
mod port_tests;
#[cfg(test)]
mod tests;

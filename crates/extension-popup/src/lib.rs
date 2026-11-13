//! The popup of Coda's Chrome extension, built from `ui` like the physician web app. The
//! extension's connector worker owns pairing with Chords, each reader's session in memory, and
//! every background read; the popup shows where the connection stands and asks the worker to pair,
//! connect, or disconnect.
//!
//! ```sh
//! cargo build -p extension-popup --target wasm32-unknown-unknown --features web
//! wasm-bindgen --target web --no-typescript --out-dir extension/chrome/popup \
//!   target/wasm32-unknown-unknown/debug/extension-popup.wasm
//! ```
#![deny(missing_docs)]

pub mod face;

//! Offline BGE encoder used by Duet and the physician retrieval path.
//! Book: `book/src/physician-site.md` (model boundary).
#![deny(missing_docs)]

mod model;
mod protocol;
mod recipe;
mod window;

pub use model::BertEncoder;
pub use protocol::{serve_duet, serve_patient, startup_line};
pub use recipe::{Protocol, fingerprint, unsupported_inference};

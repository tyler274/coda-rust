//! The CI client: `ci run` queues one job of a commit on the Firecracker CI
//! through the spool and waits for its result. `request` holds the spool
//! format the controller shares.

pub mod request;
pub mod run;

pub use run::{Target, run};

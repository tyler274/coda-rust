//! Read-only Spruce REST client and conversation change cursor.
//!
//! Spruce publishes no organization-wide item feed. [`changes`] follows the
//! documented way to notice new messages: list conversations by last activity
//! from a stored boundary, then read the items in each conversation that
//! moved. The cursor is opaque. The product stores it and owns patient
//! linking, de-duplication and effects. This crate does not send and does not
//! keep a ledger.
//!
//! An unchanged item keeps the same [`payload::Item::version`] across reads.
//! The activity poll is not treated as proof of an edit or a deletion, because
//! Spruce does not document whether those move `lastMessageAt`. [`reconcile`]
//! compares a full conversation read with the versions the product stored.
//!
//! Book: `book/src/crate-map.md`.
#![deny(missing_docs)]

pub mod api;
pub mod changes;
pub mod payload;
pub mod reconcile;

#[cfg(any(test, feature = "test-util"))]
pub mod testing;

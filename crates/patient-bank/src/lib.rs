//! The synthetic primary-care patient bank: seeds, terminology catalog, ledger
//! checks, the FHIR R4 renderer, bank build, validation and panel materialization.
//! See README.md for the pipeline.

pub mod catalog;
pub mod catalog_seed;
pub mod design;
pub mod json;
pub mod ledger;
pub mod names;
pub mod render;
pub mod rng;
pub mod terms;
pub mod time;

//! The synthetic primary-care patient bank: typed ledgers, the verified
//! terminology catalog, authoring checks, one timeline per patient, the FHIR R4
//! renderer, bank build, validation and panel materialization. See README.md.

pub mod bank;
pub mod catalog;
pub mod catalog_seed;
pub mod check;
pub mod fhir;
pub mod labs;
pub mod ledger;
pub mod materialize;
pub mod paths;
pub mod render;
pub mod rng;
pub mod seeds;
pub mod terms;
pub mod time;
pub mod timeline;
pub mod unit;
pub mod validate;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

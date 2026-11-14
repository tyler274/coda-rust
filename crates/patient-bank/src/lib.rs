//! The synthetic primary-care patient bank: typed ledgers, the verified
//! terminology catalog, the authoring check, one timeline per patient, the FHIR R4
//! renderer, the practice calendar, the bank build and its validation. See README.md.

pub mod attachments;
pub mod bank;
pub mod calendar;
pub mod catalog;
pub mod check;
pub mod draw;
pub mod fhir;
pub mod import;
pub mod labs;
pub mod ledger;
pub mod measure;
pub mod paths;
pub mod render;
pub mod repeats;
pub mod rng;
pub mod seeds;
pub mod terms;
pub mod time;
pub mod timeline;
pub mod unit;
pub mod validate;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

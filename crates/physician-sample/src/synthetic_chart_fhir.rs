//! Files raw synthetic patient-bank NDJSON lines into [`SourceRevision`] records
//! through the declared FHIR R4 adapter.

use local_product::physician_sources::SourceRevision;
use local_product::physician_sources::adapter::{
    RecordFilingError, file_fhir_record, validate_fhir_patient_binding,
};

/// Admits one raw NDJSON record line from `source_id` into `workspace`, preserving
/// `line` byte-for-byte in [`SourceRevision::content`].
pub fn source_revision(
    workspace: &str,
    source_id: &str,
    line: &str,
) -> Result<SourceRevision, RecordFilingError> {
    file_fhir_record(workspace, source_id, line, None)
}

/// Checks explicit patient references against the patient-scoped bank before filing.
/// The directory supplies scope for resources without an explicit patient reference.
pub fn patient_source_revision(
    workspace: &str,
    source_id: &str,
    patient_id: &str,
    line: &str,
) -> Result<SourceRevision, RecordFilingError> {
    let value: serde_json::Value =
        serde_json::from_str(line).map_err(RecordFilingError::InvalidJson)?;
    validate_fhir_patient_binding(&value, patient_id)?;
    source_revision(workspace, source_id, line)
}

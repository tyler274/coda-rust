//! Files raw synthetic patient-bank NDJSON lines into [`SourceRevision`] records
//! through the declared [`FHIR_R4_TABLE`] adapter table.

use local_product::physician_sources::SourceRevision;
use local_product::physician_sources::adapter::{
    FHIR_R4_TABLE, RecordFilingError, file_json_record,
};

/// Admits one raw NDJSON record line from `source_id` into `workspace`, preserving
/// `line` byte-for-byte in [`SourceRevision::content`].
pub fn source_revision(
    workspace: &str,
    source_id: &str,
    line: &str,
) -> Result<SourceRevision, RecordFilingError> {
    file_json_record(&FHIR_R4_TABLE, workspace, source_id, line, None)
}

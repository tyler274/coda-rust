#[derive(Debug)]
pub struct Failure {
    pub message: String,
    pub code: u8,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Failure {
    pub fn new(code: u8, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code,
            source: None,
        }
    }

    pub fn with_source(
        code: u8,
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            code,
            source: Some(Box::new(source)),
        }
    }
}

impl std::fmt::Display for Failure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for Failure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|error| error as &(dyn std::error::Error + 'static))
    }
}

/// Google API canonical status names that are safe to print or persist.
const API_STATUSES: [&str; 11] = [
    "PERMISSION_DENIED",
    "UNAUTHENTICATED",
    "NOT_FOUND",
    "ALREADY_EXISTS",
    "INVALID_ARGUMENT",
    "FAILED_PRECONDITION",
    "RESOURCE_EXHAUSTED",
    "DEADLINE_EXCEEDED",
    "UNAVAILABLE",
    "ABORTED",
    "INTERNAL",
];

/// Projects provider stderr onto the first allowlisted API status it names.
/// Only for reporting: decisions such as absence use structured responses.
/// The raw text never leaves this function.
pub fn api_status(stderr: &[u8]) -> Option<&'static str> {
    let text = String::from_utf8_lossy(stderr);
    let bytes = text.as_bytes();
    let boundary = |index: usize| {
        bytes
            .get(index)
            .is_none_or(|byte| !(byte.is_ascii_alphanumeric() || *byte == b'_'))
    };
    API_STATUSES
        .iter()
        .filter_map(|status| {
            text.match_indices(status)
                .find(|(start, _)| {
                    (*start == 0 || boundary(start - 1)) && boundary(start + status.len())
                })
                .map(|(start, _)| (start, *status))
        })
        .min_by_key(|(start, _)| *start)
        .map(|(_, status)| status)
}

#[cfg(test)]
mod tests {
    use super::{Failure, api_status};
    use std::error::Error;

    #[test]
    fn api_status_projects_only_the_first_allowlisted_token() {
        assert_eq!(
            api_status(
                b"ERROR: (gcloud.run.services.update) PERMISSION_DENIED: secret-token NOT_FOUND"
            ),
            Some("PERMISSION_DENIED")
        );
        assert_eq!(api_status(b"ERROR: secret-token XPERMISSION_DENIED"), None);
        assert_eq!(api_status(b"ERROR: secret-token"), None);
    }

    #[test]
    fn safe_display_retains_typed_cause() {
        let failure = Failure::with_source(
            1,
            "operation failed",
            std::io::Error::other("private diagnostic"),
        );
        assert_eq!(failure.to_string(), "operation failed");
        assert!(failure.source().unwrap().is::<std::io::Error>());
    }
}

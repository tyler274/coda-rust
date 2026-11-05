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

#[cfg(test)]
mod tests {
    use super::Failure;
    use std::error::Error;

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

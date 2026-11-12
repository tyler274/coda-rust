//! Safe messages at the logging boundary; original causes stay inspectable.
use std::{error::Error as StdError, fmt};

#[derive(thiserror::Error)]
#[error("{message}")]
pub struct Error {
    message: String,
    source: Option<Box<dyn StdError + Send + Sync>>,
}
impl Error {
    pub fn caused(
        message: impl Into<String>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
    pub fn status(operation: &str, status: reqwest::StatusCode) -> Self {
        Self::from(format!("{operation} answered {}", status.as_u16()))
    }
}
impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }
}
impl From<String> for Error {
    fn from(message: String) -> Self {
        Self {
            message,
            source: None,
        }
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_is_preserved_but_never_formatted() {
        let error = Error::caused(
            "credential request failed",
            std::io::Error::other("sensitive-response"),
        );
        assert!(error.source().is_some());
        assert!(!format!("{error:?} {error}").contains("sensitive-response"));
    }
}

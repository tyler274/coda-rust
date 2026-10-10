//! Request and stream mechanics with no schema knowledge.

use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("HTTP {status}: {body}")]
    Status { status: u16, body: String },
    #[error("{0}")]
    Transport(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseEvent {
    pub event: String,
    pub data: String,
    pub id: Option<String>,
}

pub async fn get_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
) -> Result<T, HttpError> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| HttpError::Transport(error.to_string()))?;
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|error| HttpError::Transport(error.to_string()))?;
    if !status.is_success() {
        return Err(HttpError::Status {
            status: status.as_u16(),
            body,
        });
    }
    serde_json::from_str(&body).map_err(|error| HttpError::Transport(error.to_string()))
}

pub async fn post_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    body: &impl Serialize,
) -> Result<T, HttpError> {
    let response = client
        .post(url)
        .json(body)
        .send()
        .await
        .map_err(|error| HttpError::Transport(error.to_string()))?;
    let status = response.status();
    let text = response
        .text()
        .await
        .map_err(|error| HttpError::Transport(error.to_string()))?;
    if !status.is_success() {
        return Err(HttpError::Status {
            status: status.as_u16(),
            body: text,
        });
    }
    serde_json::from_str(&text).map_err(|error| HttpError::Transport(error.to_string()))
}

/// Parse one Server-Sent Events buffer into complete events.
pub fn parse_sse(buffer: &str) -> Vec<SseEvent> {
    buffer
        .split("\n\n")
        .filter(|block| !block.trim().is_empty())
        .filter_map(|block| {
            let mut event = "message".to_owned();
            let mut data = Vec::new();
            let mut id = None;
            for line in block.lines() {
                if let Some(rest) = line.strip_prefix("event:") {
                    event = rest.trim().to_owned();
                } else if let Some(rest) = line.strip_prefix("data:") {
                    data.push(rest.trim_start().to_owned());
                } else if let Some(rest) = line.strip_prefix("id:") {
                    id = Some(rest.trim().to_owned());
                }
            }
            if data.is_empty() && id.is_none() {
                return None;
            }
            Some(SseEvent {
                event,
                data: data.join("\n"),
                id,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_named_sse_events() {
        let events = parse_sse("event: workspace_changed\ndata: {\"cursor\":1}\n\n");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "workspace_changed");
        assert_eq!(events[0].data, "{\"cursor\":1}");
    }
}

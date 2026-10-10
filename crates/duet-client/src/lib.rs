//! Duet caller. Models and operations come from the OpenAPI Generator Rust client.
//! This crate adds the app-build headers the FastAPI gate reads.

use duet_wire::apis::configuration::Configuration;
pub use duet_wire::models::AccountMoment;

pub const BUILD_HEADER: &str = "X-Duet-App-Build";
pub const VERSION_HEADER: &str = "X-Duet-App-Version";

#[derive(Debug, Clone)]
pub struct DuetClient {
    configuration: Configuration,
}

impl DuetClient {
    pub fn new(base_path: impl Into<String>, bearer: impl Into<String>, app_build: &str, app_version: &str) -> Result<Self, String> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(BUILD_HEADER, header(app_build)?);
        headers.insert(VERSION_HEADER, header(app_version)?);
        let client = reqwest::Client::builder().default_headers(headers).build().map_err(|error| error.to_string())?;
        Ok(Self {
            configuration: Configuration {
                base_path: base_path.into().trim_end_matches('/').to_owned(),
                user_agent: Some("coda-duet/0.1.0".to_owned()),
                client,
                basic_auth: None,
                oauth_access_token: None,
                bearer_access_token: Some(bearer.into()),
                api_key: None,
            },
        })
    }

    pub async fn list_moments(&self) -> Result<Vec<AccountMoment>, String> {
        duet_wire::apis::moments_api::list_moments_api_moments_get(&self.configuration)
            .await
            .map_err(|error| error.to_string())
    }
}

fn header(value: &str) -> Result<reqwest::header::HeaderValue, String> {
    reqwest::header::HeaderValue::from_str(value).map_err(|error| error.to_string())
}

/// The shared HTTP crate stays on the Duet side for stream parsing even though
/// the generated client owns the JSON calls.
pub fn parse_sse(buffer: &str) -> Vec<http::SseEvent> {
    http::parse_sse(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use axum::routing::get;
    use duet_wire::models::MomentKind;

    #[tokio::test]
    async fn lists_moments_from_a_json_server() {
        let moment = AccountMoment::new(
            uuid::Uuid::nil(),
            MomentKind::FirstSummaryPrepared,
            chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05Z").unwrap(),
        );
        let body = vec![moment];
        let app = axum::Router::new().route("/api/moments", get(move || {
            let body = body.clone();
            async move { Json(body) }
        }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let client = DuetClient::new(format!("http://{address}"), "test-token", "200", "1.0.0").unwrap();
        let moments = client.list_moments().await.unwrap();
        assert_eq!(moments.len(), 1);
        assert_eq!(moments[0].kind, MomentKind::FirstSummaryPrepared);
    }
}

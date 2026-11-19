//! Server-side speech-to-text transcription for `POST /dictation`.
//!
//! Hosted mode transcribes exclusively through the configured BAA OpenAI Audio
//! Transcriptions endpoint and fails closed (`503 Service Unavailable`) when no
//! transcription provider is configured or the upstream request fails. Synthetic
//! `TEXT:` payloads and `PHYSICIAN_DICTATION_FIXTURE` are restricted to
//! non-hosted tests and fail closed on binary audio so fabricated clinical text
//! is never returned into the composer.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use axum::Json;
use axum::body::Bytes;
use axum::http::{Extensions, StatusCode};
use axum::response::{IntoResponse, Response};
use base64::Engine as _;
use serde::Deserialize;

use crate::hosting;

const OPENAI_TRANSCRIPTIONS_ENDPOINT: &str = "https://api.openai.com/v1/audio/transcriptions";
const DEFAULT_TRANSCRIPTION_MODEL: &str = "gpt-4o-mini-transcribe";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
static BOUNDARY_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Errors constructing the HTTP transcription backend from environment settings.
#[derive(Debug, thiserror::Error)]
pub(crate) enum DictationConfigError {
    #[error("PHYSICIAN_DICTATION_FIXTURE is forbidden in hosted execution")]
    FixtureInHostedMode,
    #[error("PHYSICIAN_DICTATION_ENDPOINT is forbidden in hosted execution")]
    CustomEndpointInHostedMode,
    #[error("invalid PHYSICIAN_DICTATION_ENDPOINT: {0}")]
    InvalidEndpoint(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error(
        "PHYSICIAN_DICTATION_ENDPOINT must use https:// (or loopback http:// in non-hosted mode) without credentials or fragment"
    )]
    UnsupportedEndpointSchemeOrHost,
    #[error("failed to build dictation HTTP client: {0}")]
    HttpClient(#[source] reqwest::Error),
    #[error("{0}")]
    ApiKey(String),
}

impl From<DictationConfigError> for hosting::StartupError {
    fn from(error: DictationConfigError) -> Self {
        error.to_string().into()
    }
}

/// Errors calling the upstream speech-to-text provider.
#[derive(Debug, thiserror::Error)]
enum TranscriptionError {
    #[error("transcription request failed: {0}")]
    Transport(#[source] reqwest::Error),
    #[error("transcription provider returned HTTP {0}")]
    UpstreamStatus(StatusCode),
    #[error("transcription response body invalid: {0}")]
    InvalidBody(#[source] reqwest::Error),
    #[error("transcription provider returned empty text")]
    EmptyTranscript,
}

#[derive(Deserialize)]
struct OpenAiTranscriptionResponse {
    text: String,
}

/// Configured transcription execution path for `POST /dictation`.
#[derive(Clone)]
pub(crate) enum DictationBackend {
    /// Real speech-to-text transcription via OpenAI's `/v1/audio/transcriptions`.
    OpenAi {
        client: reqwest::Client,
        endpoint: reqwest::Url,
        api_key: String,
        model: String,
    },
    /// Non-hosted test mode accepting only explicit `PHYSICIAN_DICTATION_FIXTURE`
    /// or `TEXT:<phrase>` payloads; real binary audio fails closed (`503`).
    LocalTest { fixture: Option<String> },
    /// Fails closed with `503 Service Unavailable`.
    Unavailable,
}

impl fmt::Debug for DictationBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenAi {
                endpoint, model, ..
            } => f
                .debug_struct("OpenAi")
                .field("endpoint", endpoint)
                .field("api_key", &"<redacted>")
                .field("model", model)
                .finish(),
            Self::LocalTest { fixture } => f
                .debug_struct("LocalTest")
                .field("fixture", fixture)
                .finish(),
            Self::Unavailable => f.write_str("Unavailable"),
        }
    }
}

/// Default non-hosted test fallback layered by `durable_site` and `fixture_site`
/// so direct Rust test servers handle `TEXT:` fakes while `site_from_env`
/// supersedes it with an explicit [`DictationBackend`].
#[derive(Clone, Debug)]
pub(crate) struct FallbackDictationBackend(pub(crate) DictationBackend);

impl DictationBackend {
    /// Resolves the dictation backend from process environment settings.
    pub(crate) fn from_env(
        get: &impl Fn(&str) -> Option<String>,
    ) -> Result<Self, DictationConfigError> {
        let hosted = hosting::is_hosted(get);
        let configured = |key: &str| {
            get(key)
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
        };
        let fixture = configured("PHYSICIAN_DICTATION_FIXTURE");
        let custom_endpoint = configured("PHYSICIAN_DICTATION_ENDPOINT");
        if hosted {
            if fixture.is_some() {
                return Err(DictationConfigError::FixtureInHostedMode);
            }
            if custom_endpoint.is_some() {
                return Err(DictationConfigError::CustomEndpointInHostedMode);
            }
        }
        let model = configured("PHYSICIAN_DICTATION_MODEL")
            .unwrap_or_else(|| DEFAULT_TRANSCRIPTION_MODEL.to_owned());

        if let Some(raw_endpoint) = custom_endpoint {
            let endpoint = parse_local_or_https_endpoint(&raw_endpoint)?;
            let api_key = if hosting::is_loopback_url_host(&endpoint) {
                configured("OPENAI_API_KEY").unwrap_or_else(|| "local-proof".to_owned())
            } else {
                local_product::open_ai_api_key(
                    configured("CODA_ALLOW_MODEL_CALLS").as_deref(),
                    configured("OPENAI_API_KEY").as_deref(),
                )
                .map_err(DictationConfigError::ApiKey)?
            };
            return Self::open_ai(endpoint, api_key, model);
        }

        if let Ok(api_key) = local_product::open_ai_api_key(
            configured("CODA_ALLOW_MODEL_CALLS").as_deref(),
            configured("OPENAI_API_KEY").as_deref(),
        ) {
            let endpoint = OPENAI_TRANSCRIPTIONS_ENDPOINT
                .parse::<reqwest::Url>()
                .expect("constant OpenAI transcriptions URL is valid");
            return Self::open_ai(endpoint, api_key, model);
        }

        if !hosted
            && (fixture.is_some() || configured("PHYSICIAN_LOCAL_PROOF").as_deref() == Some("1"))
        {
            return Ok(Self::LocalTest { fixture });
        }

        Ok(Self::Unavailable)
    }

    fn open_ai(
        endpoint: reqwest::Url,
        api_key: String,
        model: String,
    ) -> Result<Self, DictationConfigError> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(DictationConfigError::HttpClient)?;
        Ok(Self::OpenAi {
            client,
            endpoint,
            api_key,
            model,
        })
    }
}

fn parse_local_or_https_endpoint(raw: &str) -> Result<reqwest::Url, DictationConfigError> {
    let url = reqwest::Url::parse(raw.trim())
        .map_err(|error| DictationConfigError::InvalidEndpoint(Box::new(error)))?;
    let valid_scheme =
        url.scheme() == "https" || (url.scheme() == "http" && hosting::is_loopback_url_host(&url));
    if !valid_scheme
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(DictationConfigError::UnsupportedEndpointSchemeOrHost);
    }
    Ok(url)
}

fn dictation_unavailable() -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        "Dictation is unavailable. Check microphone support and server configuration.",
    )
        .into_response()
}

/// Handles `POST /dictation` for the physician UI.
pub(crate) async fn transcribe_dictation(extensions: Extensions, body: Bytes) -> Response {
    let backend = extensions
        .get::<DictationBackend>()
        .cloned()
        .or_else(|| {
            extensions
                .get::<FallbackDictationBackend>()
                .map(|fallback| fallback.0.clone())
        })
        .unwrap_or(DictationBackend::Unavailable);

    if matches!(backend, DictationBackend::Unavailable) {
        return dictation_unavailable();
    }

    let Ok(request) = serde_json::from_slice::<physician_wire::DictationRequest>(&body) else {
        return dictation_unavailable();
    };
    let encoded = request.audio_base64.trim();
    if encoded.is_empty() {
        return dictation_unavailable();
    }
    let Ok(audio_bytes) = base64::engine::general_purpose::STANDARD.decode(encoded) else {
        return dictation_unavailable();
    };
    if audio_bytes.len() < 4 {
        return dictation_unavailable();
    }

    let transcript = match backend {
        DictationBackend::OpenAi {
            client,
            endpoint,
            api_key,
            model,
        } => {
            match transcribe_openai(
                &client,
                &endpoint,
                &api_key,
                &model,
                &request.mime_type,
                &audio_bytes,
            )
            .await
            {
                Ok(text) => text,
                Err(error) => {
                    tracing::warn!(%error, "dictation transcription request failed");
                    return dictation_unavailable();
                }
            }
        }
        DictationBackend::LocalTest { fixture } => {
            let Some(text) = transcribe_local_test(fixture.as_deref(), &audio_bytes) else {
                return dictation_unavailable();
            };
            text
        }
        DictationBackend::Unavailable => return dictation_unavailable(),
    };

    Json(physician_wire::DictationResponse { transcript }).into_response()
}

fn transcribe_local_test(fixture: Option<&str>, audio_bytes: &[u8]) -> Option<String> {
    if let Some(text) = audio_bytes.strip_prefix(b"TEXT:") {
        let recognized = String::from_utf8_lossy(text).trim().to_owned();
        return (!recognized.is_empty()).then_some(recognized);
    }
    fixture
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn audio_content_type_and_ext(mime_type: &str) -> (&'static str, &'static str) {
    let base = mime_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    match base.as_str() {
        "audio/webm" | "video/webm" => ("audio/webm", "webm"),
        "audio/ogg" | "application/ogg" => ("audio/ogg", "ogg"),
        "audio/mp4" | "audio/m4a" | "audio/x-m4a" => ("audio/mp4", "m4a"),
        "audio/mpeg" | "audio/mp3" => ("audio/mpeg", "mp3"),
        "audio/wav" | "audio/x-wav" | "audio/wave" => ("audio/wav", "wav"),
        "audio/flac" => ("audio/flac", "flac"),
        _ => ("audio/webm", "webm"),
    }
}

fn build_multipart_body(
    model: &str,
    content_type: &str,
    ext: &str,
    audio_bytes: &[u8],
) -> (String, Vec<u8>) {
    let seq = BOUNDARY_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut boundary = format!("codaDictationBoundary{seq:016x}");
    while audio_bytes
        .windows(boundary.len())
        .any(|window| window == boundary.as_bytes())
    {
        boundary.push('x');
    }

    let safe_model: String = model
        .chars()
        .filter(|ch| !matches!(ch, '\r' | '\n'))
        .collect();

    let mut body = Vec::with_capacity(audio_bytes.len() + 512);
    let push_field = |buf: &mut Vec<u8>, name: &str, value: &str| {
        buf.extend_from_slice(b"--");
        buf.extend_from_slice(boundary.as_bytes());
        buf.extend_from_slice(b"\r\nContent-Disposition: form-data; name=\"");
        buf.extend_from_slice(name.as_bytes());
        buf.extend_from_slice(b"\"\r\n\r\n");
        buf.extend_from_slice(value.as_bytes());
        buf.extend_from_slice(b"\r\n");
    };
    push_field(&mut body, "model", &safe_model);
    push_field(&mut body, "response_format", "json");

    body.extend_from_slice(b"--");
    body.extend_from_slice(boundary.as_bytes());
    body.extend_from_slice(
        b"\r\nContent-Disposition: form-data; name=\"file\"; filename=\"dictation.",
    );
    body.extend_from_slice(ext.as_bytes());
    body.extend_from_slice(b"\"\r\nContent-Type: ");
    body.extend_from_slice(content_type.as_bytes());
    body.extend_from_slice(b"\r\n\r\n");
    body.extend_from_slice(audio_bytes);
    body.extend_from_slice(b"\r\n--");
    body.extend_from_slice(boundary.as_bytes());
    body.extend_from_slice(b"--\r\n");

    (format!("multipart/form-data; boundary={boundary}"), body)
}

async fn transcribe_openai(
    client: &reqwest::Client,
    endpoint: &reqwest::Url,
    api_key: &str,
    model: &str,
    mime_type: &str,
    audio_bytes: &[u8],
) -> Result<String, TranscriptionError> {
    let (content_type, ext) = audio_content_type_and_ext(mime_type);
    let (multipart_header, payload) = build_multipart_body(model, content_type, ext, audio_bytes);
    let response = client
        .post(endpoint.clone())
        .bearer_auth(api_key)
        .header(reqwest::header::CONTENT_TYPE, multipart_header)
        .body(payload)
        .send()
        .await
        .map_err(TranscriptionError::Transport)?;

    let status = response.status();
    if !status.is_success() {
        return Err(TranscriptionError::UpstreamStatus(status));
    }

    let parsed: OpenAiTranscriptionResponse = response
        .json()
        .await
        .map_err(TranscriptionError::InvalidBody)?;
    let transcript = parsed.text.trim().to_owned();
    if transcript.is_empty() {
        return Err(TranscriptionError::EmptyTranscript);
    }
    Ok(transcript)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use axum::Router;
    use axum::body::Body;
    use axum::http::{HeaderMap, Request, StatusCode};
    use axum::routing::post;
    use tower::ServiceExt as _;

    use super::*;

    fn env_map(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        move |key| map.get(key).cloned()
    }

    #[test]
    fn hosted_mode_forbids_fixture_and_custom_endpoint() {
        let fixture_err = DictationBackend::from_env(&env_map(&[
            ("CODA_HOSTED", "1"),
            ("PHYSICIAN_DICTATION_FIXTURE", "synthetic"),
        ]))
        .unwrap_err();
        assert!(matches!(
            fixture_err,
            DictationConfigError::FixtureInHostedMode
        ));

        let endpoint_err = DictationBackend::from_env(&env_map(&[
            ("K_SERVICE", "physician"),
            ("PHYSICIAN_DICTATION_ENDPOINT", "https://example.com/v1"),
        ]))
        .unwrap_err();
        assert!(matches!(
            endpoint_err,
            DictationConfigError::CustomEndpointInHostedMode
        ));
    }

    #[test]
    fn hosted_mode_uses_openai_when_configured_and_fails_closed_otherwise() {
        let configured = DictationBackend::from_env(&env_map(&[
            ("CODA_HOSTED", "1"),
            ("CODA_ALLOW_MODEL_CALLS", "1"),
            ("OPENAI_API_KEY", "sk-test-secret"),
        ]))
        .unwrap();
        match configured {
            DictationBackend::OpenAi {
                endpoint,
                api_key,
                model,
                ..
            } => {
                assert_eq!(endpoint.as_str(), OPENAI_TRANSCRIPTIONS_ENDPOINT);
                assert_eq!(api_key, "sk-test-secret");
                assert_eq!(model, DEFAULT_TRANSCRIPTION_MODEL);
            }
            other => panic!("expected OpenAi backend, got {other:?}"),
        }

        let debug_text = format!(
            "{:?}",
            DictationBackend::from_env(&env_map(&[
                ("CODA_HOSTED", "1"),
                ("CODA_ALLOW_MODEL_CALLS", "1"),
                ("OPENAI_API_KEY", "sk-test-secret"),
            ]))
            .unwrap()
        );
        assert!(!debug_text.contains("sk-test-secret"));

        let missing_key = DictationBackend::from_env(&env_map(&[("CODA_HOSTED", "1")])).unwrap();
        assert!(matches!(missing_key, DictationBackend::Unavailable));

        let unallowed = DictationBackend::from_env(&env_map(&[
            ("CODA_HOSTED", "1"),
            ("OPENAI_API_KEY", "sk-test-secret"),
        ]))
        .unwrap();
        assert!(matches!(unallowed, DictationBackend::Unavailable));
    }

    #[tokio::test]
    async fn local_test_and_unavailable_never_fabricate_clinical_text_for_binary_audio() {
        let router = Router::new()
            .route("/dictation", post(transcribe_dictation))
            .layer(axum::Extension(FallbackDictationBackend(
                DictationBackend::LocalTest { fixture: None },
            )));

        // Real WebM EBML header bytes must fail closed with 503 when no real STT backend exists.
        let binary_audio = base64::engine::general_purpose::STANDARD
            .encode([0x1A, 0x45, 0xDF, 0xA3, 0x01, 0x02, 0x03, 0x04]);
        let req = Request::builder()
            .method("POST")
            .uri("/dictation")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&physician_wire::DictationRequest {
                    audio_base64: binary_audio.clone(),
                    mime_type: "audio/webm;codecs=opus".into(),
                    sample_rate_hz: Some(48_000),
                })
                .unwrap(),
            ))
            .unwrap();
        let resp = router.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);

        // Non-hosted LocalTest still honors explicit TEXT: test payloads.
        let text_audio = base64::engine::general_purpose::STANDARD.encode(b"TEXT:Check potassium.");
        let req = Request::builder()
            .method("POST")
            .uri("/dictation")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&physician_wire::DictationRequest {
                    audio_base64: text_audio.clone(),
                    mime_type: "audio/webm".into(),
                    sample_rate_hz: None,
                })
                .unwrap(),
            ))
            .unwrap();
        let resp = router.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let decoded: physician_wire::DictationResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(decoded.transcript, "Check potassium.");

        // Outer Unavailable backend (as in hosted mode without OpenAI key) rejects TEXT: and binary audio alike.
        let hosted_router = router.layer(axum::Extension(DictationBackend::Unavailable));
        for payload in [text_audio, binary_audio] {
            let req = Request::builder()
                .method("POST")
                .uri("/dictation")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&physician_wire::DictationRequest {
                        audio_base64: payload,
                        mime_type: "audio/webm".into(),
                        sample_rate_hz: None,
                    })
                    .unwrap(),
                ))
                .unwrap();
            let resp = hosted_router.clone().oneshot(req).await.unwrap();
            assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
        }
    }

    #[tokio::test]
    async fn openai_backend_posts_multipart_audio_and_fails_closed_on_upstream_failure() {
        #[derive(Clone, Default)]
        struct Captured {
            auth: Option<String>,
            content_type: Option<String>,
            body: Vec<u8>,
            next_status: u16,
            next_body: String,
        }

        let state = Arc::new(Mutex::new(Captured {
            next_status: 200,
            next_body: r#"{"text":"Start lisinopril 10 mg daily."}"#.to_owned(),
            ..Default::default()
        }));

        let captured_for_server = Arc::clone(&state);
        let upstream = Router::new().route(
            "/v1/audio/transcriptions",
            post(move |headers: HeaderMap, body: Bytes| {
                let captured = Arc::clone(&captured_for_server);
                async move {
                    let (status, reply) = {
                        let mut guard = captured.lock().unwrap();
                        guard.auth = headers
                            .get("authorization")
                            .and_then(|v| v.to_str().ok())
                            .map(str::to_owned);
                        guard.content_type = headers
                            .get("content-type")
                            .and_then(|v| v.to_str().ok())
                            .map(str::to_owned);
                        guard.body = body.to_vec();
                        (
                            StatusCode::from_u16(guard.next_status).unwrap(),
                            guard.next_body.clone(),
                        )
                    };
                    (
                        status,
                        [(axum::http::header::CONTENT_TYPE, "application/json")],
                        reply,
                    )
                }
            }),
        );

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, upstream).await.unwrap() });

        let endpoint = format!("http://{addr}/v1/audio/transcriptions");
        let backend = DictationBackend::from_env(&env_map(&[
            ("PHYSICIAN_DICTATION_ENDPOINT", &endpoint),
            ("OPENAI_API_KEY", "sk-openai-baa"),
            ("PHYSICIAN_DICTATION_MODEL", "gpt-4o-transcribe"),
        ]))
        .unwrap();

        let router = Router::new()
            .route("/dictation", post(transcribe_dictation))
            .layer(axum::Extension(backend));

        let raw_audio = b"\x1A\x45\xDF\xA3webm-sample-bytes";
        let req = Request::builder()
            .method("POST")
            .uri("/dictation")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&physician_wire::DictationRequest {
                    audio_base64: base64::engine::general_purpose::STANDARD.encode(raw_audio),
                    mime_type: "audio/webm;codecs=opus\r\nInjected: bad".into(),
                    sample_rate_hz: Some(48_000),
                })
                .unwrap(),
            ))
            .unwrap();
        let resp = router.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let decoded: physician_wire::DictationResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(decoded.transcript, "Start lisinopril 10 mg daily.");

        {
            let guard = state.lock().unwrap();
            assert_eq!(guard.auth.as_deref(), Some("Bearer sk-openai-baa"));
            let ct = guard.content_type.as_deref().unwrap();
            assert!(ct.starts_with("multipart/form-data; boundary=codaDictationBoundary"));
            let body_lossy = String::from_utf8_lossy(&guard.body);
            assert!(body_lossy.contains("name=\"model\"\r\n\r\ngpt-4o-transcribe\r\n"));
            assert!(body_lossy.contains("name=\"response_format\"\r\n\r\njson\r\n"));
            assert!(
                body_lossy.contains("filename=\"dictation.webm\"\r\nContent-Type: audio/webm\r\n")
            );
            assert!(!body_lossy.contains("Injected: bad"));
            assert!(guard.body.windows(raw_audio.len()).any(|w| w == raw_audio));
        }

        // Upstream 500 or empty transcript fails closed with 503.
        state.lock().unwrap().next_status = 500;
        let fail_req = Request::builder()
            .method("POST")
            .uri("/dictation")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&physician_wire::DictationRequest {
                    audio_base64: base64::engine::general_purpose::STANDARD.encode(raw_audio),
                    mime_type: "audio/webm".into(),
                    sample_rate_hz: None,
                })
                .unwrap(),
            ))
            .unwrap();
        let fail_resp = router.oneshot(fail_req).await.unwrap();
        assert_eq!(fail_resp.status(), StatusCode::SERVICE_UNAVAILABLE);

        server.abort();
    }
}

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, Response, StatusCode, header},
};
use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use http::iap::fixture;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, jwk::JwkSet};
use langfuse_sso::{Config, ConfigError, router_with_verifier};
use serde_json::Value;
use std::time::Duration;
use tower::ServiceExt;
use url::Url;

const AUDIENCE: &str = "/projects/123456789/locations/us-east1/services/public-site-staging";
const ISSUER: &str = "http://langfuse-sso.observability.svc.cluster.local:3000/langfuse-sso";
const AUTHORIZE: &str = "https://staging.codahealth.ai/langfuse-sso/authorize";
const CLIENT_ID: &str = "langfuse";
const SECRET: &str = "a client secret of at least 32 bytes / with + reserved & chars";
const REDIRECT: &str = "https://staging.codahealth.ai/langfuse/api/auth/callback/custom";
const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
const CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";

fn signing_key() -> String {
    fixture::key_file()["private_pkcs8_base64"]
        .as_str()
        .unwrap()
        .to_owned()
}

fn settings(name: &str) -> Option<String> {
    Some(
        match name {
            "LANGFUSE_SSO_ISSUER" => ISSUER,
            "LANGFUSE_SSO_AUTHORIZE_URL" => AUTHORIZE,
            "LANGFUSE_SSO_CLIENT_ID" => CLIENT_ID,
            "LANGFUSE_SSO_CLIENT_SECRET" => SECRET,
            "LANGFUSE_SSO_REDIRECT_URI" => REDIRECT,
            "LANGFUSE_SSO_IAP_AUDIENCE" => AUDIENCE,
            "LANGFUSE_SSO_SIGNING_KEY" => return Some(signing_key()),
            _ => return None,
        }
        .to_owned(),
    )
}

fn issuer(config: Config) -> Router {
    router_with_verifier(config, fixture::verifier(AUDIENCE))
}

fn app() -> Router {
    issuer(Config::from_lookup(settings).unwrap())
}

fn encode(value: &str) -> String {
    url::form_urlencoded::byte_serialize(value.as_bytes()).collect()
}

fn query(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(name, value)| format!("{name}={}", encode(value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn authorize_parameters() -> Vec<(&'static str, &'static str)> {
    vec![
        ("response_type", "code"),
        ("client_id", CLIENT_ID),
        ("redirect_uri", REDIRECT),
        ("scope", "openid email profile"),
        ("state", "state with spaces & symbols"),
        ("code_challenge", CHALLENGE),
        ("code_challenge_method", "S256"),
        ("nonce", "nonce-1"),
    ]
}

async fn authorize(
    app: &Router,
    pairs: &[(&str, &str)],
    headers: &[(&str, String)],
) -> Response<Body> {
    let mut request = Request::get(format!("/langfuse-sso/authorize?{}", query(pairs)));
    for (name, value) in headers {
        request = request.header(*name, value);
    }
    app.clone()
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

fn assertion() -> Vec<(&'static str, String)> {
    vec![(
        "x-coda-iap-assertion",
        fixture::identity_assertion(AUDIENCE, "accounts.google.com:77", "lu@codahealth.ai"),
    )]
}

async fn text(response: Response<Body>) -> String {
    String::from_utf8(
        to_bytes(response.into_body(), 64 * 1024)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap()
}

async fn json(response: Response<Body>) -> Value {
    serde_json::from_str(&text(response).await).unwrap()
}

/// The code from a successful authorize redirect, after checking the state.
async fn code(app: &Router) -> String {
    let response = authorize(app, &authorize_parameters(), &assertion()).await;
    assert_eq!(response.status(), StatusCode::FOUND);
    let location = Url::parse(response.headers()[header::LOCATION].to_str().unwrap()).unwrap();
    assert_eq!(&location[..url::Position::AfterPath], REDIRECT);
    let pairs: std::collections::HashMap<_, _> = location.query_pairs().into_owned().collect();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs["state"], "state with spaces & symbols");
    assert_eq!(URL_SAFE_NO_PAD.decode(&pairs["code"]).unwrap().len(), 32);
    pairs["code"].clone()
}

fn basic(id: &str, secret: &str) -> String {
    format!(
        "Basic {}",
        STANDARD.encode(format!("{}:{}", encode(id), encode(secret)))
    )
}

async fn exchange(
    app: &Router,
    authorization: Option<String>,
    form: &[(&str, &str)],
) -> Response<Body> {
    let mut request = Request::post("/langfuse-sso/token")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    if let Some(authorization) = authorization {
        request = request.header(header::AUTHORIZATION, authorization);
    }
    app.clone()
        .oneshot(request.body(Body::from(query(form))).unwrap())
        .await
        .unwrap()
}

fn token_form(code: &str) -> Vec<(&'static str, &str)> {
    vec![
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", REDIRECT),
        ("code_verifier", VERIFIER),
    ]
}

async fn redeem(app: &Router, form: &[(&str, &str)]) -> Response<Body> {
    exchange(app, Some(basic(CLIENT_ID, SECRET)), form).await
}

async fn oauth_error(response: Response<Body>) -> (StatusCode, String) {
    let status = response.status();
    assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
    (
        status,
        json(response).await["error"].as_str().unwrap().to_owned(),
    )
}

async fn get(app: &Router, path: &str) -> Response<Body> {
    app.clone()
        .oneshot(Request::get(path).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn authorize_requires_public_sites_forwarded_assertion() {
    let app = app();
    let other = "/projects/123456789/locations/us-east1/services/observability-relay";
    let forged = {
        let random = ring::rand::SystemRandom::new();
        let pkcs8 = ring::signature::EcdsaKeyPair::generate_pkcs8(
            &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            &random,
        )
        .unwrap();
        let mut header = jsonwebtoken::Header::new(Algorithm::ES256);
        header.kid = Some(fixture::KID.into());
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        jsonwebtoken::encode(
            &header,
            &fixture::claims(AUDIENCE, now),
            &jsonwebtoken::EncodingKey::from_ec_der(pkcs8.as_ref()),
        )
        .unwrap()
    };
    let tampered = {
        let token = fixture::assertion(AUDIENCE);
        let (body, signature) = token.rsplit_once('.').unwrap();
        let flipped = if signature.starts_with('A') { "B" } else { "A" };
        format!("{body}.{flipped}{}", &signature[1..])
    };
    for headers in [
        vec![],
        vec![("x-coda-iap-assertion", "garbage".to_owned())],
        vec![("x-coda-iap-assertion", tampered)],
        vec![("x-coda-iap-assertion", forged)],
        vec![("x-coda-iap-assertion", fixture::assertion(other))],
        vec![("x-goog-iap-jwt-assertion", fixture::assertion(AUDIENCE))],
        vec![
            ("x-coda-iap-assertion", fixture::assertion(AUDIENCE)),
            ("x-coda-iap-assertion", fixture::assertion(AUDIENCE)),
        ],
    ] {
        let names: Vec<_> = headers.iter().map(|(name, _)| *name).collect();
        let response = authorize(&app, &authorize_parameters(), &headers).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN, "{names:?}");
        assert!(!response.headers().contains_key(header::LOCATION));
    }
}

#[tokio::test]
async fn authorize_refuses_bad_parameters_without_redirecting() {
    let app = app();
    let change = |name: &'static str, value: Option<&'static str>| {
        let mut pairs: Vec<_> = authorize_parameters()
            .into_iter()
            .filter(|(key, _)| *key != name)
            .collect();
        if let Some(value) = value {
            pairs.push((name, value));
        }
        pairs
    };
    let cases = [
        change("response_type", Some("token")),
        change("client_id", Some("other")),
        change("client_id", None),
        change(
            "redirect_uri",
            Some("https://evil.test/langfuse/api/auth/callback/custom"),
        ),
        change(
            "redirect_uri",
            Some("https://staging.codahealth.ai/langfuse/api/auth/callback/custom/"),
        ),
        change("redirect_uri", None),
        change("scope", Some("email profile")),
        change("scope", Some("openidx")),
        change("state", None),
        change("state", Some("")),
        change("code_challenge_method", Some("plain")),
        change("code_challenge_method", None),
        change("code_challenge", Some(VERIFIER.split_at(42).0)),
        change(
            "code_challenge",
            Some("E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-c="),
        ),
        change("code_challenge", None),
        [authorize_parameters(), vec![("state", "second")]].concat(),
    ];
    for pairs in cases {
        let response = authorize(&app, &pairs, &assertion()).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "{pairs:?}");
        assert!(!response.headers().contains_key(header::LOCATION));
    }
}

#[tokio::test]
async fn code_redeems_once_for_a_signed_id_token() {
    let app = app();
    let code = code(&app).await;
    let response = redeem(&app, &token_form(&code)).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
    let body = json(response).await;
    assert_eq!(body["token_type"], "Bearer");
    assert_eq!(body["expires_in"], 300);
    assert_eq!(
        URL_SAFE_NO_PAD
            .decode(body["access_token"].as_str().unwrap())
            .unwrap()
            .len(),
        32
    );

    let keys: JwkSet =
        serde_json::from_value(json(get(&app, "/langfuse-sso/.well-known/jwks.json").await).await)
            .unwrap();
    let id_token = body["id_token"].as_str().unwrap();
    let header = jsonwebtoken::decode_header(id_token).unwrap();
    assert_eq!(header.alg, Algorithm::ES256);
    let key = keys.find(header.kid.as_deref().unwrap()).unwrap();
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_issuer(&[ISSUER]);
    validation.set_audience(&[CLIENT_ID]);
    let claims =
        jsonwebtoken::decode::<Value>(id_token, &DecodingKey::from_jwk(key).unwrap(), &validation)
            .unwrap()
            .claims;
    assert_eq!(claims["sub"], "accounts.google.com:77");
    assert_eq!(claims["email"], "lu@codahealth.ai");
    assert_eq!(claims["name"], "lu@codahealth.ai");
    assert_eq!(claims["email_verified"], true);
    assert_eq!(claims["nonce"], "nonce-1");
    let iat = claims["iat"].as_u64().unwrap();
    assert_eq!(claims["exp"].as_u64().unwrap() - iat, 300);
    assert!(claims["auth_time"].as_u64().unwrap() <= iat);

    let (status, error) = oauth_error(redeem(&app, &token_form(&code)).await).await;
    assert_eq!(
        (status, error.as_str()),
        (StatusCode::BAD_REQUEST, "invalid_grant")
    );
}

#[tokio::test]
async fn nonce_is_optional() {
    let app = app();
    let pairs: Vec<_> = authorize_parameters()
        .into_iter()
        .filter(|(name, _)| *name != "nonce")
        .collect();
    let response = authorize(&app, &pairs, &assertion()).await;
    let location = Url::parse(response.headers()[header::LOCATION].to_str().unwrap()).unwrap();
    let code = location
        .query_pairs()
        .find(|(name, _)| name == "code")
        .unwrap()
        .1
        .into_owned();
    let body = json(redeem(&app, &token_form(&code)).await).await;
    let payload = body["id_token"]
        .as_str()
        .unwrap()
        .split('.')
        .nth(1)
        .unwrap();
    let claims: Value = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(payload).unwrap()).unwrap();
    assert!(claims.get("nonce").is_none());
}

#[tokio::test]
async fn expired_code_is_refused() {
    let app = issuer(
        Config::from_lookup(settings)
            .unwrap()
            .with_code_lifetime(Duration::ZERO),
    );
    let code = code(&app).await;
    let (status, error) = oauth_error(redeem(&app, &token_form(&code)).await).await;
    assert_eq!(
        (status, error.as_str()),
        (StatusCode::BAD_REQUEST, "invalid_grant")
    );
}

#[tokio::test]
async fn failed_redemptions_still_spend_the_code() {
    let app = app();
    let wrong_verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXX";
    for (name, value) in [
        ("code_verifier", wrong_verifier),
        ("code_verifier", "short"),
        ("code_verifier", ""),
        (
            "redirect_uri",
            "https://staging.codahealth.ai/langfuse/api/auth/callback/other",
        ),
    ] {
        let code = code(&app).await;
        let mut form = token_form(&code);
        form.retain(|(key, _)| *key != name);
        form.push((name, value));
        let (status, error) = oauth_error(redeem(&app, &form).await).await;
        assert_eq!(
            (status, error.as_str()),
            (StatusCode::BAD_REQUEST, "invalid_grant"),
            "{name}={value}"
        );
        let (_, error) = oauth_error(redeem(&app, &token_form(&code)).await).await;
        assert_eq!(
            error, "invalid_grant",
            "the code is spent after {name}={value}"
        );
    }
    let (_, error) = oauth_error(redeem(&app, &token_form("unknown")).await).await;
    assert_eq!(error, "invalid_grant");
    let code = code(&app).await;
    let mut form = token_form(&code);
    form[0].1 = "refresh_token";
    let (_, error) = oauth_error(redeem(&app, &form).await).await;
    assert_eq!(error, "unsupported_grant_type");
    assert_eq!(
        redeem(&app, &token_form(&code)).await.status(),
        StatusCode::OK
    );
}

#[tokio::test]
async fn token_endpoint_authenticates_the_client() {
    let app = app();
    let code = code(&app).await;
    for authorization in [
        None,
        Some(basic("other", SECRET)),
        Some(basic(
            CLIENT_ID,
            "a client secret of at least 32 bytes / with + reserved & chars!",
        )),
        Some(format!(
            "Basic {}",
            STANDARD.encode(format!("{CLIENT_ID}:{SECRET}"))
        )),
        Some(format!("Bearer {SECRET}")),
        Some("Basic not-base64".into()),
    ] {
        let response = exchange(&app, authorization.clone(), &token_form(&code)).await;
        assert_eq!(response.headers()[header::WWW_AUTHENTICATE], "Basic");
        let (status, error) = oauth_error(response).await;
        assert_eq!(
            (status, error.as_str()),
            (StatusCode::UNAUTHORIZED, "invalid_client"),
            "{authorization:?}"
        );
    }
    assert_eq!(
        redeem(&app, &token_form(&code)).await.status(),
        StatusCode::OK,
        "client failures leave the code"
    );
}

#[tokio::test]
async fn discovery_and_keys_describe_the_issuer() {
    let app = app();
    let response = get(&app, "/langfuse-sso/.well-known/openid-configuration").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        json(response).await,
        serde_json::json!({
            "issuer": ISSUER,
            "authorization_endpoint": AUTHORIZE,
            "token_endpoint": format!("{ISSUER}/token"),
            "jwks_uri": format!("{ISSUER}/.well-known/jwks.json"),
            "response_types_supported": ["code"],
            "subject_types_supported": ["public"],
            "id_token_signing_alg_values_supported": ["ES256"],
            "token_endpoint_auth_methods_supported": ["client_secret_basic"],
            "code_challenge_methods_supported": ["S256"],
            "scopes_supported": ["openid", "email", "profile"],
            "grant_types_supported": ["authorization_code"],
        })
    );
    let keys = json(get(&app, "/langfuse-sso/.well-known/jwks.json").await).await;
    let keys = keys["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 1);
    let key = &keys[0];
    let public = &fixture::key_file()["keys"][0];
    assert_eq!(key["kty"], "EC");
    assert_eq!(key["crv"], "P-256");
    assert_eq!(key["x"], public["x"]);
    assert_eq!(key["y"], public["y"]);
    assert_eq!(key["alg"], "ES256");
    assert_eq!(key["use"], "sig");
    // RFC 7638's exact thumbprint input, written out independently.
    let thumbprint = format!(
        r#"{{"crv":"P-256","kty":"EC","x":"{}","y":"{}"}}"#,
        public["x"].as_str().unwrap(),
        public["y"].as_str().unwrap()
    );
    assert_eq!(
        key["kid"],
        URL_SAFE_NO_PAD.encode(ring::digest::digest(
            &ring::digest::SHA256,
            thumbprint.as_bytes()
        ))
    );
    assert!(key.get("d").is_none());
    for path in [
        "/",
        "/langfuse-sso",
        "/langfuse-sso/",
        "/.well-known/openid-configuration",
        "/langfuse-sso/userinfo",
        "/langfuse-ssox/authorize",
    ] {
        assert_eq!(
            get(&app, path).await.status(),
            StatusCode::NOT_FOUND,
            "{path}"
        );
    }
    let post = Request::post("/langfuse-sso/authorize")
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        app.clone().oneshot(post).await.unwrap().status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        get(&app, "/langfuse-sso/token").await.status(),
        StatusCode::NOT_FOUND
    );
}

#[test]
fn configuration_is_required_and_validated() {
    let load = |setting: &'static str, value: Option<&str>| {
        let value = value.map(str::to_owned);
        Config::from_lookup(move |name| {
            if name == setting {
                value.clone()
            } else {
                settings(name)
            }
        })
    };
    let rejects = |setting: &'static str, value: Option<&str>| {
        let error = load(setting, value).unwrap_err();
        assert!(
            error.to_string().starts_with(setting),
            "{setting}={value:?}: {error}"
        );
        error
    };
    for setting in [
        "LANGFUSE_SSO_ISSUER",
        "LANGFUSE_SSO_AUTHORIZE_URL",
        "LANGFUSE_SSO_CLIENT_ID",
        "LANGFUSE_SSO_CLIENT_SECRET",
        "LANGFUSE_SSO_REDIRECT_URI",
        "LANGFUSE_SSO_SIGNING_KEY",
        "LANGFUSE_SSO_IAP_AUDIENCE",
    ] {
        for value in [None, Some(""), Some("  ")] {
            assert!(
                matches!(rejects(setting, value), ConfigError::Missing { setting: s } if s == setting)
            );
        }
    }
    for issuer in [
        "http://langfuse-sso.local:3000",
        "http://langfuse-sso.local:3000/",
        "http://langfuse-sso.local:3000/langfuse-sso/",
        "http://langfuse-sso.local:3000/langfuse-sso?x=1",
        "http://langfuse-sso.local:3000/langfuse-sso#x",
        "http://user:pass@langfuse-sso.local:3000/langfuse-sso",
        "HTTP://langfuse-sso.local:3000/langfuse-sso",
        "ftp://langfuse-sso.local/langfuse-sso",
    ] {
        rejects("LANGFUSE_SSO_ISSUER", Some(issuer));
    }
    let error = rejects("LANGFUSE_SSO_REDIRECT_URI", Some("not a url"));
    assert!(matches!(error, ConfigError::Unparsable { .. }));
    assert!(std::error::Error::source(&error).is_some());
    rejects(
        "LANGFUSE_SSO_AUTHORIZE_URL",
        Some("https://staging.codahealth.ai/a?b=c"),
    );
    assert!(matches!(
        rejects("LANGFUSE_SSO_CLIENT_SECRET", Some(&"s".repeat(31))),
        ConfigError::ShortSecret { .. }
    ));
    assert!(matches!(
        rejects("LANGFUSE_SSO_SIGNING_KEY", Some("%%%")),
        ConfigError::KeyEncoding { .. }
    ));
    let error = rejects(
        "LANGFUSE_SSO_SIGNING_KEY",
        Some(&STANDARD.encode(b"not a key")),
    );
    assert!(matches!(error, ConfigError::Key { .. }));
    assert!(std::error::Error::source(&error).is_some());
    assert!(matches!(
        rejects("LANGFUSE_SSO_IAP_AUDIENCE", Some("public-site")),
        ConfigError::Audience { .. }
    ));
    let config = Config::from_lookup(settings).unwrap();
    let debug = format!("{config:?}");
    assert!(!debug.contains(SECRET), "{debug}");
    assert!(!debug.contains(&signing_key()[..40]), "{debug}");
}

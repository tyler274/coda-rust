//! Local signed-token fixtures only; no Identity Platform or model calls.
use async_trait::async_trait;
use duet_api::identity::{IdentityError, ProviderState, RevocationCheck, Verifier};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, encode};
use serde_json::{Value, json};
use std::collections::BTreeMap;
const NOW: i64 = 1_800_000_000;

#[tokio::test]
async fn signed_identity_resolves_the_same_durable_owner_after_reconnect() {
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            assert!(
                std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                "DUET_IDENTITY_TEST_DATABASE_URL is required"
            );
            return;
        }
    };
    let store = front_door::PostgresFrontDoorStore::connect(&url)
        .await
        .unwrap();
    store.migrate().await.unwrap();
    let (verifier, key) = fixture();
    let state = State {
        cutoff: 0,
        fail: false,
        disabled: false,
    };
    let mut body = claims("password");
    let unique = uuid::Uuid::new_v4().to_string();
    body["sub"] = json!(unique);
    body["email"] = json!(format!("{unique}@example.test"));
    let email_token = token(&key, &body);
    assert!(
        verifier
            .resolve(&email_token, NOW, &state, &store)
            .await
            .is_err()
    );
    let first = verifier
        .provision(&email_token, NOW, &state, &store)
        .await
        .unwrap();
    drop(store);
    let store = front_door::PostgresFrontDoorStore::connect(&url)
        .await
        .unwrap();
    // A securely linked Identity Platform provider preserves UID; email is not
    // used to discover a second account or to merge different subjects.
    body["firebase"]["sign_in_provider"] = json!("google.com");
    let google_token = token(&key, &body);
    assert_eq!(
        verifier
            .resolve(&google_token, NOW, &state, &store)
            .await
            .unwrap()
            .account_id,
        first.account_id
    );
    let principal = verifier.verify(&google_token, NOW, &state).await.unwrap();
    store
        .revoke_identity_sessions(&principal, NOW)
        .await
        .unwrap();
    drop(store);
    let store = front_door::PostgresFrontDoorStore::connect(&url)
        .await
        .unwrap();
    assert!(
        verifier
            .resolve(&email_token, NOW + 1, &state, &store)
            .await
            .is_err()
    );
    assert!(
        verifier
            .resolve(&google_token, NOW + 1, &state, &store)
            .await
            .is_err()
    );
}

struct State {
    cutoff: i64,
    fail: bool,
    disabled: bool,
}
#[async_trait]
impl RevocationCheck for State {
    async fn current(&self, project: &str, subject: &str) -> Result<ProviderState, IdentityError> {
        if self.fail {
            return Err(IdentityError::Unavailable);
        }
        Ok(ProviderState {
            project: project.into(),
            subject: subject.into(),
            disabled: self.disabled,
            valid_since: self.cutoff,
        })
    }
}
fn fixture() -> (Verifier, EncodingKey) {
    // Public, disposable TEST-ONLY key: never configure it as a runtime trust root.
    let public = include_str!("fixtures/identity-test-only.pub.pem");
    let private = include_str!("fixtures/identity-test-only.pem");
    let keys = BTreeMap::from([(
        "fixture".into(),
        DecodingKey::from_rsa_pem(public.as_bytes()).unwrap(),
    )]);
    (
        Verifier::new("duet-test".into(), keys, NOW + 3600).unwrap(),
        EncodingKey::from_rsa_pem(private.as_bytes()).unwrap(),
    )
}
fn claims(provider: &str) -> Value {
    json!({"iss":"https://securetoken.google.com/duet-test", "aud":"duet-test",
        "sub":"stable-subject", "iat":NOW-10, "exp":NOW+100, "auth_time":NOW-30,
        "email":"person@example.test", "email_verified":true,
        "firebase":{"sign_in_provider":provider}})
}
fn token(key: &EncodingKey, claims: &Value) -> String {
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("fixture".into());
    encode(&header, claims, key).unwrap()
}
#[tokio::test]
async fn approved_methods_share_one_verified_subject() {
    let (verifier, key) = fixture();
    let state = State {
        cutoff: 0,
        fail: false,
        disabled: false,
    };
    for provider in ["apple.com", "google.com", "password"] {
        let principal = verifier
            .verify(&token(&key, &claims(provider)), NOW, &state)
            .await
            .unwrap();
        assert_eq!(principal.subject, "stable-subject");
        assert_eq!(principal.authenticated_at, NOW - 30);
    }
}
#[tokio::test]
async fn malformed_foreign_expired_and_unapproved_tokens_fail_closed() {
    let (verifier, key) = fixture();
    let state = State {
        cutoff: 0,
        fail: false,
        disabled: false,
    };
    for (field, value) in [
        ("aud", json!("other-project")),
        ("iss", json!("https://accounts.google.com")),
        ("exp", json!(NOW)),
        ("iat", json!(NOW + 1)),
        ("auth_time", json!(NOW + 1)),
        ("email_verified", json!(false)),
        ("sub", json!("")),
        ("firebase", json!({"sign_in_provider":"anonymous"})),
        ("firebase", json!({"sign_in_provider":"custom"})),
        (
            "firebase",
            json!({"sign_in_provider":"password","tenant":"foreign"}),
        ),
    ] {
        let mut value_claims = claims("apple.com");
        value_claims[field] = value;
        assert!(
            verifier
                .verify(&token(&key, &value_claims), NOW, &state)
                .await
                .is_err(),
            "{field}"
        );
    }
    let mut unknown = Header::new(Algorithm::RS256);
    unknown.kid = Some("unknown".into());
    assert!(
        verifier
            .verify(
                &encode(&unknown, &claims("password"), &key).unwrap(),
                NOW,
                &state
            )
            .await
            .is_err()
    );
    let mut symmetric = Header::new(Algorithm::HS256);
    symmetric.kid = Some("fixture".into());
    let confused = encode(
        &symmetric,
        &claims("password"),
        &EncodingKey::from_secret(b"not-a-key"),
    )
    .unwrap();
    assert!(verifier.verify(&confused, NOW, &state).await.is_err());
    for token in [
        "dev:00000000-0000-0000-0000-000000000000",
        "iap-user@example.test",
        "garbage",
    ] {
        assert!(verifier.verify(token, NOW, &state).await.is_err());
    }
    let mut forged = token(&key, &claims("apple.com")).into_bytes();
    let signature_start = forged.iter().rposition(|byte| *byte == b'.').unwrap() + 1;
    forged[signature_start] = if forged[signature_start] == b'A' {
        b'B'
    } else {
        b'A'
    };
    assert!(
        verifier
            .verify(&String::from_utf8(forged).unwrap(), NOW, &state)
            .await
            .is_err()
    );
    assert!(
        verifier
            .verify(&token(&key, &claims("apple.com")), NOW + 3600, &state)
            .await
            .is_err()
    );
}
#[tokio::test]
async fn refresh_cannot_bypass_revocation_and_uncertainty_is_not_authority() {
    let (verifier, key) = fixture();
    let token = token(&key, &claims("password"));
    for state in [
        State {
            cutoff: NOW - 20,
            fail: false,
            disabled: false,
        },
        State {
            cutoff: 0,
            fail: true,
            disabled: false,
        },
        State {
            cutoff: 0,
            fail: false,
            disabled: true,
        },
    ] {
        assert!(verifier.verify(&token, NOW, &state).await.is_err());
    }
}

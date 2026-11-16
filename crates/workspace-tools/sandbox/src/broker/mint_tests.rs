//! `broker mint model-proxy` against a stub of the proxy's admin routes.

use std::path::Path;

use serde_json::json;

use super::commands::{Stored, mint_model_proxy};
use super::error::BrokerError;
use super::keyring::{Keyring as _, Memory};
use super::{Item, Secret};
use crate::config::Config;
use crate::fake_http::{Call, FakeHttp};

const CONFIG: &str = include_str!("../../config/cyrene.toml");
/// Shaped like an ungate client key. Fake.
const NEW_KEY: &str = "ungk_FakeNewKeyForUnitTestsOnly";
const OLD_KEY: &str = "ungk_FakeOldKeyForUnitTestsOnly";

fn client(id: &str) -> serde_json::Value {
    json!({ "id": id, "kind": "minted", "label": "coda-agent", "createdAt": 1, "revokedAt": null })
}

fn created() -> serde_json::Value {
    json!({ "client": client("new"), "key": NEW_KEY, "clients": [client("new")] })
}

/// Cyrene's config pointed at `proxy`, with a broker socket nothing serves,
/// so the test never reloads a real broker.
fn config(proxy: &FakeHttp, dir: &Path) -> Config {
    let mut config = Config::parse(Path::new("cyrene.toml"), CONFIG).expect("cyrene parses");
    config.broker.socket = dir.join("broker.sock");
    config
        .model_proxy
        .as_mut()
        .expect("cyrene has a proxy")
        .admin_url = proxy.base.clone();
    config
}

fn mint(proxy: &FakeHttp, keyring: &Memory) -> Result<super::commands::Minted, BrokerError> {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = config(proxy, dir.path());
    let model_proxy = config.model_proxy.clone().expect("proxy");
    mint_model_proxy(&config, &model_proxy, keyring)
}

#[test]
fn mints_a_key_labelled_with_the_container_and_stores_it() {
    let proxy = FakeHttp::builder()
        .route("POST", "clients", 200, created())
        .start();
    let keyring = Memory::default();
    let minted = mint(&proxy, &keyring).expect("minted");
    assert_eq!(minted.client.label, "coda-agent");
    assert_eq!(minted.stored, Stored::BrokerNotRunning);
    assert!(minted.revoked.is_none());
    assert_eq!(
        keyring.lookup(Item::ModelProxy).expect("open"),
        Some(Secret::new(NEW_KEY).expect("token"))
    );
    assert_eq!(
        proxy.calls(),
        vec![Call {
            method: "POST".to_owned(),
            path: "clients".to_owned(),
            body: Some(json!({ "label": "coda-agent" })),
        }]
    );
}

#[test]
fn replacing_a_working_key_revokes_its_client() {
    let proxy = FakeHttp::builder()
        .route(
            "GET",
            "v1/client",
            200,
            json!({ "client": { "id": "old", "label": "coda-agent" } }),
        )
        .route("POST", "clients", 200, created())
        .route("POST", "clients/old/revoke", 200, json!({ "clients": [] }))
        .start();
    let keyring = Memory::with([(Item::ModelProxy, OLD_KEY)]);
    let minted = mint(&proxy, &keyring).expect("minted");
    assert_eq!(minted.revoked.map(|old| old.id).as_deref(), Some("old"));
    assert_eq!(
        proxy.paths(),
        [
            ("GET", "v1/client"),
            ("POST", "clients"),
            ("POST", "clients/old/revoke"),
        ]
        .map(|(method, path)| Call::new(method, path))
    );
    assert_eq!(
        keyring.lookup(Item::ModelProxy).expect("open"),
        Some(Secret::new(NEW_KEY).expect("token"))
    );
}

#[test]
fn a_stored_key_the_proxy_refuses_is_replaced_without_a_revoke() {
    let proxy = FakeHttp::builder()
        .route("GET", "v1/client", 403, json!({ "code": "forbidden" }))
        .route("POST", "clients", 200, created())
        .start();
    let keyring = Memory::with([(Item::ModelProxy, OLD_KEY)]);
    let minted = mint(&proxy, &keyring).expect("minted");
    assert!(minted.revoked.is_none());
    assert_eq!(proxy.paths().len(), 2);
}

#[test]
fn a_stored_value_that_is_not_a_token_is_replaced_without_asking_the_proxy() {
    let proxy = FakeHttp::builder()
        .route("POST", "clients", 200, created())
        .start();
    let keyring = Memory::default();
    keyring.set_not_a_token(Item::ModelProxy);
    let minted = mint(&proxy, &keyring).expect("minted");
    assert!(minted.revoked.is_none());
    assert_eq!(proxy.paths(), [Call::new("POST", "clients")]);
}

#[test]
fn a_proxy_that_cannot_say_whose_the_old_key_is_mints_nothing() {
    let proxy = FakeHttp::builder()
        .route("POST", "clients", 200, created())
        .start();
    let keyring = Memory::with([(Item::ModelProxy, OLD_KEY)]);
    let error = mint(&proxy, &keyring).expect_err("no /v1/client");
    assert!(
        matches!(error, BrokerError::ProxyStatus { status: 404, .. }),
        "{error:?}"
    );
    assert_eq!(proxy.paths(), vec![Call::new("GET", "v1/client")]);
}

#[test]
fn a_proxy_that_refuses_to_mint_leaves_the_keyring_alone() {
    let proxy = FakeHttp::builder()
        .route("GET", "v1/client", 403, json!({ "code": "forbidden" }))
        .route(
            "POST",
            "clients",
            403,
            json!({ "error": "Admin routes accept only direct local requests" }),
        )
        .start();
    let keyring = Memory::with([(Item::ModelProxy, OLD_KEY)]);
    let error = mint(&proxy, &keyring).expect_err("refused");
    let shown = crate::error::chain(&error);
    assert!(
        matches!(error, BrokerError::ProxyStatus { status: 403, .. }),
        "{shown}"
    );
    assert!(shown.contains("direct local requests"), "{shown}");
    assert_eq!(
        keyring.lookup(Item::ModelProxy).expect("open"),
        Some(Secret::new(OLD_KEY).expect("token"))
    );
}

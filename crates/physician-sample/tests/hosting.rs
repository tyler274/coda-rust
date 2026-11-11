//! Process-level packaging contracts, with no database, provider, or browser.

#[test]
fn hosted_startup_requires_a_qualified_profile_before_connecting_or_binding() {
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_physician-sample"))
        .env_clear()
        .env("K_SERVICE", "contract-test")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("hosted activation is not qualified"));
}

#[test]
fn production_refuses_to_start_even_with_the_staging_profile() {
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_physician-sample"))
        .env_clear()
        .env("K_SERVICE", "contract-test")
        .env("CODA_ENVIRONMENT", "prd")
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("PHYSICIAN_AUTH", "required")
        .env("PHYSICIAN_DATABASE_URL", "postgres://127.0.0.1/unused")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("hosted activation is not qualified"));
}

#[test]
fn product_migration_requires_explicit_database_authority() {
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_physician-migrate"))
        .env_clear()
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("PHYSICIAN_DATABASE_URL is required"));
}

#[test]
fn hosted_site_refuses_a_developers_chatgpt_plan() {
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_physician-sample"))
        .env_clear()
        .env("K_SERVICE", "test")
        .env("CODA_LOCAL_MODEL_EXECUTION", "subscription")
        .env("CODA_ENVIRONMENT", "stg")
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("PHYSICIAN_AUTH", "required")
        .env("PHYSICIAN_DATABASE_URL", "postgres://127.0.0.1/unused")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("only in local execution"));
}

//! Process-level hosting contracts, without a live database or provider.

#[test]
fn hosted_startup_rejects_unqualified_runtime_before_database_access() {
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_duet-api"))
        .env_clear()
        .env("K_SERVICE", "contract-test")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("hosted activation is not qualified"));
}

#[test]
fn production_refuses_to_start_even_with_the_staging_profile() {
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_duet-api"))
        .env_clear()
        .env("K_SERVICE", "contract-test")
        .env("CODA_ENVIRONMENT", "prd")
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("DATABASE_URL", "postgres://127.0.0.1/unused")
        .env("IDENTITY_PLATFORM_PROJECT_ID", "coda-rust-stg")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("hosted activation is not qualified"));
}

#[test]
fn product_migration_requires_explicit_database_authority() {
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_duet-migrate"))
        .env_clear()
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("DUET_MIGRATION_DATABASE_URL and DUET_RUNTIME_ROLE are required")
    );
}

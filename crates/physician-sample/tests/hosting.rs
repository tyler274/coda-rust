//! Process-level packaging contracts, with no database, provider, or browser.

#[test]
fn hosted_startup_rejects_simulation_before_connecting_or_binding() {
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_physician-sample"))
        .env_clear()
        .env("K_SERVICE", "contract-test")
        .arg("--simulate")
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

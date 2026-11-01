//! Runs the physician product's existing workspace-owned schema chains.
//! Auth, front-door, and durable conversation chains run here, never on startup.
//!
//! Under the staging-preview profile, `PHYSICIAN_STAGING_ACCOUNTS` also
//! ensures the preview sign-in accounts exist in the synthetic practice. Each
//! email gets its own stable physician id. Existing accounts are left as they
//! are; every other profile refuses the list.
//!
//! `physician-migrate load-charts` loads the built synthetic charts in
//! `PHYSICIAN_SYNTHETIC_CHARTS` into that practice and does nothing else. A
//! deploy never runs it. It needs the schema and takes the accounts already there,
//! whose panels start with every patient. It tries every patient and exits
//! failed if any did not load. Loading again changes only what the charts
//! changed and leaves panel choices alone.

use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::chart_load::load_synthetic_practice;
use physician_sample::hosting::{
    StagingAccount, staging_accounts, staging_physician_id, synthetic_charts,
};

#[tokio::main]
async fn main() {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").unwrap_or_default();
    if url.trim().is_empty() {
        eprintln!("PHYSICIAN_DATABASE_URL is required");
        std::process::exit(1);
    }
    match std::env::args().nth(1).as_deref() {
        None => {}
        Some("load-charts") => {
            if let Err(error) = load_charts(&url).await {
                eprintln!("synthetic charts not loaded: {error}");
                std::process::exit(1);
            }
            return;
        }
        Some(other) => {
            eprintln!("physician-migrate has no command {other}");
            std::process::exit(1);
        }
    }
    let accounts = match staging_accounts(|key| std::env::var(key).ok()) {
        Ok(accounts) => accounts,
        Err(error) => {
            eprintln!("physician migrations refused: {error}");
            std::process::exit(1);
        }
    };
    if let Err(error) = migrate(&url, &accounts).await {
        eprintln!("physician migrations failed: {error}");
        std::process::exit(1);
    }
}

async fn migrate(url: &str, accounts: &[StagingAccount]) -> Result<(), String> {
    let auth = physician_auth::PostgresAuthStore::connect_with_migrations(url)
        .await
        .map_err(|error| format!("physician-auth: {error}"))?;
    front_door::PostgresFrontDoorStore::connect_with_migrations(url)
        .await
        .map_err(|error| format!("front-door: {error}"))?;
    let runtime = local_product::runtime::Store::connect(url)
        .await
        .map_err(|error| format!("physician runtime connection: {error}"))?;
    runtime
        .migrate()
        .await
        .map_err(|error| format!("physician runtime migration: {error}"))?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default();
    for account in accounts {
        let created = auth
            .ensure_account(
                STAGING_PRACTICE_ID,
                &staging_physician_id(&account.email),
                &account.email,
                &account.display_name,
                account.admin,
                now,
            )
            .await
            .map_err(|error| format!("staging account {}: {error}", account.email))?;
        println!(
            "staging account {} {}",
            account.email,
            if created {
                "created"
            } else {
                "already present"
            }
        );
    }
    Ok(())
}

async fn load_charts(url: &str) -> Result<(), String> {
    let charts = synthetic_charts(|key| std::env::var(key).ok())?;
    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(url)
        .await
        .map_err(|error| format!("physician-auth: {error}"))?;
    let runtime = local_product::runtime::Store::connect(url)
        .await
        .map_err(|error| format!("physician runtime connection: {error}"))?;
    let summary = load_synthetic_practice(&charts, &auth, &runtime).await?;
    for failure in &summary.failed {
        eprintln!("synthetic patient failed: {failure}");
    }
    println!(
        "synthetic charts: {} patients, {} records, {} changed, {} failed",
        summary.patients,
        summary.records,
        summary.changed,
        summary.failed.len()
    );
    if summary.failed.is_empty() {
        Ok(())
    } else {
        Err(format!("{} patients failed", summary.failed.len()))
    }
}

//! Runs the physician product's existing workspace-owned schema chains.
//! Auth, front-door, and durable conversation chains run here, never on startup.
//!
//! Under the staging-preview profile, `PHYSICIAN_STAGING_ACCOUNTS` also
//! ensures the preview sign-in accounts exist in the recorded synthetic
//! practice, on the staging physician's panel. Existing accounts are left as
//! they are; every other profile refuses the list.

use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::hosting::{StagingAccount, staging_accounts};

/// The physician whose panel the server seeds with the fixture workspaces.
const STAGING_PHYSICIAN_ID: &str = "staging-physician";

#[tokio::main]
async fn main() {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").unwrap_or_default();
    if url.trim().is_empty() {
        eprintln!("PHYSICIAN_DATABASE_URL is required");
        std::process::exit(1);
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
    local_product::runtime::Store::connect(url)
        .await
        .map_err(|error| format!("physician runtime connection: {error}"))?
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
                STAGING_PHYSICIAN_ID,
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

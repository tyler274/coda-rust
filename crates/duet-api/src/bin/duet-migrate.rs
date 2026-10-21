//! Explicit migration-only entrypoint; never invoked by API startup.
//!
//! Applies the Duet workspace chain, then the front-door chain when
//! `FRONT_DOOR_DATABASE_URL` is set.

#[tokio::main]
async fn main() {
    let url = match std::env::var("DUET_MIGRATION_DATABASE_URL") {
        Ok(url) if !url.trim().is_empty() => url,
        _ => {
            eprintln!("DUET_MIGRATION_DATABASE_URL and DUET_RUNTIME_ROLE are required");
            std::process::exit(1);
        }
    };
    let role = match std::env::var("DUET_RUNTIME_ROLE") {
        Ok(role) if !role.trim().is_empty() => role,
        _ => {
            eprintln!("DUET_MIGRATION_DATABASE_URL and DUET_RUNTIME_ROLE are required");
            std::process::exit(1);
        }
    };
    if let Err(error) = duet_api::database::migrate(&url, &role).await {
        eprintln!("Duet migration coverage incomplete: {error}");
        std::process::exit(1);
    }
    for (variable, kind) in [
        (
            "DUET_HEALTH_CONTEXT_PRODUCER_ROLE",
            duet_api::producer_database::ProducerKind::HealthContext,
        ),
        (
            "DUET_ACCEPTED_WORK_PRODUCER_ROLE",
            duet_api::producer_database::ProducerKind::AcceptedWork,
        ),
    ] {
        if let Ok(producer) = std::env::var(variable) {
            if producer == role {
                eprintln!("producer login must differ from retained-reader runtime login");
                std::process::exit(1);
            }
            if let Err(error) = duet_api::database::migrate(&url, &producer).await {
                eprintln!("Duet migration coverage incomplete: {error}");
                std::process::exit(1);
            }
            if let Err(error) = duet_api::producer_database::grant(&url, &producer, kind).await {
                eprintln!("Duet producer grant incomplete: {error}");
                std::process::exit(1);
            }
        }
    }
    if let Ok(front_url) = std::env::var("FRONT_DOOR_DATABASE_URL") {
        if let Err(error) =
            front_door::PostgresFrontDoorStore::connect_with_migrations(&front_url).await
        {
            eprintln!("front-door migration coverage incomplete: {error}");
            std::process::exit(1);
        }
    }
    println!("Duet workspace migrations applied");
}

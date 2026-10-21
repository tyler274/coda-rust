//! Explicit migration-only entrypoint; never invoked by API startup.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("DUET_MIGRATION_DATABASE_URL")?;
    let role = std::env::var("DUET_RUNTIME_ROLE")?;
    duet_api::database::migrate(&url, &role).await?;
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
                return Err("producer login must differ from retained-reader runtime login".into());
            }
            duet_api::database::migrate(&url, &producer).await?;
            duet_api::producer_database::grant(&url, &producer, kind).await?;
        }
    }
    println!("Duet workspace migrations applied");
    Ok(())
}

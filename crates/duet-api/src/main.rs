use duet_api::{
    database::AccountDatabase,
    durable::{DurableState, router},
    evidence::EvidenceStore,
    identity::{IdentityPlatform, IdentityStore},
    privacy::{FileJournal, PrivacyStore},
};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    if let Err(reason) = duet_api::hosting::validate_startup(|key| env::var(key).ok()) {
        eprintln!("{reason}");
        std::process::exit(1);
    }
    let _telemetry = telemetry::init("duet-api");
    // Migrations are a separate operator command; runtime credentials cannot own schema.
    let url = env::var("DATABASE_URL").expect("runtime DATABASE_URL");
    let database = AccountDatabase::connect(&url)
        .await
        .expect("account database boundary");
    let verifier = IdentityPlatform::new(
        env::var("IDENTITY_PLATFORM_PROJECT_ID").expect("IDENTITY_PLATFORM_PROJECT_ID"),
        env::var("IDENTITY_PLATFORM_API_KEY").expect("IDENTITY_PLATFORM_API_KEY"),
    )
    .expect("identity verifier");
    let journal: Arc<dyn duet_api::privacy::ReceiptJournal> =
        if let Ok(bucket) = env::var("DUET_PRIVACY_JOURNAL_BUCKET") {
            Arc::new(duet_api::journal_gcs::GcsJournal::new(bucket).expect("privacy bucket"))
        } else {
            assert!(
                env::var("K_SERVICE").is_err(),
                "hosted serving requires independent GCS journal"
            );
            Arc::new(
                FileJournal::open(
                    env::var("DUET_PRIVACY_JOURNAL_DIRECTORY")
                        .expect("independent DUET_PRIVACY_JOURNAL_DIRECTORY")
                        .into(),
                )
                .await
                .expect("privacy journal"),
            )
        };
    let accepted_database = match env::var("DUET_ACCEPTED_WORK_DATABASE_URL") {
        Ok(url) => AccountDatabase::connect(&url)
            .await
            .expect("accepted-work producer boundary"),
        Err(_) => database.clone(),
    };
    let state = DurableState {
        identity: IdentityStore::new(database.clone(), Arc::new(verifier)),
        privacy: PrivacyStore::new(accepted_database.clone(), journal),
        evidence: EvidenceStore::new(database.clone()),
    };
    // No listener exists until the independent inventory is read and replayed.
    // Unsupported historical receipt schemas fail this gate, never reopen restored content.
    state
        .privacy
        .reconcile_inventory()
        .await
        .expect("privacy restore quarantine: independent journal replay required");
    use base64::Engine;
    let key: [u8; 32] = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(env::var("DUET_PUBLICATION_MASTER_KEY").expect("Doppler publication master key"))
        .expect("base64url key")
        .try_into()
        .expect("256-bit key");
    let conversation = duet_api::conversation_routes::ConversationState {
        durable: state.clone(),
        store: duet_api::conversation::ConversationStore::new(accepted_database.clone()),
        model: env::var("DUET_MODEL").unwrap_or_else(|_| "gpt-5".into()),
        provider: if env::var("CODA_ALLOW_MODEL_CALLS").as_deref() == Ok("1") {
            Some((
                "https://api.openai.com/v1/responses".into(),
                env::var("OPENAI_API_KEY").expect("authorized provider key"),
            ))
        } else {
            None
        },
    };
    let producer_router = if let Ok(url) = env::var("DUET_HEALTH_CONTEXT_DATABASE_URL") {
        assert!(
            env::var("DUET_ACCEPTED_WORK_DATABASE_URL").is_ok(),
            "producer routes require separately scoped accepted-work credentials"
        );
        duet_api::producer_routes::router(duet_api::producer_routes::ProducerState {
            durable: state.clone(),
            context: duet_api::health_context::HealthContextProducer::connect(&url)
                .await
                .expect("canonical context producer boundary"),
            conversation: conversation.clone(),
            today: duet_api::today::TodayStore::connect_producer(
                &env::var("DUET_ACCEPTED_WORK_DATABASE_URL").expect("accepted work producer URL"),
            )
            .await
            .expect("Today producer boundary"),
        })
    } else {
        axum::Router::new()
    };
    let doctor = duet_api::doctor_routes::DoctorState {
        durable: state.clone(),
        workspace: duet_api::workspace::WorkspaceStore::new(database.clone()),
        publications: duet_api::publication::PublicationStore::new(
            database,
            env::var("DUET_PUBLIC_ORIGIN").expect("public origin"),
            env::var("DUET_SHL_VIEWER_URL").expect("SHL viewer URL"),
            key,
        )
        .expect("publication configuration"),
    };
    tokio::spawn(duet_api::conversation_routes::recover_pending(
        conversation.clone(),
    ));
    tokio::spawn(duet_api::background::recover_privacy(state.privacy.clone()));
    let front_door = match env::var("FRONT_DOOR_DATABASE_URL") {
        Ok(front_url) => Some(Arc::new(
            front_door::PostgresFrontDoorStore::connect_without_migrations(&front_url)
                .await
                .expect("front-door database"),
        )),
        Err(_) => None,
    };
    let mut app = router(state.clone())
        .merge(duet_api::evidence_routes::router(state))
        .merge(duet_api::doctor_routes::router(doctor))
        .merge(duet_api::conversation_routes::router(conversation))
        .merge(producer_router)
        .route("/health", axum::routing::get(duet_api::hosting::live))
        .route(
            "/readyz",
            axum::routing::get({
                let front_door = front_door.clone();
                move || {
                    let front_door = front_door.clone();
                    async move { duet_api::hosting::ready(true, front_door.as_deref()).await }
                }
            }),
        )
        .layer(telemetry::http_layer());
    if let Some(directory) = env::var_os("DUET_SITE_DIR") {
        let directory = std::path::PathBuf::from(directory);
        for name in ["index.html", "duet-app.js", "duet-app_bg.wasm"] {
            assert!(directory.join(name).is_file(), "missing Duet asset: {name}");
        }
        app = app.merge(duet_api::site::router(directory));
    }
    let address = env::var("DUET_LISTEN_ADDR").unwrap_or_else(|_| {
        env::var("PORT")
            .map(|port| format!("0.0.0.0:{port}"))
            .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
    });
    let listener = tokio::net::TcpListener::bind(address).await.expect("bind");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("serve");
}

async fn shutdown() {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = terminate.recv() => {},
        }
    }
    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await.expect("shutdown signal");
}

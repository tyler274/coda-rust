use duet_api::{
    database::AccountDatabase,
    durable::{DurableState, router},
    evidence::EvidenceStore,
    identity::{IdentityPlatform, IdentityStore, IdentityVerifier, LocalIdentity},
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
    let verifier: Arc<dyn IdentityVerifier> = match env::var("DUET_LOCAL_IDENTITY_SECRET") {
        Ok(secret) => Arc::new(LocalIdentity::new(&secret).expect("local identity secret")),
        Err(_) => Arc::new(
            IdentityPlatform::new(
                env::var("IDENTITY_PLATFORM_PROJECT_ID").expect("IDENTITY_PLATFORM_PROJECT_ID"),
                env::var("IDENTITY_PLATFORM_API_KEY").expect("IDENTITY_PLATFORM_API_KEY"),
            )
            .expect("identity verifier"),
        ),
    };
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
    let health_context_producer = if let Ok(url) = env::var("DUET_HEALTH_CONTEXT_DATABASE_URL") {
        assert!(
            env::var("DUET_ACCEPTED_WORK_DATABASE_URL").is_ok(),
            "producer routes require separately scoped accepted-work credentials"
        );
        Some(
            duet_api::health_context::HealthContextProducer::connect(&url)
                .await
                .expect("canonical context producer boundary"),
        )
    } else {
        None
    };
    let evidence = match &health_context_producer {
        Some(producer) => {
            EvidenceStore::new(database.clone()).with_health_context(producer.clone())
        }
        None => EvidenceStore::new(database.clone()),
    };
    let front_door = match env::var("FRONT_DOOR_DATABASE_URL") {
        Ok(front_url) => Some(Arc::new(
            front_door::PostgresFrontDoorStore::connect_without_migrations(&front_url)
                .await
                .expect("front-door database"),
        )),
        Err(_) => None,
    };
    let mut privacy = PrivacyStore::new(accepted_database.clone(), journal);
    if let Some(fd) = front_door.clone() {
        let fd_store: Arc<dyn front_door::FrontDoorStore> = fd;
        privacy = privacy.with_front_door_closer(Arc::new(
            duet_api::privacy::StoreFrontDoorConnectionCloser::new(fd_store),
        ));
    } else if let Ok(fd_url) = env::var("FRONT_DOOR_URL") {
        let op_token = env::var("FRONT_DOOR_OPERATOR_TOKEN")
            .expect("FRONT_DOOR_OPERATOR_TOKEN is required when FRONT_DOOR_URL is set");
        let closer = duet_api::privacy::HttpFrontDoorConnectionCloser::new(fd_url, op_token)
            .expect("valid FRONT_DOOR_URL");
        privacy = privacy.with_front_door_closer(Arc::new(closer));
    }
    let origin = env::var("DUET_PUBLIC_ORIGIN").expect("public origin");
    let mailer: Arc<dyn duet_api::email_auth::AuthMailer> =
        match env::var("DUET_MAIL_TRANSPORT").as_deref() {
            Ok("smtp-relay") => {
                let from = env::var("DUET_MAIL_FROM").expect("DUET_MAIL_FROM for smtp-relay");
                Arc::new(
                    duet_api::email_auth::SmtpRelayMailer::new(&from)
                        .expect("valid DUET_MAIL_FROM relay configuration"),
                )
            }
            _ => Arc::new(duet_api::email_auth::UnconfiguredMailer),
        };
    let email_auth =
        duet_api::email_auth::EmailAuthStore::new(database.clone(), mailer, origin.clone());
    let state = DurableState {
        identity: IdentityStore::with_email_auth(database.clone(), verifier, email_auth),
        privacy,
        evidence,
    };
    // No listener exists until the independent inventory is read and replayed.
    // Unsupported historical receipt schemas fail this gate, never reopen restored content.
    state
        .privacy
        .reconcile_inventory()
        .await
        .expect("privacy restore quarantine: independent journal replay required");
    let parked_routes =
        duet_api::assistant_harness::ParkedRoutesPolicy::from_env(|key| env::var(key).ok());
    let producer_router = if parked_routes.is_enabled() {
        if let Some(context) = health_context_producer {
            duet_api::producer_routes::router(duet_api::producer_routes::ProducerState {
                durable: state.clone(),
                context,
            })
        } else {
            axum::Router::new()
        }
    } else {
        axum::Router::new()
    };
    let portal_service = duet_api::portals::PortalService::new(
        database.clone(),
        state.evidence.clone(),
        duet_api::portals::VendorSwitches::from_env(|key| env::var(key).ok()),
        env::var("DUET_EPIC_CLIENT_ID").unwrap_or_default(),
        env::var("DUET_ORACLE_CLIENT_ID").unwrap_or_default(),
    );
    let portal_router = duet_api::portals::router(duet_api::portals::PortalState {
        durable: state.clone(),
        service: portal_service.clone(),
        diagnostics_token: env::var("DUET_PORTALS_DIAGNOSTICS_TOKEN").ok(),
    });
    let push_transport: Arc<dyn duet_api::push::PushTransport> =
        if let Ok(endpoint) = env::var("DUET_PUSH_RELAY_URL") {
            Arc::new(
                duet_api::push::HttpPushTransport::new(
                    &endpoint,
                    env::var("DUET_PUSH_RELAY_BEARER").ok(),
                )
                .expect("push relay configuration"),
            )
        } else {
            Arc::new(duet_api::push::UnconfiguredPushTransport)
        };
    let push_state = duet_api::push::PushState {
        durable: state.clone(),
        push: duet_api::push::PushStore::new(database.clone(), push_transport),
    };
    let clinical_library: Option<Arc<dyn duet_api::assistant_harness::ClinicalLibraryReader>> =
        env::var("DUET_CLINICAL_LIBRARY_URL").ok().map(|url| {
            Arc::new(
                duet_api::assistant_harness::HttpClinicalLibraryReader::new(&url)
                    .expect("clinical library URL"),
            ) as Arc<dyn duet_api::assistant_harness::ClinicalLibraryReader>
        });
    let operator_switch = match env::var("DUET_AI_OPERATOR_ENABLED").as_deref() {
        Ok("1" | "true") => duet_api::assistant_harness::AssistantOperatorSwitch::On,
        _ => duet_api::assistant_harness::AssistantOperatorSwitch::Off,
    };
    let assistant_store = duet_api::assistant_harness::AssistantFoundationStore::new(
        database.clone(),
        operator_switch,
        parked_routes,
        clinical_library,
    );
    let assistant_state = duet_api::assistant_harness::AssistantState {
        durable: state.clone(),
        assistant: assistant_store.clone(),
    };
    let doctor = duet_api::doctor_routes::DoctorState {
        durable: state.clone(),
        workspace: duet_api::workspace::WorkspaceStore::new(database.clone()),
    };
    tokio::spawn(duet_api::background::recover_privacy(state.privacy.clone()));
    tokio::spawn(duet_api::background::refresh_portals(portal_service));
    tokio::spawn(duet_api::background::prune_expired_memory(
        assistant_store,
        database.clone(),
    ));
    let connected_care_transport: Option<
        Arc<dyn duet_api::connected_care_sync::FrontDoorSyncTransport>,
    > = if let Ok(front_door_url) = env::var("FRONT_DOOR_URL") {
        Some(Arc::new(
            duet_api::connected_care_sync::HttpFrontDoorSyncTransport::new(&front_door_url)
                .expect("front-door URL for connected-care sync"),
        ))
    } else {
        front_door.clone().map(|store| {
            Arc::new(
                duet_api::connected_care_sync::DirectFrontDoorSyncTransport::new(
                    store,
                    Arc::new(|| chrono::Utc::now().timestamp().max(0) as u64),
                ),
            ) as Arc<dyn duet_api::connected_care_sync::FrontDoorSyncTransport>
        })
    };
    let connected_care_router = connected_care_transport.map(|transport| {
        duet_api::connected_care_sync::router(
            duet_api::connected_care_sync::ConnectedCareSyncState {
                durable: state.clone(),
                uploader: duet_api::connected_care_sync::ConnectedCareUploader::new(
                    database.clone(),
                    state.evidence.clone(),
                    transport,
                ),
            },
        )
    });
    let for_my_doctor_transport: Option<
        Arc<dyn duet_api::for_my_doctor::ForMyDoctorFrontDoorTransport>,
    > = if let Ok(front_door_url) = env::var("FRONT_DOOR_URL") {
        Some(Arc::new(
            duet_api::for_my_doctor::HttpFrontDoorShareTransport::new(&front_door_url)
                .expect("front-door URL for for-my-doctor"),
        ))
    } else {
        front_door.clone().map(|store| {
            Arc::new(duet_api::for_my_doctor::DirectFrontDoorShareTransport::new(
                store,
                Arc::new(|| chrono::Utc::now().timestamp().max(0) as u64),
            )) as Arc<dyn duet_api::for_my_doctor::ForMyDoctorFrontDoorTransport>
        })
    };
    let for_my_doctor_router =
        duet_api::for_my_doctor::router(duet_api::for_my_doctor::ForMyDoctorState {
            durable: state.clone(),
            store: duet_api::for_my_doctor::ForMyDoctorStore::new(
                database.clone(),
                for_my_doctor_transport,
            ),
        });
    let invitations_port = if let Some(fd) = front_door.clone() {
        Some(duet_api::connected_care_invitations::FrontDoorInvitationPort::Direct(fd))
    } else if let Ok(fd_url) = env::var("FRONT_DOOR_URL") {
        let op_token = env::var("FRONT_DOOR_OPERATOR_TOKEN")
            .expect("FRONT_DOOR_OPERATOR_TOKEN is required when FRONT_DOOR_URL is set");
        Some(
            duet_api::connected_care_invitations::FrontDoorInvitationPort::Http(
                duet_api::connected_care_invitations::HttpFrontDoorInvitations::new(
                    fd_url, op_token,
                )
                .expect("valid FRONT_DOOR_URL"),
            ),
        )
    } else {
        None
    };
    let mut app = router(state.clone())
        .merge(duet_api::evidence_routes::router(state.clone()))
        .merge(duet_api::push::router(push_state))
        .merge(duet_api::assistant_harness::router(assistant_state))
        .merge(duet_api::doctor_routes::router(doctor))
        .merge(for_my_doctor_router)
        .merge(duet_api::connected_care_invitations::router(
            duet_api::connected_care_invitations::ConnectedCareInvitationState {
                durable: state.clone(),
                front_door: invitations_port,
            },
        ))
        .merge(producer_router)
        .merge(portal_router);
    if let Some(connected_care_router) = connected_care_router {
        app = app.merge(connected_care_router);
    }
    if parked_routes.is_enabled() {
        app = app.merge(duet_api::durable::parked_legacy_router(state));
    }
    let app = duet_api::hosting::with_ingress_guard(
        app.route("/health", axum::routing::get(duet_api::hosting::live))
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
            .layer(telemetry::http_layer()),
        http::ingress::IngressGuard::from_env(duet_api::evidence::MAX_EVIDENCE_BATCH_BYTES),
    );
    let address = env::var("DUET_LISTEN_ADDR").unwrap_or_else(|_| {
        env::var("PORT")
            .map(|port| format!("0.0.0.0:{port}"))
            .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
    });
    let listener = tokio::net::TcpListener::bind(address).await.expect("bind");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
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

use super::cloud::Backup;
use super::receipt::Persistence;
use super::*;

fn request(product: Product) -> Request {
    Request {
        product,
        backup: "1234".into(),
        target: format!("rust-restore-{}-test", product.name()),
    }
}
fn instance(name: &str) -> Instance {
    serde_json::from_value(serde_json::json!({
        "name": name, "project": PROJECT, "region": "us-central1", "state": "RUNNABLE",
        "databaseVersion": "POSTGRES_17", "instanceType": "CLOUD_SQL_INSTANCE",
        "createTime": "2026-01-02T03:04:05Z", "ipAddresses": [{"type": "PRIVATE"}],
        "settings": {"dataDiskSizeGb": "20", "storageAutoResize": true, "storageAutoResizeLimit": "50", "ipConfiguration": {"ipv4Enabled": false, "privateNetwork": NETWORK, "sslMode": "ENCRYPTED_ONLY"}}
    }))
    .unwrap()
}
fn operation(target: &str, kind: &str) -> Operation {
    Operation {
        name: match kind {
            "CREATE" => "11111111-1111-1111-1111-111111111111",
            "RESTORE_VOLUME" => "22222222-2222-2222-2222-222222222222",
            _ => "33333333-3333-3333-3333-333333333333",
        }
        .into(),
        target_id: target.into(),
        target_project: PROJECT.into(),
        operation_type: kind.into(),
        status: "DONE".into(),
        error: None,
    }
}
struct FakeCloud {
    instances: Vec<Instance>,
    backup: Backup,
    writes: Vec<String>,
    operation: Option<Operation>,
    bad_operation: bool,
    restore_failure: bool,
    command_failure: bool,
    malicious_operation: bool,
    reads: usize,
    race: bool,
}
impl FakeCloud {
    fn new(product: Product) -> Self {
        Self {
            instances: vec![instance(&product.source())],
            backup: Backup {
                id: "1234".into(),
                instance: product.source(),
                status: "SUCCESSFUL".into(),
                end_time: "2026-01-01T00:00:00Z".into(),
            },
            writes: vec![],
            operation: None,
            bad_operation: false,
            restore_failure: false,
            command_failure: false,
            malicious_operation: false,
            reads: 0,
            race: false,
        }
    }
}
impl Cloud for FakeCloud {
    fn instances(&mut self) -> Result<Vec<Instance>> {
        self.reads += 1;
        if self.race && self.reads == 2 {
            self.instances
                .push(instance(&request(Product::Duet).target));
        }
        Ok(self.instances.clone())
    }
    fn instance(&mut self, target: &str) -> Result<Instance> {
        self.instances
            .iter()
            .find(|i| i.name == target)
            .cloned()
            .ok_or_else(|| refuse("missing instance"))
    }
    fn backup(&mut self, _: &str, _: &str) -> Result<Backup> {
        Ok(self.backup.clone())
    }
    fn create(&mut self, receipt: &Receipt) -> Result<Operation> {
        self.writes.push("create".into());
        if self.command_failure {
            return Err(Error::Cloud {
                code: Some(123456789),
                stderr: "SECRET-cloud-stderr".into(),
            });
        }
        let mut target = instance(&receipt.target);
        target.settings.user_labels = [
            ("coda-purpose".into(), "restore-drill".into()),
            ("coda-product".into(), receipt.product.name().into()),
            ("coda-run".into(), receipt.run_id.clone()),
        ]
        .into();
        self.instances.push(target);
        let mut op = operation(&receipt.target, "CREATE");
        if self.malicious_operation {
            op.name = "SECRET-operation?token=private".into();
        }
        if self.bad_operation {
            op.target_id = receipt.source.clone();
        }
        self.operation = Some(op.clone());
        Ok(op)
    }
    fn restore(&mut self, receipt: &Receipt) -> Result<Operation> {
        self.writes.push("restore".into());
        let mut op = operation(&receipt.target, "RESTORE_VOLUME");
        if self.restore_failure {
            op.error = Some(
                serde_json::json!({"errors":[{"code":"SECRET-provider-code", "message":"SECRET-operation-detail"}]}),
            );
        }
        self.operation = Some(op.clone());
        Ok(op)
    }
    fn operation(&mut self, _: &str) -> Result<Operation> {
        Ok(self.operation.clone().unwrap())
    }
    fn delete(&mut self, target: &str) -> Result<Operation> {
        self.writes.push("delete".into());
        self.instances.retain(|i| i.name != target);
        let op = operation(target, "DELETE");
        self.operation = Some(op.clone());
        Ok(op)
    }
    fn pause(&mut self, _: Duration) {
        panic!("fake completed operations should not wait");
    }
}
#[derive(Default)]
struct Memory {
    snapshots: Vec<Receipt>,
    fail_at: Option<usize>,
    fail_recording_failure: bool,
}
impl Persistence for Memory {
    fn create(&mut self, r: &Receipt) -> Result<()> {
        self.save(r)
    }
    fn save(&mut self, r: &Receipt) -> Result<()> {
        if self.fail_at == Some(self.snapshots.len())
            || (self.fail_recording_failure && r.failure.is_some())
        {
            return Err(refuse("injected persistence failure"));
        }
        self.snapshots.push(r.clone());
        Ok(())
    }
    fn load(&mut self) -> Result<Receipt> {
        Ok(self.snapshots.last().unwrap().clone())
    }
}

#[test]
fn preflight_is_read_only_for_both_products() {
    for product in [Product::Physician, Product::Duet] {
        let mut cloud = FakeCloud::new(product);
        let mut store = Memory::default();
        restore(&mut cloud, &mut store, request(product), false).unwrap();
        assert!(cloud.writes.is_empty());
        assert_eq!(store.snapshots.last().unwrap().phase, Phase::Preflight);
    }
}
#[test]
fn restore_never_qualifies_and_cleanup_is_separate() {
    for product in [Product::Physician, Product::Duet] {
        let mut cloud = FakeCloud::new(product);
        let original = serde_json::to_value(&cloud.backup).unwrap();
        let mut store = Memory::default();
        restore(&mut cloud, &mut store, request(product), true).unwrap();
        assert_eq!(cloud.writes, ["create", "restore"]);
        let receipt = store.snapshots.last().unwrap();
        assert_eq!(receipt.phase, Phase::RestoredUnqualified);
        assert_eq!(receipt.cleanup, Cleanup::RequiredOrCreationUncertain);
        assert!(receipt.restore_elapsed_seconds.is_some());
        assert!(matches!(
            receipt.qualification,
            receipt::Qualification::PendingProductValidation
        ));
        cleanup(&mut cloud, &mut store).unwrap();
        assert_eq!(cloud.writes, ["create", "restore", "delete"]);
        assert_eq!(cloud.instances[0].name, product.source());
        assert_eq!(serde_json::to_value(&cloud.backup).unwrap(), original);
        assert_eq!(store.snapshots.last().unwrap().cleanup, Cleanup::Deleted);
    }
}
#[test]
fn refuses_all_existing_targets_even_with_plausible_prefix() {
    let mut cloud = FakeCloud::new(Product::Duet);
    cloud
        .instances
        .push(instance(&request(Product::Duet).target));
    assert!(
        restore(
            &mut cloud,
            &mut Memory::default(),
            request(Product::Duet),
            true
        )
        .is_err()
    );
    assert!(cloud.writes.is_empty());
}
#[test]
fn refuses_managed_and_invalid_targets_before_cloud_reads() {
    for target in [
        "rust-duet",
        "rust-physician",
        "prod-db",
        "rust-restore-duet-",
        "rust-restore-duet-a;evil",
        "rust-restore-physician-test",
    ] {
        let mut cloud = FakeCloud::new(Product::Duet);
        let mut req = request(Product::Duet);
        req.target = target.into();
        assert!(restore(&mut cloud, &mut Memory::default(), req, true).is_err());
        assert_eq!(cloud.reads, 0);
    }
}
#[test]
fn source_backup_and_private_configuration_are_validated() {
    for mutation in 0..7 {
        let mut cloud = FakeCloud::new(Product::Duet);
        match mutation {
            0 => cloud.instances[0].project = "production".into(),
            1 => cloud.instances[0].settings.ip_configuration.ipv4_enabled = true,
            2 => cloud.backup.instance = "rust-physician".into(),
            3 => cloud.backup.status = "FAILED".into(),
            4 => cloud.backup.end_time = "9999-01-01T00:00:00Z".into(),
            5 => cloud.backup.id = "9999".into(),
            _ => {
                cloud.instances[0].settings.ip_configuration.private_network =
                    "other-network".into()
            }
        }
        assert!(
            restore(
                &mut cloud,
                &mut Memory::default(),
                request(Product::Duet),
                true
            )
            .is_err()
        );
        assert!(cloud.writes.is_empty());
    }
}
#[test]
fn race_and_wrong_operation_fail_closed() {
    let mut cloud = FakeCloud::new(Product::Duet);
    cloud.race = true;
    assert!(
        restore(
            &mut cloud,
            &mut Memory::default(),
            request(Product::Duet),
            true
        )
        .is_err()
    );
    assert!(cloud.writes.is_empty());
    let mut cloud = FakeCloud::new(Product::Duet);
    cloud.bad_operation = true;
    assert!(
        restore(
            &mut cloud,
            &mut Memory::default(),
            request(Product::Duet),
            true
        )
        .is_err()
    );
    assert_eq!(cloud.writes, ["create"]);
}
#[test]
fn restore_error_keeps_cleanup_required_and_never_reports_completion() {
    let mut cloud = FakeCloud::new(Product::Duet);
    cloud.restore_failure = true;
    let mut store = Memory::default();
    assert!(restore(&mut cloud, &mut store, request(Product::Duet), true).is_err());
    let receipt = store.snapshots.last().unwrap();
    assert_eq!(receipt.phase, Phase::RestoreRequested);
    assert!(receipt.failure.is_some());
    assert!(receipt.restore_elapsed_seconds.is_none());
    assert_eq!(receipt.cleanup, Cleanup::RequiredOrCreationUncertain);
}
#[test]
fn failure_receipt_persistence_error_keeps_the_operation_error() {
    let mut cloud = FakeCloud::new(Product::Duet);
    cloud.restore_failure = true;
    let mut store = Memory {
        fail_recording_failure: true,
        ..Memory::default()
    };
    let error = restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap_err();
    let Error::Unrecorded {
        primary,
        persistence,
    } = &error
    else {
        panic!("persistence failure replaced the operation error: {error:?}");
    };
    assert!(matches!(**primary, Error::Operation { .. }));
    assert!(matches!(**persistence, Error::Unsafe(_)));
    assert_eq!(error.receipt_code(), receipt::FailureCode::CloudOperation);
    assert_eq!(cloud.writes, ["create", "restore"]);
}
#[test]
fn persistence_failure_stops_further_cloud_writes() {
    for fail_at in [0, 1, 2, 3, 4] {
        let mut cloud = FakeCloud::new(Product::Duet);
        let mut store = Memory {
            fail_at: Some(fail_at),
            ..Memory::default()
        };
        assert!(restore(&mut cloud, &mut store, request(Product::Duet), true).is_err());
        assert!(!cloud.writes.contains(&"restore".to_string()));
        if fail_at <= 1 {
            assert!(cloud.writes.is_empty());
        }
    }
}
#[test]
fn cleanup_refuses_replaced_public_or_mislabeled_target() {
    for mutation in 0..6 {
        let mut cloud = FakeCloud::new(Product::Duet);
        let mut store = Memory::default();
        restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap();
        match mutation {
            0 => cloud.instances[1].create_time = "changed".into(),
            1 => cloud.instances[1].settings.ip_configuration.ipv4_enabled = true,
            2 => cloud.instances[1].settings.user_labels.clear(),
            3 => {
                cloud.instances[1].settings.ip_configuration.ssl_mode =
                    "ALLOW_UNENCRYPTED_AND_ENCRYPTED".into()
            }
            4 => cloud.instances[1].settings.storage_auto_resize_limit = "0".into(),
            _ => store.snapshots.last_mut().unwrap().target = "rust-duet".into(),
        }
        assert!(cleanup(&mut cloud, &mut store).is_err());
        assert_eq!(cloud.writes, ["create", "restore"]);
    }
}
#[test]
fn malformed_or_truncated_receipts_and_overwrites_are_refused() {
    let root = std::env::temp_dir().join(format!(
        "restore-receipt-test-{}-{}",
        std::process::id(),
        epoch().unwrap()
    ));
    std::fs::create_dir(&root).unwrap();
    let path = root.join("receipt.jsonl");
    let mut store = ReceiptStore::new(path.clone());
    let receipt = preflight(&mut FakeCloud::new(Product::Duet), &request(Product::Duet)).unwrap();
    store.create(&receipt).unwrap();
    assert!(store.create(&receipt).is_err());
    assert_eq!(store.load().unwrap().target, receipt.target);
    std::fs::write(&path, "{\"schema_version\":1}").unwrap();
    assert!(store.load().is_err());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn accepts_only_canonical_network_self_link_or_resource_name() {
    let mut source = instance("rust-duet");
    source.settings.ip_configuration.private_network =
        format!("https://www.googleapis.com/compute/v1/{NETWORK}");
    assert!(source.staging_network());
    source.settings.ip_configuration.private_network =
        format!("https://attacker.invalid/compute/v1/{NETWORK}");
    assert!(!source.staging_network());
}
#[test]
fn receipts_exclude_provider_diagnostics_codes_and_untrusted_operation_ids() {
    for scenario in 0..3 {
        let mut cloud = FakeCloud::new(Product::Duet);
        cloud.command_failure = scenario == 0;
        cloud.restore_failure = scenario == 1;
        cloud.malicious_operation = scenario == 2;
        let mut store = Memory::default();
        let error = restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap_err();
        let saved = serde_json::to_string(&store.snapshots).unwrap();
        assert!(!saved.contains("SECRET"));
        assert!(!saved.contains("123456789"));
        assert!(!saved.contains("private"));
        assert!(store.snapshots.last().unwrap().failure.is_some());
        match scenario {
            0 => assert!(
                matches!(error, Error::Cloud { code: Some(123456789), stderr } if stderr == "SECRET-cloud-stderr")
            ),
            1 => assert!(
                matches!(error, Error::Operation { details } if details.to_string().contains("SECRET-provider-code"))
            ),
            _ => assert!(matches!(error, Error::Unsafe(_))),
        }
    }
    assert!(serde_json::from_str::<receipt::FailureCode>("\"SECRET-provider-code\"").is_err());
}
#[test]
fn failure_message_names_only_the_allowlisted_api_status() {
    let error = Error::Cloud {
        code: Some(1),
        stderr: "ERROR: (gcloud.sql.instances.create) RESOURCE_EXHAUSTED: SECRET-quota".into(),
    };
    let message = failure_message(&error);
    assert!(
        message
            .starts_with("staging restore failed: CloudCommand (API status RESOURCE_EXHAUSTED);")
    );
    assert!(!message.contains("SECRET"));
    let plain = failure_message(&Error::Cloud {
        code: Some(1),
        stderr: "SECRET".into(),
    });
    assert!(plain.starts_with("staging restore failed: CloudCommand;"));
}

#[test]
fn source_disk_outside_the_drill_bound_is_refused_before_writes() {
    for size in ["9", "51", "60", "unknown"] {
        let mut cloud = FakeCloud::new(Product::Duet);
        cloud.instances[0].settings.data_disk_size_gb = size.into();
        let mut store = Memory::default();
        assert!(matches!(
            restore(&mut cloud, &mut store, request(Product::Duet), true),
            Err(Error::Unsafe(_))
        ));
        assert!(cloud.writes.is_empty());
        assert!(store.snapshots.is_empty());
    }
}
#[test]
fn creation_command_uses_supported_beta_labels_tls_and_bounded_storage() {
    let mut cloud = FakeCloud::new(Product::Duet);
    cloud.instances[0].settings.data_disk_size_gb = "30".into();
    let receipt = preflight(&mut cloud, &request(Product::Duet)).unwrap();
    assert_eq!(receipt.storage_size_gb, 30);
    let args = cloud::create_args(&receipt);
    assert_eq!(&args[..4], ["beta", "sql", "instances", "create"]);
    for required in [
        "--no-assign-ip",
        "--ssl-mode=ENCRYPTED_ONLY",
        "--storage-auto-increase",
        "--storage-auto-increase-limit=50",
        "--storage-size=30",
        "--async",
    ] {
        assert!(args.iter().any(|arg| arg == required));
    }
    let labels = args.iter().position(|arg| arg == "--labels").unwrap();
    assert_eq!(
        args[labels + 1],
        format!(
            "coda-purpose=restore-drill,coda-product=duet,coda-run={}",
            receipt.run_id
        )
    );
}

/// Exercises the installed SDK parser without credentials, network calls or cloud writes.
#[test]
#[ignore = "requires the Nix dev shell gcloud SDK"]
fn installed_sdk_accepts_creation_flags() {
    let receipt = preflight(&mut FakeCloud::new(Product::Duet), &request(Product::Duet)).unwrap();
    let output = std::process::Command::new("timeout")
        .args(["--kill-after=5s", "30s", "gcloud"])
        .args(cloud::create_args(&receipt))
        .arg("--help")
        .env("CLOUDSDK_CORE_DISABLE_PROMPTS", "1")
        .env("PAGER", "cat")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let help = String::from_utf8_lossy(&output.stdout);
    for flag in [
        "--labels",
        "--ssl-mode",
        "--storage-auto-increase-limit",
        "--storage-size",
    ] {
        assert!(help.contains(flag), "missing supported flag {flag}");
    }
    // The staging release submits service updates asynchronously and polls readiness.
    let output = std::process::Command::new("timeout")
        .args(["--kill-after=5s", "30s", "gcloud"])
        .args(["run", "services", "update", "--help"])
        .env("CLOUDSDK_CORE_DISABLE_PROMPTS", "1")
        .env("PAGER", "cat")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("--async"));
}

#[test]
fn pending_operation_has_bounded_wait() {
    let mut cloud = FakeCloud::new(Product::Duet);
    let receipt = preflight(&mut cloud, &request(Product::Duet)).unwrap();
    let mut op = operation(&receipt.target, "CREATE");
    op.status = "RUNNING".into();
    cloud.operation = Some(op.clone());
    assert!(matches!(
        wait_bounded(&mut cloud, &op.name, &receipt, "CREATE", Duration::ZERO),
        Err(Error::Timeout)
    ));
}
#[test]
fn cli_rejects_unknown_duplicate_and_partial_authorization() {
    for args in [
        vec!["--project", "production"],
        vec!["--execute", "--execute"],
        vec!["--product", "other"],
    ] {
        assert!(Options::parse(&args.into_iter().map(str::to_owned).collect::<Vec<_>>()).is_err());
    }
    assert!(run_cli(&["--execute".into(), "--receipt".into(), "/unused".into()]).is_err());
    assert!(run_cli(&["--cleanup".into(), "--receipt".into(), "/unused".into()]).is_err());
}

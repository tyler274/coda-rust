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
fn operation(target: &RestoreTarget, kind: OperationKind) -> Operation {
    Operation {
        name: String::from(match kind {
            OperationKind::Create => "11111111-1111-1111-1111-111111111111",
            OperationKind::RestoreVolume => "22222222-2222-2222-2222-222222222222",
            OperationKind::Delete => "33333333-3333-3333-3333-333333333333",
        })
        .try_into()
        .unwrap(),
        target_id: target.as_str().into(),
        target_project: PROJECT.into(),
        operation_type: kind,
        status: Ok(OperationStatus::Done),
        error: None,
    }
}
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool injects one independent fault into the fake"
)]
struct FakeCloud {
    instances: Vec<Instance>,
    backup: Backup,
    writes: Vec<String>,
    operation: Option<Operation>,
    mutation_response: Option<(OperationKind, serde_json::Value)>,
    polls: usize,
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
                id: String::from("1234").try_into().unwrap(),
                instance: product.source(),
                status: "SUCCESSFUL".into(),
                end_time: "2026-01-01T00:00:00Z".into(),
            },
            writes: vec![],
            operation: None,
            mutation_response: None,
            polls: 0,
            bad_operation: false,
            restore_failure: false,
            command_failure: false,
            malicious_operation: false,
            reads: 0,
            race: false,
        }
    }
    fn mutation_result(&self, operation: Operation) -> Result<Operation> {
        if let Some((kind, response)) = &self.mutation_response
            && *kind == operation.operation_type
        {
            let raw: cloud::RawOperation = serde_json::from_value(response.clone())?;
            return raw.try_into();
        }
        Ok(operation)
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
    fn instance(&mut self, target: &RestoreTarget) -> Result<Instance> {
        self.instances
            .iter()
            .find(|i| i.name == target.as_str())
            .cloned()
            .ok_or_else(|| refuse("missing instance"))
    }
    fn backup(&mut self, _: Product, _: &BackupId) -> Result<Backup> {
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
        let mut target = instance(receipt.target.as_str());
        target.settings.user_labels = [
            ("coda-purpose".into(), "restore-drill".into()),
            ("coda-product".into(), receipt.product.name().into()),
            ("coda-run".into(), receipt.run_id.clone()),
        ]
        .into();
        self.instances.push(target);
        let mut op = operation(&receipt.target, OperationKind::Create);
        if self.malicious_operation {
            return cloud::RawOperation {
                name: "SECRET-operation?token=private".into(),
                target_id: receipt.target.as_str().into(),
                target_project: PROJECT.into(),
                operation_type: "CREATE".into(),
                status: "DONE".into(),
                error: None,
            }
            .try_into();
        }
        if self.bad_operation {
            op.target_id = receipt.source.clone();
        }
        self.operation = Some(op.clone());
        self.mutation_result(op)
    }
    fn restore(&mut self, receipt: &Receipt) -> Result<Operation> {
        self.writes.push("restore".into());
        let mut op = operation(&receipt.target, OperationKind::RestoreVolume);
        if self.restore_failure {
            op.error = Some(
                serde_json::json!({"errors":[{"code":"SECRET-provider-code", "message":"SECRET-operation-detail"}]}),
            );
        }
        self.operation = Some(op.clone());
        self.mutation_result(op)
    }
    fn operation(&mut self, id: &SqlOperationId) -> Result<Operation> {
        self.polls += 1;
        let operation = self.operation.clone().unwrap();
        assert_eq!(&operation.name, id);
        Ok(operation)
    }
    fn delete(&mut self, target: &RestoreTarget) -> Result<Operation> {
        self.writes.push("delete".into());
        self.instances.retain(|i| i.name != target.as_str());
        let op = operation(target, OperationKind::Delete);
        self.operation = Some(op.clone());
        self.mutation_result(op)
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
    fail_recording_restore: bool,
}
impl Persistence for Memory {
    fn create(&mut self, r: &Receipt) -> Result<()> {
        self.save(r)
    }
    fn save(&mut self, r: &Receipt) -> Result<()> {
        if self.fail_at == Some(self.snapshots.len())
            || (self.fail_recording_failure && r.failure.is_some())
            || (self.fail_recording_restore && r.restore_operation.is_some())
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
            5 => cloud.backup.id = String::from("9999").try_into().unwrap(),
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
fn unknown_status_response(kind: OperationKind) -> serde_json::Value {
    let (id, kind) = match kind {
        OperationKind::Create => ("11111111-1111-1111-1111-111111111111", "CREATE"),
        OperationKind::RestoreVolume => ("22222222-2222-2222-2222-222222222222", "RESTORE_VOLUME"),
        OperationKind::Delete => ("33333333-3333-3333-3333-333333333333", "DELETE"),
    };
    serde_json::json!({
        "name": id, "targetId": request(Product::Duet).target,
        "targetProject": PROJECT, "operationType": kind, "status": "UNKNOWN"
    })
}

#[test]
fn unknown_mutation_status_retains_identity_without_further_mutation() {
    for kind in [
        OperationKind::RestoreVolume,
        OperationKind::Create,
        OperationKind::Delete,
    ] {
        let mut cloud = FakeCloud::new(Product::Duet);
        let mut store = Memory::default();
        cloud.mutation_response = Some((kind, unknown_status_response(kind)));
        let error = if kind == OperationKind::Delete {
            restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap();
            cleanup(&mut cloud, &mut store).unwrap_err()
        } else {
            restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap_err()
        };
        assert_eq!(
            error.receipt_code(),
            receipt::FailureCode::ValidationRefused
        );
        let receipt = store.snapshots.last().unwrap();
        let (id, writes, polls) = match kind {
            OperationKind::Create => {
                assert_eq!(receipt.phase, Phase::CreateRequested);
                (&receipt.create_operation, vec!["create"], 0)
            }
            OperationKind::RestoreVolume => {
                assert_eq!(receipt.phase, Phase::RestoreRequested);
                assert_eq!(
                    receipt.failure,
                    Some(receipt::FailureCode::ValidationRefused)
                );
                assert!(receipt.restore_elapsed_seconds.is_none());
                (&receipt.restore_operation, vec!["create", "restore"], 1)
            }
            OperationKind::Delete => {
                assert_eq!(receipt.cleanup, Cleanup::DeleteRequested);
                (
                    &receipt.delete_operation,
                    vec!["create", "restore", "delete"],
                    3,
                )
            }
        };
        assert_eq!(id.as_ref(), Some(&cloud.operation.as_ref().unwrap().name));
        assert_eq!(cloud.writes, writes);
        assert_eq!(
            cloud.polls, polls,
            "unknown submission status must stop before polling"
        );
        if kind == OperationKind::RestoreVolume {
            cloud.operation.as_mut().unwrap().status = Err(types::UnknownOperationStatus);
            assert!(matches!(
                cleanup(&mut cloud, &mut store),
                Err(Error::UnknownOperationStatus(_))
            ));
            assert_eq!(cloud.writes, ["create", "restore"]);
            // The same validated operation is now DONE; explicit cleanup can recover.
            cloud.operation.as_mut().unwrap().status = Ok(OperationStatus::Done);
            cleanup(&mut cloud, &mut store).unwrap();
            assert_eq!(cloud.writes, ["create", "restore", "delete"]);
            assert_eq!(store.snapshots.last().unwrap().cleanup, Cleanup::Deleted);
        }
    }
}

#[test]
fn unknown_status_preserves_provider_error_and_identity() {
    for kind in [
        OperationKind::Create,
        OperationKind::RestoreVolume,
        OperationKind::Delete,
    ] {
        for fail_save in [false, true] {
            let mut cloud = FakeCloud::new(Product::Duet);
            let mut store = Memory::default();
            if kind == OperationKind::Delete {
                restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap();
            }
            if fail_save && kind == OperationKind::Delete {
                store.fail_at = Some(store.snapshots.len() + 1);
            } else {
                store.fail_recording_failure = fail_save;
            }
            let details = serde_json::json!({"errors": [{
                "code": "SYNTHETIC_PROVIDER_ERROR", "message": "synthetic operation failure"
            }]});
            let mut response = unknown_status_response(kind);
            response["error"] = details.clone();
            cloud.mutation_response = Some((kind, response));
            let error = if kind == OperationKind::Delete {
                cleanup(&mut cloud, &mut store).unwrap_err()
            } else {
                restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap_err()
            };
            let primary = if fail_save {
                let Error::Unrecorded {
                    primary,
                    persistence,
                } = &error
                else {
                    panic!("expected primary and persistence errors: {error:?}");
                };
                assert!(matches!(**persistence, Error::Unsafe(_)));
                primary.as_ref()
            } else {
                let receipt = store.snapshots.last().unwrap();
                let id = match kind {
                    OperationKind::Create => &receipt.create_operation,
                    OperationKind::RestoreVolume => &receipt.restore_operation,
                    OperationKind::Delete => &receipt.delete_operation,
                };
                assert_eq!(id.as_ref(), Some(&cloud.operation.as_ref().unwrap().name));
                if kind != OperationKind::Delete {
                    assert_eq!(receipt.failure, Some(receipt::FailureCode::CloudOperation));
                }
                &error
            };
            assert!(
                matches!(primary, Error::Operation { details: actual } if actual == &details),
                "provider error must take precedence over unknown status: {primary:?}"
            );
            let (writes, polls) = match kind {
                OperationKind::Create => (vec!["create"], 0),
                OperationKind::RestoreVolume => (vec!["create", "restore"], 1),
                OperationKind::Delete => (vec!["create", "restore", "delete"], 3),
            };
            assert_eq!(cloud.writes, writes);
            assert_eq!(cloud.polls, polls);
        }
    }
}

#[test]
fn unknown_mutation_status_does_not_persist_malformed_or_mismatched_identity() {
    for kind in [
        OperationKind::RestoreVolume,
        OperationKind::Create,
        OperationKind::Delete,
    ] {
        for (field, invalid) in [
            ("name", "malformed"),
            ("targetId", "rust-duet"),
            ("targetProject", "production"),
            ("operationType", "UPDATE"),
            (
                "operationType",
                if kind == OperationKind::Create {
                    "DELETE"
                } else {
                    "CREATE"
                },
            ),
        ] {
            let mut cloud = FakeCloud::new(Product::Duet);
            let mut store = Memory::default();
            let mut response = unknown_status_response(kind);
            response[field] = invalid.into();
            cloud.mutation_response = Some((kind, response));
            if kind == OperationKind::Delete {
                restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap();
                assert!(cleanup(&mut cloud, &mut store).is_err());
            } else {
                assert!(restore(&mut cloud, &mut store, request(Product::Duet), true).is_err());
            }
            let receipt = store.snapshots.last().unwrap();
            let (id, expected_writes) = match kind {
                OperationKind::Create => (&receipt.create_operation, vec!["create"]),
                OperationKind::RestoreVolume => {
                    (&receipt.restore_operation, vec!["create", "restore"])
                }
                OperationKind::Delete => (
                    &receipt.delete_operation,
                    vec!["create", "restore", "delete"],
                ),
            };
            assert!(id.is_none(), "{kind:?}: persisted invalid {field}");
            assert_eq!(cloud.writes, expected_writes);
        }
    }
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
fn failed_operation_survives_an_unwritable_operation_record() {
    let mut cloud = FakeCloud::new(Product::Duet);
    cloud.restore_failure = true;
    let mut store = Memory {
        fail_recording_restore: true,
        ..Memory::default()
    };
    let error = restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap_err();
    let Error::Unrecorded { primary, .. } = &error else {
        panic!("persistence failure replaced the operation error: {error:?}");
    };
    assert!(matches!(**primary, Error::Operation { .. }), "{primary:?}");
    assert_eq!(cloud.writes, ["create", "restore"]);
}
#[test]
fn reviewer_reproduction_failed_restore_with_sixth_save_unwritable() {
    // Saves: receipt creation, create requested, create operation, created,
    // restore requested, then the restore operation record (index 5).
    let mut cloud = FakeCloud::new(Product::Duet);
    cloud.restore_failure = true;
    let mut store = Memory {
        fail_at: Some(5),
        ..Memory::default()
    };
    let error = restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap_err();
    let Error::Unrecorded { primary, .. } = &error else {
        panic!("persistence failure replaced the operation error: {error:?}");
    };
    assert!(matches!(**primary, Error::Operation { .. }), "{primary:?}");
    assert_eq!(store.snapshots.len(), 5);
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
            _ => {
                store.snapshots.last_mut().unwrap().target =
                    RestoreTarget::new(Product::Physician, "rust-restore-physician-test".into())
                        .unwrap()
            }
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
    let mut op = operation(&receipt.target, OperationKind::Create);
    op.status = Ok(OperationStatus::Running);
    cloud.operation = Some(op.clone());
    assert!(matches!(
        wait_bounded(
            &mut cloud,
            &op.name,
            &receipt,
            OperationKind::Create,
            Duration::ZERO
        ),
        Err(Error::Timeout)
    ));
}
#[test]
fn validated_ids_preserve_string_identity_and_reject_invalid_input() {
    for id in ["", "-1", "1.0", "１２", "12/34", "123\n"] {
        assert!(BackupId::try_from(id.to_owned()).is_err());
        assert!(serde_json::from_value::<BackupId>(serde_json::json!(id)).is_err());
    }
    let backup: BackupId = serde_json::from_value(serde_json::json!("0001234")).unwrap();
    assert_eq!(backup.as_str(), "0001234");
    assert_eq!(serde_json::to_value(backup).unwrap(), "0001234");
    assert!(serde_json::from_value::<BackupId>(serde_json::json!(1234)).is_err());

    let id = "ABCDEF01-2345-6789-ABCD-EF0123456789";
    let operation: SqlOperationId = serde_json::from_value(serde_json::json!(id)).unwrap();
    assert_eq!(operation.as_str(), id);
    assert_eq!(serde_json::to_value(operation).unwrap(), id);
    for id in [
        "",
        "abcdef0123456789abcdef0123456789",
        "ABCDEF01_2345-6789-ABCD-EF0123456789",
        "GBCDEF01-2345-6789-ABCD-EF0123456789",
    ] {
        assert!(SqlOperationId::try_from(id.to_owned()).is_err());
        assert!(serde_json::from_value::<SqlOperationId>(serde_json::json!(id)).is_err());
    }
}

#[test]
fn selected_operations_reject_unknown_types_statuses_and_invalid_ids() {
    let value = serde_json::json!({
        "name": "11111111-1111-1111-1111-111111111111",
        "targetId": "rust-restore-duet-test", "targetProject": PROJECT,
        "operationType": "CREATE", "status": "DONE"
    });
    for (kind, expected_kind) in [
        ("CREATE", OperationKind::Create),
        ("RESTORE_VOLUME", OperationKind::RestoreVolume),
        ("DELETE", OperationKind::Delete),
    ] {
        for (status, expected_status) in [
            ("PENDING", OperationStatus::Pending),
            ("RUNNING", OperationStatus::Running),
            ("DONE", OperationStatus::Done),
        ] {
            let mut value = value.clone();
            value["operationType"] = kind.into();
            value["status"] = status.into();
            let raw: cloud::RawOperation = serde_json::from_value(value).unwrap();
            let selected = Operation::try_from(raw).unwrap();
            assert_eq!(selected.operation_type, expected_kind);
            assert_eq!(selected.status, Ok(expected_status));
        }
    }
    for (field, invalid) in [
        ("operationType", "UPDATE"),
        ("operationType", "create"),
        ("status", "UNKNOWN"),
        ("status", "done"),
        ("name", "invalid"),
    ] {
        let mut value = value.clone();
        value[field] = invalid.into();
        let raw: cloud::RawOperation = serde_json::from_value(value).unwrap();
        let selected = Operation::try_from(raw);
        if field == "status" {
            assert!(selected.unwrap().status.is_err());
        } else {
            assert!(selected.is_err());
        }
    }
}

#[test]
fn receipt_v2_round_trip_preserves_strings_and_timestamp_lexemes() {
    let mut receipt =
        preflight(&mut FakeCloud::new(Product::Duet), &request(Product::Duet)).unwrap();
    receipt.backup.id = String::from("0001234").try_into().unwrap();
    receipt.backup.end_time = "2026-01-01T01:00:00.000+01:00".into();
    receipt.target_created_at = Some("2026-01-02T03:04:05.000+00:00".into());
    let id: SqlOperationId = String::from("ABCDEF01-2345-6789-ABCD-EF0123456789")
        .try_into()
        .unwrap();
    receipt.create_operation = Some(id.clone());
    receipt.restore_operation = Some(id.clone());
    receipt.delete_operation = Some(id);
    let value = serde_json::to_value(&receipt).unwrap();
    assert_eq!(value["schema_version"], 2);
    assert_eq!(value["target"], "rust-restore-duet-test");
    assert_eq!(value["backup"]["id"], "0001234");
    for field in ["create_operation", "restore_operation", "delete_operation"] {
        assert_eq!(value[field], "ABCDEF01-2345-6789-ABCD-EF0123456789");
        let mut invalid = value.clone();
        invalid[field] = "invalid".into();
        assert!(serde_json::from_value::<Receipt>(invalid).is_err());
    }
    let loaded: Receipt = serde_json::from_value(value.clone()).unwrap();
    loaded.validate().unwrap();
    assert_eq!(serde_json::to_value(loaded).unwrap(), value);
    for target in [
        "rust-duet",
        "rust-restore-duet-",
        "rust-restore-duet-upperA",
        "rust-restore-duet-test-",
    ] {
        let mut invalid = value.clone();
        invalid["target"] = target.into();
        assert!(serde_json::from_value::<Receipt>(invalid).is_err());
    }
    let mut wrong_product = value;
    wrong_product["target"] = "rust-restore-physician-test".into();
    assert!(
        serde_json::from_value::<Receipt>(wrong_product)
            .unwrap()
            .validate()
            .is_err()
    );
}

#[test]
fn cleanup_retains_lexical_creation_identity_check() {
    let mut cloud = FakeCloud::new(Product::Duet);
    let mut store = Memory::default();
    restore(&mut cloud, &mut store, request(Product::Duet), true).unwrap();
    // Equal instants are not equal provider identity strings.
    cloud.instances[1].create_time = "2026-01-02T03:04:05+00:00".into();
    assert!(cleanup(&mut cloud, &mut store).is_err());
    assert_eq!(cloud.writes, ["create", "restore"]);
}

#[test]
fn target_types_do_not_restrict_unrelated_instance_listing() {
    let mut cloud = FakeCloud::new(Product::Duet);
    let mut unrelated = instance("unrelated-db");
    unrelated.state = "FUTURE_PROVIDER_STATE".into();
    unrelated.instance_type = "FUTURE_PROVIDER_TYPE".into();
    cloud.instances.push(unrelated);
    preflight(&mut cloud, &request(Product::Duet)).unwrap();
    let max = format!(
        "rust-restore-duet-{}",
        "a".repeat(63 - "rust-restore-duet-".len())
    );
    assert!(RestoreTarget::new(Product::Duet, max.clone()).is_ok());
    assert!(RestoreTarget::new(Product::Duet, format!("{max}a")).is_err());
    assert!(RestoreTarget::new(Product::Physician, max).is_err());
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

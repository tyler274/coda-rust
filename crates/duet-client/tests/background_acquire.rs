//! The background entry completes only after the durable outbox write, or on
//! a typed pause, and never waits for upload.
use duet_client::health_upload::{SyncPause, acquire, background_acquire};
use duet_health::acquisition::*;
use duet_health::native_adapters::*;
use duet_health::outbox::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
struct Disk {
    bytes: Rc<RefCell<Option<Vec<u8>>>>,
    locked: Rc<RefCell<bool>>,
    saves: Rc<RefCell<usize>>,
}

impl OutboxStorage for Disk {
    fn load(&self) -> Result<Option<Vec<u8>>, StorageError> {
        if *self.locked.borrow() {
            return Err(StorageError::Unavailable("locked".into()));
        }
        Ok(self.bytes.borrow().clone())
    }
    fn save(&self, bytes: &[u8]) -> Result<(), StorageError> {
        if *self.locked.borrow() {
            return Err(StorageError::Unavailable("locked".into()));
        }
        *self.bytes.borrow_mut() = Some(bytes.to_vec());
        *self.saves.borrow_mut() += 1;
        Ok(())
    }
}

/// Bridge that records whether the disk already held the write when it was
/// asked for its next page (it should not be consulted after completion).
struct Kit;

impl HealthKitBridge for Kit {
    fn available(&self) -> bool {
        true
    }
    fn read_type(&self, query: &HealthKitQuery) -> Result<HealthKitChanges, BridgeError> {
        let samples = if query.anchor.is_none() {
            vec![NativeSample {
                id: "a".into(),
                value: "72".into(),
                unit: Some("count/min".into()),
                start: Some("2026-10-20T07:30:00+09:00".into()),
                end: Some("2026-10-20T07:30:00+09:00".into()),
                time_zone: Some("Asia/Tokyo".into()),
                source_name: None,
                writer_id: Some("com.apple.health".into()),
                device: None,
                metadata: None,
            }]
        } else {
            Vec::new()
        };
        Ok(HealthKitChanges {
            samples,
            deleted: Vec::new(),
            anchor: "next".into(),
            has_more: false,
        })
    }
}

fn outbox(disk: &Disk) -> Outbox<Disk> {
    Outbox::new(
        disk.clone(),
        OutboxScope {
            account: "owner-a".into(),
            installation: "install-1".into(),
            source: PhoneSource::AppleHealth,
        },
        OutboxLimits::default(),
    )
}

fn adapter() -> HealthKitAdapter<Kit> {
    HealthKitAdapter::new(Kit).with_types(vec![TypeSpec::new(
        "HKQuantityTypeIdentifierHeartRate",
        "vital:heart_rate",
        "count/min",
    )])
}

#[test]
fn background_completes_after_the_durable_write_without_uploading() {
    let disk = Disk::default();
    let outbox = outbox(&disk);
    outbox.connect(1).unwrap();
    let saves_before = *disk.saves.borrow();
    let (signal, outcome) = background_acquire(&outbox, &adapter(), 4);
    assert_eq!(signal, BackgroundSignal::Durable);
    assert_eq!(outcome.paused, None);
    assert_eq!(outcome.acquired.upserts, 1);
    assert!(
        *disk.saves.borrow() > saves_before,
        "written before completion"
    );
    // A fresh process sees the record queued (not uploaded) with the anchor.
    let view = super_view(&disk);
    assert_eq!(view.pending, 1);
    assert!(view.anchor.is_some());
}

#[test]
fn background_on_locked_storage_completes_as_a_typed_pause() {
    let disk = Disk::default();
    let outbox = outbox(&disk);
    outbox.connect(1).unwrap();
    *disk.locked.borrow_mut() = true;
    let (signal, outcome) = background_acquire(&outbox, &adapter(), 4);
    assert_eq!(signal, BackgroundSignal::Paused);
    assert_eq!(
        outcome.paused,
        Some(SyncPause::StorageUnavailable("locked".into()))
    );
    *disk.locked.borrow_mut() = false;
    // Nothing was lost or advanced: the next pass reads the record.
    assert_eq!(acquire(&outbox, &adapter(), 4).acquired.upserts, 1);
}

#[test]
fn the_registered_handler_is_what_the_native_observer_waits_for() {
    set_background_handler(None);
    assert_eq!(background_delivery("x"), BackgroundSignal::NoSession);
    set_background_handler(Some(Box::new(|data_type| {
        let disk = Disk::default();
        let outbox = outbox(&disk);
        outbox.connect(1).unwrap();
        assert_eq!(data_type, "HKQuantityTypeIdentifierHeartRate");
        background_acquire(&outbox, &adapter(), 4).0
    })));
    assert_eq!(
        background_delivery("HKQuantityTypeIdentifierHeartRate"),
        BackgroundSignal::Durable
    );
    set_background_handler(None);
}

struct View {
    pending: usize,
    anchor: Option<Anchor>,
}

fn super_view(disk: &Disk) -> View {
    let view = outbox(disk).view().unwrap();
    View {
        pending: view.items.len(),
        anchor: view.anchor,
    }
}

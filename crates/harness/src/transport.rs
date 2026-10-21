//! Neutral transport utilities shared by structured and legacy CLI adapters.
//! Cassettes are transport evidence, never accepted-work or effect ledgers.

use crate::execution::{Cancellation, Provider, Request, Response, StreamEvent};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, VecDeque};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// SHA-256 of exact serialized logical request bytes.
pub fn request_key(value: &impl Serialize) -> Result<String, String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    Ok(Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

/// Atomically publishes synced bytes. Callers serialize writers themselves.
/// Failure after rename is ambiguous: reconcile the same bytes on readback.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let temp = path.with_extension(format!(
        "{}-{nonce}-{}.pending",
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let mut created = false;
    let result = (|| {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&temp).map_err(|e| e.to_string())?;
        created = true;
        file.write_all(bytes)
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        fs::rename(&temp, path).map_err(|e| e.to_string())?;
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    })();
    // Never remove a preexisting pending file belonging to another writer.
    if created {
        let _ = fs::remove_file(&temp);
    }
    result
}

/// FIFO occurrence matching: repeated identical requests consume distinct
/// exchanges; repeated execution identities return their original exchange.
pub struct ReplayQueue<T: Clone> {
    pending: HashMap<String, VecDeque<T>>,
    consumed: HashMap<String, (String, T)>,
}
impl<T: Clone> ReplayQueue<T> {
    /// Builds queues in record order.
    pub fn new(items: impl IntoIterator<Item = (String, T)>) -> Self {
        let mut pending: HashMap<String, VecDeque<T>> = HashMap::new();
        for (key, value) in items {
            pending.entry(key).or_default().push_back(value);
        }
        Self {
            pending,
            consumed: HashMap::new(),
        }
    }
    /// Takes exactly one matching occurrence, or returns the prior same call.
    pub fn take(&mut self, id: &str, key: &str) -> Result<T, String> {
        if let Some((prior, value)) = self.consumed.get(id) {
            return if prior == key {
                Ok(value.clone())
            } else {
                Err("replay identity changed request".into())
            };
        }
        let value = self
            .pending
            .get_mut(key)
            .and_then(VecDeque::pop_front)
            .ok_or("cassette has no matching unconsumed exchange")?;
        self.consumed.insert(id.into(), (key.into(), value.clone()));
        Ok(value)
    }
    /// Unconsumed exchanges.
    pub fn remaining(&self) -> usize {
        self.pending.values().map(VecDeque::len).sum()
    }
}

/// Structured cassette format, separate from the legacy text-only schema 1.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cassette {
    /// This structured format is version 2.
    pub schema_version: u32,
    /// Provider adapter identity.
    pub adapter: String,
    /// Explicit synthetic capture authorization reference.
    pub authorization: String,
    /// Must be true; real patient data must never enter test cassettes.
    pub synthetic_non_phi: bool,
    /// Ordered logical request and structured response pairs.
    pub exchanges: Vec<(Request, Response)>,
}

/// Offline structured transport; supports the same tool exchanges as live.
pub struct Replay {
    queue: ReplayQueue<Response>,
    next: u64,
}
impl Replay {
    /// Validates provenance and constructs an offline transport.
    pub fn new(cassette: Cassette) -> Result<Self, String> {
        if cassette.schema_version != 2
            || cassette.adapter.trim().is_empty()
            || cassette.authorization.trim().is_empty()
            || !cassette.synthetic_non_phi
        {
            return Err("invalid structured cassette provenance".into());
        }
        let items = cassette.exchanges.into_iter().map(|(request, mut response)| {
            response.provenance = serde_json::json!({"mode": "replay", "adapter": cassette.adapter,
                "authorization": cassette.authorization, "recorded": response.provenance});
            Ok((request_key(&request)?, response))
        }).collect::<Result<Vec<_>, String>>()?;
        Ok(Self {
            queue: ReplayQueue::new(items),
            next: 0,
        })
    }
    /// Remaining exchanges, for exact qualification assertions.
    pub fn remaining(&self) -> usize {
        self.queue.remaining()
    }
}
impl Provider for Replay {
    async fn exchange(
        &mut self,
        request: &Request,
        cancel: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        cancel.check()?;
        self.next += 1;
        let response = self
            .queue
            .take(&self.next.to_string(), &request_key(request)?)?;
        if !response.text.is_empty() {
            emit(StreamEvent::Text(response.text.clone()))?;
        }
        emit(StreamEvent::Usage(response.usage.clone()))?;
        cancel.check()?;
        Ok(response)
    }
}

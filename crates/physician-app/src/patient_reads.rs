//! The one owner of the two reads a patient page needs first: the workspace
//! (`GET /patient-workspaces/{id}`) and that patient's Today rows
//! (`GET /today?patient_workspace_id=`).
//!
//! A preload starts both reads for a patient the physician is approaching and
//! keeps their answers. Opening the patient then joins the read already under
//! way, or takes the answer that settled, instead of asking again. Each kept
//! read serves one open: taking it removes it, so every later reread of the
//! open page (a retry, an event, a reconnect) asks the server afresh.
//!
//! Reads here only read. What opening a patient does to the page, such as
//! marking it gone or forgetting its drafts, belongs to the open path that
//! awaits these answers, so a preload that is refused or finds no patient
//! changes nothing anyone can see.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dioxus::core::{Runtime, ScopeId, current_scope_id};
use futures_util::FutureExt;
use futures_util::future::{LocalBoxFuture, Shared};
use physician_client::{HttpError, PhysicianClient};
use physician_wire::{PatientWorkspaceResponse, TodayListResponse};

/// How long a settled preload stays good for the open that follows it. Past
/// this, opening reads again.
const KEPT_FOR_MS: f64 = 15_000.0;

/// One read's answer. The error is shared because a read under way can have
/// more than one caller waiting on it.
pub(crate) type Read<T> = Result<T, Rc<HttpError>>;

/// The patient reads of one signed-in shell.
#[derive(Clone)]
pub(crate) struct PatientReads {
    workspaces: Slots<PatientWorkspaceResponse>,
    today: Slots<TodayListResponse>,
}

impl PatientReads {
    /// Reads owned by the component running now: a preload's read lives as
    /// long as that component, not the row that started it.
    pub(crate) fn new() -> Self {
        let owner = current_scope_id();
        Self {
            workspaces: Slots::new(owner),
            today: Slots::new(owner),
        }
    }

    /// Drops every kept read, for when the page rereads everything.
    pub(crate) fn forget_all(&self) {
        self.workspaces.0.borrow_mut().clear();
        self.today.0.borrow_mut().clear();
    }

    /// Drops what is kept for patient `id`, for when the server says it changed.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn forget(&self, id: &str) {
        self.workspaces
            .0
            .borrow_mut()
            .retain(|key, _| key.workspace != id);
        self.today
            .0
            .borrow_mut()
            .retain(|key, _| key.workspace != id);
    }

    /// Starts both reads for patient `id` unless one is already kept, and
    /// lets them run to an answer even if nobody opens the patient.
    pub(crate) fn preload(&self, client: &PhysicianClient, id: &str) {
        let key = Key::new(client, id);
        let workspace = {
            let client = client.clone();
            let id = id.to_owned();
            move || async move { client.workspace(&id).await }.boxed_local()
        };
        let today = {
            let client = client.clone();
            let id = id.to_owned();
            move || async move { client.today_for(&id).await }.boxed_local()
        };
        self.workspaces.preload(key.clone(), workspace);
        self.today.preload(key, today);
    }

    /// Patient `id`'s workspace for the open that asks: the kept answer or
    /// the read under way, else a new read.
    pub(crate) async fn workspace(
        &self,
        client: &PhysicianClient,
        id: &str,
    ) -> Read<PatientWorkspaceResponse> {
        match self.workspaces.take(&Key::new(client, id)) {
            Some(kept) => kept.await,
            None => client.workspace(id).await.map_err(Rc::new),
        }
    }

    /// Patient `id`'s Today rows for the open that asks, as [`Self::workspace`].
    pub(crate) async fn today_for(
        &self,
        client: &PhysicianClient,
        id: &str,
    ) -> Read<TodayListResponse> {
        match self.today.take(&Key::new(client, id)) {
            Some(kept) => kept.await,
            None => client.today_for(id).await.map_err(Rc::new),
        }
    }
}

/// Which patient a read is for, and under which session it was asked: an
/// answer read for one session never serves another.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Key {
    session: Option<String>,
    workspace: String,
}

impl Key {
    fn new(client: &PhysicianClient, id: &str) -> Self {
        Self {
            session: client.session().get(),
            workspace: id.to_owned(),
        }
    }
}

/// A read under way that more than one caller can await.
type Joined<T> = Shared<LocalBoxFuture<'static, Read<T>>>;

/// One kept read.
enum Entry<T: Clone + 'static> {
    /// Asked and not yet answered; every caller joins this one request.
    InFlight(Joined<T>),
    /// Answered at `at` (milliseconds since the epoch).
    Settled { answer: T, at: f64 },
}

/// The kept reads of one kind, one per patient, and the component whose
/// lifetime a preload's read runs for.
struct Slots<T: Clone + 'static>(Rc<RefCell<HashMap<Key, Entry<T>>>>, ScopeId);

impl<T: Clone + 'static> Clone for Slots<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0), self.1)
    }
}

impl<T: Clone + 'static> Slots<T> {
    fn new(owner: ScopeId) -> Self {
        Self(Rc::default(), owner)
    }

    fn preload<F>(&self, key: Key, ask: impl FnOnce() -> F)
    where
        F: std::future::Future<Output = Result<T, HttpError>> + 'static,
    {
        let now = crate::now_millis() as f64;
        let request = {
            let mut entries = self.0.borrow_mut();
            entries.retain(|_, entry| fresh(entry, now));
            if entries.contains_key(&key) {
                return;
            }
            let request = ask()
                .map(|answer| answer.map_err(Rc::new))
                .boxed_local()
                .shared();
            entries.insert(key.clone(), Entry::InFlight(request.clone()));
            request
        };
        let slots = self.clone();
        // Nobody may await a preload, so it drives itself to an answer.
        Runtime::current().spawn(self.1, async move {
            let answer = request.clone().await;
            slots.settle(&key, &request, answer);
        });
    }

    /// Keeps a preload's answer, if its entry is still the one it started and
    /// no open took it. A failed read is not kept: the open reads again.
    fn settle(&self, key: &Key, request: &Joined<T>, answer: Read<T>) {
        let mut entries = self.0.borrow_mut();
        let ours = matches!(entries.get(key), Some(Entry::InFlight(kept)) if kept.ptr_eq(request));
        if !ours {
            return;
        }
        match answer {
            Ok(answer) => {
                let at = crate::now_millis() as f64;
                entries.insert(key.clone(), Entry::Settled { answer, at });
            }
            Err(_) => {
                entries.remove(key);
            }
        }
    }

    /// Removes and returns what is kept for `key`, to be awaited by one open.
    fn take(&self, key: &Key) -> Option<LocalBoxFuture<'static, Read<T>>> {
        let now = crate::now_millis() as f64;
        let entry = self.0.borrow_mut().remove(key)?;
        if !fresh(&entry, now) {
            return None;
        }
        Some(match entry {
            Entry::InFlight(request) => request.boxed_local(),
            Entry::Settled { answer, .. } => std::future::ready(Ok(answer)).boxed_local(),
        })
    }
}

/// Whether an entry may still serve an open at `now`: a read under way
/// always may, a settled one only within [`KEPT_FOR_MS`].
fn fresh<T: Clone + 'static>(entry: &Entry<T>, now: f64) -> bool {
    match entry {
        Entry::InFlight(_) => true,
        Entry::Settled { at, .. } => now - at < KEPT_FOR_MS,
    }
}

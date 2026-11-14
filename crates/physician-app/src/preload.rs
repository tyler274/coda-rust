//! When the shell starts reading a patient before it is opened: the moment
//! the physician's pointer or keyboard says which patient row is next.
//!
//! The trigger only decides when to ask. What is read, how a read under way
//! is joined and how long an answer is kept belong to
//! [`PatientReads`](crate::patient_reads::PatientReads).

use dioxus::prelude::*;
use physician_client::PhysicianClient;

use crate::patient_reads::PatientReads;

/// How long the pointer or keyboard focus rests on a row before its patient
/// is read: long enough that sweeping across rows reads none of them.
const DWELL_MS: u32 = 60;

/// What starts a patient read before the open.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PreloadTrigger {
    /// Nothing: the patient is read when opened.
    ClickOnly,
    /// The pointer resting on a row, or keyboard focus resting on it.
    #[default]
    HoverFocus,
}

impl PreloadTrigger {
    /// The trigger a `preload` address parameter names, if it names one.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "click" => Some(Self::ClickOnly),
            "hover" => Some(Self::HoverFocus),
            _ => None,
        }
    }

    fn rests(self) -> bool {
        match self {
            Self::ClickOnly => false,
            Self::HoverFocus => true,
        }
    }
}

/// Where the shell's preload signals go, carried into every row.
#[derive(Clone, Copy)]
pub(crate) struct Preloader {
    trigger: PreloadTrigger,
    reads: Signal<PatientReads>,
    client: Signal<PhysicianClient>,
    selected: Signal<Option<String>>,
    /// Counts approaches, so a dwell that something newer replaced ends.
    resting: CopyValue<u64>,
}

impl Preloader {
    pub(crate) fn new(
        trigger: PreloadTrigger,
        reads: Signal<PatientReads>,
        client: Signal<PhysicianClient>,
        selected: Signal<Option<String>>,
    ) -> Self {
        Self {
            trigger,
            reads,
            client,
            selected,
            resting: CopyValue::new(0),
        }
    }

    /// The pointer or focus came to patient `id`'s row; once it has rested
    /// there, the patient is read.
    pub(crate) fn approach(&self, id: &str) {
        if !self.trigger.rests() {
            return;
        }
        let mut resting = self.resting;
        let ticket = *resting.peek() + 1;
        resting.set(ticket);
        let this = *self;
        let id = id.to_owned();
        spawn(async move {
            crate::sleep_ms(DWELL_MS).await;
            if *this.resting.peek() == ticket {
                this.start(&id);
            }
        });
    }

    /// The pointer or focus left the row it rested on.
    pub(crate) fn leave(&self) {
        let mut resting = self.resting;
        let ticket = *resting.peek() + 1;
        resting.set(ticket);
    }

    fn start(&self, id: &str) {
        // The open patient is already read.
        if self.selected.peek().as_deref() == Some(id) {
            return;
        }
        let client = self.client.peek().clone();
        self.reads.peek().preload(&client, id);
    }
}

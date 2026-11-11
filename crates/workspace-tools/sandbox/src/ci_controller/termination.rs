//! SIGTERM handling. Before the controller holds its lock, SIGTERM exits 143
//! at once. Once it holds the lock, SIGTERM is recorded instead: a running
//! guest's process group is stopped, the pass unwinds through its cleanup
//! (job directory, JIT runner, the local request's result) and the process
//! then exits 143, as the Python controller's `SystemExit(143)` did.

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use signal_hook::consts::SIGTERM;

/// The exit status of a controller stopped by SIGTERM (128 + 15).
pub const TERMINATED_EXIT: i32 = 143;

#[derive(Debug, Clone)]
pub struct Termination {
    requested: Arc<AtomicBool>,
    /// While true, SIGTERM exits the process from the handler.
    exit_now: Arc<AtomicBool>,
}

impl Termination {
    /// Install the SIGTERM handlers for this process.
    pub fn install() -> io::Result<Self> {
        let this = Self::untracked();
        signal_hook::flag::register_conditional_shutdown(
            SIGTERM,
            TERMINATED_EXIT,
            Arc::clone(&this.exit_now),
        )?;
        signal_hook::flag::register(SIGTERM, Arc::clone(&this.requested))?;
        Ok(this)
    }

    /// A termination no signal sets; tests call [`Self::request`] instead.
    pub fn untracked() -> Self {
        Self {
            requested: Arc::new(AtomicBool::new(false)),
            exit_now: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn request(&self) {
        self.requested.store(true, Ordering::SeqCst);
    }

    pub fn requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

    /// Run `work` with SIGTERM recorded instead of exiting the process.
    pub fn deferred<T>(&self, work: impl FnOnce() -> T) -> T {
        let _restore = Restore(&self.exit_now);
        self.exit_now.store(false, Ordering::SeqCst);
        work()
    }
}

/// Re-arms immediate exit when the deferred work ends, even by panic.
struct Restore<'a>(&'a AtomicBool);

impl Drop for Restore<'_> {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

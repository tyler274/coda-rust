//! `broker serve`: one listening socket per [`Door`], one thread per
//! connection, SIGHUP.
//!
//! A connection gets [`REQUEST_DEADLINE`] to send its whole request, however
//! slowly it writes, and each door serves at most [`MAX_CONNECTIONS`] at
//! once, so a caller that stalls or floods one door holds a bounded number
//! of threads and never the other door.

use std::io::{Read as _, Write as _};
use std::os::unix::fs::{FileTypeExt as _, PermissionsExt as _};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use super::audit::Record;
use super::core::Broker;
use super::error::BrokerError;
use super::limit::LOG_EVERY;
use super::peer::{self, Door};
use super::wire::{Denied, MAX_REQUEST, Reply};

/// How long a client gets to send its whole request, measured from the
/// accept. A client that trickles bytes gets no more than one that sends
/// nothing.
pub const REQUEST_DEADLINE: Duration = Duration::from_secs(2);

/// How long the broker waits for a client to take its reply.
pub const WRITE_TIMEOUT: Duration = Duration::from_secs(2);

/// How many connections one door serves at once. A caller sends one
/// request per process and the rate limit allows a burst of ten, so honest
/// use stays far below this.
pub const MAX_CONNECTIONS: usize = 16;

/// A bound socket and the door it is.
pub struct Listener {
    door: Door,
    socket: UnixListener,
}

/// Binds `socket` as `door`, replacing a stale socket left by an earlier
/// broker. The socket is 0666: the directory around it decides who can
/// reach it, and the peer check decides what each caller gets.
pub fn bind(door: Door, socket: &Path) -> Result<Listener, BrokerError> {
    let listen = |source| BrokerError::Listen {
        socket: socket.to_owned(),
        source,
    };
    match std::fs::symlink_metadata(socket) {
        Ok(meta) if meta.file_type().is_socket() => std::fs::remove_file(socket).map_err(listen)?,
        Ok(_) => {
            return Err(BrokerError::NotASocket {
                socket: socket.to_owned(),
            });
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(listen(error)),
    }
    let listener = UnixListener::bind(socket).map_err(listen)?;
    std::fs::set_permissions(socket, std::fs::Permissions::from_mode(0o666)).map_err(listen)?;
    Ok(Listener {
        door,
        socket: listener,
    })
}

/// The connections a door is serving now.
#[derive(Default)]
struct Slots(Mutex<Open>);

#[derive(Default)]
struct Open {
    connections: usize,
    /// When a refusal for a full door was last logged.
    full_logged: Option<Instant>,
}

/// One connection's place among a door's [`MAX_CONNECTIONS`], given back
/// when dropped.
struct Slot(Arc<Slots>);

/// A door with every slot taken.
struct Full {
    /// Whether this refusal is due a journal line: the first, then one
    /// every [`LOG_EVERY`], so a flood does not fill the journal.
    log: bool,
}

impl Slots {
    fn open(&self) -> MutexGuard<'_, Open> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn take(self: &Arc<Self>, now: Instant) -> Result<Slot, Full> {
        let mut open = self.open();
        if open.connections < MAX_CONNECTIONS {
            open.connections += 1;
            return Ok(Slot(Arc::clone(self)));
        }
        let recently = open
            .full_logged
            .is_some_and(|at| now.saturating_duration_since(at) < LOG_EVERY);
        if !recently {
            open.full_logged = Some(now);
        }
        Err(Full { log: !recently })
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        self.0.open().connections -= 1;
    }
}

/// Serves `listener` until it fails. Each connection's audit line goes to
/// `audit`.
pub fn serve(
    listener: &Listener,
    broker: &Arc<Broker>,
    audit: &Arc<dyn Fn(&Record) + Send + Sync>,
) -> std::io::Error {
    let door = listener.door;
    let slots = Arc::new(Slots::default());
    loop {
        let stream = match listener.socket.accept() {
            Ok((stream, _)) => stream,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return error,
        };
        let accepted = Instant::now();
        let slot = match slots.take(accepted) {
            Ok(slot) => slot,
            Err(full) => {
                refuse(door, &stream, &full);
                continue;
            }
        };
        let broker = Arc::clone(broker);
        let audit = Arc::clone(audit);
        std::thread::spawn(move || {
            let _slot = slot;
            if let Err(error) = connection(door, &stream, accepted, &broker, audit.as_ref()) {
                eprintln!("broker: connection failed: {error}");
            }
        });
    }
}

/// Tells a client its door is full, on the accept loop's own thread: the
/// refusal is a few bytes into an empty socket buffer, so it does not wait
/// on the client.
fn refuse(door: Door, stream: &UnixStream, full: &Full) {
    if full.log {
        println!(
            "broker busy socket={}: {MAX_CONNECTIONS} connections are open; refusing more",
            door.name()
        );
    }
    let busy = Reply::Denied {
        reason: Denied::Busy,
    };
    // A client that is gone by now has nothing to learn from the refusal.
    let _ = write_reply(stream, &busy);
}

/// Reloads the broker's cache on every SIGHUP (`systemctl --user reload`).
pub fn reload_on_sighup(broker: &Arc<Broker>) -> Result<(), BrokerError> {
    let mut signals = signal_hook::iterator::Signals::new([signal_hook::consts::SIGHUP])
        .map_err(BrokerError::Signals)?;
    let broker = Arc::clone(broker);
    std::thread::spawn(move || {
        for _ in signals.forever() {
            broker.reload();
            println!("broker reload by SIGHUP");
        }
    });
    Ok(())
}

fn connection(
    door: Door,
    stream: &UnixStream,
    accepted: Instant,
    broker: &Broker,
    audit: &(dyn Fn(&Record) + Send + Sync),
) -> std::io::Result<()> {
    let peer = peer::of(stream)?;
    // A client that timed out, vanished or sent too much still counts as a
    // malformed request, so it shows in the audit log.
    let line = read_request(stream, accepted + REQUEST_DEADLINE).unwrap_or_default();
    let answer = broker.handle(door, peer, &line, Instant::now());
    if let Some(record) = &answer.record {
        audit(record);
    }
    write_reply(stream, &answer.reply)
}

/// Reads up to the first newline, the end of the stream or `deadline`,
/// whichever comes first.
fn read_request(mut stream: &UnixStream, deadline: Instant) -> std::io::Result<Vec<u8>> {
    let limit = usize::try_from(MAX_REQUEST).unwrap_or(usize::MAX);
    let mut line = Vec::new();
    let mut chunk = [0_u8; 512];
    loop {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(std::io::ErrorKind::TimedOut.into());
        }
        stream.set_read_timeout(Some(left))?;
        let read = match stream.read(&mut chunk) {
            Ok(0) => return Ok(line),
            Ok(read) => read,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        };
        let received = &chunk[..read];
        if let Some(end) = received.iter().position(|byte| *byte == b'\n') {
            line.extend_from_slice(&received[..end]);
            return Ok(line);
        }
        line.extend_from_slice(received);
        if line.len() > limit {
            return Err(std::io::ErrorKind::InvalidData.into());
        }
    }
}

fn write_reply(mut stream: &UnixStream, reply: &Reply) -> std::io::Result<()> {
    stream.set_write_timeout(Some(WRITE_TIMEOUT))?;
    let mut bytes = serde_json::to_vec(reply).map_err(std::io::Error::other)?;
    bytes.push(b'\n');
    stream.write_all(&bytes)?;
    stream.flush()
}

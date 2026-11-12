//! `broker serve`: the socket, one thread per connection, SIGHUP.

use std::io::{BufRead as _, BufReader, Read as _, Write as _};
use std::os::unix::fs::{FileTypeExt as _, PermissionsExt as _};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::audit::Record;
use super::core::Broker;
use super::error::BrokerError;
use super::peer;
use super::wire::{MAX_REQUEST, Reply};

/// How long a client gets to send its request, so a stuck one holds only
/// its own thread.
pub const READ_TIMEOUT: Duration = Duration::from_secs(2);

/// Binds `socket`, replacing a stale socket left by an earlier broker. The
/// socket is 0666: the directory around it decides who can reach it, and
/// the peer check decides what each caller gets.
pub fn bind(socket: &Path) -> Result<UnixListener, BrokerError> {
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
    Ok(listener)
}

/// Serves until the listener fails. Each connection's audit line goes to
/// `audit`.
pub fn serve(
    listener: &UnixListener,
    broker: &Arc<Broker>,
    audit: &Arc<dyn Fn(&Record) + Send + Sync>,
) -> std::io::Error {
    loop {
        let stream = match listener.accept() {
            Ok((stream, _)) => stream,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return error,
        };
        let broker = Arc::clone(broker);
        let audit = Arc::clone(audit);
        std::thread::spawn(move || {
            if let Err(error) = connection(stream, &broker, audit.as_ref()) {
                eprintln!("broker: connection failed: {error}");
            }
        });
    }
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
    stream: UnixStream,
    broker: &Broker,
    audit: &(dyn Fn(&Record) + Send + Sync),
) -> std::io::Result<()> {
    let peer = peer::of(&stream)?;
    stream.set_read_timeout(Some(READ_TIMEOUT))?;
    stream.set_write_timeout(Some(READ_TIMEOUT))?;
    let mut line = Vec::new();
    let read = BufReader::new((&stream).take(MAX_REQUEST)).read_until(b'\n', &mut line);
    let answer = match read {
        Ok(_) => broker.handle(peer, &line, Instant::now()),
        // A client that timed out or vanished still counts as a malformed
        // request, so it shows in the audit log.
        Err(_) => broker.handle(peer, b"", Instant::now()),
    };
    if let Some(record) = &answer.record {
        audit(record);
    }
    write_reply(&stream, &answer.reply)
}

fn write_reply(mut stream: &UnixStream, reply: &Reply) -> std::io::Result<()> {
    let mut bytes = serde_json::to_vec(reply).map_err(std::io::Error::other)?;
    bytes.push(b'\n');
    stream.write_all(&bytes)?;
    stream.flush()
}

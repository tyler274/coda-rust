//! A small Chrome DevTools client for proof tests.
//!
//! The page text is the assertion. An empty document, a failed handshake, or a
//! missing value fails the test.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

pub struct Target {
    pub socket_path: String,
    #[allow(dead_code)]
    pub screen_x: f64,
    #[allow(dead_code)]
    pub screen_y: f64,
}

pub struct Page {
    stream: TcpStream,
    pending: Vec<u8>,
    next_id: u64,
    received: usize,
    method: String,
    last_frame: String,
}

pub fn targets(port: u16) -> Result<Vec<Target>, String> {
    let body = http_get(port, "/json")?;
    let value: Value = serde_json::from_str(&body)
        .map_err(|error| format!("DevTools target list was not JSON: {error}\n{body}"))?;
    let pages = value
        .as_array()
        .ok_or_else(|| format!("DevTools target list was not a page array:\n{body}"))?;
    let found = pages
        .iter()
        .filter(|page| page.get("type").and_then(Value::as_str) == Some("page"))
        .filter_map(|page| {
            let socket = page.get("webSocketDebuggerUrl")?.as_str()?;
            let path = socket.split("/devtools/").nth(1)?;
            let description = page
                .get("description")
                .and_then(Value::as_str)
                .and_then(|text| serde_json::from_str::<Value>(text).ok());
            Some(Target {
                socket_path: format!("/devtools/{path}"),
                screen_x: number(description.as_ref(), "screenX"),
                screen_y: number(description.as_ref(), "screenY"),
            })
        })
        .collect::<Vec<_>>();
    if found.is_empty() {
        return Err(format!("DevTools listed no page to validate:\n{body}"));
    }
    Ok(found)
}

pub fn connect(port: u16, socket_path: &str) -> Result<Page, String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))
        .map_err(|error| format!("DevTools connection failed: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(15)))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(15)))
        .map_err(|error| error.to_string())?;
    let request = format!(
        "GET {socket_path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("DevTools handshake: {error}"))?;
    let mut pending = Vec::new();
    let mut buf = [0; 4096];
    loop {
        let read = stream
            .read(&mut buf)
            .map_err(|error| format!("DevTools handshake failed: {error}"))?;
        if read == 0 {
            return Err("DevTools closed during the handshake".to_owned());
        }
        pending.extend_from_slice(&buf[..read]);
        if let Some(end) = pending.windows(4).position(|window| window == b"\r\n\r\n") {
            let header = String::from_utf8_lossy(&pending[..end]).into_owned();
            if !header.contains("101") {
                return Err(format!(
                    "DevTools did not accept the proof connection:\n{header}"
                ));
            }
            pending.drain(..end + 4);
            break;
        }
    }
    Ok(Page {
        stream,
        pending,
        next_id: 1,
        received: 0,
        method: String::new(),
        last_frame: String::new(),
    })
}

impl Page {
    pub fn evaluate(&mut self, expression: &str) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        let value = self.call(
            id,
            "Runtime.evaluate",
            json!({ "expression": expression, "returnByValue": true }),
        );
        if let Some(exception) = value.pointer("/result/exceptionDetails") {
            panic!("page evaluation failed: {exception}");
        }
        value
            .pointer("/result/result/value")
            .cloned()
            .unwrap_or_else(|| panic!("page evaluation returned no value:\n{value}"))
    }

    // The React comparison navigates. The Android proof reads the page the
    // WebView already opened, so this method is unused in that binary.
    #[allow(dead_code)]
    pub fn navigate(&mut self, url: &str) {
        let id = self.next_id;
        self.next_id += 1;
        let value = self.call(id, "Page.navigate", json!({ "url": url }));
        if value.get("error").is_some() {
            panic!("navigation to {url} failed: {value}");
        }
    }

    fn call(&mut self, id: u64, method: &str, params: Value) -> Value {
        let payload = serde_json::to_vec(&json!({
            "id": id,
            "method": method,
            "params": params,
        }))
        .expect("DevTools request");
        self.method = method.to_owned();
        self.send(&payload);
        let deadline = Instant::now() + Duration::from_secs(25);
        loop {
            let frame = self.read_text_frame(deadline);
            let value: Value = serde_json::from_str(&frame)
                .unwrap_or_else(|error| panic!("DevTools frame was not JSON: {error}\n{frame}"));
            if value.get("id").and_then(Value::as_u64) == Some(id) {
                return value;
            }
            self.last_frame = frame.chars().take(240).collect();
        }
    }

    fn send(&mut self, payload: &[u8]) {
        let mask = [1, 2, 3, 4];
        let mut masked = payload.to_vec();
        for (index, byte) in masked.iter_mut().enumerate() {
            *byte ^= mask[index % 4];
        }
        let mut frame = vec![0x81];
        let len = payload.len();
        if len < 126 {
            frame.push(0x80 | u8::try_from(len).unwrap());
        } else if len <= u16::MAX as usize {
            frame.push(0x80 | 126);
            frame.extend(u16::try_from(len).unwrap().to_be_bytes());
        } else {
            frame.push(0x80 | 127);
            frame.extend(u64::try_from(len).unwrap().to_be_bytes());
        }
        frame.extend(mask);
        frame.extend(masked);
        self.stream
            .write_all(&frame)
            .expect("DevTools request write");
    }

    fn read_text_frame(&mut self, deadline: Instant) -> String {
        loop {
            let header = self.read_exact(2, deadline);
            let opcode = header[0] & 0x0f;
            let masked = header[1] & 0x80 != 0;
            let mut len = (header[1] & 0x7f) as u64;
            if len == 126 {
                len = u16::from_be_bytes(self.read_exact(2, deadline).try_into().unwrap()) as u64;
            } else if len == 127 {
                len = u64::from_be_bytes(self.read_exact(8, deadline).try_into().unwrap());
            }
            let mask = if masked {
                Some(self.read_exact(4, deadline))
            } else {
                None
            };
            let mut payload =
                self.read_exact(usize::try_from(len).expect("DevTools frame"), deadline);
            if let Some(mask) = mask {
                for (index, byte) in payload.iter_mut().enumerate() {
                    *byte ^= mask[index % 4];
                }
            }
            match opcode {
                1 => return String::from_utf8(payload).expect("DevTools text frame"),
                8 => panic!(
                    "DevTools closed the page: {}",
                    String::from_utf8_lossy(&payload)
                ),
                9 => self.pong(&payload),
                _ => {}
            }
        }
    }

    fn pong(&mut self, payload: &[u8]) {
        let mut frame = vec![0x8a, u8::try_from(payload.len()).unwrap_or(0)];
        frame.extend(payload);
        let _ = self.stream.write_all(&frame);
    }

    fn read_exact(&mut self, len: usize, deadline: Instant) -> Vec<u8> {
        while self.pending.len() < len {
            if Instant::now() >= deadline {
                panic!(
                    "DevTools did not answer {} before the deadline; received {} bytes, {} buffered, last frame:\n{}",
                    self.method,
                    self.received,
                    self.pending.len(),
                    self.last_frame
                );
            }
            let mut buf = [0; 8192];
            match self.stream.read(&mut buf) {
                Ok(0) => panic!("DevTools closed before the page answered"),
                Ok(read) => {
                    self.received += read;
                    self.pending.extend_from_slice(&buf[..read]);
                }
                Err(error)
                    if error.kind() == std::io::ErrorKind::TimedOut
                        || error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("DevTools read failed: {error}"),
            }
        }
        self.pending.drain(..len).collect()
    }
}

fn http_get(port: u16, path: &str) -> Result<String, String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))
        .map_err(|error| format!("DevTools list failed: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|error| error.to_string())?;
    let request =
        format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n");
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("DevTools list: {error}"))?;
    let mut bytes = Vec::new();
    let mut buf = [0; 4096];
    loop {
        let read = stream
            .read(&mut buf)
            .map_err(|error| format!("DevTools list body: {error}"))?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&buf[..read]);
        let text = String::from_utf8_lossy(&bytes);
        let Some((header, body)) = text.split_once("\r\n\r\n") else {
            continue;
        };
        if let Some(length) = content_length(header)
            && body.len() >= length
        {
            bytes.truncate(header.len() + 4 + length);
            break;
        }
    }
    let text = String::from_utf8_lossy(&bytes);
    let (header, body) = text
        .split_once("\r\n\r\n")
        .ok_or_else(|| format!("DevTools list had no body:\n{text}"))?;
    if !header.contains("200") {
        return Err(format!("DevTools list failed:\n{header}\n{body}"));
    }
    if header.to_ascii_lowercase().contains("chunked") {
        Ok(decode_chunks(body))
    } else {
        Ok(body.to_owned())
    }
}

fn content_length(header: &str) -> Option<usize> {
    header.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("content-length")
            .then(|| value.trim().parse().ok())
            .flatten()
    })
}

fn decode_chunks(body: &str) -> String {
    let mut rest = body;
    let mut out = String::new();
    while !rest.is_empty() {
        let Some((size, after)) = rest.split_once("\r\n") else {
            break;
        };
        let size = usize::from_str_radix(size.trim(), 16).unwrap_or(0);
        if size == 0 {
            break;
        }
        let take = size.min(after.len());
        out.push_str(&after[..take]);
        rest = after.get(take + 2..).unwrap_or("");
    }
    out
}

fn number(value: Option<&Value>, key: &str) -> f64 {
    value
        .and_then(|value| value.get(key))
        .and_then(Value::as_f64)
        .unwrap_or(0.0)
}

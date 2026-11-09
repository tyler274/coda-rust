//! A small Chrome DevTools client for proof tests.
//!
//! The page text is the assertion. An empty document, a failed handshake, or a
//! missing value fails the test. Every browser instance picks an ephemeral
//! debugging port so several proofs can run in parallel in one CI job, and the
//! input helpers click, type, and resize so a proof can walk a whole journey,
//! not only read the text.

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

/// The desktop viewport a physician proof uses.
pub const DESKTOP: (u32, u32) = (1280, 800);
/// The phone viewport a Duet proof uses.
pub const PHONE: (u32, u32) = (390, 844);

/// A key as DevTools presses it.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Key {
    /// The key's name, as the page reads it.
    pub key: &'static str,
    code: &'static str,
    virtual_code: u32,
    text: &'static str,
}

#[allow(dead_code)]
impl Key {
    pub const TAB: Self = Self::named("Tab", 9, "");
    pub const ENTER: Self = Self::named("Enter", 13, "\r");
    pub const ESCAPE: Self = Self::named("Escape", 27, "");
    pub const END: Self = Self::named("End", 35, "");
    pub const HOME: Self = Self::named("Home", 36, "");
    pub const ARROW_LEFT: Self = Self::named("ArrowLeft", 37, "");
    pub const ARROW_UP: Self = Self::named("ArrowUp", 38, "");
    pub const ARROW_RIGHT: Self = Self::named("ArrowRight", 39, "");
    pub const ARROW_DOWN: Self = Self::named("ArrowDown", 40, "");
    pub const T: Self = Self {
        key: "t",
        code: "KeyT",
        virtual_code: 84,
        text: "t",
    };

    /// A key that types `text` as it is pressed, such as `?`.
    pub const fn typing(text: &'static str, code: &'static str, virtual_code: u32) -> Self {
        Self {
            key: text,
            code,
            virtual_code,
            text,
        }
    }

    const fn named(key: &'static str, virtual_code: u32, text: &'static str) -> Self {
        Self {
            key,
            code: key,
            virtual_code,
            text,
        }
    }
}

/// One headless Chromium instance: an ephemeral debugging port, one page.
///
/// Each launch binds port 0, asks the kernel for a free local port, closes it,
/// and hands it to Chromium. Two proofs therefore never collide, even when
/// cargo runs test binaries in parallel inside one CI job.
pub struct Browser {
    child: std::process::Child,
    port: u16,
    page: Page,
    base: String,
}

impl Browser {
    /// Launches headless Chromium on an ephemeral port with one blank page.
    // Success transfers the child into Browser; its Drop kills and waits.
    // Startup failure paths below also kill and wait before panicking.
    pub fn launch() -> Self {
        Self::launch_with(&[])
    }

    /// `launch`, with the unpacked Chrome extension in `folder` loaded.
    #[allow(dead_code)]
    pub fn launch_with_extension(folder: &std::path::Path) -> Self {
        let folder = folder.display();
        let profile = std::env::temp_dir().join(format!("coda-extension-{}", std::process::id()));
        Self::launch_with(&[
            format!("--user-data-dir={}", profile.display()),
            format!("--disable-extensions-except={folder}"),
            format!("--load-extension={folder}"),
        ])
    }

    #[allow(clippy::zombie_processes)]
    fn launch_with(extra: &[String]) -> Self {
        let port = free_port().expect("a free local port for the DevTools endpoint");
        let mut child = std::process::Command::new(chromium_path())
            .env_remove("LD_LIBRARY_PATH")
            .args([
                "--headless=new",
                "--disable-gpu",
                "--no-sandbox",
                "--disable-dev-shm-usage",
                &format!("--remote-debugging-port={port}"),
                &format!("--window-size={},{}", DESKTOP.0, DESKTOP.1),
                "about:blank",
            ])
            .args(extra)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("chromium");
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if Instant::now() > deadline {
                let _ = child.kill();
                let _ = child.wait();
                panic!("chromium did not open a DevTools page on port {port}");
            }
            if let Ok(pages) = targets(port)
                && let Some(target) = pages.into_iter().next()
            {
                let page = match connect(port, &target.socket_path) {
                    Ok(page) => page,
                    Err(error) => {
                        let _ = child.kill();
                        let _ = child.wait();
                        panic!("chromium DevTools: {error}");
                    }
                };
                return Self {
                    child,
                    port,
                    page,
                    base: "about:blank".to_owned(),
                };
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// The debugging port this browser listens on.
    #[allow(dead_code)]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// The raw page handle for callers that drive CDP directly.
    #[allow(dead_code)]
    pub fn page(&mut self) -> &mut Page {
        &mut self.page
    }

    /// Navigates the page and waits until `needle` shows in the body text.
    ///
    /// Returns the body text at the moment it first contained `needle`, so the
    /// caller can assert on more than the needle itself.
    pub fn text_until(&mut self, url: &str, needle: &str) -> String {
        self.page.navigate(url);
        self.base = url.to_owned();
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut last = String::new();
        while Instant::now() < deadline {
            if let Some(text) = self.body_text() {
                last = text;
                if last.contains(needle) {
                    return last;
                }
            }
            std::thread::sleep(Duration::from_millis(250));
        }
        panic!("{url} never showed {needle}:\n{last}");
    }

    /// Waits until the body text contains `needle`, without navigating.
    pub fn wait_for(&mut self, needle: &str) -> String {
        self.wait_at(needle, self.base.clone())
    }

    /// The `wait_for` variant that reports `origin` in a timeout panic.
    pub fn wait_at(&mut self, needle: &str, origin: String) -> String {
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut last = String::new();
        while Instant::now() < deadline {
            if let Some(text) = self.body_text() {
                last = text;
                if last.contains(needle) {
                    return last;
                }
            }
            std::thread::sleep(Duration::from_millis(250));
        }
        panic!("{origin} never showed {needle}:\n{last}");
    }

    /// The body innerText right now, when the page has one.
    pub fn body_text(&mut self) -> Option<String> {
        self.page
            .evaluate("document.body ? document.body.innerText : \"\"")
            .as_str()
            .map(str::to_owned)
    }

    /// The current document HTML, for asserting on classes and attributes.
    #[allow(dead_code)]
    pub fn html(&mut self) -> String {
        self.page
            .evaluate("document.documentElement ? document.documentElement.outerHTML : \"\"")
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }

    /// Clicks the first element matching `selector`.
    pub fn click(&mut self, selector: &str) {
        let found = self.page.run("click", json!([selector]));
        assert_eq!(found, "clicked", "no element to click for {selector}");
    }

    /// Types `text` into the first input or textarea matching `selector` and
    /// fires the Dioxus `input` event so signal bindings update.
    pub fn type_into(&mut self, selector: &str, text: &str) {
        let found = self.page.run("typeInto", json!([selector, text]));
        assert_eq!(found, "typed", "no input to type into for {selector}");
    }

    /// Presses `key` as the browser's own input, so what it does by default, such as moving focus
    /// or scrolling, happens too.
    #[allow(dead_code)]
    pub fn press_key(&mut self, key: Key) {
        for (kind, text) in [("keyDown", key.text), ("keyUp", "")] {
            self.page.call_session(
                "Input.dispatchKeyEvent",
                json!({
                    "type": kind,
                    "key": key.key,
                    "code": key.code,
                    "windowsVirtualKeyCode": key.virtual_code,
                    "text": text,
                }),
            );
        }
    }

    /// Resizes the viewport. `size` is one of [`DESKTOP`] or [`PHONE`].
    pub fn resize(&mut self, size: (u32, u32)) {
        self.page.call_session(
            "Emulation.setDeviceMetricsOverride",
            json!({
                "width": size.0,
                "height": size.1,
                "deviceScaleFactor": 1.0,
                "mobile": false,
            }),
        );
    }

    /// Emulates `prefers-reduced-motion: reduce` for this page.
    #[allow(dead_code)]
    pub fn emulate_reduced_motion(&mut self) {
        self.page.call_session(
            "Emulation.setEmulatedMedia",
            json!({
                "features": [{ "name": "prefers-reduced-motion", "value": "reduce" }],
            }),
        );
    }

    /// Starts recording what the page reports as wrong: uncaught exceptions,
    /// including a WebAssembly panic, and console or browser errors.
    #[allow(dead_code)]
    pub fn watch_page_errors(&mut self) {
        self.page.call_session("Runtime.enable", json!({}));
        self.page.call_session("Log.enable", json!({}));
    }

    /// The errors recorded since the last call, once the page has answered
    /// everything it sent before now.
    #[allow(dead_code)]
    pub fn take_page_errors(&mut self) -> Vec<String> {
        self.page.evaluate("0");
        std::mem::take(&mut self.page.errors)
    }

    /// The requests `Fetch.enable` has paused since the last call, oldest first,
    /// once the page has answered everything it sent before now. Each waits until
    /// the caller continues, fails or answers it.
    #[allow(dead_code)]
    pub fn take_paused_requests(&mut self) -> Vec<Value> {
        self.page.evaluate("0");
        std::mem::take(&mut self.page.paused)
    }

    /// Whether the document reports reduced motion.
    #[allow(dead_code)]
    pub fn prefers_reduced_motion(&mut self) -> bool {
        self.page
            .evaluate("window.matchMedia('(prefers-reduced-motion: reduce)').matches")
            .as_bool()
            .unwrap_or(false)
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A local port the kernel considers free right now.
pub fn free_port() -> Option<u16> {
    std::net::TcpListener::bind(("127.0.0.1", 0))
        .ok()?
        .local_addr()
        .ok()?
        .port()
        .into()
}

fn chromium_path() -> std::path::PathBuf {
    if let Some(path) = std::env::var_os("CHROMIUM") {
        return std::path::PathBuf::from(path);
    }
    for name in [
        "chromium",
        "chromium-browser",
        "google-chrome",
        "google-chrome-stable",
    ] {
        if let Some(path) = find_on_path(name) {
            return path;
        }
    }
    panic!("chromium is required: set CHROMIUM or put chromium on PATH");
}

fn find_on_path(name: &str) -> Option<std::path::PathBuf> {
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

pub struct Page {
    stream: TcpStream,
    pending: Vec<u8>,
    next_id: u64,
    received: usize,
    method: String,
    last_frame: String,
    errors: Vec<String>,
    paused: Vec<Value>,
}

/// What a DevTools event says went wrong, when it reports an error.
fn page_error(event: &Value) -> Option<String> {
    let params = event.get("params")?;
    match event.get("method")?.as_str()? {
        "Runtime.exceptionThrown" => {
            let details = params.get("exceptionDetails")?;
            let description = details.pointer("/exception/description");
            let text = description.or_else(|| details.get("text"))?.as_str()?;
            Some(format!("uncaught: {text}"))
        }
        "Runtime.consoleAPICalled" => {
            let kind = params.get("type")?.as_str()?;
            if kind != "error" && kind != "assert" {
                return None;
            }
            let words = params
                .get("args")?
                .as_array()?
                .iter()
                .map(|arg| {
                    let value = arg.get("value").or_else(|| arg.get("description"));
                    match value {
                        Some(Value::String(text)) => text.clone(),
                        Some(other) => other.to_string(),
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            Some(format!("console.{kind}: {words}"))
        }
        "Log.entryAdded" => {
            let entry = params.get("entry")?;
            if entry.get("level")?.as_str()? != "error" {
                return None;
            }
            let text = entry.get("text")?.as_str()?;
            let url = entry.get("url").and_then(Value::as_str).unwrap_or_default();
            Some(format!("browser: {text} {url}"))
        }
        _ => None,
    }
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
        errors: Vec::new(),
        paused: Vec::new(),
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

    /// Runs `name` from `page.js` with `args`, a JSON array, and returns what it
    /// gives back.
    pub fn run(&mut self, name: &str, args: Value) -> Value {
        let mut arguments = vec![json!(name)];
        arguments.extend(
            args.as_array()
                .expect("a JSON array of arguments")
                .iter()
                .cloned(),
        );
        self.call_function(include_str!("page.js"), &arguments)
    }

    /// Calls `declaration`, a JavaScript function, on the page with `args`. The
    /// values arrive as the function's arguments, never as script text. A
    /// promise is awaited, a throw fails the test, and a function that returns
    /// nothing answers null.
    pub fn call_function(&mut self, declaration: &str, args: &[Value]) -> Value {
        let global = self.call_session("Runtime.evaluate", json!({ "expression": "globalThis" }));
        let object = global
            .pointer("/result/result/objectId")
            .cloned()
            .unwrap_or_else(|| panic!("the page has no global object:\n{global}"));
        let arguments: Vec<Value> = args.iter().map(|value| json!({ "value": value })).collect();
        let value = self.call_session(
            "Runtime.callFunctionOn",
            json!({
                "functionDeclaration": declaration,
                "objectId": object,
                "arguments": arguments,
                "returnByValue": true,
                "awaitPromise": true,
            }),
        );
        if let Some(exception) = value.pointer("/result/exceptionDetails") {
            panic!("page function failed: {exception}");
        }
        value
            .pointer("/result/result/value")
            .cloned()
            .unwrap_or(Value::Null)
    }

    /// Sends one raw DevTools method on this page's session and requires an
    /// answer without an `error` field.
    pub fn call_session(&mut self, method: &str, params: Value) -> Value {
        self.try_call_session(method, params)
            .unwrap_or_else(|error| panic!("{method} failed: {error}"))
    }

    /// Sends one raw DevTools method on this page's session, and returns the
    /// `error` Chrome answered for a caller that expects one it can name.
    pub fn try_call_session(&mut self, method: &str, params: Value) -> Result<Value, Value> {
        let id = self.next_id;
        self.next_id += 1;
        let mut value = self.call(id, method, params);
        match value.get_mut("error") {
            Some(error) => Err(error.take()),
            None => Ok(value),
        }
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
            if value.get("method").and_then(Value::as_str) == Some("Fetch.requestPaused") {
                self.paused.extend(value.get("params").cloned());
            }
            self.errors.extend(page_error(&value));
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
        let mask = [1, 2, 3, 4];
        let mut masked = payload.to_vec();
        for (index, byte) in masked.iter_mut().enumerate() {
            *byte ^= mask[index % 4];
        }
        let mut frame = vec![0x8a, 0x80 | u8::try_from(masked.len()).unwrap_or(0)];
        frame.extend(mask);
        frame.extend(masked);
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

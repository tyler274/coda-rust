//! A local HTTP server that answers GitHub API paths from a fixed table and
//! records every request it receives.

use std::collections::HashMap;
use std::io::{BufRead as _, BufReader, Read as _, Write as _};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub method: String,
    pub path: String,
    pub body: Option<Value>,
}

impl Call {
    pub fn new(method: &str, path: &str) -> Self {
        Self {
            method: method.to_owned(),
            path: path.to_owned(),
            body: None,
        }
    }
}

type Routes = HashMap<(String, String), (u16, String)>;

pub struct FakeGitHub {
    pub base: String,
    calls: Arc<Mutex<Vec<Call>>>,
}

pub struct Builder {
    routes: Routes,
}

impl Builder {
    /// Answer `method path` (path without the leading slash, with its query)
    /// with `status` and `body`; `Value::Null` sends an empty body.
    pub fn route(mut self, method: &str, path: &str, status: u16, body: Value) -> Self {
        let text = if body.is_null() {
            String::new()
        } else {
            body.to_string()
        };
        self.routes
            .insert((method.to_owned(), path.to_owned()), (status, text));
        self
    }

    pub fn start(self) -> FakeGitHub {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind a local port");
        let base = format!("http://{}", listener.local_addr().expect("local address"));
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let routes = self.routes;
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { continue };
                serve(stream, &routes, &recorded);
            }
        });
        FakeGitHub { base, calls }
    }
}

impl FakeGitHub {
    pub fn builder() -> Builder {
        Builder {
            routes: HashMap::new(),
        }
    }

    pub fn calls(&self) -> Vec<Call> {
        self.calls.lock().expect("calls lock").clone()
    }

    /// The calls without their bodies, for comparing method and path only.
    pub fn paths(&self) -> Vec<Call> {
        self.calls()
            .into_iter()
            .map(|call| Call { body: None, ..call })
            .collect()
    }
}

fn serve(stream: TcpStream, routes: &Routes, calls: &Mutex<Vec<Call>>) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_owned();
    let path = parts
        .next()
        .unwrap_or_default()
        .trim_start_matches('/')
        .to_owned();
    let mut length = 0usize;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).unwrap_or(0) == 0 || header.trim().is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            length = value.trim().parse().unwrap_or(0);
        }
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body).expect("read request body");
    let body =
        (!body.is_empty()).then(|| serde_json::from_slice(&body).expect("JSON request body"));
    let (status, text) = routes
        .get(&(method.clone(), path.clone()))
        .cloned()
        .unwrap_or((404, r#"{"message":"Not Found"}"#.to_owned()));
    calls
        .lock()
        .expect("calls lock")
        .push(Call { method, path, body });
    let mut stream = stream;
    let response = format!(
        "HTTP/1.1 {status} Fake\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{text}",
        text.len()
    );
    let _ = stream.write_all(response.as_bytes());
}

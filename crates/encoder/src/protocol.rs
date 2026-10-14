use std::collections::VecDeque;
use std::io::{BufRead, Write};
use std::sync::mpsc;

use serde_json::Value;

use crate::recipe::{Protocol, QUERY_PREFIX, QUERY_TOO_LONG};
use crate::window::{BATCH, STRIDE, WINDOW, content_windows};

pub trait Encode {
    fn content_ids(&self, text: &str) -> Result<Vec<u32>, String>;
    fn forward(&mut self, windows: &[Vec<u32>]) -> Result<Vec<Vec<f32>>, String>;
}

/// Serves Duet embedding requests on stdin.
pub fn serve_duet<E: Encode>(encoder: &mut E) -> Result<(), String> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|error| error.to_string())?;
        writeln!(output, "{}", duet_response(encoder, &line)).map_err(|error| error.to_string())?;
        output.flush().map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub fn duet_response<E: Encode>(encoder: &mut E, line: &str) -> String {
    match duet_result(encoder, line) {
        Ok(result) => serde_json::json!({ "result": result }).to_string(),
        Err(_) => "{\"error\":\"encoding_unavailable\"}".into(),
    }
}

fn duet_result<E: Encode>(encoder: &mut E, line: &str) -> Result<Value, String> {
    let request: Value = serde_json::from_str(line).map_err(|error| error.to_string())?;
    if request.get("operation").and_then(|value| value.as_str()) == Some("query") {
        let text = request
            .get("text")
            .and_then(|value| value.as_str())
            .ok_or("missing text")?;
        let ids = query_ids(encoder, text)?;
        let vectors = encoder.forward(std::slice::from_ref(&ids))?;
        let vector = vectors
            .into_iter()
            .next()
            .ok_or("encoder returned no query vector")?;
        return Ok(serde_json::json!(vector));
    }
    let texts = string_list(request.get("texts").ok_or("missing texts")?)?;
    if !(1..=BATCH).contains(&texts.len()) {
        return Err("batch_window_exceeded".into());
    }
    let mut documents = Vec::with_capacity(texts.len());
    for text in texts {
        documents.push(encoder.forward(&document_windows(encoder, &text)?)?);
    }
    Ok(serde_json::json!(documents))
}

/// Serves patient embedding requests on stdin.
pub fn serve_patient<E: Encode>(encoder: &mut E) -> Result<(), String> {
    let (sender, receiver) = mpsc::channel::<Option<String>>();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    if sender.send(Some(line)).is_err() {
                        return;
                    }
                }
                Err(_) => {
                    let _ = sender.send(None);
                    return;
                }
            }
        }
        let _ = sender.send(None);
    });
    let mut session = PatientSession::default();
    let mut reading = true;
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    while reading || session.busy() {
        // Block only while nothing is queued. After one line arrives, drain without waiting so a
        // query can run before the next document forward pass.
        let mut block = !session.busy();
        loop {
            let incoming = if block {
                receiver.recv().unwrap_or_default()
            } else {
                match receiver.try_recv() {
                    Ok(line) => line,
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => None,
                }
            };
            match incoming {
                Some(line) => {
                    session.push(&line)?;
                    block = false;
                }
                None => {
                    reading = false;
                    break;
                }
            }
        }
        if let Some(line) = session.turn(encoder)? {
            writeln!(output, "{line}").map_err(|error| error.to_string())?;
            output.flush().map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

#[derive(Default)]
pub struct PatientSession {
    queries: VecDeque<Value>,
    pending: VecDeque<Value>,
    active: Option<Active>,
}

struct Active {
    id: Value,
    batches: VecDeque<Batch>,
    vectors: Vec<Vec<Vec<f32>>>,
}

struct Batch {
    owner: usize,
    windows: Vec<Vec<u32>>,
}

impl PatientSession {
    pub fn busy(&self) -> bool {
        !self.queries.is_empty() || !self.pending.is_empty() || self.active.is_some()
    }

    pub fn push(&mut self, line: &str) -> Result<(), String> {
        let request: Value = serde_json::from_str(line).map_err(|error| error.to_string())?;
        if request.get("operation").and_then(|value| value.as_str()) == Some("query") {
            self.queries.push_back(request);
        } else {
            self.pending.push_back(request);
        }
        Ok(())
    }

    /// One scheduler step: a query, or a single document forward pass, or the document's completion.
    pub fn turn<E: Encode>(&mut self, encoder: &mut E) -> Result<Option<String>, String> {
        if let Some(request) = self.queries.pop_front() {
            return Ok(Some(answer_query(encoder, &request)));
        }
        if self.active.is_none()
            && let Some(request) = self.pending.pop_front()
        {
            match start_document(encoder, &request)? {
                Started::Failed(line) => return Ok(Some(line)),
                Started::Job(job) => self.active = Some(job),
            }
        }
        let Some(batch) = self
            .active
            .as_mut()
            .and_then(|active| active.batches.pop_front())
        else {
            return Ok(self
                .active
                .take()
                .map(|done| result_line(&done.id, &serde_json::json!(done.vectors))));
        };
        let owner = batch.owner;
        match encoder.forward(&batch.windows) {
            Ok(encoded) if encoded.len() == batch.windows.len() => {
                self.active.as_mut().expect("active document").vectors[owner].extend(encoded);
                Ok(None)
            }
            Ok(_) => {
                let id = self.active.take().expect("active document").id;
                Ok(Some(error_line(
                    &id,
                    "encoder returned the wrong number of vectors",
                )))
            }
            Err(error) => {
                let id = self.active.take().expect("active document").id;
                Ok(Some(error_line(&id, &error)))
            }
        }
    }
}

enum Started {
    Job(Active),
    Failed(String),
}

fn start_document<E: Encode>(encoder: &E, request: &Value) -> Result<Started, String> {
    let id = request
        .get("id")
        .cloned()
        .ok_or("patient request is missing id")?;
    let Some(texts) = request.get("texts") else {
        return Ok(Started::Failed(error_line(&id, "missing texts")));
    };
    let texts = match string_list(texts) {
        Ok(texts) => texts,
        Err(error) => return Ok(Started::Failed(error_line(&id, &error))),
    };
    let mut vectors = Vec::with_capacity(texts.len());
    let mut batches = VecDeque::new();
    for (owner, text) in texts.iter().enumerate() {
        let windows = match document_windows(encoder, text) {
            Ok(windows) => windows,
            Err(error) => return Ok(Started::Failed(error_line(&id, &error))),
        };
        vectors.push(Vec::new());
        for chunk in windows.chunks(BATCH) {
            batches.push_back(Batch {
                owner,
                windows: chunk.to_vec(),
            });
        }
    }
    Ok(Started::Job(Active {
        id,
        batches,
        vectors,
    }))
}

fn answer_query<E: Encode>(encoder: &mut E, request: &Value) -> String {
    let Some(id) = request.get("id") else {
        return error_line(&Value::Null, "patient request is missing id");
    };
    let Some(text) = request.get("text").and_then(|value| value.as_str()) else {
        return error_line(id, "missing text");
    };
    match query_ids(encoder, text).and_then(|ids| encoder.forward(std::slice::from_ref(&ids))) {
        Ok(mut vectors) => match vectors.pop() {
            Some(vector) if vectors.is_empty() => result_line(id, &serde_json::json!(vector)),
            _ => error_line(id, "encoder returned the wrong number of vectors"),
        },
        Err(error) => error_line(id, &error),
    }
}

fn query_ids<E: Encode>(encoder: &E, text: &str) -> Result<Vec<u32>, String> {
    let ids = encoder.content_ids(&format!("{QUERY_PREFIX}{text}"))?;
    if ids.len() > 510 {
        return Err(QUERY_TOO_LONG.into());
    }
    Ok(ids)
}

fn document_windows<E: Encode>(encoder: &E, text: &str) -> Result<Vec<Vec<u32>>, String> {
    Ok(content_windows(&encoder.content_ids(text)?, WINDOW, STRIDE))
}

fn string_list(value: &Value) -> Result<Vec<String>, String> {
    let items = value.as_array().ok_or("texts is not a list")?;
    items
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| "document text is not a string".into())
        })
        .collect()
}

fn result_line(id: &Value, result: &Value) -> String {
    serde_json::json!({ "id": id, "result": result }).to_string()
}

fn error_line(id: &Value, error: &str) -> String {
    serde_json::json!({ "id": id, "error": error }).to_string()
}

/// Returns the line the encoder prints when it is ready.
pub fn startup_line(protocol: Protocol, recipe: &str) -> String {
    match protocol {
        Protocol::Duet => serde_json::json!({ "recipe": recipe }).to_string(),
        Protocol::Patient => serde_json::json!({ "type": "ready", "recipe": recipe }).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct StandIn {
        seen: std::cell::RefCell<Vec<String>>,
        forwards: usize,
        last_batch: usize,
    }

    impl Encode for StandIn {
        fn content_ids(&self, text: &str) -> Result<Vec<u32>, String> {
            self.seen.borrow_mut().push(text.to_owned());
            if text.contains("explode") {
                return Err("secret patient text".into());
            }
            if let Some((_, count)) = text.rsplit_once("tokens:") {
                let count: usize = count.parse().map_err(|_| "bad token count".to_owned())?;
                return Ok(vec![1; count]);
            }
            Ok(text.split_whitespace().map(|_| 1).collect())
        }

        fn forward(&mut self, windows: &[Vec<u32>]) -> Result<Vec<Vec<f32>>, String> {
            self.forwards += 1;
            self.last_batch = windows.len();
            Ok(windows
                .iter()
                .map(|window| vec![window.len() as f32])
                .collect())
        }
    }

    #[test]
    fn duet_windows_query_prefix_and_opaque_error() {
        let mut encoder = StandIn::default();
        let short = duet_response(&mut encoder, r#"{"operation":"query","text":"short"}"#);
        let parsed: Value = serde_json::from_str(&short).unwrap();
        assert!(parsed.get("result").is_some());
        assert_eq!(encoder.seen.borrow()[0], format!("{QUERY_PREFIX}short"));

        let documents = duet_response(
            &mut encoder,
            r#"{"operation":"documents","texts":["tokens:401","tokens:5990"]}"#,
        );
        let parsed: Value = serde_json::from_str(&documents).unwrap();
        let result = parsed["result"].as_array().unwrap();
        assert_eq!(result[0].as_array().unwrap().len(), 2);
        assert_eq!(result[1].as_array().unwrap().len(), 17);

        let error = duet_response(&mut encoder, r#"{"operation":"query","text":"explode"}"#);
        assert_eq!(error, "{\"error\":\"encoding_unavailable\"}");
        assert!(!error.contains("secret"));
        assert!(!error.contains("explode"));
    }

    #[test]
    fn patient_query_runs_before_the_next_document_batch() {
        let mut encoder = StandIn::default();
        let mut session = PatientSession::default();
        session
            .push(r#"{"id":1,"operation":"documents","texts":["tokens:5990"]}"#)
            .unwrap();
        assert!(session.turn(&mut encoder).unwrap().is_none());
        assert_eq!(encoder.last_batch, 16);

        session
            .push(r#"{"id":2,"operation":"query","text":"short"}"#)
            .unwrap();
        let query = session.turn(&mut encoder).unwrap().unwrap();
        let parsed: Value = serde_json::from_str(&query).unwrap();
        assert_eq!(parsed["id"], 2);
        assert!(parsed.get("result").is_some());
        assert_eq!(encoder.last_batch, 1);
        assert!(
            encoder
                .seen
                .borrow()
                .iter()
                .any(|text| text == &format!("{QUERY_PREFIX}short"))
        );

        assert!(session.turn(&mut encoder).unwrap().is_none());
        assert_eq!(encoder.last_batch, 1);
        let done = session.turn(&mut encoder).unwrap().unwrap();
        let parsed: Value = serde_json::from_str(&done).unwrap();
        assert_eq!(parsed["id"], 1);
        assert_eq!(parsed["result"][0].as_array().unwrap().len(), 17);
        assert!(!session.busy());
    }

    #[test]
    fn patient_query_past_the_window_keeps_its_message() {
        let mut encoder = StandIn::default();
        let mut session = PatientSession::default();
        session
            .push(r#"{"id":3,"operation":"query","text":"tokens:511"}"#)
            .unwrap();
        let line = session.turn(&mut encoder).unwrap().unwrap();
        let parsed: Value = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed["error"], QUERY_TOO_LONG);
    }
}

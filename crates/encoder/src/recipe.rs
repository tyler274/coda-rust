use std::fs::File;
use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::window::{STRIDE, WINDOW};

/// candle-core version this binary was built against. Kept beside the `=0.11.1` pin in Cargo.toml.
pub const CANDLE_RUNTIME: &str = "0.11.1";

pub const QUERY_PREFIX: &str = "Represent this sentence for searching relevant passages: ";

pub const QUERY_TOO_LONG: &str =
    "Query exceeds the encoder's 510-token input window; use a shorter authored query";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The `Protocol` cases.
pub enum Protocol {
    /// The `Duet` case.
    Duet,
    /// The `Patient` case.
    Patient,
}

impl Protocol {
    /// Parses the protocol name.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "duet" => Some(Self::Duet),
            "patient" => Some(Self::Patient),
            _ => None,
        }
    }

    /// Returns the query prefix for this protocol.
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Duet => "duet-bge-cls-v1:",
            Self::Patient => "patient-cls-v1:",
        }
    }

    /// Algorithm JSON hashed into the recipe. Duet keeps `dimensions`. Both record Candle and omit torch.
    pub fn algorithm(self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("candle".into(), serde_json::json!(CANDLE_RUNTIME));
        if self == Self::Duet {
            map.insert("dimensions".into(), serde_json::json!(384));
        }
        map.insert("normalization".into(), serde_json::json!("l2"));
        map.insert("pooling".into(), serde_json::json!("cls"));
        map.insert("query_prefix".into(), serde_json::json!(QUERY_PREFIX));
        map.insert("stride".into(), serde_json::json!(STRIDE));
        map.insert("version".into(), serde_json::json!(1));
        map.insert("window".into(), serde_json::json!(WINDOW));
        serde_json::Value::Object(map)
    }
}

/// Python `json.dumps(..., sort_keys=True)` for the objects this recipe hashes.
pub fn canonical_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let body = keys
                .into_iter()
                .map(|key| {
                    format!(
                        "{}: {}",
                        serde_json::to_string(key).expect("json key"),
                        canonical_json(&map[key])
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{body}}}")
        }
        serde_json::Value::Array(items) => {
            let body = items
                .iter()
                .map(canonical_json)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{body}]")
        }
        other => serde_json::to_string(other).expect("json scalar"),
    }
}

/// Fingerprints the encoder recipe for this directory.
pub fn fingerprint(protocol: Protocol, directory: &Path) -> Result<String, String> {
    let root = directory
        .canonicalize()
        .map_err(|error| format!("encoder directory: {error}"))?;
    if !root.is_dir() {
        return Err("encoder directory is not a directory".into());
    }
    let mut digest = Sha256::new();
    digest.update(canonical_json(&protocol.algorithm()).as_bytes());
    let mut files: Vec<_> = std::fs::read_dir(&root)
        .map_err(|error| format!("encoder directory: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("encoder directory: {error}"))?;
    files.sort_by_key(|entry| entry.file_name());
    for entry in files {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let hashed = matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("json" | "txt" | "safetensors")
        );
        if !hashed {
            continue;
        }
        let name = entry.file_name();
        digest.update(name.as_encoded_bytes());
        let mut file = File::open(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let mut buffer = vec![0_u8; 1_048_576];
        loop {
            let read = file
                .read(&mut buffer)
                .map_err(|error| format!("{}: {error}", path.display()))?;
            if read == 0 {
                break;
            }
            digest.update(&buffer[..read]);
        }
    }
    let digest = digest.finalize();
    let mut fingerprint = String::with_capacity(protocol.prefix().len() + digest.len() * 2);
    fingerprint.push_str(protocol.prefix());
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut fingerprint, "{byte:02x}");
    }
    Ok(fingerprint)
}

fn read_json(path: &Path) -> Result<Option<serde_json::Value>, String> {
    if !path.is_file() {
        return Ok(None);
    }
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|error| format!("{}: {error}", path.display()))
}

fn truthy(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => false,
        serde_json::Value::Bool(value) => *value,
        serde_json::Value::Number(value) => value.as_f64().unwrap_or(0.0) != 0.0,
        serde_json::Value::String(value) => !value.is_empty(),
        serde_json::Value::Array(value) => !value.is_empty(),
        serde_json::Value::Object(value) => !value.is_empty(),
    }
}

fn python_string(value: Option<&str>) -> String {
    match value {
        None => "None".into(),
        Some(text) => format!("'{text}'"),
    }
}

/// Why a model directory's own files disagree with this recipe, or None when they agree.
/// Missing files are declarations, not a failed read. Unreadable JSON is an error.
pub fn unsupported_inference(directory: &Path) -> Result<Option<String>, String> {
    let architecture = read_json(&directory.join("config.json"))?.and_then(|value| {
        value
            .get("model_type")
            .and_then(|item| item.as_str())
            .map(str::to_owned)
    });
    if architecture.as_deref() != Some("bert") {
        return Ok(Some(format!(
            "architecture {} is not a BERT encoder",
            python_string(architecture.as_deref())
        )));
    }
    let modules = read_json(&directory.join("modules.json"))?;
    let pooling_paths: Vec<String> = modules
        .as_ref()
        .and_then(|value| value.as_array())
        .map(|modules| {
            modules
                .iter()
                .filter_map(|module| {
                    let module = module.as_object()?;
                    let kind = module
                        .get("type")
                        .and_then(|value| value.as_str())
                        .unwrap_or("");
                    if !kind.ends_with(".Pooling") {
                        return None;
                    }
                    Some(
                        module
                            .get("path")
                            .and_then(|value| value.as_str())
                            .unwrap_or("")
                            .to_owned(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    if pooling_paths.len() != 1 {
        return Ok(Some(
            "the model declares no sentence-transformers pooling".into(),
        ));
    }
    let pooling = read_json(&directory.join(&pooling_paths[0]).join("config.json"))?
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();
    let mut modes: Vec<&str> = pooling
        .iter()
        .filter(|(key, value)| key.starts_with("pooling_mode_") && truthy(value))
        .map(|(key, _)| key.as_str())
        .collect();
    modes.sort_unstable();
    if modes != ["pooling_mode_cls_token"] {
        let shown = if modes.is_empty() {
            "none".into()
        } else {
            format!(
                "[{}]",
                modes
                    .iter()
                    .map(|mode| format!("'{mode}'"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        return Ok(Some(format!("declared pooling {shown} is not CLS")));
    }
    let settings = read_json(&directory.join("config_sentence_transformers.json"))?
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();
    let prompts = settings
        .get("prompts")
        .filter(|value| !value.is_null())
        .and_then(|value| value.as_object())
        .cloned()
        .unwrap_or_default();
    let query = prompts
        .get("query")
        .and_then(|value| value.as_str())
        .unwrap_or(QUERY_PREFIX);
    let document = prompts
        .get("document")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    let extra = prompts
        .keys()
        .any(|key| key != "query" && key != "document");
    let named_default = settings
        .get("default_prompt_name")
        .is_some_and(|value| !value.is_null());
    if query != QUERY_PREFIX || !document.is_empty() || extra || named_default {
        return Ok(Some(
            "declared prompts differ from the BGE query instruction with no document prompt".into(),
        ));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write(dir: &Path, name: &str, body: &str) {
        fs::write(dir.join(name), body).unwrap();
    }

    #[test]
    fn fingerprint_tracks_file_bytes_and_names_candle() {
        let dir = std::env::temp_dir().join(format!("coda-encoder-recipe-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir(&dir).unwrap();
        write(&dir, "config.json", "{\"hidden_size\":384}\n");
        let first = fingerprint(Protocol::Duet, &dir).unwrap();
        assert!(first.starts_with("duet-bge-cls-v1:"));
        assert!(canonical_json(&Protocol::Duet.algorithm()).contains("\"candle\""));
        assert!(!canonical_json(&Protocol::Duet.algorithm()).contains("torch"));
        assert!(!canonical_json(&Protocol::Patient.algorithm()).contains("dimensions"));
        write(&dir, "config.json", "{\"hidden_size\":385}\n");
        let second = fingerprint(Protocol::Duet, &dir).unwrap();
        assert_ne!(first, second);
        let patient = fingerprint(Protocol::Patient, &dir).unwrap();
        assert!(patient.starts_with("patient-cls-v1:"));
        assert_ne!(patient, second);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn physician_declarations_match_the_python_reasons() {
        let dir = std::env::temp_dir().join(format!("coda-encoder-decl-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("1_Pooling")).unwrap();
        write(&dir, "config.json", "{\"model_type\":\"bert\"}");
        write(
            &dir,
            "modules.json",
            "[{\"type\":\"sentence_transformers.models.Pooling\",\"path\":\"1_Pooling\"}]",
        );
        write(
            &dir.join("1_Pooling"),
            "config.json",
            "{\"pooling_mode_cls_token\":true}",
        );
        assert_eq!(unsupported_inference(&dir).unwrap(), None);

        write(
            &dir.join("1_Pooling"),
            "config.json",
            "{\"pooling_mode_cls_token\":false,\"pooling_mode_mean_tokens\":true}",
        );
        assert_eq!(
            unsupported_inference(&dir).unwrap().as_deref(),
            Some("declared pooling ['pooling_mode_mean_tokens'] is not CLS")
        );

        write(
            &dir,
            "config_sentence_transformers.json",
            "{\"prompts\":{\"query\":\"other\",\"document\":\"\"}}",
        );
        write(
            &dir.join("1_Pooling"),
            "config.json",
            "{\"pooling_mode_cls_token\":true}",
        );
        assert_eq!(
            unsupported_inference(&dir).unwrap().as_deref(),
            Some("declared prompts differ from the BGE query instruction with no document prompt")
        );

        fs::remove_file(dir.join("modules.json")).unwrap();
        assert_eq!(
            unsupported_inference(&dir).unwrap().as_deref(),
            Some("the model declares no sentence-transformers pooling")
        );

        write(&dir, "config.json", "{\"model_type\":\"modernbert\"}");
        assert_eq!(
            unsupported_inference(&dir).unwrap().as_deref(),
            Some("architecture 'modernbert' is not a BERT encoder")
        );
        let _ = fs::remove_dir_all(&dir);
    }
}

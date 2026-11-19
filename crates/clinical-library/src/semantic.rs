//! Offline BGE-small (`384`-dim) semantic embedding support and hybrid
//! reciprocal-rank fusion (RRF) for clinical library search.

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;
use uuid::Uuid;

/// Output vector dimension produced by the `bge-small` BERT encoder (`crates/encoder`).
pub const BGE_EMBEDDING_DIM: usize = 384;

const RRF_K: f64 = 60.0;
const ENCODER_DOCUMENT_BATCH: usize = 16;

/// Validated, L2-normalized [`BGE_EMBEDDING_DIM`]-dimensional BGE embedding vector.
#[derive(Clone, Debug, PartialEq)]
pub struct BgeEmbedding(Vec<f32>);

impl BgeEmbedding {
    /// Validates finite length-[`BGE_EMBEDDING_DIM`] components with positive norm and L2-normalizes.
    pub fn new(raw: Vec<f32>) -> Result<Self, EmbedError> {
        if raw.len() != BGE_EMBEDDING_DIM {
            return Err(EmbedError::InvalidDimension {
                expected: BGE_EMBEDDING_DIM,
                actual: raw.len(),
            });
        }
        let mut sq_sum = 0.0_f64;
        for &value in &raw {
            if !value.is_finite() {
                return Err(EmbedError::NonFiniteComponent);
            }
            sq_sum += f64::from(value) * f64::from(value);
        }
        let norm = sq_sum.sqrt();
        if norm <= 1e-12 {
            return Err(EmbedError::ZeroNorm);
        }
        let inv = (1.0 / norm) as f32;
        let normalized = raw.into_iter().map(|v| v * inv).collect();
        Ok(Self(normalized))
    }

    /// Borrows the L2-normalized floating-point vector slice.
    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }

    /// Consumes the wrapper and returns the L2-normalized vector.
    pub fn into_vec(self) -> Vec<f32> {
        self.0
    }

    /// Computes dot-product cosine similarity between two normalized [`BgeEmbedding`]s.
    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&a, &b)| f64::from(a) * f64::from(b))
            .sum()
    }
}

/// Failure loading, invoking, or validating semantic embeddings.
#[derive(Debug, thiserror::Error)]
pub enum EmbedError {
    /// Vector length did not match [`BGE_EMBEDDING_DIM`].
    #[error("embedding expected {expected} dimensions, got {actual}")]
    InvalidDimension {
        /// Expected vector dimension.
        expected: usize,
        /// Observed vector dimension.
        actual: usize,
    },
    /// Vector contained `NaN` or infinite values.
    #[error("embedding contains non-finite values")]
    NonFiniteComponent,
    /// Vector norm was zero and could not be normalized.
    #[error("embedding has zero L2 norm")]
    ZeroNorm,
    /// Subprocess encoder binary or I/O stream failed.
    #[error("encoder subprocess I/O failed")]
    ProcessIo(#[source] std::io::Error),
    /// Encoder returned malformed or unexpected JSON protocol output.
    #[error("encoder protocol error: {0}")]
    Protocol(String),
}

/// Abstract semantic text embedder generating [`BgeEmbedding`]s for clinical queries and passages.
#[async_trait]
pub trait TextEmbedder: Send + Sync + fmt::Debug {
    /// Embeds a single clinical search query (applying query-side prefixing when applicable).
    async fn embed_query(&self, query: &str) -> Result<BgeEmbedding, EmbedError>;

    /// Embeds a slice of passage texts in order, returning one [`BgeEmbedding`] per passage.
    async fn embed_passages(&self, texts: &[String]) -> Result<Vec<BgeEmbedding>, EmbedError>;
}

struct EncoderProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
}

impl EncoderProcess {
    async fn spawn(bin: &Path, model_dir: &Path) -> Result<Self, EmbedError> {
        let mut child = Command::new(bin)
            .args([
                "--model",
                model_dir.to_string_lossy().as_ref(),
                "--protocol",
                "duet",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .map_err(EmbedError::ProcessIo)?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| EmbedError::Protocol("missing encoder stdin pipe".into()))?;
        let stdout_pipe = child
            .stdout
            .take()
            .ok_or_else(|| EmbedError::Protocol("missing encoder stdout pipe".into()))?;
        let mut stdout = BufReader::new(stdout_pipe).lines();

        let ready_line = stdout
            .next_line()
            .await
            .map_err(EmbedError::ProcessIo)?
            .ok_or_else(|| EmbedError::Protocol("encoder exited before ready line".into()))?;
        let parsed: Value = serde_json::from_str(&ready_line)
            .map_err(|err| EmbedError::Protocol(format!("invalid encoder startup JSON: {err}")))?;
        if parsed.get("protocol").and_then(Value::as_str) != Some("duet-bge-cls-v1") {
            return Err(EmbedError::Protocol(format!(
                "unexpected encoder startup protocol: {ready_line}"
            )));
        }

        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }

    async fn exchange(&mut self, request: &Value) -> Result<Value, EmbedError> {
        let mut line = serde_json::to_string(request)
            .map_err(|err| EmbedError::Protocol(format!("encode request JSON: {err}")))?;
        line.push('\n');
        self.stdin
            .write_all(line.as_bytes())
            .await
            .map_err(EmbedError::ProcessIo)?;
        self.stdin.flush().await.map_err(EmbedError::ProcessIo)?;

        let raw = self
            .stdout
            .next_line()
            .await
            .map_err(EmbedError::ProcessIo)?
            .ok_or_else(|| EmbedError::Protocol("encoder closed stdout unexpectedly".into()))?;
        let response: Value = serde_json::from_str(&raw)
            .map_err(|err| EmbedError::Protocol(format!("invalid encoder response JSON: {err}")))?;
        if let Some(error) = response.get("error").and_then(Value::as_str) {
            return Err(EmbedError::Protocol(error.to_owned()));
        }
        response
            .get("result")
            .cloned()
            .ok_or_else(|| EmbedError::Protocol("encoder response missing `result`".into()))
    }
}

/// Out-of-process BGE encoder client talking `Protocol::Duet` JSON lines to `crates/encoder`.
#[derive(Clone)]
pub struct SubprocessBgeEncoder {
    bin: PathBuf,
    model_dir: PathBuf,
    process: Arc<Mutex<Option<EncoderProcess>>>,
}

impl fmt::Debug for SubprocessBgeEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SubprocessBgeEncoder")
            .field("bin", &self.bin)
            .field("model_dir", &self.model_dir)
            .finish()
    }
}

impl SubprocessBgeEncoder {
    /// Constructs a subprocess encoder invoking `bin --model model_dir --protocol duet`.
    pub fn new(bin: impl Into<PathBuf>, model_dir: impl Into<PathBuf>) -> Self {
        Self {
            bin: bin.into(),
            model_dir: model_dir.into(),
            process: Arc::new(Mutex::new(None)),
        }
    }

    /// Reads `CLINICAL_LIBRARY_ENCODER_MODEL` (required) and optional `CLINICAL_LIBRARY_ENCODER_BIN`.
    pub fn from_env() -> Option<Self> {
        Self::from_env_lookup(|key| env::var(key).ok())
    }

    /// Looks up encoder configuration using a custom environment reader.
    pub fn from_env_lookup(mut read: impl FnMut(&str) -> Option<String>) -> Option<Self> {
        let model = read("CLINICAL_LIBRARY_ENCODER_MODEL")?.trim().to_owned();
        if model.is_empty() {
            return None;
        }
        let bin = read("CLINICAL_LIBRARY_ENCODER_BIN")
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("encoder"));
        Some(Self::new(bin, PathBuf::from(model)))
    }

    async fn call(&self, request: &Value) -> Result<Value, EmbedError> {
        let mut guard = self.process.lock().await;
        if guard.is_none() {
            *guard = Some(EncoderProcess::spawn(&self.bin, &self.model_dir).await?);
        }
        let proc = guard.as_mut().expect("encoder spawned");
        match proc.exchange(request).await {
            Ok(result) => Ok(result),
            Err(err) => {
                if let Some(mut dead) = guard.take() {
                    let _ = dead.child.start_kill();
                }
                Err(err)
            }
        }
    }
}

#[async_trait]
impl TextEmbedder for SubprocessBgeEncoder {
    async fn embed_query(&self, query: &str) -> Result<BgeEmbedding, EmbedError> {
        let result = self
            .call(&json!({ "operation": "query", "text": query }))
            .await?;
        let raw: Vec<f32> = serde_json::from_value(result)
            .map_err(|err| EmbedError::Protocol(format!("expected query float vector: {err}")))?;
        BgeEmbedding::new(raw)
    }

    async fn embed_passages(&self, texts: &[String]) -> Result<Vec<BgeEmbedding>, EmbedError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        let mut out = Vec::with_capacity(texts.len());
        for chunk in texts.chunks(ENCODER_DOCUMENT_BATCH) {
            let result = self.call(&json!({ "texts": chunk })).await?;
            let windows_per_doc: Vec<Vec<Vec<f32>>> =
                serde_json::from_value(result).map_err(|err| {
                    EmbedError::Protocol(format!("expected passage window vectors: {err}"))
                })?;
            if windows_per_doc.len() != chunk.len() {
                return Err(EmbedError::Protocol(format!(
                    "encoder returned {} passages for {} inputs",
                    windows_per_doc.len(),
                    chunk.len()
                )));
            }
            for windows in windows_per_doc {
                out.push(pool_passage_windows(&windows)?);
            }
        }
        Ok(out)
    }
}

fn pool_passage_windows(windows: &[Vec<f32>]) -> Result<BgeEmbedding, EmbedError> {
    if windows.is_empty() {
        return Err(EmbedError::Protocol(
            "passage produced zero embedding windows".into(),
        ));
    }
    let mut summed = vec![0.0_f64; BGE_EMBEDDING_DIM];
    for window in windows {
        let normalized = BgeEmbedding::new(window.clone())?;
        for (acc, &val) in summed.iter_mut().zip(normalized.as_slice().iter()) {
            *acc += f64::from(val);
        }
    }
    let averaged = summed.into_iter().map(|v| v as f32).collect();
    BgeEmbedding::new(averaged)
}

/// Fuses semantic vector ranking (`vector_ids`, best-first) and Postgres full-text ranking
/// (`fts_ids`, best-first) via deterministic Reciprocal Rank Fusion (`1 / (60 + rank)`).
pub fn fuse_ranked_unit_ids(vector_ids: &[Uuid], fts_ids: &[Uuid]) -> Vec<Uuid> {
    let mut scores: HashMap<Uuid, f64> = HashMap::new();
    let mut first_pos: HashMap<Uuid, usize> = HashMap::new();
    let mut ordinal = 0_usize;

    for (rank, &id) in vector_ids.iter().enumerate() {
        *scores.entry(id).or_default() += 1.0 / (RRF_K + (rank as f64) + 1.0);
        first_pos.entry(id).or_insert_with(|| {
            let pos = ordinal;
            ordinal += 1;
            pos
        });
    }
    for (rank, &id) in fts_ids.iter().enumerate() {
        *scores.entry(id).or_default() += 1.0 / (RRF_K + (rank as f64) + 1.0);
        first_pos.entry(id).or_insert_with(|| {
            let pos = ordinal;
            ordinal += 1;
            pos
        });
    }

    let mut ranked: Vec<(Uuid, f64, usize)> = scores
        .into_iter()
        .map(|(id, score)| (id, score, first_pos[&id]))
        .collect();
    ranked.sort_by(|left, right| {
        right
            .1
            .total_cmp(&left.1)
            .then_with(|| left.2.cmp(&right.2))
            .then_with(|| left.0.cmp(&right.0))
    });
    ranked.into_iter().map(|(id, _, _)| id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bge_embedding_validates_dimension_finiteness_and_normalizes() {
        assert!(matches!(
            BgeEmbedding::new(vec![1.0; 10]),
            Err(EmbedError::InvalidDimension {
                expected: 384,
                actual: 10
            })
        ));
        let mut nan_vec = vec![0.0; BGE_EMBEDDING_DIM];
        nan_vec[0] = f32::NAN;
        assert!(matches!(
            BgeEmbedding::new(nan_vec),
            Err(EmbedError::NonFiniteComponent)
        ));
        assert!(matches!(
            BgeEmbedding::new(vec![0.0; BGE_EMBEDDING_DIM]),
            Err(EmbedError::ZeroNorm)
        ));

        let mut raw = vec![0.0_f32; BGE_EMBEDDING_DIM];
        raw[0] = 3.0;
        raw[1] = 4.0;
        let emb = BgeEmbedding::new(raw).unwrap();
        assert!((emb.as_slice()[0] - 0.6).abs() < 1e-6);
        assert!((emb.as_slice()[1] - 0.8).abs() < 1e-6);
        assert!((emb.cosine_similarity(&emb) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rrf_places_dual_match_first_and_retains_semantic_only_candidates() {
        let sem_only = Uuid::from_u128(1);
        let both = Uuid::from_u128(2);
        let fts_only = Uuid::from_u128(3);
        let fused = fuse_ranked_unit_ids(&[sem_only, both], &[fts_only, both]);
        assert_eq!(fused, vec![both, sem_only, fts_only]);
    }

    #[tokio::test]
    async fn subprocess_encoder_speaks_duet_protocol_lines() {
        let script = std::env::temp_dir().join(format!("mock-bge-{}.sh", Uuid::new_v4().simple()));
        let vec_json = serde_json::to_string(&vec![1.0_f32; BGE_EMBEDDING_DIM]).unwrap();
        let body = format!(
            "#!/bin/sh\necho '{{\"protocol\":\"duet-bge-cls-v1\",\"recipe\":\"test\"}}'\nwhile IFS= read -r line; do\n  case \"$line\" in\n    *'\"operation\":\"query\"'*) echo '{{\"result\":{vec_json}}}' ;;\n    *'\"texts\"'*) echo '{{\"result\":[[{vec_json}]]}}' ;;\n    *) echo '{{\"error\":\"unexpected\"}}' ;;\n  esac\ndone\n"
        );
        std::fs::write(&script, body).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        let encoder = SubprocessBgeEncoder::new(&script, "/unused-model-dir");
        let q = encoder.embed_query("renal protection").await.unwrap();
        assert_eq!(q.as_slice().len(), BGE_EMBEDDING_DIM);
        let docs = encoder
            .embed_passages(&["Initiate dapagliflozin for CKD.".into()])
            .await
            .unwrap();
        assert_eq!(docs.len(), 1);
        assert!((q.cosine_similarity(&docs[0]) - 1.0).abs() < 1e-5);
        let _ = std::fs::remove_file(&script);
    }
}

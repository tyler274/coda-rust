//! File and shell tools an HTTP worker gets beside the board tools, confined
//! to its workspace. A failed call is a tool error for the model; only
//! cancellation ends the turn.

use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use serde::Deserialize;
use serde_json::{Value, json};

use super::Cancel;
use super::mcp_client::{CallResult, Tool};
use super::process::{self, Ended, LineSink, ProcessLimits, Tail};

/// Most text one tool result carries back to the model.
const OUTPUT_CAP: usize = 32 * 1024;
const DEFAULT_READ_LINES: usize = 2000;
const MAX_LIST_ENTRIES: usize = 500;
const SEARCH_TIMEOUT: Duration = Duration::from_secs(60);
const DEFAULT_SHELL_TIMEOUT: u64 = 120;
const MAX_SHELL_TIMEOUT: u64 = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceTool {
    ReadFile,
    WriteFile,
    EditFile,
    ListDir,
    Search,
    Shell,
}

impl WorkspaceTool {
    pub const ALL: [Self; 6] = [
        Self::ReadFile,
        Self::WriteFile,
        Self::EditFile,
        Self::ListDir,
        Self::Search,
        Self::Shell,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::ReadFile => "read_file",
            Self::WriteFile => "write_file",
            Self::EditFile => "edit_file",
            Self::ListDir => "list_dir",
            Self::Search => "search",
            Self::Shell => "shell",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tool| tool.name() == name)
    }

    pub fn declaration(self) -> Tool {
        let (description, properties, required): (&str, Value, &[&str]) = match self {
            Self::ReadFile => (
                "Read a text file in your workspace. `offset` is the first line (1-based), \
                 `limit` the line count (default 2000). Long output is truncated with a marker \
                 naming the next offset.",
                json!({
                    "path": { "type": "string" },
                    "offset": { "type": "integer", "minimum": 1 },
                    "limit": { "type": "integer", "minimum": 1 },
                }),
                &["path"],
            ),
            Self::WriteFile => (
                "Create or overwrite a file in your workspace with `content`, creating parent \
                 directories.",
                json!({
                    "path": { "type": "string" },
                    "content": { "type": "string" },
                }),
                &["path", "content"],
            ),
            Self::EditFile => (
                "Replace the exact text `old` with `new` in a file. Fails unless `old` occurs \
                 exactly once, or set `replace_all` to replace every occurrence.",
                json!({
                    "path": { "type": "string" },
                    "old": { "type": "string" },
                    "new": { "type": "string" },
                    "replace_all": { "type": "boolean" },
                }),
                &["path", "old", "new"],
            ),
            Self::ListDir => (
                "List a directory in your workspace (default the workspace root). Directories \
                 end in `/`.",
                json!({ "path": { "type": "string" } }),
                &[],
            ),
            Self::Search => (
                "Search file contents with ripgrep. `pattern` is a regex; `path` narrows the \
                 search, `glob` filters files (e.g. `*.rs`). Returns `file:line:text` matches.",
                json!({
                    "pattern": { "type": "string" },
                    "path": { "type": "string" },
                    "glob": { "type": "string" },
                }),
                &["pattern"],
            ),
            Self::Shell => (
                "Run a command with `sh -c` in your workspace and return its combined output \
                 and exit status. `timeout_secs` defaults to 120, at most 600.",
                json!({
                    "command": { "type": "string" },
                    "timeout_secs": { "type": "integer", "minimum": 1, "maximum": MAX_SHELL_TIMEOUT },
                }),
                &["command"],
            ),
        };
        Tool {
            name: self.name().to_owned(),
            description: description.to_owned(),
            input_schema: json!({
                "type": "object",
                "properties": properties,
                "required": required,
                "additionalProperties": false,
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadFile {
    path: String,
    offset: Option<usize>,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WriteFile {
    path: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditFile {
    path: String,
    old: String,
    new: String,
    #[serde(default)]
    replace_all: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListDir {
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Search {
    pattern: String,
    path: Option<String>,
    glob: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shell {
    command: String,
    timeout_secs: Option<u64>,
}

#[derive(Debug)]
pub enum WorkspaceCall {
    ReadFile(ReadFile),
    WriteFile(WriteFile),
    EditFile(EditFile),
    ListDir(ListDir),
    Search(Search),
    Shell(Shell),
}

impl WorkspaceCall {
    pub fn parse(tool: WorkspaceTool, arguments: Value) -> Result<Self, String> {
        let parsed = match tool {
            WorkspaceTool::ReadFile => serde_json::from_value(arguments).map(Self::ReadFile),
            WorkspaceTool::WriteFile => serde_json::from_value(arguments).map(Self::WriteFile),
            WorkspaceTool::EditFile => serde_json::from_value(arguments).map(Self::EditFile),
            WorkspaceTool::ListDir => serde_json::from_value(arguments).map(Self::ListDir),
            WorkspaceTool::Search => serde_json::from_value(arguments).map(Self::Search),
            WorkspaceTool::Shell => serde_json::from_value(arguments).map(Self::Shell),
        };
        parsed.map_err(|error| format!("invalid {} arguments: {error}", tool.name()))
    }
}

/// The turn was cancelled while a tool ran.
pub struct Cancelled;

/// A worker's workspace: the canonical root every path must stay inside,
/// and what its commands run with.
pub struct Workspace {
    root: PathBuf,
    env: Vec<(String, String)>,
    limits: ProcessLimits,
}

impl Workspace {
    pub fn open(
        cwd: &Path,
        env: Vec<(String, String)>,
        limits: ProcessLimits,
    ) -> Result<Self, String> {
        let root = fs::canonicalize(cwd)
            .map_err(|error| format!("workspace {}: {error}", cwd.display()))?;
        Ok(Self { root, env, limits })
    }

    pub fn run(&self, call: WorkspaceCall, cancel: &Cancel) -> Result<CallResult, Cancelled> {
        let done = match call {
            WorkspaceCall::ReadFile(call) => self.read_file(&call),
            WorkspaceCall::WriteFile(call) => self.write_file(&call),
            WorkspaceCall::EditFile(call) => self.edit_file(&call),
            WorkspaceCall::ListDir(call) => self.list_dir(&call),
            WorkspaceCall::Search(call) => return self.search(&call, cancel),
            WorkspaceCall::Shell(call) => return self.shell(&call, cancel),
        };
        Ok(match done {
            Ok(text) => CallResult {
                text,
                is_error: false,
            },
            Err(text) => error(text),
        })
    }

    /// Resolves `path` inside the root. The longest existing prefix is
    /// canonicalized, so `..` and symlinks are followed before the check;
    /// the missing rest may only name new entries.
    fn resolve(&self, path: &str) -> Result<PathBuf, String> {
        let outside = || format!("{path}: outside the workspace");
        let joined = self.root.join(path);
        let mut existing = joined.as_path();
        let mut missing = Vec::new();
        while fs::symlink_metadata(existing).is_err() {
            let (Some(parent), Some(name)) = (existing.parent(), existing.file_name()) else {
                return Err(outside());
            };
            missing.push(name);
            existing = parent;
        }
        let mut resolved =
            fs::canonicalize(existing).map_err(|error| format!("{path}: {error}"))?;
        resolved.extend(missing.into_iter().rev());
        if resolved.starts_with(&self.root) {
            Ok(resolved)
        } else {
            Err(outside())
        }
    }

    fn relative<'p>(&self, path: &'p Path) -> &'p Path {
        match path.strip_prefix(&self.root) {
            Ok(relative) if !relative.as_os_str().is_empty() => relative,
            _ => Path::new("."),
        }
    }

    fn read_file(&self, call: &ReadFile) -> Result<String, String> {
        let path = self.resolve(&call.path)?;
        let file = fs::File::open(&path).map_err(|error| format!("{}: {error}", call.path))?;
        let failed = |error: std::io::Error| format!("{}: {error}", call.path);
        let first = call.offset.unwrap_or(1).max(1);
        let limit = call.limit.unwrap_or(DEFAULT_READ_LINES);
        let mut reader = BufReader::new(file);
        for _ in 1..first {
            if reader.skip_until(b'\n').map_err(failed)? == 0 {
                return Ok(String::new());
            }
        }
        let mut text = Vec::new();
        for number in first..first.saturating_add(limit) {
            let room = OUTPUT_CAP - text.len();
            let read = (&mut reader)
                .take(room as u64)
                .read_until(b'\n', &mut text)
                .map_err(failed)?;
            if read == 0 {
                return Ok(String::from_utf8_lossy(&text).into_owned());
            }
            if text.len() == OUTPUT_CAP && !reader.fill_buf().map_err(failed)?.is_empty() {
                let next = if text.ends_with(b"\n") {
                    number + 1
                } else {
                    number
                };
                return Ok(format!(
                    "{}\n[truncated at {OUTPUT_CAP} bytes: continue with offset {next}]",
                    String::from_utf8_lossy(&text)
                ));
            }
        }
        let mut text = String::from_utf8_lossy(&text).into_owned();
        if !reader.fill_buf().map_err(failed)?.is_empty() {
            text.push_str(&format!(
                "\n[truncated: continue with offset {}]",
                first.saturating_add(limit)
            ));
        }
        Ok(text)
    }

    fn write_file(&self, call: &WriteFile) -> Result<String, String> {
        let path = self.resolve(&call.path)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", call.path))?;
        }
        fs::write(&path, &call.content).map_err(|error| format!("{}: {error}", call.path))?;
        Ok(format!(
            "wrote {} bytes to {}",
            call.content.len(),
            call.path
        ))
    }

    fn edit_file(&self, call: &EditFile) -> Result<String, String> {
        if call.old.is_empty() {
            return Err("`old` must not be empty".into());
        }
        let path = self.resolve(&call.path)?;
        let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", call.path))?;
        let count = text.matches(&call.old).count();
        if count == 0 {
            return Err(format!("{}: `old` not found", call.path));
        }
        if count > 1 && !call.replace_all {
            return Err(format!(
                "{}: `old` occurs {count} times; add context or set replace_all",
                call.path
            ));
        }
        fs::write(&path, text.replace(&call.old, &call.new))
            .map_err(|error| format!("{}: {error}", call.path))?;
        Ok(format!("replaced {count} occurrence(s) in {}", call.path))
    }

    fn list_dir(&self, call: &ListDir) -> Result<String, String> {
        let shown = call.path.as_deref().unwrap_or(".");
        let path = self.resolve(shown)?;
        let mut names = fs::read_dir(&path)
            .and_then(|entries| {
                entries
                    .map(|entry| {
                        let entry = entry?;
                        let mut name = entry.file_name().to_string_lossy().into_owned();
                        if entry.file_type()?.is_dir() {
                            name.push('/');
                        }
                        Ok(name)
                    })
                    .collect::<std::io::Result<Vec<_>>>()
            })
            .map_err(|error| format!("{shown}: {error}"))?;
        names.sort();
        let total = names.len();
        names.truncate(MAX_LIST_ENTRIES);
        let mut text = names.join("\n");
        if total > MAX_LIST_ENTRIES {
            text.push_str(&format!(
                "\n[truncated: {total} entries, {MAX_LIST_ENTRIES} shown]"
            ));
        }
        Ok(text)
    }

    fn search(&self, call: &Search, cancel: &Cancel) -> Result<CallResult, Cancelled> {
        let path = match self.resolve(call.path.as_deref().unwrap_or(".")) {
            Ok(path) => path,
            Err(message) => return Ok(error(message)),
        };
        let mut command = Command::new("rg");
        command.args([
            "--line-number",
            "--no-heading",
            "--color=never",
            "--max-columns=300",
        ]);
        if let Some(glob) = &call.glob {
            command.arg("--glob").arg(glob);
        }
        command
            .arg("--regexp")
            .arg(&call.pattern)
            .arg("--")
            .arg(self.relative(&path))
            .current_dir(&self.root);
        Ok(match self.supervised(command, SEARCH_TIMEOUT, cancel) {
            Ok((Ended::Cancelled, _)) => return Err(Cancelled),
            Ok((Ended::Exited(status), _)) if status.code() == Some(1) => CallResult {
                text: "no matches".into(),
                is_error: false,
            },
            Ok((Ended::Exited(status), output)) => CallResult {
                text: output,
                is_error: !status.success(),
            },
            Ok((Ended::TimedOut(limit), output)) => error(format!(
                "{output}[search timed out after {}s]",
                limit.as_secs()
            )),
            Err(message) => error(message),
        })
    }

    fn shell(&self, call: &Shell, cancel: &Cancel) -> Result<CallResult, Cancelled> {
        let timeout = call
            .timeout_secs
            .unwrap_or(DEFAULT_SHELL_TIMEOUT)
            .clamp(1, MAX_SHELL_TIMEOUT);
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg(&call.command)
            .current_dir(&self.root)
            .envs(self.env.iter().map(|(name, value)| (name, value)));
        Ok(
            match self.supervised(command, Duration::from_secs(timeout), cancel) {
                Ok((Ended::Cancelled, _)) => return Err(Cancelled),
                Ok((Ended::Exited(status), output)) => CallResult {
                    text: format!("{output}[{status}]"),
                    is_error: !status.success(),
                },
                Ok((Ended::TimedOut(limit), output)) => {
                    error(format!("{output}[timed out after {}s]", limit.as_secs()))
                }
                Err(message) => error(message),
            },
        )
    }

    /// Runs `command` in its own process group under the turn's cancel,
    /// with stdout and stderr collected into one capped buffer.
    fn supervised(
        &self,
        command: Command,
        timeout: Duration,
        cancel: &Cancel,
    ) -> Result<(Ended, String), String> {
        let limits = ProcessLimits {
            turn_timeout: Some(timeout),
            ..self.limits.clone()
        };
        let combined = Combined::default();
        let ran = process::run(
            &limits,
            command,
            String::new(),
            combined.clone(),
            combined.clone(),
            cancel,
        )?;
        let tail = combined.0.lock().unwrap_or_else(PoisonError::into_inner);
        let mut output = tail.text().to_owned();
        if tail.truncated() {
            output.push_str(&format!("[output truncated at {OUTPUT_CAP} bytes]\n"));
        }
        Ok((ran.ended, output))
    }
}

fn error(text: String) -> CallResult {
    CallResult {
        text,
        is_error: true,
    }
}

/// One capped buffer fed by both output pipes.
#[derive(Clone)]
struct Combined(Arc<Mutex<Tail>>);

impl Default for Combined {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Tail::new(OUTPUT_CAP))))
    }
}

impl LineSink for Combined {
    fn line(&mut self, line: &str) {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(line);
    }
}

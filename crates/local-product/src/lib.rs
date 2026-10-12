//! Reimplementation of the local product's model execution.
//!
//! The coda-agents checkout is the reference and is not modified. Chat,
//! background, worker, and linking use the same bindings as
//! `modelRoleConfiguration.ts`. `api` calls OpenAI Responses. The key is an
//! argument from the environment; this crate does not read it from disk and
//! does not call the provider.
//! Book: `book/src/north-star.md`.
#![deny(missing_docs)]

use std::collections::HashMap;

use serde_json::{Value, json};

/// How this process runs a model. Unset means `live`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelExecution {
    /// The Codex CLI, the default.
    Live,
    /// Record a Codex cassette.
    Record,
    /// Replay a Codex cassette.
    Replay,
    /// OpenAI Responses. Requires an authorized key.
    Api,
}

/// One role's model, effort, and adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelBinding {
    /// The provider model name.
    pub model: String,
    /// The reasoning effort.
    pub effort: String,
    /// The adapter identity.
    pub adapter: String,
}

/// The four roles the local product binds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRoles {
    /// Physician chat.
    pub chat: ModelBinding,
    /// Background work.
    pub background: ModelBinding,
    /// Worker work.
    pub worker: ModelBinding,
    /// Linking work.
    pub linking: ModelBinding,
}

/// One turn the Responses request will carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatTurn {
    /// `user` for the physician, `assistant` for the model.
    pub role: String,
    /// The authored text.
    pub text: String,
}

/// The result of accepting a physician message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Submit {
    /// The first time this idempotency key was seen. `request` is ready to send.
    Accepted {
        /// The Responses request body.
        request: Value,
    },
    /// This idempotency key was already accepted.
    Duplicate,
    /// The message has no text.
    Rejected,
}

/// Turns for one process. The HTTP server owns persistence later.
#[derive(Debug)]
pub struct ChatSession {
    role: ModelBinding,
    chats: HashMap<(String, String), Vec<ChatTurn>>,
    seen: HashMap<String, (String, String)>,
}

impl ChatSession {
    /// Binds the chat role. Other roles are not called from a physician message.
    pub fn new(role: ModelBinding) -> Self {
        Self {
            role,
            chats: HashMap::new(),
            seen: HashMap::new(),
        }
    }

    /// Records the physician turn and builds the Responses request.
    pub fn submit(
        &mut self,
        idempotency_key: &str,
        workspace_id: &str,
        chat_id: &str,
        text: &str,
    ) -> Submit {
        if let Some(existing) = self.seen.get(idempotency_key) {
            return if existing == &(workspace_id.to_owned(), chat_id.to_owned()) {
                Submit::Duplicate
            } else {
                Submit::Rejected
            };
        }
        if text.trim().is_empty() {
            return Submit::Rejected;
        }
        let key = (workspace_id.to_owned(), chat_id.to_owned());
        let turns = self.chats.entry(key.clone()).or_default();
        turns.push(ChatTurn {
            role: "user".to_owned(),
            text: text.to_owned(),
        });
        self.seen.insert(
            idempotency_key.to_owned(),
            (workspace_id.to_owned(), chat_id.to_owned()),
        );
        Submit::Accepted {
            request: responses_body(&self.role, turns),
        }
    }

    /// Records the model's authored text on the chat the request was for.
    pub fn record_reply(&mut self, workspace_id: &str, chat_id: &str, text: &str) {
        self.chats
            .entry((workspace_id.to_owned(), chat_id.to_owned()))
            .or_default()
            .push(ChatTurn {
                role: "assistant".to_owned(),
                text: text.to_owned(),
            });
    }

    /// The authored texts in order, physician and model.
    pub fn conversation(&self, workspace_id: &str, chat_id: &str) -> Vec<String> {
        self.chats
            .get(&(workspace_id.to_owned(), chat_id.to_owned()))
            .map(|turns| turns.iter().map(|turn| turn.text.clone()).collect())
            .unwrap_or_default()
    }
}

/// Reads `CODA_LOCAL_MODEL_EXECUTION`. Unset is `live`.
pub fn model_execution(value: Option<&str>) -> Result<ModelExecution, String> {
    match value.unwrap_or("live") {
        "live" => Ok(ModelExecution::Live),
        "record" => Ok(ModelExecution::Record),
        "replay" => Ok(ModelExecution::Replay),
        "api" => Ok(ModelExecution::Api),
        _ => Err(
            "CODA_LOCAL_MODEL_EXECUTION must be live, record, replay, or api; live is the default"
                .to_owned(),
        ),
    }
}

/// The key, after the allow flag. Does not call the provider.
pub fn open_ai_api_key(allow: Option<&str>, key: Option<&str>) -> Result<String, String> {
    if allow != Some("1") {
        return Err("Set CODA_ALLOW_MODEL_CALLS=1 to authorize real model calls".to_owned());
    }
    let key = key.unwrap_or("");
    if key.is_empty() {
        return Err("OPENAI_API_KEY is required".to_owned());
    }
    Ok(key.to_owned())
}

/// Binds every role to `adapter`. `CODA_<ROLE>_MODEL` and `CODA_<ROLE>_EFFORT` override one role.
pub fn model_roles(adapter: &str, environment: &[(&str, &str)]) -> Result<ModelRoles, String> {
    Ok(ModelRoles {
        chat: binding("chat", "gpt-6-sol", "high", adapter, environment)?,
        background: binding("background", "gpt-6-luna", "max", adapter, environment)?,
        worker: binding("worker", "gpt-6-luna", "max", adapter, environment)?,
        linking: binding("linking", "gpt-6-luna", "max", adapter, environment)?,
    })
}

/// The Responses body for one chat role. `store` stays off.
pub fn responses_body(role: &ModelBinding, turns: &[ChatTurn]) -> Value {
    json!({
        "model": role.model,
        "reasoning": { "effort": role.effort },
        "input": turns.iter().map(|turn| json!({
            "role": turn.role,
            "content": turn.text,
        })).collect::<Vec<_>>(),
        "store": false,
    })
}

/// The authored text from a Responses JSON body.
pub fn authored_text(response: &Value) -> Option<String> {
    let output = response.get("output")?.as_array()?;
    let mut text = String::new();
    for item in output {
        if item.get("type").and_then(Value::as_str) != Some("message") {
            continue;
        }
        let Some(content) = item.get("content").and_then(Value::as_array) else {
            continue;
        };
        for part in content {
            if part.get("type").and_then(Value::as_str) == Some("output_text")
                && let Some(piece) = part.get("text").and_then(Value::as_str)
            {
                text.push_str(piece);
            }
        }
    }
    if text.is_empty() {
        return None;
    }
    if let Ok(value) = serde_json::from_str::<Value>(&text)
        && let Some(authored) = value.get("text").and_then(Value::as_str)
        && !authored.is_empty()
    {
        return Some(authored.to_owned());
    }
    Some(text)
}

fn binding(
    role: &str,
    model: &str,
    effort: &str,
    adapter: &str,
    environment: &[(&str, &str)],
) -> Result<ModelBinding, String> {
    let prefix = format!("CODA_{}", role.to_uppercase());
    let model_key = format!("{prefix}_MODEL");
    let effort_key = format!("{prefix}_EFFORT");
    let model = environment
        .iter()
        .find(|(key, _)| *key == model_key)
        .map(|(_, value)| *value)
        .unwrap_or(model);
    let effort = environment
        .iter()
        .find(|(key, _)| *key == effort_key)
        .map(|(_, value)| *value)
        .unwrap_or(effort);
    Ok(ModelBinding {
        model: one_word(model, &model_key, &effort_key)?,
        effort: one_word(effort, &model_key, &effort_key)?,
        adapter: adapter.to_owned(),
    })
}

fn one_word(value: &str, model_key: &str, effort_key: &str) -> Result<String, String> {
    if value.is_empty() || value.chars().any(char::is_whitespace) {
        return Err(format!(
            "{model_key} and {effort_key} must each be one nonblank word"
        ));
    }
    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_mode_refuses_without_the_allow_flag_or_key() {
        assert!(open_ai_api_key(None, Some("test-key")).is_err());
        assert!(
            open_ai_api_key(None, Some("test-key"))
                .unwrap_err()
                .contains("CODA_ALLOW_MODEL_CALLS=1")
        );
        assert!(
            open_ai_api_key(Some("1"), None)
                .unwrap_err()
                .contains("OPENAI_API_KEY is required")
        );
        assert_eq!(
            open_ai_api_key(Some("1"), Some("test-key")).unwrap(),
            "test-key"
        );
        assert_eq!(model_execution(None).unwrap(), ModelExecution::Live);
        assert_eq!(model_execution(Some("api")).unwrap(), ModelExecution::Api);
        assert!(
            model_execution(Some("paid"))
                .unwrap_err()
                .contains("live, record, replay, or api")
        );
    }

    #[test]
    fn roles_match_the_existing_luna_bindings() {
        let roles = model_roles("openai-responses", &[]).unwrap();
        assert_eq!(roles.chat.model, "gpt-6-sol");
        assert_eq!(roles.chat.effort, "high");
        assert_eq!(roles.chat.adapter, "openai-responses");
        for binding in [&roles.background, &roles.worker, &roles.linking] {
            assert_eq!(binding.model, "gpt-6-luna");
            assert_eq!(binding.effort, "max");
            assert_eq!(binding.adapter, "openai-responses");
        }
        let overridden =
            model_roles("openai-responses", &[("CODA_BACKGROUND_EFFORT", "high")]).unwrap();
        assert_eq!(overridden.background.effort, "high");
        assert_eq!(overridden.background.model, "gpt-6-luna");
        assert!(model_roles("openai-responses", &[("CODA_CHAT_MODEL", "two words")]).is_err());
    }

    #[test]
    fn a_chat_request_uses_the_chat_role_and_keeps_one_reply() {
        let roles = model_roles("openai-responses", &[]).unwrap();
        let mut session = ChatSession::new(roles.chat);
        let Submit::Accepted { request } = session.submit("k1", "pw", "chat", "Hello") else {
            panic!("expected the first submit to be accepted");
        };
        assert_eq!(request["model"], "gpt-6-sol");
        assert_eq!(request["reasoning"]["effort"], "high");
        assert_eq!(request["store"], false);
        assert_eq!(request["input"][0]["content"], "Hello");
        assert_eq!(
            session.submit("k1", "pw", "chat", "Hello"),
            Submit::Duplicate
        );
        assert_eq!(session.conversation("pw", "chat"), vec!["Hello".to_owned()]);

        let response = json!({
            "output": [{
                "type": "message",
                "content": [{ "type": "output_text", "text": "{\"text\":\"Noted.\",\"completion\":null}" }]
            }]
        });
        let text = authored_text(&response).unwrap();
        assert_eq!(text, "Noted.");
        session.record_reply("pw", "chat", &text);
        assert_eq!(
            session.conversation("pw", "chat"),
            vec!["Hello".to_owned(), "Noted.".to_owned()]
        );
    }
}

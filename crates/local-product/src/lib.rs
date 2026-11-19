//! Physician model execution and durable product state.
//! Postgres owns chat and reply revisions; the worker runs both through the shared
//! harness using the selected local subscription or API transport.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

use sha2::{Digest, Sha256};

/// Local multimodal attachment and page-by-page clinical document reading.
pub mod attachments;
pub mod chatgpt;
/// Clinical library search and exact passage read tools for physician chat.
pub mod clinical_library;
/// Recipe, initial context, and memory tools for patient understanding runs.
pub mod initial_understanding;
/// Linked patient message review and physician-requested reply revision recipes.
pub mod message_review_recipe;
/// Rounds recipe, initial context, and review tools for background and physician-requested patient reviews.
pub mod patient_reviews;
/// Hosted physician chat instructions and the `skill_view` tool.
pub mod physician_chat;
/// Shared source record format, adapter mappings, and chart-reading tools.
pub mod physician_sources;
/// Physician-owned durable conversations and execution authority.
pub mod runtime;
/// Practice-scoped source patient links, pull status, and check/reconnect operations.
pub mod source_status;
/// Unmatched Spruce message classification and patient-match suggestion recipe.
pub mod spruce_matching_recipe;
/// Recipe, context, and tool descriptors for visit selection and visit briefings.
pub mod visit_preparation;

/// How this process runs a model. A process without a selection does not run models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelExecution {
    /// No local inference has been selected.
    Disabled,
    /// Direct Responses calls authorized on a local developer's ChatGPT plan.
    Subscription,
    /// OpenAI Responses. Requires an authorized key.
    Api,
}

/// One role's model, effort, and adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelBinding {
    /// The provider model name.
    pub model: String,
    /// Product-selected working window, measured with the harness estimate.
    pub context_window_tokens: u64,
    /// Provider input ceiling, checked against the estimated dispatch size.
    pub max_input_tokens: u64,
    /// The reasoning effort.
    pub effort: String,
    /// The adapter identity.
    pub adapter: String,
}

/// The jobs the local product binds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRoles {
    /// Physician chat.
    pub chat: ModelBinding,
    /// Background work.
    pub background: ModelBinding,
}

/// Selects the explicitly configured local inference mode.
pub fn model_execution(value: Option<&str>) -> Result<ModelExecution, String> {
    match value {
        None => Ok(ModelExecution::Disabled),
        Some("subscription") => Ok(ModelExecution::Subscription),
        Some("api") => Ok(ModelExecution::Api),
        Some(_) => Err("CODA_LOCAL_MODEL_EXECUTION must be subscription or api".into()),
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

/// Shared validation of the product's configured model roles.
pub use harness::settings::Error as ModelBindingError;

/// The same explicit role settings are read by local and hosted Chords.
pub const MODEL_SETTING_KEYS: [&str; 8] = [
    "CODA_CHAT_MODEL",
    "CODA_CHAT_EFFORT",
    "CODA_CHAT_CONTEXT_WINDOW_TOKENS",
    "CODA_CHAT_INPUT_LIMIT_TOKENS",
    "CODA_BACKGROUND_MODEL",
    "CODA_BACKGROUND_EFFORT",
    "CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS",
    "CODA_BACKGROUND_INPUT_LIMIT_TOKENS",
];

/// Bind each role to its required model, effort, working window and input limit.
pub fn model_roles(
    adapter: &str,
    environment: &[(&str, &str)],
) -> Result<ModelRoles, ModelBindingError> {
    let settings = harness::settings::Reader(|key: &str| {
        environment
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| (*value).to_owned())
    });
    let binding = |role: &str| -> Result<ModelBinding, ModelBindingError> {
        let job = settings.job(&format!("CODA_{role}"))?;
        Ok(ModelBinding {
            model: job.model,
            effort: job.effort,
            context_window_tokens: job.context_window_tokens,
            max_input_tokens: job.max_input_tokens,
            adapter: adapter.to_owned(),
        })
    };
    Ok(ModelRoles {
        chat: binding("CHAT")?,
        background: binding("BACKGROUND")?,
    })
}

/// Computes a lowercase hexadecimal SHA-256 digest of `bytes`.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

/// Sanitizes untrusted channel text (patient messages, sender display names, and
/// filenames) before embedding it in model prompts.
///
/// Strips C0/C1 control characters (preserving tabs and line breaks), Unicode
/// bidirectional overrides, and active HTML/script blocks or markup tags while
/// leaving clinical prose and numeric inequalities (`K+ > 5.0 mmol/L`, `A1c < 6.5%`)
/// intact.
pub fn sanitize_untrusted_channel_text(raw: &str) -> String {
    let mut cleaned = String::with_capacity(raw.len());
    for ch in raw.chars() {
        let forbidden_override = matches!(
            ch,
            '\u{200E}'
                | '\u{200F}'
                | '\u{202A}'..='\u{202E}'
                | '\u{2066}'..='\u{2069}'
                | '\u{FEFF}'
        );
        let allowed_whitespace = matches!(ch, '\n' | '\r' | '\t');
        if forbidden_override || (ch.is_control() && !allowed_whitespace) {
            continue;
        }
        cleaned.push(ch);
    }
    strip_active_html_markup(&cleaned)
}

fn strip_active_html_markup(input: &str) -> String {
    const BLOCK_TAGS: &[&str] = &["script", "style", "iframe", "svg", "object", "embed"];
    let mut out = String::with_capacity(input.len());
    let mut idx = 0;
    let bytes = input.as_bytes();
    while idx < input.len() {
        let rest = &input[idx..];
        let Some(rel_lt) = rest.find('<') else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..rel_lt]);
        idx += rel_lt;

        let after_lt = &input[idx + 1..];
        if let Some(after_comment) = after_lt.strip_prefix("!--")
            && let Some(end_comment) = after_comment.find("-->")
        {
            idx += 1 + 3 + end_comment + 3;
            continue;
        }

        let lower_prefix: String = after_lt
            .chars()
            .take(16)
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let mut matched_block = None;
        for tag in BLOCK_TAGS {
            if let Some(after_tag) = lower_prefix.strip_prefix(tag)
                && after_tag
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_whitespace() || matches!(c, '>' | '/'))
            {
                matched_block = Some(*tag);
                break;
            }
        }

        if let Some(tag) = matched_block
            && let Some(open_end) = after_lt.find('>')
        {
            let body_start = idx + 1 + open_end + 1;
            let close_needle = format!("</{tag}");
            let body_lower = input[body_start..].to_ascii_lowercase();
            if let Some(rel_close) = body_lower.find(&close_needle)
                && let Some(close_gt) = input[body_start + rel_close..].find('>')
            {
                idx = body_start + rel_close + close_gt + 1;
                continue;
            }
            idx = body_start;
            continue;
        }

        let is_tag_start = after_lt
            .as_bytes()
            .first()
            .is_some_and(|b| b.is_ascii_alphabetic())
            || (after_lt.starts_with('/')
                && after_lt
                    .as_bytes()
                    .get(1)
                    .is_some_and(|b| b.is_ascii_alphabetic()))
            || (after_lt.starts_with('!')
                && after_lt
                    .as_bytes()
                    .get(1)
                    .is_some_and(|b| b.is_ascii_alphabetic()));

        if is_tag_start
            && let Some(rel_gt) = after_lt.find('>')
            && rel_gt <= 512
            && !after_lt[..rel_gt].contains('<')
        {
            idx += 1 + rel_gt + 1;
            continue;
        }

        let _ = bytes;
        out.push('<');
        idx += 1;
    }
    out
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
        assert_eq!(model_execution(None).unwrap(), ModelExecution::Disabled);
        assert_eq!(model_execution(Some("api")).unwrap(), ModelExecution::Api);
        assert_eq!(
            model_execution(Some("subscription")).unwrap(),
            ModelExecution::Subscription
        );
        assert!(model_execution(Some("paid")).is_err());
    }

    #[test]
    fn every_role_requires_a_model_effort_and_both_capacities() {
        let environment = [
            ("CODA_CHAT_MODEL", "chat-model"),
            ("CODA_CHAT_EFFORT", "high"),
            ("CODA_CHAT_CONTEXT_WINDOW_TOKENS", "272000"),
            ("CODA_CHAT_INPUT_LIMIT_TOKENS", "922000"),
            ("CODA_BACKGROUND_MODEL", "background-model"),
            ("CODA_BACKGROUND_EFFORT", "max"),
            ("CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS", "200000"),
            ("CODA_BACKGROUND_INPUT_LIMIT_TOKENS", "150000"),
        ];
        let roles = model_roles("openai-responses", &environment).unwrap();
        for (binding, expected) in [
            (&roles.chat, ("chat-model", "high", 272000, 922000)),
            (
                &roles.background,
                ("background-model", "max", 200000, 150000),
            ),
        ] {
            assert_eq!(
                (
                    binding.model.as_str(),
                    binding.effort.as_str(),
                    binding.context_window_tokens,
                    binding.max_input_tokens
                ),
                expected
            );
            assert_eq!(binding.adapter, "openai-responses");
        }
        // Each deployment key must reach startup; value validation belongs to the shared reader.
        for (missing, _) in environment {
            let incomplete: Vec<_> = environment
                .iter()
                .copied()
                .filter(|(key, _)| *key != missing)
                .collect();
            assert!(
                matches!(model_roles("openai-responses", &incomplete), Err(ModelBindingError::Missing(key)) if key == missing)
            );
        }
    }
}

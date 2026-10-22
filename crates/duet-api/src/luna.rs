//! Duet's Luna assignment: the product's composed instructions, the model input
//! assembled by the conversation owner, and the gated live provider config.
//!
//! Instruction and skill markdown under `agent/` is copied verbatim from the
//! product's `backend/app/agent_instructions` and `agent_skills` (coda-app
//! 8af41d37). Clinical wording is product doctrine: change it there first.
//! Composition order follows `agent_instructions.instructions_for("conversation")`.
//! The version is this workspace's own hash, not byte-identical to Python's.
use front_door::{
    PostgresFrontDoorStore,
    conversation::{OperatorAttestation, execution::Claim},
    identity::AuthorityError,
};
use harness::{
    engine::{Context, Item},
    responses::Responses,
};
use sha2::{Digest, Sha256};

const BLOCKS: [(&str, &str); 6] = [
    (
        "identity",
        include_str!("../agent/instructions/identity.md"),
    ),
    ("health", include_str!("../agent/instructions/health.md")),
    (
        "skill_discovery",
        include_str!("../agent/instructions/skill_discovery.md"),
    ),
    (
        "understanding",
        include_str!("../agent/instructions/understanding.md"),
    ),
    (
        "conversation_continuity",
        include_str!("../agent/instructions/conversation_continuity.md"),
    ),
    (
        "conversation",
        include_str!("../agent/instructions/conversation.md"),
    ),
];
const SKILLS: [&str; 6] = [
    include_str!("../agent/skills/doctor-preparation.md"),
    include_str!("../agent/skills/follow-through.md"),
    include_str!("../agent/skills/health-evidence.md"),
    include_str!("../agent/skills/personal-continuity.md"),
    include_str!("../agent/skills/practical-health-support.md"),
    include_str!("../agent/skills/saved-routines.md"),
];
/// Canonical model named by the product's provider policy.
pub const MODEL: &str = "gpt-6-luna";

/// One bundled skill. Content grants no tool or authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Skill {
    /// Directory name in the product.
    pub name: String,
    /// One-line index description.
    pub description: String,
    /// Body after the front matter.
    pub markdown: String,
}

fn skill(document: &str) -> Result<Skill, String> {
    let rest = document
        .trim()
        .strip_prefix("---")
        .ok_or("skill front matter missing")?;
    let (front, body) = rest.split_once("\n---").ok_or("skill front matter open")?;
    let field = |key: &str| {
        front
            .lines()
            .find_map(|l| l.strip_prefix(&format!("{key}:")))
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
            .ok_or(format!("skill {key} missing"))
    };
    let name = field("name")?;
    let description = field("description")?;
    if !name
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        || description.chars().count() > 500
        || body.trim().is_empty()
    {
        return Err("invalid skill document".into());
    }
    Ok(Skill {
        name,
        description,
        markdown: body.trim().to_owned(),
    })
}

/// Bundled skills in name order.
pub fn skills() -> Result<Vec<Skill>, String> {
    let mut skills = SKILLS
        .iter()
        .map(|d| skill(d))
        .collect::<Result<Vec<_>, _>>()?;
    skills.sort_by(|a, b| a.name.cmp(&b.name));
    if skills.windows(2).any(|w| w[0].name == w[1].name) {
        return Err("duplicate skill".into());
    }
    Ok(skills)
}

/// Composed system instructions for the conversation assignment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instructions {
    /// Exact instruction text.
    pub text: String,
    /// `duet-agent-conversation-<sha256[..16]>` over text and skill library.
    pub version: String,
}

/// Compose identity, health, the skill index, understanding, continuity and
/// the conversation assignment, exactly in the product's order.
pub fn conversation_instructions() -> Result<Instructions, String> {
    let skills = skills()?;
    let mut blocks = Vec::new();
    for (name, text) in BLOCKS {
        let text = text.trim();
        if name == "skill_discovery" {
            let mut index = vec![text.to_owned()];
            index.extend(
                skills
                    .iter()
                    .map(|s| format!("- {}: {}", s.name, s.description)),
            );
            blocks.push(index.join("\n"));
        } else {
            blocks.push(text.to_owned());
        }
    }
    let text = blocks.join("\n\n");
    let mut hash = Sha256::new();
    hash.update(text.as_bytes());
    for s in &skills {
        for part in [&s.name, &s.description, &s.markdown] {
            hash.update((part.len() as u64).to_be_bytes());
            hash.update(part.as_bytes());
        }
    }
    let digest = hash.finalize();
    let hex: String = digest[..8].iter().map(|b| format!("{b:02x}")).collect();
    Ok(Instructions {
        text,
        version: format!("duet-agent-conversation-{hex}"),
    })
}

/// Engine context for a live claim: composed instructions and the owner's
/// JSON document as one user item, the product's input shape. No tools.
pub async fn conversation_context(
    store: &PostgresFrontDoorStore,
    claim: &Claim,
) -> Result<(Context, String), AuthorityError> {
    let instructions = conversation_instructions().map_err(|_| AuthorityError::Refused)?;
    let assembled = store.conversation_context(claim).await?;
    let input = serde_json::to_string(&assembled.payload).map_err(|_| AuthorityError::Refused)?;
    Ok((
        Context {
            instructions: instructions.text,
            items: vec![Item::Message {
                role: "user".into(),
                text: input,
            }],
            tools: vec![],
        },
        instructions.version,
    ))
}

/// Why a live provider was not configured. Never includes secret values.
#[derive(Debug, PartialEq, Eq)]
pub enum NotConfigured {
    /// The API role advertises availability but never holds the credential.
    ApiRole,
    /// `OPENAI_API_KEY` absent or blank.
    MissingKey,
    /// Operator has not attested the project/key for health data.
    NotAttested,
    /// Adapter construction refused the configuration.
    Invalid,
}

/// Gated live provider configuration, mirroring the product's
/// `OpenAISettings.from_env`. Holding one authorizes nothing by itself: the
/// execution permit, tools and memory owners are still required.
pub struct LiveModel {
    key: String,
}
impl LiveModel {
    /// Requires a worker role, a key and the operator health-data attestation.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, NotConfigured> {
        if get("DUET_RUNTIME_ROLE").as_deref() == Some("api") {
            return Err(NotConfigured::ApiRole);
        }
        let key = get("OPENAI_API_KEY").unwrap_or_default().trim().to_owned();
        if key.is_empty() {
            return Err(NotConfigured::MissingKey);
        }
        OperatorAttestation::from_env(&get).ok_or(NotConfigured::NotAttested)?;
        Ok(Self { key })
    }
    /// Streaming Responses adapter at the fixed production endpoint.
    pub fn provider(&self) -> Result<Responses, NotConfigured> {
        Responses::new(self.key.clone(), MODEL.into())
            .map(Responses::streaming)
            .map_err(|_| NotConfigured::Invalid)
    }
}
impl std::fmt::Debug for LiveModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LiveModel { key: <redacted> }")
    }
}

/// Whether the API may advertise Luna availability: operator attestation only,
/// as in the product. Execution still fails closed without a worker permit.
pub fn advertised(get: impl Fn(&str) -> Option<String>) -> bool {
    OperatorAttestation::from_env(get).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_compose_in_product_order_with_full_index() {
        let i = conversation_instructions().unwrap();
        assert!(
            i.text
                .starts_with("You are Duet, one person's ongoing health assistant.")
        );
        let order = [
            "Distinguish what the person reports",
            "Bundled Duet skills are trusted product guidance",
            "- doctor-preparation: ",
            "- saved-routines: ",
            "Maintain the person's private understanding",
            "Conversation context has two lifetimes.",
            "Help the person with their current message",
        ];
        let mut at = 0;
        for needle in order {
            let found = i.text[at..]
                .find(needle)
                .unwrap_or_else(|| panic!("{needle}"));
            at += found + needle.len();
        }
        // Emergency and crisis referral lines are carried verbatim.
        assert!(i.text.contains("call 911 now."));
        assert!(i.text.contains("988"));
        assert_eq!(skills().unwrap().len(), 6);
        assert!(i.version.starts_with("duet-agent-conversation-"));
        assert_eq!(i, conversation_instructions().unwrap());
    }

    #[test]
    fn malformed_skill_documents_are_refused() {
        for bad in [
            "no front matter",
            "---\nname: x\n---\n",
            "---\nname: Bad Name\ndescription: d\n---\nbody",
        ] {
            assert!(skill(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn live_model_requires_worker_key_and_operator_attestation() {
        let env = |pairs: &'static [(&'static str, &'static str)]| {
            move |k: &str| {
                pairs
                    .iter()
                    .find(|(n, _)| *n == k)
                    .map(|(_, v)| (*v).to_owned())
            }
        };
        assert_eq!(
            LiveModel::from_env(env(&[])).unwrap_err(),
            NotConfigured::MissingKey
        );
        assert_eq!(
            LiveModel::from_env(env(&[("OPENAI_API_KEY", "sk-test")])).unwrap_err(),
            NotConfigured::NotAttested
        );
        assert_eq!(
            LiveModel::from_env(env(&[
                ("OPENAI_API_KEY", "sk-test"),
                ("DUET_OPENAI_PHI_READY", "false")
            ]))
            .unwrap_err(),
            NotConfigured::NotAttested
        );
        assert_eq!(
            LiveModel::from_env(env(&[
                ("DUET_RUNTIME_ROLE", "api"),
                ("OPENAI_API_KEY", "sk-test"),
                ("DUET_OPENAI_PHI_READY", "1")
            ]))
            .unwrap_err(),
            NotConfigured::ApiRole
        );
        let live = LiveModel::from_env(env(&[
            ("OPENAI_API_KEY", "sk-test"),
            ("DUET_OPENAI_PHI_READY", "yes"),
        ]))
        .unwrap();
        assert!(!format!("{live:?}").contains("sk-test"));
        assert!(live.provider().is_ok());
        assert!(!advertised(env(&[("OPENAI_API_KEY", "sk-test")])));
        assert!(advertised(env(&[("DUET_OPENAI_PHI_READY", "true")])));
    }
}

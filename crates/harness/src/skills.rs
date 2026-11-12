//! Tool declaration and run-trace provenance for reading a product's packaged guidance.

use std::collections::HashMap;

pub use agent_skills::{
    AgentSkill, Package, PreloadError, PreloadedGuidance, SKILL_VIEW, SkillLoadOrigin,
    SkillLoadRecord, SkillResource, SkillTarget, SkillView, SkillVisibility, load_agent_skills,
    version_id,
};
use serde::{Deserialize, Serialize};

use crate::engine::{Event, EventKind, recipe::RequestRecipe};

/// Complete skill provenance reconstructed from a frozen [`Package`], [`RequestRecipe`], and run [`Event`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillRunTrace {
    /// Product owning the admitted package.
    pub product: String,
    /// Exact `sha256:...` digest of the admitted [`Package`].
    pub package_version: String,
    /// Preloaded and `skill_view`-discovered guidance loads in execution order, with exact `version_id`s.
    pub loads: Vec<SkillLoadRecord>,
}

/// Reconstructs every preloaded and `skill_view`-opened skill or resource and its exact `version_id`
/// from a run's frozen [`Package`], frozen [`RequestRecipe`], and ordered execution [`Event`]s.
/// Parallel read-only `skill_view` calls in one batch retain their deterministic `ToolStarted` order.
pub fn trace_run(
    package: &Package,
    recipe: &RequestRecipe,
    events: &[Event],
) -> Result<SkillRunTrace, PreloadError> {
    let mut loads = package.preload(&recipe.preloaded_skills)?.records;
    let mut pending: HashMap<(&str, &str), (usize, serde_json::Value)> = HashMap::new();
    let mut discovered: Vec<(usize, SkillLoadRecord)> = Vec::new();
    for (index, event) in events.iter().enumerate() {
        match &event.kind {
            EventKind::ToolStarted(call) if call.name == SKILL_VIEW => {
                if let Ok(arguments) = serde_json::from_str::<serde_json::Value>(&call.arguments) {
                    pending.insert(
                        (event.attempt_id.as_str(), call.id.as_str()),
                        (index, arguments),
                    );
                }
            }
            EventKind::ToolReturned { call_id, result } => {
                let Some((started_index, arguments)) =
                    pending.remove(&(event.attempt_id.as_str(), call_id.as_str()))
                else {
                    continue;
                };
                if !result.executed() {
                    continue;
                }
                if let Some((target, version_id)) = package.resolve_view_target(&arguments) {
                    discovered.push((
                        started_index,
                        SkillLoadRecord {
                            target,
                            origin: SkillLoadOrigin::Discovered {
                                call_id: call_id.clone(),
                            },
                            version_id,
                        },
                    ));
                }
            }
            EventKind::ProviderStarted { .. }
            | EventKind::ProviderObservation { .. }
            | EventKind::ProviderFailed { .. }
            | EventKind::TransportRetry { .. }
            | EventKind::ProviderReturned(_)
            | EventKind::ToolStarted(_)
            | EventKind::ProviderBlocked { .. }
            | EventKind::ContextRejected
            | EventKind::Candidate { .. }
            | EventKind::WindowOpened { .. } => (),
        }
    }
    discovered.sort_by_key(|(started_index, _)| *started_index);
    loads.extend(discovered.into_iter().map(|(_, record)| record));
    Ok(SkillRunTrace {
        product: package.product.clone(),
        package_version: package.version.clone(),
        loads,
    })
}

/// The shared skill protocol as an engine tool. Each product supplies the package
/// the index and reader may use. Reading guidance grants no other capability.
pub fn tool() -> crate::engine::Tool {
    crate::engine::Tool {
        name: SKILL_VIEW.into(),
        description: "Read one exact approved packaged skill or one supporting resource returned in its metadata. Supply the name to read SKILL.md; use an exact returned package-relative path for a supporting file. Returns the exact content and registered resource paths, or an explicit unavailable outcome. Large results have a preview; read_tool_result reads the retained content. Read guidance does not grant tools or action authority.".into(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type":"string", "minLength":1, "maxLength":64, "pattern":"^[a-z0-9]+(?:-[a-z0-9]+)*$"},
                "file_path": {"anyOf":[{"type":"string", "minLength":1, "maxLength":500}, {"type":"null"}]}
            },
            "required": ["name"],
            "additionalProperties": false
        }),
    }
}

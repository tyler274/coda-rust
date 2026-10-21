//! Loads `skills/` once and admits `skill_view` on a model message.
//!
//! Skill admission and neutral execution mechanics. Product adapters retain
//! authority, persistence, recovery and completion decisions. Skills never
//! authorize a tool or a patient read. The legacy `Harness::admit` surface
//! admits only `skill_view`; `execution` supplies the structured provider loop.
#![deny(missing_docs)]

pub mod execution;
pub mod responses;
pub mod transport;

use std::path::Path;

use agent_skills::{AgentSkill, SkillView};

/// The skill catalog for one process.
pub struct Harness {
    skills: Vec<AgentSkill>,
}

impl Harness {
    /// Reads `root` once. Later views use this copy.
    pub fn load(root: &Path) -> Result<Self, String> {
        Ok(Self {
            skills: agent_skills::load_agent_skills(root)?,
        })
    }

    /// The compact catalog JSON. Resource metadata only.
    pub fn catalog(&self) -> String {
        agent_skills::catalog_json(&self.skills)
    }

    /// Executes one `skill_view` through the skill crate.
    pub fn skill_view(
        &self,
        name: &str,
        file_path: Option<&str>,
        max_result_bytes: u32,
    ) -> Result<SkillView, String> {
        agent_skills::view_skill(&self.skills, name, file_path, max_result_bytes)
    }

    /// Admits one model message against the loaded catalog.
    ///
    /// Each call named `skill_view` whose arguments the loader can use runs
    /// through [`Self::skill_view`] and returns that view, including an
    /// unavailable skill or an over-budget body. Any other tool name is
    /// refused and its arguments are not read. `skill_view` arguments the
    /// loader rejects are refused with the loader's reason and produce no
    /// view. A valid `skill_view` in the same message still runs. Authored
    /// text is returned unchanged. Nothing from the message is retained.
    pub fn admit(&self, message: &ModelMessage) -> AdmittedMessage {
        AdmittedMessage {
            text: message.text.clone(),
            calls: message
                .calls
                .iter()
                .map(|call| self.admit_call(call))
                .collect(),
        }
    }

    fn admit_call(&self, call: &ToolCall) -> AdmittedCall {
        if call.name != "skill_view" {
            let name = call.name.clone();
            return AdmittedCall::Refused {
                reason: format!("unknown tool {name}"),
                name,
            };
        }
        match self.skill_view(
            &call.arguments.name,
            call.arguments.file_path.as_deref(),
            call.arguments.max_result_bytes,
        ) {
            Ok(view) => AdmittedCall::SkillView(view),
            Err(reason) => AdmittedCall::Refused {
                name: call.name.clone(),
                reason,
            },
        }
    }
}

/// Authored text and the tool calls on one model message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelMessage {
    /// Text the model authored on this message.
    pub text: String,
    /// Tool calls in message order.
    pub calls: Vec<ToolCall>,
}

/// One tool call. Arguments are read only when `name` is `skill_view`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCall {
    /// Tool name as the model wrote it.
    pub name: String,
    /// `name`, optional file path, and max bytes.
    pub arguments: SkillViewArguments,
}

/// The arguments [`Harness::skill_view`] already takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillViewArguments {
    /// Skill name.
    pub name: String,
    /// Resource path. Absent reads the skill body.
    pub file_path: Option<String>,
    /// Maximum encoded result size, in bytes.
    pub max_result_bytes: u32,
}

/// A model message after admission. Calls stay in message order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedMessage {
    /// The message's authored text.
    pub text: String,
    /// One outcome per call.
    pub calls: Vec<AdmittedCall>,
}

/// One tool call after admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmittedCall {
    /// The loader ran and returned this view.
    SkillView(SkillView),
    /// The call was not executed.
    Refused {
        /// Tool name from the call.
        name: String,
        /// Why the call was not executed.
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    fn skills_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills")
    }

    #[test]
    fn skill_view_returns_the_body_or_a_named_gap() {
        let root = skills_dir();
        let harness = Harness::load(&root).unwrap();
        assert!(
            harness
                .catalog()
                .contains("\"kind\":\"approved_skill_catalog\"")
        );

        let body = fs::read_to_string(root.join("pre-visit-preparation/SKILL.md")).unwrap();
        match harness
            .skill_view("pre-visit-preparation", None, 65536)
            .unwrap()
        {
            SkillView::Available {
                content, resources, ..
            } => {
                assert_eq!(content, body);
                assert!(resources.is_some());
            }
            other => panic!("expected the skill body, got {other:?}"),
        }

        match harness.skill_view("not-a-skill", None, 65536).unwrap() {
            SkillView::Unavailable { name, .. } => assert_eq!(name, "not-a-skill"),
            other => panic!("expected unavailable, got {other:?}"),
        }

        match harness
            .skill_view("pre-visit-preparation", None, 4096)
            .unwrap()
        {
            SkillView::OverBudget { required_bytes, .. } => assert!(required_bytes > 4096),
            other => panic!("expected over_budget, got {other:?}"),
        }

        let guide = fs::read_to_string(
            root.join("patient-memory-maintenance/references/authoring-guide.md"),
        )
        .unwrap();
        match harness
            .skill_view(
                "patient-memory-maintenance",
                Some("references/authoring-guide.md"),
                65536,
            )
            .unwrap()
        {
            SkillView::Available {
                content, resources, ..
            } => {
                assert_eq!(content, guide);
                assert!(resources.is_none());
            }
            other => panic!("expected the authoring guide, got {other:?}"),
        }
    }

    #[test]
    fn recorded_turn_runs_skill_view_and_refuses_the_rest() {
        let root = skills_dir();
        let harness = Harness::load(&root).unwrap();
        let body = fs::read_to_string(root.join("pre-visit-preparation/SKILL.md")).unwrap();
        let bad_bytes = 1;
        let loader_reason = harness
            .skill_view("pre-visit-preparation", None, bad_bytes)
            .expect_err("bytes outside 4096..=65536");

        let looked_up = harness.admit(&ModelMessage {
            text: "Reading the pre-visit skill.".to_owned(),
            calls: vec![
                ToolCall {
                    name: "skill_view".to_owned(),
                    arguments: SkillViewArguments {
                        name: "pre-visit-preparation".to_owned(),
                        file_path: None,
                        max_result_bytes: 65536,
                    },
                },
                ToolCall {
                    name: "not_a_tool".to_owned(),
                    arguments: SkillViewArguments {
                        name: "pre-visit-preparation".to_owned(),
                        file_path: None,
                        max_result_bytes: 65536,
                    },
                },
                ToolCall {
                    name: "skill_view".to_owned(),
                    arguments: SkillViewArguments {
                        name: "pre-visit-preparation".to_owned(),
                        file_path: None,
                        max_result_bytes: bad_bytes,
                    },
                },
            ],
        });

        assert_eq!(looked_up.text, "Reading the pre-visit skill.");
        assert_eq!(looked_up.calls.len(), 3);
        match &looked_up.calls[0] {
            AdmittedCall::SkillView(SkillView::Available { content, .. }) => {
                assert_eq!(content, &body);
            }
            other => panic!("expected the skill body, got {other:?}"),
        }
        match &looked_up.calls[1] {
            AdmittedCall::Refused { name, reason } => {
                assert_eq!(name, "not_a_tool");
                assert_eq!(reason, "unknown tool not_a_tool");
            }
            other => panic!("expected an unknown tool to be refused, got {other:?}"),
        }
        match &looked_up.calls[2] {
            AdmittedCall::Refused { name, reason } => {
                assert_eq!(name, "skill_view");
                assert_eq!(reason, &loader_reason);
            }
            other => panic!("expected a bad skill_view to be refused, got {other:?}"),
        }

        let finished = harness.admit(&ModelMessage {
            text: "Nothing else to look up.".to_owned(),
            calls: Vec::new(),
        });
        assert_eq!(finished.text, "Nothing else to look up.");
        assert!(finished.calls.is_empty());
    }
}

//! Loads `skills/` once and answers `skill_view`.
//!
//! This is the Rust caller for the skill catalog. It does not journal, run a
//! model, or read patient evidence. A skill never authorizes a tool or a
//! patient read. Book: `book/src/personal-agent-program.md`.
#![deny(missing_docs)]

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
}

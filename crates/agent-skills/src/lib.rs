//! Loads the clinical skill catalog and answers `skill_view`.
//!
//! The markdown lives in `skills/` at the workspace root. This crate reads that
//! directory once: one folder per skill, `SKILL.md` with a YAML mapping, no
//! symlinks, and a sha256 identity of the file bytes. A skill never authorizes
//! a tool or a patient read. The harness in `crates/harness` is the caller.
//! Book: `book/src/personal-agent-program.md`.
#![deny(missing_docs)]

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
/// The `SkillResource` value exchanged by this crate.
pub struct SkillResource {
    /// The `file_path` value.
    pub file_path: String,
    /// The `version_id` value.
    pub version_id: String,
    /// The `byte_length` value.
    pub byte_length: usize,
    /// The `content` value.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The `AgentSkill` value exchanged by this crate.
pub struct AgentSkill {
    /// The `name` value.
    pub name: String,
    /// The `description` value.
    pub description: String,
    /// The `file_path` value.
    pub file_path: String,
    /// The `version_id` value.
    pub version_id: String,
    /// The `byte_length` value.
    pub byte_length: usize,
    /// The `content` value.
    pub content: String,
    /// The `resources` value.
    pub resources: Vec<SkillResource>,
}

/// Loads each skill directory under `root`.
pub fn load_agent_skills(root: &Path) -> Result<Vec<AgentSkill>, String> {
    let root = fs::canonicalize(root).map_err(|error| error.to_string())?;
    let mut skills = Vec::new();
    let mut names = Vec::new();
    let entries = fs::read_dir(&root).map_err(|error| error.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let file_type = entry.file_type().map_err(|error| error.to_string())?;
        if file_type.is_symlink() {
            return Err(format!(
                "Packaged skill entry {name} cannot be a symbolic link"
            ));
        }
        if !file_type.is_dir() {
            continue;
        }
        let skill = load_skill(&root, &name)?;
        if names.iter().any(|existing| existing == &skill.name) {
            return Err(format!("Duplicate packaged skill name {}", skill.name));
        }
        names.push(skill.name.clone());
        skills.push(skill);
    }
    if skills.is_empty() {
        return Err("No packaged skills were found".to_owned());
    }
    skills.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(skills)
}

/// Publishes the compact catalog. Resource metadata only; no file bodies.
pub fn catalog_json(skills: &[AgentSkill]) -> String {
    let catalog = Catalog {
        kind: "approved_skill_catalog",
        skills: skills.iter().map(SkillMetadata::from).collect(),
    };
    serde_json::to_string(&catalog).expect("skill catalog is json")
}

/// Reads one skill body or one registered resource.
///
/// `file_path` is null for the skill body. `max_result_bytes` is 4096 through
/// 65536. A larger JSON result is `over_budget` and carries no partial content.
pub fn view_skill(
    skills: &[AgentSkill],
    name: &str,
    file_path: Option<&str>,
    max_result_bytes: u32,
) -> Result<SkillView, String> {
    if !(4096..=65536).contains(&max_result_bytes) {
        return Err(format!(
            "max_result_bytes {max_result_bytes} is outside 4096..=65536"
        ));
    }
    let Some(skill) = skills.iter().find(|skill| skill.name == name) else {
        return Ok(unavailable(name, file_path));
    };
    let resource = match file_path {
        None => Some(SkillResource {
            file_path: skill.file_path.clone(),
            version_id: skill.version_id.clone(),
            byte_length: skill.byte_length,
            content: skill.content.clone(),
        }),
        Some(path) if is_registered_path(path) => skill
            .resources
            .iter()
            .find(|candidate| candidate.file_path == path)
            .cloned(),
        Some(_) => None,
    };
    let Some(resource) = resource else {
        return Ok(unavailable(name, file_path));
    };
    let encoded_len = {
        let available = AvailableView {
            status: "available",
            name: &skill.name,
            file_path,
            version_id: &resource.version_id,
            byte_length: resource.byte_length,
            content: &resource.content,
            resources: if file_path.is_none() {
                Some(skill.resources.iter().map(ResourceMetadata::from).collect())
            } else {
                None
            },
        };
        serde_json::to_vec(&available)
            .map_err(|error| error.to_string())?
            .len()
    };
    if encoded_len <= usize::try_from(max_result_bytes).unwrap_or(usize::MAX) {
        Ok(SkillView::Available {
            name: skill.name.clone(),
            file_path: file_path.map(str::to_owned),
            version_id: resource.version_id,
            byte_length: resource.byte_length,
            content: resource.content,
            resources: file_path.is_none().then(|| {
                skill
                    .resources
                    .iter()
                    .map(|item| OwnedResourceMetadata {
                        file_path: item.file_path.clone(),
                        version_id: item.version_id.clone(),
                        byte_length: item.byte_length,
                    })
                    .collect()
            }),
        })
    } else {
        Ok(SkillView::OverBudget {
            name: skill.name.clone(),
            file_path: file_path.map(str::to_owned),
            version_id: resource.version_id,
            byte_length: resource.byte_length,
            required_bytes: encoded_len,
        })
    }
}

fn unavailable(name: &str, file_path: Option<&str>) -> SkillView {
    SkillView::Unavailable {
        name: name.to_owned(),
        file_path: file_path.map(str::to_owned),
    }
}

fn is_registered_path(path: &str) -> bool {
    !path.is_empty()
        && path.len() <= 500
        && !path.starts_with('/')
        && !path.contains('\\')
        && path
            .split('/')
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

/// One `skill_view` outcome. `OverBudget` has no content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillView {
    /// The whole body, plus resource metadata when `file_path` was null.
    Available {
        /// Skill name.
        name: String,
        /// The requested path. Null is the skill body.
        file_path: Option<String>,
        /// Sha256 identity of the returned file.
        version_id: String,
        /// File byte length.
        byte_length: usize,
        /// The whole file. Never a prefix.
        content: String,
        /// Present only for the skill body.
        resources: Option<Vec<OwnedResourceMetadata>>,
    },
    /// Unknown skill or unregistered path.
    Unavailable {
        /// Requested skill name.
        name: String,
        /// Requested path.
        file_path: Option<String>,
    },
    /// The available JSON is larger than `max_result_bytes`.
    OverBudget {
        /// Requested skill name.
        name: String,
        /// Requested path.
        file_path: Option<String>,
        /// Sha256 identity of the file that did not fit.
        version_id: String,
        /// File byte length.
        byte_length: usize,
        /// Bytes the available JSON would have used.
        required_bytes: usize,
    },
}

/// Resource identity published with a skill body. No file content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedResourceMetadata {
    /// Relative path inside the skill folder.
    pub file_path: String,
    /// Sha256 identity of that file.
    pub version_id: String,
    /// File byte length.
    pub byte_length: usize,
}

#[derive(Serialize)]
struct Catalog<'a> {
    kind: &'a str,
    skills: Vec<SkillMetadata<'a>>,
}

#[derive(Serialize)]
struct SkillMetadata<'a> {
    name: &'a str,
    description: &'a str,
    version_id: &'a str,
    byte_length: usize,
    resources: Vec<ResourceMetadata<'a>>,
}

#[derive(Serialize)]
struct ResourceMetadata<'a> {
    file_path: &'a str,
    version_id: &'a str,
    byte_length: usize,
}

#[derive(Serialize)]
struct AvailableView<'a> {
    status: &'a str,
    name: &'a str,
    file_path: Option<&'a str>,
    version_id: &'a str,
    byte_length: usize,
    content: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<ResourceMetadata<'a>>>,
}

impl<'a> From<&'a AgentSkill> for SkillMetadata<'a> {
    fn from(skill: &'a AgentSkill) -> Self {
        Self {
            name: &skill.name,
            description: &skill.description,
            version_id: &skill.version_id,
            byte_length: skill.byte_length,
            resources: skill.resources.iter().map(ResourceMetadata::from).collect(),
        }
    }
}

impl<'a> From<&'a SkillResource> for ResourceMetadata<'a> {
    fn from(resource: &'a SkillResource) -> Self {
        Self {
            file_path: &resource.file_path,
            version_id: &resource.version_id,
            byte_length: resource.byte_length,
        }
    }
}

fn load_skill(root: &Path, folder: &str) -> Result<AgentSkill, String> {
    let folder_path = resolve_inside(root, folder)?;
    let meta = fs::symlink_metadata(&folder_path).map_err(|error| error.to_string())?;
    if meta.file_type().is_symlink() || !meta.is_dir() {
        return Err(format!("Packaged skill {folder} is not a directory"));
    }
    let main = load_resource(&folder_path, "SKILL.md")?;
    let (yaml, body) = split_frontmatter(&main.content)
        .ok_or_else(|| format!("Packaged skill {folder} has no YAML frontmatter"))?;
    if body.trim().is_empty() {
        return Err(format!("Packaged skill {folder} has no instructions"));
    }
    let fields = parse_frontmatter(yaml).map_err(|error| {
        format!("Packaged skill {folder} has invalid YAML frontmatter: {error}")
    })?;
    let name = yaml_string(&fields, "name")
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("Packaged skill {folder} has no name"))?;
    if !valid_skill_name(&name) {
        return Err(format!(
            "Packaged skill {folder} has an invalid standard skill name"
        ));
    }
    if name != folder {
        return Err(format!(
            "Packaged skill folder and name must both be {folder}"
        ));
    }
    let description = yaml_string(&fields, "description")
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("Packaged skill {folder} has no description"))?;
    if description.len() > 1024 {
        return Err(format!(
            "Packaged skill {folder} description exceeds 1024 characters"
        ));
    }
    let description = description.trim().to_owned();
    Ok(AgentSkill {
        name,
        description,
        file_path: main.file_path,
        version_id: main.version_id,
        byte_length: main.byte_length,
        content: main.content,
        resources: collect_resources(&folder_path)?,
    })
}

fn collect_resources(folder: &Path) -> Result<Vec<SkillResource>, String> {
    let mut resources = Vec::new();
    visit(folder, folder, &mut resources)?;
    resources.sort_by(|left, right| left.file_path.cmp(&right.file_path));
    Ok(resources)
}

fn visit(
    folder: &Path,
    directory: &Path,
    resources: &mut Vec<SkillResource>,
) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let file_type = entry.file_type().map_err(|error| error.to_string())?;
        let relative = directory
            .strip_prefix(folder)
            .unwrap_or(Path::new(""))
            .join(&name);
        let relative = relative.to_string_lossy().replace('\\', "/");
        if file_type.is_symlink() {
            return Err(format!(
                "Packaged skill resource {relative} cannot be a symbolic link"
            ));
        }
        if file_type.is_dir() {
            visit(folder, &entry.path(), resources)?;
        } else if file_type.is_file() && relative != "SKILL.md" {
            resources.push(load_resource(folder, &relative)?);
        } else if !file_type.is_file() {
            return Err(format!(
                "Packaged skill resource {relative} is not a regular file"
            ));
        }
    }
    Ok(())
}

fn load_resource(folder: &Path, file_path: &str) -> Result<SkillResource, String> {
    let absolute = resolve_inside(folder, file_path)?;
    let meta = fs::symlink_metadata(&absolute).map_err(|error| error.to_string())?;
    if meta.file_type().is_symlink() || !meta.is_file() {
        return Err(format!(
            "Packaged skill resource {file_path} is not a regular file"
        ));
    }
    let bytes = fs::read(&absolute).map_err(|error| error.to_string())?;
    let content = String::from_utf8(bytes.clone())
        .map_err(|_| format!("Packaged skill resource {file_path} is not UTF-8 text"))?;
    Ok(SkillResource {
        file_path: file_path.to_owned(),
        version_id: version_id(&bytes),
        byte_length: bytes.len(),
        content,
    })
}

/// Returns the sha256 identity of `bytes`.
pub fn version_id(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity("sha256:".len() + digest.len() * 2);
    out.push_str("sha256:");
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    let rest = content
        .strip_prefix("---\n")
        .or_else(|| content.strip_prefix("---\r\n"))?;
    let (yaml, after) = rest.split_once("\n---")?;
    let body = after
        .strip_prefix('\n')
        .or_else(|| after.strip_prefix("\r\n"))
        .unwrap_or(after);
    Some((yaml, body))
}

fn parse_frontmatter(yaml: &str) -> Result<serde_yaml::Mapping, String> {
    let value: serde_yaml::Value = serde_yaml::from_str(yaml).map_err(|error| error.to_string())?;
    match value {
        serde_yaml::Value::Mapping(mapping) => Ok(mapping),
        _ => Err("frontmatter must be a mapping".to_owned()),
    }
}

fn yaml_string(fields: &serde_yaml::Mapping, key: &str) -> Option<String> {
    fields
        .get(serde_yaml::Value::String(key.to_owned()))
        .and_then(serde_yaml::Value::as_str)
        .map(str::to_owned)
}

fn valid_skill_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if name.len() > 64 || !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return false;
    }
    let mut dash = false;
    for character in chars {
        if character.is_ascii_lowercase() || character.is_ascii_digit() {
            dash = false;
        } else if character == '-' && !dash {
            dash = true;
        } else {
            return false;
        }
    }
    !name.ends_with('-')
}

fn resolve_inside(root: &Path, child: &str) -> Result<PathBuf, String> {
    if child.is_empty()
        || child.starts_with('/')
        || child.contains('\\')
        || child.split('/').any(|part| part == "..")
    {
        return Err(format!("Invalid packaged skill path {child}"));
    }
    Ok(root.join(child))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills")
    }

    #[test]
    fn catalog_matches_the_skill_directory_bytes() {
        let root = catalog_root();
        let skills = load_agent_skills(&root).unwrap();
        let mut folders = fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| !name.starts_with('.'))
            .collect::<Vec<_>>();
        folders.sort();
        let names = skills
            .iter()
            .map(|skill| skill.name.clone())
            .collect::<Vec<_>>();
        assert_eq!(names, folders);
        let catalog = catalog_json(&skills);
        assert!(catalog.contains("\"kind\":\"approved_skill_catalog\""));
        assert!(!catalog.contains("\"content\""));
        for skill in skills {
            let bytes = fs::read(root.join(&skill.name).join("SKILL.md")).unwrap();
            assert_eq!(skill.version_id, version_id(&bytes));
            assert_eq!(skill.byte_length, bytes.len());
            assert!(!skill.description.is_empty());
        }
    }

    #[test]
    fn frontmatter_keeps_a_colon_inside_a_yaml_string() {
        let root = std::env::temp_dir().join(format!("coda-skills-{}", std::process::id()));
        let folder = root.join("visit-note");
        fs::create_dir_all(&folder).unwrap();
        fs::write(
            folder.join("SKILL.md"),
            "---\nname: visit-note\ndescription: \"Use when: the note has a colon\"\n---\n\nKeep the chart.\n",
        )
        .unwrap();
        let skills = load_agent_skills(&root).unwrap();
        assert_eq!(skills[0].description, "Use when: the note has a colon");
        fs::write(
            folder.join("SKILL.md"),
            "---\n- not-a-mapping\n---\n\nBody.\n",
        )
        .unwrap();
        assert!(load_agent_skills(&root).is_err());
        let _ = fs::remove_dir_all(root);
    }
}

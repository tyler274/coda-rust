//! Loads the clinical skill catalog from its owner.
//!
//! The markdown stays in `coda-agents/skills`. This crate reads that directory:
//! one folder per skill, `SKILL.md` with YAML frontmatter, no symlinks, and a
//! sha256 identity of the file bytes. It does not copy the skills.
//! Book: `book/src/personal-agent-program.md`.
#![deny(missing_docs)]

use std::fs;
use std::path::{Path, PathBuf};

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
    let fields = parse_frontmatter(yaml)?;
    let name = fields
        .iter()
        .find(|(key, _)| key == "name")
        .map(|(_, value)| value.clone())
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
    let description = fields
        .iter()
        .find(|(key, _)| key == "description")
        .map(|(_, value)| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("Packaged skill {folder} has no description"))?;
    if description.len() > 1024 {
        return Err(format!(
            "Packaged skill {folder} description exceeds 1024 characters"
        ));
    }
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
    format!("sha256:{digest:x}")
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

fn parse_frontmatter(yaml: &str) -> Result<Vec<(String, String)>, String> {
    let mut fields = Vec::new();
    for line in yaml.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            return Err(format!(
                "Packaged skill frontmatter line is not a field: {line}"
            ));
        };
        fields.push((key.trim().to_owned(), value.trim().to_owned()));
    }
    Ok(fields)
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
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../submodules/coda-agents/skills")
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
        for skill in skills {
            let bytes = fs::read(root.join(&skill.name).join("SKILL.md")).unwrap();
            assert_eq!(skill.version_id, version_id(&bytes));
            assert_eq!(skill.byte_length, bytes.len());
            assert!(!skill.description.is_empty());
        }
    }
}

//! Regenerate `crates/duet-wire` with OpenAPI Generator, then apply the
//! post-edits the shell used to make by hand.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::failure::Failure;

const PROPERTIES: &str = "packageName=duet_wire,packageVersion=0.1.0,reqwestDefaultFeatures=rustls,supportAsync=true,hideGenerationTimestamp=true";

const NULL_MODEL: &str = "\n#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]\npub struct Null;\n";

pub fn from_args(args: &[String]) -> Result<(), Failure> {
    let root = crate::workspace_root();
    let committed = root.join("crates/duet-wire");
    match args {
        [] => regenerate(&root, &committed),
        [flag] if flag == "--check" => check_regenerated(&root, &committed),
        _ => Err(Failure::new(
            2,
            "usage: workspace-tools generate-duet-wire [--check]",
        )),
    }
}

fn regenerate(root: &Path, out: &Path) -> Result<(), Failure> {
    let work = scratch_dir("duet-wire-spec")?;
    let result = (|| {
        let spec = compose_spec(root, &work)?;
        generate_tree(root, &spec, out)
    })();
    let _ = fs::remove_dir_all(&work);
    result
}

fn check_regenerated(root: &Path, committed: &Path) -> Result<(), Failure> {
    let work = scratch_dir("duet-wire-check")?;
    let result = (|| {
        let spec = compose_spec(root, &work)?;
        let fresh = work.join("fresh");
        fs::create_dir_all(&fresh).map_err(|error| {
            Failure::new(1, format!("could not create {}: {error}", fresh.display()))
        })?;
        generate_tree(root, &spec, &fresh)?;
        diff_wire(committed, &fresh)
    })();
    let _ = fs::remove_dir_all(&work);
    result
}

fn generate_tree(root: &Path, spec: &Path, out: &Path) -> Result<(), Failure> {
    run_generator(spec, out)?;
    post_edit(out)?;
    finish_generated(root, spec, out)
}

fn compose_spec(root: &Path, work: &Path) -> Result<PathBuf, Failure> {
    let spec = work.join("openapi.json");
    let script = root.join("scripts/duet-wire/compose.py");
    let additions = root.join("crates/duet-api/openapi/additions.json");
    run_program(
        "python3",
        &[
            script.as_os_str(),
            reference_openapi(root).as_os_str(),
            additions.as_os_str(),
            spec.as_os_str(),
        ],
    )?;
    Ok(spec)
}

fn reference_openapi(root: &Path) -> PathBuf {
    match env::var("DUET_REFERENCE_OPENAPI") {
        Ok(value) if !value.is_empty() => PathBuf::from(value),
        _ => root.join("submodules/coda-app/docs/openapi.json"),
    }
}

fn finish_generated(root: &Path, spec: &Path, out: &Path) -> Result<(), Failure> {
    cargo_fmt(out)?;
    let script = root.join("scripts/duet-wire/finalize.py");
    run_program(
        "python3",
        &[script.as_os_str(), spec.as_os_str(), out.as_os_str()],
    )?;
    cargo_fmt(out)
}

fn cargo_fmt(out: &Path) -> Result<(), Failure> {
    run_program(
        "cargo",
        &[
            "fmt".as_ref(),
            "--manifest-path".as_ref(),
            out.join("Cargo.toml").as_os_str(),
        ],
    )
}

fn scratch_dir(label: &str) -> Result<PathBuf, Failure> {
    let path = env::temp_dir().join(format!(
        "{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&path).map_err(|error| {
        Failure::new(1, format!("could not create {}: {error}", path.display()))
    })?;
    Ok(path)
}

fn run_program(program: &str, args: &[&std::ffi::OsStr]) -> Result<(), Failure> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|error| Failure::new(1, format!("could not run {program}: {error}")))?;
    if status.success() {
        Ok(())
    } else {
        Err(Failure::new(1, format!("{program} failed")))
    }
}

fn run_generator(spec: &Path, out: &Path) -> Result<(), Failure> {
    let status = Command::new("openapi-generator-cli")
        .arg("generate")
        .arg("-i")
        .arg(spec)
        .arg("-g")
        .arg("rust")
        .arg("-o")
        .arg(out)
        .arg("--additional-properties")
        .arg(PROPERTIES)
        .status()
        .map_err(|error| {
            Failure::new(1, format!("could not run openapi-generator-cli: {error}"))
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(Failure::new(1, "openapi-generator-cli failed"))
    }
}

pub fn post_edit(out: &Path) -> Result<(), Failure> {
    let cargo_path = out.join("Cargo.toml");
    let cargo = fs::read_to_string(&cargo_path).map_err(|error| {
        Failure::new(
            1,
            format!("could not read {}: {error}", cargo_path.display()),
        )
    })?;
    let cargo = replace_prefix(
        &cargo,
        "license = \"Unlicense\"",
        "license = \"UNLICENSED\"",
    );
    let cargo = replace_prefix(&cargo, "edition = \"2021\"", "edition = \"2024\"");
    fs::write(&cargo_path, cargo).map_err(|error| {
        Failure::new(
            1,
            format!("could not write {}: {error}", cargo_path.display()),
        )
    })?;

    let models = out.join("src/models/mod.rs");
    let mut body = fs::read_to_string(&models).map_err(|error| {
        Failure::new(1, format!("could not read {}: {error}", models.display()))
    })?;
    body.push_str(NULL_MODEL);
    fs::write(&models, body).map_err(|error| {
        Failure::new(1, format!("could not write {}: {error}", models.display()))
    })?;

    remove_generated(out.join(".openapi-generator"))?;
    remove_generated(out.join("git_push.sh"))?;
    // Keep README.md: generated API/model pages navigate back to this index.
    Ok(())
}

fn replace_prefix(text: &str, from: &str, to: &str) -> String {
    let mut out = String::new();
    for line in text.split_inclusive('\n') {
        let (body, newline) = line
            .strip_suffix('\n')
            .map_or((line, ""), |body| (body, "\n"));
        if let Some(rest) = body.strip_prefix(from) {
            out.push_str(to);
            out.push_str(rest);
            out.push_str(newline);
        } else {
            out.push_str(line);
        }
    }
    out
}

fn remove_generated(path: PathBuf) -> Result<(), Failure> {
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(Failure::new(
                1,
                format!("could not inspect {}: {error}", path.display()),
            ));
        }
    };
    let result = if metadata.is_dir() {
        fs::remove_dir_all(&path)
    } else {
        fs::remove_file(&path)
    };
    result.map_err(|error| Failure::new(1, format!("could not remove {}: {error}", path.display())))
}

#[cfg(test)]
pub fn check_wire(
    committed: &Path,
    generate: impl FnOnce(&Path) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let temp = std::env::temp_dir().join(format!(
        "duet-wire-check-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&temp).map_err(|error| {
        Failure::new(1, format!("could not create {}: {error}", temp.display()))
    })?;
    let fresh = temp.join("fresh");
    fs::create_dir_all(&fresh).map_err(|error| {
        Failure::new(1, format!("could not create {}: {error}", fresh.display()))
    })?;
    let generated = generate(&fresh);
    let edited = generated.and_then(|()| post_edit(&fresh));
    let compared = edited.and_then(|()| diff_wire(committed, &fresh));
    let _ = fs::remove_dir_all(&temp);
    compared
}

fn diff_wire(committed: &Path, fresh: &Path) -> Result<(), Failure> {
    let mut drift = String::new();
    for directory in ["src", "docs"] {
        drift.push_str(&diff_dir(
            &committed.join(directory),
            &fresh.join(directory),
            directory,
        )?);
    }
    for file in ["Cargo.toml", "README.md"] {
        drift.push_str(&diff_file(&committed.join(file), &fresh.join(file), file)?);
    }
    if drift.is_empty() {
        Ok(())
    } else {
        Err(Failure::new(
            1,
            format!("duet-wire differs from the generator output:\n{drift}"),
        ))
    }
}

fn diff_dir(left: &Path, right: &Path, label: &str) -> Result<String, Failure> {
    let mut left_files = files_under(left)?;
    let mut right_files = files_under(right)?;
    left_files.sort();
    right_files.sort();
    let mut drift = String::new();
    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left_files.len() || right_index < right_files.len() {
        match (left_files.get(left_index), right_files.get(right_index)) {
            (Some(left_rel), Some(right_rel)) if left_rel == right_rel => {
                drift.push_str(&diff_file(
                    &left.join(left_rel),
                    &right.join(right_rel),
                    &format!("{label}/{}", left_rel.display()),
                )?);
                left_index += 1;
                right_index += 1;
            }
            (Some(left_rel), Some(right_rel)) if left_rel < right_rel => {
                drift.push_str(&format!(
                    "{label}/{} only in the committed tree\n",
                    left_rel.display()
                ));
                left_index += 1;
            }
            (Some(left_rel), None) => {
                drift.push_str(&format!(
                    "{label}/{} only in the committed tree\n",
                    left_rel.display()
                ));
                left_index += 1;
            }
            (_, Some(right_rel)) => {
                drift.push_str(&format!(
                    "{label}/{} only in the generator output\n",
                    right_rel.display()
                ));
                right_index += 1;
            }
            (None, None) => break,
        }
    }
    Ok(drift)
}

fn files_under(root: &Path) -> Result<Vec<PathBuf>, Failure> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = fs::read_dir(&dir).map_err(|error| {
            Failure::new(1, format!("could not read {}: {error}", dir.display()))
        })?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                Failure::new(1, format!("could not read {}: {error}", dir.display()))
            })?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let relative = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
                files.push(relative);
            }
        }
    }
    Ok(files)
}

fn diff_file(left: &Path, right: &Path, label: &str) -> Result<String, Failure> {
    let left_bytes = read_optional(left)?;
    let right_bytes = read_optional(right)?;
    if left_bytes == right_bytes {
        Ok(String::new())
    } else {
        Ok(format!("{label} differs\n"))
    }
}

fn read_optional(path: &Path) -> Result<Option<Vec<u8>>, Failure> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(Failure::new(
            1,
            format!("could not read {}: {error}", path.display()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "wire-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|duration| duration.as_nanos())
                    .unwrap_or(0)
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn write_generator_output(out: &Path) {
        fs::create_dir_all(out.join("src/models")).unwrap();
        fs::create_dir_all(out.join("docs")).unwrap();
        fs::create_dir_all(out.join(".openapi-generator")).unwrap();
        fs::write(out.join("src/lib.rs"), "pub fn generated() {}\n").unwrap();
        fs::write(out.join("src/models/mod.rs"), "pub mod thing;\n").unwrap();
        fs::write(
            out.join("Cargo.toml"),
            "license = \"Unlicense\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            out.join("README.md"),
            "# Generated client\n\n[Thing](docs/Thing.md)\n",
        )
        .unwrap();
        fs::write(
            out.join("docs/Thing.md"),
            "# Thing\n\n[Back to README](../README.md)\n",
        )
        .unwrap();
        fs::write(out.join("git_push.sh"), "#!/bin/sh\n").unwrap();
        fs::write(out.join(".openapi-generator/FILES"), "x\n").unwrap();
    }

    #[test]
    fn post_edit_rewrites_the_generator_output() {
        let temp = TempDir::new("edit");
        let out = temp.0.join("fresh");
        write_generator_output(&out);
        post_edit(&out).unwrap();

        let cargo = fs::read_to_string(out.join("Cargo.toml")).unwrap();
        assert!(cargo.contains("license = \"UNLICENSED\""));
        assert!(cargo.contains("edition = \"2024\""));
        assert!(!cargo.contains("Unlicense"));
        assert!(!cargo.contains("edition = \"2021\""));

        let models = fs::read_to_string(out.join("src/models/mod.rs")).unwrap();
        assert!(models.contains("pub struct Null;"));
        assert!(models.contains("serde::Serialize"));
        let docs = fs::read_to_string(out.join("docs/Thing.md")).unwrap();
        let backlink = docs.split("](").nth(1).unwrap().split(')').next().unwrap();
        assert!(
            out.join("docs").join(backlink).is_file(),
            "generated documentation backlink must resolve"
        );
        assert!(!out.join("git_push.sh").exists());
        assert!(!out.join(".openapi-generator").exists());
    }

    #[test]
    fn check_fails_when_the_generated_sources_differ() {
        let temp = TempDir::new("check");
        let committed = temp.0.join("committed");
        write_generator_output(&committed);
        post_edit(&committed).unwrap();
        fs::write(committed.join("src/lib.rs"), "pub fn committed() {}\n").unwrap();

        let error = check_wire(&committed, |out| {
            write_generator_output(out);
            Ok(())
        })
        .unwrap_err();
        assert_eq!(error.code, 1);
        assert!(error.message.contains("src/lib.rs differs"));
    }

    #[test]
    fn check_detects_missing_or_changed_generated_documentation() {
        for path in ["README.md", "docs/Thing.md"] {
            for remove in [false, true] {
                let temp = TempDir::new("docs");
                let committed = temp.0.join("committed");
                write_generator_output(&committed);
                // Matching Rust output isolates the documentation check.
                post_edit(&committed).unwrap();
                check_wire(&committed, |out| {
                    write_generator_output(out);
                    Ok(())
                })
                .unwrap();
                if remove {
                    fs::remove_file(committed.join(path)).unwrap();
                } else {
                    fs::write(committed.join(path), "changed documentation\n").unwrap();
                }
                let result = check_wire(&committed, |out| {
                    write_generator_output(out);
                    Ok(())
                });
                let error = result.expect_err("generated documentation drift must fail --check");
                assert!(error.message.contains(path), "{}", error.message);
            }
        }
    }
}

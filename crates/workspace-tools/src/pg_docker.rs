//! Docker-backed PostgreSQL 16 binaries for hosts whose dev-shell socket
//! directory is not writable.
//!
//! `pg-docker link DEST` symlinks this executable as `initdb`, `pg_ctl`, and
//! `postgres`. Those names dispatch on argv0 into one helper container.

use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::Command;

use crate::failure::Failure;

pub const IMAGE: &str = "pgvector/pgvector:pg16@sha256:7b822b0aac60967beb1ea5e576b8602c94c300a157d187f385ae3e0da199b90a";
pub const HELPER: &str = "coda-test-pg16";

unsafe extern "C" {
    fn geteuid() -> u32;
    fn getegid() -> u32;
}

struct CommandOutput {
    status: i32,
    stdout: String,
    stderr: String,
}

trait Docker {
    fn run(&mut self, args: &[String]) -> Result<CommandOutput, Failure>;
    fn exec(&mut self, args: &[String]) -> Result<(), Failure>;
}

struct RealDocker;

impl Docker for RealDocker {
    fn run(&mut self, args: &[String]) -> Result<CommandOutput, Failure> {
        let output = Command::new("docker")
            .args(args)
            .output()
            .map_err(|error| Failure::new(1, format!("could not run docker: {error}")))?;
        Ok(CommandOutput {
            status: output.status.code().unwrap_or(1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    fn exec(&mut self, args: &[String]) -> Result<(), Failure> {
        use std::os::unix::process::CommandExt;
        let error = Command::new("docker").args(args).exec();
        Err(Failure::new(1, format!("could not exec docker: {error}")))
    }
}

pub fn from_args(args: &[String]) -> Result<(), Failure> {
    match args {
        [tool, rest @ ..] if is_tool(tool) => launch(tool, rest),
        [command, dest] if command == "link" => link(Path::new(dest)),
        _ => Err(Failure::new(
            2,
            "usage: workspace-tools pg-docker initdb|pg_ctl|postgres [args...] | link DEST",
        )),
    }
}

pub fn tool_from_argv0(argv0: &str) -> Option<&'static str> {
    match Path::new(argv0).file_name().and_then(|name| name.to_str()) {
        Some("initdb") => Some("initdb"),
        Some("pg_ctl") => Some("pg_ctl"),
        Some("postgres") => Some("postgres"),
        _ => None,
    }
}

fn is_tool(name: &str) -> bool {
    matches!(name, "initdb" | "pg_ctl" | "postgres")
}

pub fn launch(tool: &str, args: &[String]) -> Result<(), Failure> {
    let tmpdir = env::var("TMPDIR").ok();
    let uid = unsafe { geteuid() };
    let gid = unsafe { getegid() };
    run_helper(&mut RealDocker, tool, args, tmpdir.as_deref(), uid, gid)
}

pub fn link(dest: &Path) -> Result<(), Failure> {
    fs::create_dir_all(dest).map_err(|error| {
        Failure::new(1, format!("could not create {}: {error}", dest.display()))
    })?;
    let exe = env::current_exe().map_err(|error| {
        Failure::new(
            1,
            format!("could not resolve the current executable: {error}"),
        )
    })?;
    for name in ["initdb", "pg_ctl", "postgres"] {
        let path = dest.join(name);
        if path.symlink_metadata().is_ok() {
            fs::remove_file(&path).map_err(|error| {
                Failure::new(1, format!("could not replace {}: {error}", path.display()))
            })?;
        }
        symlink(&exe, &path).map_err(|error| {
            Failure::new(1, format!("could not link {}: {error}", path.display()))
        })?;
    }
    Ok(())
}

fn run_helper(
    docker: &mut impl Docker,
    tool: &str,
    args: &[String],
    tmpdir: Option<&str>,
    uid: u32,
    gid: u32,
) -> Result<(), Failure> {
    let inspect = docker.run(&owned(&["inspect", "-f", "{{.State.Running}}", HELPER]))?;
    let running = inspect.status == 0 && inspect.stdout.trim() == "true";
    if !running {
        let image = docker.run(&owned(&["image", "inspect", IMAGE]))?;
        if image.status != 0 {
            must(docker, &owned(&["pull", IMAGE]))?;
        }
        let _ = docker.run(&owned(&["rm", "-f", HELPER]));
        let mut run = owned(&[
            "run",
            "-d",
            "--name",
            HELPER,
            "--pid",
            "host",
            "--network",
            "host",
            "-v",
            "/tmp:/tmp",
        ]);
        if let Some(mount) = tmpdir_mount(tmpdir) {
            run.push("-v".to_owned());
            run.push(mount);
        }
        run.push(IMAGE.to_owned());
        run.push("sleep".to_owned());
        run.push("infinity".to_owned());
        must(docker, &run)?;
    }
    let mut exec = vec![
        "exec".to_owned(),
        "-u".to_owned(),
        format!("{uid}:{gid}"),
        HELPER.to_owned(),
        format!("/usr/lib/postgresql/16/bin/{tool}"),
    ];
    exec.extend(args.iter().cloned());
    docker.exec(&exec)
}

fn tmpdir_mount(tmpdir: Option<&str>) -> Option<String> {
    let tmpdir = tmpdir?;
    if tmpdir.is_empty() || tmpdir == "/tmp" {
        None
    } else {
        Some(format!("{tmpdir}:{tmpdir}"))
    }
}

fn must(docker: &mut impl Docker, args: &[String]) -> Result<(), Failure> {
    let output = docker.run(args)?;
    if output.status == 0 {
        Ok(())
    } else {
        Err(Failure::new(
            1,
            format!("docker {} failed\n{}", args.join(" "), output.stderr),
        ))
    }
}

fn owned(parts: &[&str]) -> Vec<String> {
    parts.iter().copied().map(str::to_owned).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeDocker {
        calls: Vec<Vec<String>>,
        running: bool,
        image_present: bool,
    }

    impl Docker for FakeDocker {
        fn run(&mut self, args: &[String]) -> Result<CommandOutput, Failure> {
            self.calls.push(args.to_vec());
            if args.first().is_some_and(|arg| arg == "inspect") {
                return Ok(if self.running {
                    CommandOutput {
                        status: 0,
                        stdout: "true\n".to_owned(),
                        stderr: String::new(),
                    }
                } else {
                    CommandOutput {
                        status: 1,
                        stdout: String::new(),
                        stderr: "missing".to_owned(),
                    }
                });
            }
            if args.len() >= 2 && args[0] == "image" && args[1] == "inspect" && !self.image_present
            {
                return Ok(CommandOutput {
                    status: 1,
                    stdout: String::new(),
                    stderr: "missing image".to_owned(),
                });
            }
            Ok(CommandOutput {
                status: 0,
                stdout: String::new(),
                stderr: String::new(),
            })
        }

        fn exec(&mut self, args: &[String]) -> Result<(), Failure> {
            self.calls.push(args.to_vec());
            Ok(())
        }
    }

    #[test]
    fn a_stopped_helper_is_started_then_execs_the_tool() {
        let mut docker = FakeDocker {
            calls: Vec::new(),
            running: false,
            image_present: false,
        };
        run_helper(
            &mut docker,
            "pg_ctl",
            &owned(&["-D", "/tmp/cluster", "status"]),
            Some("/var/tmp/coda-tests"),
            1000,
            1000,
        )
        .unwrap();

        let image = "pgvector/pgvector:pg16@sha256:7b822b0aac60967beb1ea5e576b8602c94c300a157d187f385ae3e0da199b90a";
        assert_eq!(
            docker.calls,
            vec![
                owned(&["inspect", "-f", "{{.State.Running}}", "coda-test-pg16"]),
                owned(&["image", "inspect", image]),
                owned(&["pull", image]),
                owned(&["rm", "-f", "coda-test-pg16"]),
                owned(&[
                    "run",
                    "-d",
                    "--name",
                    "coda-test-pg16",
                    "--pid",
                    "host",
                    "--network",
                    "host",
                    "-v",
                    "/tmp:/tmp",
                    "-v",
                    "/var/tmp/coda-tests:/var/tmp/coda-tests",
                    image,
                    "sleep",
                    "infinity",
                ]),
                owned(&[
                    "exec",
                    "-u",
                    "1000:1000",
                    "coda-test-pg16",
                    "/usr/lib/postgresql/16/bin/pg_ctl",
                    "-D",
                    "/tmp/cluster",
                    "status",
                ]),
            ]
        );
    }

    #[test]
    fn argv0_selects_the_postgres_tool() {
        assert_eq!(tool_from_argv0("/usr/local/bin/pg_ctl"), Some("pg_ctl"));
        assert_eq!(tool_from_argv0("initdb"), Some("initdb"));
        assert_eq!(tool_from_argv0("postgres"), Some("postgres"));
        assert_eq!(tool_from_argv0("workspace-tools"), None);
    }

    #[test]
    fn link_installs_the_three_tool_names() {
        let dest = std::env::temp_dir().join(format!(
            "pg-docker-link-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ));
        fs::create_dir_all(&dest).unwrap();
        let result = link(&dest);
        let linked = ["initdb", "pg_ctl", "postgres"].map(|name| {
            dest.join(name)
                .symlink_metadata()
                .map(|metadata| metadata.file_type().is_symlink())
                .unwrap_or(false)
        });
        let _ = fs::remove_dir_all(&dest);
        result.unwrap();
        assert_eq!(linked, [true, true, true]);
    }
}

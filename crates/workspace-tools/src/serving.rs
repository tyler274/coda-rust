//! Boot a clinical-evidence serving replica: mount its disk, wait for the GPU,
//! then `docker run` the pinned image.
//!
//! Command execution is injected so tests never mount a disk or start Docker.

use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::failure::Failure;

const DISK: &str = "/dev/disk/by-id/google-local-nvme-ssd-0";
const CONTAINER: &str = "coda-clinical-evidence";

trait Host {
    fn is_mounted(&mut self, path: &Path) -> Result<bool, Failure>;
    fn run(&mut self, program: &str, args: &[String]) -> Result<i32, Failure>;
    fn run_quiet(&mut self, program: &str, args: &[String]) -> Result<i32, Failure> {
        self.run(program, args)
    }
    fn has_tool(&mut self, name: &str) -> bool;
    fn sleep(&mut self, duration: Duration);
}

struct SystemHost;

impl Host for SystemHost {
    fn is_mounted(&mut self, path: &Path) -> Result<bool, Failure> {
        let status = Command::new("mountpoint")
            .arg("-q")
            .arg(path)
            .status()
            .map_err(|error| Failure::new(1, format!("could not run mountpoint: {error}")))?;
        Ok(status.success())
    }

    fn run(&mut self, program: &str, args: &[String]) -> Result<i32, Failure> {
        spawn(program, args, false)
    }

    fn run_quiet(&mut self, program: &str, args: &[String]) -> Result<i32, Failure> {
        spawn(program, args, true)
    }

    fn has_tool(&mut self, name: &str) -> bool {
        let Some(path) = env::var_os("PATH") else {
            return false;
        };
        env::split_paths(&path).any(|dir| dir.join(name).is_file())
    }

    fn sleep(&mut self, duration: Duration) {
        std::thread::sleep(duration);
    }
}

fn spawn(program: &str, args: &[String], quiet: bool) -> Result<i32, Failure> {
    let mut command = Command::new(program);
    command.args(args);
    if quiet {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }
    let status = command
        .status()
        .map_err(|error| Failure::new(1, format!("could not run {program}: {error}")))?;
    Ok(status.code().unwrap_or(1))
}

pub fn from_args(args: &[String]) -> Result<(), Failure> {
    let options = parse(args)?;
    start(
        Path::new(&options.data_root),
        &options.image,
        Path::new(&options.env_file),
        &mut SystemHost,
    )
}

struct Options {
    data_root: String,
    image: String,
    env_file: String,
}

fn parse(args: &[String]) -> Result<Options, Failure> {
    let mut data_root = None;
    let mut image = None;
    let mut env_file = None;
    let mut args = args.iter();
    while let Some(flag) = args.next() {
        let Some(value) = args.next() else {
            return Err(usage());
        };
        match flag.as_str() {
            "--data-root" => data_root = Some(value.clone()),
            "--image" => image = Some(value.clone()),
            "--env-file" => env_file = Some(value.clone()),
            _ => return Err(usage()),
        }
    }
    match (data_root, image, env_file) {
        (Some(data_root), Some(image), Some(env_file)) => Ok(Options {
            data_root,
            image,
            env_file,
        }),
        _ => Err(usage()),
    }
}

fn usage() -> Failure {
    Failure::new(
        2,
        "usage: workspace-tools clinical-serving-startup --data-root PATH --image IMAGE --env-file PATH",
    )
}

fn start(
    data_root: &Path,
    image: &str,
    env_file: &Path,
    host: &mut impl Host,
) -> Result<(), Failure> {
    if !host.is_mounted(data_root)? {
        let blkid = host.run_quiet("blkid", &owned(&[DISK]))?;
        if blkid != 0 {
            must(
                host,
                "mkfs.ext4",
                &owned(&[
                    "-q",
                    "-F",
                    "-m",
                    "0",
                    "-E",
                    "lazy_itable_init=0,lazy_journal_init=0,discard",
                    DISK,
                ]),
            )?;
        }
        fs::create_dir_all(data_root).map_err(|error| {
            Failure::new(
                1,
                format!("could not create {}: {error}", data_root.display()),
            )
        })?;
        let mut mount_args = owned(&["-o", "discard,defaults,noatime", DISK]);
        mount_args.push(data_root.display().to_string());
        must(host, "mount", &mount_args)?;
    }
    fs::create_dir_all(data_root.join("runtime")).map_err(|error| {
        Failure::new(
            1,
            format!("could not create {}: {error}", data_root.display()),
        )
    })?;
    fs::create_dir_all(data_root.join("table")).map_err(|error| {
        Failure::new(
            1,
            format!("could not create {}: {error}", data_root.display()),
        )
    })?;

    loop {
        if host.run_quiet("nvidia-smi", &[])? == 0 {
            break;
        }
        log(host, "waiting for the GPU driver")?;
        host.sleep(Duration::from_secs(10));
    }
    for tool in ["docker", "nvidia-ctk"] {
        if !host.has_tool(tool) {
            let message = format!("missing {tool} on the boot image");
            log(host, &message)?;
            return Err(Failure::new(1, message));
        }
    }

    must(
        host,
        "gcloud",
        &owned(&[
            "auth",
            "configure-docker",
            "us-east1-docker.pkg.dev",
            "--quiet",
        ]),
    )?;
    must(host, "docker", &owned(&["pull", image]))?;
    let _ = host.run_quiet("docker", &owned(&["rm", "-f", CONTAINER]));
    let volume = format!("{}:{}", data_root.display(), data_root.display());
    must(
        host,
        "docker",
        &owned_with(
            &[
                "run",
                "--detach",
                "--name",
                CONTAINER,
                "--restart",
                "unless-stopped",
                "--gpus",
                "all",
                "--network",
                "host",
                "--shm-size",
                "2g",
                "--env-file",
            ],
            &[
                env_file.display().to_string(),
                "--volume".to_owned(),
                volume,
                "--log-driver".to_owned(),
                "gcplogs".to_owned(),
                image.to_owned(),
            ],
        ),
    )?;
    log(host, &format!("started {image}"))?;
    Ok(())
}

fn log(host: &mut impl Host, message: &str) -> Result<(), Failure> {
    let status = host.run("logger", &owned(&["-t", "coda-clinical-serving", message]))?;
    if status != 0 {
        return Err(Failure::new(1, format!("logger exited {status}")));
    }
    println!("coda-clinical-serving: {message}");
    Ok(())
}

fn must(host: &mut impl Host, program: &str, args: &[String]) -> Result<(), Failure> {
    let status = host.run(program, args)?;
    if status == 0 {
        Ok(())
    } else {
        Err(Failure::new(1, format!("{program} exited {status}")))
    }
}

fn owned(parts: &[&str]) -> Vec<String> {
    parts.iter().copied().map(str::to_owned).collect()
}

fn owned_with(prefix: &[&str], extra: &[String]) -> Vec<String> {
    let mut args = owned(prefix);
    args.extend(extra.iter().cloned());
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeHost {
        mounted: bool,
        calls: Vec<(String, Vec<String>)>,
    }

    impl Host for FakeHost {
        fn is_mounted(&mut self, _path: &Path) -> Result<bool, Failure> {
            Ok(self.mounted)
        }

        fn run(&mut self, program: &str, args: &[String]) -> Result<i32, Failure> {
            self.calls.push((program.to_owned(), args.to_vec()));
            Ok(0)
        }

        fn has_tool(&mut self, _name: &str) -> bool {
            true
        }

        fn sleep(&mut self, _duration: Duration) {}
    }

    #[test]
    fn a_mounted_data_root_skips_mkfs_and_runs_the_pinned_container() {
        let root = std::env::temp_dir().join(format!(
            "serving-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ));
        fs::create_dir_all(&root).unwrap();
        let image = "us-east1-docker.pkg.dev/coda/clinical:pinned";
        let env_file = Path::new("/etc/coda-clinical-serving.env");
        let mut host = FakeHost {
            mounted: true,
            calls: Vec::new(),
        };
        let started = start(&root, image, env_file, &mut host);
        let runtime = root.join("runtime").is_dir();
        let table = root.join("table").is_dir();
        let _ = fs::remove_dir_all(&root);
        started.unwrap();
        assert!(runtime);
        assert!(table);
        assert!(host.calls.iter().all(|(program, _)| program != "mkfs.ext4"));
        assert!(host.calls.iter().all(|(program, _)| program != "blkid"));
        assert!(host.calls.iter().all(|(program, _)| program != "mount"));

        let volume = format!("{}:{}", root.display(), root.display());
        let run = host
            .calls
            .iter()
            .find(|(program, args)| {
                program == "docker" && args.first().map(String::as_str) == Some("run")
            })
            .unwrap();
        assert_eq!(
            run.1,
            vec![
                "run".to_owned(),
                "--detach".to_owned(),
                "--name".to_owned(),
                "coda-clinical-evidence".to_owned(),
                "--restart".to_owned(),
                "unless-stopped".to_owned(),
                "--gpus".to_owned(),
                "all".to_owned(),
                "--network".to_owned(),
                "host".to_owned(),
                "--shm-size".to_owned(),
                "2g".to_owned(),
                "--env-file".to_owned(),
                "/etc/coda-clinical-serving.env".to_owned(),
                "--volume".to_owned(),
                volume,
                "--log-driver".to_owned(),
                "gcplogs".to_owned(),
                image.to_owned(),
            ]
        );
    }
}

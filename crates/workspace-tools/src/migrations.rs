//! Apply the coda-app backend migration chain in sorted order.
//!
//! SQL still runs through `psql -f`. A missing `schema_migrations` table means
//! the version is not applied. Version stems are accepted only when they match
//! `[0-9]{4}_[a-z0-9_]+`, before that stem is interpolated into a query.

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::failure::Failure;

#[derive(Debug)]
pub struct Counts {
    pub applied: usize,
    pub skipped: usize,
}

pub fn from_args(args: &[String]) -> Result<(), Failure> {
    let psql = psql_command(args);
    let dir = crate::workspace_root().join("submodules/coda-app/backend/migrations");
    let counts = apply(&dir, &psql, None)?;
    println!(
        "migrations: {} applied, {} already present",
        counts.applied, counts.skipped
    );
    Ok(())
}

pub fn default_psql() -> Vec<String> {
    [
        "psql",
        "-v",
        "ON_ERROR_STOP=1",
        "-h",
        "127.0.0.1",
        "-p",
        "5433",
        "-U",
        "coda",
        "-d",
        "duet",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn psql_command(args: &[String]) -> Vec<String> {
    let args = if args.first().is_some_and(|arg| arg == "--") {
        &args[1..]
    } else {
        args
    };
    if args.is_empty() {
        default_psql()
    } else {
        args.to_vec()
    }
}

pub fn apply(dir: &Path, psql: &[String], path_prefix: Option<&Path>) -> Result<Counts, Failure> {
    if psql.is_empty() {
        return Err(Failure::new(1, "psql command is empty"));
    }
    if !dir.is_dir() {
        return Err(Failure::new(
            1,
            format!(
                "migrations directory missing: {}\ninitialize the coda-app submodule first",
                dir.display()
            ),
        ));
    }
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)
        .map_err(|error| Failure::new(1, format!("could not read {}: {error}", dir.display())))?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            Failure::new(1, format!("could not read {}: {error}", dir.display()))
        })?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|extension| extension == "sql") {
            files.push(path);
        }
    }
    files.sort();

    let mut applied = 0;
    let mut skipped = 0;
    for file in files {
        let version = file
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default()
            .to_owned();
        if !valid_version(&version) {
            return Err(Failure::new(
                1,
                format!("refusing migration version '{version}'"),
            ));
        }
        if already_applied(psql, &version, path_prefix)? {
            skipped += 1;
            continue;
        }
        println!("applying {version}");
        let mut args = psql[1..].to_vec();
        args.push("-f".to_owned());
        args.push(file.display().to_string());
        let output = command(&psql[0], &args, path_prefix)
            .output()
            .map_err(|error| Failure::new(1, format!("could not run {}: {error}", psql[0])))?;
        if !output.status.success() {
            return Err(Failure::new(
                1,
                format!(
                    "psql failed for {version}\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            ));
        }
        applied += 1;
    }
    Ok(Counts { applied, skipped })
}

fn valid_version(stem: &str) -> bool {
    let bytes = stem.as_bytes();
    bytes.len() >= 6
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[4] == b'_'
        && bytes[5..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || byte.is_ascii_lowercase() || *byte == b'_')
}

fn already_applied(
    psql: &[String],
    version: &str,
    path_prefix: Option<&Path>,
) -> Result<bool, Failure> {
    let query = format!("select 1 from public.schema_migrations where version = '{version}'");
    let mut args = psql[1..].to_vec();
    args.push("-tAc".to_owned());
    args.push(query);
    let output = command(&psql[0], &args, path_prefix)
        .output()
        .map_err(|error| Failure::new(1, format!("could not run {}: {error}", psql[0])))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains('1'))
}

fn command(program: &str, args: &[String], path_prefix: Option<&Path>) -> Command {
    let mut command = Command::new(program);
    command.args(args);
    if let Some(prefix) = path_prefix {
        let mut path = prefix.as_os_str().to_owned();
        path.push(":");
        if let Some(existing) = std::env::var_os("PATH") {
            path.push(existing);
        }
        command.env("PATH", path);
    }
    command
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "migrations-{label}-{}-{}",
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

    fn write_psql(dir: &Path, body: &str) -> PathBuf {
        let bin = dir.join("bin");
        fs::create_dir_all(&bin).unwrap();
        let path = bin.join("psql");
        fs::write(&path, body).unwrap();
        let mut permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).unwrap();
        bin
    }

    fn fake_psql(log: &Path) -> String {
        format!(
            r#"#!/bin/sh
printf '%s\n' "$*" >> '{log}'
prev=
for arg in "$@"; do
  if [ "$prev" = "-f" ]; then
    printf 'APPLY %s\n' "$arg" >> '{log}'
    exit 0
  fi
  prev=$arg
done
case $prev in
  *"version = '0001_present'"*)
    printf '1\n'
    exit 0
    ;;
  *)
    exit 1
    ;;
esac
"#,
            log = log.display()
        )
    }

    #[test]
    fn migrations_apply_in_sorted_order_with_the_default_psql() {
        let temp = TempDir::new("order");
        let migrations = temp.0.join("migrations");
        fs::create_dir_all(&migrations).unwrap();
        fs::write(migrations.join("0002_b.sql"), "select 2;\n").unwrap();
        fs::write(migrations.join("0001_a.sql"), "select 1;\n").unwrap();
        let log = temp.0.join("psql.log");
        let bin = write_psql(&temp.0, &fake_psql(&log));

        let counts = apply(&migrations, &default_psql(), Some(&bin)).unwrap();
        assert_eq!(counts.applied, 2);
        assert_eq!(counts.skipped, 0);
        let log = fs::read_to_string(log).unwrap();
        assert!(log.contains("-h 127.0.0.1 -p 5433 -U coda -d duet"));
        let applied: Vec<_> = log
            .lines()
            .filter(|line| line.starts_with("APPLY "))
            .collect();
        assert_eq!(applied.len(), 2);
        assert!(applied[0].ends_with("0001_a.sql"));
        assert!(applied[1].ends_with("0002_b.sql"));
    }

    #[test]
    fn an_applied_version_is_skipped() {
        let temp = TempDir::new("skip");
        let migrations = temp.0.join("migrations");
        fs::create_dir_all(&migrations).unwrap();
        fs::write(migrations.join("0001_present.sql"), "select 1;\n").unwrap();
        fs::write(migrations.join("0002_new.sql"), "select 2;\n").unwrap();
        let log = temp.0.join("psql.log");
        let bin = write_psql(&temp.0, &fake_psql(&log));

        let counts = apply(&migrations, &default_psql(), Some(&bin)).unwrap();
        assert_eq!(counts.applied, 1);
        assert_eq!(counts.skipped, 1);
        let log = fs::read_to_string(log).unwrap();
        let applied: Vec<_> = log
            .lines()
            .filter(|line| line.starts_with("APPLY "))
            .collect();
        assert_eq!(applied.len(), 1);
        assert!(applied[0].ends_with("0002_new.sql"));
    }

    #[test]
    fn an_unsafe_version_stem_is_refused_before_psql() {
        let temp = TempDir::new("reject");
        let migrations = temp.0.join("migrations");
        fs::create_dir_all(&migrations).unwrap();
        fs::write(migrations.join("not-a-token.sql"), "select 1;\n").unwrap();
        let log = temp.0.join("psql.log");
        let bin = write_psql(
            &temp.0,
            &format!("#!/bin/sh\necho invoked >> '{}'\nexit 99\n", log.display()),
        );

        let error = apply(&migrations, &default_psql(), Some(&bin)).unwrap_err();
        assert!(error.message.contains("not-a-token"));
        assert!(!log.exists());
    }
}

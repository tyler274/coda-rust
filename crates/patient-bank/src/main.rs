//! The patient bank pipeline: one command per step.
//!
//! ```text
//! cargo run -p patient-bank -- catalog [--offline] [--roger-fixture DIR]
//! cargo run -p patient-bank -- brief|check|preview UNIT [--seeds PATH]
//! cargo run -p patient-bank -- build [--seeds PATH] [--ledgers DIR] [--out DIR]
//! cargo run -p patient-bank -- validate [--bank DIR] [--schema PATH] [--jobs 8] [--report PATH]
//! ```
//!
//! Defaults come from `patient_bank::paths`: inputs in `crates/patient-bank/data/`,
//! seeds and ledgers in `patients/`, drafts and the bank under `target/patient-bank/`.

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use patient_bank::{bank, catalog, paths, unit, validate};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match dispatch(&args) {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

type Failure = Box<dyn std::error::Error + Send + Sync>;

fn dispatch(args: &[String]) -> Result<u8, Failure> {
    let Some((command, rest)) = args.split_first() else {
        return Err(usage());
    };
    let status = match command.as_str() {
        "catalog" => {
            let o = Opts::parse(rest, &["--roger-fixture"], &["--offline"])?;
            let roger = o
                .path("--roger-fixture")
                .unwrap_or_else(paths::roger_fixture);
            catalog::build(&roger, o.on("--offline"))?
        }
        "brief" | "check" | "preview" => {
            let o = Opts::parse(rest, &["--seeds"], &[])?;
            let [uid] = o.positional.as_slice() else {
                return Err(format!("{command} takes one unit id").into());
            };
            let seeds = o.path("--seeds").unwrap_or_else(paths::seeds);
            unit::run(command, uid, &seeds)?
        }
        "build" => {
            let o = Opts::parse(rest, &["--seeds", "--ledgers", "--out"], &[])?;
            bank::run(
                &o.path("--seeds").unwrap_or_else(paths::seeds),
                &o.path("--ledgers").unwrap_or_else(paths::ledgers),
                &o.path("--out")
                    .unwrap_or_else(|| paths::root().join("bank")),
            )?;
            0
        }
        "validate" => {
            let o = Opts::parse(rest, &["--bank", "--schema", "--jobs", "--report"], &[])?;
            validate::run(
                &o.path("--bank")
                    .unwrap_or_else(|| paths::root().join("bank")),
                &o.path("--schema").unwrap_or_else(paths::schema),
                o.number("--jobs")?.unwrap_or(8),
                o.path("--report").as_deref(),
            )?
        }
        _ => return Err(usage()),
    };
    Ok(u8::try_from(status).unwrap_or(1))
}

/// `--flag value`, `--flag=value`, on/off switches and positional arguments.
struct Opts {
    values: HashMap<String, String>,
    switches: Vec<String>,
    positional: Vec<String>,
}

impl Opts {
    fn parse(args: &[String], valued: &[&str], switches: &[&str]) -> Result<Opts, Failure> {
        let mut o = Opts {
            values: HashMap::new(),
            switches: Vec::new(),
            positional: Vec::new(),
        };
        let mut it = args.iter();
        while let Some(arg) = it.next() {
            let (name, inline) = match arg.split_once('=') {
                Some((name, value)) if arg.starts_with("--") => (name, Some(value.to_string())),
                _ => (arg.as_str(), None),
            };
            if valued.contains(&name) {
                let value = match inline {
                    Some(value) => value,
                    None => it.next().ok_or(format!("{name} needs a value"))?.clone(),
                };
                o.values.insert(name.to_string(), value);
            } else if switches.contains(&name) && inline.is_none() {
                o.switches.push(name.to_string());
            } else if arg.starts_with("--") {
                return Err(format!("unknown option {arg}\n\n{}", usage()).into());
            } else {
                o.positional.push(arg.clone());
            }
        }
        Ok(o)
    }

    fn path(&self, name: &str) -> Option<PathBuf> {
        self.values.get(name).map(PathBuf::from)
    }

    fn on(&self, name: &str) -> bool {
        self.switches.iter().any(|s| s == name)
    }

    fn number<T: std::str::FromStr>(&self, name: &str) -> Result<Option<T>, Failure> {
        self.values
            .get(name)
            .map(|v| {
                v.parse()
                    .map_err(|_| format!("{name}: not a number: {v}").into())
            })
            .transpose()
    }
}

fn usage() -> Failure {
    "\
patient-bank runs the synthetic patient bank pipeline.

  patient-bank catalog [--offline] [--roger-fixture DIR]
  patient-bank brief|check|preview UNIT [--seeds PATH]
  patient-bank build [--seeds PATH] [--ledgers DIR] [--out DIR]
  patient-bank validate [--bank DIR] [--schema PATH] [--jobs 8] [--report PATH]

PATIENT_BANK_DATA and PATIENT_BANK_ROOT override crates/patient-bank/data and target/patient-bank."
        .into()
}

//! The patient bank pipeline: one command per step.
//!
//! ```text
//! cargo run -p patient-bank -- catalog [--roger-fixture DIR]
//! cargo run -p patient-bank -- import --db PATH
//! cargo run -p patient-bank -- brief|check|preview UNIT
//! cargo run -p patient-bank -- find icd10|loinc|snomed|rxnorm WORDS...
//! cargo run -p patient-bank -- build [--out DIR]
//! cargo run -p patient-bank -- validate [--bank DIR] [--schema PATH] [--jobs 8] [--report PATH]
//! ```
//!
//! Defaults come from `patient_bank::paths`: inputs in `crates/patient-bank/data/`,
//! one folder per unit in `patients/units/`, previews and the bank under `target/patient-bank/`.

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use patient_bank::{bank, catalog, import, paths, unit, validate};

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
            let o = Opts::parse(rest, &["--roger-fixture"])?;
            let roger = o
                .path("--roger-fixture")
                .unwrap_or_else(paths::roger_fixture);
            catalog::build(&roger)?
        }
        "import" => {
            let o = Opts::parse(rest, &["--db"])?;
            let db = o
                .path("--db")
                .ok_or("import needs --db, the synthetic_hospital benchmark_v1.3.db")?;
            import::run(&db, &paths::patients_dir())?
        }
        "brief" | "check" | "preview" => {
            let o = Opts::parse(rest, &[])?;
            let [uid] = o.positional.as_slice() else {
                return Err(format!("{command} takes one unit id").into());
            };
            unit::run(command, uid)?
        }
        "find" => {
            let o = Opts::parse(rest, &[])?;
            let [system, words @ ..] = o.positional.as_slice() else {
                return Err("find takes a system (icd10, loinc, snomed, rxnorm) and words".into());
            };
            // reads the shared answers and writes none, so authors can search at once
            let mut look = catalog::Lookup::layered(&paths::root().join("find-lookups.json"))?;
            for row in look.find(system, &words.join(" "))? {
                println!("{row}");
            }
            0
        }
        "build" => {
            let o = Opts::parse(rest, &["--out"])?;
            bank::run(
                &paths::patients_dir(),
                &o.path("--out")
                    .unwrap_or_else(|| paths::root().join("bank")),
            )?;
            0
        }
        "validate" => {
            let o = Opts::parse(rest, &["--bank", "--schema", "--jobs", "--report"])?;
            validate::run(
                &o.path("--bank")
                    .unwrap_or_else(|| paths::root().join("bank")),
                &paths::patients_dir(),
                &o.path("--schema").unwrap_or_else(paths::schema),
                o.number("--jobs")?.unwrap_or(8),
                o.path("--report").as_deref(),
            )?
        }
        _ => return Err(usage()),
    };
    Ok(u8::try_from(status).unwrap_or(1))
}

/// `--flag value`, `--flag=value` and positional arguments.
struct Opts {
    values: HashMap<String, String>,
    positional: Vec<String>,
}

impl Opts {
    fn parse(args: &[String], valued: &[&str]) -> Result<Opts, Failure> {
        let mut o = Opts {
            values: HashMap::new(),
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

  patient-bank catalog [--roger-fixture DIR]
  patient-bank import --db PATH
  patient-bank brief|check|preview UNIT
  patient-bank find icd10|loinc|snomed|rxnorm WORDS...
  patient-bank build [--out DIR]
  patient-bank validate [--bank DIR] [--schema PATH] [--jobs 8] [--report PATH]

PATIENT_BANK_DATA, PATIENT_BANK_PATIENTS and PATIENT_BANK_ROOT override
crates/patient-bank/data, patients and target/patient-bank."
        .into()
}

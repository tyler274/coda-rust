use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use encoder::{
    BertEncoder, Protocol, fingerprint, serve_duet, serve_patient, startup_line,
    unsupported_inference,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

struct Args {
    model: PathBuf,
    protocol: Protocol,
    recipe_only: bool,
    threads: usize,
}

fn run() -> Result<(), String> {
    let args = Args::parse()?;
    if args.recipe_only {
        println!(
            "{}",
            serde_json::json!({ "recipe": fingerprint(args.protocol, &args.model)? })
        );
        return Ok(());
    }
    if args.protocol == Protocol::Patient {
        if let Some(reason) = unsupported_inference(&args.model)? {
            return Err(format!(
                "This model's declared inference is not supported: {reason}"
            ));
        }
    }
    let identity = fingerprint(args.protocol, &args.model)?;
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .map_err(|error| error.to_string())?;
    let mut encoder = BertEncoder::load(&args.model)?;
    if fingerprint(args.protocol, &args.model)? != identity {
        return Err("Health encoder files changed during startup".into());
    }
    println!("{}", startup_line(args.protocol, &identity));
    let _ = std::io::stdout().flush();
    match args.protocol {
        Protocol::Duet => serve_duet(&mut encoder),
        Protocol::Patient => serve_patient(&mut encoder),
    }
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut model = None;
        let mut protocol = None;
        let mut recipe_only = false;
        let mut threads = 4_usize;
        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--model" => model = Some(required(&mut args, "--model")?),
                "--protocol" => {
                    let value = required(&mut args, "--protocol")?;
                    protocol = Some(
                        Protocol::parse(&value)
                            .ok_or_else(|| format!("unknown protocol {value}"))?,
                    );
                }
                "--recipe-only" => recipe_only = true,
                "--threads" => {
                    let value = required(&mut args, "--threads")?;
                    threads = value
                        .parse()
                        .map_err(|_| format!("invalid thread count {value}"))?;
                    if threads < 1 {
                        return Err("thread count must be at least 1".into());
                    }
                }
                other => return Err(format!("unknown argument {other}")),
            }
        }
        Ok(Self {
            model: PathBuf::from(model.ok_or("missing --model")?),
            protocol: protocol.ok_or("missing --protocol")?,
            recipe_only,
            threads,
        })
    }
}

fn required(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("missing value for {flag}"))
}

//! Build-time CLI seeding `demo.sqlite` from `patients/units/`.

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use physician_demo_db::{DemoStore, SqliteDriver, seed};
    use std::path::PathBuf;

    let (mut units, mut out, mut limit) = (
        PathBuf::from("patients/units"),
        PathBuf::from("target/demo.sqlite"),
        60usize,
    );
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let val = args
            .next()
            .ok_or_else(|| format!("missing value for {arg}"))?;
        match arg.as_str() {
            "--units" => units = PathBuf::from(val),
            "--out" => out = PathBuf::from(val),
            "--limit" => limit = val.parse()?,
            other => return Err(format!("unknown argument `{other}`").into()),
        }
    }
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if out.exists() {
        std::fs::remove_file(&out)?;
    }
    let store = DemoStore::new(SqliteDriver::open_file(&out)?);
    let count = seed::seed_from_units_dir(&store, &units, limit)?;
    println!("Seeded {count} patients into {}", out.display());
    Ok(())
}

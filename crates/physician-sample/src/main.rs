use std::env;

use local_product::{DEFAULT_SIMULATION_SCENARIO, SimulationSession, local_simulation_scenarios};
use serde_json::json;

#[tokio::main]
async fn main() {
    if let Err(reason) = physician_sample::hosting::validate_startup(|key| env::var(key).ok()) {
        eprintln!("{reason}");
        std::process::exit(1);
    }
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "--list-simulations") {
        let catalog = local_simulation_scenarios();
        println!(
            "{}",
            serde_json::to_string_pretty(&catalog).expect("simulation catalog JSON")
        );
        return;
    }

    if let Some(pos) = args.iter().position(|arg| arg == "--run-simulation") {
        let scenario = args
            .get(pos + 1)
            .map(String::as_str)
            .unwrap_or(DEFAULT_SIMULATION_SCENARIO);
        run_cli_simulation(scenario);
        return;
    }

    if let Some(pos) = args.iter().position(|arg| arg == "--simulate") {
        let scenario = args
            .get(pos + 1)
            .filter(|val| !val.starts_with("--"))
            .map(String::as_str)
            .unwrap_or(DEFAULT_SIMULATION_SCENARIO);
        // SAFETY: set once on the main thread before binding or spawning tasks.
        unsafe {
            env::set_var("CODA_LOCAL_SIMULATION", scenario);
        }
        eprintln!(
            "local simulation active: scenario={scenario} (rerunnable from the physician UI)"
        );
    }

    let bind_addr = env::var("PHYSICIAN_LISTEN_ADDR").unwrap_or_else(|_| {
        env::var("PORT")
            .map(|port| format!("0.0.0.0:{port}"))
            .unwrap_or_else(|_| "127.0.0.1:8790".to_owned())
    });
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind {bind_addr}: {err}"));
    eprintln!(
        "physician site on http://{}",
        listener.local_addr().unwrap()
    );
    physician_sample::serve(listener).await;
}

fn run_cli_simulation(scenario: &str) {
    let mut session = SimulationSession::default();
    if scenario == "all" {
        session.run_history.clear();
        for item in local_simulation_scenarios() {
            session
                .run_scenario(&item.key)
                .unwrap_or_else(|err| panic!("scenario {} failed: {err}", item.key));
        }
        let report = json!({
            "runs": session.run_history,
            "aggregate_metric": session.aggregate_metric,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("simulation report JSON")
        );
        if session.run_history.iter().any(|run| !run.passed) {
            std::process::exit(1);
        }
        return;
    }

    match session.run_scenario(scenario) {
        Ok(record) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&record).expect("simulation run JSON")
            );
            if !record.passed {
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

//! Model-check a journey chart's generated Stateright model, or serve its Explorer.
//!
//! Each chart's `stateright.rs` holds the Verus model's definitions as ordinary Rust and
//! the graph the chart search reaches. `check` searches every reachable state for a broken
//! rule and confirms the relation reaches exactly the search's graph; `serve` opens
//! Stateright's Explorer for clicking through the chart.
use std::collections::BTreeSet;
use std::process::ExitCode;

use stateright::{Checker, Model};

// One line per chart: its name on the command line, and its generated model.
macro_rules! charts {
    ($($name:literal => $module:ident at $path:literal),* $(,)?) => {
        $(
            #[path = $path]
            #[allow(dead_code, unused_parens, unused_variables, clippy::all)]
            mod $module;
        )*
        const NAMES: &[&str] = &[$($name),*];
        fn run(chart: &str, mode: Mode) -> Option<ExitCode> {
            match chart {
                $($name => Some(match mode {
                    Mode::Check => check($module::Chart, $module::captions, $module::SEARCH_STATES, $module::SEARCH_EDGES),
                    Mode::Serve(address) => {
                        println!("Explorer for {chart}: http://{address}");
                        $module::Chart.checker().serve(address);
                        ExitCode::SUCCESS
                    }
                }),)*
                _ => None,
            }
        }
    };
}

charts! {
    "physician/sign-in" => physician_sign_in at "../../physician/sign-in/stateright.rs",
}

enum Mode {
    Check,
    Serve(String),
}

type Edge = (Vec<&'static str>, &'static str, Vec<&'static str>);

fn check<M>(
    model: M,
    captions: fn(M::State) -> Vec<&'static str>,
    search_states: &[&[&'static str]],
    search_edges: &[(usize, &'static str, usize)],
) -> ExitCode
where
    M: Model<Action = (&'static str, <M as Model>::State)> + Send + Sync + 'static,
    M::State: Copy + std::fmt::Debug + std::hash::Hash + Eq + Send + Sync + 'static,
{
    let mut failed = false;

    // The relation's graph, walked directly, against the graph the chart search reached.
    let mut states = BTreeSet::new();
    let mut edges = BTreeSet::<Edge>::new();
    let mut queue = model.init_states();
    let mut seen: std::collections::HashSet<M::State> = queue.iter().copied().collect();
    while let Some(s) = queue.pop() {
        states.insert(captions(s));
        let mut actions = Vec::new();
        model.actions(&s, &mut actions);
        for (event, t) in actions {
            edges.insert((captions(s), event, captions(t)));
            if seen.insert(t) {
                queue.push(t);
            }
        }
    }
    let searched: BTreeSet<Vec<&str>> = search_states.iter().map(|s| s.to_vec()).collect();
    let searched_edges: BTreeSet<Edge> = search_edges
        .iter()
        .map(|&(from, event, to)| {
            (
                search_states[from].to_vec(),
                event,
                search_states[to].to_vec(),
            )
        })
        .collect();
    for s in states.difference(&searched).take(10) {
        failed = true;
        println!("Only the model reaches: {}", s.join(" · "));
    }
    for s in searched.difference(&states).take(10) {
        failed = true;
        println!("Only the chart search reaches: {}", s.join(" · "));
    }
    for (from, event, to) in edges.symmetric_difference(&searched_edges).take(10) {
        failed = true;
        let side = if edges.contains(&(from.clone(), *event, to.clone())) {
            "model"
        } else {
            "chart search"
        };
        println!(
            "Only the {side} steps: {} --{event}--> {}",
            from.join(" · "),
            to.join(" · ")
        );
    }
    if !failed {
        println!(
            "Agrees with the chart search: {} states, {} steps.",
            states.len(),
            edges.len()
        );
    }

    // Stateright's own breadth-first search, for the shortest path to each broken rule.
    // With no rules it has nothing to look for and stops at once.
    if model.properties().is_empty() {
        println!("No chart rules; no safety claim checked.");
        return if failed {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        };
    }
    let checker = model.checker().spawn_bfs().join();
    for property in checker.model().properties() {
        if let Some(path) = checker.discovery(property.name) {
            failed = true;
            println!("BROKEN {}", property.name);
            let states = path.clone().into_states();
            println!("  1. Start → {:?}", states[0]);
            for (i, (event, t)) in path.into_actions().into_iter().enumerate() {
                println!("  {}. {event} → {t:?}", i + 2);
            }
        }
    }
    if checker.is_done() && !failed {
        println!(
            "Exhausted {} reachable states; every rule holds. Data, progress and implementation remain separate.",
            checker.unique_state_count()
        );
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = match args.get(1).map(String::as_str) {
        Some("check") if args.len() == 2 => Some(Mode::Check),
        Some("serve") if args.len() <= 3 => Some(Mode::Serve(
            args.get(2)
                .cloned()
                .unwrap_or_else(|| "localhost:3000".into()),
        )),
        _ => None,
    };
    match mode.and_then(|mode| run(&args[0], mode)) {
        Some(code) => code,
        None => {
            eprintln!("Usage: chart-stateright CHART check | CHART serve [ADDRESS]");
            eprintln!("Charts: {}", NAMES.join(", "));
            ExitCode::from(2)
        }
    }
}

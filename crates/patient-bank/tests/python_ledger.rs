//! `ledger::check` returns exactly the errors and warnings Python's `ledger.check` did, and
//! raises where it raised. `data/python-ledger.json` holds small ledgers derived from one
//! real ledger (unit-0080, trimmed), each case a patch on that base exercising one branch, and
//! the output of CPython 3.14 on each.

use std::panic::{AssertUnwindSafe, catch_unwind};

use patient_bank::json::{self, Json};
use patient_bank::ledger;

/// Apply `[path, value]` sets and `[path]` deletes; an index one past the end appends.
fn patch(base: &Json, ops: &Json) -> Json {
    let mut v = base.clone();
    for op in ops.as_array() {
        let op = op.as_array();
        apply(&mut v, op[0].as_array(), op.get(1));
    }
    v
}

fn apply(at: &mut Json, path: &[Json], value: Option<&Json>) {
    let Some((key, rest)) = path.split_first() else {
        *at = value.expect("a value for the root").clone();
        return;
    };
    match (at, key) {
        (Json::Obj(m), Json::Str(k)) if !rest.is_empty() => apply(
            &mut m.iter_mut().find(|(x, _)| x == k).expect("path").1,
            rest,
            value,
        ),
        (Json::Arr(a), Json::Int(i)) if !rest.is_empty() => apply(&mut a[*i as usize], rest, value),
        (obj @ Json::Obj(_), Json::Str(k)) => match value {
            Some(value) => obj.set(k, value.clone()),
            None => drop(obj.remove(k)),
        },
        (Json::Arr(a), Json::Int(i)) => {
            let i = *i as usize;
            match value {
                None => drop(a.remove(i)),
                Some(value) if i == a.len() => a.push(value.clone()),
                Some(value) => a[i] = value.clone(),
            }
        }
        _ => panic!("bad patch path"),
    }
}

#[test]
fn check_matches_python_on_every_branch() {
    let data = json::parse(include_str!("data/python-ledger.json")).expect("reference");
    let catalog = data.get("catalog");
    std::panic::set_hook(Box::new(|_| {}));
    let mut failures = Vec::new();
    for case in data.get("cases").as_array() {
        let name = case.get("name").as_str().unwrap();
        let unit = patch(data.get("unit"), case.get("unit"));
        let led = patch(data.get("ledger"), case.get("ledger"));
        let got = catch_unwind(AssertUnwindSafe(|| ledger::check(&unit, &led, catalog)));
        let strings = |v: &Json| -> Vec<String> {
            v.as_array()
                .iter()
                .map(|s| s.as_str().unwrap().to_string())
                .collect()
        };
        let ok = match (&got, case.contains("raised")) {
            (Err(_), true) => true,
            (Ok((e, w)), false) => {
                *e == strings(case.get("errors")) && *w == strings(case.get("warnings"))
            }
            _ => false,
        };
        if !ok {
            failures.push(format!("{name}: got {:?}", got.as_ref().ok()));
        }
    }
    let _ = std::panic::take_hook();
    assert!(failures.is_empty(), "{failures:#?}");
}

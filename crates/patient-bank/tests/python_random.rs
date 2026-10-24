//! The port of CPython's `random.Random` replays CPython's draws exactly.
//! `data/python-random.json` was produced by CPython 3.14 for these seeds.

use patient_bank::json::{self, Json};
use patient_bank::rng::PyRandom;

fn floats(v: Vec<f64>) -> Json {
    Json::Arr(v.into_iter().map(Json::Float).collect())
}

fn ints(v: Vec<i64>) -> Json {
    Json::Arr(v.into_iter().map(Json::Int).collect())
}

fn replay(mut r: PyRandom) -> Json {
    let random = floats((0..5).map(|_| r.random()).collect());
    let mut randint: Vec<i64> = (0..5).map(|_| r.randint(0, 600)).collect();
    randint.extend((0..3).map(|_| r.randint(-6, 6)));
    randint.extend((0..2).map(|_| r.randint(0, 1 << 40)));
    let uniform = floats((0..3).map(|_| r.uniform(32.6, 34.2)).collect());
    let gauss = floats((0..5).map(|_| r.gauss(0.0, 1.35)).collect());
    let seven: Vec<i64> = (0..7).collect();
    let choice = ints((0..5).map(|_| *r.choice(&seven)).collect());
    let sample_small = ints(r.sample(&(0..19).collect::<Vec<i64>>(), 4));
    let sample_large = ints(r.sample(&(0..200).collect::<Vec<i64>>(), 10));
    let mut shuffle: Vec<i64> = (0..12).collect();
    r.shuffle(&mut shuffle);
    patient_bank::obj![
        ("random", random),
        ("randint", ints(randint)),
        ("uniform", uniform),
        ("gauss", gauss),
        ("choice", choice),
        ("sample_small", sample_small),
        ("sample_large", sample_large),
        ("shuffle", ints(shuffle)),
        ("after", r.random()),
    ]
}

#[test]
fn integer_and_string_seeds_replay_cpython() {
    let want = json::parse(include_str!("data/python-random.json")).expect("reference JSON");
    for (name, rng) in [
        ("int-20261024", PyRandom::new(20_261_024)),
        ("int-0", PyRandom::new(0)),
        ("int-big", PyRandom::new((1u128 << 80) + 12_345)),
        ("str-mrn", PyRandom::from_text("MRN00000014")),
    ] {
        assert_eq!(&replay(rng), want.get(name), "{name}");
    }
}

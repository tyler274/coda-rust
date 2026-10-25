//! `json::dumps` writes exactly what Python's `json.dumps` wrote for the bank's
//! layouts, and `json::parse` keeps key order and exact floats.
//! `data/python-json.json` was produced by CPython 3.14.

use patient_bank::json::{self, Json, Layout};

#[test]
fn dumps_matches_python_for_every_layout() {
    let cases = json::parse(include_str!("data/python-json.json")).expect("reference");
    for case in cases.as_array() {
        // Each reference string parses to the value Python serialized.
        let value = json::parse(case.get("compact").as_str().unwrap()).expect("sample");
        for (key, layout, ascii) in [
            ("compact", Layout::Compact, false),
            ("default", Layout::Default, true),
            ("indent1", Layout::Indent(1), false),
            ("indent1_ascii", Layout::Indent(1), true),
        ] {
            assert_eq!(
                json::dumps(&value, layout, ascii),
                case.get(key).as_str().unwrap(),
                "{key}"
            );
        }
    }
}

#[test]
fn parse_keeps_order_and_number_kinds() {
    let v = json::parse(r#"{"z": 1, "a": 1.0, "m": 0.018778981204688794}"#).unwrap();
    let keys: Vec<&str> = v.as_object().iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(keys, ["z", "a", "m"]);
    assert_eq!(v.get("z"), &Json::Int(1));
    assert_eq!(v.get("a"), &Json::Float(1.0));
    assert_eq!(v.get("m").as_f64(), Some(0.018778981204688794));
}

/// `data/python-json-errors.json`: small documents (real ledger and seed text among them)
/// with random deletions, insertions and truncations, and what CPython's `json.loads` made
/// of each: the value, or `str(JSONDecodeError)`. Lone surrogates and integers outside
/// i64, the two documented exceptions, are left out.
#[test]
fn parse_accepts_and_rejects_like_python() {
    let cases = json::parse(include_str!("data/python-json-errors.json")).expect("reference");
    let mut errors = 0;
    for case in cases.as_array() {
        let [text, want] = case.as_array() else {
            panic!("case shape");
        };
        let text = text.as_str().unwrap();
        let got = match json::parse(text) {
            Ok(v) => ("value", json::dumps(&v, Layout::Compact, true)),
            Err(e) => {
                errors += 1;
                ("error", e.to_string())
            }
        };
        let (kind, expected) = want.as_object()[0].clone();
        assert_eq!(
            (got.0, got.1.as_str()),
            (kind.as_str(), expected.as_str().unwrap()),
            "{text:?}"
        );
    }
    assert!(errors > 100, "the corpus exercises the error paths");
}

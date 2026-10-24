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

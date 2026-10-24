//! The practice clock converts and adds exactly as Python's zoneinfo did.
//! `data/python-time.json` was produced by CPython 3.14.

use patient_bank::json;
use patient_bank::time;

#[test]
fn eastern_wall_clock_matches_python_zoneinfo() {
    let want = json::parse(include_str!("data/python-time.json")).unwrap();
    for (stamp, utc) in want.get("to_utc").as_object() {
        assert_eq!(
            time::stamp_to_utc(stamp).as_deref(),
            utc.as_str(),
            "{stamp}"
        );
    }
    for case in want.get("plus").as_array() {
        let c = case.as_array();
        let start = time::local(c[0].as_str().unwrap()).unwrap();
        let got = time::fmt_local(time::add(start, time::minutes(c[1].as_i64().unwrap())));
        assert_eq!(Some(got.as_str()), c[2].as_str());
    }
    for (utc, wall) in want.get("back").as_object() {
        let got = time::utc_to_local(utc)
            .unwrap()
            .strftime("%Y-%m-%d %H:%M")
            .to_string();
        assert_eq!(Some(got.as_str()), wall.as_str(), "{utc}");
    }
}

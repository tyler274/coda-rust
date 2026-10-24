//! `json::float_repr` breaks exact ties between two shortest digit strings the way
//! CPython's `repr` does (to the even last digit), where Rust's own shortest formatting
//! rounds up. Expected strings are CPython 3.14's `repr` of each bit pattern.

use patient_bank::json::float_repr;

const CASES: [(u64, &str); 21] = [
    // Exact ties: the value's expansion is one digit longer than the shortest and ends in 5.
    (0x43092c5624242ad2, "885702862865754.2"), // exact 885702862865754.25
    (0x430c4b9d4d9d9836, "995554791502598.8"), // exact 995554791502598.75
    (0xc2e2823841268594, "-162803990737964.62"), // exact -162803990737964.625
    (0xc314aea4d880a235, "-1455380641097869.2"), // exact -1455380641097869.25
    (0xc31341b48c5ebf87, "-1355067065610209.8"), // exact -1355067065610209.75
    (0x43039c8b847a4df6, "690018449312190.8"), // exact 690018449312190.75
    (0xc3116ed2e577bb8d, "-1226731669024483.2"), // exact -1226731669024483.25
    (0xc3108c5fd3f3a511, "-1164485708278084.2"), // exact -1164485708278084.25
    (0x42d1c70a4ac50038, "78186275345408.88"), // exact 78186275345408.875
    (0xc31c1c6d5365b597, "-1978138806021477.8"), // exact -1978138806021477.75
    (0x430d473cd9d7ed52, "1030137625378218.2"), // exact 1030137625378218.25
    (0x431d9a254452ae5d, "2083064793836439.2"), // exact 2083064793836439.25
    // No tie: unchanged.
    (0x4350000000000000, "1.8014398509481984e+16"),
    (0x3fb999999999999a, "0.1"),
    (0x4480f0cf064dd592, "1e+22"),
    (0x0000000000000001, "5e-324"),
    (0x7fefffffffffffff, "1.7976931348623157e+308"),
    (0x0000000000000003, "1.5e-323"),
    (0x433316ea079d6800, "5373218963417088.0"),
    (0x435381124218cffb, "2.1959759903277036e+16"),
    (0x4351c007084ca0c8, "1.998484416249936e+16"),
];

#[test]
fn shortest_ties_round_half_even_like_cpython() {
    for (bits, want) in CASES {
        assert_eq!(float_repr(f64::from_bits(bits)), want, "{bits:#018x}");
    }
}

//! Verify the out-of-catalog codes one unit's authors proposed.
//!
//! A proposed ICD-10-CM code is accepted only when NLM Clinical Tables returns it,
//! and its official display replaces the author's. A drug written as text is coded
//! only when RxNav resolves it to a clinical or branded drug with the same
//! ingredient and an equivalent strength: `3 mg/0.5 mL` is 6 mg/mL, so it never
//! matches a 3 mg/mL pen. A drug whose text states no strength stays text-only. A
//! lab or study's LOINC is accepted when NLM Clinical Tables returns it, and an
//! allergen or procedure's SNOMED CT code when tx.fhir.org knows it as an active
//! concept of the right kind (a substance, product or organism; a procedure).
//! Everything that fails stays text-only and is listed as rejected.

use std::sync::LazyLock;

use regex::Regex;

use crate::Result;
use crate::catalog::{Catalog, Lookup};
use crate::ledger::{LabResult, Patient, Terms, VerifiedCode};

/// One strength: an amount of a base unit per one base unit of something, or a bare
/// quantity (a pen's volume, a patch's wear time) when `per` is empty.
#[derive(Clone, Debug, PartialEq)]
pub struct Strength {
    pub amount: f64,
    pub unit: &'static str,
    pub per: &'static str,
}

static STRENGTH: LazyLock<Regex> = LazyLock::new(|| {
    let unit =
        r"(?:mg|mcg|µg|g|gm|ml|unt|units?|iu|meq|actuat(?:ions?)?|puffs?|hr|hours?|h|days?)\b|%";
    Regex::new(&format!(
        r"(?i)(\d+(?:\.\d+)?|\.\d+)\s*({unit})(?:\s*(?:/|per)\s*(\d+(?:\.\d+)?|\.\d+)?\s*({unit}))?"
    ))
    .expect("regex")
});

/// A unit in base form and the factor to it: mass in mg, volume in mL, time in
/// hours. Per gram (a cream's mg/g) counts as per mL.
fn base(unit: &str, denominator: bool) -> Option<(&'static str, f64)> {
    Some(match unit.to_lowercase().as_str() {
        "mg" => ("mg", 1.0),
        "mcg" | "µg" => ("mg", 0.001),
        "g" | "gm" if denominator => ("ml", 1.0),
        "g" | "gm" => ("mg", 1000.0),
        "ml" => ("ml", 1.0),
        "unt" | "unit" | "units" | "iu" => ("unit", 1.0),
        "meq" => ("meq", 1.0),
        "%" => ("%", 1.0),
        "actuat" | "actuation" | "actuations" | "puff" | "puffs" => ("actuat", 1.0),
        "hr" | "hour" | "hours" | "h" => ("hr", 1.0),
        "day" | "days" => ("hr", 24.0),
        _ => return None,
    })
}

/// Every strength stated in a drug name or text, normalized.
pub fn strengths(text: &str) -> Vec<Strength> {
    let mut out = Vec::new();
    for m in STRENGTH.captures_iter(text) {
        let Some((unit, factor)) = base(&m[2], false) else {
            continue;
        };
        let amount: f64 = m[1].parse().expect("digits");
        let mut s = match m.get(4).and_then(|u| base(u.as_str(), true)) {
            Some((per, per_factor)) => {
                let per_amount: f64 = m
                    .get(3)
                    .map_or(1.0, |n| n.as_str().parse().expect("digits"));
                Strength {
                    amount: amount * factor / (per_amount * per_factor),
                    unit,
                    per,
                }
            }
            None => Strength {
                amount: amount * factor,
                unit,
                per: "",
            },
        };
        // A percentage is grams per 100 mL: 1% is 10 mg/mL.
        if s.unit == "%" && s.per.is_empty() {
            s = Strength {
                amount: s.amount * 10.0,
                unit: "mg",
                per: "ml",
            };
        }
        // A mass per mass (mg/mg) is a fraction; as mg/mL it is a thousand times that.
        if s.unit == "mg" && s.per == "mg" {
            s = Strength {
                amount: s.amount * 1000.0,
                unit: "mg",
                per: "ml",
            };
        }
        out.push(s);
    }
    out
}

fn same(a: &[Strength], b: &[Strength]) -> bool {
    let close = |x: &Strength, y: &Strength| {
        x.unit == y.unit
            && x.per == y.per
            && (x.amount - y.amount).abs() <= 1e-9 * x.amount.abs().max(y.amount.abs())
    };
    a.len() == b.len()
        && a.iter().all(|x| b.iter().any(|y| close(x, y)))
        && b.iter().all(|y| a.iter().any(|x| close(x, y)))
}

/// Whether an author's drug text states the same strength as an RxNorm name.
///
/// A pen or patch name leads with its volume or wear time (`0.5 ML dulaglutide 3
/// MG/ML`); the text matches either its concentrations or the amounts per device,
/// and must agree with the bare quantity only when it states one.
pub fn equivalent(text: &str, rxnorm_name: &str) -> bool {
    let (text_bare, wanted): (Vec<Strength>, Vec<Strength>) = strengths(text)
        .into_iter()
        .partition(|s| s.per.is_empty() && s.unit != "mg" && s.unit != "unit" && s.unit != "meq");
    let (name_bare, offered): (Vec<Strength>, Vec<Strength>) = strengths(rxnorm_name)
        .into_iter()
        .partition(|s| s.per.is_empty() && s.unit != "mg" && s.unit != "unit" && s.unit != "meq");
    if wanted.is_empty() || (!text_bare.is_empty() && !same(&text_bare, &name_bare)) {
        return false;
    }
    if same(&wanted, &offered) {
        return true;
    }
    // Amounts per device: a concentration per mL (or per hour) times the device's
    // volume (or wear time).
    let per_device: Vec<Strength> = offered
        .iter()
        .map(|s| match name_bare.iter().find(|b| b.unit == s.per) {
            Some(b) => Strength {
                amount: s.amount * b.amount,
                unit: s.unit,
                per: "",
            },
            None => s.clone(),
        })
        .collect();
    same(&wanted, &per_device)
}

fn ingredient(text: &str) -> Option<String> {
    let lower = text.to_lowercase();
    let word = lower
        .split(|c: char| !c.is_ascii_lowercase())
        .find(|w| !w.is_empty())?;
    Some(word.to_string())
}

/// The dose form a drug name ends with ("oral tablet", "extended release oral
/// capsule"): what follows its last strength.
fn form(text: &str) -> String {
    let rest = STRENGTH
        .find_iter(text)
        .last()
        .map_or("", |m| &text[m.end()..]);
    rest.split_whitespace()
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join(" ")
}

/// RxNav's best clinical or branded drug for `text` with the same ingredient and
/// an equivalent strength.
pub fn resolve_drug(look: &mut Lookup, text: &str) -> Result<Option<VerifiedCode>> {
    let Some(first) = ingredient(text) else {
        return Ok(None);
    };
    let found = |code: &str, display: &str, receipt: &str| VerifiedCode {
        code: code.into(),
        display: display.into(),
        ingredient: Some(first.clone()),
        receipt: receipt.into(),
        ..VerifiedCode::default()
    };
    // RxNorm's own name for this text ("calcium carbonate 600 MG Oral Tablet" names
    // its 1500 MG tablet; "pyridoxine" names vitamin B6; "petrolatum Topical
    // Ointment" names a dose form) is the drug itself
    if let Some(rxcui) = look.rxnorm_exact(text)?
        && let Some((tty, name)) = look.rxnorm_properties(&rxcui)?
        && matches!(
            tty.as_str(),
            "SCD" | "SBD" | "GPCK" | "BPCK" | "SCDF" | "SBDF" | "SCDG" | "SBDG"
        )
    {
        return Ok(Some(found(&rxcui, &name, "NLM RxNav rxcui (exact name)")));
    }
    let candidates = look.rxnorm_candidates(text)?;
    let mut others = Vec::new();
    for rxcui in &candidates {
        let Some((tty, name)) = look.rxnorm_properties(rxcui)? else {
            continue;
        };
        if !matches!(tty.as_str(), "SCD" | "SBD" | "GPCK" | "BPCK") {
            others.push(rxcui.clone());
            continue;
        }
        if name.to_lowercase().contains(&first) && equivalent(text, &name) {
            return Ok(Some(found(
                rxcui,
                &name,
                "NLM RxNav approximateTerm + properties",
            )));
        }
    }
    // approximateTerm can miss an exact clinical drug; try the ingredient's own list
    for drug in look.rxnorm_drugs(&first)? {
        let (Some(rxcui), Some(name)) = (drug["rxcui"].as_str(), drug["name"].as_str()) else {
            continue;
        };
        if equivalent(text, name) {
            return Ok(Some(found(rxcui, name, "NLM RxNav drugs")));
        }
    }
    // or it found only a drug component ("penicillin V potassium 250 MG"): its
    // drugs come in every form, so the text's form picks among them
    let wanted = form(text);
    for rxcui in others {
        for (code, name) in look.rxnorm_related_drugs(&rxcui)? {
            if equivalent(text, &name) && !wanted.is_empty() && form(&name) == wanted {
                return Ok(Some(found(
                    &code,
                    &name,
                    "NLM RxNav approximateTerm + related",
                )));
            }
        }
    }
    Ok(None)
}

/// Verify every out-of-catalog condition code and drug text in one unit's charts.
pub fn resolve(patients: &[Patient], catalog: &Catalog, look: &mut Lookup) -> Result<Terms> {
    let mut terms = Terms::default();
    for p in patients {
        for ev in &p.events {
            for c in &ev.conditions {
                let Some(code) = c.icd10.as_deref() else {
                    continue;
                };
                if catalog.conditions.contains_key(&c.label) {
                    continue;
                }
                let key = format!("{}|{code}", c.label);
                if terms.conditions.contains_key(&key)
                    || terms.rejected.iter().any(|r| r.contains(&key))
                {
                    continue;
                }
                let code = code.trim().to_uppercase();
                match look.icd10(&code) {
                    Ok(Some(display)) => {
                        let v = VerifiedCode {
                            code,
                            display,
                            receipt: "NLM Clinical Tables ICD-10-CM".into(),
                            ..VerifiedCode::default()
                        };
                        terms.conditions.insert(key, v);
                    }
                    Ok(None) => terms.rejected.push(format!(
                        "condition {key} not found{}",
                        closest(look.near(false, &code))
                    )),
                    Err(e) => terms
                        .rejected
                        .push(format!("condition {key} lookup failed: {e}")),
                }
            }
            for m in ev.medications.iter().filter(|m| !m.uncoded) {
                let Some(text) = m.drug_text.as_deref() else {
                    continue;
                };
                if terms.medications.contains_key(text)
                    || terms
                        .rejected
                        .iter()
                        .any(|r| r.ends_with(&format!("{text:?}")))
                {
                    continue;
                }
                match resolve_drug(look, text) {
                    Ok(Some(v)) => {
                        terms.medications.insert(text.into(), v);
                    }
                    Ok(None) => terms.rejected.push(format!("drug {text:?}")),
                    Err(e) => terms
                        .rejected
                        .push(format!("drug lookup failed ({e}): {text:?}")),
                }
            }
        }
    }
    // Labs and studies the catalog lacks, by their LOINC code; an order for one is
    // coded as a study.
    let mut extra: Vec<(bool, &str, &str, Option<&LabResult>)> = Vec::new();
    for ev in patients.iter().flat_map(|p| &p.events) {
        for r in ev.labs.iter().flat_map(|l| &l.results) {
            if let (false, Some(code)) = (catalog.labs.contains_key(&r.test), &r.loinc) {
                extra.push((true, &r.test, code, Some(r)));
            }
        }
        for r in &ev.reports {
            if let (false, Some(code)) = (catalog.reports.contains_key(&r.study), &r.loinc) {
                extra.push((false, &r.study, code, None));
            }
        }
        for o in &ev.orders {
            if let (Some(what), Some(code)) = (&o.order, &o.loinc) {
                extra.push((false, what, code, None));
            }
        }
    }
    // a name's unit comes from a result that states one, and one name is one test
    extra.sort_by_key(|(_, _, _, r)| r.and_then(|r| r.unit.as_ref()).is_none());
    for (lab, label, code, result) in extra {
        let unit = result.and_then(|r| r.unit.as_deref());
        if let (true, Some(v), Some(u)) = (lab, terms.labs.get(label), unit)
            && v.unit.as_deref().is_some_and(|known| known != u)
        {
            terms.rejected.push(format!(
                "lab {label:?} is given in {:?} and {u:?}; one name is one test",
                v.unit.as_deref().unwrap_or_default()
            ));
        }
        let known = if lab { &terms.labs } else { &terms.reports };
        if known.contains_key(label)
            || terms
                .rejected
                .iter()
                .any(|r| r.contains(&format!("{label:?}")))
        {
            continue;
        }
        let kind = if lab { "lab" } else { "report" };
        let code = code.trim().to_string();
        let unit = result.and_then(|r| r.unit.clone());
        // a blank unit is no unit, and the service rejects the question
        if let Some(u) = &unit
            && (u.trim().is_empty() || !look.ucum(u)?)
        {
            terms.rejected.push(format!(
                "{kind} {label:?} unit {u:?} is not a UCUM unit (mg/dL, 10*3/uL, [IU]/L, ...); \
                 leave it out for a result with no unit"
            ));
            continue;
        }
        match look.loinc(&code) {
            Ok(Some(display)) => {
                let v = VerifiedCode {
                    code,
                    display,
                    unit,
                    range: result.and_then(|r| r.range.clone()),
                    receipt: "NLM Clinical Tables LOINC".into(),
                    ..VerifiedCode::default()
                };
                let map = if lab {
                    &mut terms.labs
                } else {
                    &mut terms.reports
                };
                map.insert(label.to_string(), v);
            }
            Ok(None) => terms.rejected.push(format!(
                "{kind} {label:?} LOINC {code} not found{}",
                closest(look.near(true, label))
            )),
            Err(e) => terms
                .rejected
                .push(format!("{kind} {label:?} LOINC lookup failed: {e}")),
        }
    }
    // Allergens and procedures the catalog lacks, by their SNOMED CT concept.
    let mut snomed: Vec<(bool, &str, &str)> = Vec::new();
    for ev in patients.iter().flat_map(|p| &p.events) {
        for a in &ev.allergies {
            if let (false, Some(code)) = (catalog.allergens.contains_key(&a.substance), &a.snomed) {
                snomed.push((true, &a.substance, code));
            }
        }
        for x in &ev.procedures {
            if let (false, Some(code)) = (catalog.procedures.contains_key(&x.procedure), &x.snomed)
            {
                snomed.push((false, &x.procedure, code));
            }
        }
        for o in &ev.orders {
            if let (Some(what), Some(code)) = (&o.order, &o.snomed) {
                snomed.push((false, what, code));
            }
        }
    }
    for (allergen, label, code) in snomed {
        let (kind, tags): (&str, &[&str]) = if allergen {
            (
                "allergen",
                &["substance", "product", "medicinal product", "organism"],
            )
        } else {
            ("procedure", &["procedure", "regime/therapy"])
        };
        let known = if allergen {
            &terms.allergens
        } else {
            &terms.procedures
        };
        if known.contains_key(label)
            || terms
                .rejected
                .iter()
                .any(|r| r.contains(&format!("{label:?}")))
        {
            continue;
        }
        let code = code.trim().to_string();
        match look.snomed_of(&code, tags) {
            Some(display) => {
                let v = VerifiedCode {
                    code,
                    display,
                    receipt: "tx.fhir.org SNOMED CT".into(),
                    ..VerifiedCode::default()
                };
                let map = if allergen {
                    &mut terms.allergens
                } else {
                    &mut terms.procedures
                };
                map.insert(label.to_string(), v);
            }
            None => terms.rejected.push(format!(
                "{kind} {label:?} SNOMED {code} is not an active {} concept",
                tags.join(" or ")
            )),
        }
    }
    look.save()?;
    Ok(terms)
}

fn closest(near: Vec<String>) -> String {
    if near.is_empty() {
        String::new()
    } else {
        format!("; closest real codes: {}", near.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pen_matches_only_its_own_strength() {
        let pen = "0.5 ML dulaglutide 3 MG/ML Auto-Injector";
        assert!(!equivalent(
            "dulaglutide 3 mg/0.5 mL subcutaneous auto-injector",
            pen
        ));
        assert!(equivalent(
            "dulaglutide 1.5 mg/0.5 mL subcutaneous auto-injector",
            pen
        ));
        assert!(equivalent("dulaglutide 1.5 mg weekly pen", pen));
        assert!(!equivalent(
            "tirzepatide 5 mg/0.5 mL subcutaneous auto-injector",
            "0.5 ML tirzepatide 5 MG/ML Auto-Injector"
        ));
        assert!(equivalent(
            "tirzepatide 5 mg/0.5 mL subcutaneous auto-injector",
            "0.5 ML tirzepatide 10 MG/ML Auto-Injector"
        ));
    }

    #[test]
    fn tablets_patches_inhalers_and_creams_compare_by_quantity() {
        assert!(equivalent(
            "ibuprofen 200 mg oral tablet",
            "ibuprofen 200 MG Oral Tablet"
        ));
        assert!(!equivalent(
            "ibuprofen 200 mg oral tablet",
            "ibuprofen 400 MG Oral Tablet"
        ));
        assert!(equivalent(
            "valacyclovir 1 g oral tablet",
            "valacyclovir 1000 MG Oral Tablet"
        ));
        assert!(equivalent(
            "nicotine 21 mg/24 hr patch",
            "24 HR nicotine 0.875 MG/HR Transdermal System"
        ));
        assert!(equivalent(
            "estradiol 0.05 mg/day patch",
            "estradiol 0.05 MG/DAY Transdermal System"
        ));
        assert!(equivalent(
            "albuterol 90 mcg/actuation inhaler",
            "200 ACTUAT albuterol 0.09 MG/ACTUAT Metered Dose Inhaler"
        ));
        assert!(equivalent(
            "hydrocortisone 1% cream",
            "hydrocortisone 10 MG/ML Topical Cream"
        ));
        assert!(equivalent(
            "triamcinolone .1% cream",
            "triamcinolone 1 MG/ML Topical Cream"
        ));
        assert!(!equivalent(
            "triamcinolone .1% cream",
            "triamcinolone 10 MG/ML Topical Cream"
        ));
        assert!(equivalent(
            "lisinopril 20 mg / hydrochlorothiazide 12.5 mg tablet",
            "hydrochlorothiazide 12.5 MG / lisinopril 20 MG Oral Tablet"
        ));
    }

    #[test]
    fn text_without_a_strength_stays_uncoded() {
        assert!(!equivalent(
            "nicotine patch",
            "24 HR nicotine 0.875 MG/HR Transdermal System"
        ));
        assert!(!equivalent(
            "prenatal vitamin",
            "prenatal multivitamin Oral Tablet"
        ));
    }
}
